# bitcoinleveldb-snapshot

Low-level snapshot list implementation for the LevelDB-inspired storage engine used in `bitcoin-rs`.

This crate provides the internal snapshot representation for a Bitcoin-oriented fork/port of LevelDB. It exposes:

- An abstract `Snapshot` trait (immutable database view)
- A `SnapshotImpl` doubly-linked node storing a `SequenceNumber`
- A `SnapshotList` that maintains a heap-pinned sentinel and a monotonically ordered list of snapshots

It is primarily intended for internal consumption by the broader `bitcoinleveldb-*` ecosystem rather than for direct end-user use. Nevertheless, the code is fully documented and usable as a general-purpose, pointer-based snapshot list.

## Conceptual model

In an LSM-based key–value store (such as LevelDB and its descendants), each mutation is assigned a monotonically increasing `SequenceNumber`. A snapshot is a logical view of the database at a specific sequence number. Reads performed under a snapshot must see all writes with `sequence_number <= snapshot_sequence`, and none of the later writes.

This crate implements the bookkeeping for those snapshots:

- Snapshots are represented as nodes in a circular doubly-linked list.
- The list has a heap-allocated sentinel head whose address is stable, even if the `SnapshotList` itself moves.
- Snapshots are ordered by their `SequenceNumber` and appended in monotonically non-decreasing order.
- Raw pointers (`*mut SnapshotImpl`) are used instead of `Rc`/`Arc` to closely match LevelDB's original design and to minimize overhead.

### Safety and invariants

The design relies on several invariants, enforced by debug assertions where possible:

- The sentinel head is always self-linked: `head.prev == head` and `head.next == head` for an empty list.
- For a non-empty list, `head.next` points to the *oldest* snapshot and `head.prev` to the *newest* snapshot.
- `SnapshotList::new` must be called with monotonically increasing `SequenceNumber`s. A debug assertion checks this when the list is non-empty.
- `SnapshotList::delete` must never be called with a pointer to the sentinel head.
- In non-`NDEBUG` builds, each `SnapshotImpl` contains a back-pointer to its owning `SnapshotList`, and `delete` validates that pointer.

All list links are manipulated via raw pointers under `unsafe` blocks. From the perspective of the public API, the operations are logically safe provided that:

- Callers do not hold onto snapshot pointers after they have been passed to `delete`.
- Snapshots are not concurrently modified (they represent an immutable DB view, but their list links are mutated internally by the list operations).

## Crate structure

At a high level, the crate defines:

```rust
pub trait Snapshot {}

#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SnapshotImpl {
    prev:            *mut SnapshotImpl,
    next:            *mut SnapshotImpl,
    sequence_number: SequenceNumber,
    list:            *mut SnapshotList,
}

#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SnapshotList {
    head: Box<SnapshotImpl>,
}
```

Where `SequenceNumber` is a type alias provided elsewhere in the `bitcoinleveldb` stack (typically `u64`).

`SnapshotImpl` implements `Snapshot`, allowing it to be passed through the higher-level API as an opaque handle.

## Usage

This crate is designed to be used by the database engine, not by application code directly. In practice, a DB will:

1. Maintain a `SnapshotList` inside the DB state.
2. On `DB::GetSnapshot` (or equivalent), call `SnapshotList::new(current_sequence_number)` to allocate and link a new snapshot, and return it as a `&dyn Snapshot` to the caller.
3. On `DB::ReleaseSnapshot`, cast the API-level snapshot reference back to the internal `SnapshotImpl` pointer and call `SnapshotList::delete`.

### Creating and managing snapshots

```rust
use bitcoinleveldb_snapshot::{Snapshot, SnapshotImpl, SnapshotList};

fn example() {
    // Initialize an empty snapshot list with a heap-pinned sentinel.
    let mut list = SnapshotList::default();
    assert!(list.empty());

    // Current DB sequence number.
    let seq1: SequenceNumber = 42;

    // Create a new snapshot. This returns a raw pointer owned by the list.
    let snap1: *mut SnapshotImpl = list.new(seq1);
    assert!(!snap1.is_null());
    assert!(!list.empty());

    unsafe {
        // Read the sequence number via the generated getter.
        let sref: &SnapshotImpl = &*snap1;
        assert_eq!(*sref.sequence_number(), seq1);
    }

    // Suppose the DB advances and we create another snapshot.
    let seq2: SequenceNumber = 100;
    let snap2: *mut SnapshotImpl = list.new(seq2);

    // The oldest snapshot is `snap1`, and the newest is `snap2`.
    let oldest = list.oldest();
    let newest = list.newest();
    assert_eq!(oldest, snap1);
    assert_eq!(newest, snap2);

    // When snapshots are no longer needed, they must be deleted.
    // This unlinks and deallocates them.
    list.delete(snap1 as *const SnapshotImpl);
    list.delete(snap2 as *const SnapshotImpl);

    assert!(list.empty());
}
```

Be very careful to avoid use-after-free: after `delete`, the pointer is invalid.

### Interoperating with higher-level APIs

The trait `Snapshot` is intentionally minimal:

```rust
pub trait Snapshot {}
```

`SnapshotImpl` implements `Snapshot`, enabling APIs like:

```rust
fn db_get_snapshot(list: &mut SnapshotList, seq: SequenceNumber) -> *const dyn Snapshot {
    // Implementation sketch; actual code may differ in the real DB layer.
    let impl_ptr: *mut SnapshotImpl = list.new(seq);
    impl_ptr as *const dyn Snapshot
}

fn db_release_snapshot(list: &mut SnapshotList, snap: *const dyn Snapshot) {
    // In the real engine, this downcast is usually done using knowledge of
    // the concrete representation, often via `SnapshotImpl` internals.
    let impl_ptr = snap as *const SnapshotImpl;
    list.delete(impl_ptr);
}
```

The public `Snapshot` interface is intentionally opaque: snapshot semantics (which sequence number, which files, etc.) are upheld entirely by the surrounding engine code. This crate focuses on ownership and ordering of those internal snapshot structures.

## Design details

### Heap-pinned sentinel

The `SnapshotList` stores a `Box<SnapshotImpl>` sentinel `head`. Its address is stable, even when the `SnapshotList` itself is moved in memory. This allows the list nodes to keep raw pointers to `head` without worrying about self-referential struct invalidation.

On `Default` construction:

- A `SnapshotImpl` with `sequence_number = 0` is allocated and boxed.
- `head.prev` and `head.next` are set to point to `head` itself.

The sentinel is never exposed as a public snapshot and must never be passed to `delete`.

### Oldest and newest snapshots

The `oldest` and `newest` methods work as follows:

- `oldest` returns `head.next`, i.e., the first real node after the sentinel.
- `newest` returns `head.prev`, i.e., the last real node before wrapping back to the sentinel.

Both methods assert that the list is non-empty in debug builds.

### Insertion (`new`)

`SnapshotList::new(sequence_number)` performs:

1. When the list is not empty, assert that the new sequence is `>=` the current newest sequence.
2. Allocate a `SnapshotImpl` with the given sequence.
3. In non-`NDEBUG` builds, write its `list` back-pointer.
4. Insert the new node just before `head`, updating the adjacent `prev`/`next` pointers accordingly.

### Deletion (`delete`)

`SnapshotList::delete(snapshot)` performs:

1. Return early if `snapshot` is null.
2. Assert that `snapshot` is not the sentinel head.
3. In non-`NDEBUG` builds, assert that the snapshot's `list` matches `self`.
4. Relink `prev.next` and `next.prev` to bypass the node.
5. Clear the node's `prev`, `next`, and (non-`NDEBUG`) `list` fields.
6. `drop(Box::from_raw(snapshot_mut));` deallocates the node.

The caller must ensure that `snapshot` is a valid pointer to a node currently linked into this list, and that no further dereferences occur after deletion.

## Concurrency

Snapshots as logical database views are immutable: higher-level code must treat `Snapshot` as read-only once published. The list itself, however, is *not* internally synchronized. The crate does **not** perform any locking; it assumes that the owner (`DB` or similar) enforces appropriate synchronization.

Typical patterns:

- Guard all snapshot list manipulations (`new`, `delete`) with a mutex protecting the DB state.
- Allow concurrent readers to use snapshots through immutable references once `DB::GetSnapshot` has returned them.

## Logging and diagnostics

The implementation uses macros such as `debug!`, `trace!`, and `warn!`. These are typically provided by the `log` crate or a similar facade. They offer insight into:

- Creation and deletion of snapshots
- Pointer topology and list structure changes
- Empty vs non-empty list checks

In performance-sensitive deployments you may choose a logging configuration that elides these at compile time.

## Relationship to `bitcoin-rs`

This crate lives in the `bitcoin-rs` repository:

- Repository: <https://github.com/klebs6/bitcoin-rs>

It is one piece of a broader system that re-implements or adapts LevelDB for use as the backing store for Bitcoin-related data structures. The snapshot system is critical to providing consistent point-in-time reads while compactions, memtable flushes, and other background operations occur.

## License

This crate is distributed under the MIT license. See the repository for details.
