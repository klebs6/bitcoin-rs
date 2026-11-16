# bitcoinleveldb-compaction

Compaction plans and bookkeeping for the LevelDB storage engine.

`Compaction` records input files from adjacent levels, the input `Version`, a
`VersionEdit`, grandparent overlap, and output-size limits. Its predicates decide
whether a plan is a trivial move, whether a key occurs at a lower level, and when
to stop an output file because of overlap. `CompactionState` tracks generated
outputs; `ManualCompaction` carries a requested range. The exported traits connect
these plans to version-set selection and execution.

This crate describes and tracks compaction work; it is not a standalone
background compactor. It uses raw pointers into engine-owned options, versions,
and file metadata. Their lifetime and reference-counting obligations belong to
the engine; dropping a plan releases its retained input-version reference.

See [plan state](src/compaction.rs), [lifecycle](src/compaction_lifecycle.rs), and
[execution interfaces](src/interface.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
