crate::ix!();

// Bitwise XOR (self ^ &other => new BaseUInt)
impl<const BITS: usize> BitXor<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Output = BaseUInt<BITS>;

    fn bitxor(self, other: &BaseUInt<BITS>) -> Self::Output {
        let mut ret = self.clone();
        ret ^= other;
        ret
    }
}

impl<const BITS: usize> BitXorAssign<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Bitwise XOR assignment: `self ^= other`
    #[inline]
    fn bitxor_assign(&mut self, b: &BaseUInt<BITS>) {
        for i in 0..(BITS / 32) {
            self.pn[i] ^= b.pn[i];
        }
    }
}

impl<const BITS: usize> BitXorAssign<u64> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Bitwise XOR assignment with u64: `self ^= u64`
    #[inline]
    fn bitxor_assign(&mut self, b: u64) {
        self.pn[0] ^= (b & 0xffff_ffff) as u32;
        if BITS / 32 > 1 {
            self.pn[1] ^= ((b >> 32) & 0xffff_ffff) as u32;
        }
    }
}

#[cfg(test)]
mod base_uint_bitxor_exhaustive_tests {
    use super::*;

    /// Exhaustive tests for:
    /// - `BitXorAssign<&BaseUInt<BITS>>`
    /// - `BitXorAssign<u64>`
    /// - The free `BitXor` operator (`x ^ &y`) -> new BaseUInt.
    ///
    /// We will cover 32-bit, 64-bit, and 256-bit edge cases plus random tests.
    #[traced_test]
    fn test_bitxor_32_bits_edge_cases() {
        info!("Testing bitwise XOR (`^=`) for 32-bit BaseUInt edge cases.");

        type U32 = BaseUInt<32>;

        // 1) 0 ^ 0 => 0
        let mut x = U32::default();
        let y = U32::default();
        x ^= &y;
        debug!("(0 ^ 0) => 0x{:08X}", x.pn[0]);
        assert_eq!(x.pn[0], 0);

        // 2) XOR with 0 => no change
        let mut a = U32::default();
        a.pn[0] = 0xABCD_1234;
        a ^= 0u64; // => no effect
        assert_eq!(a.pn[0], 0xABCD_1234);

        // 3) All ones ^ anything => bitwise invert
        let mut b = U32::default();
        b.pn[0] = 0xFFFF_FFFF; 
        b ^= 0x1234_5678_9ABC_DEF0u64; 
        // For 32 bits, only the lower 32 bits => 0x9ABC_DEF0
        // So b => 0xFFFF_FFFF ^ 0x9ABC_DEF0 =>  ~(0x9ABC_DEF0) => 0x6543_210F
        let expect = 0xFFFF_FFFF ^ 0x9ABC_DEF0;
        assert_eq!(b.pn[0], expect);

        // 4) partial bits
        //    x=0x1234_0000, y=0x0000_9999 => => 0x1234_9999 for OR, but for XOR => 0x1234_9999 ^ ??? ...
        //    let's do x^y => for ex:  0x1234_0000 ^ 0x0000_9999 => 0x1234_9999
        //    but only for bits that differ
        let mut d = U32::default();
        d.pn[0] = 0x1234_0000;
        let mut e = U32::default();
        e.pn[0] = 0x0000_9999;
        d ^= &e; 
        // => 0x1234_0000 ^ 0x0000_9999 = 0x1234_9999
        assert_eq!(d.pn[0], 0x1234_9999);

        info!("32-bit XOR edge-case tests passed.");
    }

    #[traced_test]
    fn test_bitxor_64_bits_edge_cases() {
        info!("Testing bitwise XOR for 64-bit BaseUInt edge cases.");

        type U64B = BaseUInt<64>;

        // 1) 0 ^ 0 => 0
        let mut x = U64B::default();
        x ^= &U64B::default();
        assert_eq!(x.pn, [0, 0]);

        // 2) XOR with a 64-bit => only low 2 limbs
        let mut a = U64B::default();
        a.pn[0] = 0xAAAA_0000;
        a.pn[1] = 0x0000_BBBB;
        a ^= 0x1122_3344_5566_7788u64;
        // => a.pn[0] ^= 0x5566_7788, a.pn[1] ^= 0x1122_3344
        let res0 = 0xAAAA_0000 ^ 0x5566_7788;
        let res1 = 0x0000_BBBB ^ 0x1122_3344;
        assert_eq!(a.pn[0], res0);
        assert_eq!(a.pn[1], res1);

        // 3) partial bits
        //    x=0xFFFF_0000, y=0x1234_5678 => x^=y => ?
        let mut b = U64B::default();
        b.pn[0] = 0xFFFF_0000;
        let mut c = U64B::default();
        c.pn[0] = 0x1234_5678;
        b ^= &c;
        let expected0 = 0xFFFF_0000 ^ 0x1234_5678;
        debug!("(0xFFFF0000 ^ 0x12345678) => 0x{:08X}", expected0);
        assert_eq!(b.pn[0], expected0);
        assert_eq!(b.pn[1], 0);

        info!("64-bit XOR edge-case tests passed.");
    }

    #[traced_test]
    fn test_bitxor_256_bits_edge_cases() {
        info!("Testing bitwise XOR for 256-bit BaseUInt edge cases.");

        type U256 = BaseUInt<256>;

        // 1) 0 ^ 0 => 0
        let mut x = U256::default();
        x ^= &U256::default();
        assert!(x.pn.iter().all(|&limb| limb == 0));

        // 2) partial multi-limb
        let mut a = U256::default();
        a.pn[0] = 0xAAAA_AAAA;
        a.pn[1] = 0x0000_FFFF;
        a.pn[7] = 0x1111_2222; // top limb
        let mut b = U256::default();
        b.pn[0] = 0x5555_5555;
        b.pn[1] = 0xFFFF_0000;
        b.pn[3] = 0x1234_5678;
        b.pn[7] = 0x8888_9999;

        a ^= &b;
        // => each limb = old ^ new
        assert_eq!(a.pn[0], 0xFFFF_FFFF);  // 0xAAAA_AAAA ^ 0x5555_5555
        assert_eq!(a.pn[1], 0xFFFF_FFFF);  // 0x0000_FFFF ^ 0xFFFF_0000
        assert_eq!(a.pn[3], 0x1234_5678);  // was 0 => ^ 0x1234_5678 => 0x1234_5678
        // top limb => 0x1111_2222 ^ 0x8888_9999 => 0x9999_BBBB
        assert_eq!(a.pn[7], 0x9999_BBBB);

        info!("256-bit XOR edge-case tests passed.");
    }

    /// Random tests for 64 bits & 256 bits. We'll do reference XOR in normal 64 or lumps of 64 for 256.
    #[traced_test]
    fn test_bitxor_random_64_and_256() {
        info!("Testing bitwise XOR with random data in 64-bit & 256-bit BaseUInt.");

        let mut rng = SimpleLCG::new(0x1234_5678_9ABC_DEF0);

        // For 64 bits
        for _ in 0..50 {
            let a_val = rng.next_u64();
            let b_val = rng.next_u64();
            let expected64 = a_val ^ b_val;

            let mut a_bu = BaseUInt::<64>::default();
            a_bu.pn[0] = (a_val & 0xFFFF_FFFF) as u32;
            a_bu.pn[1] = ((a_val >> 32) & 0xFFFF_FFFF) as u32;
            // XOR with b_val
            a_bu ^= b_val;

            // read out
            let got64 = ((a_bu.pn[1] as u64) << 32) | (a_bu.pn[0] as u64);
            assert_eq!(got64, expected64, "64-bit random XOR mismatch");
        }

        // For 256 bits
        for _ in 0..50 {
            let a0 = rng.next_u64();
            let a1 = rng.next_u64();
            let a2 = rng.next_u64();
            let a3 = rng.next_u64();

            let b0 = rng.next_u64();
            let b1 = rng.next_u64();
            let b2 = rng.next_u64();
            let b3 = rng.next_u64();

            let r0 = a0 ^ b0;
            let r1 = a1 ^ b1;
            let r2 = a2 ^ b2;
            let r3 = a3 ^ b3;

            let mut a_bu = BaseUInt::<256>::default();
            a_bu.pn[0] = (a0 & 0xFFFF_FFFF) as u32;
            a_bu.pn[1] = ((a0 >> 32) & 0xFFFF_FFFF) as u32;
            a_bu.pn[2] = (a1 & 0xFFFF_FFFF) as u32;
            a_bu.pn[3] = ((a1 >> 32) & 0xFFFF_FFFF) as u32;
            a_bu.pn[4] = (a2 & 0xFFFF_FFFF) as u32;
            a_bu.pn[5] = ((a2 >> 32) & 0xFFFF_FFFF) as u32;
            a_bu.pn[6] = (a3 & 0xFFFF_FFFF) as u32;
            a_bu.pn[7] = ((a3 >> 32) & 0xFFFF_FFFF) as u32;

            // XOR with b as baseuint
            let mut b_bu = BaseUInt::<256>::default();
            b_bu.pn[0] = (b0 & 0xFFFF_FFFF) as u32;
            b_bu.pn[1] = ((b0 >> 32) & 0xFFFF_FFFF) as u32;
            b_bu.pn[2] = (b1 & 0xFFFF_FFFF) as u32;
            b_bu.pn[3] = ((b1 >> 32) & 0xFFFF_FFFF) as u32;
            b_bu.pn[4] = (b2 & 0xFFFF_FFFF) as u32;
            b_bu.pn[5] = ((b2 >> 32) & 0xFFFF_FFFF) as u32;
            b_bu.pn[6] = (b3 & 0xFFFF_FFFF) as u32;
            b_bu.pn[7] = ((b3 >> 32) & 0xFFFF_FFFF) as u32;

            a_bu ^= &b_bu;

            // read result
            let rr0 = ((a_bu.pn[1] as u64) << 32) | (a_bu.pn[0] as u64);
            let rr1 = ((a_bu.pn[3] as u64) << 32) | (a_bu.pn[2] as u64);
            let rr2 = ((a_bu.pn[5] as u64) << 32) | (a_bu.pn[4] as u64);
            let rr3 = ((a_bu.pn[7] as u64) << 32) | (a_bu.pn[6] as u64);

            assert_eq!(rr0, r0, "256-bit random XOR mismatch, lower 64 bits");
            assert_eq!(rr1, r1, "256-bit random XOR mismatch, next 64 bits");
            assert_eq!(rr2, r2, "256-bit random XOR mismatch, next 64 bits");
            assert_eq!(rr3, r3, "256-bit random XOR mismatch, top 64 bits");
        }

        info!("Random XOR tests for 64-bit & 256-bit completed successfully.");
    }

    /// Test the free BitXor operator: `self ^ &other => new BaseUInt`.
    #[traced_test]
    fn test_bitxor_operator_new() {
        info!("Testing `self ^ &other => new BaseUInt` with some small examples.");

        // 1) 64 bits => x=0xFFFF_0000, y=0x1234_5678 => z = x^y
        let mut x = BaseUInt::<64>::default();
        x.pn[0] = 0xFFFF_0000;
        let mut y = BaseUInt::<64>::default();
        y.pn[0] = 0x1234_5678;
        let z = x ^ &y;
        let expect = 0xFFFF_0000 ^ 0x1234_5678;
        assert_eq!(z.pn[0], expect);
        assert_eq!(z.pn[1], 0);

        // 2) 32 bits => x=0xAAAA_AAAA, y=0x5555_0000 => z= x^y => ?
        {
            let mut x32 = BaseUInt::<32>::default();
            x32.pn[0] = 0xAAAA_AAAA;
            let mut y32 = BaseUInt::<32>::default();
            y32.pn[0] = 0x5555_0000;
            let z32 = x32 ^ &y32;
            let expected32 = 0xAAAA_AAAA ^ 0x5555_0000;
            assert_eq!(z32.pn[0], expected32);
        }

        info!("`BitXor` operator returning new BaseUInt tested OK.");
    }
}
