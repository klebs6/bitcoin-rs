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

lazy_static!{
    /*
    #if defined(WIDEMUL_INT128)
    #include "field_5x52.h"
    #elif defined(WIDEMUL_INT64)
    #include "field_10x26.h"
    #else
    #error "Please select wide multiplication implementation"
    #endif
    */
}

/**
  | Normalize a field element. This brings
  | the field element to a canonical representation,
  | reduces its magnitude to 1, and reduces
  | it modulo field size `p`.
  |
  */
pub fn fe_normalize(r: *mut Fe)  {
    
    todo!();
        /*
        
        */
}

/**
  | Weakly normalize a field element: reduce
  | its magnitude to 1, but don't fully normalize.
  |
  */
pub fn fe_normalize_weak(r: *mut Fe)  {
    
    todo!();
        /*
        
        */
}

/**
  | Normalize a field element, without
  | constant-time guarantee.
  |
  */
pub fn fe_normalize_var(r: *mut Fe)  {
    
    todo!();
        /*
        
        */
}

/**
  | Verify whether a field element represents
  | zero i.e. would normalize to a zero value.
  |
  */
pub fn fe_normalizes_to_zero(r: *const Fe) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Verify whether a field element represents
  | zero i.e. would normalize to a zero value,
  | without constant-time guarantee.
  |
  */
pub fn fe_normalizes_to_zero_var(r: *const Fe) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Set a field element equal to a small integer.
  | Resulting field element is normalized.
  |
  */
pub fn fe_set_int(
        r: *mut Fe,
        a: i32)  {
    
    todo!();
        /*
        
        */
}

/**
  | Sets a field element equal to zero, initializing
  | all fields.
  |
  */
pub fn fe_clear(a: *mut Fe)  {
    
    todo!();
        /*
        
        */
}

/**
  | Verify whether a field element is zero.
  | Requires the input to be normalized.
  |
  */
pub fn fe_is_zero(a: *const Fe) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Check the "oddness" of a field element.
  | Requires the input to be normalized.
  |
  */
pub fn fe_is_odd(a: *const Fe) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Compare two field elements. Requires
  | both inputs to be normalized
  |
  */
pub fn fe_cmp_var(
        a: *const Fe,
        b: *const Fe) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Set a field element equal to 32-byte
  | big endian value. If successful, the
  | resulting field element is normalized.
  |
  */
pub fn fe_set_b32(
        r: *mut Fe,
        a: *const u8) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Convert a field element to a 32-byte
  | big endian value. Requires the input
  | to be normalized
  |
  */
pub fn fe_get_b32(
        r: *mut u8,
        a: *const Fe)  {
    
    todo!();
        /*
        
        */
}

/**
  | Set a field element equal to the additive
  | inverse of another. Takes a maximum
  | magnitude of the input as an argument.
  | The magnitude of the output is one higher.
  |
  */
pub fn fe_negate(
        r: *mut Fe,
        a: *const Fe,
        m: i32)  {
    
    todo!();
        /*
        
        */
}

/**
  | Multiplies the passed field element
  | with a small integer constant. Multiplies
  | the magnitude by that small integer.
  |
  */
pub fn fe_mul_int(
        r: *mut Fe,
        a: i32)  {
    
    todo!();
        /*
        
        */
}

/**
  | Adds a field element to another. The
  | result has the sum of the inputs' magnitudes
  | as magnitude.
  |
  */
pub fn fe_add(
        r: *mut Fe,
        a: *const Fe)  {
    
    todo!();
        /*
        
        */
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
pub fn fe_mul(
        r: *mut Fe,
        a: *const Fe,
        b: *const Fe)  {
    
    todo!();
        /*
        
        */
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
pub fn fe_sqr(
        r: *mut Fe,
        a: *const Fe)  {
    
    todo!();
        /*
        
        */
}

/**
  | Sets a field element to be the (modular)
  | inverse of another. Requires the input's
  | magnitude to be at most 8. The output
  | magnitude is 1 (but not guaranteed to
  | be normalized).
  |
  */
pub fn fe_inv(
        r: *mut Fe,
        a: *const Fe)  {
    
    todo!();
        /*
        
        */
}

/**
  | Potentially faster version of fe_inv,
  | without constant-time guarantee.
  |
  */
pub fn fe_inv_var(
        r: *mut Fe,
        a: *const Fe)  {
    
    todo!();
        /*
        
        */
}

/**
  | Convert a field element to the storage
  | type.
  |
  */
pub fn fe_to_storage(
        r: *mut FeStorage,
        a: *const Fe)  {
    
    todo!();
        /*
        
        */
}

/**
  | Convert a field element back from the
  | storage type.
  |
  */
pub fn fe_from_storage(
        r: *mut Fe,
        a: *const FeStorage)  {
    
    todo!();
        /*
        
        */
}

/**
  | If flag is true, set *r equal to *a; otherwise
  | leave it. Constant-time. Both *r and
  | *a must be initialized.
  |
  */
pub fn fe_storage_cmov(
        r:    *mut FeStorage,
        a:    *const FeStorage,
        flag: i32)  {
    
    todo!();
        /*
        
        */
}

/**
  | If flag is true, set *r equal to *a; otherwise
  | leave it. Constant-time. Both *r and
  | *a must be initialized.
  |
  */
pub fn fe_cmov(
        r:    *mut Fe,
        a:    *const Fe,
        flag: i32)  {
    
    todo!();
        /*
        
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/field_impl.h]

lazy_static!{
    /*
    #if defined(WIDEMUL_INT128)
    #include "field_5x52_impl.h"
    #elif defined(WIDEMUL_INT64)
    #include "field_10x26_impl.h"
    #else
    #error "Please select wide multiplication implementation"
    #endif
    */
}

/**
  | Compare two field elements. Requires
  | magnitude-1 inputs.
  |
  */
#[inline] pub fn fe_equal(
        a: *const Fe,
        b: *const Fe) -> i32 {
    
    todo!();
        /*
            Fe na;
        fe_negate(&na, a, 1);
        fe_add(&na, b);
        return fe_normalizes_to_zero(&na);
        */
}

/**
  | Same as fe_equal, but may be variable
  | time.
  |
  */
#[inline] pub fn fe_equal_var(
        a: *const Fe,
        b: *const Fe) -> i32 {
    
    todo!();
        /*
            Fe na;
        fe_negate(&na, a, 1);
        fe_add(&na, b);
        return fe_normalizes_to_zero_var(&na);
        */
}

/**
  | If a has a square root, it is computed
  | in r and 1 is returned. If a does not have
  | a square root, the root of its negation
  | is computed and 0 is returned.
  | 
  | The input's magnitude can be at most
  | 8. The output magnitude is 1 (but not
  | guaranteed to be normalized). The result
  | in r will always be a square itself.
  |
  */
pub fn fe_sqrt(
        r: *mut Fe,
        a: *const Fe) -> i32 {
    
    todo!();
        /*
            /** Given that p is congruent to 3 mod 4, we can compute the square root of
         *  a mod p as the (p+1)/4'th power of a.
         *
         *  As (p+1)/4 is an even number, it will have the same result for a and for
         *  (-a). Only one of these two numbers actually has a square root however,
         *  so we test at the end by squaring and comparing to the input.
         *  Also because (p+1)/4 is an even number, the computed square root is
         *  itself always a square (a ** ((p+1)/4) is the square of a ** ((p+1)/8)).
         */
        Fe x2, x3, x6, x9, x11, x22, x44, x88, x176, x220, x223, t1;
        int j;

        VERIFY_CHECK(r != a);

        /** The binary representation of (p + 1)/4 has 3 blocks of 1s, with lengths in
         *  { 2, 22, 223 }. Use an addition chain to calculate 2^n - 1 for each block:
         *  1, [2], 3, 6, 9, 11, [22], 44, 88, 176, 220, [223]
         */

        fe_sqr(&x2, a);
        fe_mul(&x2, &x2, a);

        fe_sqr(&x3, &x2);
        fe_mul(&x3, &x3, a);

        x6 = x3;
        for (j=0; j<3; j++) {
            fe_sqr(&x6, &x6);
        }
        fe_mul(&x6, &x6, &x3);

        x9 = x6;
        for (j=0; j<3; j++) {
            fe_sqr(&x9, &x9);
        }
        fe_mul(&x9, &x9, &x3);

        x11 = x9;
        for (j=0; j<2; j++) {
            fe_sqr(&x11, &x11);
        }
        fe_mul(&x11, &x11, &x2);

        x22 = x11;
        for (j=0; j<11; j++) {
            fe_sqr(&x22, &x22);
        }
        fe_mul(&x22, &x22, &x11);

        x44 = x22;
        for (j=0; j<22; j++) {
            fe_sqr(&x44, &x44);
        }
        fe_mul(&x44, &x44, &x22);

        x88 = x44;
        for (j=0; j<44; j++) {
            fe_sqr(&x88, &x88);
        }
        fe_mul(&x88, &x88, &x44);

        x176 = x88;
        for (j=0; j<88; j++) {
            fe_sqr(&x176, &x176);
        }
        fe_mul(&x176, &x176, &x88);

        x220 = x176;
        for (j=0; j<44; j++) {
            fe_sqr(&x220, &x220);
        }
        fe_mul(&x220, &x220, &x44);

        x223 = x220;
        for (j=0; j<3; j++) {
            fe_sqr(&x223, &x223);
        }
        fe_mul(&x223, &x223, &x3);

        /* The final result is then assembled using a sliding window over the blocks. */

        t1 = x223;
        for (j=0; j<23; j++) {
            fe_sqr(&t1, &t1);
        }
        fe_mul(&t1, &t1, &x22);
        for (j=0; j<6; j++) {
            fe_sqr(&t1, &t1);
        }
        fe_mul(&t1, &t1, &x2);
        fe_sqr(&t1, &t1);
        fe_sqr(r, &t1);

        /* Check that a square root was actually calculated */

        fe_sqr(&t1, r);
        return fe_equal(&t1, a);
        */
}

pub const FE_ONE: Fe = fe_const!(0, 0, 0, 0, 0, 0, 0, 1);
