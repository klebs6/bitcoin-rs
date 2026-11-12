# bitcoinleveldb-reversekeycomparator

A comparator used to exercise non-default key ordering in LevelDB tests.

`ReverseKeyComparator` reverses input bytes, converts them with
`String::from_utf8_lossy`, and compares the resulting strings with the bytewise
comparator. It does not simply negate ordinary bytewise comparison. Separator
and successor methods operate in reversed-byte space and reverse the result back.

The crate also supplies the `reverse` string helper and `increment` test helper.
`increment` appends a NUL for the bytewise comparator and uses character-reversed
string handling for other comparator names. Because reversed non-ASCII bytes
can undergo lossy UTF-8 conversion, this is not a general arbitrary-byte ordering
or Unicode collation API; it is intended for controlled test keys.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
