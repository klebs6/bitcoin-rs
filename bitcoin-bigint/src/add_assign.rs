crate::ix!();

impl<const BITS: usize> AddAssign<&BaseUInt<BITS>> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Addition with carry. `self += other`
    #[inline]
    fn add_assign(&mut self, other: &BaseUInt<BITS>) {
        let mut carry = 0u64;
        for i in 0..(BITS / 32) {
            let sum = carry + self.pn[i] as u64 + other.pn[i] as u64;
            self.pn[i] = (sum & 0xffff_ffff) as u32;
            carry = sum >> 32;
        }
    }
}

impl<const BITS: usize> AddAssign<u64> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// self += u64  => construct a BaseUInt with `u64` and then add
    #[inline]
    fn add_assign(&mut self, other: u64) {
        let mut b = Self::default();
        // Put the lower 32 bits into pn[0], upper 32 bits in pn[1] if BITS >= 64
        b.pn[0] = (other & 0xffff_ffff) as u32;
        if BITS / 32 > 1 {
            b.pn[1] = ((other >> 32) & 0xffff_ffff) as u32;
        }
        *self += &b;
    }
}

#[cfg(test)]
mod base_uint_add_assign_exhaustive_tests {
    use super::*;

    /// We create several tests here to cover all edge cases for `AddAssign` with &BaseUInt<BITS>`
    /// and `AddAssign<u64>`. We do both small (32/64 bits) and larger (256 bits),
    /// plus random tests to ensure no carry or overflow scenarios are missed.
    #[traced_test]
    fn test_add_assign_32_bits_edge_cases() {
        info!("Testing `AddAssign<&BaseUInt>` and `AddAssign<u64>` for 32-bit BaseUInt edge cases.");

        type U32 = BaseUInt<32>;
        // This means we have exactly one 32-bit limb: pn[0].

        // 1) 0 + 0 => 0
        let mut x = U32::default();
        let y = U32::default();
        debug!("x=0, y=0 => x += y => x=0");
        x += &y;
        assert_eq!(x.pn[0], 0);

        // 2) 0 + 1 => 1
        let mut a = U32::default();
        a += 1u64; // AddAssign<u64> => uses only the low 32 bits
        debug!("a=0 => a+=1 => a={:?}", a);
        assert_eq!(a.pn[0], 1);

        // 3) Max + 1 => wrap to 0 (2^32)
        let mut b = U32::default();
        b.pn[0] = 0xFFFF_FFFF;
        debug!("b before +1 => 0x{:08X}", b.pn[0]);
        b += 1u64;
        debug!("b after  +1 => 0x{:08X}", b.pn[0]);
        assert_eq!(b.pn[0], 0, "Expect wrap-around for 32-bit max + 1 => 0");

        // 4) Some random partial check: e.g. 0x1234_0000 + 0x0000_5678 => 0x1234_5678
        let mut c = U32::default();
        c.pn[0] = 0x1234_0000;
        c += 0x5678u64;
        debug!("c=0x12340000 => c += 0x5678 => c=0x{:08X}", c.pn[0]);
        assert_eq!(c.pn[0], 0x1234_5678);

        info!("32-bit AddAssign edge-case tests passed.");
    }

    #[traced_test]
    fn test_add_assign_256_bits_edge_cases() {
        info!("Testing `AddAssign<&BaseUInt>` and `AddAssign<u64>` for 256-bit BaseUInt edge cases.");

        type U256 = BaseUInt<256>;

        // 1) 0 + 0 => 0
        let mut x = U256::default();
        let y = U256::default();
        x += &y;
        assert!(x.pn.iter().all(|&limb| limb == 0), "All limbs zero after 0+0");

        // 2) small + small => no carry beyond limb 0
        let mut a = U256::default();
        a += 12345u64;
        let mut b = U256::default();
        b += 6789u64;
        a += &b; // => 19134
        debug!("After add => a.low64() = {}", a.low64());
        assert_eq!(a.low64(), 12345 + 6789);

        // 3) carry from limb 0 to limb 1: 
        //    set limb0=0xFFFF_FFFF, limb1=some nonzero => see that we propagate
        let mut c = U256::default();
        c.pn[0] = 0xFFFF_FFFF;
        c.pn[1] = 0x0000_1234;
        c += 1u64; // => carry from limb0 => limb0=0, limb1=0x1235
        debug!("pn[0]=0x{:08X}, pn[1]=0x{:08X}", c.pn[0], c.pn[1]);
        assert_eq!(c.pn[0], 0);
        assert_eq!(c.pn[1], 0x0000_1235);

        // 4) full 256-bit wrap-around: set all limbs=0xFFFF_FFFF, add 1 => => 0
        let mut d = U256::default();
        for i in 0..8 {
            d.pn[i] = 0xFFFF_FFFF;
        }
        d += 1u64;
        for i in 0..8 {
            assert_eq!(d.pn[i], 0, "After wrap-around, all limbs zero");
        }

        info!("256-bit AddAssign edge-case tests passed.");
    }

    /// Now let's do some random tests to ensure correctness for large runs.
    /// We'll test 64 bits & 256 bits, as they are commonly used sizes.
    #[traced_test]
    fn test_add_assign_random_64_and_256() {
        info!("Testing random AddAssign with 64-bit and 256-bit BaseUInt, checking for truncated sums.");

        let mut rng = SimpleLCG::new(0xAABB_CCdd_eeff_1122);

        // We'll define a small function to do normal 128-bit or 512-bit adds, then compare lower bits.
        fn full_add_64(a: u64, b: u64) -> u64 {
            // do 128-bit sum => keep mod 2^64
            let sum_128 = (a as u128).wrapping_add(b as u128);
            (sum_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64
        }

        fn full_add_256(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
            // 256 bits => we do 64-bit lumps => each is mod 2^64, plus carry
            // a[0] is the least significant 64 bits, a[3] is the most significant.
            let mut carry = 0u128;
            let mut out = [0u64; 4];
            for i in 0..4 {
                let s = carry + (a[i] as u128) + (b[i] as u128);
                out[i] = (s & 0xFFFF_FFFF_FFFF_FFFF) as u64;
                carry = s >> 64;
            }
            out
        }

        // 64 bits
        for _ in 0..50 {
            let a_val = rng.next_u64();
            let b_val = rng.next_u64();
            let correct_64 = full_add_64(a_val, b_val);

            let mut a_bu = BaseUInt::<64>::from(a_val);
            a_bu += b_val; // self += u64
            let sum_low64 = a_bu.low64();
            assert_eq!(sum_low64, correct_64, "Failed random 64-bit add with carry at the mod 2^64 boundary.");
        }

        // 256 bits
        for _ in 0..50 {
            // We'll build a random 256 by combining 4 random 64s
            let a_0 = rng.next_u64();
            let a_1 = rng.next_u64();
            let a_2 = rng.next_u64();
            let a_3 = rng.next_u64();
            let b_0 = rng.next_u64();
            let b_1 = rng.next_u64();
            let b_2 = rng.next_u64();
            let b_3 = rng.next_u64();

            let a_ref = [a_0, a_1, a_2, a_3];
            let b_ref = [b_0, b_1, b_2, b_3];
            let correct_256 = full_add_256(a_ref, b_ref);

            // Construct BaseUInt<256> from these lumps
            let mut a_bu = BaseUInt::<256>::default();
            // We store each 64 => 2 limbs of 32 bits
            // pn[0..=7] => 8 limbs of 32 bits => 256 bits
            a_bu.pn[0] = (a_0 & 0xFFFF_FFFF) as u32;
            a_bu.pn[1] = ((a_0 >> 32) & 0xFFFF_FFFF) as u32;
            a_bu.pn[2] = (a_1 & 0xFFFF_FFFF) as u32;
            a_bu.pn[3] = ((a_1 >> 32) & 0xFFFF_FFFF) as u32;
            a_bu.pn[4] = (a_2 & 0xFFFF_FFFF) as u32;
            a_bu.pn[5] = ((a_2 >> 32) & 0xFFFF_FFFF) as u32;
            a_bu.pn[6] = (a_3 & 0xFFFF_FFFF) as u32;
            a_bu.pn[7] = ((a_3 >> 32) & 0xFFFF_FFFF) as u32;

            let mut b_bu = BaseUInt::<256>::default();
            b_bu.pn[0] = (b_0 & 0xFFFF_FFFF) as u32;
            b_bu.pn[1] = ((b_0 >> 32) & 0xFFFF_FFFF) as u32;
            b_bu.pn[2] = (b_1 & 0xFFFF_FFFF) as u32;
            b_bu.pn[3] = ((b_1 >> 32) & 0xFFFF_FFFF) as u32;
            b_bu.pn[4] = (b_2 & 0xFFFF_FFFF) as u32;
            b_bu.pn[5] = ((b_2 >> 32) & 0xFFFF_FFFF) as u32;
            b_bu.pn[6] = (b_3 & 0xFFFF_FFFF) as u32;
            b_bu.pn[7] = ((b_3 >> 32) & 0xFFFF_FFFF) as u32;

            // Now do add
            a_bu += &b_bu;
            // Compare with correct_256 mod 2^256
            // We'll reassemble the result from a_bu
            let r_0 = ((a_bu.pn[1] as u64) << 32) | (a_bu.pn[0] as u64);
            let r_1 = ((a_bu.pn[3] as u64) << 32) | (a_bu.pn[2] as u64);
            let r_2 = ((a_bu.pn[5] as u64) << 32) | (a_bu.pn[4] as u64);
            let r_3 = ((a_bu.pn[7] as u64) << 32) | (a_bu.pn[6] as u64);

            assert_eq!(r_0, correct_256[0]);
            assert_eq!(r_1, correct_256[1]);
            assert_eq!(r_2, correct_256[2]);
            assert_eq!(r_3, correct_256[3], "256-bit random add mismatch in the top 64 bits.");
        }

        info!("Random add_assign tests for 64-bit & 256-bit completed successfully.");
    }

    #[traced_test]
    fn test_add_assign_64_bits_edge_cases() {
        info!("Testing `AddAssign<&BaseUInt>` and `AddAssign<u64>` for 64-bit BaseUInt edge cases.");

        type U64B = BaseUInt<64>;

        // 1) 0 + 0 => 0
        let mut x = U64B::default();
        let y = U64B::default();
        debug!("x=0, y=0 => x+=y => x=0");
        x += &y;
        assert_eq!(x.pn, [0, 0]);

        // 2) small + small => no carry
        let mut a = U64B::default();
        a += 5u64;  // => a=5
        let mut b = U64B::default();
        b += 10u64; // => b=10
        a += &b;    // => a=15
        debug!("a=5, b=10 => a+=b => a=15 => 0x{:08X} {:08X}", a.pn[1], a.pn[0]);
        assert_eq!(a.pn, [15, 0]);

        // 3) carry that crosses the 32-bit boundary but still within 64 bits:
        //    0xFFFF_FFFE + 3 => 4,294,967,294 + 3 = 4,294,967,297 => 0x0001_0000_0001
        //    => pn[0] = 0x0000_0001, pn[1] = 0x0000_0001
        let mut c = U64B::default();
        c.pn[0] = 0xFFFF_FFFE;
        c += 3u64;
        debug!(
            "0xFFFF_FFFE + 3 => pn[0]=0x{:08X}, pn[1]=0x{:08X}",
            c.pn[0],
            c.pn[1]
        );
        assert_eq!(c.pn[0], 0x0000_0001);
        assert_eq!(c.pn[1], 0x0000_0001);

        // 4) crossing into second limb from all-lower-limb bits
        //    e.g. 0xFFFF_FFFF + 1 => => 0 in pn[0], 1 in pn[1]
        let mut d = U64B::default();
        d.pn[0] = 0xFFFF_FFFF;
        d += 1u64;
        debug!(
            "(0xFFFF_FFFF + 1) => pn[0]=0x{:08X}, pn[1]=0x{:08X}",
            d.pn[0],
            d.pn[1]
        );
        assert_eq!(d.pn[0], 0);
        assert_eq!(d.pn[1], 1);

        // 5) full 64-bit wrap-around: e.g. 0xFFFF_FFFF_FFFF_FFFF + 1 => 0
        let mut e = U64B::default();
        e.pn[0] = 0xFFFF_FFFF;
        e.pn[1] = 0xFFFF_FFFF;
        e += 1u64;
        debug!(
            "(0xFFFFFFFFFFFFFFFF +1) => pn[0]=0x{:08X}, pn[1]=0x{:08X}",
            e.pn[0],
            e.pn[1]
        );
        assert_eq!(e.pn, [0, 0]);

        info!("64-bit AddAssign edge-case tests passed.");
    }
}
