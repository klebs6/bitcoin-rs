# LevelDB publication preparation

Baseline: `leveldb-subsystem-qualified-20261111`, commit
`0214a455d9f49084dc093d281d4dcce42173de1e`.

**Status: publication is held pending the source and packaging checks below.**
The baseline records 107 compiled packages, 2,041 accumulated native passes and
107 successful doctest commands on the recorded Linux/default-feature setup.
Those results retain their original run/source provenance; they do not prove
all-feature coverage, arbitrary binary-data correctness, or memory safety.

## This preparation

- Review all 107 crate READMEs against captured source and manifests: 85
  `bitcoinleveldb-*` components, 21 `bitcoinleveldbt-*` test/support packages,
  and the `bitcoin-leveldb` facade. Replace placeholders and false APIs, keep
  useful crate-specific contracts and source links, and avoid unverified examples.
- Give local path dependencies explicit version requirements matching their
  existing package versions. No package version is bumped and no registry
  availability is inferred. The complete recorded local runtime/build closure
  contains 112 packages; the facade alone reaches 87, including two test-named
  support packages. Selecting all 85 component libraries plus the facade reaches
  109 local packages because some component crates aggregate test infrastructure.
  Package names alone do not define what can be omitted.
- Correct package descriptions/repository links, remove fabricated placeholder
  author entries, and remove the invalid `<none>` address from legacy author
  names. Preserve upstream author metadata and source notices. Do not assign
  authorship to a guessed identity.
- Repair Arena backing-allocation ownership: keep the exact allocation layout
  in a private owner, request the documented alignment, reject zero requests,
  and check aligned-size arithmetic. Regression coverage accompanies the repair;
  the protected targeted result is recorded separately.
- Journal retirement of clean merged repair reservations at their exact tips.
  Keep their worktree files, indexes, caches and evidence. Hold dirty worktrees.

## Remaining release gates

1. **Binary representation.** The [focused063 follow-up](leveldb-byte-buffers063.md)
   changes WriteBatch and private InternalKey storage to byte vectors. Its
   protected result is recorded separately. The [iterator066 follow-up](leveldb-iterator-bytes066.md)
   migrates iterator scratch/saved owners and the raw key append sink. Its
   separate066 client receipt records 164 affected native passes and five
   successful doctest commands. The [block067 follow-up](leveldb-block-bytes067.md)
   migrates builder keys/block bytes and constructor backing storage, with byte
   coding append helpers. Its separate067 client receipt records 105 affected
   native passes, five doctest commands, and compilation-only harness compatibility.
   The [table metadata068 follow-up](leveldb-table-metadata068.md) migrates block
   handle/footer encoding and their table-builder metadata outputs. The069
   fixture correction qualified that group with 111 accumulated native passes
   (94 audited historical,17 fresh), five doctest commands and compilation-only
   harness compatibility. The [compression070 follow-up](leveldb-snappy-bytes070.md)
   migrates Snappy output/scratch. Its070 client receipt records separate
   default (74 native, three doctest commands) and named Snappy-enabled
   (77 native, three doctest commands) profiles, plus compilation-only harness
   and benchmark compatibility. The [manifest071 follow-up](leveldb-manifest-bytes071.md)
   migrates the remaining coding append sinks, manifest records and compaction
   pointers; its separate071 receipt records318 native passes and five successful
   doctest commands (zero doctest cases), plus compilation-only harness compatibility.
   The [Env file076 follow-up](leveldb-env-file-bytes076.md) adds exact byte reads
   and whole-buffer UTF-8 validation at the text-file boundary; its076 receipt
   records136 native passes, two successful doctest commands with zero cases, and
   compilation-only harness/recovery compatibility. Retrieval outputs
   and other listed owners still need coherent byte-buffer migration. Rust `String` requires valid UTF-8; binary
   storage and lossy conversion remain source-level release blockers, including
   outside optional Snappy support.
2. **Known helper limits.** ModelDB's point `get` path and its text-keyed data
   paths remain gates. The [reverse077 follow-up](leveldb-reverse-bytes077.md)
   replaces lossy reversal with exact byte ordering and byte-output helpers.
   It uses the distinct persistent name `bitcoinrs.ReverseBytewiseComparator.BytesV1`:
   the former component's binary ordering is incompatible. The separate harness
   comparator is unchanged. This source still needs its own guarded result.
3. **Packaged consumer.** Inspect each intended `.crate` file, license/notices,
   examples and includes; establish registry dependency availability and publish
   order; verify a consumer outside the workspace under bounded resources.
   Workspace path resolution and version declarations are not package verification.
4. **Features/toolchain.** The current source uses nightly features. Record an
   explicit supported toolchain and test matrix; do not manufacture an MSRV or
   imply that default-feature results exercised optional features.
5. **Release identity/version.** Choose the release version and public package
   set, finish release notes, and make the final tag with the configured real
   Git identity. Do not silently move the qualification tag or publish artifacts.

The README/publication pass does not require repeating the entire native test
inventory. The preparation062 Arena change has its own targeted result. The focused063
binary-buffer follow-up checks batch, key, writebatch and C API crates. Later
owner and package changes need their own targeted checks. See the
[bounded testing guide](../testing/leveldb-subsystem.md).

## Repair reservation lifecycle

The capture found 16 merged repair refs. Twelve attached worktrees were clean. Preparation062 retired their refs after
verifying the exact tip, master ancestry, clean status and unchanged index. Detaching at the same commit preserves the
worktree and its data. A retirement receipt records actual changes and reflog
copies. Historical one-shot mutation-controller branch reservations are then
retired; their old branch-name preconditions should not be replayed as if active.

Four reservations have dirty/staged/untracked work and remain held:
`repair/leveldb-final-crates-runner033`,
`repair/leveldb-randomized-controlled023`,
`repair/leveldb-writablefile-shared`, and `repair/leveldb-sparsemerge`.
No worktree, cache, receipt, parked artifact, or directory is disposable merely
because its commits are merged.

## Sources and support

- [Repository and issue tracker](https://github.com/klebs6/bitcoin-rs)
- [Cargo packaging rules](https://doc.rust-lang.org/cargo/commands/cargo-package.html)
- [Rust String representation](https://doc.rust-lang.org/std/string/struct.String.html)
- [Attribution correction record](leveldb-attribution062.md)


## Point-read boundary081

The [byte-preserving getter stage](leveldb-get-bytes081.md) adds binary DBImpl
and C-API point reads with checked text adapters. Its separate client receipt is
required. The earlier comparator sign correction078 qualified two fresh library
cases and retained ten unchanged integration cases at its own source boundary.
Storage080 is activated; later repair controllers must bind its exact helper
and policy before verification. ModelDB and other listed release gates remain.
