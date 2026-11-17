# Bitcoin-RS LevelDB 0.2.0

Status: qualified release artifacts; publication pending.

This release set contains 112 modular LevelDB and support packages, including
the `bitcoin-leveldb` umbrella, harness and test crates. Every package archive
uses 0.2.0; declared internal published requirements accept compatible 0.2.x.
The complete dependency-first publication plan includes optional, target,
build and seven published development edges. DBImpl's versionless local
DBConstructor fixture remains repository-only.

## Contents and provenance

The release carries the focused LevelDB repairs and routine test controls
recorded in the retained qualification history. All 107 LevelDB READMEs and
five shared READMEs were reviewed. Source-specific attribution, notices and
license declarations were completed: 104 packages declare
`MIT AND BSD-3-Clause`; eight retain `MIT`. Each archive includes its README,
NOTICE and three retained license texts. Original author/history links remain.

The [artifact manifest](leveldb-0.2.0-artifacts103.json) binds all 112 archives
to qualified source commit `ce2b34a645131c6216f9d409ab672ef64308a44e` and tree
`aeadad599322d16b63aaf12c8d9a33def2de47e6`. The tracked
[publication projection](leveldb-version-projection102.md) explicitly derives
package versions/internal requirements and the qualified registry native pair
libevent 0.2.0/libevent-sys 0.4.0. Shared Core manifests and the root lock remain
unchanged. Archive `Cargo.toml.orig` contains the declared derived manifest;
the source commit plus original/derived hashes provide provenance. A later
metadata-only release-notes commit is recorded separately in the handoff.

## Migration notes

Callers of earlier String-oriented serialization interfaces must use byte
vectors. For example, coding append functions and internal-key/footer/version
encoding use `Vec<u8>` or caller-owned `*mut Vec<u8>` destinations. Arbitrary
database values use `DBGet::get_bytes`. Its default returns `NotSupported`
until a custom implementation opts in; text adapters validate UTF-8 and
preserve output on errors. Consult each affected crate's README for its exact
ownership and buffer contract.

The reverse-key test comparator now persists the name
`bitcoinrs.ReverseBytewiseComparator.BytesV1`, distinguishing exact reversed
bytes from the former lossy `leveldb.ReverseBytewiseComparator` ordering.
Normal recovery rejects the old name. Recreate affected test databases from
original logical records; changing the MANIFEST name does not convert their
ordering or recover previously collapsed keys. This is specific to that test
comparator; the harness crate's separate comparator remains unchanged.

## Verified scope

102 generated and inspected 112 actual 0.2.0 archives with 2147 members,
including exact source, manifest, dependency, license, NOTICE and README bytes.
An independent consumer resolved those 112 extracted archives and 312 locked
registry identities. Its library check produced all 113 expected selected/root
library and procedural-macro artifacts, with no compiler errors. All 18
guarded phases completed successfully on the recorded x86_64 Linux/Nix
nightly environment. The original source, held/retired work and old evidence
were preserved.

These are library compilation/metadata results. Final executable linking and
runtime execution of the published dependency graph, stable-Rust MSRV,
other platforms and all-features qualification are unmeasured. Packaging work
did not rerun whole native suites or full randomized stress. Earlier runtime
receipts retain their exact source/toolchain/options scope; their overlapping
counts are not added into a new universal total.

## Publication handoff

The sealed bundle contains the exact inspected `.crate` bytes, checksums,
these notes, the artifact manifest and the dependency-first order. Its
deterministic export format is not a reproducible-compilation claim.
The bundle hash, subsequent metadata commit and proposed tag are recorded
after sealing; no committed file depends on its own future hash.

Before publication, review the concrete master fast-forward/annotated-tag
plan, refresh registry name/version/ownership checks and select an
authenticated publishing host. The local preparation reads no credentials,
creates no release tag, pushes nothing and publishes no crate.
