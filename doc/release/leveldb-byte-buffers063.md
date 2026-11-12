# LevelDB binary-owner repair063

Parent release candidate: `f2f364b048e05816715025cfc1c2998887e2754f` on
`release/leveldb-readiness062`. Original master remains
`0214a455d9f49084dc093d281d4dcce42173de1e`.

Publication remains held. This change addresses two binary owners, and does
not establish a cause for the earlier SparseMerge allocator abort.

## Changed ownership and compatibility

- `WriteBatch.rep` becomes `Vec<u8>`. Its header remains fixed64 little-endian
  sequence plus fixed32 little-endian count; record tags, varint lengths,
  payload bytes and insertion order are unchanged. The generated `rep`,
  `rep_mut` and `set_rep` accessors consequently change Rust types. This is an
  explicit Rust API change; the C-style handle representation and pointer/length
  interfaces are unchanged. No package version is selected or published here.
- Private `InternalKey.rep` becomes `Vec<u8>`, keeping the existing public
  construction, encode/decode, user-key, clear, set-from and debug signatures.
  `append_internal_key_to_vec` is added. Its encoding is the exact user bytes
  followed by fixed64 little-endian `(sequence << 8) | type`. Debug formatting
  remains escaped text. `set_from` first copies the new encoding, allowing its
  parsed input to borrow the current key buffer until that copy is complete.
- Five existing key test helpers use byte buffers instead of invalid binary
  Strings. Existing test names, attributes, workloads and assertions remain.
  Six batch and five key regressions add literal wire-format expectations,
  invalid UTF-8, NUL, high-bit sequence/count fields, varint length boundaries,
  callback ordering, owned copies, and the corrected self-borrowing paths.

Borrowed `Slice` views still require live storage and appropriate nonoverlap
during mutation. These repairs do not prove the safety of every raw-pointer API.
The internal batch append helper supports an identical source/destination
pointer; the C-style wrapper's reference construction has a separate aliasing
contract and is not qualified for self-append by that internal regression.

## Focused verification

Controller063 reuses the permanent, unchanged bounded subsystem runner and warm
Cargo target. It selects only `bitcoinleveldb-batch`, `bitcoinleveldb-key`,
`bitcoinleveldb-writebatch` and `bitcoinleveldbt-capi`; it runs their discovered
native tests and eligible doctests. The controller requires all eleven new
regression names in successful protected executions. Compiler/test evidence is
retained with the actual candidate commit. A checked-in source change is not a
claim that this client-side run has passed.

Builds use one job and a 4 GiB limit. Ordinary cases use a 512 MiB limit;
doctest compilation uses the existing 2 GiB envelope. Verified cgroup controls,
no swap, deadlines, output/data limits and private test directories remain in
force. There is no full subsystem or randomized-stress rerun, budget increase,
unrestricted fallback, debugger, allocator interposition, or external target.

The candidate is committed on the existing release branch with the real
configured author/committer and existing signing policy. Original master, tags,
dirty reservations, retired worktrees, caches and historical evidence are
preserved. Native failure retains the candidate and diagnostic receipt; it does
not promote or publish the change.

## Remaining work

The [owner inventory](leveldb-binary-owners063.json) distinguishes binary
owners, lossy data paths, byte-backed production code and textual values. It
is a source review, not a complete lifetime or synchronization proof.

1. Migrate coding sinks and connected block/table/manifest/compression owners
   while preserving fixed fields, varints, restart arrays, footer bytes and
   compression fallback semantics. The legacy String key append API remains
   pending until its external callers migrate.
2. Migrate DB retrieval outputs coherently through MemTable, Saver, Version,
   DBGet/DBImpl and the C API temporary; migrate iterator saved/seek buffers.
   Keep properties, filenames, status messages and debug output textual.
3. Correct lossy compaction-pointer, model, constructor and reverse-comparator
   paths with exact-byte and ordering tests. ModelDB point `get` is separately
   unimplemented and needs an explicit scope/implementation decision.
4. Finish registry/package-consumer, notices, feature/toolchain and release
   version checks from the [release preparation record](leveldb-readiness062.md).

BlockIter's production key buffer, active cache key identity, LogReader scratch,
filter/bloom output and StringSink/StringSource already use bytes. Their names
or test-only text helpers do not justify changing unrelated textual APIs.
