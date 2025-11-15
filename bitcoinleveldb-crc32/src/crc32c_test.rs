// ---------------- [ File: bitcoinleveldb-crc32/src/crc32c_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/crc32c_test.cc]

#[cfg(test)]
mod crc32c_port_spec {
    use super::*;

    #[inline]
    fn crc_of_bytes(data: &[u8]) -> u32 {
        crc32c_value(data.as_ptr(), data.len())
    }

    #[inline]
    fn make_deterministic_bytes(len: usize) -> Vec<u8> {
        // Simple deterministic xorshift-based generator to avoid external deps.
        let mut x: u32 = 0x1234_5678;
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            out.push((x & 0xff) as u8);
        }
        out
    }

    #[traced_test]
    fn crc32c_matches_rfc3720_standard_vectors() {
        // 32 zero bytes
        let buf_zeros = [0u8; 32];
        let crc_zeros = crc_of_bytes(&buf_zeros);
        assert_eq!(crc_zeros, 0x8a91_36aa, "CRC of 32 zeros");

        // 32 bytes of 0xff
        let buf_ff = [0xffu8; 32];
        let crc_ff = crc_of_bytes(&buf_ff);
        assert_eq!(crc_ff, 0x62a8_ab43, "CRC of 32 0xff bytes");

        // buf[i] = i
        let mut buf_inc = [0u8; 32];
        for (i, b) in buf_inc.iter_mut().enumerate() {
            *b = i as u8;
        }
        let crc_inc = crc_of_bytes(&buf_inc);
        assert_eq!(crc_inc, 0x46dd_794e, "CRC of 0..31");

        // buf[i] = 31 - i
        let mut buf_dec = [0u8; 32];
        for (i, b) in buf_dec.iter_mut().enumerate() {
            *b = (31 - i) as u8;
        }
        let crc_dec = crc_of_bytes(&buf_dec);
        assert_eq!(crc_dec, 0x113f_db5c, "CRC of 31..0");

        // 48-byte test vector from crc32c test
        let data: [u8; 48] = [
            0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
            0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let crc_vec = crc_of_bytes(&data);
        assert_eq!(crc_vec, 0xd996_3a56, "CRC of RFC-style 48-byte vector");
    }

    #[traced_test]
    fn crc32c_distinguishes_different_inputs() {
        let a = crc_of_bytes(b"a");
        let foo = crc_of_bytes(b"foo");
        assert_ne!(a, foo, "different inputs must have different CRCs");
    }

    #[traced_test]
    fn crc32c_extend_matches_single_shot_value() {
        let full = crc_of_bytes(b"hello world");

        let prefix = b"hello ";
        let suffix = b"world";

        let crc_prefix = crc32c_value(prefix.as_ptr(), prefix.len());
        let extended = crc32c_extend(crc_prefix, suffix.as_ptr(), suffix.len());

        assert_eq!(full, extended, "extend(crc(prefix), suffix) must equal crc(prefix||suffix)");
    }

    #[traced_test]
    fn crc32c_mask_and_unmask_roundtrip_behaviour() {
        let crc = crc_of_bytes(b"foo");

        let masked = crc32c_mask(crc);
        let masked_twice = crc32c_mask(masked);
        assert_ne!(crc, masked, "mask must change the crc");
        assert_ne!(crc, masked_twice, "double mask must still differ from original");

        let unmasked_once = crc32c_unmask(masked);
        assert_eq!(crc, unmasked_once, "unmask(mask(crc)) must recover crc");

        let unmasked_twice = crc32c_unmask(crc32c_unmask(masked_twice));
        assert_eq!(crc, unmasked_twice, "double unmask of double mask must recover crc");
    }

    #[traced_test]
    fn crc32c_extend_is_consistent_for_all_split_positions() {
        for &len in &[0usize, 1, 2, 3, 4, 7, 16, 31, 32, 33, 64, 128] {
            let data = make_deterministic_bytes(len);
            let full_crc = crc_of_bytes(&data);

            for split in 0..=len {
                let (first, second) = data.split_at(split);
                let crc_first = crc_of_bytes(first);
                let crc_via_extend = crc32c_extend(
                    crc_first,
                    second.as_ptr(),
                    second.len(),
                );
                assert_eq!(
                    full_crc,
                    crc_via_extend,
                    "split {split} of length {len} produced mismatched CRC"
                );
            }
        }
    }

    #[traced_test]
    fn crc32c_empty_input_is_zero() {
        let empty: [u8; 0] = [];
        let crc = crc_of_bytes(&empty);
        assert_eq!(crc, 0, "CRC32C of empty buffer must be zero");
    }
}
