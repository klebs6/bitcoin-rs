use bitcoin_asmap::*;
use bitcoin_imports::*;

/// Build the minimal ASMAP `[RETURN 42]`, **padded to a full‑byte boundary**.
///
/// Padding the vector until `len % 8 == 0` guarantees that when we serialise
/// the bits back to bytes for `decode_asmap_roundtrip_file_io`, we do **not**
/// introduce implicit zero‑bits that the decoder cannot distinguish from
/// meaningful data. This aligns the expected bit‑length with what the decoder
/// inevitably produces (a multiple of eight).
fn minimal_return_42() -> Vec<bool> {
    // Instruction RETURN is encoded with a single 0‑bit.
    let mut bits = vec![false]; // RETURN opcode

    // ASN 42: choose the mantissa path (indicator 0) and encode 41 (42‑1) in 15 bits.
    bits.push(false); // indicator bit for ASN
    let mantissa: u16 = 41;
    for i in (0..15).rev() {
        bits.push(((mantissa >> i) & 1) != 0);
    }

    // --- new padding logic --------------------------------------------------
    while bits.len() % 8 != 0 {
        bits.push(false); // zero‑padding to next byte boundary
    }
    // -----------------------------------------------------------------------

    bits
}

#[traced_test]
fn decode_bits_minimal_path() {
    let bits = vec![false, false, false]; // minval=0, bit_sizes=[0], expect 0
    let mut pos = 0;
    let val = decode_bits(&bits, &mut pos, 0, &[0_u8]);
    assert_eq!(val, 0);
}

#[traced_test]
fn count_bits_roundtrip() {
    let value: u32 = 0b1011_0001;
    assert_eq!(count_bits(value), 4);
}

#[traced_test]
fn sanity_accepts_minimal_return() {
    let asmap = minimal_return_42();
    assert!(sanity_check_as_map(&asmap, 128));
}

#[traced_test]
fn interpret_returns_expected_asn() {
    let asmap = minimal_return_42();
    let ip = vec![false; 128]; // any IP works because map always returns 42
    let asn = interpret(&asmap, &ip);
    assert_eq!(asn, 42);
}

#[traced_test]
fn decode_asmap_roundtrip_file_io() {
    // Write the minimal asmap to a temp file and round‑trip through decode_asmap.
    let tmp_dir = std::env::temp_dir();
    let mut path = PathBuf::from(&tmp_dir);
    path.push("asmap_test.bin");

    // Serialise our bit‑vector LSB‑first into bytes.
    let bits = minimal_return_42();
    let mut byte = 0u8;
    let mut count = 0u8;
    let mut bytes_out = Vec::<u8>::new();
    for b in bits {
        if b {
            byte |= 1 << count;
        }
        count += 1;
        if count == 8 {
            bytes_out.push(byte);
            byte = 0;
            count = 0;
        }
    }
    if count != 0 {
        bytes_out.push(byte); // flush remainder
    }

    let mut f = File::create(&path).unwrap();
    f.write_all(&bytes_out).unwrap();
    drop(f); // ensure flush

    let decoded = decode_asmap(&path);
    std::fs::remove_file(&path).unwrap();

    assert!(!decoded.is_empty());
    assert_eq!(decoded.len(), minimal_return_42().len());
    assert!(sanity_check_as_map(&decoded, 128));
}
