# bitcoin-epoch

Epoch-based deduplication for Bitcoin mempool graph traversals, extracted as a focused, RAII-driven Rust crate.

This crate implements the C++-style *epoch traversal* pattern used in Bitcoin Core to avoid revisiting transactions during ancestor/descendant walks over the mempool graph. It provides a tiny, contention-free mechanism to replace `std::set` / `HashSet`-based deduplication with a monotonic counter and per-node markers.

## Conceptual overview

When walking a graph (e.g., mempool transactions linked by dependencies), a naïve deduplication strategy stores a `visited` set containing each transaction seen so far. This introduces:

- **Time complexity**: `O(log n)` per insert/lookup in tree-based sets, or extra constant factors for hash sets.
- **Memory traffic**: dynamic allocations and cache-unfriendly pointer-chasing for each insertion.

The *epoch* technique replaces the explicit `visited` set with:

- A single **global traversal counter**: the `Epoch`.
- A **per-node marker**: an `EpochMarker` stored with each transaction.

Traversal algorithm:

1. Start a new traversal by incrementing the global `epoch` and marking it as **guarded** via `EpochGuard`.
2. For each node (transaction), compare its `EpochMarker` against the current `epoch` value.
3. If the marker is **less than** the current epoch, this is the first visit in this traversal; update the marker to the current epoch and continue.
4. If the marker is **equal to or greater than** the current epoch, the node has already been visited in this traversal; skip.

This yields:

- **Amortized O(1)** tracking per node
- **Zero per-visit heap allocations**
- **Excellent cache locality** when `EpochMarker` is embedded in existing structures

This pattern is mathematically analogous to using a strictly monotone mapping from traversal instances to integers and storing a scalar per vertex for the most recent traversal that touched it.

## Crate API

### Types

```rust
use std::cell::RefCell;
use std::rc::Rc;

use bitcoin_epoch::{Epoch, EpochGuard, EpochMarker};
``

#### `Epoch`

```rust
#[derive(Getters, Builder)]
#[getset(get = "pub")]
#[builder(setter(into), default)]
pub struct Epoch {
    raw_epoch: u64,
    guarded:   bool,
}
```

An RAII-style epoch tracker. It holds:

- `raw_epoch: u64` – the current epoch counter.
- `guarded: bool` – `true` when an `EpochGuard` is active.

Key methods:

- `fn visited(&self, marker: &mut EpochMarker) -> bool`
  - Returns `true` if the marker indicates that the node has already been visited in the **current** epoch.
  - Returns `false` and updates the marker for the first visit in the current epoch.
  - **Panics** if called outside an `EpochGuard` scope (`guarded == false`).

Internal methods (crate-internal, documented here for conceptual completeness):

- `fn increment_epoch(&mut self)` – `raw_epoch += 1` with overflow check.
- `fn set_guarded(&mut self, value: bool)` – toggles `guarded`.

`Epoch` implements `Default` with `raw_epoch = 0`, `guarded = false`.

#### `EpochMarker`

```rust
#[derive(Getters, Builder, Default)]
#[getset(get = "pub")]
pub struct EpochMarker {
    marker: u64,
}
```

A per-transaction (or per-node) marker storing the last epoch in which this node was visited.

- Initially `0` via `Default`.
- Updated *only* by `Epoch` through a crate-internal method, exposed here for understanding:

```rust
impl EpochMarker {
    pub(crate) fn update(&mut self, value: u64) {
        self.marker = value;
    }
}
```

In a consumer of this crate, you would typically store an `EpochMarker` as a field on your mempool entry struct and never mutate it manually.

#### `EpochGuard`

```rust
pub struct EpochGuard {
    epoch: Rc<RefCell<Epoch>>,
}
```

An RAII guard for a **scope-bound epoch**. On construction:

- Verifies that there is no active guard (`guarded == false`), panicking if a nested guard is attempted.
- Increments the epoch and sets `guarded = true`.

On `Drop` (i.e., at scope exit):

- Asserts that `guarded == true`.
- Increments the epoch again to create a clear separation between traversals.
- Sets `guarded = false`.

This ensures each guarded traversal obtains a **fresh epoch span** and that nested traversals are disallowed, preventing subtle logical corruption.

Constructor:

```rust
impl EpochGuard {
    pub fn new(epoch: Rc<RefCell<Epoch>>) -> Self { /* ... */ }
}
```

### Macro: `with_fresh_epoch!`

```rust
with_fresh_epoch!(mempool_epoch);
```

The macro expands to construction of a scoped `EpochGuard`:

```rust
let _epoch_guard = bitcoin_epoch::EpochGuard::new(mempool_epoch.clone());
```

Usage pattern:

```rust
{
    with_fresh_epoch!(mempool_epoch);
    // run traversal using mempool_epoch
} // guard dropped here; epoch advanced and unguarded
```

This mirrors the original C++ `WITH_FRESH_EPOCH` helper in Bitcoin Core.

## Typical usage pattern

Assume you have a mempool entry type embedding an `EpochMarker`:

```rust
use std::cell::RefCell;
use std::rc::Rc;

use bitcoin_epoch::{Epoch, EpochGuard, EpochMarker};

struct TxEntry {
    id: TxId,              // your transaction identifier type
    parents: Vec<TxId>,    // graph edges
    children: Vec<TxId>,
    epoch_marker: EpochMarker,
}

struct Mempool {
    epoch: Rc<RefCell<Epoch>>,
    entries: HashMap<TxId, TxEntry>,
}
```

### Initialisation

```rust
impl Mempool {
    fn new() -> Self {
        Self {
            epoch: Rc::new(RefCell::new(Epoch::default())),
            entries: HashMap::new(),
        }
    }
}
```

### Traversal with deduplication

```rust
impl Mempool {
    /// Depth-first traversal over descendants, without revisiting entries.
    fn walk_descendants(&mut self, root: &TxId) {
        // 1. Start a fresh epoch for this traversal.
        with_fresh_epoch!(self.epoch);

        // 2. Inner recursive/iterative walk.
        self.walk_descendants_impl(root);
    }

    fn walk_descendants_impl(&mut self, txid: &TxId) {
        let epoch_rc = self.epoch.clone();
        let epoch_ref = epoch_rc.borrow();

        let Some(entry) = self.entries.get_mut(txid) else { return; };

        // 3. Deduplicate with O(1) epoch check instead of a HashSet.
        if epoch_ref.visited(&mut entry.epoch_marker) {
            return; // already visited in this traversal
        }

        // 4. Process the entry.
        self.process_entry(entry);

        // 5. Recurse or iterate through descendants.
        let children: Vec<TxId> = entry.children.clone();
        drop(epoch_ref); // avoid borrow conflicts if needed

        for child in children {
            self.walk_descendants_impl(&child);
        }
    }

    fn process_entry(&self, _entry: &TxEntry) {
        // domain-specific logic
    }
}
```

Properties of this approach:

- No `HashSet<TxId>` or allocation-heavy containers are needed for visited tracking.
- Each entry carries its own `EpochMarker`, which is just a `u64`.
- Concurrent traversals using the same `Epoch` are **not** supported; the guard enforces exclusivity via the `guarded` flag.

## Safety and invariants

The crate enforces several invariants via panics:

- `EpochGuard::new` asserts that the supplied `Epoch` is not already guarded, preventing nested guards for the same `Epoch` instance.
- `Drop for EpochGuard` asserts that the `Epoch` is guarded when the guard is dropped.
- `Epoch::visited` asserts that the epoch is guarded, i.e., it must be called within the dynamic scope of an `EpochGuard`.
- `Epoch::increment_epoch` uses `checked_add` and panics on overflow of the 64-bit epoch counter.

These panics are deliberate: they defend against subtle misuse leading to incorrect traversal deduplication.

In typical Bitcoin-style mempool usage, epochs will never approach `u64::MAX`, so overflow is a theoretical boundary rather than a practical concern.

## Concurrency model

The crate itself uses `Rc<RefCell<Epoch>>`, which is single-threaded. For multi-threaded contexts, you can wrap the `Epoch` in `Arc<Mutex<_>>` or similar, reimplementing the RAII guard semantics on top, or by using one epoch per thread.

The presence of annotations like `#[LOCKABLE]`, `#[SCOPED_LOCKABLE]`, `#[EXCLUSIVE_LOCK_FUNCTION]`, and `#[EXCLUSIVE_LOCKS_REQUIRED(*this)]` mirrors Clang thread-safety attributes from C++, primarily for static analysis tooling and documentation. They express the logical locking discipline:

- `EpochGuard` works like a scoped lock.
- `Epoch::visited` requires the epoch to be logically *locked* (i.e., guarded by `EpochGuard`).

These annotations do not, by themselves, provide synchronization; the caller is responsible for ensuring proper thread safety when sharing epochs across threads.

## Integration notes

- The crate is intentionally minimal and focused; it does **not** define the mempool structure, transaction IDs, or graph storage.
- Designed to integrate into existing Bitcoin-related Rust codebases or any project using graph traversals where high-performance deduplication is valuable.
- The API mirrors the semantics of the original Bitcoin Core implementation to ease porting of algorithms from C++.

## When to use this crate

Use `bitcoin-epoch` when:

- You maintain a large, mostly in-memory graph (mempool, dependency DAG, reachability graph, etc.).
- Traversals are frequent and deduplication cost is material.
- You can afford to store a small `EpochMarker` per node.

Do **not** use this crate when:

- You require unbounded overlapping traversals on the same epoch object.
- You cannot or do not wish to store extra state per node.

## License and provenance

- **License**: MIT
- **Edition**: Rust 2021
- **Repository**: <https://github.com/klebs6/bitcoin-rs>

This crate is part of a broader effort to provide high-fidelity Rust analogues of the Bitcoin Core internals.
