# bitcoinleveldb-versionedit

LevelDB manifest-edit records and their binary encoding.

`VersionEdit` tracks optional comparator/log/sequence/file-number metadata,
compaction pointers, removed files, and added table-file metadata.
`encode_to(*mut Vec<u8>)` appends the tagged binary record; `decode_from`
parses a record and returns `Status`. Deleted-file entries are sorted before
encoding, independently of `HashSet` iteration order. `clear` resets scalar/file
state while preserving compaction pointers; `debug_string` formats the edit for
inspection.

The type describes a change; it does not commit that change to a manifest.
`VersionEditLogAndApply` defines the higher-level application interface.
A nonnull output pointer must refer to a live writable byte vector; decoding
requires valid source bytes for the call. Internal keys and file metadata must
satisfy the engine's comparator and level contracts. Comparator names and
diagnostic text remain strings; binary keys and encoded records are byte-backed.

See [record state](src/versionedit.rs), [encoding](src/versionedit_encode_to.rs),
and [decoding](src/versionedit_decode_from.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) ·
[Bounded verification](../doc/testing/leveldb-subsystem.md) ·
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues).
License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Upstream notices](NOTICE)
