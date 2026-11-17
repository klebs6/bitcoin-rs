# LevelDB release readiness

Work remains on `release/leveldb-readiness062`. Original master, existing tags
and remote publication are unchanged. Release preparation uses the configured
real Git identity and preserves the [attribution correction record](leveldb-attribution062.md).

## Source and execution evidence

The baseline qualification tag identifies
`0214a455d9f49084dc093d281d4dcce42173de1e`. Focused 063-087 repairs retain their
own source/artifact/command bindings. Their overlapping counts are not added
into a new global test count. Full randomized stress and the whole harness
were not rerun by packaging work.

096 created and inspected 112 development-version archives, including selected
source, normalized manifests, licenses/notices and README members. Their
independent default-feature consumer resolved the archive paths and the
intended registry graph, then completed cargo-check with 113/113 expected
library/proc-macro artifacts and no compiler errors on the recorded NixOS
x86_64 host. Original shared Core manifests/configuration/lock retained their
Git native selections; the tracked publication projection selects registry
libevent 0.2.0/libevent-sys 0.4.0 explicitly. See
[the dependency boundary](leveldb-registry-libevent096.md).

These results cover the measured source and graph. Cargo-check is not final
executable linking or runtime execution of published artifacts. Stable-Rust
MSRV, other platforms and all-feature qualification are not established.

## Package and attribution scope

The prepared local set has 112 packages. The umbrella's full declared local
runtime/build closure has 87; harness has 88; the test umbrella has 102. All 112
have a dependency-first order, including the seven published development
edges. Optional/target edges remain visible. A path-only repository fixture
omitted from the DBImpl published dev table remains repository-only.

107 LevelDB READMEs and five shared READMEs were reviewed against source.
101 updates license sentences to match the source-specific declarations.
[The attribution review](leveldb-source-attribution101.md) and its per-file
ledger cover 1251 code-like members. Package notices retain applicable upstream
headers and full license texts; original authorship/history are unchanged.

## Registry survey and remaining publication gates

099 observed 53 existing names, all directly listed under klebs6, and 59 names
not found. All 195 observed existing versions were 0.1.x. The proposed coherent
0.2.0 line was absent in that public snapshot. This reserves no name/version
and verifies no authenticated publishing permission. The public ownership
survey and final publishing authorization remain separate. Preparation needs
no registry credentials.

1. Finalize the public set and release versions with exact internal
   requirements. Existing development versions remain unchanged by101.
2. Combine final-version packaging with these metadata/NOTICE changes in one
   final archive inspection and independent-consumer qualification. Retained
   096 archives remain evidence for their original bytes, not future versions.
3. Record the supported toolchain/platform/features and the exact checked
   operations. Keep unmeasured final linking/runtime claims separate.
4. Finish release notes and review the concrete source/artifact/tag plan before
   promotion, push or publication. Use the existing real identity and a chosen
   authenticated publication host; preparation reads no credentials.

No whole-suite rerun is required for the 101 documentation/metadata delta.
Storage080 still binds builds to the fixed warm target, incremental disabled,
one job, unchanged 2 GiB reserve and original sampled admission/growth controls.
Source-only work uses the existing source guard and creates no build cache.
The 12 retired clean worktrees and four dirty held reservations remain protected.
