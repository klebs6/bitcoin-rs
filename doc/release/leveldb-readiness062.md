# LevelDB publication preparation

Status: source qualification is recorded; publication remains held for the
distribution checks below. The current work stays on
`release/leveldb-readiness062`. Original master, qualification tags and remote
publication are unchanged by this preparation.

## Qualified source and scope

The baseline `leveldb-subsystem-qualified-20261111` points to
`0214a455d9f49084dc093d281d4dcce42173de1e`. Its original receipt records
107 compiled packages, 2,041 accumulated native passes and 107 successful
doctest commands on its measured Linux/default-feature configuration.

Focused stages 063-087 subsequently addressed byte ownership and checked text
boundaries in batches, keys, iterators, blocks, tables, compression, manifests,
Env file reads, point reads, ModelDB/snapshots, generators and constructors.
Their receipts retain their own source/artifact/command bindings. Counts from
overlapping stages must not be added into a new subsystem-wide pass count.

Recent closures include getter082 (354 cumulative native identities),
model084 (181), generators085 (six), and constructor087 (70 affected-crate
identities: 49 retained and 21 fresh, plus four separate finite harness cases).
The constructor group has five successful doctest commands with zero doctest
cases. Harness/randomized consumer builds completed; the whole harness and
full randomized stress were not rerun by these stages.

Named Snappy profiles have their separate070 evidence. These records establish
their measured contracts, not universal feature coverage, arbitrary pointer
validity, memory safety or an explanation of the historical allocator abort.
See the [bounded testing guide](../testing/leveldb-subsystem.md) and the focused
source records in this directory.

## Public package boundary

The source inventory088 and actual Cargo file listings cover all 107 LevelDB
seeds and the 112-package declared local runtime/build closure. The closure
includes optional and target-specific local edges; dev edges are recorded
separately. It has 40 dependency-first layers and no local cycle.

The facade alone reaches 87 local packages. The85 component libraries plus
the facade reach109. Selecting all 107 seeds reaches 112. The additional shared
packages are `bitcoin-cfg`, `bitcoin-crc32c`, `bitcoin-derive`, `bitcoin-imports`
and `bitcoin-support`. Their names do not make them optional dependencies.
Selecting the final public package set remains a release decision.

The107 LevelDB READMEs were reviewed in062 and updated alongside later source
changes. Preparation089 corrects the five shared READMEs and their package
descriptions against the captured source. In particular, procedural annotations
do not enforce concurrency rules, event helpers are Rust stand-ins, and
`SecureString`'s boxed-object allocator does not cover its character buffer.

## Package notices and actual verification

Inventory088 selected every README but no license or notice file in any of
the 112 packages. Preparation089 adds package-local copies of the existing root
MIT notice, LevelDB/CRC32C upstream license texts, and an explanatory NOTICE.
The root MIT file, author metadata and historical Git authorship are preserved.
Primary-source URLs, capture date and normalized formatting are recorded in
NOTICE and the [packaging record](leveldb-package-preparation089.md).

These additions require their own guarded Cargo file-selection result.
Selection is not .crate creation, extraction, or compilation. A complete
source-specific attribution review and accurate final SPDX declarations remain
required; retaining additional notices does not itself establish compliance
for every file or external dependency.

## Remaining release gates

1. Finalize the public package set and versions, check registry availability,
   and bind the dependency-first publication plan to those exact versions.
   Existing development versions are not an approved publication plan.
2. Create and inspect the actual .crate archives, including normalized
   manifests, source, licenses/notices, README links and build-script inputs.
3. Verify a consumer outside the workspace against those artifacts and the
   intended registry dependencies. `bitcoin-imports` and `bitcoin-support`
   inherit a Git fork of libevent/libevent-sys; equivalence to registry versions
   is not established by workspace builds or package listings.
4. Verify detached build-script behavior. `bitcoin-imports/build.rs` can scan
   its parent workspace and tests `CARGO_PUBLISH`; the surrounding environment
   must not be assumed to reproduce a workspace build.
5. Finalize per-source license/attribution mapping and package SPDX metadata,
   preserving upstream conditions, notices and contributor provenance.
6. Record and verify the supported toolchain/platform/feature matrix. The
   measured088 tools are rustc 1.97.0-nightly (9eb3be26b,2026-05-18), Cargo
   1.97.0-nightly (4d1f98451,2026-05-15), host x86_64-unknown-linux-gnu on
   NixOS. No stable-Rust MSRV, AArch64 result, or all-feature result is implied.
7. Finish release notes and review the concrete candidate before promotion,
   tagging and publication. Use the configured real Git identity and preserve
   the existing qualification tags.

The known broad `bitcoin-imports` surface includes unrelated `Broken` helper
methods that call `todo!()`; the shared README now says so. Publication of that
support surface needs an explicit scope decision. This record does not turn
the unfinished wider Bitcoin Core implementation into a qualified node API.

## Resource and reservation lifecycle

Storage080 binds subsequent work to the fixed warm Cargo cache, with
incremental compilation disabled, a 2 GiB reserve and the unchanged sampled
session/cache growth controls. It is an admission/stop monitor, not a global
filesystem quota. No per-repair cache or blanket native rerun is required
for this documentation and packaging step.

Preparation062 retired 12 clean merged repair refs by detaching their worktrees
at the same tips. Files, indexes and retained evidence remain protected.
Four dirty reservations remain held: `repair/leveldb-final-crates-runner033`,
`repair/leveldb-randomized-controlled023`, `repair/leveldb-writablefile-shared`
and `repair/leveldb-sparsemerge`. Merged commits do not make their directories
or caches disposable.

[Repository and support](https://github.com/klebs6/bitcoin-rs) ·
[Attribution correction record](leveldb-attribution062.md)


## Registry/isolated-workspace boundary091

Archive rehearsal090 reached its 150-second process limit without creating an
archive. The source guard recorded repeated memory-high pressure, with no OOM
kill. That receipt does not establish the underlying Cargo/dependency cause.
The controller also lost partial Cargo output on timeout;091 retains streamed
stdout/stderr and termination status. No protected native tests were rerun.

Preparation091 replaces the inherited bare wildcard requirements for sqlite,
sqlite3, sqlite3-sys and signal-hook with the exact versions already present
in the unchanged root lockfile. It changes no package version or Rust source.
The registry boundary is checked first with only libevent 0.1.0 and
libevent-sys 0.2.6. When explicitly requested, that small resolver may download
public registry metadata/source; it executes no downloaded code.

Conditional packaging uses a recorded 112-member workspace projection outside
both repositories. Member/source bytes remain bound to the qualified source;
only referenced inherited workspace aliases are retained. Its lockfile starts
from the original lock bytes and is intentionally derived in that new scope.
Archive and detached-resolution commands remain offline. No original lock,
Git attribution, resource budget or Cargo target directory is replaced.
See [the boundary record](leveldb-registry-boundary091.md) and the client
receipt for actual progress. A successful resolution is not API/build or
publication qualification.

## Packaging development cycle093

The `bitcoinleveldb-dbimpl` development dependency on the local DBConstructor
fixture is path-only. The runtime/build graph, local path, features, package
versions, lockfile, and Rust source remain unchanged. Cargo omits this fixture
from the published dev-dependency table. Tests requiring it are repository-only;
standalone packaged test execution is not qualified.

The093 controller preserves and inspects retained092 staged archives, then
performs one fresh archive-only batch. No native test qualification is added.
Publication remains pending the actual archive and independent-consumer results
and the other release gates recorded above.
