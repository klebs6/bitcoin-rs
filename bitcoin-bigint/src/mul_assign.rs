crate::ix!();

impl<const BITS: usize> core::ops::MulAssign<&BaseUInt<BITS>> for BaseUInt<BITS> 
where
    [(); BITS / 32]:,
{
    fn mul_assign(&mut self, rhs: &BaseUInt<BITS>) {
        tracing::trace!(
            "Entering mul_assign<&BaseUInt<{}>>; self={:X?}, rhs={:X?}",
            BITS,
            self.pn,
            rhs.pn
        );

        // Standard "long multiplication" in base-2^32, ignoring overflow beyond BITS bits.
        let num_limbs = BITS / 32;

        // Temporary buffer up to 2*num_limbs (but we'll only store lower `num_limbs`).
        // For BITS <= 256, 2*num_limbs <= 16, so a fixed [0u64;16] is safe; we only read up to index (num_limbs-1).
        let mut accum = [0u64; 16];

        for i in 0..num_limbs {
            let mut carry = 0u64;
            let a_i = self.pn[i] as u64;

            for j in 0..num_limbs {
                // index in accum:
                let k = i + j;
                if k >= num_limbs {
                    // we only keep mod 2^BITS, so skip storing beyond that
                    break;
                }

                let sum = accum[k]
                    .wrapping_add(a_i.wrapping_mul(rhs.pn[j] as u64))
                    .wrapping_add(carry);

                accum[k] = sum & 0xFFFF_FFFF;
                carry = sum >> 32;

                tracing::trace!(
                    "  i={}, j={}, k={}, a_i=0x{:X}, b_j=0x{:X}, sum=0x{:X}, accum[k]=0x{:X}, carry=0x{:X}",
                    i,
                    j,
                    k,
                    a_i,
                    rhs.pn[j],
                    sum,
                    accum[k],
                    carry
                );
            }
        }

        // Now copy the low `num_limbs` results back into self
        for i in 0..num_limbs {
            self.pn[i] = accum[i] as u32;
        }

        tracing::trace!(
            "Leaving mul_assign<&BaseUInt<{}>>; final self={:X?}",
            BITS,
            self.pn
        );
    }
}

impl<const BITS: usize> core::ops::MulAssign<u32> for BaseUInt<BITS> 
where
    [(); BITS / 32]:,
{
    fn mul_assign(&mut self, rhs: u32) {
        tracing::trace!(
            "Entering mul_assign<u32> with BITS={}, rhs=0x{:08X}, initial self={:X?}",
            BITS,
            rhs,
            self.pn
        );

        // We treat BaseUInt<BITS> as a little-end array of (BITS/32) limbs, each a 32-bit chunk.
        // This yields mod 2^BITS arithmetic automatically by ignoring the final carry.
        let num_limbs = BITS / 32;
        let mut carry = 0u64;

        for i in 0..num_limbs {
            let product = (self.pn[i] as u64)
                .wrapping_mul(rhs as u64)
                .wrapping_add(carry);

            let new_limb = product & 0xFFFF_FFFF;
            carry = product >> 32;

            tracing::debug!(
                "  limb={}, old=0x{:08X}, product=0x{:X}, new=0x{:08X}, carry=0x{:08X}",
                i,
                self.pn[i],
                product,
                new_limb,
                carry
            );

            self.pn[i] = new_limb as u32;
        }

        // Carry is discarded for mod 2^BITS
        tracing::trace!(
            "Leaving mul_assign<u32>; final self={:X?} (carry=0x{:08X} was dropped).",
            self.pn,
            carry
        );
    }
}

#[cfg(test)]
mod mul_assign_exhaustive_tests {
    use super::*;

    /// Helper: create a 64-bit BaseUInt from a u64.
    fn make64(val: u64) -> BaseUInt<64> {
        BaseUInt::<64>::from(val)
    }

    /// Helper: create a 256-bit BaseUInt from a u64 (low 64 bits).
    fn make256_from_u64(val: u64) -> BaseUInt<256> {
        BaseUInt::<256>::from(val)
    }

    #[traced_test]
    fn test_mul_assign_u32_64_bits() {
        info!("Testing `MulAssign<u32>` for 64-bit BaseUInt with extra step-by-step logging.");

        type U64 = BaseUInt<64>;

        // 1) multiply zero by any u32 => zero
        let mut x = U64::default();
        let multiplier = 1234u32;
        trace!(
            "Case1: x=0x{:016X}, multiplier=0x{:08X} => expected final=0",
            x.low64(),
            multiplier
        );
        x *= multiplier;
        let got = x.low64();
        assert_eq!(got, 0, "0 * any => 0");

        // 2) Basic example: 0x0000_FFFF_FFFF * 2 => check carefully
        //    (2^32 - 1) = 4294967295 decimal
        //    times 2 => 8589934590 decimal => 0x1_FFFF_FFFE
        //    So the 64-bit result is 0x0001FFFF_FFFE.
        let val = 0x0000_FFFF_FFFFu64;
        let mut y = U64::from(val);
        let m2 = 2u32;

        trace!("Case2: y=0x{:016X}, multiplier=0x{:08X}", val, m2);
        let expected_64 = {
            // do 128-bit to check
            let big = (val as u128) * (m2 as u128);
            (big & 0xFFFF_FFFF_FFFF_FFFF) as u64
        };
        trace!(
            "Case2 intermediate: big=0x{:X}, expected_64=0x{:X}",
            ((val as u128) * 2u128),
            expected_64
        );

        y *= m2;
        let got2 = y.low64();
        trace!(
            "Case2 result => got=0x{:016X}, expected=0x{:016X}",
            got2,
            expected_64
        );
        assert_eq!(
            got2, expected_64,
            "0xFFFF_FFFF * 2 => mismatch in 64 bits"
        );

        // 3) "High-limb" example: set the upper 32 bits to 0xFFFF_FFFF => multiply by 5 => see result mod 2^64
        // We'll do step-by-step debug logging.
        let mut high = U64::default();
        high.pn[1] = 0xFFFF_FFFF;
        let raw_val_64 = high.low64(); // for reference, though we know lower limb=0
        let multi = 5u32;

        // decimal perspective:
        //   top-limb is 0xFFFF_FFFF => meaning (2^32 - 1)
        //   so the 64-bit value is (2^32 - 1)<<32 => 0xFFFF_FFFF_00000000 in hex
        // We'll do a big integer multiplication: (val * multi) mod 2^64
        let big_val = (raw_val_64 as u128)
            + ((0xFFFF_FFFFu128) << 32) // reconstruct the full 64-bit from the limbs (here raw_val_64=0, so let's do it manually)
            ; 
        let product_128 = big_val.wrapping_mul(multi as u128);
        let expected_mod_64 = product_128 & 0xFFFF_FFFF_FFFF_FFFF;

        trace!(
            "Case3: upper-limb=0xFFFF_FFFF => full 64=0x{:016X}, multiplier=0x{:08X}",
            ((0xFFFF_FFFFu64 as u128) << 32) as u64, // just for clarity
            multi
        );
        trace!(
            "Case3 decimal reconstruction => big_val(dec)={}, multiplier(dec)={} => product(dec)={}",
            big_val,
            multi,
            product_128
        );
        trace!(
            "Case3 => product(hex)=0x{:X}, expected_mod_64=0x{:016X}",
            product_128,
            expected_mod_64
        );

        high *= multi;
        let got3 = high.low64();
        trace!(
            "Case3 final => got=0x{:016X}, expected=0x{:016X}",
            got3,
            expected_mod_64
        );
        assert_eq!(
            got3,
            expected_mod_64 as u64,
            "Overflowed product mismatch in 64-bit arithmetic."
        );

        // 4) multiply by zero => always zero
        let mut a = U64::from(0x1234567890ABCDEFu64);
        trace!("Case4: a=0x{:016X}, multiply by zero", a.low64());
        a *= 0u32;
        let got4 = a.low64();
        trace!("Case4 => got=0x{:016X}, expected=0", got4);
        assert_eq!(got4, 0, "Anything * 0 => 0.");

        info!("MulAssign<u32> tests (64-bit) with step-by-step logs concluded.");
    }

    #[traced_test]
    fn test_mul_assign_u32_256_bits() {
        info!("Testing `MulAssign<u32>` for 256-bit BaseUInt.");

        type U256 = BaseUInt<256>;

        // 1) 0 * b => 0
        let mut x = U256::default();
        x *= 0xFFFF_0000u32;
        for limb in x.pn.iter() {
            assert_eq!(*limb, 0, "0 * anything => 0");
        }

        // 2) small example: 1 * 1 => 1
        let mut y = U256::default();
        y.pn[0] = 1; 
        y *= 1u32;
        assert_eq!(y.pn[0], 1);
        for i in 1..8 {
            assert_eq!(y.pn[i], 0);
        }

        // 3) partial example: (1<<128) * 2 => (1<<129)
        let mut big = U256::default();
        big.pn[4] = 1; // => bit #128 set
        big *= 2u32;   // => bit #129 set
        assert_eq!(big.pn[4], 2, "bit #129 in limb[4]");
        for (i, limb) in big.pn.iter().enumerate() {
            if i != 4 {
                assert_eq!(*limb, 0, "Other limbs remain zero after mul");
            }
        }

        info!("MulAssign<u32> tests (256-bit) passed.");
    }

    #[traced_test]
    fn test_mul_assign_baseuint_64_bits() {
        info!("Testing `MulAssign(&BaseUInt<BITS>)` for 64-bit BaseUInt.");

        type U64 = BaseUInt<64>;

        // 1) 0 * anything => 0
        let mut x = U64::default();
        let y = U64::from(12345u64);
        x *= &y;
        assert_eq!(x.low64(), 0, "0 * anything => 0");

        // 2) simple small
        let mut a = U64::from(6u64);
        let b = U64::from(7u64);
        a *= &b;
        assert_eq!(a.low64(), 42, "6 * 7 => 42");

        // 3) partial overflow: 0xFFFF_FFFF * 0xFFFF_FFFF => (2^32-1)*(2^32-1) => 2^64 -2^33 +1 => truncated mod 2^64
        let mut c = U64::from(0xFFFF_FFFFu64);
        let d = U64::from(0xFFFF_FFFFu64);
        c *= &d;
        let expected = 0xFFFF_FFFE_00000001u64;
        trace!(
            "overflow test => got=0x{:016X}, expected=0x{:016X}",
            c.low64(),
            expected
        );
        assert_eq!(c.low64(), expected, "Check big multiplication in 64 bits");

        // 4) big-limb crossing: e.g. 0x00000001_00000000 * 2 => 0x00000002_00000000
        let mut high_bit = U64::default();
        high_bit.pn[1] = 1; // => 1<<32
        let factor_2 = make64(2);
        high_bit *= &factor_2;
        // => 2<<32 => limb[1]=2
        assert_eq!(high_bit.pn[1], 2);
        assert_eq!(high_bit.pn[0], 0);

        info!("mul_assign(&BaseUInt<64>) tests done.");
    }

    #[traced_test]
    fn test_mul_assign_baseuint_256_bits() {
        info!("Testing `MulAssign(&BaseUInt<BITS>)` for 256-bit BaseUInt.");

        type U256 = BaseUInt<256>;

        // 1) zero times any => zero
        let mut x = U256::default();
        let nonzero = make256_from_u64(9999);
        x *= &nonzero;
        for limb in x.pn.iter() {
            assert_eq!(*limb, 0);
        }

        // 2) small example
        let mut a = make256_from_u64(12);
        let b = make256_from_u64(34);
        a *= &b; 
        // => 408
        assert_eq!(a.low64(), 408, "12 * 34 => 408 in low64");
        // other limbs => 0
        for i in 2..8 {
            assert_eq!(a.pn[i], 0);
        }

        // 3) partial overflow across multiple limbs
        // We'll do (1<<128) * (1<<128) => 1<<256 => 0 mod 2^256
        let mut c = U256::default();
        c.pn[4] = 1; // => 1<<128
        let mut d = U256::default();
        d.pn[4] = 1; // => 1<<128
        c *= &d;     // => 1<<256 => 0 in 256 bits
        for limb in c.pn.iter() {
            assert_eq!(*limb, 0, "overflow => 0 in mod 2^256 arithmetic");
        }
        // Another example: (1<<127)*2 => 1<<128
        let mut e = U256::default();
        e.pn[3] = 0x8000_0000; // => bit #127 set
        let two = make256_from_u64(2);
        e *= &two;
        // => bit #128 => that is limb index=4 => 1
        assert_eq!(e.pn[3], 0, "bit #127 cleared now");
        assert_eq!(e.pn[4], 1, "bit #128 set in the next limb above");

        info!("mul_assign(&BaseUInt<256>) tests complete.");
    }

    #[traced_test]
    fn test_mul_assign_u32_random_64_bits() {
        info!("Testing random `MulAssign<u32>` in 64 bits; partial overflow verification with detailed logs.");

        let mut rng = SimpleLCG::new(0xABCD_1234_EF01_5678);
        type U64 = BaseUInt<64>;

        for i in 0..20 {
            let val64 = rng.next_u64();
            let val32 = (rng.next_u64() & 0xFFFF_FFFF) as u32;

            // step-by-step debug
            trace!("Random iteration i={} => val64=0x{:016X}, val32=0x{:08X}", i, val64, val32);

            let mut x = U64::from(val64);

            // do the 128-bit reference multiplication
            let ref_128 = (val64 as u128).wrapping_mul(val32 as u128);
            let truncated_64 = (ref_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            trace!(
                " ref_128=0x{:X}, truncated_64=0x{:016X} (decimal={})",
                ref_128,
                truncated_64,
                truncated_64
            );

            x *= val32;
            let got_64 = x.low64();

            trace!(
                "After mul_assign => got_64=0x{:016X}, expected=0x{:016X}",
                got_64,
                truncated_64
            );
            assert_eq!(got_64, truncated_64, "64-bit mul_assign(u32) mismatch");
        }

        info!("Random tests for mul_assign(u32) in 64 bits passed.");
    }

    #[traced_test]
    fn test_mul_assign_u32_random_256_bits() {
        info!("Testing random `MulAssign<u32>` in 256 bits. We'll only check the truncated lower 128 bits for reference, plus logs.");

        let mut rng = SimpleLCG::new(0xFFFF_9999_0000_2222);
        type U256 = BaseUInt<256>;

        for i in 0..20 {
            let val64 = rng.next_u64();
            let val32 = (rng.next_u64() & 0xFFFF_FFFF) as u32;
            trace!("Random iteration i={}, val64=0x{:016X}, val32=0x{:08X}", i, val64, val32);

            let mut x = make256_from_u64(val64);

            // we do a 128-bit reference for the "lower half" check
            let product_128 = (val64 as u128).wrapping_mul(val32 as u128);
            let truncated_64 = (product_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            trace!(
                " product_128=0x{:X}, truncated_64=0x{:016X}",
                product_128,
                truncated_64
            );

            x *= val32;
            let got_64 = x.low64();

            trace!(
                "Got low64=0x{:016X}, expected=0x{:016X}",
                got_64,
                truncated_64
            );
            assert_eq!(got_64, truncated_64, "Mismatch in low64 after mul_assign(u32)");
        }

        info!("Random tests for mul_assign(u32) in 256 bits completed.");
    }

    #[traced_test]
    fn test_mul_assign_baseuint_random_64_bits() {
        info!("Testing random `mul_assign(&BaseUInt<64>)` with reference in 128 bits for lower 64 check, with logs.");

        let mut rng = SimpleLCG::new(0x1349_8756_ABCD_0001);
        type U64 = BaseUInt<64>;

        for i in 0..25 {
            let a_val = rng.next_u64();
            let b_val = rng.next_u64();

            let mut a = U64::from(a_val);
            let b = U64::from(b_val);

            let product_128 = (a_val as u128).wrapping_mul(b_val as u128);
            let truncated_64 = (product_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            trace!(
                "i={}, a_val=0x{:016X}, b_val=0x{:016X}, product_128=0x{:X}, truncated_64=0x{:016X}",
                i,
                a_val,
                b_val,
                product_128,
                truncated_64
            );

            a *= &b;
            let got_64 = a.low64();

            trace!(" => got_64=0x{:016X}, expected=0x{:016X}", got_64, truncated_64);
            assert_eq!(got_64, truncated_64, "Mismatch in 64-bit mul_assign");
        }

        info!("Random tests for mul_assign(&BaseUInt<64>) done.");
    }

    #[traced_test]
    fn test_mul_assign_baseuint_random_256_bits() {
        info!("Testing random `mul_assign(&BaseUInt<256>)` for partial checks on lower bits, with logs.");

        let mut rng = SimpleLCG::new(0x4444_5555_6666_7777);

        for i in 0..20 {
            let mut a = BaseUInt::<256>::default();
            let mut b = BaseUInt::<256>::default();

            let a_val = rng.next_u64();
            let b_val = rng.next_u64();

            a.pn[0] = (a_val & 0xFFFF_FFFF) as u32;
            a.pn[1] = ((a_val >> 32) & 0xFFFF_FFFF) as u32;

            b.pn[0] = (b_val & 0xFFFF_FFFF) as u32;
            b.pn[1] = ((b_val >> 32) & 0xFFFF_FFFF) as u32;

            let product_128 = (a_val as u128).wrapping_mul(b_val as u128);
            let truncated_64 = (product_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            trace!(
                "i={}, a_val=0x{:016X}, b_val=0x{:016X}, product_128=0x{:X}, truncated_64=0x{:016X}",
                i,
                a_val,
                b_val,
                product_128,
                truncated_64
            );

            a *= &b;
            let got_64 = a.low64();

            trace!(" => got_64=0x{:016X}, expected=0x{:016X}", got_64, truncated_64);
            assert_eq!(got_64, truncated_64, "Mismatch in low64 for 256-bit mul_assign");
        }

        info!("Random tests for mul_assign(&BaseUInt<256>) completed.");
    }
}
