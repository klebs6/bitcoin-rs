# LevelDB 0.2.0 publication projection

The selected publication set is all112 packages in
[the version and dependency plan](leveldb-publication-plan102.json).
Each projected package is0.2.0. Existing internal published requirements use
`version = "0.2.0"`, Cargo's compatible0.2.x range. Optional, target, build and
published development edges retain their features and other semantics.
DBImpl's path-only DBConstructor development fixture remains repository-only.

The tracked shared Core manifests, package versions and root lock are kept
unchanged. The readable tracked projector creates detached publication
manifests, records original and derived hashes, and changes only selected local
identities/references in retained release locks. The registry libevent pair
remains the qualified096 pair. External lock identities/checksums are preserved.
`Cargo.toml.orig` contains the declared derived manifest; generated archives
do not contain fabricated Git metadata. The source commit and per-file
derivation ledger are the provenance record.

102 qualifies newly generated archives only after exact file/source/license/
README/dependency inspection and an independent consumer check. That consumer
selects exact`=0.2.0` from all112 verified archive directories and preserves the
312 qualified external registry identities. Its successful cargo-check is a
library metadata/code check, not final executable linking or runtime coverage.
No native test suite is rerun. Existing runtime qualification remains scoped
to its measured source/toolchain, platform and options.

The supported measured environment is the retained nightly Rust toolchain on
x86_64 Linux under the existing Nix shell and locked default library graph.
Stable MSRV, other platforms, all features and final executable linking are
unmeasured. The archive/consumer return must record the actual outcome; this
plan does not itself claim success, reserve registry names or publish.

Before promotion/tagging/push/publication, review release notes and exact final
source/artifact hashes, refresh registry availability and select an
authenticated host using the owner's real account and configured Git identity.
The102 controller reads no credentials and performs none of those actions.
