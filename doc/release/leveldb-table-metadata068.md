# LevelDB table metadata byte owners068

Parent: `af99f27a8988a36cd7af264a4b28d7780e67eda0` on the existing
`release/leveldb-readiness062` branch. Master and its qualification tag remain
unchanged. Source068 requires its own protected client receipt; the parent's
105 native passes, five doctest commands and harness compilation retain their
original067 source provenance.

## Source and format

`BlockHandle::encode_to(*mut Vec<u8>)` appends the offset and size as exact
varint64 bytes. Both fields must be initialized; `u64::MAX` remains the
uninitialized sentinel. The largest initialized value is `u64::MAX - 1`.
`Footer::encode_to(*mut Vec<u8>)` stages its two handles, pads their joint
region to 40 bytes, adds the eight-byte little-endian table magic, and appends
exactly 48 bytes after any existing binary prefix. A byte vector can own every
encoded result without imposing Rust String's UTF-8 invariant.

The table builder's pending/final index handles, filter metaindex handles and
footer locals use byte vectors. Every captured direct caller of the changed
encoder signatures is updated, including the handle/footer decoder fixtures.
Existing test names and assertions are retained. The Rust output-pointer types
change before publication; no package version is chosen and no public C
pointer/length entry point is changed. The LevelDB wire format is unchanged.

New regressions compare literal varints, binary prefixes, footer padding/magic,
and encoding boundaries. The table regression writes three uncompressed blocks
with arbitrary binary keys/values, independently reads their entry boundaries,
decodes each index handle and filter handle, and checks the final footer and
recovered payloads. Every supplied Slice refers to valid allocated data.

## Focused qualification

Select blockhandle, footer, tablebuilder, table and tableconstructor for native
and doctest verification. Compile the harness consumer separately with the
existing bounded offline Cargo observer and `--no-run`. Its long ignored
randomized workloads are not executed. Compilation has its own evidence and
source checks and contributes no native passes. That cached compile command
runs once per invocation, including native-verification continuations.

Use the unchanged finite resource guard and exact executable/test binding.
Repeat applications recognize only the exact committed file set, parent,
working bytes and configured real identity. Audit retained same-candidate
checks before running only failures or unexecuted checks. Keep fresh and
historical counts separate. No full subsystem/randomized rerun is selected.

No master/tag update, identity override, history rewrite, remote operation,
publication, or worktree/cache/evidence deletion is performed.

## Remaining gates

Optional Snappy compression scratch and its legacy String output remain
pending together with feature-enabled qualification. Legacy coding sinks,
manifest records, point-get outputs, compact pointers and other listed owners
still need migration/review. Model/helper boundaries, packaged consumers,
licenses/notices, versions, dependency publish order and toolchain/feature
support remain release gates. See [publication preparation](leveldb-readiness062.md).
