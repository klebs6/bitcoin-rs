// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/lib.rs ]
#![allow(unused_parens)]
#[macro_use] mod imports; use imports::*;

x!{constants}
x!{fe_add}
x!{fe_clear}
x!{fe_cmov}
x!{fe_cmp_var}
x!{fe_const}
x!{fe_from_signed62}
x!{fe_from_storage}
x!{fe_get_b32}
x!{fe_inv}
x!{fe_inv_var}
x!{fe_is_odd}
x!{fe_is_zero}
x!{fe_mul}
x!{fe_mul_int}
x!{fe_negate}
x!{fe_normalize}
x!{fe_normalize_var}
x!{fe_normalize_weak}
x!{fe_normalizes_to_zero}
x!{fe_normalizes_to_zero_var}
x!{fe_set_b32}
x!{fe_set_int}
x!{fe_sqr}
x!{fe_storage}
x!{fe_storage_cmov}
x!{fe_to_signed62}
x!{fe_to_storage}
x!{fe_verify}
x!{field_5x52}
x!{secp_256k1_fe_mul_inner}
x!{secp_256k1_fe_sqr_inner}
x!{verify_bits}

#[cfg(test)]
mod fe_set_b32_boundary_sweep_additional_tests {
    use super::*;

    const FIELD_P_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
    ];

    fn u64_to_be32(v: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out
    }

    fn be32_add(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let mut out = [0u8; 32];
        let mut carry: u16 = 0;
        let mut i: i32 = 31;
        while i >= 0 {
            let sum = (a[i as usize] as u16) + (b[i as usize] as u16) + carry;
            out[i as usize] = (sum & 0xFF) as u8;
            carry = sum >> 8;
            i -= 1;
        }
        assert_eq!(carry, 0);
        out
    }

    fn be32_sub(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let mut out = [0u8; 32];
        let mut borrow: i32 = 0;
        let mut i: i32 = 31;
        while i >= 0 {
            let ai = a[i as usize] as i32;
            let bi = b[i as usize] as i32;
            let mut v = ai - bi - borrow;
            if v < 0 {
                v += 256;
                borrow = 1;
            } else {
                borrow = 0;
            }
            out[i as usize] = v as u8;
            i -= 1;
        }
        assert_eq!(borrow, 0);
        out
    }

    unsafe fn fe_roundtrip_norm(bytes: &[u8; 32]) -> [u8; 32] {
        let mut fe = Fe5x52::new();
        let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, bytes.as_ptr());
        assert_eq!(ret, 1);

        crate::fe_normalize(&mut fe as *mut Fe5x52);

        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), &fe as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_set_b32_boundary_sweep_below_and_above_modulus_behaves_correctly() {
        tracing::info!("sweeping small deltas around field modulus for fe_set_b32 acceptance/rejection behavior");

        unsafe {
            let deltas: [u64; 16] = [
                1, 2, 3, 4, 5, 7, 8, 15, 16, 31, 32, 33, 63, 64, 255, 1024,
            ];

            for (idx, &k) in deltas.iter().enumerate() {
                tracing::debug!(case_index = idx, delta_u64 = k, "testing p-k accepted");
                let below = be32_sub(&FIELD_P_B32, &u64_to_be32(k));

                let mut fe = Fe5x52::new();
                let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, below.as_ptr());
                assert_eq!(ret, 1);

                let got = fe_roundtrip_norm(&below);
                assert_eq!(got, below);

                tracing::debug!(case_index = idx, delta_u64 = k, "testing p+k rejected (for small k)");
                let above = be32_add(&FIELD_P_B32, &u64_to_be32(k));

                let mut fe2 = Fe5x52::new();
                let ret2 = crate::fe_set_b32(&mut fe2 as *mut Fe5x52, above.as_ptr());
                assert_eq!(ret2, 0);
            }

            tracing::debug!("sanity: p rejected");
            let mut fe_p = Fe5x52::new();
            let ret_p = crate::fe_set_b32(&mut fe_p as *mut Fe5x52, FIELD_P_B32.as_ptr());
            assert_eq!(ret_p, 0);

            tracing::debug!("sanity: 2^256-1 rejected");
            let all_ones = [0xFFu8; 32];
            let mut fe_hi = Fe5x52::new();
            let ret_hi = crate::fe_set_b32(&mut fe_hi as *mut Fe5x52, all_ones.as_ptr());
            assert_eq!(ret_hi, 0);
        }
    }
}

#[cfg(test)]
mod field_element_roundtrip_deterministic_fuzz_tests {
    use super::*;

    const FIELD_P_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
    ];

    fn xorshift64star(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        *state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D_u64)
    }

    fn be32_lt(a: &[u8; 32], b: &[u8; 32]) -> bool {
        let mut i: usize = 0;
        while i < 32 {
            if a[i] < b[i] {
                return true;
            }
            if a[i] > b[i] {
                return false;
            }
            i += 1;
        }
        false
    }

    fn be32_cmp_i32(a: &[u8; 32], b: &[u8; 32]) -> i32 {
        let mut i: usize = 0;
        while i < 32 {
            if a[i] > b[i] {
                return 1;
            }
            if a[i] < b[i] {
                return -1;
            }
            i += 1;
        }
        0
    }

    fn random_b32_under_p(state: &mut u64) -> [u8; 32] {
        loop {
            let w0 = xorshift64star(state).to_be_bytes();
            let w1 = xorshift64star(state).to_be_bytes();
            let w2 = xorshift64star(state).to_be_bytes();
            let w3 = xorshift64star(state).to_be_bytes();

            let mut out = [0u8; 32];
            out[0..8].copy_from_slice(&w0);
            out[8..16].copy_from_slice(&w1);
            out[16..24].copy_from_slice(&w2);
            out[24..32].copy_from_slice(&w3);

            if be32_lt(&out, &FIELD_P_B32) {
                return out;
            }
        }
    }

    fn is_zero_b32(x: &[u8; 32]) -> bool {
        let mut acc: u8 = 0;
        let mut i: usize = 0;
        while i < 32 {
            acc |= x[i];
            i += 1;
        }
        acc == 0
    }

    unsafe fn fe_from_b32_checked(bytes: &[u8; 32]) -> Fe5x52 {
        let mut fe = Fe5x52::new();
        let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, bytes.as_ptr());
        assert_eq!(ret, 1);
        fe
    }

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    unsafe fn storage_from_fe_normalized(fe: &Fe5x52) -> Fe5x52Storage {
        let mut stor = core::mem::MaybeUninit::<Fe5x52Storage>::uninit();
        crate::fe_to_storage(stor.as_mut_ptr(), fe as *const Fe5x52);
        stor.assume_init()
    }

    unsafe fn b32_from_storage(stor: &Fe5x52Storage) -> [u8; 32] {
        let mut fe = Fe5x52::new();
        crate::fe_from_storage(&mut fe as *mut Fe5x52, stor as *const Fe5x52Storage);
        fe_to_b32_normalized(&mut fe)
    }

    unsafe fn signed62_from_fe_normalized(fe: &Fe5x52) -> ModInv64Signed62 {
        let mut s = core::mem::MaybeUninit::<ModInv64Signed62>::uninit();
        crate::fe_to_signed62(s.as_mut_ptr(), fe as *const Fe5x52);
        s.assume_init()
    }

    unsafe fn b32_from_signed62(s: &ModInv64Signed62) -> [u8; 32] {
        let mut fe = Fe5x52::new();
        crate::fe_from_signed62(&mut fe as *mut Fe5x52, s as *const ModInv64Signed62);
        fe_to_b32_normalized(&mut fe)
    }

    #[traced_test]
    fn randomized_b32_roundtrips_through_set_and_get_for_many_samples() {
        tracing::info!("deterministic fuzz: fe_set_b32 + fe_get_b32 roundtrip for many random values < p");

        unsafe {
            let mut seed: u64 = 0xC0FFEE_1234_5678_u64;
            let iters: usize = 512;

            let mut i: usize = 0;
            while i < iters {
                let b = random_b32_under_p(&mut seed);
                if (i & 63) == 0 {
                    tracing::debug!(iter = i, seed = seed, "roundtrip progress checkpoint");
                } else {
                    tracing::trace!(iter = i, "roundtrip one sample");
                }

                let mut fe = fe_from_b32_checked(&b);
                let got = fe_to_b32_normalized(&mut fe);

                assert_eq!(got, b);
                assert!(be32_lt(&got, &FIELD_P_B32) || is_zero_b32(&got));
                i += 1;
            }
        }
    }

    #[traced_test]
    fn randomized_roundtrips_through_storage_conversion_for_many_samples() {
        tracing::info!("deterministic fuzz: fe_to_storage + fe_from_storage roundtrip for many random values < p");

        unsafe {
            let mut seed: u64 = 0xBADC0DE_DEAD_BEEFu64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let b = random_b32_under_p(&mut seed);
                if (i & 63) == 0 {
                    tracing::debug!(iter = i, seed = seed, "storage roundtrip progress checkpoint");
                } else {
                    tracing::trace!(iter = i, "storage roundtrip one sample");
                }

                let mut fe = fe_from_b32_checked(&b);
                crate::fe_normalize(&mut fe as *mut Fe5x52);

                let stor = storage_from_fe_normalized(&fe);
                let got = b32_from_storage(&stor);

                assert_eq!(got, b);
                i += 1;
            }
        }
    }

    #[traced_test]
    fn randomized_roundtrips_through_signed62_conversion_for_many_samples() {
        tracing::info!("deterministic fuzz: fe_to_signed62 + fe_from_signed62 roundtrip for many random values < p");

        unsafe {
            let mut seed: u64 = 0x1234_5678_9ABC_DEF0u64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let b = random_b32_under_p(&mut seed);
                if (i & 63) == 0 {
                    tracing::debug!(iter = i, seed = seed, "signed62 roundtrip progress checkpoint");
                } else {
                    tracing::trace!(iter = i, "signed62 roundtrip one sample");
                }

                let mut fe = fe_from_b32_checked(&b);
                crate::fe_normalize(&mut fe as *mut Fe5x52);

                let s62 = signed62_from_fe_normalized(&fe);
                let got = b32_from_signed62(&s62);

                assert_eq!(got, b);
                i += 1;
            }
        }
    }

    #[traced_test]
    fn cmp_var_matches_big_endian_numeric_ordering_for_random_normalized_values() {
        tracing::info!("deterministic fuzz: fe_cmp_var matches big-endian lexicographic numeric ordering");

        unsafe {
            let mut seed: u64 = 0xD15EA5E_0DDC0FFEu64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let a_b32 = random_b32_under_p(&mut seed);
                let b_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "cmp_var progress checkpoint");
                } else {
                    tracing::trace!(iter = i, "cmp_var one sample");
                }

                let mut a = fe_from_b32_checked(&a_b32);
                let mut b = fe_from_b32_checked(&b_b32);

                crate::fe_normalize(&mut a as *mut Fe5x52);
                crate::fe_normalize(&mut b as *mut Fe5x52);

                let got = crate::fe_cmp_var(&a as *const Fe5x52, &b as *const Fe5x52);
                let expected = be32_cmp_i32(&a_b32, &b_b32);

                assert_eq!(got, expected);
                i += 1;
            }
        }
    }

    #[traced_test]
    fn is_odd_and_is_zero_match_canonical_byte_properties_for_random_values() {
        tracing::info!("deterministic fuzz: fe_is_odd/fe_is_zero agree with normalized byte properties");

        unsafe {
            let mut seed: u64 = 0xFEEDFACE_CAFEBABEu64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let b = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "odd/zero progress checkpoint");
                } else {
                    tracing::trace!(iter = i, "odd/zero one sample");
                }

                let mut fe = fe_from_b32_checked(&b);
                crate::fe_normalize(&mut fe as *mut Fe5x52);

                let odd = crate::fe_is_odd(&fe as *const Fe5x52);
                let zero = crate::fe_is_zero(&fe as *const Fe5x52);

                assert_eq!(odd, (b[31] & 1) as i32);
                assert_eq!(zero, if is_zero_b32(&b) { 1 } else { 0 });

                i += 1;
            }
        }
    }

    #[traced_test]
    fn cmov_selects_expected_value_for_random_cases() {
        tracing::info!("deterministic fuzz: fe_cmov selects expected value for flag 0/1");

        unsafe {
            let mut seed: u64 = 0x0F0E_0D0C_0B0A_0908u64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let r0_b32 = random_b32_under_p(&mut seed);
                let a_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "cmov progress checkpoint");
                } else {
                    tracing::trace!(iter = i, "cmov one sample");
                }

                let mut r0 = fe_from_b32_checked(&r0_b32);
                let a = fe_from_b32_checked(&a_b32);

                crate::fe_cmov(&mut r0 as *mut Fe5x52, &a as *const Fe5x52, 0);
                let got0 = fe_to_b32_normalized(&mut r0);
                assert_eq!(got0, r0_b32);

                crate::fe_cmov(&mut r0 as *mut Fe5x52, &a as *const Fe5x52, 1);
                let got1 = fe_to_b32_normalized(&mut r0);
                assert_eq!(got1, a_b32);

                i += 1;
            }
        }
    }

    #[traced_test]
    fn storage_cmov_selects_expected_value_for_random_cases() {
        tracing::info!("deterministic fuzz: fe_storage_cmov selects expected value for flag 0/1");

        unsafe {
            let mut seed: u64 = 0xA5A5_A5A5_5A5A_5A5Au64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let r0_b32 = random_b32_under_p(&mut seed);
                let a_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "storage_cmov progress checkpoint");
                } else {
                    tracing::trace!(iter = i, "storage_cmov one sample");
                }

                let mut r0_fe = fe_from_b32_checked(&r0_b32);
                let mut a_fe = fe_from_b32_checked(&a_b32);

                crate::fe_normalize(&mut r0_fe as *mut Fe5x52);
                crate::fe_normalize(&mut a_fe as *mut Fe5x52);

                let mut r0 = storage_from_fe_normalized(&r0_fe);
                let a = storage_from_fe_normalized(&a_fe);

                crate::fe_storage_cmov(&mut r0 as *mut Fe5x52Storage, &a as *const Fe5x52Storage, 0);
                let got0 = b32_from_storage(&r0);
                assert_eq!(got0, r0_b32);

                crate::fe_storage_cmov(&mut r0 as *mut Fe5x52Storage, &a as *const Fe5x52Storage, 1);
                let got1 = b32_from_storage(&r0);
                assert_eq!(got1, a_b32);

                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod field_element_algebraic_identity_fuzz_tests {
    use super::*;

    const FIELD_P_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
    ];

    fn xorshift64star(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        *state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D_u64)
    }

    fn be32_lt(a: &[u8; 32], b: &[u8; 32]) -> bool {
        let mut i: usize = 0;
        while i < 32 {
            if a[i] < b[i] {
                return true;
            }
            if a[i] > b[i] {
                return false;
            }
            i += 1;
        }
        false
    }

    fn is_zero_b32(x: &[u8; 32]) -> bool {
        let mut acc: u8 = 0;
        let mut i: usize = 0;
        while i < 32 {
            acc |= x[i];
            i += 1;
        }
        acc == 0
    }

    fn random_b32_under_p(state: &mut u64) -> [u8; 32] {
        loop {
            let w0 = xorshift64star(state).to_be_bytes();
            let w1 = xorshift64star(state).to_be_bytes();
            let w2 = xorshift64star(state).to_be_bytes();
            let w3 = xorshift64star(state).to_be_bytes();

            let mut out = [0u8; 32];
            out[0..8].copy_from_slice(&w0);
            out[8..16].copy_from_slice(&w1);
            out[16..24].copy_from_slice(&w2);
            out[24..32].copy_from_slice(&w3);

            if be32_lt(&out, &FIELD_P_B32) {
                return out;
            }
        }
    }

    fn random_nonzero_b32_under_p(state: &mut u64) -> [u8; 32] {
        loop {
            let out = random_b32_under_p(state);
            if !is_zero_b32(&out) {
                return out;
            }
        }
    }

    fn u64_to_be32(v: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out
    }

    unsafe fn fe_from_b32_checked(bytes: &[u8; 32]) -> Fe5x52 {
        let mut fe = Fe5x52::new();
        let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, bytes.as_ptr());
        assert_eq!(ret, 1);
        fe
    }

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    unsafe fn fe_add_then_norm(a_b32: &[u8; 32], b_b32: &[u8; 32]) -> [u8; 32] {
        let mut a = fe_from_b32_checked(a_b32);
        let b = fe_from_b32_checked(b_b32);
        crate::fe_add(&mut a as *mut Fe5x52, &b as *const Fe5x52);
        fe_to_b32_normalized(&mut a)
    }

    unsafe fn fe_mul_then_norm(a_b32: &[u8; 32], b_b32: &[u8; 32]) -> [u8; 32] {
        let a = fe_from_b32_checked(a_b32);
        let b = fe_from_b32_checked(b_b32);
        let mut r = Fe5x52::new();
        crate::fe_mul(&mut r as *mut Fe5x52, &a as *const Fe5x52, &b as *const Fe5x52);
        fe_to_b32_normalized(&mut r)
    }

    unsafe fn fe_sqr_then_norm(a_b32: &[u8; 32]) -> [u8; 32] {
        let a = fe_from_b32_checked(a_b32);
        let mut r = Fe5x52::new();
        crate::fe_sqr(&mut r as *mut Fe5x52, &a as *const Fe5x52);
        fe_to_b32_normalized(&mut r)
    }

    #[traced_test]
    fn addition_commutativity_and_associativity_hold_after_normalization_for_random_values() {
        tracing::info!("deterministic fuzz: (a+b)=(b+a) and (a+b)+c=a+(b+c) after normalization");

        unsafe {
            let mut seed: u64 = 0x1111_2222_3333_4444u64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let a = random_b32_under_p(&mut seed);
                let b = random_b32_under_p(&mut seed);
                let c = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "addition law checkpoint");
                } else {
                    tracing::trace!(iter = i, "addition law sample");
                }

                let ab = fe_add_then_norm(&a, &b);
                let ba = fe_add_then_norm(&b, &a);
                assert_eq!(ab, ba);

                let abc_left = {
                    let mut t = fe_from_b32_checked(&a);
                    let b_fe = fe_from_b32_checked(&b);
                    let c_fe = fe_from_b32_checked(&c);
                    crate::fe_add(&mut t as *mut Fe5x52, &b_fe as *const Fe5x52);
                    crate::fe_add(&mut t as *mut Fe5x52, &c_fe as *const Fe5x52);
                    fe_to_b32_normalized(&mut t)
                };

                let abc_right = {
                    let mut t = fe_from_b32_checked(&b);
                    let c_fe = fe_from_b32_checked(&c);
                    crate::fe_add(&mut t as *mut Fe5x52, &c_fe as *const Fe5x52);

                    let mut u = fe_from_b32_checked(&a);
                    crate::fe_add(&mut u as *mut Fe5x52, &t as *const Fe5x52);
                    fe_to_b32_normalized(&mut u)
                };

                assert_eq!(abc_left, abc_right);

                assert!(be32_lt(&abc_left, &FIELD_P_B32) || is_zero_b32(&abc_left));
                i += 1;
            }
        }
    }

    #[traced_test]
    fn multiplication_commutativity_and_associativity_hold_after_normalization_for_random_values() {
        tracing::info!("deterministic fuzz: (a*b)=(b*a) and (a*b)*c=a*(b*c) after normalization");

        unsafe {
            let mut seed: u64 = 0xAAAA_BBBB_CCCC_DDDDu64;
            let iters: usize = 192;

            let mut i: usize = 0;
            while i < iters {
                let a = random_b32_under_p(&mut seed);
                let b = random_b32_under_p(&mut seed);
                let c = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "multiplication law checkpoint");
                } else {
                    tracing::trace!(iter = i, "multiplication law sample");
                }

                let ab = fe_mul_then_norm(&a, &b);
                let ba = fe_mul_then_norm(&b, &a);
                assert_eq!(ab, ba);

                let assoc_left = {
                    let a_fe = fe_from_b32_checked(&a);
                    let b_fe = fe_from_b32_checked(&b);
                    let c_fe = fe_from_b32_checked(&c);

                    let mut ab_fe = Fe5x52::new();
                    crate::fe_mul(&mut ab_fe as *mut Fe5x52, &a_fe as *const Fe5x52, &b_fe as *const Fe5x52);

                    let mut left = Fe5x52::new();
                    crate::fe_mul(&mut left as *mut Fe5x52, &ab_fe as *const Fe5x52, &c_fe as *const Fe5x52);

                    fe_to_b32_normalized(&mut left)
                };

                let assoc_right = {
                    let a_fe = fe_from_b32_checked(&a);
                    let b_fe = fe_from_b32_checked(&b);
                    let c_fe = fe_from_b32_checked(&c);

                    let mut bc_fe = Fe5x52::new();
                    crate::fe_mul(&mut bc_fe as *mut Fe5x52, &b_fe as *const Fe5x52, &c_fe as *const Fe5x52);

                    let mut right = Fe5x52::new();
                    crate::fe_mul(&mut right as *mut Fe5x52, &a_fe as *const Fe5x52, &bc_fe as *const Fe5x52);

                    fe_to_b32_normalized(&mut right)
                };

                assert_eq!(assoc_left, assoc_right);
                assert!(be32_lt(&assoc_left, &FIELD_P_B32) || is_zero_b32(&assoc_left));
                i += 1;
            }
        }
    }

    #[traced_test]
    fn distributivity_holds_after_normalization_for_random_values() {
        tracing::info!("deterministic fuzz: (a+b)*c == a*c + b*c after normalization");

        unsafe {
            let mut seed: u64 = 0x0BAD_F00D_DEAD_BEEFu64;
            let iters: usize = 192;

            let mut i: usize = 0;
            while i < iters {
                let a = random_b32_under_p(&mut seed);
                let b = random_b32_under_p(&mut seed);
                let c = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "distributivity checkpoint");
                } else {
                    tracing::trace!(iter = i, "distributivity sample");
                }

                let left = {
                    let mut sum = fe_from_b32_checked(&a);
                    let b_fe = fe_from_b32_checked(&b);
                    let c_fe = fe_from_b32_checked(&c);

                    crate::fe_add(&mut sum as *mut Fe5x52, &b_fe as *const Fe5x52);

                    let mut out = Fe5x52::new();
                    crate::fe_mul(&mut out as *mut Fe5x52, &sum as *const Fe5x52, &c_fe as *const Fe5x52);
                    fe_to_b32_normalized(&mut out)
                };

                let right = {
                    let a_fe = fe_from_b32_checked(&a);
                    let b_fe = fe_from_b32_checked(&b);
                    let c_fe = fe_from_b32_checked(&c);

                    let mut ac = Fe5x52::new();
                    crate::fe_mul(&mut ac as *mut Fe5x52, &a_fe as *const Fe5x52, &c_fe as *const Fe5x52);

                    let mut bc = Fe5x52::new();
                    crate::fe_mul(&mut bc as *mut Fe5x52, &b_fe as *const Fe5x52, &c_fe as *const Fe5x52);

                    crate::fe_add(&mut ac as *mut Fe5x52, &bc as *const Fe5x52);
                    fe_to_b32_normalized(&mut ac)
                };

                assert_eq!(left, right);
                assert!(be32_lt(&left, &FIELD_P_B32) || is_zero_b32(&left));
                i += 1;
            }
        }
    }

    #[traced_test]
    fn negation_is_additive_inverse_and_double_negation_roundtrips_for_random_values() {
        tracing::info!("deterministic fuzz: a + (-a) == 0 and -(-a) == a after normalization");

        unsafe {
            let mut seed: u64 = 0x1357_9BDF_2468_ACE0u64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let a_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "negation checkpoint");
                } else {
                    tracing::trace!(iter = i, "negation sample");
                }

                let a = fe_from_b32_checked(&a_b32);

                let mut neg = Fe5x52::new();
                crate::fe_negate(&mut neg as *mut Fe5x52, &a as *const Fe5x52, 1);

                let mut sum = fe_from_b32_checked(&a_b32);
                crate::fe_add(&mut sum as *mut Fe5x52, &neg as *const Fe5x52);

                let zflag_ct = crate::fe_normalizes_to_zero(&sum as *const Fe5x52);
                let zflag_var = crate::fe_normalizes_to_zero_var(&sum as *const Fe5x52);
                assert_eq!(zflag_ct, 1);
                assert_eq!(zflag_var, 1);

                let sum_b32 = fe_to_b32_normalized(&mut sum);
                assert_eq!(sum_b32, [0u8; 32]);

                let mut dbl = Fe5x52::new();
                crate::fe_negate(&mut dbl as *mut Fe5x52, &neg as *const Fe5x52, 2);

                let dbl_b32 = fe_to_b32_normalized(&mut dbl);
                assert_eq!(dbl_b32, a_b32);

                i += 1;
            }
        }
    }

    #[traced_test]
    fn mul_int_matches_mul_by_small_scalar_element_for_random_values_and_scalars() {
        tracing::info!("deterministic fuzz: fe_mul_int matches fe_mul by scalar element for small scalars");

        unsafe {
            let mut seed: u64 = 0xDEAD_BEEF_F00D_BAADu64;
            let iters: usize = 256;
            let scalars: [i32; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

            let mut i: usize = 0;
            while i < iters {
                let a_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "mul_int checkpoint");
                } else {
                    tracing::trace!(iter = i, "mul_int sample");
                }

                for &k in scalars.iter() {
                    let mut r1 = fe_from_b32_checked(&a_b32);
                    crate::fe_mul_int(&mut r1 as *mut Fe5x52, k);
                    let r1_b32 = fe_to_b32_normalized(&mut r1);

                    let a = fe_from_b32_checked(&a_b32);
                    let mut kfe = Fe5x52::new();
                    crate::fe_set_int(&mut kfe as *mut Fe5x52, k);

                    let mut r2 = Fe5x52::new();
                    crate::fe_mul(&mut r2 as *mut Fe5x52, &a as *const Fe5x52, &kfe as *const Fe5x52);
                    let r2_b32 = fe_to_b32_normalized(&mut r2);

                    assert_eq!(r1_b32, r2_b32);
                }

                i += 1;
            }
        }
    }

    #[traced_test]
    fn sqr_matches_mul_self_for_random_values() {
        tracing::info!("deterministic fuzz: fe_sqr matches fe_mul(a,a) for random values");

        unsafe {
            let mut seed: u64 = 0x0102_0304_0506_0708u64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let a_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "sqr checkpoint");
                } else {
                    tracing::trace!(iter = i, "sqr sample");
                }

                let sq = fe_sqr_then_norm(&a_b32);
                let mul = fe_mul_then_norm(&a_b32, &a_b32);
                assert_eq!(sq, mul);

                i += 1;
            }
        }
    }

    #[traced_test]
    fn inversion_roundtrip_and_inverse_product_properties_hold_for_random_nonzero_values() {
        tracing::info!("deterministic fuzz: inv(inv(a))==a and inv(a*b)==inv(a)*inv(b) for random nonzero values");

        unsafe {
            let mut seed: u64 = 0x4242_1337_900D_C0DEu64;
            let iters: usize = 128;

            let mut i: usize = 0;
            while i < iters {
                let a_b32 = random_nonzero_b32_under_p(&mut seed);
                let b_b32 = random_nonzero_b32_under_p(&mut seed);

                if (i & 31) == 0 {
                    tracing::debug!(iter = i, "inversion checkpoint");
                } else {
                    tracing::trace!(iter = i, "inversion sample");
                }

                let a = fe_from_b32_checked(&a_b32);
                let b = fe_from_b32_checked(&b_b32);

                let mut inv_a = Fe5x52::new();
                crate::fe_inv(&mut inv_a as *mut Fe5x52, &a as *const Fe5x52);

                let mut inv_inv_a = Fe5x52::new();
                crate::fe_inv(&mut inv_inv_a as *mut Fe5x52, &inv_a as *const Fe5x52);

                let got_roundtrip = fe_to_b32_normalized(&mut inv_inv_a);
                assert_eq!(got_roundtrip, a_b32);

                let mut ab = Fe5x52::new();
                crate::fe_mul(&mut ab as *mut Fe5x52, &a as *const Fe5x52, &b as *const Fe5x52);

                let mut inv_ab = Fe5x52::new();
                crate::fe_inv(&mut inv_ab as *mut Fe5x52, &ab as *const Fe5x52);

                let mut inv_b = Fe5x52::new();
                crate::fe_inv(&mut inv_b as *mut Fe5x52, &b as *const Fe5x52);

                let mut inv_a_mul_inv_b = Fe5x52::new();
                crate::fe_mul(&mut inv_a_mul_inv_b as *mut Fe5x52, &inv_a as *const Fe5x52, &inv_b as *const Fe5x52);

                let inv_ab_b32 = fe_to_b32_normalized(&mut inv_ab);
                let inv_a_mul_inv_b_b32 = fe_to_b32_normalized(&mut inv_a_mul_inv_b);

                assert_eq!(inv_ab_b32, inv_a_mul_inv_b_b32);

                let mut check = Fe5x52::new();
                crate::fe_mul(&mut check as *mut Fe5x52, &ab as *const Fe5x52, &inv_ab as *const Fe5x52);
                let check_b32 = fe_to_b32_normalized(&mut check);
                assert_eq!(check_b32, u64_to_be32(1));

                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod field_element_normalization_equivalence_fuzz_tests {
    use super::*;

    const FIELD_P_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
    ];

    fn xorshift64star(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        *state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D_u64)
    }

    fn be32_lt(a: &[u8; 32], b: &[u8; 32]) -> bool {
        let mut i: usize = 0;
        while i < 32 {
            if a[i] < b[i] {
                return true;
            }
            if a[i] > b[i] {
                return false;
            }
            i += 1;
        }
        false
    }

    fn is_zero_b32(x: &[u8; 32]) -> bool {
        let mut acc: u8 = 0;
        let mut i: usize = 0;
        while i < 32 {
            acc |= x[i];
            i += 1;
        }
        acc == 0
    }

    fn random_b32_under_p(state: &mut u64) -> [u8; 32] {
        loop {
            let w0 = xorshift64star(state).to_be_bytes();
            let w1 = xorshift64star(state).to_be_bytes();
            let w2 = xorshift64star(state).to_be_bytes();
            let w3 = xorshift64star(state).to_be_bytes();

            let mut out = [0u8; 32];
            out[0..8].copy_from_slice(&w0);
            out[8..16].copy_from_slice(&w1);
            out[16..24].copy_from_slice(&w2);
            out[24..32].copy_from_slice(&w3);

            if be32_lt(&out, &FIELD_P_B32) {
                return out;
            }
        }
    }

    unsafe fn fe_from_b32_checked(bytes: &[u8; 32]) -> Fe5x52 {
        let mut fe = Fe5x52::new();
        let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, bytes.as_ptr());
        assert_eq!(ret, 1);
        fe
    }

    unsafe fn fe_to_b32_assuming_normalized(fe: &Fe5x52) -> [u8; 32] {
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    unsafe fn fe_to_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        fe_to_b32_assuming_normalized(fe)
    }

    unsafe fn fe_bitwise_copy(src: &Fe5x52) -> Fe5x52 {
        let mut out = core::mem::MaybeUninit::<Fe5x52>::uninit();
        core::ptr::copy_nonoverlapping(src as *const Fe5x52, out.as_mut_ptr(), 1);
        out.assume_init()
    }

    #[traced_test]
    fn normalize_and_normalize_var_agree_on_values_with_small_magnitude_growth() {
        tracing::info!("deterministic fuzz: fe_normalize and fe_normalize_var agree for non-normalized values built via add/mul_int");

        unsafe {
            let mut seed: u64 = 0x9999_8888_7777_6666u64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let a_b32 = random_b32_under_p(&mut seed);
                let b_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "normalize equivalence checkpoint");
                } else {
                    tracing::trace!(iter = i, "normalize equivalence sample");
                }

                let mut r = fe_from_b32_checked(&a_b32);
                let b = fe_from_b32_checked(&b_b32);

                crate::fe_add(&mut r as *mut Fe5x52, &b as *const Fe5x52);
                crate::fe_mul_int(&mut r as *mut Fe5x52, 8);
                crate::fe_add(&mut r as *mut Fe5x52, &b as *const Fe5x52);

                let mut r_norm = fe_bitwise_copy(&r);
                let mut r_var = fe_bitwise_copy(&r);

                crate::fe_normalize(&mut r_norm as *mut Fe5x52);
                crate::fe_normalize_var(&mut r_var as *mut Fe5x52);

                let bn = fe_to_b32_assuming_normalized(&r_norm);
                let bv = fe_to_b32_assuming_normalized(&r_var);
                assert_eq!(bn, bv);

                assert!(be32_lt(&bn, &FIELD_P_B32) || is_zero_b32(&bn));
                i += 1;
            }
        }
    }

    #[traced_test]
    fn normalize_weak_then_normalize_matches_direct_normalize_for_scaled_values() {
        tracing::info!("deterministic fuzz: fe_normalize_weak + fe_normalize matches direct fe_normalize for scaled values");

        unsafe {
            let mut seed: u64 = 0xABCD_EF01_2345_6789u64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let a_b32 = random_b32_under_p(&mut seed);
                let b_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "weak normalize checkpoint");
                } else {
                    tracing::trace!(iter = i, "weak normalize sample");
                }

                let mut r = fe_from_b32_checked(&a_b32);
                let b = fe_from_b32_checked(&b_b32);

                crate::fe_mul_int(&mut r as *mut Fe5x52, 8);
                crate::fe_add(&mut r as *mut Fe5x52, &b as *const Fe5x52);
                crate::fe_mul_int(&mut r as *mut Fe5x52, 3);

                let mut direct = fe_bitwise_copy(&r);
                let mut weak = fe_bitwise_copy(&r);

                crate::fe_normalize(&mut direct as *mut Fe5x52);

                crate::fe_normalize_weak(&mut weak as *mut Fe5x52);
                crate::fe_normalize(&mut weak as *mut Fe5x52);

                let bd = fe_to_b32_assuming_normalized(&direct);
                let bw = fe_to_b32_assuming_normalized(&weak);
                assert_eq!(bd, bw);

                i += 1;
            }
        }
    }

    #[traced_test]
    fn normalizes_to_zero_variants_agree_for_constructed_zero_representations_and_nonzero_perturbations() {
        tracing::info!("deterministic fuzz: fe_normalizes_to_zero and _var agree on constructed zero representations");

        unsafe {
            let mut seed: u64 = 0x0DDC_0FFE_EE11_2233u64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let a_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "normalizes_to_zero checkpoint");
                } else {
                    tracing::trace!(iter = i, "normalizes_to_zero sample");
                }

                let a = fe_from_b32_checked(&a_b32);

                let mut neg = Fe5x52::new();
                crate::fe_negate(&mut neg as *mut Fe5x52, &a as *const Fe5x52, 1);

                let mut sum = fe_from_b32_checked(&a_b32);
                crate::fe_add(&mut sum as *mut Fe5x52, &neg as *const Fe5x52);

                let ct = crate::fe_normalizes_to_zero(&sum as *const Fe5x52);
                let var = crate::fe_normalizes_to_zero_var(&sum as *const Fe5x52);
                assert_eq!(ct, 1);
                assert_eq!(var, 1);

                let sum_b32 = fe_to_b32_normalized(&mut sum);
                assert_eq!(sum_b32, [0u8; 32]);

                let mut one = Fe5x52::new();
                crate::fe_set_int(&mut one as *mut Fe5x52, 1);
                crate::fe_add(&mut sum as *mut Fe5x52, &one as *const Fe5x52);

                let ct2 = crate::fe_normalizes_to_zero(&sum as *const Fe5x52);
                let var2 = crate::fe_normalizes_to_zero_var(&sum as *const Fe5x52);
                assert_eq!(ct2, 0);
                assert_eq!(var2, 0);

                i += 1;
            }
        }
    }

    #[traced_test]
    fn normalize_idempotence_holds_for_both_constant_time_and_variable_time_normalizers() {
        tracing::info!("deterministic fuzz: fe_normalize and fe_normalize_var are idempotent on their outputs");

        unsafe {
            let mut seed: u64 = 0xCAFED00D_DEADC0DEu64;
            let iters: usize = 256;

            let mut i: usize = 0;
            while i < iters {
                let a_b32 = random_b32_under_p(&mut seed);

                if (i & 63) == 0 {
                    tracing::debug!(iter = i, "idempotence checkpoint");
                } else {
                    tracing::trace!(iter = i, "idempotence sample");
                }

                let mut a1 = fe_from_b32_checked(&a_b32);
                crate::fe_normalize(&mut a1 as *mut Fe5x52);
                let before1 = fe_to_b32_assuming_normalized(&a1);
                crate::fe_normalize(&mut a1 as *mut Fe5x52);
                let after1 = fe_to_b32_assuming_normalized(&a1);
                assert_eq!(before1, after1);

                let mut a2 = fe_from_b32_checked(&a_b32);
                crate::fe_normalize_var(&mut a2 as *mut Fe5x52);
                let before2 = fe_to_b32_assuming_normalized(&a2);
                crate::fe_normalize_var(&mut a2 as *mut Fe5x52);
                let after2 = fe_to_b32_assuming_normalized(&a2);
                assert_eq!(before2, after2);

                i += 1;
            }
        }
    }
}
