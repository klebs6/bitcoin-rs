use bitcoinleveldb_coding::*;
use bitcoinleveldb_slice::Slice;

#[test]
fn fixed_byte_sinks_preserve_binary_prefix_and_literal_endianness() {
    let mut dst = vec![0xff, 0x00];
    put_fixed32_bytes(&mut dst, 0x80ff_007f);
    put_fixed64_bytes(&mut dst, 0xff01_0280_7f00_fefd);
    assert_eq!(dst, [
        0xff, 0x00, 0x7f, 0x00, 0xff, 0x80,
        0xfd, 0xfe, 0x00, 0x7f, 0x80, 0x02, 0x01, 0xff,
    ]);
}

#[test]
fn varint_byte_sinks_match_literal_boundaries_for_both_widths() {
    let cases: &[(u32, &[u8])] = &[
        (0, &[0]), (1, &[1]), (127, &[0x7f]), (128, &[0x80, 1]),
        (16383, &[0xff, 0x7f]), (16384, &[0x80, 0x80, 1]),
        (u32::MAX, &[0xff, 0xff, 0xff, 0xff, 0x0f]),
    ];
    for &(value, encoded) in cases {
        let mut small = vec![0xfe];
        let mut wide = small.clone();
        put_varint32_bytes(&mut small, value);
        put_varint64_bytes(&mut wide, u64::from(value));
        let mut expected = vec![0xfe];
        expected.extend_from_slice(encoded);
        assert_eq!(small, expected);
        assert_eq!(wide, expected);
    }
    let mut largest = Vec::new();
    put_varint64_bytes(&mut largest, u64::MAX);
    assert_eq!(largest, [0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 1]);
}

#[test]
fn length_prefixed_byte_sink_preserves_every_byte_and_consumes_exactly_one_record() {
    let payload: Vec<u8> = (0u8..=255).collect();
    let mut dst = vec![0xfe, 0x00];
    assert!(put_length_prefixed_slice_bytes(&mut dst, &Slice::from(payload.as_slice())));
    assert_eq!(&dst[..4], &[0xfe, 0x00, 0x80, 0x02]);
    assert_eq!(&dst[4..], payload.as_slice());
    dst.push(0x7f);
    let mut input = Slice::from(&dst[2..]);
    let mut decoded = Slice::default();
    assert!(get_length_prefixed_slice(&mut input, &mut decoded));
    assert_eq!(decoded.as_bytes(), payload.as_slice());
    assert_eq!(input.as_bytes(), &[0x7f]);
}

#[test]
fn length_prefixed_byte_sink_copies_its_previous_allocation_before_growth() {
    let mut dst: Vec<u8> = (0u8..=255).collect();
    dst.shrink_to_fit();
    let original = dst.clone();
    let view = Slice::from_ptr_len(dst.as_ptr(), dst.len());
    assert!(put_length_prefixed_slice_bytes(&mut dst, &view));
    assert_eq!(&dst[..256], original.as_slice());
    assert_eq!(&dst[256..258], &[0x80, 0x02]);
    assert_eq!(&dst[258..], original.as_slice());
}

#[test]
fn length_prefixed_byte_sink_accepts_an_empty_slice_without_dereferencing_null() {
    let mut dst = vec![0xff];
    assert!(put_length_prefixed_slice_bytes(&mut dst, &Slice::default()));
    assert_eq!(dst, [0xff, 0]);
}
