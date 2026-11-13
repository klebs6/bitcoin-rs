use bitcoinleveldb_blockhandle::{BlockHandle, BLOCK_HANDLE_MAX_ENCODED_LENGTH};
use bitcoinleveldb_slice::Slice;

fn handle(offset: u64, size: u64) -> BlockHandle {
    let mut result = BlockHandle::new();
    result.set_offset(offset);
    result.set_size(size);
    result
}

fn assert_decodes(bytes: &[u8], offset: u64, size: u64, suffix: &[u8]) {
    let mut input = Slice::from(bytes);
    let mut decoded = BlockHandle::new();
    assert!(decoded.decode_from(&mut input as *mut Slice).is_ok());
    assert_eq!(decoded.offset(), offset);
    assert_eq!(decoded.size(), size);
    assert_eq!(input.as_bytes(), suffix);
}

#[test]
fn binary_handle_varints_preserve_prefix_and_literal_wire_bytes() {
    let prefix = [0xff, 0x00, 0xc3, 0x28];
    let mut bytes = prefix.to_vec();
    handle(128, 16_384).encode_to(&mut bytes as *mut Vec<u8>);
    assert_eq!(
        bytes.as_slice(),
        &[0xff, 0x00, 0xc3, 0x28, 0x80, 0x01, 0x80, 0x80, 0x01]
    );
    assert_decodes(&bytes[prefix.len()..], 128, 16_384, &[]);
}

#[test]
fn binary_handle_largest_initialized_fields_use_literal_twenty_byte_encoding() {
    let mut bytes = Vec::new();
    handle(u64::MAX - 1, u64::MAX - 1)
        .encode_to(&mut bytes as *mut Vec<u8>);
    // u64::MAX is the existing uninitialized sentinel; MAX - 1 is
    // the largest accepted field and still occupies ten varint bytes.
    let literal = [
        0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
        0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
    ];
    assert_eq!(BLOCK_HANDLE_MAX_ENCODED_LENGTH, 20);
    assert_eq!(bytes.as_slice(), &literal);
    bytes.extend_from_slice(&[0x00, 0xff]);
    assert_decodes(&bytes, u64::MAX - 1, u64::MAX - 1, &[0x00, 0xff]);
}

#[test]
fn binary_handle_nine_and_ten_byte_boundaries_and_zero_remain_distinct() {
    let mut bytes = Vec::new();
    handle(1u64 << 63, (1u64 << 63) - 1)
        .encode_to(&mut bytes as *mut Vec<u8>);
    handle(0, 127).encode_to(&mut bytes as *mut Vec<u8>);
    assert_eq!(bytes.as_slice(), &[
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
        0x00, 0x7f,
    ]);
    assert_decodes(&bytes, 1u64 << 63, (1u64 << 63) - 1, &[0x00, 0x7f]);
    assert_decodes(&bytes[19..], 0, 127, &[]);
}
