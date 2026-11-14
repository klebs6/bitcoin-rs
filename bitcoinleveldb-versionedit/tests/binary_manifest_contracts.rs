use bitcoinleveldb_key::{InternalKey, ValueType};
use bitcoinleveldb_slice::Slice;
use bitcoinleveldb_versionedit::VersionEdit;

fn internal_key(user: &[u8], sequence: u64, kind: ValueType) -> InternalKey {
    InternalKey::new(&Slice::from(user), sequence, kind)
}

#[test]
fn binary_manifest_literal_wire_preserves_prefix_key_bytes_and_field_order() {
    let smallest = internal_key(&[0x00u8, 0xff], 128, ValueType::TypeValue);
    let largest = internal_key(&[0x80u8, 0x00, 0xff], 129, ValueType::TypeDeletion);
    let mut edit = VersionEdit::default();
    edit.set_comparator_name(&Slice::from(b"cmp".as_slice()));
    edit.set_log_number(128);
    edit.set_prev_log_number(16_384);
    edit.set_next_file(129);
    edit.set_last_sequence(255);
    edit.set_compact_pointer(2, &smallest);
    edit.delete_file(3, 128);
    edit.delete_file(1, 129);
    edit.add_file(4, 128, 16_384, &smallest, &largest);

    let prefix = [0xffu8, 0x00, 0xc3, 0x28];
    let mut output = prefix.to_vec();
    edit.encode_to(&mut output as *mut Vec<u8>);
    // Independent LevelDB tag/varint/internal-key oracle. Deleted files are
    // sorted by (level, number), regardless of insertion/hash iteration order.
    let literal = [
        0x01u8, 0x03, b'c', b'm', b'p',
        0x02, 0x80, 0x01,
        0x09, 0x80, 0x80, 0x01,
        0x03, 0x81, 0x01,
        0x04, 0xff, 0x01,
        0x05, 0x02, 0x0a, 0x00, 0xff, 0x01, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x06, 0x01, 0x81, 0x01,
        0x06, 0x03, 0x80, 0x01,
        0x07, 0x04, 0x80, 0x01, 0x80, 0x80, 0x01,
        0x0a, 0x00, 0xff, 0x01, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0b, 0x80, 0x00, 0xff, 0x00, 0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    assert_eq!(&output[..prefix.len()], &prefix);
    assert_eq!(&output[prefix.len()..], &literal);

    let mut parsed = VersionEdit::default();
    assert!(parsed.decode_from(&Slice::from(&output[prefix.len()..])).is_ok());
    assert_eq!(*parsed.log_number(), 128);
    assert_eq!(*parsed.prev_log_number(), 16_384);
    assert_eq!(*parsed.next_file_number(), 129);
    assert_eq!(*parsed.last_sequence(), 255);
    assert_eq!(parsed.compact_pointers()[0].1.encode().as_bytes(), smallest.encode().as_bytes());
    assert_eq!(parsed.new_files()[0].1.smallest().encode().as_bytes(), smallest.encode().as_bytes());
    assert_eq!(parsed.new_files()[0].1.largest().encode().as_bytes(), largest.encode().as_bytes());
    let mut reencoded = Vec::<u8>::new();
    parsed.encode_to(&mut reencoded as *mut Vec<u8>);
    assert_eq!(reencoded.as_slice(), &literal);
}

#[test]
fn binary_manifest_full_width_scalar_varints_match_literal_wire_bytes() {
    let mut edit = VersionEdit::default();
    edit.set_log_number(u64::MAX);
    edit.set_prev_log_number(1u64 << 63);
    edit.set_next_file(u64::MAX - 1);
    edit.set_last_sequence((1u64 << 56) - 1);
    let mut output = Vec::<u8>::new();
    edit.encode_to(&mut output as *mut Vec<u8>);
    let literal = [
        0x02u8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
        0x09, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01,
        0x03, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
        0x04, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
    ];
    assert_eq!(output.as_slice(), &literal);
    let mut parsed = VersionEdit::default();
    assert!(parsed.decode_from(&Slice::from(output.as_slice())).is_ok());
    assert_eq!(*parsed.log_number(), u64::MAX);
    assert_eq!(*parsed.prev_log_number(), 1u64 << 63);
    assert_eq!(*parsed.next_file_number(), u64::MAX - 1);
    assert_eq!(*parsed.last_sequence(), (1u64 << 56) - 1);
}
