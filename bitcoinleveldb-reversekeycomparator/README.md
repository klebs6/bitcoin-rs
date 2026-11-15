# bitcoinleveldb-reversekeycomparator

A comparator for exercising reversed-byte key ordering in LevelDB tests.

`ReverseKeyComparator` compares exact reversed input bytes with the bytewise
comparator. It supports arbitrary byte keys, including empty keys and embedded
NULs. This is neither descending bytewise order nor Unicode collation. Separator
and successor methods shorten keys in reversed-byte space and reverse them back.

`reverse(&Slice)` returns `Vec<u8>`. `increment` takes a caller-owned byte-vector
output pointer: it appends NUL for the bytewise comparator and prepends NUL for
this comparator. Other custom orderings are outside that helper's contract.
Its comparator box is consumed; the output pointer must be valid and uniquely
writable for the call.

The persisted name is `bitcoinrs.ReverseBytewiseComparator.BytesV1`. It deliberately
differs from the former lossy implementation's `leveldb.ReverseBytewiseComparator`:
their binary-key orderings are incompatible. Normal recovery rejects a manifest
with the old name. Recreate a database from original logical records; rewriting
its manifest name does not convert ordering or recover previously collapsed keys.
The harness crate's separate comparator implementation is unchanged.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
