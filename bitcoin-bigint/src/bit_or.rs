// ---------------- [ File: bitcoin-bigint/src/bit_or.rs ]
crate::ix!();

impl<const BITS: usize> BitOrAssign<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Bitwise OR assignment: `self |= other`
    #[inline]
    fn bitor_assign(&mut self, b: &BaseUInt<BITS>) {
        for i in 0..(BITS / 32) {
            self.pn[i] |= b.pn[i];
        }
    }
}

impl<const BITS: usize> BitOrAssign<u64> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Bitwise OR assignment with a 64-bit integer: `self |= some_u64`
    /// The low limb gets bits [31:0], next limb [63:32].
    #[inline]
    fn bitor_assign(&mut self, b: u64) {
        // limb 0 gets the lower 32 bits of `b`
        self.pn[0] |= (b & 0xffff_ffff) as u32;
        // limb 1 gets the higher 32 bits of `b`
        if BITS / 32 > 1 {
            self.pn[1] |= ((b >> 32) & 0xffff_ffff) as u32;
        }
    }
}

// Bitwise OR (self | &other => new BaseUInt)
impl<const BITS: usize> BitOr<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    fn bitor(self, other: &BaseUInt<BITS>) -> Self::Output {
        let mut ret = self.clone();
        ret |= other;
        ret
    }
}

#[cfg(test)]
mod base_uint_bitor_exhaustive_tests {
    use super::*;

    /// Exhaustive tests for `BitOrAssign<&BaseUInt<BITS>>` and `BitOrAssign<u64>`,
    /// plus the `BitOr<&BaseUInt>` operator returning a new BaseUInt.
    ///
    /// We verify correctness for:
    /// - 32-bit, 64-bit, and 256-bit edge cases
    /// - Random tests to confirm correct bitwise OR across multiple limbs
    #[traced_test]
    fn test_bitor_32_bits_edge_cases() {
        info!("Testing bitwise OR (`|=`) for 32-bit BaseUInt edge cases.");

        type U32 = BaseUInt<32>;
        // We only have 1 limb: pn[0].

        // 1) 0 OR 0 => 0
        let mut x = U32::default();
        let y = U32::default();
        x |= &y;
        debug!("(0 | 0) => 0x{:08X}", x.pn[0]);
        assert_eq!(x.pn[0], 0);

        // 2) 0 OR u64 => only the low 32 bits are stored
        let mut a = U32::default();
        a |= 0x1234_5678_9ABC_DEF0u64; // large 64-bit
        // For 32 bits, only the low 32 bits matter => 0x9ABC_DEF0
        assert_eq!(a.pn[0], 0x9ABC_DEF0);

        // 3) All ones OR anything => remains all ones
        let mut b = U32::default();
        b.pn[0] = 0xFFFF_FFFF;
        let c = U32::default(); // zero
        b |= &c; // still 0xFFFF_FFFF
        assert_eq!(b.pn[0], 0xFFFF_FFFF);

        // 4) Partial bits example:
        //    x=0x1234_0000, y=0x0000_9999 => (x|y)=0x1234_9999
        let mut d = U32::default();
        d.pn[0] = 0x1234_0000;
        let mut e = U32::default();
        e.pn[0] = 0x0000_9999;
        d |= &e;
        debug!("(0x12340000 | 0x00009999) => 0x{:08X}", d.pn[0]);
        assert_eq!(d.pn[0], 0x1234_9999);

        info!("32-bit OR edge-case tests passed.");
    }

    #[traced_test]
    fn test_bitor_64_bits_edge_cases() {
        info!("Testing bitwise OR for 64-bit BaseUInt edge cases.");

        type U64B = BaseUInt<64>;

        // 1) 0 OR 0 => 0
        let mut x = U64B::default();
        let y = U64B::default();
        x |= &y;
        assert_eq!(x.pn, [0, 0]);

        // 2) OR with a 64-bit integer => the low 32 bits go to pn[0], high 32 to pn[1]
        let mut a = U64B::default();
        a |= 0x1122_3344_5566_7788u64;
        // => a.pn[0] = 0x5566_7788, a.pn[1] = 0x1122_3344
        debug!("a |= 0x1122334455667788 => pn[0]={:08X}, pn[1]={:08X}", a.pn[0], a.pn[1]);
        assert_eq!(a.pn[0], 0x5566_7788);
        assert_eq!(a.pn[1], 0x1122_3344);

        // 3) OR with BaseUInt => partial bits
        //    x=0xFFFF_0000, y=0x0000_FFFF => => x=0xFFFF_FFFF
        let mut b = U64B::default();
        b.pn[0] = 0xFFFF_0000;
        let mut c = U64B::default();
        c.pn[0] = 0x0000_FFFF;
        b |= &c;
        debug!("(0xFFFF0000 | 0x0000FFFF) => pn[0]=0x{:08X}, pn[1]=0x{:08X}", b.pn[0], b.pn[1]);
        assert_eq!(b.pn[0], 0xFFFF_FFFF);
        assert_eq!(b.pn[1], 0);

        // 4) OR with partial upper bits
        //    e.g. x=0x0000_0000_1234_0000, y=0x0000_0002_0000_5678 => => top limb => 0x0000_0003, low => 0x1234_5678
        let mut d = U64B::default();
        d.pn[0] = 0x1234_0000;
        d.pn[1] = 0x0000_0001;
        let mut e = U64B::default();
        e.pn[0] = 0x0000_5678;
        e.pn[1] = 0x0000_0002;
        d |= &e;
        debug!("d OR e => d={:?}", d);
        assert_eq!(d.pn[0], 0x1234_5678);
        assert_eq!(d.pn[1], 0x0000_0003);

        info!("64-bit OR edge-case tests passed.");
    }

    #[traced_test]
    fn test_bitor_256_bits_edge_cases() {
        info!("Testing bitwise OR for 256-bit BaseUInt edge cases.");

        type U256 = BaseUInt<256>;

        // 1) 0 OR 0 => 0
        let mut x = U256::default();
        let y = U256::default();
        x |= &y;
        assert!(x.pn.iter().all(|&limb| limb == 0), "All limbs zero after 0|0");

        // 2) partial combine across multiple limbs
        let mut a = U256::default();
        // We'll set some bits in a
        a.pn[0] = 0xAAAA_AAAA;
        a.pn[1] = 0x0000_FFFF;
        a.pn[2] = 0;
        a.pn[3] = 0x1234_0000;
        a.pn[7] = 0xFFFF_0000; // top limb

        let mut b = U256::default();
        b.pn[0] = 0x5555_5555;
        b.pn[1] = 0xFFFF_0000;
        b.pn[2] = 0xFFFF_FFFF;
        b.pn[3] = 0x0000_5678;
        // top limbs of b are zero

        a |= &b;
        // Check combined
        assert_eq!(a.pn[0], 0xFFFF_FFFF); // 0xAAAA_AAAA | 0x5555_5555
        assert_eq!(a.pn[1], 0xFFFF_FFFF); // 0x0000_FFFF | 0xFFFF_0000
        assert_eq!(a.pn[2], 0xFFFF_FFFF);
        assert_eq!(a.pn[3], 0x1234_5678);
        // top limb => remains 0xFFFF_0000 OR 0 => 0xFFFF_0000
        assert_eq!(a.pn[7], 0xFFFF_0000);

        // 3) OR with a large u64 => merges only limb0, limb1
        let mut c = U256::default();
        c.pn[4] = 0xAAAA_BBBB; // unaffected by 64-bit or
        c |= 0x1122_3344_5566_7788u64;
        assert_eq!(c.pn[0], 0x5566_7788);
        assert_eq!(c.pn[1], 0x1122_3344);
        // check limb[4] => unchanged
        assert_eq!(c.pn[4], 0xAAAA_BBBB);

        info!("256-bit OR edge-case tests passed.");
    }

    /// Random tests for 64 and 256 bits. We'll do a reference OR in normal 64 bits or in 4x64 lumps for 256.
    #[traced_test]
    fn test_bitor_random_64_and_256() {
        info!("Testing bitwise OR with random data in 64-bit & 256-bit BaseUInt.");

        let mut rng = SimpleLCG::new(0x1122_3344_5566_7788);

        // 64-bit random
        for _ in 0..50 {
            let a_val = rng.next_u64();
            let b_val = rng.next_u64();
            let expected64 = a_val | b_val;

            // build BaseUInt<64> from a_val
            let mut a_bu = BaseUInt::<64>::default();
            a_bu.pn[0] = (a_val & 0xFFFF_FFFF) as u32;
            a_bu.pn[1] = ((a_val >> 32) & 0xFFFF_FFFF) as u32;
            // do OR with b_val
            a_bu |= b_val;

            // read result
            let got64 = ((a_bu.pn[1] as u64) << 32) | (a_bu.pn[0] as u64);
            assert_eq!(got64, expected64, "64-bit random OR mismatch");
        }

        // 256-bit random
        for _ in 0..50 {
            let a0 = rng.next_u64();
            let a1 = rng.next_u64();
            let a2 = rng.next_u64();
            let a3 = rng.next_u64();
            let b0 = rng.next_u64();
            let b1 = rng.next_u64();
            let b2 = rng.next_u64();
            let b3 = rng.next_u64();

            let r0 = a0 | b0;
            let r1 = a1 | b1;
            let r2 = a2 | b2;
            let r3 = a3 | b3;

            let mut a_bu = BaseUInt::<256>::default();
            a_bu.pn[0] = (a0 & 0xFFFF_FFFF) as u32;
            a_bu.pn[1] = ((a0 >> 32) & 0xFFFF_FFFF) as u32;
            a_bu.pn[2] = (a1 & 0xFFFF_FFFF) as u32;
            a_bu.pn[3] = ((a1 >> 32) & 0xFFFF_FFFF) as u32;
            a_bu.pn[4] = (a2 & 0xFFFF_FFFF) as u32;
            a_bu.pn[5] = ((a2 >> 32) & 0xFFFF_FFFF) as u32;
            a_bu.pn[6] = (a3 & 0xFFFF_FFFF) as u32;
            a_bu.pn[7] = ((a3 >> 32) & 0xFFFF_FFFF) as u32;

            let mut b_bu = BaseUInt::<256>::default();
            b_bu.pn[0] = (b0 & 0xFFFF_FFFF) as u32;
            b_bu.pn[1] = ((b0 >> 32) & 0xFFFF_FFFF) as u32;
            b_bu.pn[2] = (b1 & 0xFFFF_FFFF) as u32;
            b_bu.pn[3] = ((b1 >> 32) & 0xFFFF_FFFF) as u32;
            b_bu.pn[4] = (b2 & 0xFFFF_FFFF) as u32;
            b_bu.pn[5] = ((b2 >> 32) & 0xFFFF_FFFF) as u32;
            b_bu.pn[6] = (b3 & 0xFFFF_FFFF) as u32;
            b_bu.pn[7] = ((b3 >> 32) & 0xFFFF_FFFF) as u32;

            a_bu |= &b_bu;

            // reconstruct
            let rr0 = ((a_bu.pn[1] as u64) << 32) | (a_bu.pn[0] as u64);
            let rr1 = ((a_bu.pn[3] as u64) << 32) | (a_bu.pn[2] as u64);
            let rr2 = ((a_bu.pn[5] as u64) << 32) | (a_bu.pn[4] as u64);
            let rr3 = ((a_bu.pn[7] as u64) << 32) | (a_bu.pn[6] as u64);

            assert_eq!(rr0, r0, "256-bit random OR mismatch in limb0");
            assert_eq!(rr1, r1, "256-bit random OR mismatch in limb1");
            assert_eq!(rr2, r2, "256-bit random OR mismatch in limb2");
            assert_eq!(rr3, r3, "256-bit random OR mismatch in limb3");
        }

        info!("Random OR tests for 64-bit & 256-bit completed successfully.");
    }

    /// Test the `BitOr` operator returning a new BaseUInt.
    #[traced_test]
    fn test_bitor_operator_new() {
        info!("Testing `self | &other => new BaseUInt` with a few small examples.");

        // 1) 64 bits => x=0xFFFF_0000, y=0x1234_5678 => z=(x|y)=0xFFFF_5678
        let x = {
            let mut tmp = BaseUInt::<64>::default();
            tmp.pn[0] = 0xFFFF_0000;
            tmp
        };
        let y = {
            let mut tmp = BaseUInt::<64>::default();
            tmp.pn[0] = 0x1234_5678;
            tmp
        };
        let z = x | &y;
        assert_eq!(z.pn[0], 0xFFFF_5678);
        assert_eq!(z.pn[1], 0);

        // 2) 32 bits => x=0xA5A5_A5A5, y=0x0F0F_0F0F => => z => 0xAFAF_AFAF
        {
            let mut x32 = BaseUInt::<32>::default();
            x32.pn[0] = 0xA5A5_A5A5;
            let mut y32 = BaseUInt::<32>::default();
            y32.pn[0] = 0x0F0F_0F0F;
            let z32 = x32 | &y32;
            assert_eq!(z32.pn[0], 0xAFAF_AFAF);
        }

        info!("`BitOr` operator returning a new BaseUInt tested OK.");
    }
}
