crate::ix!();

pub fn mix_columns(
        s:   *mut AES_state,
        inv: i32)  {
    
    todo!();
        /*
            /* The MixColumns transform treats the bytes of the columns of the state as
         * coefficients of a 3rd degree polynomial over GF(2^8) and multiplies them
         * by the fixed polynomial a(x) = {03}x^3 + {01}x^2 + {01}x + {02}, modulo
         * x^4 + {01}.
         *
         * In the inverse transform, we multiply by the inverse of a(x),
         * a^-1(x) = {0b}x^3 + {0d}x^2 + {09}x + {0e}. This is equal to
         * a(x) * ({04}x^2 + {05}), so we can reuse the forward transform's code
         * (found in OpenSSL's bsaes-x86_64.pl, attributed to Jussi Kivilinna)
         *
         * In the bitsliced representation, a multiplication of every column by x
         * mod x^4 + 1 is simply a right rotation.
         */

        /* Shared for both directions is a multiplication by a(x), which can be
         * rewritten as (x^3 + x^2 + x) + {02}*(x^3 + {01}).
         *
         * First compute s into the s? variables, (x^3 + {01}) * s into the s?_01
         * variables and (x^3 + x^2 + x)*s into the s?_123 variables.
         */
        uint16_t s0 = s->slice[0], s1 = s->slice[1], s2 = s->slice[2], s3 = s->slice[3];
        uint16_t s4 = s->slice[4], s5 = s->slice[5], s6 = s->slice[6], s7 = s->slice[7];
        uint16_t s0_01 = s0 ^ ROT(s0, 1), s0_123 = ROT(s0_01, 1) ^ ROT(s0, 3);
        uint16_t s1_01 = s1 ^ ROT(s1, 1), s1_123 = ROT(s1_01, 1) ^ ROT(s1, 3);
        uint16_t s2_01 = s2 ^ ROT(s2, 1), s2_123 = ROT(s2_01, 1) ^ ROT(s2, 3);
        uint16_t s3_01 = s3 ^ ROT(s3, 1), s3_123 = ROT(s3_01, 1) ^ ROT(s3, 3);
        uint16_t s4_01 = s4 ^ ROT(s4, 1), s4_123 = ROT(s4_01, 1) ^ ROT(s4, 3);
        uint16_t s5_01 = s5 ^ ROT(s5, 1), s5_123 = ROT(s5_01, 1) ^ ROT(s5, 3);
        uint16_t s6_01 = s6 ^ ROT(s6, 1), s6_123 = ROT(s6_01, 1) ^ ROT(s6, 3);
        uint16_t s7_01 = s7 ^ ROT(s7, 1), s7_123 = ROT(s7_01, 1) ^ ROT(s7, 3);
        /* Now compute s = s?_123 + {02} * s?_01. */
        s->slice[0] = s7_01 ^ s0_123;
        s->slice[1] = s7_01 ^ s0_01 ^ s1_123;
        s->slice[2] = s1_01 ^ s2_123;
        s->slice[3] = s7_01 ^ s2_01 ^ s3_123;
        s->slice[4] = s7_01 ^ s3_01 ^ s4_123;
        s->slice[5] = s4_01 ^ s5_123;
        s->slice[6] = s5_01 ^ s6_123;
        s->slice[7] = s6_01 ^ s7_123;
        if (inv) {
            /* In the reverse direction, we further need to multiply by
             * {04}x^2 + {05}, which can be written as {04} * (x^2 + {01}) + {01}.
             *
             * First compute (x^2 + {01}) * s into the t?_02 variables: */
            uint16_t t0_02 = s->slice[0] ^ ROT(s->slice[0], 2);
            uint16_t t1_02 = s->slice[1] ^ ROT(s->slice[1], 2);
            uint16_t t2_02 = s->slice[2] ^ ROT(s->slice[2], 2);
            uint16_t t3_02 = s->slice[3] ^ ROT(s->slice[3], 2);
            uint16_t t4_02 = s->slice[4] ^ ROT(s->slice[4], 2);
            uint16_t t5_02 = s->slice[5] ^ ROT(s->slice[5], 2);
            uint16_t t6_02 = s->slice[6] ^ ROT(s->slice[6], 2);
            uint16_t t7_02 = s->slice[7] ^ ROT(s->slice[7], 2);
            /* And then update s += {04} * t?_02 */
            s->slice[0] ^= t6_02;
            s->slice[1] ^= t6_02 ^ t7_02;
            s->slice[2] ^= t0_02 ^ t7_02;
            s->slice[3] ^= t1_02 ^ t6_02;
            s->slice[4] ^= t2_02 ^ t6_02 ^ t7_02;
            s->slice[5] ^= t3_02 ^ t7_02;
            s->slice[6] ^= t4_02;
            s->slice[7] ^= t5_02;
        }
        */
}
