# LevelDB registry dependency boundary

The shared repository workspace continues to use its selected libevent Git
fork. Five wider Core crates directly select that fork too. This release
change does not edit those manifests, the root manifest/configuration or lock.

The detached112-member LevelDB publication workspace explicitly replaces two
native selections in its workspace aliases and two in bitcoin-support:
libevent =0.2.0 and libevent-sys =0.4.0, from checksummed crates.io sources.
`contrib/release/leveldb-registry-projection096.py` records the readable,
deterministic transformation. It changes no Rust source, package version,
feature/default/optional policy or public re-export declaration. The dependency
API versions change; this is not a claim of global API equivalence.

095 qualified the exact modern pair and all113 expected library/proc-macro
artifacts in an independent default-feature consumer on the current Nix host.
Its lock preserved307 existing registry identities and recorded5 additions,
19 native-exclusive removals and5 feature-unification differences.
This establishes library typechecking for that graph, not final linking,
runtime tests, other platforms, all features or wider Bitcoin Core.

096 refreshes package generation from the explicitly transformed source
workspace. All112 retained archives include Cargo.lock, so regenerated locks
must also reflect the registry boundary. Existing package staging is retained
by one journaled atomic rename, not deleted. Cargo uses the same admitted warm
target. Every new archive is inspected against the original file selection,
the declared manifest transformation and source fingerprints; no fabricated
Git metadata is inserted. A final offline locked consumer check qualifies
the actual generated archives at current development versions.

No publication, final release-version selection, master update or tag is
performed by this step. Earlier native evidence remains scoped to its recorded
source and dependency graph; no full suite or stress rerun is implied.
