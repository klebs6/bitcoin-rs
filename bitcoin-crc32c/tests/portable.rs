use bitcoin_imports::*;
use bitcoin_crc32c::*;

/// Reference, branch‑per‑bit implementation of CRC‑32C (Castagnoli).
/// Used only for correctness validation in tests.
fn crc32c_naive(data: &[u8]) -> u32 {
    const POLY: u32 = 0x82f6_3b78;
    let mut crc: u32 = 0xffff_ffff;

    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (0u32.wrapping_sub(crc & 1)) & POLY;
            crc = (crc >> 1) ^ mask;
        }
    }
    crc ^ 0xffff_ffff
}

/// Safe helper that calls the portable implementation from a slice.
fn crc32c_portable(data: &[u8]) -> u32 {
    // Safety: `data.as_ptr()` is valid for `data.len()` bytes by definition.
    unsafe { crc32c_extend_portable(0, data.as_ptr(), data.len()) }
}

#[traced_test]
fn single_byte_values_match_reference() {
    for byte in 0u8..=255 {
        let buf = [byte];
        let expected = crc32c_naive(&buf);
        let got = crc32c_portable(&buf);
        assert_eq!(
            got, expected,
            "mismatch for single byte value {:#04x}: portable={:#010x} naive={:#010x}",
            byte, got, expected
        );
    }
    info!("validated all 256 single‑byte inputs");
}

#[traced_test]
fn lengths_up_to_1024_match_reference() {
    let mut data = [0u8; 1024];
    // Deterministic but “random‑looking” pattern; simple and seed‑free.
    for (i, b) in data.iter_mut().enumerate() {
        *b = (i.wrapping_mul(251) ^ 0x5d) as u8;
    }

    for len in 0..=1024 {
        let slice = &data[..len];
        let expected = crc32c_naive(slice);
        let got = crc32c_portable(slice);
        assert_eq!(
            got, expected,
            "mismatch at length {}: portable={:#010x} naive={:#010x}",
            len, got, expected
        );
    }
    info!("validated all lengths 0 … 1024");
}

#[traced_test]
fn incremental_extension_matches_one_shot() {
    let mut data = [0u8; 4096];
    for (i, b) in data.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(11);
    }

    // Split at every possible position and ensure the incremental path
    // equals the monolithic one.
    for split in 0..=data.len() {
        let (left, right) = data.split_at(split);

        let one_shot = crc32c_portable(&data);

        // First half
        let mut crc = unsafe { crc32c_extend_portable(0, left.as_ptr(), left.len()) };
        // Extend with second half
        crc = unsafe { crc32c_extend_portable(crc, right.as_ptr(), right.len()) };

        assert_eq!(
            crc, one_shot,
            "mismatch with split {} (left {}, right {}): incremental={:#010x} one‑shot={:#010x}",
            split,
            left.len(),
            right.len(),
            crc,
            one_shot
        );
    }
    debug!("incremental extension validated for all 4097 splits");
}
