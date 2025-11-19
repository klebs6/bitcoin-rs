// ---------------- [ File: bitcoinleveldb-key/tests/dbformat.rs ]
use bitcoinleveldb_key::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_comparator::*;
use bitcoin_imports::*;

/// Construct a null-like SliceComparator pointer for tests.
///
/// We *never* dereference this; InternalKeyComparator will detect null
/// and fall back to bytewise comparison and our local shortening helpers.
fn null_slice_comparator() -> *const dyn SliceComparator {
    unsafe {
        // Represent a null fat pointer as (0, 0).
        std::mem::transmute::<(usize, usize), *const dyn SliceComparator>((0, 0))
    }
}

/// Build the encoded *internal key* bytes for (user_key, seq, vt).
///
/// This matches LevelDB's IKey(user_key, seq, vt):
///   internal = user_key_bytes || little_endian( (seq << 8) | vt )
fn key(user_key_: &str, seq: u64, vt: ValueType) -> Vec<u8> {
    trace!(
        "test helper key: user_key='{}', seq={}, vt={:?}",
        user_key_,
        seq,
        vt
    );

    let user_bytes = user_key_.as_bytes();
    let mut encoded = Vec::with_capacity(user_bytes.len() + 8);
    encoded.extend_from_slice(user_bytes);

    let tag = pack_sequence_and_type(seq, vt);
    let le = encode_fixed64_le(tag);
    encoded.extend_from_slice(&le);

    encoded
}

/// Apply InternalKeyComparator::FindShortestSeparator to two internal keys.
///
/// Returns the modified internal key bytes.
fn shorten(s: &Vec<u8>, l: &Vec<u8>) -> Vec<u8> {
    trace!(
        "test helper shorten: s_len={}, l_len={}",
        s.len(),
        l.len()
    );

    let mut start = s.clone();
    let limit = l.as_slice();

    let icmp = InternalKeyComparator::new(null_slice_comparator());
    icmp.find_shortest_separator(&mut start, limit);

    start
}

/// Apply InternalKeyComparator::FindShortSuccessor to an internal key.
///
/// Returns the modified internal key bytes.
fn short_successor(s: &Vec<u8>) -> Vec<u8> {
    trace!(
        "test helper short_successor: s_len={}",
        s.len()
    );

    let mut bytes = s.clone();

    let icmp = InternalKeyComparator::new(null_slice_comparator());
    icmp.find_short_successor(&mut bytes);

    bytes
}

/// Round-trip test for ParseInternalKey / encode logic, matching LevelDB's TestKey.
fn test_key(user_key_: &str, seq: u64, vt: ValueType) {
    trace!(
        "test helper test_key: user_key='{}', seq={}, vt={:?}",
        user_key_,
        seq,
        vt
    );

    let encoded = key(user_key_, seq, vt);

    unsafe {
        let input = Slice::from_ptr_len(encoded.as_ptr(), encoded.len());
        let mut decoded = ParsedInternalKey::default();

        assert!(
            parse_internal_key(&input, &mut decoded as *mut ParsedInternalKey),
            "parse_internal_key should succeed"
        );

        // Extract user key bytes from decoded.user_key() and interpret as UTF-8.
        let decoded_user = {
            let s = decoded.user_key();
            let data = *s.data();
            let len = *s.size();
            let bytes = std::slice::from_raw_parts(data, len);
            String::from_utf8_lossy(bytes).into_owned()
        };

        assert_eq!(user_key_, decoded_user);
        assert_eq!(seq, *decoded.sequence());
        assert_eq!(vt, *decoded.ty());

        // Short key should fail.
        let bad_input = Slice::from_ptr_len(b"bar".as_ptr(), 3);
        assert!(
            !parse_internal_key(&bad_input, &mut decoded as *mut ParsedInternalKey),
            "parse_internal_key should fail for short key"
        );
    }
}

struct FormatTest {}

#[traced_test]
fn format_test_internal_key_encode_decode() {
    let keys = ["", "k", "hello", "longggggggggggggggggggggg"];
    let seqs: [u64; 12] = [
        1,
        2,
        3,
        (1u64 << 8) - 1,
        1u64 << 8,
        (1u64 << 8) + 1,
        (1u64 << 16) - 1,
        1u64 << 16,
        (1u64 << 16) + 1,
        (1u64 << 32) - 1,
        1u64 << 32,
        (1u64 << 32) + 1,
    ];

    for key_str in &keys {
        for &s in &seqs {
            test_key(key_str, s, ValueType::TypeValue);
            test_key("hello", 1, ValueType::TypeDeletion);
        }
    }
}

#[traced_test]
fn format_test_internal_key_decode_from_empty() {
    let mut internal_key = InternalKey::default();
    let data = "";
    unsafe {
        let slice = Slice::from_ptr_len(data.as_ptr(), data.len());
        assert!(
            !internal_key.decode_from(&slice),
            "decode_from should fail for empty slice"
        );
    }
}

#[traced_test]
fn format_test_internal_key_short_separator() {
    // Helper matching LevelDB's IKey helper.
    let ikey = |u: &str, seq: u64, vt: ValueType| -> Vec<u8> {
        key(u, seq, vt)
    };

    // When user keys are same
    assert_eq!(
        ikey("foo", 100, ValueType::TypeValue),
        shorten(
            &ikey("foo", 100, ValueType::TypeValue),
            &ikey("foo", 99, ValueType::TypeValue)
        )
    );
    assert_eq!(
        ikey("foo", 100, ValueType::TypeValue),
        shorten(
            &ikey("foo", 100, ValueType::TypeValue),
            &ikey("foo", 101, ValueType::TypeValue)
        )
    );
    assert_eq!(
        ikey("foo", 100, ValueType::TypeValue),
        shorten(
            &ikey("foo", 100, ValueType::TypeValue),
            &ikey("foo", 100, ValueType::TypeValue)
        )
    );
    assert_eq!(
        ikey("foo", 100, ValueType::TypeValue),
        shorten(
            &ikey("foo", 100, ValueType::TypeValue),
            &ikey("foo", 100, ValueType::TypeDeletion)
        )
    );

    // When user keys are misordered
    assert_eq!(
        ikey("foo", 100, ValueType::TypeValue),
        shorten(
            &ikey("foo", 100, ValueType::TypeValue),
            &ikey("bar", 99, ValueType::TypeValue)
        )
    );

    // When user keys are different, but correctly ordered
    assert_eq!(
        ikey("g", MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK),
        shorten(
            &ikey("foo", 100, ValueType::TypeValue),
            &ikey("hello", 200, ValueType::TypeValue)
        )
    );

    // When start user key is prefix of limit user key
    assert_eq!(
        ikey("foo", 100, ValueType::TypeValue),
        shorten(
            &ikey("foo", 100, ValueType::TypeValue),
            &ikey("foobar", 200, ValueType::TypeValue)
        )
    );

    // When limit user key is prefix of start user key
    assert_eq!(
        ikey("foobar", 100, ValueType::TypeValue),
        shorten(
            &ikey("foobar", 100, ValueType::TypeValue),
            &ikey("foo", 200, ValueType::TypeValue)
        )
    );
}

#[traced_test]
fn format_test_internal_key_shortest_successor() {

    // Helper: Build internal key bytes from a UTF-8 user key string.
    fn ikey_from_str(u: &str, seq: u64, vt: ValueType) -> Vec<u8> {
        trace!("test helper ikey_from_str: u='{}', seq={}, vt={:?}", u, seq, vt);

        // We MUST build through `String` because `append_internal_key`
        // requires `*mut String`, not a Vec<u8>.
        let mut tmp = String::new();
        unsafe {
            let user_slice = Slice::from_ptr_len(u.as_ptr(), u.len());
            let parsed = ParsedInternalKey::new(&user_slice, &seq, vt);
            append_internal_key(&mut tmp as *mut String, &parsed);
        }

        // Now pull the underlying bytes (LevelDB internal keys are binary).
        let bytes = tmp.as_bytes().to_vec();
        bytes
    }

    // Helper: build from raw user bytes (including non-UTF8 like 0xff).
    fn ikey_from_bytes(u: &[u8], seq: u64, vt: ValueType) -> Vec<u8> {
        trace!("test helper ikey_from_bytes: seq={}, vt={:?}", seq, vt);

        let mut tmp = String::new();
        unsafe {
            let user_slice = Slice::from_ptr_len(u.as_ptr(), u.len());
            let parsed = ParsedInternalKey::new(&user_slice, &seq, vt);
            append_internal_key(&mut tmp as *mut String, &parsed);
        }
        tmp.as_bytes().to_vec()
    }

    // Helper: run FindShortSuccessor and return new bytes.
    fn run_short_successor(ikey: &[u8]) -> Vec<u8> {
        trace!("run_short_successor: ikey_len={}", ikey.len());

        let mut v = ikey.to_vec();

        let null_comp: *const dyn SliceComparator = unsafe {
            // Construct a null trait-object pointer (fat pointer = (0,0)).
            std::mem::transmute::<(usize, usize), *const dyn SliceComparator>((0, 0))
        };

        let icmp = InternalKeyComparator::new(null_comp);
        icmp.find_short_successor(&mut v);

        v
    }

    //
    // === Test case 1: "foo" → "g" with max seq ===
    //
    let expected = ikey_from_str("g", MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
    let computed = run_short_successor(&ikey_from_str("foo", 100, ValueType::TypeValue));
    assert_eq!(
        expected,
        computed,
        "short successor of 'foo' did not match expected"
    );

    //
    // === Test case 2: [0xff,0xff] → unchanged ===
    //
    let user_ff = [0xffu8, 0xffu8];

    let expected_ff = ikey_from_bytes(&user_ff, 100, ValueType::TypeValue);
    let computed_ff = run_short_successor(&expected_ff);

    assert_eq!(
        expected_ff,
        computed_ff,
        "short successor of [0xff,0xff] should remain unchanged"
    );
}

#[traced_test]
fn format_test_parsed_internal_key_debug_string() {
    let s = "The \"key\" in 'single quotes'";
    let seq: SequenceNumber = 42;
    let key;
    unsafe {
        let user = Slice::from_ptr_len(s.as_ptr(), s.len());
        key = ParsedInternalKey::new(&user, &seq, ValueType::TypeValue);
    }
    let debug = key.debug_string();
    let expected = "'The \\\"key\\\" in 'single quotes'' @ 42 : 1";
    assert_eq!(expected, debug);
}

#[traced_test]
fn format_test_internal_key_debug_string() {
    let s = "The \"key\" in 'single quotes'";
    let seq: SequenceNumber = 42;

    let key = unsafe {
        let user = Slice::from_ptr_len(s.as_ptr(), s.len());
        InternalKey::new(&user, seq, ValueType::TypeValue)
    };

    let expected = "'The \\\"key\\\" in 'single quotes'' @ 42 : 1";
    assert_eq!(expected, key.debug_string());

    let invalid_key = InternalKey::default();
    assert_eq!("(bad)", invalid_key.debug_string());
}
