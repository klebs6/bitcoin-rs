// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_split_lambda_verify.rs ]
crate::ix!();

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
