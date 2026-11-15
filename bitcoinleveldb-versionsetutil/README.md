# bitcoinleveldb-versionsetutil

Helpers used by LevelDB version management and compaction selection.

The crate supplies file-size and level-budget calculations, file-range tests,
boundary-file expansion, a smallest-key file comparator, file-iterator creation,
and lookup callback state (`Saver`/`SaverState`). These functions support
`bitcoinleveldb-versionset`; they do not manage a manifest or a live version set
on their own.

Many APIs take raw `Options`, comparator, table-cache, or `FileMetaData` pointers.
Keep the referenced objects live, supply correctly ordered file collections
where required, and retain the storage used by callback state until the
corresponding lookup finishes. File sizes and overlap limits are byte counts.

`Saver.value` is a `*mut Vec<u8>` destination. `save_value` copies raw value
bytes without constructing text. The destination must remain live and writable
until the callback finishes.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
