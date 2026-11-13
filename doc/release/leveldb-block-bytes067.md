# LevelDB block byte owners067

Parent: `39a970edba1d905f5c6c7f06377809605187d134` on the existing
`release/leveldb-readiness062` branch. Master and its qualification tag remain
unchanged. This source change has a separate protected client receipt; the
parent's 164 affected native passes do not qualify this new source.

## Change and compatibility

`BlockBuilder.buffer` and `last_key` are byte vectors, not UTF-8 strings.
`buffer_bytes()` replaces `buffer_string()`; generated buffer/last-key
getters/setters expose byte vectors. `buffer_as_mut_vec()` remains available
with a safe implementation. `add` copies any input borrowing a mutable builder
allocation before appending, retains strictly increasing comparator order, and
preserves the shared/unshared/value varints, key/value bytes, restart offsets
and little-endian restart trailer. `finish()` still returns a borrowed Slice;
its validity ends at a backing-buffer mutation, reset or owner drop.

`BlockConstructor.data` likewise owns encoded block bytes as a byte vector.
`data_bytes()`/`data_bytes_mut()` replace the String accessors. Its text-keyed
KVMap input is unchanged; this repair does not turn that fixture into a general
binary input model. Rebuild only after dropping every iterator borrowing the
old block or comparator.

Coding adds explicit `put_fixed32_bytes`, `put_fixed64_bytes`,
`put_varint32_bytes`, `put_varint64_bytes`, and
`put_length_prefixed_slice_bytes` helpers. They append to `&mut Vec<u8>`.
The length-prefixed helper rejects unrepresentable lengths before inspecting
the payload and snapshots it before destination growth. Legacy String sinks
remain unchanged and remain release blockers for their binary callers.

These are prepublication Rust API changes. The C pointer/length interface and
the LevelDB block format are unchanged. No package version is chosen here.
The constructor's test dependency uses the already locked local
`bitcoinleveldb-iteratorinner`; only that dependency edge is added to Cargo.lock.

## Focused verification

Select coding, blockbuilder, blockconstructor, blockiter and tablebuilder for
native/doctest verification. Compile the harness consumer separately with the
same guarded offline Cargo observer and `--no-run`; do not execute its long
ignored randomized workloads. Verify source snapshots around that build and
record compilation separately from native test passes. This cached compilation
command runs once per invocation, including on native-verification continuations.
No full subsystem or full randomized workload is selected. Exact new test
identities are in the accompanying owner inventory.
They check literal wire bytes, varint boundaries, binary payloads, input views
borrowing previous storage, restart points, reset/rebuild, and block iteration.
All input slices refer to allocated readable bytes; no hostile allocator or
debugger experiments are part of this workflow.

Use the existing finite resource guard and exact executable/test bindings.
On repeats, recognize only the exact one-parent commit with the expected
committed blobs, working bytes and configured real attribution. Audit retained
same-candidate results before running only failed or unexecuted checks. Keep
historical and fresh counts separate; never rewrite a failed historical receipt.

No history rewrite, master/tag update, publication, remote operation or deletion
of reservations, caches, parked artifacts or evidence is performed.

## Remaining release gates

Table handle/footer/compression buffers, manifest records, point-get output
chains, compact pointers and other pending owners still require review and
coherent byte migration. ModelDB/helper limitations, packaged consumers,
licenses/notices, release versions, publish order and the toolchain/feature
matrix also remain. See [publication preparation](leveldb-readiness062.md).
