// ---------------- [ File: bitcoin-u256/src/convert.rs ]
crate::ix!();

/// Convert from `ArithU256` to the opaque `u256` by writing each 32-bit limb
/// in **little-endian** order into the 32 bytes of `u256`.
///
/// This matches the C++ code: 
/// ```cpp
/// for x in 0..a.WIDTH:
///   writele32(b.as_ptr().offset(x * 4), a.pn[x]);
/// ```
pub fn arith_to_uint256(a: &ArithU256) -> u256 {
    trace!("arith_to_uint256 => converting ArithU256 into u256 by LE limb writes.");

    let mut out = u256::default();
    let limb_count = 256 / 32; // => 8
    for i in 0..limb_count {
        let limb = a.base.get_limb(i); 
        let le = limb.to_le_bytes(); 
        // Copy these 4 bytes into out's slice at offset i*4
        out.as_slice_mut()[(i * 4)..(i * 4 + 4)].copy_from_slice(&le);
    }
    out
}

/// Convert from `u256` to `ArithU256` by **reading** each set of 4 bytes
/// in little-endian order. 
///
/// This matches the C++:
/// ```cpp
/// for x in 0..b.WIDTH:
///   b.base.pn[x] = readle32(a.as_ptr().offset(x * 4));
/// ```
pub fn uint_to_arith256(a: &u256) -> ArithU256 {
    trace!("uint_to_arith256 => converting u256 into ArithU256 by LE limb reads.");

    let mut b = ArithU256::default();
    let limb_count = 256 / 32; // => 8
    for i in 0..limb_count {
        let le_slice = &a.as_slice()[(i * 4)..(i * 4 + 4)];
        let val = u32::from_le_bytes(le_slice.try_into().unwrap());
        b.base.set_limb(i, val);
    }
    b
}

#[cfg(test)]
mod arith_uint256_convert_exhaustive_tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    /// A tiny pseudo-random generator for stable reproducible tests.
    struct TinyRng(u64);
    impl TinyRng {
        fn new(seed: u64) -> Self { Self(seed) }
        fn next_u64(&mut self) -> u64 {
            // standard LCG step
            self.0 = self.0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1);
            self.0
        }
    }

    /// 1) Test `arith_to_uint256` with small, large, random limb patterns
    #[traced_test]
    fn test_arith_to_uint256() {
        info!("Testing `arith_to_uint256` with a variety of limb patterns...");

        // (a) All zero => must yield a u256 with all zero bytes
        let a_zero = ArithU256::default();
        let z_u256 = arith_to_uint256(&a_zero);
        assert!(z_u256.is_null(), "0 => must yield all zero bytes in u256");
        trace!("(a) zero => ok");

        // (b) One limb set, rest zero
        let mut a_one = ArithU256::default();
        a_one.base.set_limb(0, 0xDEAD_BEEF);
        // => in LE => the first 4 bytes in the resulting u256
        let out = arith_to_uint256(&a_one);
        let out_slice = out.as_slice();
        // check the first 4 => DE AD BE EF in LE => EFBEADDE if we read them as a single 32-bit
        assert_eq!(&out_slice[0..4], &0xDEAD_BEEF_u32.to_le_bytes());
        // the rest must be zero
        for b in &out_slice[4..] {
            assert_eq!(*b, 0);
        }
        trace!("(b) single-limb => ok");

        // (c) All limbs distinct => e.g. limb i = i
        let mut a_struct = ArithU256::default();
        for i in 0..8 {
            a_struct.base.set_limb(i, i as u32);
        }
        let out_struct = arith_to_uint256(&a_struct);
        let sl_struct = out_struct.as_slice();
        for i in 0..8 {
            let chunk = &sl_struct[i*4..(i+1)*4];
            let val = u32::from_le_bytes(chunk.try_into().unwrap());
            assert_eq!(val, i as u32, "limb[{}] mismatch in the output bytes", i);
        }
        trace!("(c) distinct-limbs => ok");

        // (d) Random patterns
        let mut rng = TinyRng::new(0xABC12345);
        for _ in 0..5 {
            let mut a_rand = ArithU256::default();
            for i in 0..8 {
                let r = rng.next_u64() as u32;
                a_rand.base.set_limb(i, r);
            }
            let out_rand = arith_to_uint256(&a_rand);
            // Then read them back => check
            let sl = out_rand.as_slice();
            for i in 0..8 {
                let chunk = &sl[i*4..(i+1)*4];
                let val = u32::from_le_bytes(chunk.try_into().unwrap());
                let expected = a_rand.base.get_limb(i);
                assert_eq!(val, expected, "random-limb[{}] mismatch => arith_to_uint256", i);
            }
        }
        trace!("(d) random-limbs => ok");

        info!("test_arith_to_uint256 => all sub-checks passed.");
    }

    /// 2) Test `uint_to_arith256` with carefully built `u256` patterns
    #[traced_test]
    fn test_uint_to_arith256() {
        info!("Testing `uint_to_arith256` => reading 32 bytes in LE groups of 4 => set limbs.");

        // (a) All zero => => ArithU256::default()
        let z = u256::default();
        let out_z = uint_to_arith256(&z);
        assert_eq!(out_z, ArithU256::default(), "all zero => default ArithU256");
        trace!("(a) zero => ok");

        // (b) Single set => e.g. the first 4 bytes => 0x11223344 in LE
        let mut x1 = u256::default();
        x1.as_slice_mut()[0..4].copy_from_slice(&0x1122_3344u32.to_le_bytes());
        let out1 = uint_to_arith256(&x1);
        let limb0 = out1.base.get_limb(0);
        assert_eq!(limb0, 0x1122_3344, "lowest limb must match the first 4 bytes in LE");
        for i in 1..8 {
            assert_eq!(out1.base.get_limb(i), 0, "other limbs => 0");
        }
        trace!("(b) single-limb => ok");

        // (c) Distinct => i => in the i-th 4 bytes => i
        let mut x_struct = u256::default();
        for i in 0..8 {
            x_struct.as_slice_mut()[i*4..(i+1)*4]
                .copy_from_slice(&(i as u32).to_le_bytes());
        }
        let out_struct = uint_to_arith256(&x_struct);
        for i in 0..8 {
            let limb = out_struct.base.get_limb(i);
            assert_eq!(limb, i as u32, "limb[{}] => mismatch in distinct-limb test", i);
        }
        trace!("(c) distinct-limb => ok");

        // (d) random patterns
        let mut rng = TinyRng::new(0x5566_7788_AABB_CCDDu64);
        for _ in 0..5 {
            let mut x_rand = u256::default();
            // fill 32 bytes randomly
            for b in x_rand.as_slice_mut() {
                *b = (rng.next_u64() & 0xFF) as u8;
            }
            // parse => check the limbs
            let out_rand = uint_to_arith256(&x_rand);
            // each 4 bytes => a limb
            for i in 0..8 {
                let chunk = &x_rand.as_slice()[i*4..(i+1)*4];
                let val_le = u32::from_le_bytes(chunk.try_into().unwrap());
                let limb = out_rand.base.get_limb(i);
                assert_eq!(val_le, limb, "random => mismatch limb[{}]", i);
            }
        }
        trace!("(d) random => ok");

        info!("test_uint_to_arith256 => sub-checks passed successfully.");
    }

    /// 3) Round-trip: `ArithU256 -> u256 -> ArithU256`
    #[traced_test]
    fn test_arith_uint256_convert_roundtrip() {
        info!("Testing round-trip: ArithU256 -> u256 -> ArithU256 => must yield the original.");

        let mut rng = TinyRng::new(0xDEAD_BEEFu64);

        for _ in 0..10 {
            let mut a_orig = ArithU256::default();
            // set random limbs
            for i in 0..8 {
                let val32 = (rng.next_u64() & 0xFFFF_FFFF) as u32;
                a_orig.base.set_limb(i, val32);
            }

            // Convert => u256 => convert back
            let x = arith_to_uint256(&a_orig);
            let round = uint_to_arith256(&x);

            assert_eq!(a_orig, round, "Round-trip mismatch for random ArithU256");
        }

        // Also test zero & single-limb
        let zero = ArithU256::default();
        let xz = arith_to_uint256(&zero);
        let backz = uint_to_arith256(&xz);
        assert_eq!(zero, backz, "Round-trip zero => mismatch");

        let mut single = ArithU256::default();
        single.base.set_limb(1, 0xFFFF_0000);
        let xs = arith_to_uint256(&single);
        let backs = uint_to_arith256(&xs);
        assert_eq!(single, backs, "Round-trip single-limb => mismatch");

        info!("test_arith_uint256_convert_roundtrip => all done successfully.");
    }
}
