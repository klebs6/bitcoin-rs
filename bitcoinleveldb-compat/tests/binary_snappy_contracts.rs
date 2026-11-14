use bitcoinleveldb_compat::{
    snappy_compress, snappy_get_uncompressed_length, snappy_uncompress,
};

fn assert_roundtrip(encoded: &[u8], expected: &[u8]) {
    let mut length = usize::MAX;
    assert!(snappy_get_uncompressed_length(
        encoded.as_ptr(), encoded.len(), &mut length as *mut usize,
    ));
    // Bound the writable destination using the known fixture size, and
    // require the length probe to agree before asking the raw port to write.
    assert_eq!(length, expected.len());
    let mut decoded = vec![0u8; expected.len()];
    assert!(snappy_uncompress(
        encoded.as_ptr(), encoded.len(), decoded.as_mut_ptr(),
    ));
    assert_eq!(decoded.as_slice(), expected);
}

#[test]
fn binary_snappy_literal_wire_roundtrip_or_disabled_output_preservation() {
    let payload = [0xffu8, 0x00, 0xc3, 0x28];
    let before = vec![0x80u8, 0x00, 0xff];
    let mut encoded = before.clone();
    let supported = snappy_compress(
        payload.as_ptr(), payload.len(), &mut encoded as *mut Vec<u8>,
    );
    assert_eq!(supported, cfg!(feature = "leveldb_snappy"));
    if supported {
        // Raw Snappy: uncompressed length4, four-byte literal tag0x0c,
        // then the original bytes. This is binary even for valid text input.
        assert_eq!(encoded.as_slice(), &[0x04u8, 0x0c, 0xff, 0x00, 0xc3, 0x28]);
        assert_roundtrip(&encoded, &payload);
    } else {
        assert_eq!(encoded, before);
    }
}

#[test]
fn binary_snappy_self_borrowed_input_survives_destination_replacement() {
    let mut encoded = Vec::<u8>::with_capacity(4096);
    for _ in 0..512 {
        encoded.extend_from_slice(&[0xffu8, 0x00, 0xc3, 0x28]);
    }
    let before = encoded.clone();
    let expected = encoded[3..2045].to_vec();
    // The source is a valid allocated subrange of the previous destination.
    // No reference into it remains in use after the codec replaces that Vec.
    let input = unsafe { encoded.as_ptr().add(3) };
    let supported = snappy_compress(
        input, expected.len(), &mut encoded as *mut Vec<u8>,
    );
    assert_eq!(supported, cfg!(feature = "leveldb_snappy"));
    if supported {
        assert_roundtrip(&encoded, &expected);
    } else {
        assert_eq!(encoded, before);
    }
}

#[test]
fn binary_snappy_empty_input_has_literal_wire_or_preserves_disabled_output() {
    let payload = Vec::<u8>::new();
    let before = vec![0xffu8, 0x00, 0x80];
    let mut encoded = before.clone();
    let supported = snappy_compress(
        payload.as_ptr(), payload.len(), &mut encoded as *mut Vec<u8>,
    );
    assert_eq!(supported, cfg!(feature = "leveldb_snappy"));
    if supported {
        assert_eq!(encoded.as_slice(), &[0u8]);
        assert_roundtrip(&encoded, &payload);
    } else {
        assert_eq!(encoded, before);
    }
}

#[test]
fn binary_snappy_rejected_null_input_leaves_live_output_unchanged() {
    let before = vec![0xffu8, 0x00, 0x80];
    let mut encoded = before.clone();
    // Null input is an explicitly rejected boundary of the port; no
    // input allocation is read and the ordinary output remains untouched.
    assert!(!snappy_compress(
        core::ptr::null(), 0, &mut encoded as *mut Vec<u8>,
    ));
    assert_eq!(encoded, before);
}
