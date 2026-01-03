// ---------------- [ File: bitcoinsecp256k1-modinv32/src/test_support.rs ]
crate::ix!();

pub mod support {
    use super::*;

    pub(crate) const M30_I32: i32 = (u32::MAX >> 2) as i32;
    pub(crate) const M30_U64: u64 = (1u64 << 30) - 1;
    pub(crate) const TWO_POW_30_U64: u64 = 1u64 << 30;

    pub(crate) fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    pub(crate) fn modinv_u64(a: u64, modulus: u64) -> u64 {
        assert!(modulus > 0);
        let mut t: i128 = 0;
        let mut new_t: i128 = 1;
        let mut r: i128 = modulus as i128;
        let mut new_r: i128 = (a % modulus) as i128;

        while new_r != 0 {
            let q: i128 = r / new_r;

            let tmp_t: i128 = t - q * new_t;
            t = new_t;
            new_t = tmp_t;

            let tmp_r: i128 = r - q * new_r;
            r = new_r;
            new_r = tmp_r;
        }

        assert!(r == 1);
        if t < 0 {
            t += modulus as i128;
        }
        (t as u64) % modulus
    }

    pub(crate) fn normalize_mod_u64(x: i128, modulus: u64) -> u64 {
        let m = modulus as i128;
        let mut r = x % m;
        if r < 0 {
            r += m;
        }
        r as u64
    }

    pub(crate) fn signed30_from_u64(mut x: u64) -> ModInv32Signed30 {
        let mut out = ModInv32Signed30 { v: [0i32; 9] };
        let mut i: usize = 0;
        while i < 8 {
            out.v[i] = (x & M30_U64) as i32;
            x >>= 30;
            i += 1;
        }
        out.v[8] = x as i32;
        out
    }

    pub(crate) fn signed30_from_i32_low(x: i32) -> ModInv32Signed30 {
        ModInv32Signed30 {
            v: [x, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    pub(crate) fn signed30_from_i128_sign_extended(mut x: i128) -> ModInv32Signed30 {
        let mut out = ModInv32Signed30 { v: [0i32; 9] };
        let mask: i128 = (1i128 << 30) - 1;
        let mut i: usize = 0;

        while i < 8 {
            out.v[i] = (x & mask) as i32;
            x >>= 30;
            i += 1;
        }

        let top: i32 = x as i32;
        assert!((top as i128) == x);
        out.v[8] = top;

        out
    }

    pub(crate) fn signed30_to_i128_horner(x: &ModInv32Signed30) -> i128 {
        let mut acc: i128 = x.v[8] as i128;
        let mut idx: i32 = 7;
        while idx >= 0 {
            acc = (acc << 30) + (x.v[idx as usize] as i128);
            idx -= 1;
        }
        acc
    }

    pub(crate) fn signed30_to_u128_horner(x: &ModInv32Signed30) -> u128 {
        let mut acc: u128 = 0;
        let mut idx: i32 = 8;
        while idx >= 0 {
            let limb = x.v[idx as usize];
            assert!(limb >= 0);
            assert!((limb >> 30) == 0);
            assert!(acc <= (u128::MAX >> 30));
            acc = (acc << 30) + (limb as u128);
            idx -= 1;
        }
        acc
    }

    pub(crate) fn assert_signed30_limbs_are_normalized(x: &ModInv32Signed30) {
        let mut i: usize = 0;
        while i < 9 {
            let limb = x.v[i];
            assert!(limb >= 0);
            assert!((limb >> 30) == 0);
            i += 1;
        }
    }

    pub(crate) fn assert_signed30_limbs_within_signed_bound(x: &ModInv32Signed30) {
        let mut i: usize = 0;
        while i < 9 {
            let limb = x.v[i];
            assert!(limb >= -M30_I32);
            assert!(limb <= M30_I32);
            i += 1;
        }
    }

    pub(crate) fn modinfo_from_u64(modulus: u64) -> ModInv32ModInfo {
        assert!(modulus >= 3);
        assert!((modulus & 1) == 1);

        let modulus_s = signed30_from_u64(modulus);
        let low = (modulus & M30_U64) as u64;
        let modulus_inv30: u32 = modinv_u64(low, TWO_POW_30_U64) as u32;

        ModInv32ModInfo {
            modulus: modulus_s,
            modulus_inv30,
        }
    }

    pub(crate) fn xorshift64_star(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        *state = x;
        x.wrapping_mul(0x2545F4914F6CDD1Du64)
    }

    pub(crate) fn noncanonical_negative_one() -> ModInv32Signed30 {
        let mut v = [0i32; 9];
        let mut i: usize = 0;
        while i < 8 {
            v[i] = M30_I32;
            i += 1;
        }
        v[8] = -1;
        ModInv32Signed30 { v }
    }
}
