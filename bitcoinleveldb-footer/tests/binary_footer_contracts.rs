use bitcoinleveldb_blockhandle::BlockHandle;
use bitcoinleveldb_footer::{Footer, FOOTER_ENCODED_LENGTH};
use bitcoinleveldb_slice::Slice;

const MAGIC: [u8; 8] = [0x57, 0xfb, 0x80, 0x8b, 0x24, 0x75, 0x47, 0xdb];

fn footer(meta_offset: u64, meta_size: u64, index_offset: u64, index_size: u64) -> Footer {
    let mut metaindex = BlockHandle::new();
    metaindex.set_offset(meta_offset);
    metaindex.set_size(meta_size);
    let mut index = BlockHandle::new();
    index.set_offset(index_offset);
    index.set_size(index_size);
    let mut result = Footer::default();
    result.set_metaindex_handle(metaindex);
    result.set_index_handle(index);
    result
}

fn assert_fields(actual: &Footer, expected: [u64; 4]) {
    assert_eq!(actual.metaindex_handle().offset(), expected[0]);
    assert_eq!(actual.metaindex_handle().size(), expected[1]);
    assert_eq!(actual.index_handle().offset(), expected[2]);
    assert_eq!(actual.index_handle().size(), expected[3]);
}

#[test]
fn binary_footer_prefix_padding_and_magic_match_literal_wire_bytes() {
    let prefix = [0xff, 0x00, 0xc3, 0x28];
    let mut bytes = prefix.to_vec();
    footer(128, 16_384, 0, 127).encode_to(&mut bytes as *mut Vec<u8>);
    let mut literal = prefix.to_vec();
    literal.extend_from_slice(&[0x80, 0x01, 0x80, 0x80, 0x01, 0x00, 0x7f]);
    literal.extend_from_slice(&[0; 33]);
    literal.extend_from_slice(&MAGIC);
    assert_eq!(FOOTER_ENCODED_LENGTH, 48);
    assert_eq!(bytes, literal);
    let mut input = Slice::from(&bytes[prefix.len()..]);
    let mut decoded = Footer::default();
    assert!(decoded.decode_from(&mut input as *mut Slice).is_ok());
    assert_fields(&decoded, [128, 16_384, 0, 127]);
    assert!(input.as_bytes().is_empty());
}

#[test]
fn binary_footer_maximum_handles_fill_the_region_without_padding() {
    let maximum = u64::MAX - 1;
    let mut bytes = Vec::new();
    footer(maximum, maximum, maximum, maximum)
        .encode_to(&mut bytes as *mut Vec<u8>);
    let field = [0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
    let mut literal = Vec::new();
    for _ in 0..4 {
        literal.extend_from_slice(&field);
    }
    literal.extend_from_slice(&MAGIC);
    assert_eq!(bytes, literal);
    assert_eq!(bytes.len(), 48);
    let mut input = Slice::from(bytes.as_slice());
    let mut decoded = Footer::default();
    assert!(decoded.decode_from(&mut input as *mut Slice).is_ok());
    assert_fields(&decoded, [maximum; 4]);
    assert!(input.as_bytes().is_empty());
}

#[test]
fn binary_footer_back_to_back_encodings_leave_the_exact_binary_suffix() {
    let mut bytes = Vec::new();
    footer(0, 0, 0, 0).encode_to(&mut bytes as *mut Vec<u8>);
    footer(1, 2, 3, 4).encode_to(&mut bytes as *mut Vec<u8>);
    bytes.extend_from_slice(&[0xff, 0x00, 0x80]);
    let mut literal = vec![0; 40];
    literal.extend_from_slice(&MAGIC);
    literal.extend_from_slice(&[1, 2, 3, 4]);
    literal.extend_from_slice(&[0; 36]);
    literal.extend_from_slice(&MAGIC);
    literal.extend_from_slice(&[0xff, 0x00, 0x80]);
    assert_eq!(bytes, literal);
    let mut input = Slice::from(bytes.as_slice());
    let mut first = Footer::default();
    assert!(first.decode_from(&mut input as *mut Slice).is_ok());
    assert_fields(&first, [0; 4]);
    assert_eq!(input.as_bytes(), &bytes[48..]);
    let mut second = Footer::default();
    assert!(second.decode_from(&mut input as *mut Slice).is_ok());
    assert_fields(&second, [1, 2, 3, 4]);
    assert_eq!(input.as_bytes(), &[0xff, 0x00, 0x80]);
}
