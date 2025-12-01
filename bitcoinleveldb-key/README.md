# bitcoinleveldb-key

Low-level internal key encoding utilities for the `bitcoin-rs` LevelDB port. This crate provides a faithful, byte-for-byte reproduction of LevelDB's **internal key** machinery, specialized for Bitcoin's storage engine.

It focuses on:

- Packing and unpacking LevelDB **internal keys** `(user_key, sequence_number, value_type)` into an opaque byte sequence.
- Correct ordering semantics via an `InternalKeyComparator` that respects both user key order and sequence numbers.
- Filter policy adaptation so that user-level Bloom filters remain valid when the DB internally stores extended keys.
- Efficient construction of lookup keys for MemTable and SSTable access.

## Design overview

LevelDB (and this port) do **not** store raw user keys directly. Instead, they use an **internal key**:

```text
internal_key := user_key || tag

where
  tag := pack(sequence_number: 56 bits, value_type: 8 bits)
```

This design allows multiple versions of the same logical user key to coexist, with different `sequence_number` values and `ValueType` variants:

- `ValueType::TypeValue`   — key is present with a value
- `ValueType::TypeDeletion` — key is logically deleted at that sequence

The **sort order** for internal keys is:

1. First by the user key, using a user-supplied `SliceComparator` (or bytewise lexicographic fallback).
2. For equal user keys, by **decreasing** sequence number (newest entries first).

This crate encapsulates these invariants so that higher-level components (MemTable, SSTable, DB implementation) can operate safely without re-implementing the encoding logic.

The following invariants are critical and preserved here:

- `ValueType` is `#[repr(u8)]` and the discriminants are **stable on disk** — they must not change.
- `SequenceNumber` is a 64‑bit integer, but only the high 56 bits are used when packed, leaving the lower 8 bits for `ValueType`.
- All encoding/decoding paths are little-endian to match LevelDB.

## Crate features and components

### ValueType

```rust
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueType {
    TypeDeletion,
    TypeValue,
}
```

Represents the logical type of a versioned key. The enum values are encoded directly into the low 8 bits of the tag and therefore must remain stable.

`ValueType::from_tag(tag: u8) -> Option<Self>` maps raw tags back to the enum, enforcing the valid subset `{0x00, 0x01}`.

### Key and Value traits

```rust
pub trait Key {
    fn key(&self) -> Slice;
}

pub trait Value {
    fn value(&self) -> Slice;
}
```

Minimal traits abstracting over key/value providers that expose their representation as a `Slice` (defined elsewhere in the `bitcoinleveldb` ecosystem).

These traits are intentionally narrow: they push policy into higher layers and keep the encoding layer purely about bytes.

### ParsedInternalKey

```rust
pub type SequenceNumber = u64;

#[derive(Setters, Getters)]
#[getset(get = "pub", set = "pub")]
pub struct ParsedInternalKey {
    user_key: Slice,
    sequence: SequenceNumber,
    ty:       ValueType,
}
```

`ParsedInternalKey` is a structured view of an internal key:

- `user_key`: the raw user key bytes
- `sequence`: the sequence number
- `ty`: the `ValueType`

Key operations:

- `ParsedInternalKey::new(u: &Slice, seq: &SequenceNumber, t: ValueType) -> Self`
- `ParsedInternalKey::default()` — uses an empty key and `VALUE_TYPE_FOR_SEEK` as configured elsewhere.
- `debug_string(&self) -> String` — stable debug formatting: `'<escaped_user>' @ seq : type_tag`.

### InternalKey

```rust
#[derive(Clone)]
pub struct InternalKey {
    rep: String,
}
```

Represents a fully encoded internal key as a sequence of bytes. The encoding is **not** UTF‑8 by design; the use of `String` is purely a compatibility artifact with the original LevelDB.

Key methods:

- `InternalKey::new(user_key: &Slice, s: SequenceNumber, t: ValueType) -> Self`  
  Encodes `(user_key, s, t)` into the `rep` buffer using `append_internal_key`.

- `fn decode_from(&mut self, s: &Slice) -> bool`  
  Decodes an internal key from raw bytes into `self.rep`. Returns `false` for empty input.

- `fn encode(&self) -> Slice`  
  Returns a `Slice` view over the internal key bytes. Panics if `rep` is empty.

- `fn user_key(&self) -> Slice`  
  Extracts the user key portion by stripping the trailing 8‑byte tag.

- `fn set_from(&mut self, p: &ParsedInternalKey)` / `fn clear(&mut self)`  
  Reset or rebuild `rep` from a parsed representation.

- `fn debug_string(&self) -> String`  
  Parses the internal representation and renders a `ParsedInternalKey` debug string, or `(bad)<escaped_bytes>` if parsing fails.

The `Debug` implementation for `InternalKey` uses `debug_string()` instead of raw bytes, yielding stable, human-meaningful logging.

### Encoding helpers

#### pack_sequence_and_type

```rust
pub fn pack_sequence_and_type(seq: u64, t: ValueType) -> u64
```

Packs `seq` and `t` into a single `u64`:

- High 56 bits: `seq` (constrained by `MAX_SEQUENCE_NUMBER` elsewhere)
- Low 8 bits: `t as u8` (constrained by `VALUE_TYPE_FOR_SEEK` elsewhere)

Ensures:

- `seq <= MAX_SEQUENCE_NUMBER`
- `t as u64 <= VALUE_TYPE_FOR_SEEK as u64`

#### internal_key_encoding_length

```rust
pub fn internal_key_encoding_length(k: &ParsedInternalKey) -> usize
```

Returns `user_key_len + 8`. This is deterministic and matches the C++ implementation.

#### parse_internal_key

```rust
pub fn parse_internal_key(internal_key: &Slice, result: *mut ParsedInternalKey) -> bool
```

Given an encoded internal key, fills `*result` with:

- `user_key` = all but last 8 bytes
- `sequence` = high 56 bits of the decoded tag
- `ty` = `ValueType::from_tag(low_8_bits)`

Returns `false` if:

- The length is `< 8`.
- The tag is invalid or beyond `VALUE_TYPE_FOR_SEEK`.

Caller provides a non-null pointer to a `ParsedInternalKey` instance.

#### append_internal_key

```rust
pub fn append_internal_key(result: *mut String, k: &ParsedInternalKey)
```

Appends the encoded internal key for `k` onto the provided `String` buffer:

1. Copies raw user key bytes.
2. Packs and appends the (sequence, type) tag via `pack_sequence_and_type` and `encode_fixed64_le`.

### Low-level byte utilities

These functions provide self-contained, allocation-free primitives for manipulating key bytes:

- `encode_fixed64_le(value: u64) -> [u8; 8]` and `decode_fixed64_le(ptr: *const u8) -> u64` — fixed 64‑bit little-endian encode/decode.
- `put_varint32_vec(dst: &mut Vec<u8>, v: u32)` — varint32 encoding into an existing `Vec<u8>`.
- `decode_varint32(src: &[u8]) -> (u32, usize)` — decodes a varint32, asserting on malformed input.
- `slice_as_bytes(s: &Slice) -> &[u8]` — safe view over a `Slice`.
- `bytewise_compare(a: &[u8], b: &[u8]) -> i32` — lexicographic compare with LevelDB-style `{-1,0,1}` result.

Key-range shortening helpers, used by compaction and iterators:

- `shorten_separator_user_keys(start: &[u8], limit: &[u8]) -> Option<Vec<u8>>`  
  Attempts to construct a minimal user key between `start` and `limit` by bumping the first differing byte when possible.

- `find_short_successor_user_key(key: &mut Vec<u8>) -> bool`  
  Makes `key` a short successor by incrementing the first non-`0xff` byte and truncating after it.

Debug utility:

- `escape_for_debug(input: &[u8]) -> String`  
  Renders a byte string using C-style escapes for control and non-ASCII bytes.

### extract_user_key

```rust
pub fn extract_user_key(internal_key: &Slice) -> Slice
```

Returns the user key portion from a full internal key. Panics if the key is shorter than 8 bytes.

This is the canonical way to slice away the tag when you know you have a valid internal key.

### InternalFilterPolicy

```rust
pub struct InternalFilterPolicy {
    user_policy: *const dyn FilterPolicy,
}
```

Wraps a user-provided `FilterPolicy` (e.g., Bloom filter) and transparently converts internal keys into user keys before delegating.

Implements:

- `FilterPolicy`
- `Named`
- `CreateFilter`
- `KeyMayMatch`

Key behavior:

- `create_filter(&self, keys: *const Slice, n: i32, dst: &mut Vec<u8>)`
  - Rewrites the `keys` array **in place**: each entry becomes its user key by applying `extract_user_key`.
  - Delegates to `user_policy.create_filter` if provided.

- `key_may_match(&self, k: &Slice, f: &Slice) -> bool`
  - Extracts `user_key` from internal key `k`.
  - Delegates to `user_policy.key_may_match`.

- `name(&self) -> Cow<'_, str>`
  - If no user policy is set, returns an empty name; otherwise forwards.

`InternalFilterPolicy::new(p: *const dyn FilterPolicy) -> Self` constructs a wrapper around `p`. Passing a null pointer disables filtering.

### InternalKeyComparator

```rust
pub struct InternalKeyComparator {
    user_comparator: *const dyn SliceComparator,
}
```

Implements the precise ordering semantics for internal keys:

1. Compare on user key using `user_comparator` if provided, else bytewise.
2. If equal, compare the packed `sequence+type` tag such that **higher sequence numbers sort first**.

Implements:

- `SliceComparator`
- `Compare`
- `Named`
- `FindShortSuccessor`
- `FindShortestSeparator`

Core methods:

#### compare_slices

```rust
pub fn compare_slices(&self, akey: &Slice, bkey: &Slice) -> i32
```

- Extracts user key portions using `extract_user_key`.
- Compares user keys using `user_comparator` or `bytewise_compare`.
- If equal, decodes tags via `decode_fixed64_le` and orders by descending tag (`a_num > b_num` yields `-1`).

`compare_internal_key(&self, a: &InternalKey, b: &InternalKey) -> i32` is a convenience wrapper taking `InternalKey` objects.

#### Shortening functions

Used during compaction to minimize index key sizes while preserving ordering.

- `find_short_successor(&self, k: &mut Vec<u8>)`
  - Treats `k` as an internal-key byte vector.
  - Extracts and shortens the user key (via user comparator or `find_short_successor_user_key`).
  - Re-appends a tag with `MAX_SEQUENCE_NUMBER` and `VALUE_TYPE_FOR_SEEK`.
  - Ensures the new key is strictly greater than the original in internal-key order.

- `find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8])`
  - Uses user-level separator logic to compute a shorter user key between `start` and `limit`.
  - Reconstructs a full internal key, again using `MAX_SEQUENCE_NUMBER` and `VALUE_TYPE_FOR_SEEK` for the tag.
  - Asserts the new key lies strictly between original `start` and `limit` in internal-key order.

`InternalKeyComparator::new(c: *const dyn SliceComparator) -> Self` creates a comparator; `null_slice_comparator()` can be used to trigger fallback bytewise logic without providing a concrete comparator.

The `name()` implementation returns a stable identifier:

```rust
"leveldb.InternalKeyComparator"
```

### LookupKey

```rust
pub struct LookupKey {
    start:  *const u8,
    kstart: *const u8,
    end:    *const u8,
    space:  [u8; 200],
    buf:    Vec<u8>,
}
```

`LookupKey` assists with constructing the exact key bytes used when issuing point lookups against MemTables and internal iterators.

The layout encoded in `buf` is:

```text
varint32(internal_key_len) || user_key || tag
```

Where `internal_key_len = user_key_len + 8`.

Construction:

```rust
impl LookupKey {
    pub fn new(user_key: &Slice, sequence: SequenceNumber) -> Self;
}
```

The constructor:

- Varint-encodes the internal key length into `buf`.
- Appends the raw user key bytes.
- Packs `(sequence, VALUE_TYPE_FOR_SEEK)` into a tag and appends it.
- Sets raw pointers `start`, `kstart`, and `end` inside `buf` for fast slicing.

Accessors:

- `memtable_key(&self) -> Slice`  
  Returns `varint_len || internal_key` — used directly as the key in the MemTable.

- `internal_key(&self) -> Slice`  
  Returns `user_key || tag`, i.e., the internal key view.

- `user_key(&self) -> Slice`  
  Returns the user portion, ensuring there is room for the tag.

The `Drop` implementation is trivial: the backing `Vec` and stack storage clean themselves up; raw pointers are only references into that storage and do not own anything.

### null_slice_comparator

```rust
pub fn null_slice_comparator() -> *const dyn SliceComparator
```

Constructs a syntactically valid but semantically null trait-object pointer to `SliceComparator`.

This is never dereferenced intentionally; it exists solely to exercise and validate the *"no user comparator provided"* code paths. It uses `transmute` over `(0usize, 0usize)` for the vtable/data pair and must be handled with care.

## Usage examples

### Encoding and decoding an internal key

```rust
use bitcoinleveldb_key::{
    ParsedInternalKey, InternalKey, ValueType,
    SequenceNumber, internal_key_encoding_length,
    parse_internal_key, extract_user_key,
};
use bitcoinleveldb_slice::Slice; // assuming this is the Slice type used in the project

fn roundtrip_example() {
    let user_bytes = b"example-key";
    let user_slice = unsafe { Slice::from_ptr_len(user_bytes.as_ptr(), user_bytes.len()) };

    let seq: SequenceNumber = 42;
    let ty = ValueType::TypeValue;

    // Structured representation
    let parsed = ParsedInternalKey::new(&user_slice, &seq, ty);
    let encoded_len = internal_key_encoding_length(&parsed);
    assert_eq!(encoded_len, user_bytes.len() + 8);

    // Opaque InternalKey wrapper
    let ikey = InternalKey::new(&user_slice, seq, ty);
    let encoded_slice = ikey.encode();

    // Parse back into ParsedInternalKey
    let mut parsed_back = ParsedInternalKey::default();
    let ok = parse_internal_key(&encoded_slice, &mut parsed_back as *mut ParsedInternalKey);
    assert!(ok);

    let back_user = extract_user_key(&encoded_slice);
    let back_user_bytes = unsafe {
        std::slice::from_raw_parts(*back_user.data(), *back_user.size())
    };

    assert_eq!(back_user_bytes, user_bytes);
    assert_eq!(*parsed_back.sequence(), seq);
    assert_eq!(*parsed_back.ty(), ty);
}
```

### Using InternalKeyComparator with a bytewise comparator

```rust
use bitcoinleveldb_key::{
    InternalKey, InternalKeyComparator,
    ValueType, SequenceNumber,
};
use bitcoinleveldb_slice::SliceComparator; // your implementation

fn comparator_example() {
    // For pure bytewise ordering, you may pass a null comparator and rely on fallback.
    let cmp = InternalKeyComparator::new(bitcoinleveldb_key::null_slice_comparator());

    let uk1 = b"a";
    let uk2 = b"a";
    let s1: SequenceNumber = 1;
    let s2: SequenceNumber = 2;

    let k1 = InternalKey::new(
        &unsafe { bitcoinleveldb_slice::Slice::from_ptr_len(uk1.as_ptr(), uk1.len()) },
        s1,
        ValueType::TypeValue,
    );
    let k2 = InternalKey::new(
        &unsafe { bitcoinleveldb_slice::Slice::from_ptr_len(uk2.as_ptr(), uk2.len()) },
        s2,
        ValueType::TypeValue,
    );

    // k2 is newer (higher sequence) and must sort before k1.
    assert!(cmp.compare_internal_key(&k2, &k1) < 0);
}
```

### Building a LookupKey for MemTable access

```rust
use bitcoinleveldb_key::{LookupKey, SequenceNumber};
use bitcoinleveldb_slice::Slice;

fn memtable_lookup_example() {
    let user_key_bytes = b"height:00000010";
    let user_key = unsafe { Slice::from_ptr_len(user_key_bytes.as_ptr(), user_key_bytes.len()) };
    let snapshot_seq: SequenceNumber = 123_456;

    let lookup = LookupKey::new(&user_key, snapshot_seq);

    let memtable_key = lookup.memtable_key();    // varint32(len) || internal_key
    let internal_key = lookup.internal_key();    // user_key || tag
    let just_user = lookup.user_key();           // user_key only

    // The MemTable layer treats `memtable_key` as the primary key.
    drop((memtable_key, internal_key, just_user));
}
```

## Integration notes

- This crate is part of the `bitcoin-rs` repository and is intended to be used in concert with sibling crates providing `Slice`, filter policies, comparators, MemTable implementations, and SSTable abstractions.
- The on-disk format is designed to be **stable and compatible** with the original LevelDB layout; do not change `ValueType` discriminants or encoding routines if you care about read-compatibility.
- Many functions use `unsafe` with raw pointers (`Slice`, trait-object pointers, etc.). The invariants are mirroring the C++ library; ensure any external uses respect lifetimes and aliasing constraints.

## Repository, license, and authors

- Repository: <https://github.com/klebs6/bitcoin-rs>
- Crate: `bitcoinleveldb-key`
- Edition: 2021
- License: MIT
- Author: `klebs <none>`

If you extend or wrap this crate, preserve the encoding, comparison, and filter semantics to maintain compatibility with existing data and with canonical LevelDB behavior.
