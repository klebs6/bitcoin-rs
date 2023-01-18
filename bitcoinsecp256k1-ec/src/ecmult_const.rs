crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_const.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_const_impl.h]

/**
  | This is like `ECMULT_TABLE_GET_GE`
  | but is constant time
  |
  */
macro_rules! ecmult_const_table_get_ge {
    ($r:ident, 
     $pre:ident, 
     $n:ident, 
     $w:ident) => {
        /*
                do { 
            int m = 0; 
            /* Extract the sign-bit for a constant time absolute-value. */ 
            int mask = (n) >> (sizeof(n) * CHAR_BIT - 1); 
            int abs_n = ((n) + mask) ^ mask; 
            int idx_n = abs_n >> 1; 
            fe neg_y; 
            VERIFY_CHECK(((n) & 1) == 1); 
            VERIFY_CHECK((n) >= -((1 << ((w)-1)) - 1)); 
            VERIFY_CHECK((n) <=  ((1 << ((w)-1)) - 1)); 
            VERIFY_SETUP(fe_clear(&(r)->x)); 
            VERIFY_SETUP(fe_clear(&(r)->y)); 
            /* Unconditionally set r->x = (pre)[m].x. r->y = (pre)[m].y. because it's either the correct one 
             * or will get replaced in the later iterations, this is needed to make sure `r` is initialized. */ 
            (r)->x = (pre)[m].x; 
            (r)->y = (pre)[m].y; 
            for (m = 1; m < ECMULT_TABLE_SIZE(w); m++) { 
                /* This loop is used to avoid secret data in array indices. See
                 * the comment in ecmult_gen_impl.h for rationale. */ 
                fe_cmov(&(r)->x, &(pre)[m].x, m == idx_n); 
                fe_cmov(&(r)->y, &(pre)[m].y, m == idx_n); 
            } 
            (r)->infinity = 0; 
            fe_negate(&neg_y, &(r)->y, 1); 
            fe_cmov(&(r)->y, &neg_y, (n) != abs_n); 
        } while(0)
        */
    }
}

/** 
 | Convert a number to WNAF notation.
 |
 |  The number becomes represented by 
 |  sum(2^{wi} * wnaf[i], i=0..WNAF_SIZE(w)+1) - return_val.
 |
 |  It has the following guarantees:
 |
 |  - each wnaf[i] an odd integer 
 |    between -(1 << w) and (1 << w)
 |
 |  - each wnaf[i] is nonzero
 |
 |  - the number of words set is always
 |    WNAF_SIZE(w) + 1
 |
 |  Adapted from `The Width-w NAF Method Provides
 |  Small Memory and Fast Elliptic Scalar
 |  Multiplications Secure against Side Channel
 |  Attacks`, Okeya and Tagaki. M. Joye (Ed.)
 |  CT-RSA 2003, LNCS 2612, pp. 328-443,
 |  2003. Springer-Verlag Berlin Heidelberg 2003
 |
 |  Numbers reference steps of `Algorithm
 |  SPA-resistant Width-w NAF with Odd Scalar` on
 |  pp. 335
 */
pub fn wnaf_const(
        wnaf:   *mut i32,
        scalar: *const Scalar,
        w:      i32,
        size:   i32) -> i32 {
    
    todo!();
        /*
            int global_sign;
        int skew = 0;
        int word = 0;

        /* 1 2 3 */
        int u_last;
        int u;

        int flip;
        int bit;
        scalar s;
        int not_neg_one;

        VERIFY_CHECK(w > 0);
        VERIFY_CHECK(size > 0);

        /* Note that we cannot handle even numbers by negating them to be odd, as is
         * done in other implementations, since if our scalars were specified to have
         * width < 256 for performance reasons, their negations would have width 256
         * and we'd lose any performance benefit. Instead, we use a technique from
         * Section 4.2 of the Okeya/Tagaki paper, which is to add either 1 (for even)
         * or 2 (for odd) to the number we are encoding, returning a skew value indicating
         * this, and having the caller compensate after doing the multiplication.
         *
         * In fact, we _do_ want to negate numbers to minimize their bit-lengths (and in
         * particular, to ensure that the outputs from the endomorphism-split fit into
         * 128 bits). If we negate, the parity of our number flips, inverting which of
         * {1, 2} we want to add to the scalar when ensuring that it's odd. Further
         * complicating things, -1 interacts badly with `scalar_cadd_bit` and
         * we need to special-case it in this logic. */
        flip = scalar_is_high(scalar);
        /* We add 1 to even numbers, 2 to odd ones, noting that negation flips parity */
        bit = flip ^ !scalar_is_even(scalar);
        /* We check for negative one, since adding 2 to it will cause an overflow */
        scalar_negate(&s, scalar);
        not_neg_one = !scalar_is_one(&s);
        s = *scalar;
        scalar_cadd_bit(&s, bit, not_neg_one);
        /* If we had negative one, flip == 1, s.d[0] == 0, bit == 1, so caller expects
         * that we added two to it and flipped it. In fact for -1 these operations are
         * identical. We only flipped, but since skewing is required (in the sense that
         * the skew must be 1 or 2, never zero) and flipping is not, we need to change
         * our flags to claim that we only skewed. */
        global_sign = scalar_cond_negate(&s, flip);
        global_sign *= not_neg_one * 2 - 1;
        skew = 1 << bit;

        /* 4 */
        u_last = scalar_shr_int(&s, w);
        do {
            int even;

            /* 4.1 4.4 */
            u = scalar_shr_int(&s, w);
            /* 4.2 */
            even = ((u & 1) == 0);
            /* In contrast to the original algorithm, u_last is always > 0 and
             * therefore we do not need to check its sign. In particular, it's easy
             * to see that u_last is never < 0 because u is never < 0. Moreover,
             * u_last is never = 0 because u is never even after a loop
             * iteration. The same holds analogously for the initial value of
             * u_last (in the first loop iteration). */
            VERIFY_CHECK(u_last > 0);
            VERIFY_CHECK((u_last & 1) == 1);
            u += even;
            u_last -= even * (1 << w);

            /* 4.3, adapted for global sign change */
            wnaf[word++] = u_last * global_sign;

            u_last = u;
        } while (word * w < size);
        wnaf[word] = u * global_sign;

        VERIFY_CHECK(scalar_is_zero(&s));
        VERIFY_CHECK(word == WNAF_SIZE_BITS(size, w));
        return skew;
        */
}

/**
  | Multiply: R = q*A (in constant-time)
  | 
  | Here `bits` should be set to the maximum
  | bitlength of the _absolute value_ of
  | `q`, plus one because we internally
  | sometimes add 2 to the number during
  | the WNAF conversion.
  |
  */
pub fn ecmult_const(
        r:      *mut Gej,
        a:      *const Ge,
        scalar: *const Scalar,
        size:   i32)  {
    
    todo!();
        /*
        ge pre_a[ECMULT_TABLE_SIZE(WINDOW_A)];
        ge tmpa;
        fe Z;

        int skew_1;
        ge pre_a_lam[ECMULT_TABLE_SIZE(WINDOW_A)];
        int wnaf_lam[1 + WNAF_SIZE(WINDOW_A - 1)];
        int skew_lam;
        scalar q_1, q_lam;
        int wnaf_1[1 + WNAF_SIZE(WINDOW_A - 1)];

        int i;

        /* build wnaf representation for q. */
        int rsize = size;
        if (size > 128) {
            rsize = 128;
            /* split q into q_1 and q_lam (where q = q_1 + q_lam*lambda, and q_1 and q_lam are ~128 bit) */
            scalar_split_lambda(&q_1, &q_lam, scalar);
            skew_1   = wnaf_const(wnaf_1,   &q_1,   WINDOW_A - 1, 128);
            skew_lam = wnaf_const(wnaf_lam, &q_lam, WINDOW_A - 1, 128);
        } else
        {
            skew_1   = wnaf_const(wnaf_1, scalar, WINDOW_A - 1, size);
            skew_lam = 0;
        }

        /* Calculate odd multiples of a.
         * All multiples are brought to the same Z 'denominator', which is stored
         * in Z. Due to secp256k1' isomorphism we can do all operations pretending
         * that the Z coordinate was 1, use affine addition formulae, and correct
         * the Z coordinate of the result once at the end.
         */
        gej_set_ge(r, a);
        ecmult_odd_multiples_table_globalz_windowa(pre_a, &Z, r);
        for (i = 0; i < ECMULT_TABLE_SIZE(WINDOW_A); i++) {
            fe_normalize_weak(&pre_a[i].y);
        }
        if (size > 128) {
            for (i = 0; i < ECMULT_TABLE_SIZE(WINDOW_A); i++) {
                ge_mul_lambda(&pre_a_lam[i], &pre_a[i]);
            }

        }

        /* first loop iteration (separated out so we can directly set r, rather
         * than having it start at infinity, get doubled several times, then have
         * its new value added to it) */
        i = wnaf_1[WNAF_SIZE_BITS(rsize, WINDOW_A - 1)];
        VERIFY_CHECK(i != 0);
        ECMULT_CONST_TABLE_GET_GE(&tmpa, pre_a, i, WINDOW_A);
        gej_set_ge(r, &tmpa);
        if (size > 128) {
            i = wnaf_lam[WNAF_SIZE_BITS(rsize, WINDOW_A - 1)];
            VERIFY_CHECK(i != 0);
            ECMULT_CONST_TABLE_GET_GE(&tmpa, pre_a_lam, i, WINDOW_A);
            gej_add_ge(r, r, &tmpa);
        }
        /* remaining loop iterations */
        for (i = WNAF_SIZE_BITS(rsize, WINDOW_A - 1) - 1; i >= 0; i--) {
            int n;
            int j;
            for (j = 0; j < WINDOW_A - 1; ++j) {
                gej_double(r, r);
            }

            n = wnaf_1[i];
            ECMULT_CONST_TABLE_GET_GE(&tmpa, pre_a, n, WINDOW_A);
            VERIFY_CHECK(n != 0);
            gej_add_ge(r, r, &tmpa);
            if (size > 128) {
                n = wnaf_lam[i];
                ECMULT_CONST_TABLE_GET_GE(&tmpa, pre_a_lam, n, WINDOW_A);
                VERIFY_CHECK(n != 0);
                gej_add_ge(r, r, &tmpa);
            }
        }

        fe_mul(&r->z, &r->z, &Z);

        {
            /* Correct for wNAF skew */
            ge correction = *a;
            ge_storage correction_1_stor;
            ge_storage correction_lam_stor;
            ge_storage a2_stor;
            gej tmpj;
            gej_set_ge(&tmpj, &correction);
            gej_double_var(&tmpj, &tmpj, NULL);
            ge_set_gej(&correction, &tmpj);
            ge_to_storage(&correction_1_stor, a);
            if (size > 128) {
                ge_to_storage(&correction_lam_stor, a);
            }
            ge_to_storage(&a2_stor, &correction);

            /* For odd numbers this is 2a (so replace it), for even ones a (so no-op) */
            ge_storage_cmov(&correction_1_stor, &a2_stor, skew_1 == 2);
            if (size > 128) {
                ge_storage_cmov(&correction_lam_stor, &a2_stor, skew_lam == 2);
            }

            /* Apply the correction */
            ge_from_storage(&correction, &correction_1_stor);
            ge_neg(&correction, &correction);
            gej_add_ge(r, r, &correction);

            if (size > 128) {
                ge_from_storage(&correction, &correction_lam_stor);
                ge_neg(&correction, &correction);
                ge_mul_lambda(&correction, &correction);
                gej_add_ge(r, r, &correction);
            }
        }
        */
}
