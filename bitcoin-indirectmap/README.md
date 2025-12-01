# bitcoin-indirectmap

A thin, well‑typed façade around `BTreeMap` that stores `Arc<K>` keys but orders and queries them **by the dereferenced `K` value**. This mirrors the `indirectmap` pattern used in the Bitcoin Core C++ code‑base and provides explicit iterator and size types via a trait abstraction.

---

## Overview

`bitcoin-indirectmap` implements an *indirection‑aware* ordered map:

- Keys are stored as `Arc<K>` for shared ownership and cheap cloning.
- Ordering and lookup are performed by `K: Ord` on the **underlying value**, not on pointer identity.
- The internal data structure is a `BTreeMap<IndirectKey<K>, V>` where `IndirectKey<K>` is a wrapper that defines ordering on `K` while physically storing `Arc<K>`.
- A `HasIndirectMapTypes` trait abstracts over iterator and size types, allowing consumers (notably a Bitcoin Core style code‑base) to depend on a stable façade instead of concrete container types.

This is useful when you want:

- Deterministic ordered maps keyed by complex, heap‑allocated objects shared across the system via `Arc`.
- Map semantics defined by **value equality** (`K: Ord + Eq`) rather than pointer identity, while still benefiting from shared ownership.
- A Rust analogue of Bitcoin Core’s C++ `indirectmap`, to ease porting and keep conceptual parity.

---

## Core Types

### `HasIndirectMapTypes<'a>`

```rust
pub trait HasIndirectMapTypes<'a> {
    type Iterator:      Iterator<Item = (&'a Arc<Self::Key>, &'a Self::Value)> + 'a;
    type ConstIterator: Iterator<Item = (&'a Arc<Self::Key>, &'a Self::Value)> + 'a;
    type SizeType;
    type Key;
    type Value;
}
```

A trait used to expose associated iterator and size types without committing to a specific container implementation. For `IndirectMap<K, V>`:

- `Iterator` / `ConstIterator` are boxed trait objects over iterators yielding `(&Arc<K>, &V)`.
- `SizeType` is `usize`.
- `Key` and `Value` are `K` and `V` respectively.

This abstraction is useful in a large, modular code‑base that wishes to hide container implementation details while still exposing a rich iterator API.

### `IndirectKey<K>`

```rust
#[derive(Clone)]
pub struct IndirectKey<K: Ord>(Arc<K>);
```

`IndirectKey<K>` is a small wrapper around `Arc<K>` that implements:

- `Borrow<K>`
- `PartialEq`, `Eq`
- `PartialOrd`, `Ord`

All comparisons and ordering operations are defined on the **dereferenced `K`**. Consequently:

- The map treats two `Arc<K>` values as equal if their underlying `K` values compare equal, even if they are distinct heap allocations.
- `Borrow<K>` lets `BTreeMap` perform lookups by `&K`, not just by `IndirectKey<K>`.

Key invariants:

- `K: Ord` must define a **total order** over keys.
- Once inserted into the map, a `K` **must not be mutated** in any way that changes its ordering; otherwise the `BTreeMap` invariants are violated. This is the standard constraint for ordered maps with interior mutability.

### `IndirectMap<K, V>`

```rust
#[derive(Builder, Getters, MutGetters)]
#[builder(pattern = "owned")]
pub struct IndirectMap<K, V>
where
    K: Ord,
{
    map: BTreeMap<IndirectKey<K>, V>,
}
```

`IndirectMap<K, V>` is the primary public type:

- Internally: `BTreeMap<IndirectKey<K>, V>`.
- Externally: operations keyed by `Arc<K>` for insertion and by `&K` for lookup and removal.
- Deterministic order defined by `Ord` on `K`.

The crate also implements memory‑accounting traits:

```rust
impl<K: Ord, V> DynamicUsage for IndirectMap<K, V> {
    fn dynamic_usage(&self) -> usize { /* ... */ }
}

impl<K: Ord, V> IncrementalDynamicUsage for IndirectMap<K, V> {
    fn incremental_dynamic_usage(&self) -> usize { /* ... */ }
}
```

These provide coarse estimates of dynamic memory usage per map and per entry using `mem::size_of::<IndirectKey<K>>()` and `mem::size_of::<V>()`, making it easier to reason about memory footprints in environments where memory usage needs to be budgeted explicitly (as is common in Bitcoin Core).

---

## API Summary

Below is the principal surface of `IndirectMap<K, V>` (simplified to highlight behavior):

```rust
impl<K: Ord, V> IndirectMap<K, V> {
    pub fn insert(&mut self, key: Arc<K>, value: V) -> bool;
    pub fn find(&self, k: &K) -> Option<(&Arc<K>, &V)>;
    pub fn find_mut(&mut self, k: &K) -> Option<(Arc<K>, &mut V)>;

    pub fn get(&self, k: &K) -> Option<(&Arc<K>, &V)>;
    pub fn get_mut(&mut self, k: &K) -> Option<(Arc<K>, &mut V)>;

    pub fn erase(&mut self, k: &K) -> bool;
    pub fn count(&self, k: &K) -> usize; // 0 or 1

    pub fn lower_bound<'a>(&'a self, k: &K)
        -> Box<dyn Iterator<Item = (&'a Arc<K>, &'a V)> + 'a>;

    pub fn is_empty(&self) -> bool;
    pub fn size(&self) -> usize;
    pub fn clear(&mut self);

    pub fn iter<'a>(&'a self)
        -> Box<dyn Iterator<Item = (&'a Arc<K>, &'a V)> + 'a>;

    pub fn iter_mut<'a>(&'a mut self)
        -> Box<dyn Iterator<Item = (&'a Arc<K>, &'a mut V)> + 'a>;

    pub fn max_size(&self) -> usize; // == usize::MAX
}
```

### Insert

```rust
pub fn insert(&mut self, key: Arc<K>, value: V) -> bool
```

- Inserts `(Arc<K>, V)` into the map.
- Returns `true` if the key (by value) was **newly** inserted.
- Returns `false` if a value with the same underlying `K` already existed and was overwritten.

### Lookup by `&K`

All lookup operations take a `&K` and operate on the ordering of `K`:

- `find(&self, k: &K) -> Option<(&Arc<K>, &V)>`
- `get(&self, k: &K) -> Option<(&Arc<K>, &V)>`

These are semantically similar; `find` is the more explicit API.

`find_mut` and `get_mut` are the mutable counterparts:

```rust
pub fn find_mut(&mut self, k: &K) -> Option<(Arc<K>, &mut V)>;
pub fn get_mut(&mut self, k: &K) -> Option<(Arc<K>, &mut V)>; // convenience wrapper
```

Implementation detail:

- `find_mut` performs a two‑phase operation:
  1. It obtains an immutable reference to the internal key, clones the `Arc<K>`.
  2. It then acquires a mutable reference to the value, avoiding aliasing a mutable reference with a live immutable reference to the same entry.

You get back:

- A cloned `Arc<K>` that you may hold freely.
- A `&mut V` allowing you to mutate the mapped value.

### Erase and Count

```rust
pub fn erase(&mut self, k: &K) -> bool;
pub fn count(&self, k: &K) -> usize; // ∈ {0,1}
```

- `erase` removes an entry by key value and returns `true` if something was removed.
- `count` returns `1` if an entry exists and `0` otherwise, mirroring the C++ `std::map` API.

### Ordered Iteration and `lower_bound`

```rust
pub fn lower_bound<'a>(&'a self, k: &K)
    -> Box<dyn Iterator<Item = (&'a Arc<K>, &'a V)> + 'a>;
```

- Returns an iterator starting at the first element whose key is **not less than** `k` (i.e., `k' >= k` under `Ord` on `K`).
- This is directly analogous to C++ `std::map::lower_bound`.

Regular iteration:

```rust
pub fn iter<'a>(&'a self)
    -> Box<dyn Iterator<Item = (&'a Arc<K>, &'a V)> + 'a>;

pub fn iter_mut<'a>(&'a mut self)
    -> Box<dyn Iterator<Item = (&'a Arc<K>, &'a mut V)> + 'a>;
```

Both preserve the underlying `BTreeMap` order (ascending according to `K: Ord`).

---

## Example Usage

```rust
use std::sync::Arc;
use bitcoin_indirectmap::IndirectMap; // adjust if the actual module path differs

fn main() {
    let mut map: IndirectMap<String, u64> = IndirectMap::default();

    let k1 = Arc::new("alice".to_owned());
    let k2 = Arc::new("bob".to_owned());

    assert!(map.insert(k1.clone(), 10)); // new
    assert!(map.insert(k2.clone(), 42)); // new
    assert!(!map.insert(Arc::new("alice".to_owned()), 11)); // same logical key, overwritten

    // Lookup by &K
    if let Some((key_arc, value)) = map.find(&"alice".to_owned()) {
        assert!(Arc::ptr_eq(key_arc, &k1) || **key_arc == "alice".to_string());
        assert_eq!(*value, 11);
    }

    // Mutable lookup
    if let Some((_key_arc, value_mut)) = map.get_mut(&"bob".to_owned()) {
        *value_mut += 1;
    }

    // Ordered iteration
    for (key, value) in map.iter() {
        println!("{} => {}", key, value);
    }

    // Erase
    assert!(map.erase(&"alice".to_owned()));
    assert_eq!(map.count(&"alice".to_owned()), 0);
}
```

### Using `lower_bound`

```rust
use std::sync::Arc;
use bitcoin_indirectmap::IndirectMap;

fn demo_lower_bound() {
    let mut map: IndirectMap<i32, &'static str> = IndirectMap::default();

    map.insert(Arc::new(1), "one");
    map.insert(Arc::new(3), "three");
    map.insert(Arc::new(5), "five");

    // Iterate starting from the first key >= 3
    let from_three: Vec<_> = map
        .lower_bound(&3)
        .map(|(k, v)| (**k, *v))
        .collect();

    assert_eq!(from_three, vec![(3, "three"), (5, "five")]);
}
```

---

## Design Rationale

### Value‑Based Ordering with Shared Ownership

The central idea is to decouple **storage** from **semantic identity**:

- Storage: `Arc<K>` allows multiple subsystems to share ownership of complex keys (e.g., Bitcoin transactions, block headers, or script descriptors) with minimal cloning cost.
- Semantic identity: ordering and equality are defined on the logical key `K`, not on the pointer.

This avoids pathologies where two distinct `Arc<K>` instances holding equal `K` values are treated as different keys. Instead, the map behaves as if you keyed directly on `K`, but lends out `Arc<K>` on iteration and lookup, improving ergonomics and efficiency for consumers that expect shared ownership.

### Safety and Invariants

Because the underlying container is a `BTreeMap`, the usual invariant applies:

- `K` must not be mutated in a way that affects `Ord` after insertion. Doing so would break the ordering invariants of the tree.

In practice, this usually means that `K` is either:

- An immutable data structure (e.g., hash digests, heights, identifiers), or
- Mutated only in fields that do not participate in the `Ord` implementation.

The Rust type system enforces many of these constraints, but any interior mutability (`Cell`, `RefCell`, `Mutex`, etc.) inside `K` must be used judiciously.

### Trait‑Driven Indirection (`HasIndirectMapTypes`)

The `HasIndirectMapTypes<'a>` trait formalizes the iterator and size types associated with the map. This promotes decoupling by allowing higher‑level components to program against a stable interface without committing to a specific container type.

Conceptually, this is analogous to using C++ typedefs and container concepts while keeping open the possibility of changing the underlying container in the future.

---

## Memory Accounting

`DynamicUsage` and `IncrementalDynamicUsage` provide simple, composable estimates:

- `dynamic_usage(&self)` computes an approximate total dynamic usage as:

  ```text
  (size_of::<IndirectKey<K>>() + size_of::<V>()) * self.len()
  ```

- `incremental_dynamic_usage(&self)` returns the per‑entry incremental usage:

  ```text
  size_of::<IndirectKey<K>>() + size_of::<V>()
  ```

This ignores allocator overhead and certain internal `BTreeMap` details but is predictable and cheap to compute. It is appropriate for coarse budget enforcement and telemetry.

---

## Integration Notes

- **Crate name:** `bitcoin-indirectmap`
- **Repository:** <https://github.com/klebs6/bitcoin-rs>
- **License:** MIT
- **Rust edition:** 2021

This crate is intended to integrate into a larger Bitcoin‑oriented Rust code‑base and mirrors data‑structure patterns from Bitcoin Core. It should also be generally usable for any project that wants ordered maps keyed by `Arc<K>` with value‑based ordering semantics.

---

## Caveats and Limitations

- The map is not thread‑safe by itself; it is a standard `BTreeMap`‑backed structure. Wrap it in synchronization primitives (`Mutex`, `RwLock`) if shared across threads.
- `DynamicUsage` is an estimate; do not treat it as a precise byte‑accurate measurement.
- Be careful not to mutate `K` in a way that changes ordering after insertion.

---

## Contributing

Contributions to improve ergonomics, coverage of Bitcoin Core idioms, and documentation are welcome. Please open issues and pull requests in the upstream repository:

<https://github.com/klebs6/bitcoin-rs>
