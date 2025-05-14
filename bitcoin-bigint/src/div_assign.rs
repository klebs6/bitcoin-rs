crate::ix!();

impl<const BITS: usize> DivAssign<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// self /= other, using shift-based long division.
    #[inline]
    fn div_assign(&mut self, b: &BaseUInt<BITS>) {
        // Copy `b` so we can shift it
        let mut div = b.clone();
        // Copy `self` so we can treat it as the 'numerator'
        let mut num = self.clone();
        // We'll store the result (the quotient) in `self` by zeroing it first
        *self = BaseUInt::<BITS>::default();

        let num_bits = num.bits();
        let div_bits = div.bits();
        if div_bits == 0 {
            panic!("Division by zero in BaseUInt");
        }
        if div_bits > num_bits {
            // quotient is 0
            return;
        }
        // We'll shift `div` so that its highest set bit aligns with num's highest set bit
        let shift_amount = num_bits - div_bits;
        div <<= shift_amount;

        // Instead of "while shift as i32 >= 0", we do a downward for loop:
        for s in (0..=shift_amount).rev() {
            // If num >= div, subtract div from num, and set the bit s in self
            if num >= div {
                num -= &div;
                let limb_index = (s / 32) as usize;
                let bit_index = s % 32;
                self.pn[limb_index] |= 1 << bit_index;
            }
            div >>= 1; // shift div down by 1 bit
        }
        // remainder is in `num`, but we discard it
    }
}

#[cfg(test)]
mod div_assign_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn test_div_assign_basic_64_bits() {
        info!("Testing basic division (div_assign) in 64-bit BaseUInt with direct checks.");

        type U64 = BaseUInt<64>;

        // 1) Zero / anything_nonzero => zero
        let mut x = U64::default(); // 0
        let y = U64::from(1234u64);
        x /= &y; // => 0/1234 => 0
        assert_eq!(x.low64(), 0, "0 / nonzero should yield 0.");

        // 2) smaller / bigger => 0
        let mut a = U64::from(5u64);
        let b = U64::from(10u64);
        a /= &b; // => 5/10 => 0
        assert_eq!(a.low64(), 0, "5 / 10 => 0 for integer division.");

        // 3) normal exact: 15 / 3 => 5
        let mut c = U64::from(15u64);
        let d = U64::from(3u64);
        c /= &d;
        assert_eq!(c.low64(), 5, "15 / 3 => 5.");

        // 4) partial example: 100 / 6 => 16 (since integer division truncates)
        let mut e = U64::from(100u64);
        let f = U64::from(6u64);
        e /= &f;
        assert_eq!(e.low64(), 16, "100 / 6 => 16, remainder=4.");

        // 5) shifting example: (1<<35) / 2 => 1<<34
        let mut g = U64::default();
        g.pn[0] = 1; // now g=1
        g <<= 35;    // => 1<<35
        let two = U64::from(2u64);
        g /= &two;   // => 1<<34
        let expected = 1u64 << 34;
        assert_eq!(g.low64(), expected);

        info!("Basic 64-bit div_assign checks passed.");
    }

    #[traced_test]
    fn test_div_assign_random_64_bits() {
        info!("Testing random 64-bit division using div_assign, comparing against normal u128 fallback.");

        // We'll access our shared LCG from elsewhere in the crate:
        //   use crate::some_shared_rng::SimpleLCG;   (already in scope if it's in super::*)
        // We just pick a 64-bit seed that won't overflow:
        let mut rng = SimpleLCG::new(0x1357_9BDF_0246_8ABC);

        type U64 = BaseUInt<64>;

        for i in 0..25 {
            // We'll create two random 64-bit values, A and B, ensuring B != 0.
            let a_val = rng.next_u64();
            let mut b_val = rng.next_u64();
            if b_val == 0 {
                b_val = 1; // avoid zero to prevent panic
            }

            let mut a_bu = U64::from(a_val);
            let b_bu = U64::from(b_val);

            // We'll do standard 128-bit math in Rust for reference
            let expected = (a_val as u128).wrapping_div(b_val as u128) as u64;

            // Now do a_bu /= &b_bu
            a_bu /= &b_bu;
            let got = a_bu.low64();

            trace!(
                "Iter={} => a_val=0x{:016X}, b_val=0x{:016X}, expected_div=0x{:016X}, got=0x{:016X}",
                i,
                a_val,
                b_val,
                expected,
                got
            );

            assert_eq!(got, expected, "Mismatch in random 64-bit division");
        }

        info!("Random 64-bit div_assign checks passed.");
    }

    #[traced_test]
    fn test_div_assign_256_bits_edge_cases() {
        info!("Testing div_assign with 256-bit BaseUInt for certain edge cases.");

        type U256 = BaseUInt<256>;

        // 1) Zero / any_nonzero => zero
        let mut z = U256::default();
        let mut nonzero = U256::default();
        nonzero.pn[0] = 1; // => 1
        z /= &nonzero;
        for limb in z.pn.iter() {
            assert_eq!(*limb, 0, "0 / anything => 0 in 256-bit");
        }

        // 2) Very large numerator, smaller divisor
        //    We'll do numerator ~ 1<<255, divisor=2 => result ~ 1<<254
        let mut numerator = U256::default();
        numerator.pn[7] = 0x8000_0000; // top bit of 256 set
        let mut divisor = U256::default();
        divisor.pn[0] = 2;
        numerator /= &divisor;
        // => bit 254 set => limb[7] = 0x4000_0000
        assert_eq!(numerator.pn[7], 0x4000_0000);
        for i in 0..7 {
            assert_eq!(
                numerator.pn[i], 0,
                "other limbs should be zero after dividing 1<<255 by 2"
            );
        }

        // 3) If divisor > numerator => 0
        let mut small_num = U256::default();
        small_num.pn[0] = 0x1000;
        let mut bigger_div = U256::default();
        bigger_div.pn[1] = 1; // => ~1<<32
        small_num /= &bigger_div;
        for limb in small_num.pn.iter() {
            assert_eq!(
                *limb, 0,
                "smaller / bigger => 0 in 256-bit integer division"
            );
        }

        info!("Edge-case coverage for 256-bit div_assign completed.");
    }

    #[traced_test]
    fn test_div_assign_256_bits_random() {
        info!("Testing random 256-bit div_assign, comparing partial results to truncated 2^256 logic.");

        type U256 = BaseUInt<256>;
        let mut rng = SimpleLCG::new(0xAABB_CCdd_0011_2233);

        for i in 0..30 {
            // We'll only fill the lower 64 bits randomly so we can compare to 128-bit reference easily.
            let a_val = rng.next_u64();
            let b_val = rng.next_u64() | 1; // ensure nonzero

            let mut a = U256::default();
            a.pn[0] = (a_val & 0xFFFF_FFFF) as u32;
            a.pn[1] = ((a_val >> 32) & 0xFFFF_FFFF) as u32;

            let mut b = U256::default();
            b.pn[0] = (b_val & 0xFFFF_FFFF) as u32;
            b.pn[1] = ((b_val >> 32) & 0xFFFF_FFFF) as u32;

            // Reference in 128 bits:
            let big128_a = a_val as u128;
            let big128_b = b_val as u128;
            let expected_64 = (big128_a.wrapping_div(big128_b)) as u64;

            let mut copy_a = a.clone();
            copy_a /= &b;

            // Compare just the low 64 bits
            let got_64 = {
                let low0 = copy_a.pn[0] as u64;
                let low1 = copy_a.pn[1] as u64;
                (low1 << 32) | low0
            };

            trace!(
                "Iter={}, a_val=0x{:016X}, b_val=0x{:016X}, expected_64=0x{:016X}, got_64=0x{:016X}",
                i,
                a_val,
                b_val,
                expected_64,
                got_64
            );
            assert_eq!(
                got_64, expected_64,
                "Mismatch in truncated 64-bit portion of 256-bit / operation"
            );
        }

        info!("Random 256-bit div_assign tests passed for truncated comparisons.");
    }
}
