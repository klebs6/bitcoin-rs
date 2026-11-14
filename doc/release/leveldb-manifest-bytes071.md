# LevelDB manifest byte owners071

Parent: `7701604126d5eba59acb4b88b8e520c6ddbcc045` on the existing
`release/leveldb-readiness062` branch. This source needs its own guarded client
receipt. Prior qualifications retain their original source and profile scope.

## Source and compatibility

The five legacy named coding append sinks now accept `*mut Vec<u8>`, using the
existing `_bytes` helpers. Null coding destinations trigger an assertion before
dereference; VersionEdit preserves its existing null-output no-op. The
length-prefixed wrapper retains its void return and rejects an unrepresentable
length without appending. Its helper copies valid source bytes before growth,
including a Slice borrowing the previous destination allocation. The unused
`put_varint64_to_string` function is removed; no String compatibility sink remains.

`VersionEdit::encode_to` appends to a byte vector. DBImpl new-database creation,
VersionSet log-and-apply/snapshot writing, and Repairer descriptor writing own
their encoded MANIFEST records as byte vectors. The existing tags, deterministic
deleted-file ordering, append behavior and LevelDB wire format remain unchanged.

VersionSet compaction pointers own exact encoded internal-key bytes. Builder
application and input expansion copy those bytes without lossy Slice-to-text
conversion; compaction selection compares their borrowed bytes. Existing snapshot
writing does not emit compaction-pointer entries; this patch preserves that policy.
Comparator names, filenames, debug strings and other actual text remain strings.
These are deliberate prepublication Rust API changes. No C API entry point,
dependency, lockfile, package version or public release tag is changed.

The accompanying source regressions use valid allocated binary data and literal
wire expectations. Updated fixtures explicitly type their byte vectors. Existing
native test names and assertions remain present. No debugger, interposition,
allocator experiment, exploit development or arbitrary invalid-memory probe is
part of this repair.

## Focused qualification

Native/doctest selection is coding, versionedit, versionset, dbimpl and repair.
The existing harness receives one compilation-only `--no-run` check; no harness,
benchmark, full randomized or full-subsystem workload is run. The Snappy profiles
qualified070 are recorded as prior results, not relabeled as071 passes.

The offline/locked/single-job Cargo observer, exact executable/test identities,
finite resource guard, no-swap/one-CPU limit, disk floor and source-preservation
checks are unchanged. A repeated application admits only the exact child commit
and bytes, audits retained same-candidate evidence and runs failed/unexecuted
checks. Compiler diagnostics and successful earlier checks remain available on
failure. Fresh and retained counts are separate; budgets are never raised
automatically and there is no unrestricted fallback.

The controller uses the configured real Git identity on the existing release
branch. Master, tags, original user changes, held/retired worktree contents and
retained evidence remain preserved. There is no history rewrite, remote access,
publication or cache/worktree deletion. The approved `return-buffer` transports
the complete result.

Retrieval outputs, Env file reads, model/comparator/generator boundaries,
packaged consumers, license/notices, release versions, registry availability,
publish order and supported toolchain/features remain listed gates. See
[publication preparation](leveldb-readiness062.md).
