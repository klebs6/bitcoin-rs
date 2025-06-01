// ---------------- [ File: bitcoin-bigint/src/add_assign.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_baseuint_addassign {
    ($uint_type:ident, $bits:expr, $limbs:expr) => {

        impl core::ops::AddAssign<&$uint_type> for $uint_type {
            /// Addition with carry. `self += other`
            #[inline]
            fn add_assign(&mut self, other: &Self) {
                let mut carry = 0u64;
                for i in 0..$limbs {
                    let sum = carry + (self.pn[i] as u64) + (other.pn[i] as u64);
                    self.pn[i] = (sum & 0xffff_ffff) as u32;
                    carry = sum >> 32;
                }
            }
        }

        impl core::ops::AddAssign<u64> for $uint_type {
            /// self += u64 => construct a temporary $uint_type from the u64, then add
            #[inline]
            fn add_assign(&mut self, other: u64) {
                let mut b = Self::default();
                // Put lower 32 bits in pn[0], upper 32 bits in pn[1] if available
                b.pn[0] = (other & 0xffff_ffff) as u32;
                if $limbs > 1 {
                    b.pn[1] = ((other >> 32) & 0xffff_ffff) as u32;
                }
                *self += &b;
            }
        }
    }
}

#[cfg(test)]
mod base_uint_add_assign_exhaustive_tests {
    use super::*;
    use tracing::{info, debug, trace};

    // We rely on macros expanded below for each type. 
    // The tests from the original code used `BaseUInt32`, `BaseUInt64`, `BaseUInt256` etc.
    // We'll replicate them using new type names: BaseUInt32, BaseUInt64, BaseUInt256, etc.

    #[traced_test]
    fn test_add_assign_32_bits_edge_cases() {
        info!("Testing `AddAssign<&BaseUInt>` and `AddAssign<u64>` for 32-bit BaseUInt edge cases.");

        type U32 = BaseUInt32; // We define this type at the bottom expansions

        // 1) 0 + 0 => 0
        let mut x = U32::default();
        let y = U32::default();
        debug!("x=0, y=0 => x += y => x=0");
        x += &y;
        assert_eq!(x.pn[0], 0);

        // 2) 0 + 1 => 1
        let mut a = U32::default();
        a += 1u64; 
        debug!("a=0 => a+=1 => a={:?}", a);
        assert_eq!(a.pn[0], 1);

        // 3) Max + 1 => wrap to 0 (2^32)
        let mut b = U32::default();
        b.pn[0] = 0xFFFF_FFFF;
        debug!("b before +1 => 0x{:08X}", b.pn[0]);
        b += 1u64;
        debug!("b after  +1 => 0x{:08X}", b.pn[0]);
        assert_eq!(b.pn[0], 0, "Expect wrap-around for 32-bit max + 1 => 0");

        // 4) e.g. 0x1234_0000 + 0x0000_5678 => 0x1234_5678
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

        type U256 = BaseUInt256;

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
        a += &b;
        debug!("After add => a.low64() = {}", a.low64());
        assert_eq!(a.low64(), 12345 + 6789);

        // 3) carry from limb 0 to limb 1
        let mut c = U256::default();
        c.pn[0] = 0xFFFF_FFFF;
        c.pn[1] = 0x0000_1234;
        c += 1u64;
        debug!("pn[0]=0x{:08X}, pn[1]=0x{:08X}", c.pn[0], c.pn[1]);
        assert_eq!(c.pn[0], 0);
        assert_eq!(c.pn[1], 0x0000_1235);

        // 4) full 256-bit wrap-around
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

    #[traced_test]
    fn test_add_assign_random_64_and_256() {
        info!("Testing random AddAssign with 64-bit and 256-bit BaseUInt, checking truncated sums.");

        let mut rng = SimpleLCG::new(0xAABB_CCdd_eeff_1122);

        fn full_add_64(a: u64, b: u64) -> u64 {
            let sum_128 = (a as u128).wrapping_add(b as u128);
            (sum_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64
        }

        fn full_add_256(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
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

            let mut a_bu = BaseUInt64::from(a_val);
            a_bu += b_val;
            let sum_low64 = a_bu.low64();
            assert_eq!(sum_low64, correct_64, "Failed random 64-bit add with mod 2^64 carry.");
        }

        // 256 bits
        for _ in 0..50 {
            let a0 = rng.next_u64();
            let a1 = rng.next_u64();
            let a2 = rng.next_u64();
            let a3 = rng.next_u64();
            let b0 = rng.next_u64();
            let b1 = rng.next_u64();
            let b2 = rng.next_u64();
            let b3 = rng.next_u64();

            let a_ref = [a0, a1, a2, a3];
            let b_ref = [b0, b1, b2, b3];
            let correct_256 = full_add_256(a_ref, b_ref);

            let mut blob_a = BaseUInt256::default();
            blob_a.pn[0] = (a0 & 0xFFFF_FFFF) as u32;
            blob_a.pn[1] = ((a0 >> 32) & 0xFFFF_FFFF) as u32;
            blob_a.pn[2] = (a1 & 0xFFFF_FFFF) as u32;
            blob_a.pn[3] = ((a1 >> 32) & 0xFFFF_FFFF) as u32;
            blob_a.pn[4] = (a2 & 0xFFFF_FFFF) as u32;
            blob_a.pn[5] = ((a2 >> 32) & 0xFFFF_FFFF) as u32;
            blob_a.pn[6] = (a3 & 0xFFFF_FFFF) as u32;
            blob_a.pn[7] = ((a3 >> 32) & 0xFFFF_FFFF) as u32;

            let mut blob_b = BaseUInt256::default();
            blob_b.pn[0] = (b0 & 0xFFFF_FFFF) as u32;
            blob_b.pn[1] = ((b0 >> 32) & 0xFFFF_FFFF) as u32;
            blob_b.pn[2] = (b1 & 0xFFFF_FFFF) as u32;
            blob_b.pn[3] = ((b1 >> 32) & 0xFFFF_FFFF) as u32;
            blob_b.pn[4] = (b2 & 0xFFFF_FFFF) as u32;
            blob_b.pn[5] = ((b2 >> 32) & 0xFFFF_FFFF) as u32;
            blob_b.pn[6] = (b3 & 0xFFFF_FFFF) as u32;
            blob_b.pn[7] = ((b3 >> 32) & 0xFFFF_FFFF) as u32;

            blob_a += &blob_b;

            let r0 = ((blob_a.pn[1] as u64) << 32) | (blob_a.pn[0] as u64);
            let r1 = ((blob_a.pn[3] as u64) << 32) | (blob_a.pn[2] as u64);
            let r2 = ((blob_a.pn[5] as u64) << 32) | (blob_a.pn[4] as u64);
            let r3 = ((blob_a.pn[7] as u64) << 32) | (blob_a.pn[6] as u64);

            assert_eq!(r0, correct_256[0]);
            assert_eq!(r1, correct_256[1]);
            assert_eq!(r2, correct_256[2]);
            assert_eq!(r3, correct_256[3]);
        }

        info!("Random add_assign tests for 64-bit & 256-bit completed successfully.");
    }

    #[traced_test]
    fn test_add_assign_64_bits_edge_cases() {
        info!("Testing `AddAssign<&BaseUInt>` and `AddAssign<u64>` for 64-bit BaseUInt edge cases.");

        type U64B = BaseUInt64;

        // 1) 0 + 0 => 0
        let mut x = U64B::default();
        let y = U64B::default();
        debug!("x=0, y=0 => x+=y => x=0");
        x += &y;
        assert_eq!(x.pn, [0, 0]);

        // 2) small + small => no carry
        let mut a = U64B::default();
        a += 5u64;
        let mut b = U64B::default();
        b += 10u64;
        a += &b;
        debug!("a=5, b=10 => a+=b => a=15 => 0x{:08X} {:08X}", a.pn[1], a.pn[0]);
        assert_eq!(a.pn, [15, 0]);

        // 3) 0xFFFF_FFFE + 3 => 4,294,967,294 +3=4,294,967,297 => 0x0001_0000_0001
        let mut c = U64B::default();
        c.pn[0] = 0xFFFF_FFFE;
        c += 3u64;
        debug!(
            "0xFFFF_FFFE + 3 => pn[0]=0x{:08X}, pn[1]=0x{:08X}",
            c.pn[0], c.pn[1]
        );
        assert_eq!(c.pn[0], 0x0000_0001);
        assert_eq!(c.pn[1], 0x0000_0001);

        // 4) crossing into second limb
        let mut d = U64B::default();
        d.pn[0] = 0xFFFF_FFFF;
        d += 1u64;
        debug!(
            "(0xFFFF_FFFF +1) => pn[0]=0x{:08X}, pn[1]=0x{:08X}",
            d.pn[0], d.pn[1]
        );
        assert_eq!(d.pn[0], 0);
        assert_eq!(d.pn[1], 1);

        // 5) full 64-bit wrap-around
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
