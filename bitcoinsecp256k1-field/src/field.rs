// ---------------- [ File: bitcoinsecp256k1-field/src/field.rs ]
/*!
  | Field element module.
  | 
  | Field elements can be represented in
  | several ways, but code accessing it
  | (and implementations) need to take
  | certain properties into account:
  | 
  | - Each field element can be normalized
  | or not.
  | 
  | - Each field element has a magnitude,
  | which represents how far away its representation
  | is away from normalization. Normalized
  | elements always have a magnitude of
  | 1, but a magnitude of 1 doesn't imply
  | normality.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field.h]

#[cfg(feature="widemul-int128")] pub type Fe = Fe5x52;
#[cfg(feature="widemul-int128")] pub type FeStorage = Fe5x52Storage;

#[cfg(feature="widemul-int64")] pub type Fe = Fe10x26;
#[cfg(feature="widemul-int64")] pub type FeStorage = Fe10x26Storage;

/**
  | Normalize a field element. This brings
  | the field element to a canonical representation,
  | reduces its magnitude to 1, and reduces
  | it modulo field size `p`.
  |
  */
pub fn fe_normalize(r: *mut Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_normalize(r);
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_normalize(r);
    }
}

/**
  | Weakly normalize a field element: reduce
  | its magnitude to 1, but don't fully normalize.
  |
  */
pub fn fe_normalize_weak(r: *mut Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_normalize_weak(r);
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_normalize_weak(r);
    }
}

/**
  | Normalize a field element, without
  | constant-time guarantee.
  |
  */
pub fn fe_normalize_var(r: *mut Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_normalize_var(r);
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_normalize_var(r);
    }
}

/**
  | Verify whether a field element represents
  | zero i.e. would normalize to a zero value.
  |
  */
pub fn fe_normalizes_to_zero(r: *const Fe) -> i32 {
    #[cfg(feature="widemul-int128")]
    {
        return bitcoinsecp256k1_fe5x52::fe_normalizes_to_zero(r);
    }
    #[cfg(feature="widemul-int64")]
    {
        return bitcoinsecp256k1_fe10x26::fe_normalizes_to_zero(r);
    }
}

/**
  | Verify whether a field element represents
  | zero i.e. would normalize to a zero value,
  | without constant-time guarantee.
  |
  */
pub fn fe_normalizes_to_zero_var(r: *const Fe) -> i32 {
    #[cfg(feature="widemul-int128")]
    {
        return bitcoinsecp256k1_fe5x52::fe_normalizes_to_zero_var(
            r,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        return bitcoinsecp256k1_fe10x26::fe_normalizes_to_zero_var(
            r,
        );
    }
}

/**
  | Set a field element equal to a small integer.
  | Resulting field element is normalized.
  |
  */
pub fn fe_set_int(r: *mut Fe, a: i32) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_set_int(r, a);
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_set_int(r, a);
    }
}

/**
  | Sets a field element equal to zero, initializing
  | all fields.
  |
  */
pub fn fe_clear(a: *mut Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_clear(a);
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_clear(a);
    }
}

/**
  | Verify whether a field element is zero.
  | Requires the input to be normalized.
  |
  */
pub fn fe_is_zero(a: *const Fe) -> i32 {
    #[cfg(feature="widemul-int128")]
    {
        return bitcoinsecp256k1_fe5x52::fe_is_zero(a);
    }
    #[cfg(feature="widemul-int64")]
    {
        return bitcoinsecp256k1_fe10x26::fe_is_zero(a);
    }
}

/**
  | Check the "oddness" of a field element.
  | Requires the input to be normalized.
  |
  */
pub fn fe_is_odd(a: *const Fe) -> i32 {
    #[cfg(feature="widemul-int128")]
    {
        return bitcoinsecp256k1_fe5x52::fe_is_odd(a);
    }
    #[cfg(feature="widemul-int64")]
    {
        return bitcoinsecp256k1_fe10x26::fe_is_odd(a);
    }
}

/**
  | Compare two field elements. Requires
  | both inputs to be normalized
  |
  */
pub fn fe_cmp_var(a: *const Fe, b: *const Fe) -> i32 {
    #[cfg(feature="widemul-int128")]
    {
        return bitcoinsecp256k1_fe5x52::fe_cmp_var(
            a,
            b,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        return bitcoinsecp256k1_fe10x26::fe_cmp_var(
            a,
            b,
        );
    }
}

/**
  | Set a field element equal to 32-byte
  | big endian value. If successful, the
  | resulting field element is normalized.
  |
  */
pub fn fe_set_b32(r: *mut Fe, a: *const u8) -> i32 {
    #[cfg(feature="widemul-int128")]
    {
        return bitcoinsecp256k1_fe5x52::fe_set_b32(r, a);
    }
    #[cfg(feature="widemul-int64")]
    {
        return bitcoinsecp256k1_fe10x26::fe_set_b32(r, a);
    }
}

/**
  | Convert a field element to a 32-byte
  | big endian value. Requires the input
  | to be normalized
  |
  */
pub fn fe_get_b32(r: *mut u8, a: *const Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_get_b32(r, a);
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_get_b32(r, a);
    }
}

/**
  | Set a field element equal to the additive
  | inverse of another. Takes a maximum
  | magnitude of the input as an argument.
  | The magnitude of the output is one higher.
  |
  */
pub fn fe_negate(r: *mut Fe, a: *const Fe, m: i32) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_negate(
            r,
            a,
            m,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_negate(
            r,
            a,
            m,
        );
    }
}

/**
  | Multiplies the passed field element
  | with a small integer constant. Multiplies
  | the magnitude by that small integer.
  |
  */
pub fn fe_mul_int(r: *mut Fe, a: i32) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_mul_int(r, a);
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_mul_int(r, a);
    }
}

/**
  | Adds a field element to another. The
  | result has the sum of the inputs' magnitudes
  | as magnitude.
  |
  */
pub fn fe_add(r: *mut Fe, a: *const Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_add(
            r,
            a,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_add(
            r,
            a,
        );
    }
}

/**
  | Sets a field element to be the product
  | of two others. Requires the inputs'
  | magnitudes to be at most 8.
  | 
  | The output magnitude is 1 (but not guaranteed
  | to be normalized).
  |
  */
pub fn fe_mul(r: *mut Fe, a: *const Fe, b: *const Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_mul(
            r,
            a,
            b,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_mul(
            r,
            a,
            b,
        );
    }
}

/**
  | Sets a field element to be the square
  | of another. Requires the input's magnitude
  | to be at most 8.
  | 
  | The output magnitude is 1 (but not guaranteed
  | to be normalized).
  |
  */
pub fn fe_sqr(r: *mut Fe, a: *const Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_sqr(
            r,
            a,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_sqr(
            r,
            a,
        );
    }
}

/**
  | Sets a field element to be the (modular)
  | inverse of another. Requires the input's
  | magnitude to be at most 8. The output
  | magnitude is 1 (but not guaranteed to
  | be normalized).
  |
  */
pub fn fe_inv(r: *mut Fe, a: *const Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_inv(
            r,
            a,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_inv(
            r,
            a,
        );
    }
}

/**
  | Potentially faster version of fe_inv,
  | without constant-time guarantee.
  |
  */
pub fn fe_inv_var(r: *mut Fe, a: *const Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_inv_var(
            r,
            a,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_inv_var(
            r,
            a,
        );
    }
}

/**
  | Convert a field element to the storage
  | type.
  |
  */
pub fn fe_to_storage(r: *mut FeStorage, a: *const Fe) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_to_storage(
            r,
            a,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_to_storage(
            r,
            a,
        );
    }
}

/**
  | Convert a field element back from the
  | storage type.
  |
  */
pub fn fe_from_storage(r: *mut Fe, a: *const FeStorage) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_from_storage(
            r,
            a,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_from_storage(
            r,
            a,
        );
    }
}

/**
  | If flag is true, set *r equal to *a; otherwise
  | leave it. Constant-time. Both *r and
  | *a must be initialized.
  |
  */
pub fn fe_storage_cmov(r: *mut FeStorage, a: *const FeStorage, flag: i32) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_storage_cmov(
            r,
            a,
            flag,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_storage_cmov(
            r,
            a,
            flag,
        );
    }
}

/// If flag is true, set *r equal to *a; otherwise leave it. Constant-time. Both *r and *a must be
/// initialized.
/// 
pub fn fe_cmov(r: *mut Fe, a: *const Fe, flag: i32) {
    #[cfg(feature="widemul-int128")]
    {
        bitcoinsecp256k1_fe5x52::fe_cmov(
            r,
            a,
            flag,
        );
    }
    #[cfg(feature="widemul-int64")]
    {
        bitcoinsecp256k1_fe10x26::fe_cmov(
            r,
            a,
            flag,
        );
    }
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_impl.h]

/// Compare two field elements. Requires magnitude-1 inputs.
/// 
#[inline]
pub fn fe_equal(a: *const Fe, b: *const Fe) -> i32 {
    unsafe {
        let mut na = core::mem::MaybeUninit::<Fe>::uninit();
        fe_negate(na.as_mut_ptr(), a, 1);
        fe_add(na.as_mut_ptr(), b);
        fe_normalizes_to_zero(na.as_ptr())
    }
}

/// Same as fe_equal, but may be variable time.
/// 
#[inline]
pub fn fe_equal_var(a: *const Fe, b: *const Fe) -> i32 {
    unsafe {
        let mut na = core::mem::MaybeUninit::<Fe>::uninit();
        fe_negate(na.as_mut_ptr(), a, 1);
        fe_add(na.as_mut_ptr(), b);
        fe_normalizes_to_zero_var(na.as_ptr())
    }
}

/// If a has a square root, it is computed in r and 1 is returned. 
///
/// If a does not have a square root, the root of its negation is computed and 0 is returned.
/// 
/// The input's magnitude can be at most 8. 
///
/// The output magnitude is 1 (but not guaranteed to be normalized). The result in r will always be
/// a square itself.
/// 
pub fn fe_sqrt(r: *mut Fe, a: *const Fe) -> i32 {
    unsafe {

        /** Given that p is congruent to 3 mod 4, we can compute the square root of
         *  a mod p as the (p+1)/4'th power of a.
         *
         *  As (p+1)/4 is an even number, it will have the same result for a and for
         *  (-a). Only one of these two numbers actually has a square root however,
         *  so we test at the end by squaring and comparing to the input.
         *  Also because (p+1)/4 is an even number, the computed square root is
         *  itself always a square (a ** ((p+1)/4) is the square of a ** ((p+1)/8)).
         */
        let mut x2: Fe = core::mem::zeroed();
        let mut x3: Fe = core::mem::zeroed();
        let mut x6: Fe;
        let mut x9: Fe;
        let mut x11: Fe;
        let mut x22: Fe;
        let mut x44: Fe;
        let mut x88: Fe;
        let mut x176: Fe;
        let mut x220: Fe;
        let mut x223: Fe;
        let mut t1: Fe;
        let mut j: i32;

        verify_check!(r != a as *mut Fe);

        /** The binary representation of (p + 1)/4 has 3 blocks of 1s, with lengths in
         *  { 2, 22, 223 }. Use an addition chain to calculate 2^n - 1 for each block:
         *  1, [2], 3, 6, 9, 11, [22], 44, 88, 176, 220, [223]
         */
        fe_sqr(&mut x2 as *mut Fe, a);
        fe_mul(&mut x2 as *mut Fe, &x2 as *const Fe, a);

        fe_sqr(&mut x3 as *mut Fe, &x2 as *const Fe);
        fe_mul(&mut x3 as *mut Fe, &x3 as *const Fe, a);

        x6 = core::ptr::read(&x3 as *const Fe);
        j = 0;
        while j < 3 {
            fe_sqr(&mut x6 as *mut Fe, &x6 as *const Fe);
            j += 1;
        }
        fe_mul(&mut x6 as *mut Fe, &x6 as *const Fe, &x3 as *const Fe);

        x9 = core::ptr::read(&x6 as *const Fe);
        j = 0;
        while j < 3 {
            fe_sqr(&mut x9 as *mut Fe, &x9 as *const Fe);
            j += 1;
        }
        fe_mul(&mut x9 as *mut Fe, &x9 as *const Fe, &x3 as *const Fe);

        x11 = core::ptr::read(&x9 as *const Fe);
        j = 0;
        while j < 2 {
            fe_sqr(&mut x11 as *mut Fe, &x11 as *const Fe);
            j += 1;
        }
        fe_mul(&mut x11 as *mut Fe, &x11 as *const Fe, &x2 as *const Fe);

        x22 = core::ptr::read(&x11 as *const Fe);
        j = 0;
        while j < 11 {
            fe_sqr(&mut x22 as *mut Fe, &x22 as *const Fe);
            j += 1;
        }
        fe_mul(&mut x22 as *mut Fe, &x22 as *const Fe, &x11 as *const Fe);

        x44 = core::ptr::read(&x22 as *const Fe);
        j = 0;
        while j < 22 {
            fe_sqr(&mut x44 as *mut Fe, &x44 as *const Fe);
            j += 1;
        }
        fe_mul(&mut x44 as *mut Fe, &x44 as *const Fe, &x22 as *const Fe);

        x88 = core::ptr::read(&x44 as *const Fe);
        j = 0;
        while j < 44 {
            fe_sqr(&mut x88 as *mut Fe, &x88 as *const Fe);
            j += 1;
        }
        fe_mul(&mut x88 as *mut Fe, &x88 as *const Fe, &x44 as *const Fe);

        x176 = core::ptr::read(&x88 as *const Fe);
        j = 0;
        while j < 88 {
            fe_sqr(&mut x176 as *mut Fe, &x176 as *const Fe);
            j += 1;
        }
        fe_mul(&mut x176 as *mut Fe, &x176 as *const Fe, &x88 as *const Fe);

        x220 = core::ptr::read(&x176 as *const Fe);
        j = 0;
        while j < 44 {
            fe_sqr(&mut x220 as *mut Fe, &x220 as *const Fe);
            j += 1;
        }
        fe_mul(&mut x220 as *mut Fe, &x220 as *const Fe, &x44 as *const Fe);

        x223 = core::ptr::read(&x220 as *const Fe);
        j = 0;
        while j < 3 {
            fe_sqr(&mut x223 as *mut Fe, &x223 as *const Fe);
            j += 1;
        }
        fe_mul(&mut x223 as *mut Fe, &x223 as *const Fe, &x3 as *const Fe);

        /* The final result is then assembled using a sliding window over the blocks. */

        t1 = core::ptr::read(&x223 as *const Fe);
        j = 0;
        while j < 23 {
            fe_sqr(&mut t1 as *mut Fe, &t1 as *const Fe);
            j += 1;
        }
        fe_mul(&mut t1 as *mut Fe, &t1 as *const Fe, &x22 as *const Fe);
        j = 0;
        while j < 6 {
            fe_sqr(&mut t1 as *mut Fe, &t1 as *const Fe);
            j += 1;
        }
        fe_mul(&mut t1 as *mut Fe, &t1 as *const Fe, &x2 as *const Fe);
        fe_sqr(&mut t1 as *mut Fe, &t1 as *const Fe);
        fe_sqr(r, &t1 as *const Fe);

        /* Check that a square root was actually calculated */

        fe_sqr(&mut t1 as *mut Fe, r as *const Fe);
        fe_equal(&t1 as *const Fe, a)
    }
}

pub const FE_ONE: Fe = fe_const!(0, 0, 0, 0, 0, 0, 0, 1);

#[cfg(test)]
mod field_element_api_contract_suite {
    use super::*;

    const B32_ZERO: [u8; 32] = [0u8; 32];

    const B32_ONE: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01,
    ];

    const FIELD_MODULUS_P: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
    ];

    const FIELD_MODULUS_P_MINUS_1: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFE, 0xFF, 0xFF, 0xFC, 0x2E,
    ];

    const FIELD_MODULUS_P_MINUS_2: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFE, 0xFF, 0xFF, 0xFC, 0x2D,
    ];

    const U256_MAX: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];

    fn field_element_from_small_int(value: i32) -> Fe {
        debug!(value, "constructing field element from small int");
        let mut fe = core::mem::MaybeUninit::<Fe>::uninit();
        unsafe {
            fe_set_int(fe.as_mut_ptr(), value);
            fe.assume_init()
        }
    }

    fn field_element_from_b32_with_status(bytes: &[u8; 32]) -> (i32, Fe) {
        trace!("constructing field element from 32-byte big-endian input");
        let mut fe = core::mem::MaybeUninit::<Fe>::uninit();
        unsafe {
            fe_clear(fe.as_mut_ptr());
            let status = fe_set_b32(fe.as_mut_ptr(), bytes.as_ptr());
            (status, fe.assume_init())
        }
    }

    fn field_element_from_b32_expect_success(label: &'static str, bytes: &[u8; 32]) -> Fe {
        let (status, fe) = field_element_from_b32_with_status(bytes);
        if status != 1 {
            error!(label, status, "fe_set_b32 unexpectedly rejected a supposedly-valid input");
        }
        assert_eq!(status, 1, "fe_set_b32 expected success for {label}");
        fe
    }

    fn assert_set_b32_rejects(label: &'static str, bytes: &[u8; 32]) {
        let (status, _fe) = field_element_from_b32_with_status(bytes);
        if status != 0 {
            error!(label, status, "fe_set_b32 unexpectedly accepted an out-of-range input");
        }
        assert_eq!(status, 0, "fe_set_b32 expected failure for {label}");
    }

    fn clone_field_element_via_cmov(src: &Fe) -> Fe {
        trace!("cloning field element via fe_cmov (interface-level clone)");
        let mut dst = core::mem::MaybeUninit::<Fe>::uninit();
        unsafe {
            fe_clear(dst.as_mut_ptr());
            fe_cmov(dst.as_mut_ptr(), src as *const Fe, 1);
            dst.assume_init()
        }
    }

    fn canonical_b32_from_field_element(a: &Fe) -> [u8; 32] {
        trace!("extracting canonical b32 from field element (clone + normalize + get_b32)");
        let mut tmp = clone_field_element_via_cmov(a);
        unsafe {
            fe_normalize(&mut tmp as *mut Fe);
        }
        let mut out = [0u8; 32];
        unsafe {
            fe_get_b32(out.as_mut_ptr(), &tmp as *const Fe);
        }
        out
    }

    fn direct_b32_from_normalized_field_element(a: &Fe) -> [u8; 32] {
        trace!("extracting b32 via fe_get_b32 (requires normalized input)");
        let mut out = [0u8; 32];
        unsafe {
            fe_get_b32(out.as_mut_ptr(), a as *const Fe);
        }
        out
    }

    fn field_storage_from_field_element(a: &Fe) -> FeStorage {
        trace!("converting field element to storage representation");
        let mut storage = core::mem::MaybeUninit::<FeStorage>::uninit();
        unsafe {
            fe_to_storage(storage.as_mut_ptr(), a as *const Fe);
            storage.assume_init()
        }
    }

    fn field_element_from_storage(a: &FeStorage) -> Fe {
        trace!("converting field element from storage representation");
        let mut fe = core::mem::MaybeUninit::<Fe>::uninit();
        unsafe {
            fe_from_storage(fe.as_mut_ptr(), a as *const FeStorage);
            fe.assume_init()
        }
    }

    #[traced_test]
    fn field_element_set_int_and_clear_obey_zero_and_odd_contract() {
        info!("validating fe_set_int/fe_clear and normalized predicates (fe_is_zero/fe_is_odd)");

        let zero = field_element_from_small_int(0);
        let zero_b32 = direct_b32_from_normalized_field_element(&zero);
        assert_eq!(zero_b32, B32_ZERO, "set_int(0) must serialize to 0");
        assert_eq!(
            unsafe { fe_is_zero(&zero as *const Fe) },
            1,
            "set_int(0) must be recognized as zero"
        );
        assert_eq!(
            unsafe { fe_is_odd(&zero as *const Fe) },
            0,
            "0 must not be odd"
        );

        let one = field_element_from_small_int(1);
        let one_b32 = direct_b32_from_normalized_field_element(&one);
        assert_eq!(one_b32, B32_ONE, "set_int(1) must serialize to 1");
        assert_eq!(
            unsafe { fe_is_zero(&one as *const Fe) },
            0,
            "1 must not be recognized as zero"
        );
        assert_eq!(
            unsafe { fe_is_odd(&one as *const Fe) },
            1,
            "1 must be odd"
        );

        let two = field_element_from_small_int(2);
        let two_b32 = direct_b32_from_normalized_field_element(&two);
        assert_eq!(two_b32[31], 0x02, "set_int(2) must serialize to 2");
        assert_eq!(
            unsafe { fe_is_odd(&two as *const Fe) },
            0,
            "2 must not be odd"
        );

        let mut cleared = field_element_from_small_int(7);
        unsafe {
            fe_clear(&mut cleared as *mut Fe);
            fe_normalize(&mut cleared as *mut Fe);
        }
        assert_eq!(
            unsafe { fe_is_zero(&cleared as *const Fe) },
            1,
            "fe_clear must produce zero"
        );
        assert_eq!(
            direct_b32_from_normalized_field_element(&cleared),
            B32_ZERO,
            "cleared field element must serialize to 0"
        );

        let fe_one_b32 = canonical_b32_from_field_element(&FE_ONE);
        assert_eq!(
            fe_one_b32, B32_ONE,
            "FE_ONE must serialize to canonical 1"
        );

        let cmp_one = unsafe { fe_cmp_var(&one as *const Fe, &FE_ONE as *const Fe) };
        assert_eq!(
            cmp_one, 0,
            "fe_cmp_var must report equality for identical canonical values"
        );
    }

    #[traced_test]
    fn field_element_set_b32_accepts_canonical_range_and_rejects_overflow() {
        info!("validating fe_set_b32 boundary behavior and fe_get_b32 roundtrip");

        let fe0 = field_element_from_b32_expect_success("b32=0", &B32_ZERO);
        assert_eq!(
            direct_b32_from_normalized_field_element(&fe0),
            B32_ZERO,
            "b32=0 must roundtrip through get_b32"
        );
        assert_eq!(
            unsafe { fe_is_zero(&fe0 as *const Fe) },
            1,
            "b32=0 must be recognized as zero"
        );

        let fe1 = field_element_from_b32_expect_success("b32=1", &B32_ONE);
        assert_eq!(
            direct_b32_from_normalized_field_element(&fe1),
            B32_ONE,
            "b32=1 must roundtrip through get_b32"
        );
        assert_eq!(
            unsafe { fe_is_odd(&fe1 as *const Fe) },
            1,
            "b32=1 must be odd"
        );

        let fe_pm1 = field_element_from_b32_expect_success("b32=p-1", &FIELD_MODULUS_P_MINUS_1);
        assert_eq!(
            direct_b32_from_normalized_field_element(&fe_pm1),
            FIELD_MODULUS_P_MINUS_1,
            "b32=p-1 must roundtrip through get_b32"
        );
        assert_eq!(
            unsafe { fe_is_zero(&fe_pm1 as *const Fe) },
            0,
            "p-1 must not be recognized as zero"
        );
        assert_eq!(
            unsafe { fe_is_odd(&fe_pm1 as *const Fe) },
            0,
            "p-1 is even in canonical encoding; is_odd must be 0"
        );

        warn!("testing fe_set_b32 rejection cases (expected failures)");
        assert_set_b32_rejects("b32=p (overflow)", &FIELD_MODULUS_P);
        assert_set_b32_rejects("b32=2^256-1 (overflow)", &U256_MAX);
    }

    #[traced_test]
    fn field_element_normalization_and_zero_detection_handle_noncanonical_representations() {
        info!("validating normalization functions and (normalizes_to_zero) behaviors on noncanonical values");

        let mut pm1 = field_element_from_b32_expect_success("p-1", &FIELD_MODULUS_P_MINUS_1);
        let one = field_element_from_small_int(1);

        trace!("constructing a noncanonical representation of 0 by computing (p-1)+1 = p");
        unsafe {
            fe_add(&mut pm1 as *mut Fe, &one as *const Fe);
        }

        assert_eq!(
            unsafe { fe_normalizes_to_zero(&pm1 as *const Fe) },
            1,
            "(p-1)+1 must normalize to zero"
        );
        assert_eq!(
            unsafe { fe_normalizes_to_zero_var(&pm1 as *const Fe) },
            1,
            "(p-1)+1 must normalize to zero (var-time check)"
        );

        let mut weak = clone_field_element_via_cmov(&pm1);
        unsafe {
            fe_normalize_weak(&mut weak as *mut Fe);
        }
        assert_eq!(
            unsafe { fe_normalizes_to_zero(&weak as *const Fe) },
            1,
            "weak-normalized (p-1)+1 must still normalize to zero"
        );

        let zero = field_element_from_small_int(0);
        let eq_after_weak = unsafe { fe_equal(&weak as *const Fe, &zero as *const Fe) };
        assert_eq!(
            eq_after_weak, 1,
            "weak-normalized p representation must be equal (mod p) to zero"
        );

        let mut full = clone_field_element_via_cmov(&pm1);
        unsafe {
            fe_normalize(&mut full as *mut Fe);
        }
        assert_eq!(
            unsafe { fe_is_zero(&full as *const Fe) },
            1,
            "full normalization of p must produce canonical zero"
        );
        assert_eq!(
            direct_b32_from_normalized_field_element(&full),
            B32_ZERO,
            "normalized p must serialize to canonical zero"
        );

        let mut var = clone_field_element_via_cmov(&pm1);
        unsafe {
            fe_normalize_var(&mut var as *mut Fe);
        }
        assert_eq!(
            unsafe { fe_is_zero(&var as *const Fe) },
            1,
            "variable-time normalization of p must produce canonical zero"
        );
        assert_eq!(
            direct_b32_from_normalized_field_element(&var),
            B32_ZERO,
            "var-normalized p must serialize to canonical zero"
        );

        trace!("constructing a noncanonical representation of (p-2) by computing (p-1)+(p-1) = 2p-2");
        let mut twice_pm1 = field_element_from_b32_expect_success("p-1 (lhs)", &FIELD_MODULUS_P_MINUS_1);
        let pm1_rhs = field_element_from_b32_expect_success("p-1 (rhs)", &FIELD_MODULUS_P_MINUS_1);
        unsafe {
            fe_add(&mut twice_pm1 as *mut Fe, &pm1_rhs as *const Fe);
        }

        let mut twice_full = clone_field_element_via_cmov(&twice_pm1);
        unsafe {
            fe_normalize(&mut twice_full as *mut Fe);
        }
        assert_eq!(
            direct_b32_from_normalized_field_element(&twice_full),
            FIELD_MODULUS_P_MINUS_2,
            "normalize(2p-2) must yield p-2"
        );

        let mut twice_var = clone_field_element_via_cmov(&twice_pm1);
        unsafe {
            fe_normalize_var(&mut twice_var as *mut Fe);
        }
        assert_eq!(
            direct_b32_from_normalized_field_element(&twice_var),
            FIELD_MODULUS_P_MINUS_2,
            "normalize_var(2p-2) must yield p-2"
        );
    }

    #[traced_test]
    fn field_element_addition_and_negation_obey_additive_group_laws_mod_p() {
        info!("validating additive group laws via fe_negate, fe_add, and normalization");

        let samples: [i32; 8] = [0, 1, 2, 3, 7, 9, 42, 127];

        for &v in samples.iter() {
            trace!(v, "testing (-a)+a == 0 and -(-a) == a");

            let a = field_element_from_small_int(v);

            let mut neg_a = field_element_from_small_int(0);
            unsafe {
                fe_negate(&mut neg_a as *mut Fe, &a as *const Fe, 1);
            }

            let mut sum = clone_field_element_via_cmov(&neg_a);
            unsafe {
                fe_add(&mut sum as *mut Fe, &a as *const Fe);
            }

            assert_eq!(
                unsafe { fe_normalizes_to_zero(&sum as *const Fe) },
                1,
                "(-a)+a must normalize to 0 for v={v}"
            );

            unsafe {
                fe_normalize(&mut sum as *mut Fe);
            }
            assert_eq!(
                unsafe { fe_is_zero(&sum as *const Fe) },
                1,
                "(-a)+a must be canonical zero after normalization for v={v}"
            );
            assert_eq!(
                direct_b32_from_normalized_field_element(&sum),
                B32_ZERO,
                "(-a)+a must serialize to 0 after normalization for v={v}"
            );

            let mut neg_neg_a = field_element_from_small_int(0);
            unsafe {
                fe_negate(&mut neg_neg_a as *mut Fe, &neg_a as *const Fe, 2);
                fe_normalize(&mut neg_neg_a as *mut Fe);
            }

            let a_b32 = canonical_b32_from_field_element(&a);
            let neg_neg_a_b32 = direct_b32_from_normalized_field_element(&neg_neg_a);
            assert_eq!(
                neg_neg_a_b32, a_b32,
                "-(-a) must equal a after normalization for v={v}"
            );
        }
    }

    #[traced_test]
    fn field_element_multiplication_square_and_mul_int_match_expected_values() {
        info!("validating fe_mul, fe_sqr, and fe_mul_int against known arithmetic identities");

        let mul_vectors: &[(i32, i32, i32)] = &[
            (0, 5, 0),
            (1, 123, 123),
            (2, 2, 4),
            (3, 7, 21),
            (12, 13, 156),
        ];

        for &(a_i, b_i, expected_i) in mul_vectors.iter() {
            trace!(a_i, b_i, expected_i, "testing multiplication vector");

            let a = field_element_from_small_int(a_i);
            let b = field_element_from_small_int(b_i);
            let expected = field_element_from_small_int(expected_i);

            let mut r = field_element_from_small_int(0);
            unsafe {
                fe_mul(&mut r as *mut Fe, &a as *const Fe, &b as *const Fe);
                fe_normalize(&mut r as *mut Fe);
            }

            let r_b32 = direct_b32_from_normalized_field_element(&r);
            let expected_b32 = direct_b32_from_normalized_field_element(&expected);
            assert_eq!(
                r_b32, expected_b32,
                "fe_mul({a_i},{b_i}) must equal {expected_i} after normalization"
            );
        }

        let sqr_samples: [i32; 6] = [0, 1, 2, 7, 12, 16];
        for &v in sqr_samples.iter() {
            trace!(v, "testing squaring sample");

            let a = field_element_from_small_int(v);
            let expected = field_element_from_small_int(v * v);

            let mut r = field_element_from_small_int(0);
            unsafe {
                fe_sqr(&mut r as *mut Fe, &a as *const Fe);
                fe_normalize(&mut r as *mut Fe);
            }

            let r_b32 = direct_b32_from_normalized_field_element(&r);
            let expected_b32 = direct_b32_from_normalized_field_element(&expected);
            assert_eq!(
                r_b32, expected_b32,
                "fe_sqr({v}) must equal {} after normalization",
                v * v
            );
        }

        trace!("testing fe_mul_int with a small scalar multiplier");
        let mut r = field_element_from_small_int(9);
        unsafe {
            fe_mul_int(&mut r as *mut Fe, 7);
            fe_normalize(&mut r as *mut Fe);
        }
        let expected_63 = field_element_from_small_int(63);
        assert_eq!(
            direct_b32_from_normalized_field_element(&r),
            direct_b32_from_normalized_field_element(&expected_63),
            "9*7 must equal 63 after normalization"
        );

        trace!("testing fe_mul and fe_sqr near the modulus: (p-1)*(p-1) == 1 and (p-1)^2 == 1");
        let pm1 = field_element_from_b32_expect_success("p-1", &FIELD_MODULUS_P_MINUS_1);

        let mut mul_pm1 = field_element_from_small_int(0);
        unsafe {
            fe_mul(
                &mut mul_pm1 as *mut Fe,
                &pm1 as *const Fe,
                &pm1 as *const Fe,
            );
            fe_normalize(&mut mul_pm1 as *mut Fe);
        }
        assert_eq!(
            direct_b32_from_normalized_field_element(&mul_pm1),
            B32_ONE,
            "(p-1)*(p-1) must normalize to 1"
        );

        let mut sqr_pm1 = field_element_from_small_int(0);
        unsafe {
            fe_sqr(&mut sqr_pm1 as *mut Fe, &pm1 as *const Fe);
            fe_normalize(&mut sqr_pm1 as *mut Fe);
        }
        assert_eq!(
            direct_b32_from_normalized_field_element(&sqr_pm1),
            B32_ONE,
            "(p-1)^2 must normalize to 1"
        );

        trace!("testing fe_mul_int near the modulus: (p-1)*2 == p-2");
        let mut pm1_times_two = clone_field_element_via_cmov(&pm1);
        unsafe {
            fe_mul_int(&mut pm1_times_two as *mut Fe, 2);
            fe_normalize(&mut pm1_times_two as *mut Fe);
        }
        assert_eq!(
            direct_b32_from_normalized_field_element(&pm1_times_two),
            FIELD_MODULUS_P_MINUS_2,
            "(p-1)*2 must normalize to p-2"
        );
    }

    #[traced_test]
    fn field_element_inversion_produces_multiplicative_identity_for_nonzero_inputs() {
        info!("validating fe_inv and fe_inv_var via a * inv(a) == 1 for nonzero inputs");

        let mut test_elements: [Fe; 10] = [
            field_element_from_small_int(1),
            field_element_from_small_int(2),
            field_element_from_small_int(3),
            field_element_from_small_int(4),
            field_element_from_small_int(5),
            field_element_from_small_int(7),
            field_element_from_small_int(11),
            field_element_from_small_int(42),
            field_element_from_small_int(127),
            field_element_from_b32_expect_success("p-1", &FIELD_MODULUS_P_MINUS_1),
        ];

        for (idx, a) in test_elements.iter_mut().enumerate() {
            let a_b32 = direct_b32_from_normalized_field_element(a);
            trace!(idx, "testing inversion sample");

            if unsafe { fe_is_zero(a as *const Fe) } != 0 {
                warn!(idx, "skipping inversion identity for zero input (not part of contract here)");
                continue;
            }

            let mut inv_ct = field_element_from_small_int(0);
            unsafe {
                fe_inv(&mut inv_ct as *mut Fe, a as *const Fe);
            }

            let mut inv_vt = field_element_from_small_int(0);
            unsafe {
                fe_inv_var(&mut inv_vt as *mut Fe, a as *const Fe);
            }

            let mut prod_ct = field_element_from_small_int(0);
            unsafe {
                fe_mul(
                    &mut prod_ct as *mut Fe,
                    &inv_ct as *const Fe,
                    a as *const Fe,
                );
                fe_normalize(&mut prod_ct as *mut Fe);
            }
            assert_eq!(
                direct_b32_from_normalized_field_element(&prod_ct),
                B32_ONE,
                "a*inv(a) must normalize to 1 (constant-time inverse path)"
            );

            let mut prod_vt = field_element_from_small_int(0);
            unsafe {
                fe_mul(
                    &mut prod_vt as *mut Fe,
                    &inv_vt as *const Fe,
                    a as *const Fe,
                );
                fe_normalize(&mut prod_vt as *mut Fe);
            }
            assert_eq!(
                direct_b32_from_normalized_field_element(&prod_vt),
                B32_ONE,
                "a*inv_var(a) must normalize to 1 (variable-time inverse path)"
            );

            let inv_ct_norm = canonical_b32_from_field_element(&inv_ct);
            let inv_vt_norm = canonical_b32_from_field_element(&inv_vt);
            if inv_ct_norm != inv_vt_norm {
                error!(
                    idx,
                    "fe_inv and fe_inv_var produced different canonical inverses"
                );
            }
            assert_eq!(
                inv_ct_norm, inv_vt_norm,
                "fe_inv and fe_inv_var must agree on the unique modular inverse"
            );

            debug!(
                idx,
                a_last_byte = a_b32[31],
                "inversion identity validated"
            );
        }
    }

    #[traced_test]
    fn field_element_storage_roundtrip_and_conditional_moves_preserve_values() {
        info!("validating fe_to_storage/fe_from_storage roundtrip and conditional move semantics");

        let values: [Fe; 6] = [
            field_element_from_small_int(0),
            field_element_from_small_int(1),
            field_element_from_small_int(2),
            field_element_from_small_int(7),
            field_element_from_small_int(9),
            field_element_from_b32_expect_success("p-1", &FIELD_MODULUS_P_MINUS_1),
        ];

        for (idx, a) in values.iter().enumerate() {
            trace!(idx, "testing storage roundtrip");
            let a_b32 = direct_b32_from_normalized_field_element(a);

            let storage = field_storage_from_field_element(a);
            let mut back = field_element_from_storage(&storage);
            unsafe {
                fe_normalize(&mut back as *mut Fe);
            }

            let back_b32 = direct_b32_from_normalized_field_element(&back);
            assert_eq!(
                back_b32, a_b32,
                "to_storage/from_storage must preserve canonical field value"
            );
        }

        trace!("testing fe_cmov functional behavior (flag=0 keeps, flag=1 overwrites)");
        let five = field_element_from_small_int(5);
        let nine = field_element_from_small_int(9);

        let mut r = clone_field_element_via_cmov(&five);
        unsafe {
            fe_cmov(&mut r as *mut Fe, &nine as *const Fe, 0);
            fe_normalize(&mut r as *mut Fe);
        }
        assert_eq!(
            direct_b32_from_normalized_field_element(&r),
            direct_b32_from_normalized_field_element(&five),
            "fe_cmov flag=0 must leave destination unchanged"
        );

        unsafe {
            fe_cmov(&mut r as *mut Fe, &nine as *const Fe, 1);
            fe_normalize(&mut r as *mut Fe);
        }
        assert_eq!(
            direct_b32_from_normalized_field_element(&r),
            direct_b32_from_normalized_field_element(&nine),
            "fe_cmov flag=1 must copy source to destination"
        );

        trace!("testing fe_storage_cmov functional behavior via roundtrip comparison");
        let mut r_storage = field_storage_from_field_element(&five);
        let a_storage = field_storage_from_field_element(&nine);

        unsafe {
            fe_storage_cmov(&mut r_storage as *mut FeStorage, &a_storage as *const FeStorage, 0);
        }
        let mut r_after_0 = field_element_from_storage(&r_storage);
        unsafe {
            fe_normalize(&mut r_after_0 as *mut Fe);
        }
        assert_eq!(
            direct_b32_from_normalized_field_element(&r_after_0),
            direct_b32_from_normalized_field_element(&five),
            "fe_storage_cmov flag=0 must leave destination unchanged"
        );

        unsafe {
            fe_storage_cmov(&mut r_storage as *mut FeStorage, &a_storage as *const FeStorage, 1);
        }
        let mut r_after_1 = field_element_from_storage(&r_storage);
        unsafe {
            fe_normalize(&mut r_after_1 as *mut Fe);
        }
        assert_eq!(
            direct_b32_from_normalized_field_element(&r_after_1),
            direct_b32_from_normalized_field_element(&nine),
            "fe_storage_cmov flag=1 must copy source to destination"
        );
    }

    #[traced_test]
    fn field_element_equality_and_comparison_are_consistent_across_interfaces() {
        info!("validating fe_equal/fe_equal_var consistency and alignment with fe_cmp_var");

        let zero = field_element_from_small_int(0);
        let one = field_element_from_small_int(1);
        let two = field_element_from_small_int(2);
        let pm1 = field_element_from_b32_expect_success("p-1", &FIELD_MODULUS_P_MINUS_1);
        let pm2 = field_element_from_b32_expect_success("p-2", &FIELD_MODULUS_P_MINUS_2);

        let cases: &[(Fe, Fe, bool)] = &[
            (clone_field_element_via_cmov(&zero), clone_field_element_via_cmov(&zero), true),
            (clone_field_element_via_cmov(&one), clone_field_element_via_cmov(&one), true),
            (clone_field_element_via_cmov(&one), clone_field_element_via_cmov(&two), false),
            (clone_field_element_via_cmov(&pm1), clone_field_element_via_cmov(&pm1), true),
            (clone_field_element_via_cmov(&pm2), clone_field_element_via_cmov(&pm1), false),
            (clone_field_element_via_cmov(&one), clone_field_element_via_cmov(&pm1), false),
        ];

        for (idx, (a, b, expect_equal)) in cases.iter().enumerate() {
            trace!(idx, expect_equal, "testing equality/comparison case");

            let eq = unsafe { fe_equal(a as *const Fe, b as *const Fe) };
            let eq_var = unsafe { fe_equal_var(a as *const Fe, b as *const Fe) };
            assert_eq!(
                eq, eq_var,
                "fe_equal and fe_equal_var must agree for case {idx}"
            );

            let eq_sym = unsafe { fe_equal(b as *const Fe, a as *const Fe) };
            assert_eq!(
                eq, eq_sym,
                "fe_equal must be symmetric for case {idx}"
            );

            let cmp = unsafe { fe_cmp_var(a as *const Fe, b as *const Fe) };
            if *expect_equal {
                assert_eq!(eq, 1, "expected equality for case {idx}");
                assert_eq!(cmp, 0, "cmp must be 0 for equal values in case {idx}");
            } else {
                assert_eq!(eq, 0, "expected inequality for case {idx}");
                assert_ne!(cmp, 0, "cmp must be nonzero for unequal values in case {idx}");
            }
        }

        trace!("testing comparison sign behavior on obvious orderings");
        let cmp_1_2 = unsafe { fe_cmp_var(&one as *const Fe, &two as *const Fe) };
        let cmp_2_1 = unsafe { fe_cmp_var(&two as *const Fe, &one as *const Fe) };
        assert!(cmp_1_2 < 0, "cmp(1,2) must be negative");
        assert!(cmp_2_1 > 0, "cmp(2,1) must be positive");

        let cmp_0_pm1 = unsafe { fe_cmp_var(&zero as *const Fe, &pm1 as *const Fe) };
        let cmp_pm1_0 = unsafe { fe_cmp_var(&pm1 as *const Fe, &zero as *const Fe) };
        assert!(cmp_0_pm1 < 0, "cmp(0,p-1) must be negative");
        assert!(cmp_pm1_0 > 0, "cmp(p-1,0) must be positive");
    }

    #[traced_test]
    fn field_element_sqrt_reports_residuosity_and_returns_correct_root_relation() {
        info!("validating fe_sqrt for quadratic residues and a known non-residue (-1)");

        let residue_squares: [i32; 7] = [0, 1, 4, 9, 25, 81, 256];

        for &square in residue_squares.iter() {
            trace!(square, "testing sqrt on an obvious square");

            let a = field_element_from_small_int(square);
            let mut r = field_element_from_small_int(0);

            let ok = unsafe { fe_sqrt(&mut r as *mut Fe, &a as *const Fe) };
            if ok != 1 {
                error!(square, ok, "fe_sqrt unexpectedly reported non-residue for a square");
            }
            assert_eq!(ok, 1, "fe_sqrt must return 1 for quadratic residues");

            let mut r2 = field_element_from_small_int(0);
            unsafe {
                fe_sqr(&mut r2 as *mut Fe, &r as *const Fe);
                fe_normalize(&mut r2 as *mut Fe);
            }

            assert_eq!(
                direct_b32_from_normalized_field_element(&r2),
                direct_b32_from_normalized_field_element(&a),
                "r^2 must equal a for residue input {square}"
            );
        }

        trace!("testing sqrt on a known non-residue when p ≡ 3 (mod 4): a = -1 = p-1");
        let a_non_residue = field_element_from_b32_expect_success("p-1 (=-1)", &FIELD_MODULUS_P_MINUS_1);
        let mut r = field_element_from_small_int(0);

        let ok = unsafe { fe_sqrt(&mut r as *mut Fe, &a_non_residue as *const Fe) };
        assert_eq!(
            ok, 0,
            "fe_sqrt must return 0 for a non-residue and compute sqrt(-a)"
        );

        let mut r2 = field_element_from_small_int(0);
        unsafe {
            fe_sqr(&mut r2 as *mut Fe, &r as *const Fe);
            fe_normalize(&mut r2 as *mut Fe);
        }

        assert_eq!(
            direct_b32_from_normalized_field_element(&r2),
            B32_ONE,
            "for a=-1, sqrt must return r with r^2 == -a == 1"
        );
    }

}
