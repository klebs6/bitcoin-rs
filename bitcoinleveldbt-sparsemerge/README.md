# bitcoinleveldbt-sparsemerge

Sparse-merge compaction and environment-borrow regressions.

The original `db_test_sparse_merge` retains the full database workload and checks sparse-range iteration/compaction behavior. A separate rotation test exercises environment-borrow coordination. The subsystem runner gives the original case its bounded stress envelope; source and diagnostic history do not establish a cause for every earlier allocator abort.

Sources: [bitcoinleveldbt_sparsemerge](src/bitcoinleveldbt_sparsemerge.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-sparsemerge`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
