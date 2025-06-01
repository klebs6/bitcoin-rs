// ---------------- [ File: bitcoin-bigint/src/mul_assign.rs ]
crate::ix!();

// ---------------------------------------------------------------------------
// 3) Macro for MulAssign (both &BaseUInt, u32, u64) and the corresponding tests
// ---------------------------------------------------------------------------
#[macro_export]
macro_rules! define_base_uint_mulassign {
    ($name:ident, $bits:expr, $limbs:expr) => {

        impl core::ops::MulAssign<&$name> for $name {
            fn mul_assign(&mut self, rhs: &Self) {
                let limb_count = $limbs; // replaced BITS/32
                trace!(
                    "{}::mul_assign => self *= rhs; initial self={:08X?}, rhs={:08X?}",
                    stringify!($name),
                    self.pn,
                    rhs.pn
                );

                // We'll accumulate into a 2*limb_count array of 64-bit partials:
                let mut accum = vec![0u64; limb_count * 2];

                for i in 0..limb_count {
                    let a_i = self.pn[i] as u64;
                    let mut carry = 0u64;
                    for j in 0..limb_count {
                        let idx = i + j;
                        let old_val = accum[idx];
                        let mul_val = a_i.wrapping_mul(rhs.pn[j] as u64);
                        let sum = old_val.wrapping_add(mul_val).wrapping_add(carry);

                        accum[idx] = sum & 0xFFFF_FFFF;
                        carry      = sum >> 32;

                        trace!(
                            "  i={}, j={}, accum[{}]: old=0x{:08X}, a_i=0x{:X}, b_j=0x{:X}, mul=0x{:X}, sum=0x{:X}, new=0x{:08X}, carry=0x{:X}",
                            i, j, idx, 
                            (old_val & 0xFFFF_FFFF), 
                            a_i, rhs.pn[j], 
                            mul_val, sum, accum[idx], carry
                        );
                    }
                    let extra_idx = i + limb_count;
                    if extra_idx < accum.len() {
                        let old_val = accum[extra_idx];
                        let sum     = old_val.wrapping_add(carry);
                        accum[extra_idx] = sum & 0xFFFF_FFFF;
                        let leftover = sum >> 32;
                        trace!(
                            "  carry-out => accum[{}] old=0x{:08X}, sum=0x{:X}, new=0x{:08X}, leftover=0x{:X}",
                            extra_idx,
                            (old_val & 0xFFFF_FFFF),
                            sum,
                            accum[extra_idx],
                            leftover
                        );
                    }
                }

                trace!("After i/j loops => accum= [");
                for (k, val) in accum.iter().enumerate() {
                    trace!("    i={}: 0x{:X}", k, val);
                }
                trace!("]");

                // copy low limbs back
                for i in 0..limb_count {
                    self.pn[i] = accum[i] as u32;
                }

                trace!("Leaving mul_assign => final self={:08X?}", self.pn);
            }
        }

        impl core::ops::MulAssign<u32> for $name {
            fn mul_assign(&mut self, rhs: u32) {
                trace!(
                    "Entering mul_assign<u32> with BITS={}, rhs=0x{:08X}, initial self={:X?}",
                    $bits,
                    rhs,
                    self.pn
                );

                let num_limbs = $limbs;
                let mut carry = 0u64;

                for i in 0..num_limbs {
                    let product = (self.pn[i] as u64)
                        .wrapping_mul(rhs as u64)
                        .wrapping_add(carry);
                    self.pn[i] = (product & 0xFFFF_FFFF) as u32;
                    carry = product >> 32;

                    debug!(
                        "  limb={}, product=0x{:X}, new=0x{:08X}, carry=0x{:08X}",
                        i,
                        product,
                        self.pn[i],
                        carry
                    );
                }

                trace!(
                    "Leaving mul_assign<u32>; final self={:X?} (carry=0x{:08X} dropped).",
                    self.pn,
                    carry
                );
            }
        }

        impl core::ops::MulAssign<u64> for $name {
            fn mul_assign(&mut self, rhs: u64) {
                let limb_count = $limbs;
                let mut carry = 0u64;

                for i in 0..limb_count {
                    let product = (self.pn[i] as u64)
                        .wrapping_mul(rhs)
                        .wrapping_add(carry);
                    self.pn[i] = product as u32;
                    carry = product >> 32;
                }
                // any final carry is discarded in mod 2^($bits)
            }
        }
    }
}

#[cfg(test)]
mod mul_assign_exhaustive_tests {
    use super::*;
    use core::ops::MulAssign;
    use tracing::{debug, error, info, trace};

    // For convenience in these tests:
    fn make64(val: u64) -> BaseUInt64 {
        BaseUInt64::from(val)
    }
    fn make256_from_u64(val: u64) -> BaseUInt256 {
        BaseUInt256::from(val)
    }

    #[traced_test]
    fn test_mul_assign_u32_64_bits() {
        info!("Testing `MulAssign<u32>` for 64-bit BaseUInt with extra step-by-step logging.");

        // type U64 = BaseUInt64; replaced by:
        type U64 = BaseUInt64;

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
        let val = 0x0000_FFFF_FFFFu64;
        let mut y = U64::from(val);
        let m2 = 2u32;
        let expected_64 = {
            let big = (val as u128) * (m2 as u128);
            (big & 0xFFFF_FFFF_FFFF_FFFF) as u64
        };
        y *= m2;
        let got2 = y.low64();
        assert_eq!(got2, expected_64, "0xFFFF_FFFF * 2 => mismatch in 64 bits");

        // 3) "High-limb" example:
        let mut high = U64::default();
        high.pn[1] = 0xFFFF_FFFF;
        let raw_val_64 = high.low64();
        let multi = 5u32;
        let big_val = raw_val_64 as u128;
        let product_128 = big_val.wrapping_mul(multi as u128);
        let expected_mod_64 = product_128 & 0xFFFF_FFFF_FFFF_FFFF;
        high *= multi;
        let got3 = high.low64();
        assert_eq!(got3, expected_mod_64 as u64, "Overflowed product mismatch.");

        // 4) multiply by zero => always zero
        let mut a = U64::from(0x1234567890ABCDEFu64);
        a *= 0u32;
        let got4 = a.low64();
        assert_eq!(got4, 0, "Anything * 0 => 0.");

        info!("MulAssign<u32> tests (64-bit) concluded.");
    }

    #[traced_test]
    fn test_mul_assign_u32_256_bits() {
        info!("Testing `MulAssign<u32>` for 256-bit BaseUInt.");

        type U256 = BaseUInt256;

        // 1) 0 * b => 0
        let mut x = U256::default();
        x *= 0xFFFF_0000u32;
        for limb in x.pn.iter() {
            assert_eq!(*limb, 0);
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
        big.pn[4] = 1;
        big *= 2u32;
        assert_eq!(big.pn[4], 2);
        for (i, limb) in big.pn.iter().enumerate() {
            if i != 4 {
                assert_eq!(*limb, 0);
            }
        }

        info!("MulAssign<u32> tests (256-bit) passed.");
    }

    #[traced_test]
    fn test_mul_assign_baseuint_64_bits() {
        info!("Testing `MulAssign(&BaseUInt<BITS>)` for 64-bit BaseUInt.");

        type U64 = BaseUInt64;

        // 1) 0 * anything => 0
        let mut x = U64::default();
        let y = U64::from(12345u64);
        x *= &y;
        assert_eq!(x.low64(), 0);

        // 2) simple small
        let mut a = U64::from(6u64);
        let b = U64::from(7u64);
        a *= &b;
        assert_eq!(a.low64(), 42);

        // 3) partial overflow
        let mut c = U64::from(0xFFFF_FFFFu64);
        let d = U64::from(0xFFFF_FFFFu64);
        c *= &d;
        let expected = 0xFFFF_FFFE_00000001u64;
        assert_eq!(c.low64(), expected);

        // 4) big-limb crossing
        let mut high_bit = U64::default();
        high_bit.pn[1] = 1; // => 1<<32
        let factor_2 = make64(2);
        high_bit *= &factor_2;
        assert_eq!(high_bit.pn[1], 2);
        assert_eq!(high_bit.pn[0], 0);

        info!("mul_assign(&BaseUInt64) tests done.");
    }

    #[traced_test]
    fn test_mul_assign_baseuint_256_bits() {
        info!("Testing `MulAssign(&BaseUInt<BITS>)` for 256-bit BaseUInt.");

        type U256 = BaseUInt256;

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
        assert_eq!(a.low64(), 408);
        for i in 2..8 {
            assert_eq!(a.pn[i], 0);
        }

        // 3) partial overflow
        let mut c = U256::default();
        c.pn[4] = 1; // => 1<<128
        let mut d = U256::default();
        d.pn[4] = 1; // => 1<<128
        c *= &d;     // => 1<<256 => 0 in 256 bits
        for limb in c.pn.iter() {
            assert_eq!(*limb, 0);
        }
        let mut e = U256::default();
        e.pn[3] = 0x8000_0000; // => bit #127
        let two = make256_from_u64(2);
        e *= &two;
        assert_eq!(e.pn[3], 0);
        assert_eq!(e.pn[4], 1);

        info!("mul_assign(&BaseUInt256) tests complete.");
    }

    #[traced_test]
    fn test_mul_assign_u32_random_64_bits() {
        info!("Testing random `MulAssign<u32>` in 64 bits.");

        let mut rng = super::super::simple_lcg::SimpleLCG::new(0xABCD_1234_EF01_5678);
        type U64 = BaseUInt64;

        for i in 0..20 {
            let val64 = rng.next_u64();
            let val32 = (rng.next_u64() & 0xFFFF_FFFF) as u32;
            let mut x = U64::from(val64);

            let ref_128 = (val64 as u128).wrapping_mul(val32 as u128);
            let truncated_64 = (ref_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            x *= val32;
            let got_64 = x.low64();
            assert_eq!(got_64, truncated_64, "64-bit mul_assign(u32) mismatch, i={}", i);
        }

        info!("Random tests for mul_assign(u32) in 64 bits passed.");
    }

    #[traced_test]
    fn test_mul_assign_u32_random_256_bits() {
        info!("Testing random `MulAssign<u32>` in 256 bits.");

        let mut rng = super::super::simple_lcg::SimpleLCG::new(0xFFFF_9999_0000_2222);
        type U256 = BaseUInt256;

        for i in 0..20 {
            let val64 = rng.next_u64();
            let val32 = (rng.next_u64() & 0xFFFF_FFFF) as u32;
            let mut x = make256_from_u64(val64);

            let product_128 = (val64 as u128).wrapping_mul(val32 as u128);
            let truncated_64 = (product_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            x *= val32;
            let got_64 = x.low64();
            assert_eq!(got_64, truncated_64, "Mismatch in low64 after mul_assign(u32), i={}", i);
        }

        info!("Random tests for mul_assign(u32) in 256 bits completed.");
    }

    #[traced_test]
    fn test_mul_assign_baseuint_random_64_bits() {
        info!("Testing random `mul_assign(&BaseUInt64)` with reference in 128 bits.");

        let mut rng = super::super::simple_lcg::SimpleLCG::new(0x1349_8756_ABCD_0001);
        type U64 = BaseUInt64;

        for i in 0..25 {
            let a_val = rng.next_u64();
            let b_val = rng.next_u64();
            let mut a = U64::from(a_val);
            let b = U64::from(b_val);

            let product_128 = (a_val as u128).wrapping_mul(b_val as u128);
            let truncated_64 = (product_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            a *= &b;
            let got_64 = a.low64();
            assert_eq!(got_64, truncated_64, "Mismatch in 64-bit mul_assign, i={}", i);
        }

        info!("Random tests for mul_assign(&BaseUInt64) done.");
    }

    #[traced_test]
    fn test_mul_assign_baseuint_random_256_bits() {
        info!("Testing random `mul_assign(&BaseUInt256)` for partial checks on lower bits.");

        let mut rng = super::super::simple_lcg::SimpleLCG::new(0x4444_5555_6666_7777);

        for i in 0..20 {
            let mut a = BaseUInt256::default();
            let mut b = BaseUInt256::default();

            let a_val = rng.next_u64();
            let b_val = rng.next_u64();

            a.pn[0] = (a_val & 0xFFFF_FFFF) as u32;
            a.pn[1] = ((a_val >> 32) & 0xFFFF_FFFF) as u32;
            b.pn[0] = (b_val & 0xFFFF_FFFF) as u32;
            b.pn[1] = ((b_val >> 32) & 0xFFFF_FFFF) as u32;

            let product_128 = (a_val as u128).wrapping_mul(b_val as u128);
            let truncated_64 = (product_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            a *= &b;
            let got_64 = a.low64();
            assert_eq!(got_64, truncated_64, "Mismatch in low64 for 256-bit mul_assign, i={}", i);
        }

        info!("Random tests for mul_assign(&BaseUInt256) completed.");
    }
}
