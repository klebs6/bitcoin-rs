# bitcoinleveldb-snapshot

Snapshot handles and the internal sequence-number snapshot list for LevelDB.

`Snapshot` describes a database read boundary. Its semantic dispatch methods
identify supported snapshot implementations, optionally expose a sequence
number for read reconstruction, or clone an equivalent read handle. The default
implementation reports `Unsupported` and supplies no reconstruction handle.
An optional immutable byte-map capability supports reference-model payloads and
forwarding wrappers without a concrete-layout cast. A snapshot kind does not
by itself prove that this capability exists.

`SnapshotImpl` stores a sequence number and intrusive links. `SnapshotList`
maintains these nodes around a heap-allocated sentinel and exposes `new`,
`delete`, `oldest`, `newest`, and `empty`. Insert sequence numbers in
nondecreasing order. Delete only nodes allocated by that same list, and release
outstanding nodes before the list's lifetime ends.

These internal raw-pointer list operations do not provide an independently
owned database snapshot. Application callers should obtain and release
snapshots through their database implementation.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
