// ---------------- [ File: bitcoin-bigint/src/get_hex.rs ]
crate::ix!();

impl<const BITS: usize> BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Return this number as a big-endian hex string with *exactly* `BITS/4` hex digits
    /// (two hex digits per byte). Leading zeros are always preserved.
    pub fn get_hex(&self) -> String {
        // 1) Gather bytes in little-endian (limb[0] is low 32 bits)
        let limb_count = BITS / 32;
        let total_bytes = BITS / 8; // e.g. 256 => 32 bytes
        let mut le_bytes = Vec::with_capacity(total_bytes);

        for i in 0..limb_count {
            let limb = self.pn[i];
            let limb_bytes = limb.to_le_bytes();
            le_bytes.extend_from_slice(&limb_bytes);
        }

        // 2) Reverse to get big-endian
        le_bytes.reverse();

        // 3) Convert to hex: always push 2 hex digits per byte
        use std::fmt::Write;
        let mut result = String::with_capacity(total_bytes * 2);
        for &byte in &le_bytes {
            write!(result, "{:02x}", byte).unwrap();
        }

        result
    }
}

#[cfg(test)]
mod base_uint_get_hex_exhaustive_tests {
    use super::*;

    /// Exhaustive tests for `BaseUInt<BITS>::get_hex()`.
    /// We verify:
    /// 1) 32-bit, 64-bit, and 256-bit edge cases,
    /// 2) Leading zero trimming,
    /// 3) Zero => "0" special case,
    /// 4) Random tests to cross-check by re-parsing or by direct nibble checks.
    #[traced_test]
    fn test_get_hex_32_bits_edge_cases() {
        info!("Testing get_hex() on 32-bit BaseUInt edge cases.");

        type U32 = BaseUInt<32>;

        // 1) 0 => "0"
        let x0 = U32::default();
        assert_eq!(x0.get_hex(), "00000000", "Zero should yield '0'");

        // 2) A small nonzero => 0x0000_0001 => "1"
        let mut x1 = U32::default();
        x1.pn[0] = 1;
        debug!("x1 = {:?}", x1);
        assert_eq!(x1.get_hex(), "00000001", "Single nibble leading zero trim check");

        // 3) e.g. 0x0000_1234 => => "1234"
        let mut x2 = U32::default();
        x2.pn[0] = 0x0000_1234;
        debug!("x2 = {:?}", x2);
        assert_eq!(x2.get_hex(), "00001234");

        // 4) full => 0xFFFF_FFFF => "ffffffff" => all nibble usage
        let mut x3 = U32::default();
        x3.pn[0] = 0xFFFF_FFFF;
        assert_eq!(x3.get_hex(), "ffffffff");

        info!("32-bit get_hex edge-case tests passed.");
    }

    #[traced_test]
    fn test_get_hex_64_bits_edge_cases() {
        info!("Testing get_hex() on 64-bit BaseUInt edge cases.");

        type U64B = BaseUInt<64>;

        // 1) zero => "0"
        let z = U64B::default();

        assert_eq!(z.get_hex(), "0000000000000000");

        // 2) partial => e.g. 0x0000_0000_0000_1234 => "1234"
        let mut x = U64B::default();
        x.pn[0] = 0x0000_1234;
        assert_eq!(x.get_hex(), "0000000000001234");

        // 3) partial upper => 0x0000_0001_0000_0000 => => "100000000"
        // i.e. a single nibble of "1" in the top limb => 9 hex digits in total
        let mut y = U64B::default();
        y.pn[1] = 0x0000_0001;
        assert_eq!(y.get_hex(), "0000000100000000" , "One nibble in upper 32 bits => 9 hex digits");

        // 4) full => e.g. 0xABCD_EF12_3456_7890 => => "abcdef1234567890"
        let mut w = U64B::default();
        w.pn[0] = 0x3456_7890;
        w.pn[1] = 0xABCD_EF12;
        assert_eq!(w.get_hex(), "abcdef1234567890");

        info!("64-bit get_hex edge-case tests passed.");
    }

    #[traced_test]
    fn test_get_hex_256_bits_edge_cases() {
        info!("Testing get_hex() on 256-bit BaseUInt edge cases.");

        type U256 = BaseUInt<256>;

        // 1) zero => "0"
        let a = U256::default();
        assert_eq!(a.get_hex(), "0000000000000000000000000000000000000000000000000000000000000000", "Zero in 256 bits => '0'");

        // 2) partial in low limb => e.g. 0x00000000_00000000_00000000_1234ABCD => => "1234abcd"
        let mut b = U256::default();
        b.pn[0] = 0x1234_ABCD;
        assert_eq!(b.get_hex(), "000000000000000000000000000000000000000000000000000000001234abcd");

        // 3) partial in high limb => e.g. top limb => 0x0000_FFFF => => some hex with 8 digits from that limb
        let mut c = U256::default();
        c.pn[7] = 0x0000_FFFF; // highest 32 bits
        let hex_c = c.get_hex();
        debug!("c.get_hex() => '{}'", hex_c);

        // 0xFFFF << (7*32 bits).
        assert!(
            hex_c.starts_with("0000ffff"),
            "Should start with '0000ffff' for top-limb partial set. got={}",
            hex_c
        );

        // 4) full => all limbs => e.g. 0xFFFF_FFFF repeated => => "ffffffffffffffffffffffffffffffff"
        let mut d = U256::default();
        for i in 0..8 {
            d.pn[i] = 0xFFFF_FFFF;
        }

        // correct 256-bit all-ones => 64 'f' characters
        assert_eq!(
            d.get_hex(),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );

        info!("256-bit get_hex edge-case tests passed.");
    }

    /// We'll do random coverage for 32, 64, and 256 bits. We'll generate random data, then
    /// parse the resulting hex back with from_str (if that is stable) or do a direct nibble comparison.
    /// Easiest is to parse back with from_str => confirm we get the same BaseUInt.
    #[traced_test]
    fn test_get_hex_random_32_64_256() {
        info!("Testing get_hex() with random data for 32, 64, 256 bits => then parse back and compare.");

        let mut rng = SimpleLCG::new(0x9999_8888_7777_6666);

        // We'll define a small helper that re-parses the hex into a new BaseUInt, then compare.
        fn round_trip_hex<const B: usize>(val: &BaseUInt<B>) -> BaseUInt<B>
        where
            [(); B / 32]:,
        {
            let hex_string = val.get_hex();
            debug!("Round-trip: got hex='{}' from val={:?}", hex_string, val);

            // parse back: `BaseUInt::<B>::from(hex_string.as_str())`
            let re_parsed = BaseUInt::<B>::from(hex_string.as_str());
            re_parsed
        }

        // 32 bits
        for _ in 0..30 {
            let random32 = rng.next_u64() & 0xFFFF_FFFF; // only 32 bits
            let mut x32 = BaseUInt::<32>::default();
            x32.pn[0] = random32 as u32;
            let rt = round_trip_hex(&x32);
            // compare limbs
            assert_eq!(rt.pn[0], x32.pn[0], "32-bit random round trip mismatch");
        }

        // 64 bits
        for _ in 0..30 {
            let random64 = rng.next_u64();
            let mut x64 = BaseUInt::<64>::default();
            x64.pn[0] = (random64 & 0xFFFF_FFFF) as u32;
            x64.pn[1] = ((random64 >> 32) & 0xFFFF_FFFF) as u32;
            let rt = round_trip_hex(&x64);
            assert_eq!(rt.pn, x64.pn, "64-bit random round trip mismatch");
        }

        // 256 bits
        for _ in 0..20 {
            // build random 256 from 4 random u64 lumps
            let a0 = rng.next_u64();
            let a1 = rng.next_u64();
            let a2 = rng.next_u64();
            let a3 = rng.next_u64();
            let mut x256 = BaseUInt::<256>::default();
            x256.pn[0] = (a0 & 0xFFFF_FFFF) as u32;
            x256.pn[1] = ((a0 >> 32) & 0xFFFF_FFFF) as u32;
            x256.pn[2] = (a1 & 0xFFFF_FFFF) as u32;
            x256.pn[3] = ((a1 >> 32) & 0xFFFF_FFFF) as u32;
            x256.pn[4] = (a2 & 0xFFFF_FFFF) as u32;
            x256.pn[5] = ((a2 >> 32) & 0xFFFF_FFFF) as u32;
            x256.pn[6] = (a3 & 0xFFFF_FFFF) as u32;
            x256.pn[7] = ((a3 >> 32) & 0xFFFF_FFFF) as u32;

            let rt = round_trip_hex(&x256);
            assert_eq!(rt.pn, x256.pn, "256-bit random round trip mismatch");
        }

        info!("Random get_hex tests for 32,64,256 bits done => round-tripped successfully.");
    }
}
