# bitcoin-cuckoo-cache

A faithful, safe Rust port of Bitcoin Core's *cuckoo-style* eviction cache, extracted and packaged as a reusable crate.

This structure underpins high‑performance, bounded‑memory caches in latency‑sensitive consensus code. It is engineered for:

- *Predictable memory usage* – fixed upper bound on entries
- *Fast membership queries* – O(1) expected time, 8‑way cuckoo probing
- *Lock‑free reads* – interior mutability for GC flags using `AtomicU8`
- *Lazy reclamation* – entries are logically discarded before being physically overwritten

The crate provides two primitives:

- [`EightWayHasher`] – abstraction over the 8 independent 32‑bit hash functions
- [`Cache`] – the cuckoo‑style cache itself, parameterised by element type and hasher

It also exposes the internal [`BitPackedAtomicFlags`] type used to compactly store collection flags.

---

## High‑level design

The cache implements an *8‑way cuckoo hash table with bounded kick‑out depth* and a lazily‑applied garbage‑collection scheme.

### Cuckoo hashing recap

Classical cuckoo hashing stores each element in one of several candidate buckets determined by independent hash functions. Upon insertion, if all candidate buckets are occupied, the algorithm *kicks out* an existing element and re‑inserts it in one of its alternative buckets. This process repeats up to some depth bound; if insertion still fails, an element is evicted from the table.

This crate implements that logic with:

- **Eight** independent 32‑bit hashes per element (`[u32; 8]`)
- A **bounded insertion depth** (`depth_limit ≈ log2(table_size)`) to cap worst‑case work
- A **logical garbage‑collection flag** per bucket which marks entries as discardable

The combination provides a *bounded‑size, probabilistic cache* ideal for ephemeral data such as transaction IDs, UTXO hints, or short‑lived deduplication filters.

### Hash mapping without division

The user‑supplied hasher yields eight 32‑bit hashes. These are mapped to bucket indices in `[0, size)` using a multiply‑and‑shift reduction:

```text
index = floor( (h * size) / 2^32 )
``

This treats `h` as a 32‑bit fixed‑point number in `[0, 1)` and scales by `size`. It is:

- Close to uniform (similar bias to `h % size` for large 32‑bit `h`)
- Much faster than integer division on many architectures
- Implementable using the upper 32 bits of a 64‑bit product

See: *Daniel Lemire, "A fast alternative to the modulo reduction"* for background.

---

## Core traits and types

### `EightWayHasher<E>`

```rust
pub trait EightWayHasher<E> {
    /// Compute eight independent 32‑bit hashes for `e`.
    fn hashes(&self, e: &E) -> [u32; 8];
}
```

You implement this trait to define how `E` is hashed into eight independent 32‑bit values. The cache assumes these hashes have high entropy and are sufficiently independent for cuckoo hashing to work well.

For example, you might use a domain‑separated cryptographic hash or a good non‑cryptographic PRF over `(e, k)` for `k ∈ {0..8}`.

#### Example: trivial wrapper over `bitcoin_hashes`

```rust
use bitcoin_cuckoo_cache::EightWayHasher;
use bitcoin_hashes::{sha256, Hash};

#[derive(Default, Clone)]
struct Sha256EightWay;

impl EightWayHasher<[u8; 32]> for Sha256EightWay {
    fn hashes(&self, e: &[u8; 32]) -> [u32; 8] {
        // For demonstration only; choose a real construction for production.
        let mut out = [0u32; 8];
        for i in 0..8u8 {
            let mut engine = sha256::Hash::engine();
            engine.input(e);
            engine.input(&[i]);
            let h = sha256::Hash::from_engine(engine);
            let bytes = h.as_inner();
            out[i as usize] = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        }
        out
    }
}
```

### `Cache<Element, Hash>`

```rust
#[derive(Getters, Debug)]
pub struct Cache<Element, Hash>
where
    Element: PartialEq + Clone,
    Hash: EightWayHasher<Element>,
{
    table:                   Vec<Option<Element>>,
    size:                    u32,
    collection_flags:        BitPackedAtomicFlags,
    epoch_flags:             Vec<bool>,
    epoch_heuristic_counter: u32,
    epoch_size:              u32,
    depth_limit:             u8,
    hash_function:           Hash,
}
```

Key properties:

- **Bounded size:**
  - Up to `(~(uint32_t)0) - 1` elements (limited by `u32` indices).
  - `size` is chosen once via [`Cache::setup`] or [`Cache::setup_bytes`].
- **Threading model:**
  - Writes (`insert`, `setup`, `setup_bytes`) must be externally synchronized.
  - Reads (`contains(erase = false)`) must not race with writes.
  - Erase‑style reads (`contains(erase = true)` or `allow_erase`) must not race with writes.
  - Lazy reclamation via `BitPackedAtomicFlags` uses only relaxed atomics; the caller is responsible for higher‑level happens‑before ordering between writers and readers.

The internal `epoch_*` fields implement a *generational aging heuristic*: frequently accessed elements are preferentially preserved while older, rarely touched entries are more likely to be overwritten under pressure.

---

## Construction and sizing

### `Cache::default`

```rust
impl<E, H> Default for Cache<E, H>
where
    E: PartialEq + Clone,
    H: EightWayHasher<E> + Default,
{
    fn default() -> Self { /* ... */ }
}
```

`Default` constructs an *empty and unusable* cache. You **must** call either [`setup`] or [`setup_bytes`] before any other operations; otherwise, you will hit panics or UB in practice.

#### Recommended pattern

```rust
use bitcoin_cuckoo_cache::{Cache, EightWayHasher};

#[derive(Default)]
struct MyHasher; // implements EightWayHasher<MyElement>

#[derive(Clone, PartialEq)]
struct MyElement { /* ... */ }

fn build_cache(bytes_budget: usize) -> Cache<MyElement, MyHasher> {
    let mut cache = Cache::<MyElement, MyHasher>::default();
    cache.setup_bytes(bytes_budget);
    cache
}
```

### `setup`

```rust
impl<E, H> Cache<E, H>
where
    E: PartialEq + Clone,
    H: EightWayHasher<E>,
{
    pub fn setup(&mut self, new_size: u32) -> u32 { /* ... */ }
}
```

- Configures the cache to store at most `new_size` elements.
- Chooses internal `size = max(2, new_size)`.
- Sets the depth limit approximately to `log2(size)`.
- Allocates:
  - `table: Vec<Option<E>>` of length `size`
  - `collection_flags: BitPackedAtomicFlags` with one bit per bucket
  - `epoch_flags: Vec<bool>` with one boolean per bucket

Returns the actual configured `size` (≥ 2).

This should be called *exactly once* for a given cache instance.

### `setup_bytes`

```rust
#[inline]
pub fn setup_bytes(&mut self, bytes: usize) -> u32 {
    let elem_sz = cmp::max(mem::size_of::<E>(), 1);
    self.setup((bytes / elem_sz) as u32)
}
```

Converts a byte budget to an element count using `sizeof(E)` and then calls [`setup`]. This does *not* include struct overhead or flag storage; for realistic use, assume a small constant‑factor overhead and size accordingly.

---

## Core operations

### `compute_hashes`

```rust
#[inline]
pub fn compute_hashes(&self, e: &E) -> [u32; 8] { /* ... */ }
```

- Calls `hash_function.hashes(e)` to obtain eight 32‑bit hashes.
- Maps each hash into `[0, size)` via the multiply‑and‑shift technique described above.
- Guarantees that every index returned is `0 <= index < size`.

This is a convenience helper used internally by insertion and lookup; you can also use it directly to inspect bucket placements.

### `invalid`

```rust
#[inline]
pub fn invalid() -> u32 {
    u32::MAX
}
```

Returns a sentinel bucket index that can never be produced by [`compute_hashes`]. Internal logic uses this as a marker for "no previous bucket" when choosing eviction candidates.

### `insert`

```rust
#[inline]
pub fn insert(&mut self, mut e: E) { /* ... */ }
```

Semantics:

- Runs the 8‑way cuckoo insertion algorithm with a bounded depth (`depth_limit`).
- Before insertion, `epoch_check()` (not shown here) may advance the aging epoch and update reclamation state.
- If any of the eight candidate buckets is currently *collectable* (its bit in `collection_flags` is **set**), the element is written there and marked as *keep* (`bit_unset`).
- If all candidate buckets are non‑collectable, an eviction sequence begins:
  - Choose one candidate location derived from the last eviction, cycling through the 8 bucket indices.
  - Swap the incoming element `e` with the victim stored at the chosen location.
  - Re‑hash the evicted element and repeat.
- If the depth limit is exhausted, the final evicted element is **dropped from the cache**; this is a deliberate eviction policy.

**Important invariant:**

```rust
cache.insert(x);
let may_be_present = cache.contains(&x, false);
```

`may_be_present` is not guaranteed to be `true`. An insertion can cause some other element (or even `x` itself) to be evicted under high load. This is intended: the cache behaves like a finite‑capacity, approximate remember‑set.

### `contains`

```rust
#[inline]
pub fn contains(&self, e: &E, erase: bool) -> bool { /* ... */ }
```

- Iterates over all eight candidate buckets for `e`.
- Returns `true` if a matching `Element` is found (based on `PartialEq`).
- Ignores the collection flags when checking membership: an element flagged as collectable still counts as *present* until physically overwritten.
- If `erase == true` and `e` is found, the corresponding bucket is marked collectable via [`allow_erase`].

A key property in a single‑threaded context:

```rust
insert(x);
if contains(&x, true) {
    assert!(contains(&x, false));
} else {
    // If the first contains fails, we do not make any assertion.
}
```

In the reference C++ comments, a slightly different pseudo‑code is given; for re‑org heavy workloads, this semantics is tuned so that certain sequences of `insert` and `contains` behave in a predictably idempotent way when executed without concurrency.

### `allow_erase` and `please_keep`

```rust
#[inline]
pub fn allow_erase(&self, n: u32) { /* ... */ }

#[inline]
pub fn please_keep(&self, n: u32) { /* ... */ }
```

- `allow_erase(n)` sets the collection bit for bucket `n` to **true** (collectable).
- `please_keep(n)` clears the collection bit for bucket `n` (not collectable).

Both operations are thread‑safe *in the absence of concurrent `insert`*, as they only manipulate `BitPackedAtomicFlags` with relaxed atomics. They do not modify `table` or `epoch_flags`.

The idea is that higher‑level logic can mark entries as *logically deleted* (`allow_erase`) while deferring physical reclamation to future `insert` calls that need space.

---

## Bit‑packed GC flags

### `BitPackedAtomicFlags`

```rust
#[derive(Getters, Debug)]
pub struct BitPackedAtomicFlags {
    mem: Box<[AtomicU8]>,
}
```

This structure stores one **garbage‑collection flag bit per bucket**, packed 8 per `AtomicU8` cell.

- **Construction:**

  ```rust
  impl BitPackedAtomicFlags {
      #[inline]
      pub fn new(size: u32) -> Self { /* ... */ }

      #[inline]
      pub fn setup(&mut self, b: u32) { /* ... */ }
  }
  ```

  - `new(size)` reserves `ceil(size / 8)` bytes and initialises all bits to 1 (`0xFF`), meaning **collectable by default**.
  - `setup(b)` replaces the internal storage with a new `BitPackedAtomicFlags` sized for at least `b` bits, again initialised with all bits set.
  - Both are thread‑unsafe while called, but safe once the structure is published.

- **Bit operations:**

  ```rust
  #[inline]
  pub fn bit_set(&self, s: u32) { /* ... */ }

  #[inline]
  pub fn bit_unset(&self, s: u32) { /* ... */ }

  #[inline]
  pub fn bit_is_set(&self, s: u32) -> bool { /* ... */ }
  ```

  - `bit_set(s)` marks index `s` as **collectable**.
  - `bit_unset(s)` marks index `s` as **keep / not collectable**.
  - `bit_is_set(s)` returns `true` if bucket `s` is currently collectable.

All operations use `Ordering::Relaxed`. The correctness of the cache relies on higher‑level synchronisation to order updates to `table` vs. these bits.

This design preserves the C++ implementation's memory layout and semantics while expressing them in Rust via interior mutability instead of `mutable` data members.

---

## Concurrency and memory ordering

This crate mirrors the reference Bitcoin implementation:

- **Writers** must be serialized.
- **Readers** must not race with writers that modify the same cache instance.
- All atomic accesses to collection flags use `Relaxed` ordering.

The intent is that this cache is used *behind* a higher‑level synchronisation primitive (e.g. `Mutex`, `RwLock`, or a custom epoch‑based reclamation scheme) that enforces the desired consistency model for your application.

### Safety rules recap

The documentation and comments imply the following discipline:

1. Write requires synchronized access (e.g. a lock).
2. Read requires no concurrent write, synchronized with the last insert.
3. Erase requires no concurrent write, synchronized with the last insert.
4. An Erase caller must release all memory before allowing a new writer.

Violating these assumptions may lead to observable races even though the crate uses only safe Rust (e.g., inconsistent visibility of `table` vs. flags).

---

## Usage example

Below is a minimal, self‑contained example demonstrating basic API usage in a single‑threaded context.

```rust
use bitcoin_cuckoo_cache::{Cache, EightWayHasher};

#[derive(Clone, PartialEq, Debug)]
struct Key(u64);

#[derive(Default)]
struct SimpleHasher;

impl EightWayHasher<Key> for SimpleHasher {
    fn hashes(&self, e: &Key) -> [u32; 8] {
        // Extremely simplistic; do not use in production.
        let mut x = e.0.wrapping_mul(0x9E3779B97F4A7C15);
        let mut out = [0u32; 8];
        for i in 0..8 {
            x ^= (x >> 12) ^ ((i as u64) << 32);
            x = x.rotate_left(27).wrapping_mul(0x94D049BB133111EB);
            out[i] = (x as u32) ^ ((x >> 32) as u32);
        }
        out
    }
}

fn main() {
    // Construct cache with a target memory budget.
    let mut cache: Cache<Key, SimpleHasher> = Cache::default();
    let capacity = cache.setup_bytes(1024); // approx capacity in elements

    println!("configured cache capacity: {}", capacity);

    let k1 = Key(1);
    let k2 = Key(2);

    cache.insert(k1.clone());
    cache.insert(k2.clone());

    assert!(cache.contains(&k1, false));
    assert!(cache.contains(&k2, false));

    // Mark k1 as discardable if present.
    if cache.contains(&k1, true) {
        // At this point, a future insert might evict `k1`.
    }
}
```

---

## When should you use this crate?

This cache is appropriate when you need:

- A *fixed‑memory*, high‑throughput cache with probabilistic eviction
- Very cheap membership checks (`contains`) with stable latency
- Control over logical deletion vs. physical reclamation
- Behaviour consistent with Bitcoin Core's existing C++ implementation

Typical applications include:

- Transaction or block ID replay caches
- Short‑lived deduplication filters
- Bounded caches for intermediate validation state where false negatives are tolerable but unbounded memory growth is not

It is *not* a drop‑in replacement for keyed maps or fully general LRU structures; the eviction semantics are designed around cuckoo hashing, not recency or frequency tracking.

---

## Crate metadata

- **Crate name:** `bitcoin-cuckoo-cache`
- **Version:** `0.1.19`
- **Edition:** `2021`
- **License:** MIT
- **Repository:** <https://github.com/klebs6/bitcoin-rs>
- **Authors:** `klebs <none>`
