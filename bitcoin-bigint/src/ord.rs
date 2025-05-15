// ---------------- [ File: bitcoin-bigint/src/ord.rs ]
crate::ix!();

impl<const BITS: usize> Ord for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // We want the numeric comparison. The high limb is at pn[WIDTH-1], the low limb is at pn[0].
        // So we compare from the top down.
        for i in (0..(BITS / 32)).rev() {
            if self.pn[i] < other.pn[i] {
                return Ordering::Less;
            } else if self.pn[i] > other.pn[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl<const BITS: usize> PartialOrd<BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    fn partial_cmp(&self, other: &BaseUInt<BITS>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BITS: usize> PartialEq for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    fn eq(&self, other: &Self) -> bool {
        // Compare the entire array for equality. In C++ we had memcmp, but in Rust we can just compare slices.
        self.pn == other.pn
    }
}


impl<const BITS: usize> Eq for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{}

#[cfg(test)]
mod base_uint_ord_exhaustive_tests {
    use super::*;

    /// Exhaustive tests for the comparison-related traits:
    /// - `Ord` (cmp)
    /// - `PartialOrd`
    /// - `PartialEq` & `Eq`
    ///
    /// We'll check 32-bit, 64-bit, and 256-bit `BaseUInt`, including:
    /// 1) Edge cases: zero vs. zero, zero vs. nonzero, all-limb set vs. partial.
    /// 2) Cases where the difference is in the highest limb vs. in a lower limb.
    /// 3) Random tests for partial ordering correctness.
    #[traced_test]
    fn test_cmp_32_bits_edge_cases() {
        info!("Testing cmp, partial_cmp, eq for 32-bit BaseUInt edge cases.");
        type U32 = BaseUInt<32>;

        // 1) Zero vs zero => equal
        let z1 = U32::default();
        let z2 = U32::default();
        assert_eq!(z1, z2, "Both zeros => eq");
        assert_eq!(z1.cmp(&z2), Ordering::Equal);

        // 2) Zero vs nonzero
        let mut x = U32::default();
        x.pn[0] = 1;
        assert_ne!(z1, x, "Zero vs. 1 => not eq");
        assert!(z1 < x, "0 < 1 => true");
        assert!(x > z1, "1 > 0 => true");

        // 3) both nonzero => e.g. 0x1234 vs 0x1235
        let mut a = U32::default();
        a.pn[0] = 0x1234;
        let mut b = U32::default();
        b.pn[0] = 0x1235;
        assert!(a < b, "0x1234 < 0x1235");
        assert!(b > a);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
        // also check eq
        b.pn[0] = 0x1234;
        assert_eq!(a, b, "Both =0x1234 => eq");
        assert_eq!(a.cmp(&b), Ordering::Equal);

        // 4) full-limb => 0xFFFF_FFFF vs. e.g. 0xFFFF_FFFE
        let mut ff = U32::default();
        ff.pn[0] = 0xFFFF_FFFF;
        let mut fe = U32::default();
        fe.pn[0] = 0xFFFF_FFFE;
        assert!(fe < ff);
        assert!(ff > fe);

        info!("32-bit cmp edge-case tests passed.");
    }

    #[traced_test]
    fn test_cmp_64_bits_edge_cases() {
        info!("Testing cmp, partial_cmp, eq for 64-bit BaseUInt edge cases.");
        type U64B = BaseUInt<64>;

        // 1) zero => eq with zero
        let z1 = U64B::default();
        let z2 = U64B::default();
        assert_eq!(z1, z2);

        // 2) difference in lower limb
        let mut a = U64B::default();
        a.pn[0] = 0x0000_0001;
        assert!(z1 < a);
        assert!(a > z1);

        // 3) difference in upper limb => e.g. x= 0x0000_0001_0000_0000 vs y=0
        let mut x = U64B::default();
        x.pn[1] = 1; // => 0x00000001_00000000
        assert!(x > z1);

        // 4) compare two that differ only in top limb
        let mut y = U64B::default();
        y.pn[1] = 2;
        assert!(y > x, "top-limb 2 > top-limb 1 => y > x");
        let cmpxy = x.cmp(&y);
        assert_eq!(cmpxy, Ordering::Less);

        // 5) check eq across limbs => e.g. (0xAAAA_BBBB, 0xCCCC_DDDD) for both
        let mut w = U64B::default();
        w.pn[0] = 0xAAAA_BBBB;
        w.pn[1] = 0xCCCC_DDDD;
        let mut v = w.clone();
        assert_eq!(w, v, "identical => eq");
        // change one nibble => see ordering
        v.pn[0] ^= 0x0000_0001; // e.g. 0xAAAA_BBBA => now less or greater?
        if v.pn[0] < w.pn[0] {
            assert!(v < w);
        } else {
            assert!(v > w);
        }

        info!("64-bit cmp edge-case tests passed.");
    }

    #[traced_test]
    fn test_cmp_256_bits_edge_cases() {
        info!("Testing cmp, partial_cmp, eq for 256-bit BaseUInt edge cases.");
        type U256 = BaseUInt<256>;

        // 1) all zero vs all zero => eq
        let a = U256::default();
        let b = U256::default();
        assert_eq!(a, b);

        // 2) difference in highest limb => e.g. a.pn[7] =1 => b=0 => a > b
        let mut c = U256::default();
        c.pn[7] = 1; // top limb => c > a
        assert!(c > a);
        // 3) difference in a lower limb => e.g. c vs d
        let mut d = U256::default();
        d.pn[7] = 1; 
        d.pn[0] = 1;  // d slightly bigger if top-limb eq, but lower-limb bigger => we do top-limb compare first => if same => we move next
        // Actually, because the top-limb is the same (1 vs 1), we compare next-lower-limb (6). All zero => eventually 0 vs 0 => 
        // until we get to limb 0 => 1 vs 0 => d> c
        assert!(d > c);

        // 4) full-limb set => 0xFFFF_FFFF in each => bigger than partial
        let mut e = U256::default();
        for i in 0..8 {
            e.pn[i] = 0xFFFF_FFFF;
        }
        assert!(e > d);
        assert!(d < e);

        // 5) eq check => partial
        let mut f = e.clone();
        assert_eq!(e, f);
        f.pn[3] ^= 1; // now differ
        assert_ne!(e, f);

        info!("256-bit cmp edge-case tests passed.");
    }

    /// Now let's do random tests: we randomly fill two BaseUInts, compare them,
    /// check we can replicate that with big 128 or bigger references. But simpler is to do direct limb comparisons:
    ///
    ///   - Construct "a" and "b" as random. 
    ///   - We do manual top-limb descending compare => see which is bigger, then compare with a.cmp(&b).
    #[traced_test]
    fn test_cmp_random_32_64_256() {
        info!("Testing random comparisons for 32, 64, 256 bits of BaseUInt.");

        let mut rng = SimpleLCG::new(0xABCDE_12345);

        // function to do a top-down limb compare
        fn limb_compare<const W: usize>(lhs: &BaseUInt<W>, rhs: &BaseUInt<W>) -> Ordering
        where
            [(); W / 32]:,
        {
            let count = W / 32;
            for i in (0..count).rev() {
                if lhs.pn[i] < rhs.pn[i] {
                    return Ordering::Less;
                } else if lhs.pn[i] > rhs.pn[i] {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        }

        // test a function that builds random for 32 bits
        fn random_32(lcg: &mut SimpleLCG) -> BaseUInt<32> {
            let val = lcg.next_u64() & 0xFFFF_FFFF;
            let mut x = BaseUInt::<32>::default();
            x.pn[0] = val as u32;
            x
        }
        // 64 bits
        fn random_64(lcg: &mut SimpleLCG) -> BaseUInt<64> {
            let val = lcg.next_u64();
            let mut x = BaseUInt::<64>::default();
            x.pn[0] = (val & 0xFFFF_FFFF) as u32;
            x.pn[1] = ((val >> 32) & 0xFFFF_FFFF) as u32;
            x
        }
        // 256 bits => build from 4 random u64 lumps
        fn random_256(lcg: &mut SimpleLCG) -> BaseUInt<256> {
            let mut x = BaseUInt::<256>::default();
            for i in 0..4 {
                let val = lcg.next_u64();
                x.pn[2 * i] = (val & 0xFFFF_FFFF) as u32;
                x.pn[2 * i + 1] = ((val >> 32) & 0xFFFF_FFFF) as u32;
            }
            x
        }

        // 1) 32 bits random
        for _ in 0..30 {
            let a = random_32(&mut rng);
            let b = random_32(&mut rng);
            let expected = limb_compare(&a, &b);
            assert_eq!(a.cmp(&b), expected, "random 32-bit mismatch with manual top-limb compare");
        }

        // 2) 64 bits random
        for _ in 0..30 {
            let a = random_64(&mut rng);
            let b = random_64(&mut rng);
            let expected = limb_compare(&a, &b);
            assert_eq!(a.cmp(&b), expected, "random 64-bit mismatch with manual compare");
        }

        // 3) 256 bits random
        for _ in 0..30 {
            let a = random_256(&mut rng);
            let b = random_256(&mut rng);
            let expected = limb_compare(&a, &b);
            assert_eq!(a.cmp(&b), expected, "random 256-bit mismatch with manual compare");
        }

        info!("Random comparison tests for 32, 64, 256 bits passed OK.");
    }
}
