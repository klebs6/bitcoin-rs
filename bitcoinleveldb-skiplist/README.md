# bitcoinleveldb-skiplist

A faithful, arena-backed translation of LevelDB's internal `SkipList<Key, Comparator>` to safe(ish) Rust, intended for use inside high‑throughput storage engines such as LevelDB clones, LSM‑trees, or log‑structured key–value stores.

This crate packages the skip list used by the `bitcoinleveldb` components of [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs). It mirrors the invariants, complexity properties, and memory layout of the original C++ implementation closely enough to be used as a drop‑in internal building block.

---

## Features at a Glance

- **Arena‑backed allocation**: Nodes are allocated from a user‑supplied `Arena`, matching LevelDB's custom allocator usage.
- **Deterministic layout**: `#[repr(C)]` node layout with a flexible tail for `next` pointers, equivalent to a C++ FAM (flexible array member).
- **LevelDB‑style comparator**: A `SkipListComparator<K>` trait that returns `< 0`, `0`, or `> 0`, and a blanket impl for `Fn(&K, &K) -> i32 + Send + Sync`.
- **Lock‑free reads**: Carefully chosen atomic orderings (`Acquire`/`Release`/`Relaxed`) to permit concurrent readers in typical LevelDB usage patterns.
- **Iterator with LevelDB semantics**: `SkipListIterator` exposes `seek`, `seek_to_first`, `seek_to_last`, `next`, and `prev`, plus `valid`/`key`.
- **Random tower heights**: `RandomHeight()` with branching factor 4, as in the original LevelDB skip list.

This crate intentionally caters to consumers who already understand LevelDB's concurrency model and are willing to work near raw pointers and arenas for maximal control.

---

## Conceptual Overview

A **skip list** is an ordered probabilistic data structure that supports search, insertion, and membership queries with expected logarithmic time complexity, comparable to balanced binary search trees, but usually easier to implement and more cache‑friendly.

### Structural Invariant

- Nodes are sorted by key in strictly increasing order according to a user‑provided comparator `C: SkipListComparator<K>`.
- Each node participates in a **tower** of forward pointers of height `h ∈ [1, H_max]`.
- Level `0` forms a sorted singly linked list of all nodes.
- Higher levels form sparser, exponentially thinning express lanes used to accelerate search.

Heights are chosen independently with geometric distribution: with branching factor 4, each additional level is present with probability 1/4, up to `SkipListMaxHeight`. This yields an expected complexity of `O(log n)` for search and insertion, with high probability bounds similar to balanced trees.

### Concurrency Model (LevelDB‑style)

The design follows LevelDB's **single writer + multiple lock‑free readers** model:

- **Writer**: Inserts into the skip list, using `Release` stores to publish new nodes.
- **Readers**: Traverse the skip list using `Acquire` loads, observing a consistent, fully‑initialized view of any node they reach.
- **No deletion**: Nodes are never removed individually. The arena lifetime matches the skip list lifetime, and memory for nodes is reclaimed only when the arena is dropped.

If you follow the original LevelDB pattern (single mutable owner for insertions, many readers with only shared references), the atomic semantics in this crate are designed to be correct.

---

## Data Structures

### `SkipListComparator<K>`

```rust
pub trait SkipListComparator<K> {
    /// Compare two keys. Return < 0 if a < b, 0 if a == b, > 0 if a > b.
    fn compare(&self, a: &K, b: &K) -> i32;
}
```

This trait abstracts key ordering. Semantics mirror LevelDB:

- It must define a **strict weak ordering** over `K`.
- All operations on the `SkipList` assume the comparator is **consistent** and **transitive**.

A blanket implementation is provided for any function or closure of type `Fn(&K, &K) -> i32 + Send + Sync`, allowing you to use simple lambdas as comparators.

### `SkipListNode<K>`

```rust
#[repr(C)]
pub struct SkipListNode<K>
where
    K: Copy + Default,
{
    key:  K,
    next: [AtomicPtr<SkipListNode<K>>; 1],
}
```

This internal node struct is arranged so that, when allocated via the arena, it can reserve additional trailing storage for higher levels' `next` pointers (like a flexible array member). You do not normally interact with `SkipListNode` directly, but its methods are:

- `key_ref(&self) -> &K`: Borrow the stored key.
- `next(level: i32) -> *mut Self`: Load `next` pointer at a level with **Acquire** semantics.
- `set_next(level: i32, node: *mut Self)`: Store `next` with **Release** semantics.
- `no_barrier_next(level: i32)`, `no_barrier_set_next(level: i32, node)`: Same, but with **Relaxed** orderings.

These are used internally by the `SkipList` to maintain lock‑free traversal guarantees.

### `SkipList<K, C>`

```rust
#[derive(Getters, MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct SkipList<K, C>
where
    K: Debug + Copy + Default,
    C: SkipListComparator<K>,
{
    compare:    C,
    arena:      *mut Arena,
    head:       *mut SkipListNode<K>,
    max_height: AtomicI32,
    rnd:        Random,
}
```

Key constraints:

- `K: Copy + Default`: The key must be trivially copyable; `Default` is used for the dummy head node.
- `K: Debug`: Only required for logging, assertions, and tracing.
- `C: SkipListComparator<K>`: Provides ordering.

Important methods:

- `pub fn new(compare: C, arena: *mut Arena) -> Self`
- `pub fn insert(&mut self, key: K)`
- `pub fn contains(&self, key: &K) -> bool`
- `pub fn get_max_height(&self) -> i32`

Low‑level navigation primitives (intended for internal / advanced users):

- `find_greater_or_equal(&self, key: &K, prev: Option<&mut [*mut SkipListNode<K>]>) -> *mut SkipListNode<K>`
- `find_less_than(&self, key: &K) -> *mut SkipListNode<K>`
- `find_last(&self) -> *mut SkipListNode<K>`

### `SkipListIterator<'a, K, C>`

```rust
pub struct SkipListIterator<'a, K, C>
where
    K: Debug + Copy + Default,
    C: SkipListComparator<K>,
{
    list: &'a SkipList<K, C>,
    node: *mut SkipListNode<K>,
}
```

This iterator is **intentionally copyable** and holds:

- An immutable reference to the underlying skip list.
- A raw pointer to the current node.

Methods mirror LevelDB's iterator interface:

- `pub fn new(list: &'a SkipList<K, C>) -> Self`
- `pub fn valid(&self) -> bool`
- `pub fn key(&self) -> K`
- `pub fn next(&mut self)`
- `pub fn prev(&mut self)`
- `pub fn seek(&mut self, target: &K)`
- `pub fn seek_to_first(&mut self)`
- `pub fn seek_to_last(&mut self)`

---

## Basic Usage

### Dependencies

In your `Cargo.toml`:

```toml
[dependencies]
bitcoinleveldb-skiplist = "0.1.19"
```

### Constructing a Skip List

You must provide:

- An `Arena` (from the surrounding `bitcoinleveldb` or storage engine implementation).
- A comparator `C`, usually as a closure.

```rust
use bitcoinleveldb_skiplist::{SkipList, SkipListComparator};

// Pseudocode: obtain a mutable pointer to an Arena from your system.
let arena: *mut Arena = obtain_arena();

// Comparator: ascending `u64` keys.
let cmp = |a: &u64, b: &u64| {
    if *a < *b { -1 } else if *a > *b { 1 } else { 0 }
};

let mut list: SkipList<u64, _> = SkipList::new(cmp, arena);

list.insert(10);
list.insert(20);
list.insert(15);

assert!(list.contains(&10));
assert!(!list.contains(&42));
```

Note that `Arena` is not defined in this crate; it is taken as a raw pointer so that the skip list can integrate with an external allocator with custom alignment and lifetime guarantees. This matches LevelDB's design: the arena is long‑lived and freed only when the entire structure is discarded.

### Iteration

```rust
let mut it = bitcoinleveldb_skiplist::SkipListIterator::new(&list);

// Forward iteration from the smallest key
it.seek_to_first();
while it.valid() {
    let key = it.key();
    // process key...
    it.next();
}

// Seek and scan from an arbitrary key
let mut it = bitcoinleveldb_skiplist::SkipListIterator::new(&list);
it.seek(&15);
if it.valid() {
    assert!(it.key() >= 15);
}

// Reverse traversal
it.seek_to_last();
while it.valid() {
    let key = it.key();
    // process key from high to low...
    it.prev();
}
```

The iterator intentionally uses raw pointers and `assert!`s to catch misuse, rather than returning `Option<K>` from every operation. You must respect the preconditions documented in the method comments:

- `key`, `next`, `prev` require `valid() == true`.

### Custom Key Types

Keys must be `Copy + Default + Debug`. A typical pattern is to use integer or fixed‑size types (e.g., hashes, pointers, or small structs).

Example with a fixed‑size 32‑byte key:

```rust
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Key32([u8; 32]);

impl Key32 {
    fn cmp(&self, other: &Self) -> i32 {
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            if a < b { return -1; }
            if a > b { return  1; }
        }
        0
    }
}

let arena: *mut Arena = obtain_arena();
let cmp = |a: &Key32, b: &Key32| a.cmp(b);

let mut list: SkipList<Key32, _> = SkipList::new(cmp, arena);

let key = Key32([0; 32]);
list.insert(key);
assert!(list.contains(&key));
```

---

## Safety and Correctness Considerations

This crate operates close to the metal:

- **Raw pointers**: Both `SkipList` and `SkipListIterator` store raw pointers into an external arena.
- **External allocator (Arena)**: You are responsible for ensuring the arena outlives the `SkipList` and all iterators.
- **No deletion**: There is no support for removing individual entries. This is by design to simplify memory management and concurrency; you are expected to discard or recycle entire arenas.

### Invariants You Must Maintain

1. **Arena lifetime**: The `arena: *mut Arena` supplied to `SkipList::new` must remain valid and must not be freed or repurposed while any skip list node is in use.
2. **Comparator consistency**: For all `a`, `b`, `c`, the comparator must be transitive and antisymmetric. Breaking this assumption invalidates search and insertion logic.
3. **Single writer discipline**: To mimic LevelDB's behavior, you should perform all `insert` operations from a single thread or otherwise serialize writes logically; concurrent writers are not supported by the current algorithm.
4. **Read‑only from shared references**: Readers must only hold shared references to `SkipList` while traversing; they must not mutate the data structure.

Violating these invariants can lead to undefined behavior due to the use of unsafe code and raw pointers.

---

## Performance Characteristics

The skip list uses the classical geometric tower height distribution with branching factor 4. Under standard probabilistic assumptions:

- Search `find_greater_or_equal`, `contains`: `O(log n)` expected time.
- Insertion `insert`: `O(log n)` expected time, dominated by search for insertion points.
- Memory overhead: Each node stores on average a constant number of pointers (given by the geometric series), plus the key itself.

The arena allocator reduces per‑node allocation overhead, improving spatial locality and cache performance. The skip list is well‑suited to workloads where:

- The structure is constructed and extended over time.
- Individual deletions are not required, or reclamation is managed by discarding entire arenas.
- Lock‑free or low‑lock reads are important, and a single writer pattern is acceptable.

---

## Integration with `bitcoin-rs`

This crate lives in the [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs) repository and is primarily intended as an internal building block for the LevelDB‑like storage layer used by that project. Nonetheless, it is packaged as a separate crate so it can be reused by other storage engines or experimental systems that require:

- A close translation of LevelDB's `SkipList` behavior.
- Explicit control over allocation and concurrency.

When integrating into other systems, you will typically:

1. Implement or reuse an `Arena` abstraction comparable to the one used in `bitcoin-rs`.
2. Decide on a key type `K` that satisfies `Copy + Default + Debug`.
3. Supply a comparator that matches your key's ordering.

---

## License

This crate is licensed under the **MIT** license.

See the repository for full license text and any additional notices.
