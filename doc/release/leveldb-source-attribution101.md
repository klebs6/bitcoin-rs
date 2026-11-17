# LevelDB source attribution and package declarations

The captured 112-package source set includes 1251 code-like members:1249 Rust
files and two local scripts. Each current file is fingerprinted in
[the attribution ledger](leveldb-source-attribution101.json). The ledger binds
the source reviewed at `1bbbb9950b1f1600c820bc1f6eeaa20ada786965` to explicit
origin comments, reviewed implementation/prose splits, and local Rust glue.

104 package declarations use `MIT AND BSD-3-Clause` for the included port
material and repository MIT context. Eight retain MIT: bitcoin-cfg,
bitcoin-imports, bitcoin-leveldb, bitcoin-support, bitcoinleveldb-erroriterator,
bitcoinleveldb-test, bitcoinleveldbt-mockversionset, and
bitcoinleveldbt-versionsettestutil. Dependency use alone is not a source-license
assignment. The error-iterator adapter is distinguished from the translated
EmptyIterator implementation; new Rust test helpers are likewise separate.

AND records cumulative conditions, not a choice of licenses. The original MIT,
LevelDB and CRC32C license files remain byte-identical. Package NOTICE files
retain corresponding primary header notices, including CRC32C 2008 material,
later LevelDB dates and named Core notices where witnessed. README license
sentences agree with the updated manifests. No source header is replaced and
no author, committer, tag or historical parent link is rewritten.

The comparison snapshot is bitcoin/bitcoin v22.0. Primary file contents were
checked against their Git blob IDs. That establishes the reference inspected;
it does not identify the exact historical C++ revision originally translated.
Per-file empty witness lists do not claim original authorship: the package
review retains both direct references and reviewed module/implementation
correspondences. Root COPYRIGHT remains a broad preserved aggregate; its
unrelated tinyformat/Boost block is not assigned by name to these packages.

This step changes packaging declarations and notices only. Rust source,
dependency requirements/features, package versions, original master and
qualification artifacts remain unchanged. Regeneration is deferred until the
final publication set/version can share one final artifact qualification.

The separate 312 registry-dependency packages in096 keep their own metadata and
notices. This ledger covers the 112 local packages, not an external-dependency
source audit or a universal legal-compliance guarantee.
