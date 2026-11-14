use bitcoinleveldb_coding::{
    put_fixed32, put_fixed64, put_varint32, put_varint64, put_length_prefixed_slice,
    get_length_prefixed_slice,
};
use bitcoinleveldb_slice::Slice;

#[test]
fn named_byte_sinks_append_literal_fixed_varint_and_binary_payloads() {
    let mut output = vec![0xffu8, 0x00, 0xc3, 0x28];
    put_fixed32(&mut output as *mut Vec<u8>, 0xff80_00fe);
    put_fixed64(&mut output as *mut Vec<u8>, 0xff80_0001_0203_04fe);
    put_varint32(&mut output as *mut Vec<u8>, u32::MAX);
    put_varint64(&mut output as *mut Vec<u8>, u64::MAX);
    let payload = [0x00u8, 0xff, 0xc3, 0x28];
    put_length_prefixed_slice(&mut output as *mut Vec<u8>, &Slice::from(payload.as_slice()));
    assert_eq!(output.as_slice(), &[
        0xffu8, 0x00, 0xc3, 0x28,
        0xfe, 0x00, 0x80, 0xff,
        0xfe, 0x04, 0x03, 0x02, 0x01, 0x00, 0x80, 0xff,
        0xff, 0xff, 0xff, 0xff, 0x0f,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
        0x04, 0x00, 0xff, 0xc3, 0x28,
    ]);
}

#[test]
fn named_length_prefix_snapshots_its_live_destination_view_before_growth() {
    let mut output = vec![0xffu8, 0x00, 0x80, 0xc3, 0x28];
    // Fill the allocation's reported capacity without allocating more first.
    // Appending this full live view then necessarily exceeds that capacity.
    let initial_capacity = output.capacity();
    output.resize(initial_capacity, 0xa5);
    let expected = output.clone();
    let input = Slice::from(output.as_slice());
    put_length_prefixed_slice(&mut output as *mut Vec<u8>, &input);
    assert!(output.len() > initial_capacity);
    assert_eq!(&output[..expected.len()], expected.as_slice());
    let mut appended = Slice::from(&output[expected.len()..]);
    let mut decoded = Slice::default();
    assert!(get_length_prefixed_slice(&mut appended, &mut decoded));
    assert_eq!(decoded.as_bytes(), expected.as_slice());
    assert!(appended.empty());
}

#[test]
fn named_byte_sinks_reject_null_destinations_before_dereference() {
    // These are defined null-destination guards, not non-null invalid buffers.
    assert!(std::panic::catch_unwind(|| put_fixed32(core::ptr::null_mut(), 128)).is_err());
    assert!(std::panic::catch_unwind(|| put_fixed64(core::ptr::null_mut(), 128)).is_err());
    assert!(std::panic::catch_unwind(|| put_varint32(core::ptr::null_mut(), 128)).is_err());
    assert!(std::panic::catch_unwind(|| put_varint64(core::ptr::null_mut(), 128)).is_err());
    let payload = [0x00u8, 0xff];
    let value = Slice::from(payload.as_slice());
    assert!(std::panic::catch_unwind(|| {
        put_length_prefixed_slice(core::ptr::null_mut(), &value)
    }).is_err());
}
