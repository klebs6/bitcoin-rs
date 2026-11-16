# bitcoinleveldb-key

Internal-key encoding and comparison for the `bitcoin-rs` LevelDB port.
An internal key appends an eight-byte sequence/type tag to a user key. User keys
sort according to their supplied comparator; equal user keys sort by decreasing
packed sequence/type tag so newer entries are visited first.

- `InternalKey`, `ParsedInternalKey`, and the parse/append helpers build and
  inspect encoded keys.
- `InternalKeyComparator` adapts the user comparator to internal-key order.
- `LookupKey` prepares the memtable lookup representation.
- `InternalFilterPolicy` strips internal tags before delegating filtering to a
  user-key policy.

`InternalKey` owns its serialized representation as `Vec<u8>`. Its
`encode`/`decode_from` views preserve arbitrary user bytes and tag bytes.
`append_internal_key_to_vec` appends this binary format to a byte buffer.
`append_internal_key` now takes a `*mut Vec<u8>` destination as well; its
former String sink was a binary-storage defect. This is a Rust signature
change documented in the [migration record](../doc/release/leveldb-iterator-bytes066.md).

`SequenceNumber` uses a 56-bit limit and `ValueType` distinguishes values from
deletions. Pointer-backed slices and comparator/filter pointers borrow their
storage or owners; keep them valid for the operation or adapter lifetime.

See [key representation](src/internal_key.rs),
[comparison](src/internal_key_comparator.rs), and [lookup keys](src/lookup_key.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
