// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_split_lambda_verify.rs ]
crate::ix!();

#[cfg(any())]
#[cfg(not(feature="exhaustive-test-order"))]
#[cfg(feature="secp256k1-verify")]
pub fn scalar_split_lambda_verify(_r1: *const Scalar, _r2: *const Scalar, _k: *const Scalar) {}

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
#[cfg(feature="secp256k1-verify")]
#[cfg(not(feature="exhaustive-test-order"))]
pub fn scalar_split_lambda_verify(r1: *const Scalar, r2: *const Scalar, k: *const Scalar) {
    unsafe {
        let mut s: Scalar = Scalar::new();
        let mut buf1: [u8; 32] = [0u8; 32];
        let mut buf2: [u8; 32] = [0u8; 32];

        /* (a1 + a2 + 1)/2 is 0xa2a8918ca85bafe22016d0b917e4dd77 */
        const k1_bound: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xa2, 0xa8, 0x91, 0x8c, 0xa8, 0x5b, 0xaf, 0xe2, 0x20, 0x16, 0xd0, 0xb9, 0x17, 0xe4, 0xdd, 0x77
        ];

        /* (-b1 + b2)/2 + 1 is 0x8a65287bd47179fb2be08846cea267ed */
        const k2_bound: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x8a, 0x65, 0x28, 0x7b, 0xd4, 0x71, 0x79, 0xfb, 0x2b, 0xe0, 0x88, 0x46, 0xce, 0xa2, 0x67, 0xed
        ];

        scalar_mul(&mut s, &*const_lambda, r2);
        scalar_add(&mut s, &s, r1);
        verify_check!(scalar_eq(&s, k) != 0);

        scalar_negate(&mut s, r1);
        scalar_get_b32(buf1.as_mut_ptr(), r1);
        scalar_get_b32(buf2.as_mut_ptr(), &s);
        verify_check!(
            memcmp_var(
                buf1.as_ptr().cast::<libc::c_void>(),
                k1_bound.as_ptr().cast::<libc::c_void>(),
                32
            ) < 0
                || memcmp_var(
                    buf2.as_ptr().cast::<libc::c_void>(),
                    k1_bound.as_ptr().cast::<libc::c_void>(),
                    32
                ) < 0
        );

        scalar_negate(&mut s, r2);
        scalar_get_b32(buf1.as_mut_ptr(), r2);
        scalar_get_b32(buf2.as_mut_ptr(), &s);
        verify_check!(
            memcmp_var(
                buf1.as_ptr().cast::<libc::c_void>(),
                k2_bound.as_ptr().cast::<libc::c_void>(),
                32
            ) < 0
                || memcmp_var(
                    buf2.as_ptr().cast::<libc::c_void>(),
                    k2_bound.as_ptr().cast::<libc::c_void>(),
                    32
                ) < 0
        );
    }
}

#[cfg(test)]
#[cfg(all(feature = "secp256k1-verify", not(feature = "exhaustive-test-order")))]
mod scalar_split_lambda_verify_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_split_lambda_verify_accepts_outputs_from_scalar_split_lambda() {
        info!("validating scalar_split_lambda_verify passes for outputs produced by scalar_split_lambda");

        let vectors: &[[u8; 32]] = &[
            SCALAR_ZERO_BE,
            SCALAR_ONE_BE,
            SCALAR_TWO_BE,
            SCALAR_THREE_BE,
            SCALAR_MAX_U32_BE,
            SECP256K1_ORDER_HALF_BE,
            SECP256K1_ORDER_MINUS_1_BE,
        ];

        for (i, k_be) in vectors.iter().enumerate() {
            let mut k = scalar_from_be_bytes(k_be);
            let mut r1 = scalar_zero_value();
            let mut r2 = scalar_zero_value();

            unsafe {
                scalar_split_lambda(
                    &mut r1 as *mut Scalar,
                    &mut r2 as *mut Scalar,
                    &mut k as *mut Scalar,
                );

                debug!(i, "calling scalar_split_lambda_verify");
                scalar_split_lambda_verify(
                    &r1 as *const Scalar,
                    &r2 as *const Scalar,
                    &k as *const Scalar,
                );
            }
        }
    }
}
