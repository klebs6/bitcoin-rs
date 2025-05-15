// ---------------- [ File: bitcoin-bigint/src/from_u64.rs ]
crate::ix!();

impl<const BITS: usize> From<u64> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Creates a BaseUInt from a 64-bit value, putting the low 32 bits in pn[0],
    /// the high 32 bits in pn[1], and 0 in the rest.
    fn from(value: u64) -> Self {
        let mut ret = Self::default();
        ret.pn[0] = (value & 0xffff_ffff) as u32;
        if BITS / 32 > 1 {
            ret.pn[1] = ((value >> 32) & 0xffff_ffff) as u32;
        }
        ret
    }
}

#[cfg(test)]
mod base_uint_from_u64_exhaustive_tests {
    use super::*;

    /// Exhaustive tests for `impl<const BITS: usize> From<u64> for BaseUInt<BITS>`.
    /// We'll check 32-bit, 64-bit, and 256-bit variants, plus random coverage.
    #[traced_test]
    fn test_from_u64_32_bits_edge_cases() {
        info!("Testing From<u64> for BaseUInt<32> edge cases.");

        type U32 = BaseUInt<32>; // single 32-bit limb

        // 1) 0 => should store 0 in pn[0]
        let x = U32::from(0u64);
        assert_eq!(x.pn[0], 0, "Zero => all bits 0 in a 32-bit container");

        // 2) small value => e.g. 0x1234_5678
        let a = U32::from(0x1234_5678u64);
        debug!("From<u64>: 0x12345678 => pn[0]=0x{:08X}", a.pn[0]);
        assert_eq!(a.pn[0], 0x1234_5678);

        // 3) large value => e.g. 0xFFFF_FFFF => still fits in 32 bits
        let b = U32::from(0xFFFF_FFFFu64);
        assert_eq!(b.pn[0], 0xFFFF_FFFF, "Should store the full 32 bits");

        // 4) if value > 0xFFFF_FFFF, we only keep low 32 bits => e.g. 0x1_2345_6789AB_CDEF
        //    => 32 bits => 0xAB_CDEF (lowest 32 bits). That is 0x6789AB_CDEF => parted out => Actually let's pick a simpler example
        let c = U32::from(0xAAAABBBB_CCCC_DDDD_u64);
        // The 64-bit literal => 0xAAAABBBB_CCCC_DDDD
        // Low 32 bits => 0xCCCC_DDDD
        debug!("From<u64>: 0xAAAABBBBCCCCDDDD => in 32 bits => pn[0]=0x{:08X}", c.pn[0]);
        assert_eq!(c.pn[0], 0xCCCC_DDDD);

        info!("32-bit From<u64> edge-case tests passed.");
    }

    #[traced_test]
    fn test_from_u64_64_bits_edge_cases() {
        info!("Testing From<u64> for BaseUInt<64> edge cases.");

        type U64B = BaseUInt<64>;

        // 1) 0 => => [0, 0]
        let x = U64B::from(0u64);
        assert_eq!(x.pn, [0, 0]);

        // 2) small => e.g. 0x1234_5678 => goes to pn[0], pn[1]=0
        let a = U64B::from(0x1234_5678u64);
        assert_eq!(a.pn[0], 0x1234_5678);
        assert_eq!(a.pn[1], 0);

        // 3) large => e.g. 0xAAAA_BBBB_CCCC_DDDD => 64 bits
        // => pn[0] = 0xCCCC_DDDD, pn[1] = 0xAAAA_BBBB
        let b = U64B::from(0xAAAA_BBBB_CCCC_DDDD_u64);
        debug!("From<u64>: 0xAAAABBBBCCCCDDDD => pn[0]=0x{:08X}, pn[1]=0x{:08X}", b.pn[0], b.pn[1]);
        assert_eq!(b.pn[0], 0xCCCC_DDDD);
        assert_eq!(b.pn[1], 0xAAAA_BBBB);

        // 4) full 64-bit max => 0xFFFF_FFFF_FFFF_FFFF => => [0xFFFF_FFFF, 0xFFFF_FFFF]
        let c = U64B::from(0xFFFF_FFFF_FFFF_FFFFu64);
        assert_eq!(c.pn[0], 0xFFFF_FFFF);
        assert_eq!(c.pn[1], 0xFFFF_FFFF);

        info!("64-bit From<u64> edge-case tests passed.");
    }

    #[traced_test]
    fn test_from_u64_256_bits_edge_cases() {
        info!("Testing From<u64> for BaseUInt<256> edge cases.");

        type U256 = BaseUInt<256>;

        // 1) 0 => all limbs 0
        let x = U256::from(0u64);
        assert!(x.pn.iter().all(|&limb| limb == 0), "All zero limbs from 0");

        // 2) small => e.g. 0x1234_5678 => => pn[0]=0x1234_5678, rest=0
        let a = U256::from(0x1234_5678u64);
        assert_eq!(a.pn[0], 0x1234_5678);
        for i in 1..8 {
            assert_eq!(a.pn[i], 0);
        }

        // 3) large => e.g. 0xAAAABBBBCCCCDDDD => pn[0]=0xCCCCDDDD, pn[1]=0xAAAA_BBBB, rest=0
        let b = U256::from(0xAAAABBBB_CCCC_DDDD_u64);
        assert_eq!(b.pn[0], 0xCCCC_DDDD);
        assert_eq!(b.pn[1], 0xAAAA_BBBB);
        for i in 2..8 {
            assert_eq!(b.pn[i], 0, "No other limbs should be set");
        }

        // 4) all bits => 0xFFFF_FFFF_FFFF_FFFF => => pn[0]=0xFFFF_FFFF, pn[1]=0xFFFF_FFFF, rest=0
        let c = U256::from(0xFFFF_FFFF_FFFF_FFFFu64);
        assert_eq!(c.pn[0], 0xFFFF_FFFF);
        assert_eq!(c.pn[1], 0xFFFF_FFFF);
        for i in 2..8 {
            assert_eq!(c.pn[i], 0);
        }

        info!("256-bit From<u64> edge-case tests passed.");
    }

    /// Random coverage for 32, 64, and 256 bits. We'll compare the result's low64 to the original
    /// if bits exceed the container, those are truncated for 32 bits but we can still verify partial.
    #[traced_test]
    fn test_from_u64_random() {
        info!("Testing From<u64> random coverage for 32, 64, 256 bits.");

        let mut rng = SimpleLCG::new(0xDEAD_BEEF_CAFE_BABE);

        // We'll do 50 random 64-bit values, parse them into BaseUInt<32>, <64>, <256>, then check:
        //  - For 32 bits, only the low 32 bits match
        //  - For 64 bits, the entire value is matched
        //  - For 256 bits, entire value matched plus the rest of limbs=0

        for _ in 0..50 {
            let val = rng.next_u64();

            // For 32 bits => only low 32 bits matter
            let x32 = BaseUInt::<32>::from(val);
            let x32_low = x32.pn[0];
            let expected_32 = (val & 0xFFFF_FFFF) as u32;
            assert_eq!(x32_low, expected_32, "32-bit truncated from 0x{:016X}", val);

            // For 64 bits => should store entire val
            let x64 = BaseUInt::<64>::from(val);
            let re64 = ((x64.pn[1] as u64) << 32) | (x64.pn[0] as u64);
            assert_eq!(re64, val, "64-bit exact parse mismatch for 0x{:016X}", val);

            // For 256 bits => store the entire val in the first two limbs, rest = 0
            let x256 = BaseUInt::<256>::from(val);
            let re256_low = ((x256.pn[1] as u64) << 32) | (x256.pn[0] as u64);
            assert_eq!(re256_low, val, "256-bit parse mismatch for 0x{:016X}", val);
            for i in 2..8 {
                assert_eq!(x256.pn[i], 0, "Higher limbs should be zero in 256-bit parse from u64");
            }
        }

        info!("Random coverage for From<u64> to 32,64,256 bits done.");
    }
}
