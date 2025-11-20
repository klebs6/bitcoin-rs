// ---------------- [ File: bitcoinleveldb-versionedit/tests/versionedit.rs ]
use bitcoinleveldb_versionedit::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_edit_test.cc]

fn test_encode_decode_roundtrip(edit: &VersionEdit) {
    trace!("test_encode_decode_roundtrip: starting roundtrip test");

    let mut encoded = String::new();
    let mut encoded2 = String::new();

    edit.encode_to(&mut encoded as *mut String);

    let encoded_slice = Slice::from(&encoded);
    let mut parsed = VersionEdit::default();

    let status = parsed.decode_from(&encoded_slice);
    debug!(
        "test_encode_decode_roundtrip: decode_from status.ok()={}",
        status.ok()
    );
    assert!(status.ok());

    parsed.encode_to(&mut encoded2 as *mut String);

    assert_eq!(
        encoded, encoded2,
        "encoded manifest record did not roundtrip through DecodeFrom/EncodeTo"
    );
}

#[traced_test]
fn version_edit_roundtrip_encoding_decoding() {
    trace!("version_edit_roundtrip_encoding_decoding: constructing test VersionEdit instance");

    const K_BIG: u64 = 1u64 << 50;

    let mut edit = VersionEdit::default();

    for i in 0..4 {
        test_encode_decode_roundtrip(&edit);

        let file_number = K_BIG + 300 + i;
        let file_size = K_BIG + 400 + i;

        let smallest =
            InternalKey::new("foo", K_BIG + 500 + i, ValueType::TypeValue);
        let largest =
            InternalKey::new("zoo", K_BIG + 600 + i, ValueType::TypeDeletion);

        edit.add_file(3, file_number, file_size, &smallest, &largest);

        let delete_file_number = K_BIG + 700 + i;
        edit.delete_file(4, delete_file_number);

        let compact_key =
            InternalKey::new("x", K_BIG + 900 + i, ValueType::TypeValue);
        edit.set_compact_pointer(i as i32, &compact_key);
    }

    let comparator_name = String::from("foo");
    let comparator_slice = Slice::from(&comparator_name);
    edit.set_comparator_name(&comparator_slice);
    edit.set_log_number(K_BIG + 100);
    edit.set_next_file(K_BIG + 200);
    edit.set_last_sequence(K_BIG + 1000);

    test_encode_decode_roundtrip(&edit);
}
