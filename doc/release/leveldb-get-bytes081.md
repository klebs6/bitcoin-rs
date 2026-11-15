# LevelDB byte-preserving point reads081

Parent: `7aad31b99cb309a3229de8e05c58bec0a6b4c7d5`, existing
`release/leveldb-readiness062` branch. This source requires its own guarded client
qualification. Earlier receipts retain their original source/profile scope.

## Read surface and compatibility

`MemTable::get_bytes`, `Saver.value`, `Version::get_bytes` and
`DBGet::get_bytes` use byte vectors. DBImpl reads into temporary byte storage and
commits the caller output only after an OK result, after releasing read refs and
its mutex. A miss, tombstone or other error leaves the DB output unchanged.

Existing text `get` signatures remain. DBImpl inherits DBGet's strict UTF-8
adapter; MemTable and Version provide equivalent checked adapters. Invalid
value text reports Corruption and leaves its String output unchanged. Direct
legacy MemTable tombstone lookups still clear the text output, preserving their
existing contract. Direct MemTable status on a found value/miss remains as before.

The new DBGet byte method defaults to NotSupported so an existing text-only
implementation remains source-compatible without claiming binary capability.
ModelDB's old getter is still unimplemented; its map/iterator/point-read repair
remains a separate publication gate. Third-party DB implementations must opt in
to get_bytes. SaverBuilder's internal output field now takes *mut Vec<u8>.

The C-style leveldb_get pointer/length interface is unchanged. These remain
Rust functions; this stage does not introduce C ABI exports. It copies Vec bytes into
libc allocation, which remains released by leveldb_free. A found empty value gets
a non-null freeable allocation with length zero, distinguishable from NotFound.
Serialized key/value formats and persistent comparator identities are unchanged.
This change cannot reconstruct data that an older lossy path already discarded.

## Qualification and storage

Seven new cases cover strict UTF-8/default capability, memtable sequence/deletion
semantics, all256bytes, empty values, a valid Saver destination view, and a real
C-API lifecycle through snapshots, deletion, forced SST creation and reopening.
The explicit SST assertion prevents treating log-only recovery as table coverage.
No hostile pointers, debugger/interposition, exploitation or external systems are
involved; tests use owned ordinary allocations and private test directories.

Only six affected crates run native/doc verification: dbinterface, memtable,
versionsetutil, version, dbimpl and db. Harness, recovery and CAPI test consumers
are compiled with --no-run; their native workloads and doctests are not claimed.
Unchanged subsystem and full randomized stress tests are not rerun.

The exact installed storage080 helper and policy bind verification to warm033,
incremental0, the existing profile/CPU/memory/time budgets, a fixed lifetime cache
ceiling and a finite session growth allowance. No new per-repair target or silent
policy/budget increase is permitted. Telemetry accompanies the client receipt.
This sampled gate is not a global filesystem quota or automatic reclamation.

Remaining gates include ModelDB/KVMap and generator/fixture binary semantics,
package-content/license/consumer validation, supported toolchain/features,
version/publication ordering and final release review. No publication-ready or
universal memory-safety/full-coverage claim is made.
