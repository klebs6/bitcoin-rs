# LevelDB Snappy byte owners070

Parent: `4652b7aebaab2e15bdad463534f3a8bf0215020b` on the existing
`release/leveldb-readiness062` branch. This new source needs its own guarded
client receipt. Historical qualification keeps its original source references.

## Ownership and compatibility

Both configurations of `snappy_compress` accept `*mut Vec<u8>` output. Enabled
compression completes all input reads into a separate compressed vector before
replacing that owner, including when input borrows its previous allocation.
No unchecked String conversion remains in the compressor. False leaves a live
output unchanged. Input/output raw pointers retain their validity contracts;
no arbitrary oversized allocations or invalid input buffers are manufactured.

TableBuilderRep owns compressed scratch as a byte vector. Its generated Rust
accessor types change accordingly. All seven captured direct compressor calls
and their buffers are updated, including the existing block-reader fixture and
three benchmark buffers. The existing read test uses16 valid binary bytes and
records whether its compressed path was actually reached. No production read
algorithm, file format, public C entry point, manifest, lockfile or version is
changed. These are deliberate prepublication Rust API changes.

Four new codec regressions cover literal raw Snappy bytes or disabled-output
preservation, empty input, an input view borrowing its previous output, and the
port's rejected null-input boundary. A new table-writer regression checks a
4111-byte binary block, fallback or actual compressed storage, compression type,
independent trailer CRC32C, decompression and scratch reset. Existing test names
and assertions are retained.

## Separate qualification profiles

Default native/doctest verification selects compat, tablebuilder and blockhandle.
The second profile selects those same three packages with the exact named
Snappy feature for each package. The shared permanent runner gains a narrow
`features070` API for those three choices, with features bound to compile/doc
commands and continuation history. Offline/locked/single-job commands and all
guard limits, snapshots, parsers and artifact bindings remain in force. Python
runner file mode remains executable. Default facade behavior is unchanged.

The codec's measured package features and the required writer/reader runtime
markers distinguish an enabled backend from an unsupported early return.
Profiles have separate source-bound receipts and histories. Counts are not
combined into a new all-features, platform-wide or full-subsystem claim.
Same-candidate continuations independently audit the matching profile and run
only its failed/unexecuted checks. Compiler diagnostics remain in failed
returns. No automatic retries or budget increases occur.

The harness and benchmark consumer packages receive guarded `--no-run`
compilation only. No benchmark, long randomized harness case, original full
randomized suite or full subsystem rerun is selected. Consumer compilation
runs once per invocation and contributes no native passes.

The controller changes only the existing release branch under the configured
real identity. It does not rewrite history, move master/tags, publish, access
remotes or delete worktrees/caches/receipts. `return-buffer` remains the approved
disposable transport. Legacy coding, manifest, retrieval and other pending
owners plus package/license/version/publish-order/toolchain gates remain; see
[publication preparation](leveldb-readiness062.md).
