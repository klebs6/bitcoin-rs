// ---------------- [ File: bitcoin-bigint/src/ord.rs ]
crate::ix!();

// ---------------------------------------------------------------------------
// 5) Macro for Ord, Eq, PartialOrd, PartialEq
// ---------------------------------------------------------------------------
#[macro_export]
macro_rules! define_base_uint_ord_eq {
    ($name:ident, $bits:expr, $limbs:expr) => {

        impl core::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                for i in (0..$limbs).rev() {
                    if self.pn[i] < other.pn[i] {
                        return core::cmp::Ordering::Less;
                    } else if self.pn[i] > other.pn[i] {
                        return core::cmp::Ordering::Greater;
                    }
                }
                core::cmp::Ordering::Equal
            }
        }

        impl core::cmp::PartialOrd<$name> for $name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.pn == other.pn
            }
        }
        impl core::cmp::Eq for $name {}
    }
}

#[cfg(test)]
mod base_uint_ord_exhaustive_tests {
    use super::*;
    use core::cmp::Ordering;
    use crate::simple_lcg::SimpleLCG;
    use tracing::{info, debug, trace};

    #[traced_test]
    fn test_cmp_32_bits_edge_cases() {
        info!("Testing cmp, partial_cmp, eq for 32-bit BaseUInt edge cases.");
        type U32 = BaseUInt32;

        // 1) Zero vs zero => equal
        let z1 = U32::default();
        let z2 = U32::default();
        assert_eq!(z1, z2);

        // 2) Zero vs nonzero
        let mut x = U32::default();
        x.pn[0] = 1;
        assert_ne!(z1, x);
        assert!(z1 < x);
        assert!(x > z1);

        // 3) both nonzero => e.g. 0x1234 vs 0x1235
        let mut a = U32::default();
        a.pn[0] = 0x1234;
        let mut b = U32::default();
        b.pn[0] = 0x1235;
        assert!(a < b);
        b.pn[0] = 0x1234;
        assert_eq!(a, b);

        // 4) full-limb => 0xFFFF_FFFF vs. 0xFFFF_FFFE
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
        info!("Testing cmp for 64-bit BaseUInt edge cases.");
        type U64B = BaseUInt64;
        let z1 = U64B::default();
        let z2 = U64B::default();
        assert_eq!(z1, z2);

        let mut a = U64B::default();
        a.pn[0] = 1;
        assert!(z1 < a);

        let mut x = U64B::default();
        x.pn[1] = 1;
        assert!(x > z1);

        let mut y = U64B::default();
        y.pn[1] = 2;
        assert!(y > x);

        let mut w = U64B::default();
        w.pn[0] = 0xAAAA_BBBB;
        w.pn[1] = 0xCCCC_DDDD;
        let mut v = w.clone();
        assert_eq!(w, v);
        v.pn[0] ^= 1;
        if v.pn[0] < w.pn[0] {
            assert!(v < w);
        } else {
            assert!(v > w);
        }

        info!("64-bit cmp edge-case tests passed.");
    }

    #[traced_test]
    fn test_cmp_256_bits_edge_cases() {
        info!("Testing cmp for 256-bit BaseUInt edge cases.");
        type U256 = BaseUInt256;

        let a = U256::default();
        let b = U256::default();
        assert_eq!(a, b);

        let mut c = U256::default();
        c.pn[7] = 1;
        assert!(c > a);

        let mut d = U256::default();
        d.pn[7] = 1;
        d.pn[0] = 1;
        assert!(d > c);

        let mut e = U256::default();
        for i in 0..8 {
            e.pn[i] = 0xFFFF_FFFF;
        }
        assert!(e > d);

        let mut f = e.clone();
        assert_eq!(e, f);
        f.pn[3] ^= 1;
        assert_ne!(e, f);

        info!("256-bit cmp edge-case tests passed.");
    }

    #[traced_test]
    fn test_cmp_random_32_64_256() {
        info!("Testing random comparisons for 32, 64, 256 bits of BaseUInt.");
        let mut rng = SimpleLCG::new(0xABCDE_12345);

        fn limb_compare_32(lhs: &BaseUInt32, rhs: &BaseUInt32) -> Ordering {
            if lhs.pn[0] < rhs.pn[0] {
                Ordering::Less
            } else if lhs.pn[0] > rhs.pn[0] {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        fn random_32(lcg: &mut SimpleLCG) -> BaseUInt32 {
            let val = lcg.next_u64() & 0xFFFF_FFFF;
            let mut x = BaseUInt32::default();
            x.pn[0] = val as u32;
            x
        }

        fn limb_compare_64(lhs: &BaseUInt64, rhs: &BaseUInt64) -> Ordering {
            for i in (0..2).rev() {
                if lhs.pn[i] < rhs.pn[i] {
                    return Ordering::Less;
                } else if lhs.pn[i] > rhs.pn[i] {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        }
        fn random_64(lcg: &mut SimpleLCG) -> BaseUInt64 {
            let val = lcg.next_u64();
            let mut x = BaseUInt64::default();
            x.pn[0] = (val & 0xFFFF_FFFF) as u32;
            x.pn[1] = ((val >> 32) & 0xFFFF_FFFF) as u32;
            x
        }

        fn limb_compare_256(lhs: &BaseUInt256, rhs: &BaseUInt256) -> Ordering {
            for i in (0..8).rev() {
                if lhs.pn[i] < rhs.pn[i] {
                    return Ordering::Less;
                } else if lhs.pn[i] > rhs.pn[i] {
                    return Ordering::Greater;
                }
            }
            Ordering::Equal
        }
        fn random_256(lcg: &mut SimpleLCG) -> BaseUInt256 {
            let mut x = BaseUInt256::default();
            for i in 0..4 {
                let val = lcg.next_u64();
                x.pn[2 * i] = (val & 0xFFFF_FFFF) as u32;
                x.pn[2 * i + 1] = ((val >> 32) & 0xFFFF_FFFF) as u32;
            }
            x
        }

        for _ in 0..30 {
            let a = random_32(&mut rng);
            let b = random_32(&mut rng);
            let expected = limb_compare_32(&a, &b);
            assert_eq!(a.cmp(&b), expected);
        }

        for _ in 0..30 {
            let a = random_64(&mut rng);
            let b = random_64(&mut rng);
            let expected = limb_compare_64(&a, &b);
            assert_eq!(a.cmp(&b), expected);
        }

        for _ in 0..30 {
            let a = random_256(&mut rng);
            let b = random_256(&mut rng);
            let expected = limb_compare_256(&a, &b);
            assert_eq!(a.cmp(&b), expected);
        }

        info!("Random comparison tests for 32,64,256 bits passed OK.");
    }
}
