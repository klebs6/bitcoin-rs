# bitcoinleveldb-versionedit

LevelDB manifest-edit records and their binary encoding.

`VersionEdit` tracks optional comparator/log/sequence/file-number metadata,
compaction pointers, removed files, and added table-file metadata. `encode_to`
appends a tagged encoding to a caller-provided string buffer; `decode_from`
parses a record and returns `Status`. Deleted-file entries are sorted before
encoding, making that portion independent of `HashSet` iteration order.
`clear` resets the edit, and `debug_string` formats it for inspection.

The type describes a change; it does not itself commit that change to a manifest.
`VersionEditLogAndApply` defines the higher-level application interface.
`encode_to` uses a raw output pointer, which must refer to a valid writable
`String`; decoding requires valid source bytes for the call. Internal-key and
file metadata must satisfy the engine's comparator and level contracts.

See [record state](src/versionedit.rs), [encoding](src/versionedit_encode_to.rs),
and [decoding](src/versionedit_decode_from.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
