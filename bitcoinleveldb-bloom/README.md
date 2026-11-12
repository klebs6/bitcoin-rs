# bitcoinleveldb-bloom

LevelDB-style Bloom filters for the `bitcoin-rs` storage subsystem. The policy
uses the name `leveldb.BuiltinBloomFilter2` and records its probe count in the
filter's final byte.

`BloomFilterPolicy::new(bits_per_key)` creates a concrete policy;
`new_bloom_filter_policy` returns a boxed `FilterPolicy`. For byte-oriented
callers, `create_filter_from_bytes` appends an encoding to a `Vec<u8>` and
`key_may_match_bytes` queries it. A positive result is only a possible match;
the database must still check the key. The `FilterPolicy` interface also accepts
the subsystem's pointer-backed `Slice` values, whose storage must remain valid
during each call.

See [policy construction](src/bloom_filter_policy.rs),
[encoding](src/create_filter_from_bytes.rs), and
[matching](src/key_may_match_bytes.rs). Block-level filter indexing is supplied
by [`bitcoinleveldb-filter`](../bitcoinleveldb-filter/README.md).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
