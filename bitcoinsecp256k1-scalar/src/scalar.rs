crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar.h]

lazy_static!{
    /*
    #if defined(EXHAUSTIVE_TEST_ORDER)
    #include "scalar_low.h"
    #elif defined(WIDEMUL_INT128)
    #include "scalar_4x64.h"
    #elif defined(WIDEMUL_INT64)
    #include "scalar_8x32.h"
    #else
    #error "Please select wide multiplication implementation"
    #endif
    */
}

/**
  | Clear a scalar to prevent the leak of
  | sensitive data.
  |
  */
pub fn scalar_clear(r: *mut Scalar)  {
    
    todo!();
        /*
        
        */
}

/**
  | Access bits from a scalar. All requested
  | bits must belong to the same 32-bit limb.
  |
  */
pub fn scalar_get_bits(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
        
        */
}

/**
  | Access bits from a scalar. Not constant
  | time.
  |
  */
pub fn scalar_get_bits_var(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
        
        */
}

/** 
 | Set a scalar from a big endian byte array. The
 | scalar will be reduced modulo group order `n`.
 |
 | In:      bin:        pointer to a 32-byte array.
 |
 | Out:     r:          scalar to be set.
 |
 |          overflow:   non-zero if the scalar was
 |          bigger or equal to `n` before
 |          reduction, zero otherwise (can be
 |          NULL).
 */
pub fn scalar_set_b32(
        r:        *mut Scalar,
        bin:      *const u8,
        overflow: *mut i32)  {
    
    todo!();
        /*
        
        */
}

/**
  | Set a scalar to an unsigned integer.
  |
  */
pub fn scalar_set_int(
        r: *mut Scalar,
        v: u32)  {
    
    todo!();
        /*
        
        */
}

/**
  | Convert a scalar to a byte array.
  |
  */
pub fn scalar_get_b32(
        bin: *mut u8,
        a:   *const Scalar)  {
    
    todo!();
        /*
        
        */
}

/**
  | Add two scalars together (modulo the
  | group order). Returns whether it overflowed.
  |
  */
pub fn scalar_add(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Conditionally add a power of two to a
  | scalar. The result is not allowed to
  | overflow.
  |
  */
pub fn scalar_cadd_bit(
        r:    *mut Scalar,
        bit:  u32,
        flag: i32)  {
    
    todo!();
        /*
        
        */
}

/**
  | Multiply two scalars (modulo the group
  | order).
  |
  */
pub fn scalar_mul(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar)  {
    
    todo!();
        /*
        
        */
}

/**
  | Shift a scalar right by some amount strictly
  | between 0 and 16, returning the low bits
  | that were shifted off
  |
  */
pub fn scalar_shr_int(
        r: *mut Scalar,
        n: i32) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Compute the inverse of a scalar (modulo
  | the group order).
  |
  */
pub fn scalar_inverse(
        r: *mut Scalar,
        a: *const Scalar)  {
    
    todo!();
        /*
        
        */
}

/**
  | Compute the inverse of a scalar (modulo
  | the group order), without constant-time
  | guarantee.
  |
  */
pub fn scalar_inverse_var(
        r: *mut Scalar,
        a: *const Scalar)  {
    
    todo!();
        /*
        
        */
}

/**
  | Compute the complement of a scalar (modulo
  | the group order).
  |
  */
pub fn scalar_negate(
        r: *mut Scalar,
        a: *const Scalar)  {
    
    todo!();
        /*
        
        */
}

/**
  | Check whether a scalar equals zero.
  |
  */
pub fn scalar_is_zero(a: *const Scalar) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Check whether a scalar equals one.
  |
  */
pub fn scalar_is_one(a: *const Scalar) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Check whether a scalar, considered
  | as an nonnegative integer, is even.
  |
  */
pub fn scalar_is_even(a: *const Scalar) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Check whether a scalar is higher than
  | the group order divided by 2.
  |
  */
pub fn scalar_is_high(a: *const Scalar) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Conditionally negate a number, in constant
  | time.
  | 
  | Returns -1 if the number was negated,
  | 1 otherwise
  |
  */
pub fn scalar_cond_negate(
        a:    *mut Scalar,
        flag: i32) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Compare two scalars.
  |
  */
pub fn scalar_eq(
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
        
        */
}

/**
  | Find r1 and r2 such that r1+r2*2^128
  | = k.
  |
  */
pub fn scalar_split_128(
        r1: *mut Scalar,
        r2: *mut Scalar,
        k:  *const Scalar)  {
    
    todo!();
        /*
        
        */
}

/**
  | Multiply a and b (without taking the
  | modulus!), divide by 2**shift, and
  | round to the nearest integer. Shift
  | must be at least 256.
  |
  */
pub fn scalar_mul_shift_var(
        r:     *mut Scalar,
        a:     *const Scalar,
        b:     *const Scalar,
        shift: u32)  {
    
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
pub fn scalar_cmov(
        r:    *mut Scalar,
        a:    *const Scalar,
        flag: i32)  {
    
    todo!();
        /*
        
        */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_impl.h]

lazy_static!{
    /*
    #if defined(EXHAUSTIVE_TEST_ORDER)
    #include "scalar_low_impl.h"
    #elif defined(WIDEMUL_INT128)
    #include "scalar_4x64_impl.h"
    #elif defined(WIDEMUL_INT64)
    #include "scalar_8x32_impl.h"
    #else
    #error "Please select wide multiplication implementation"
    #endif
    */
}

pub const SCALAR_ONE:  Scalar = scalar_const!(0, 0, 0, 0, 0, 0, 0, 1);
pub const SCALAR_ZERO: Scalar = scalar_const!(0, 0, 0, 0, 0, 0, 0, 0);

/**
  | Set a scalar from a big endian byte array
  | and returns 1 if it is a valid seckey and
  | 0 otherwise.
  |
  */
pub fn scalar_set_b32_seckey(
        r:   *mut Scalar,
        bin: *const u8) -> i32 {
    
    todo!();
        /*
            int overflow;
        scalar_set_b32(r, bin, &overflow);
        return (!overflow) & (!scalar_is_zero(r));
        */
}

/**
  | These parameters are generated using
  | sage/gen_exhaustive_groups.sage.
  |
  */
#[cfg(EXHAUSTIVE_TEST_ORDER)]
lazy_static!{
    /*
    #  if EXHAUSTIVE_TEST_ORDER == 13
    #    define EXHAUSTIVE_TEST_LAMBDA 9
    #  elif EXHAUSTIVE_TEST_ORDER == 199
    #    define EXHAUSTIVE_TEST_LAMBDA 92
    #  else
    #    error No known lambda for the specified exhaustive test group order.
    #  endif
    */
}

/**
  | The curve has an endomorphism, where
  | lambda * (x, y) = (beta * x, y), where lambda
  | is:
  |
  */
#[cfg(not(EXHAUSTIVE_TEST_ORDER))]
lazy_static!{
    /*
    static const scalar const_lambda = SCALAR_CONST(
        0x5363AD4CUL, 0xC05C30E0UL, 0xA5261C02UL, 0x8812645AUL,
        0x122E22EAUL, 0x20816678UL, 0xDF02967CUL, 0x1B23BD72UL
    );
    */
}

#[cfg(not(EXHAUSTIVE_TEST_ORDER))]
#[cfg(VERIFY)]
pub fn scalar_split_lambda_verify(
        r1: *const Scalar,
        r2: *const Scalar,
        k:  *const Scalar)  {
    
    todo!();
        /*
        
        */
}

/**
 | Proof for scalar_split_lambda's bounds.
 |
 | Let
 |  - epsilon1 = 2^256 * |g1/2^384 - b2/d|
 |  - epsilon2 = 2^256 * |g2/2^384 - (-b1)/d|
 |  - c1 = round(k*g1/2^384)
 |  - c2 = round(k*g2/2^384)
 |
 | Lemma 1: |c1 - k*b2/d| < 2^-1 + epsilon1
 |
 |    |c1 - k*b2/d|
 |  =
 |    |c1 - k*g1/2^384 + k*g1/2^384 - k*b2/d|
 | <=   {triangle inequality}
 |    |c1 - k*g1/2^384| + |k*g1/2^384 - k*b2/d|
 |  =
 |    |c1 - k*g1/2^384| + k*|g1/2^384 - b2/d|
 | <    {rounding in c1 and 0 <= k < 2^256}
 |    2^-1 + 2^256 * |g1/2^384 - b2/d|
 |  =   {definition of epsilon1}
 |    2^-1 + epsilon1
 |
 | Lemma 2: |c2 - k*(-b1)/d| < 2^-1 + epsilon2
 |
 |    |c2 - k*(-b1)/d|
 |  =
 |    |c2 - k*g2/2^384 + k*g2/2^384 - k*(-b1)/d|
 | <=   {triangle inequality}
 |    |c2 - k*g2/2^384| + |k*g2/2^384 - k*(-b1)/d|
 |  =
 |    |c2 - k*g2/2^384| + k*|g2/2^384 - (-b1)/d|
 | <    {rounding in c2 and 0 <= k < 2^256}
 |    2^-1 + 2^256 * |g2/2^384 - (-b1)/d|
 |  =   {definition of epsilon2}
 |    2^-1 + epsilon2
 |
 | Let
 |  - k1 = k - c1*a1 - c2*a2
 |  - k2 = - c1*b1 - c2*b2
 |
 | Lemma 3: |k1| < (a1 + a2 + 1)/2 < 2^128
 |
 |    |k1|
 |  =   {definition of k1}
 |    |k - c1*a1 - c2*a2|
 |  =   {(a1*b2 - b1*a2)/n = 1}
 |    |k*(a1*b2 - b1*a2)/n - c1*a1 - c2*a2|
 |  =
 |    |a1*(k*b2/n - c1) + a2*(k*(-b1)/n - c2)|
 | <=   {triangle inequality}
 |    a1*|k*b2/n - c1| + a2*|k*(-b1)/n - c2|
 | <    {Lemma 1 and Lemma 2}
 |    a1*(2^-1 + epslion1) + a2*(2^-1 + epsilon2)
 | <    {rounding up to an integer}
 |    (a1 + a2 + 1)/2
 | <    {rounding up to a power of 2}
 |    2^128
 |
 | Lemma 4: |k2| < (-b1 + b2)/2 + 1 < 2^128
 |
 |    |k2|
 |  =   {definition of k2}
 |    |- c1*a1 - c2*a2|
 |  =   {(b1*b2 - b1*b2)/n = 0}
 |    |k*(b1*b2 - b1*b2)/n - c1*b1 - c2*b2|
 |  =
 |    |b1*(k*b2/n - c1) + b2*(k*(-b1)/n - c2)|
 | <=   {triangle inequality}
 |    (-b1)*|k*b2/n - c1| + b2*|k*(-b1)/n - c2|
 | <    {Lemma 1 and Lemma 2}
 |    (-b1)*(2^-1 + epslion1) + b2*(2^-1 + epsilon2)
 | <    {rounding up to an integer}
 |    (-b1 + b2)/2 + 1
 | <    {rounding up to a power of 2}
 |    2^128
 |
 | Let
 |  - r2 = k2 mod n
 |  - r1 = k - r2*lambda mod n.
 |
 | Notice that r1 is defined such that r1 + r2 * lambda == k (mod n).
 |
 | Lemma 5: r1 == k1 mod n.
 |
 |    r1
 | ==   {definition of r1 and r2}
 |    k - k2*lambda
 | ==   {definition of k2}
 |    k - (- c1*b1 - c2*b2)*lambda
 | ==
 |    k + c1*b1*lambda + c2*b2*lambda
 | ==  {a1 + b1*lambda == 0 mod n and a2 + b2*lambda == 0 mod n}
 |    k - c1*a1 - c2*a2
 | ==  {definition of k1}
 |    k1
 |
 | From Lemma 3, Lemma 4, Lemma 5 and the definition of r2, we can conclude that
 |
 |  - either r1 < 2^128 or -r1 mod n < 2^128
 |  - either r2 < 2^128 or -r2 mod n < 2^128.
 |
 | Q.E.D.
 */
#[cfg(VERIFY)]
#[cfg(not(EXHAUSTIVE_TEST_ORDER))]
pub fn scalar_split_lambda_verify(
        r1: *const Scalar,
        r2: *const Scalar,
        k:  *const Scalar)  {
    
    todo!();
        /*
            scalar s;
        unsigned char buf1[32];
        unsigned char buf2[32];

        /* (a1 + a2 + 1)/2 is 0xa2a8918ca85bafe22016d0b917e4dd77 */
        static const unsigned char k1_bound[32] = {
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xa2, 0xa8, 0x91, 0x8c, 0xa8, 0x5b, 0xaf, 0xe2, 0x20, 0x16, 0xd0, 0xb9, 0x17, 0xe4, 0xdd, 0x77
        };

        /* (-b1 + b2)/2 + 1 is 0x8a65287bd47179fb2be08846cea267ed */
        static const unsigned char k2_bound[32] = {
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x8a, 0x65, 0x28, 0x7b, 0xd4, 0x71, 0x79, 0xfb, 0x2b, 0xe0, 0x88, 0x46, 0xce, 0xa2, 0x67, 0xed
        };

        scalar_mul(&s, &const_lambda, r2);
        scalar_add(&s, &s, r1);
        VERIFY_CHECK(scalar_eq(&s, k));

        scalar_negate(&s, r1);
        scalar_get_b32(buf1, r1);
        scalar_get_b32(buf2, &s);
        VERIFY_CHECK(memcmp_var(buf1, k1_bound, 32) < 0 || memcmp_var(buf2, k1_bound, 32) < 0);

        scalar_negate(&s, r2);
        scalar_get_b32(buf1, r2);
        scalar_get_b32(buf2, &s);
        VERIFY_CHECK(memcmp_var(buf1, k2_bound, 32) < 0 || memcmp_var(buf2, k2_bound, 32) < 0);
        */
}
