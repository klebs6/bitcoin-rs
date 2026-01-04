// ---------------- [ File: bitcoinsecp256k1-ecdsa/src/ecdsa.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecdsa.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecdsa_impl.h]

/** 
 | Group order for secp256k1 defined as 'n' in
 | "Standards for Efficient Cryptography" (SEC2)
 | 2.7.1
 |
 |  sage: for t in xrange(1023, -1, -1):
 |     ..   p = 2**256 - 2**32 - t
 |     ..   if p.is_prime():
 |     ..     print '%x'%p
 |     ..     break
 |   'fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f'
 |  sage: a = 0
 |  sage: b = 7
 |  sage: F = FiniteField (p)
 |  sage: '%x' % (EllipticCurve ([F (a), F (b)]).order())
 |   'fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141'
 */
lazy_static!{
    /*
    static const fe ecdsa_const_order_as_fe = FE_CONST(
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFE,
        0xBAAEDCE6, 0xAF48A03B, 0xBFD25E8C, 0xD0364141
    );
    */
}

/** 
  | Difference between field and order, values 'p' and 'n' values defined in
  |  "Standards for Efficient Cryptography" (SEC2) 2.7.1.
  |  sage: p = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
  |  sage: a = 0
  |  sage: b = 7
  |  sage: F = FiniteField (p)
  |  sage: '%x' % (p - EllipticCurve ([F (a), F (b)]).order())
  |   '14551231950b75fc4402da1722fc9baee'
  */
lazy_static!{
    /*
    static const fe ecdsa_const_p_minus_order = FE_CONST(
        0, 0, 0, 1, 0x45512319UL, 0x50B75FC4UL, 0x402DA172UL, 0x2FC9BAEEUL
    );
    */
}

pub fn der_read_len(
        len:    *mut usize,
        sigp:   *const *const u8,
        sigend: *const u8) -> i32 {
    
    todo!();
        /*
        size_t lenleft;
        unsigned char b1;
        VERIFY_CHECK(len != NULL);
        *len = 0;
        if (*sigp >= sigend) {
            return 0;
        }
        b1 = *((*sigp)++);
        if (b1 == 0xFF) {
            /* X.690-0207 8.1.3.5.c the value 0xFF shall not be used. */
            return 0;
        }
        if ((b1 & 0x80) == 0) {
            /* X.690-0207 8.1.3.4 short form length octets */
            *len = b1;
            return 1;
        }
        if (b1 == 0x80) {
            /* Indefinite length is not allowed in DER. */
            return 0;
        }
        /* X.690-207 8.1.3.5 long form length octets */
        lenleft = b1 & 0x7F; /* lenleft is at least 1 */
        if (lenleft > (size_t)(sigend - *sigp)) {
            return 0;
        }
        if (**sigp == 0) {
            /* Not the shortest possible length encoding. */
            return 0;
        }
        if (lenleft > sizeof(size_t)) {
            /* The resulting length would exceed the range of a size_t, so
             * certainly longer than the passed array size.
             */
            return 0;
        }
        while (lenleft > 0) {
            *len = (*len << 8) | **sigp;
            (*sigp)++;
            lenleft--;
        }
        if (*len > (size_t)(sigend - *sigp)) {
            /* Result exceeds the length of the passed array. */
            return 0;
        }
        if (*len < 128) {
            /* Not the shortest possible length encoding. */
            return 0;
        }
        return 1;
        */
}

pub fn der_parse_integer(
        r:      *mut Scalar,
        sig:    *const *const u8,
        sigend: *const u8) -> i32 {
    
    todo!();
        /*
        int overflow = 0;
        unsigned char ra[32] = {0};
        size_t rlen;

        if (*sig == sigend || **sig != 0x02) {
            /* Not a primitive integer (X.690-0207 8.3.1). */
            return 0;
        }
        (*sig)++;
        if (der_read_len(&rlen, sig, sigend) == 0) {
            return 0;
        }
        if (rlen == 0 || *sig + rlen > sigend) {
            /* Exceeds bounds or not at least length 1 (X.690-0207 8.3.1).  */
            return 0;
        }
        if (**sig == 0x00 && rlen > 1 && (((*sig)[1]) & 0x80) == 0x00) {
            /* Excessive 0x00 padding. */
            return 0;
        }
        if (**sig == 0xFF && rlen > 1 && (((*sig)[1]) & 0x80) == 0x80) {
            /* Excessive 0xFF padding. */
            return 0;
        }
        if ((**sig & 0x80) == 0x80) {
            /* Negative. */
            overflow = 1;
        }
        /* There is at most one leading zero byte:
         * if there were two leading zero bytes, we would have failed and returned 0
         * because of excessive 0x00 padding already. */
        if (rlen > 0 && **sig == 0) {
            /* Skip leading zero byte */
            rlen--;
            (*sig)++;
        }
        if (rlen > 32) {
            overflow = 1;
        }
        if (!overflow) {
            if (rlen) memcpy(ra + 32 - rlen, *sig, rlen);
            scalar_set_b32(r, ra, &overflow);
        }
        if (overflow) {
            scalar_set_int(r, 0);
        }
        (*sig) += rlen;
        return 1;
        */
}

pub fn ecdsa_sig_parse(
        rr:   *mut Scalar,
        rs:   *mut Scalar,
        sig:  *const u8,
        size: usize) -> i32 {
    
    todo!();
        /*
        const unsigned char *sigend = sig + size;
        size_t rlen;
        if (sig == sigend || *(sig++) != 0x30) {
            /* The encoding doesn't start with a constructed sequence (X.690-0207 8.9.1). */
            return 0;
        }
        if (der_read_len(&rlen, &sig, sigend) == 0) {
            return 0;
        }
        if (rlen != (size_t)(sigend - sig)) {
            /* Tuple exceeds bounds or garage after tuple. */
            return 0;
        }

        if (!der_parse_integer(rr, &sig, sigend)) {
            return 0;
        }
        if (!der_parse_integer(rs, &sig, sigend)) {
            return 0;
        }

        if (sig != sigend) {
            /* Trailing garbage inside tuple. */
            return 0;
        }

        return 1;
        */
}

pub fn ecdsa_sig_serialize(
        sig:  *mut u8,
        size: *mut usize,
        ar:   *const Scalar,
        as_:  *const Scalar) -> i32 {
    
    todo!();
        /*
        unsigned char r[33] = {0}, s[33] = {0};
        unsigned char *rp = r, *sp = s;
        size_t lenR = 33, lenS = 33;
        scalar_get_b32(&r[1], ar);
        scalar_get_b32(&s[1], as);
        while (lenR > 1 && rp[0] == 0 && rp[1] < 0x80) { lenR--; rp++; }
        while (lenS > 1 && sp[0] == 0 && sp[1] < 0x80) { lenS--; sp++; }
        if (*size < 6+lenS+lenR) {
            *size = 6 + lenS + lenR;
            return 0;
        }
        *size = 6 + lenS + lenR;
        sig[0] = 0x30;
        sig[1] = 4 + lenS + lenR;
        sig[2] = 0x02;
        sig[3] = lenR;
        memcpy(sig+4, rp, lenR);
        sig[4+lenR] = 0x02;
        sig[5+lenR] = lenS;
        memcpy(sig+lenR+6, sp, lenS);
        return 1;
        */
}

pub fn ecdsa_sig_verify(
        ctx:     *const EcMultContext,
        sigr:    *const Scalar,
        sigs:    *const Scalar,
        pubkey:  *const Ge,
        message: *const Scalar) -> i32 {
    
    todo!();
        /*
            unsigned char c[32];
        scalar sn, u1, u2;
    #if !defined(EXHAUSTIVE_TEST_ORDER)
        fe xr;
    #endif
        gej pubkeyj;
        gej pr;

        if (scalar_is_zero(sigr) || scalar_is_zero(sigs)) {
            return 0;
        }

        scalar_inverse_var(&sn, sigs);
        scalar_mul(&u1, &sn, message);
        scalar_mul(&u2, &sn, sigr);
        gej_set_ge(&pubkeyj, pubkey);
        ecmult(ctx, &pr, &pubkeyj, &u2, &u1);
        if (gej_is_infinity(&pr)) {
            return 0;
        }

    #if defined(EXHAUSTIVE_TEST_ORDER)
    {
        scalar computed_r;
        ge pr_ge;
        ge_set_gej(&pr_ge, &pr);
        fe_normalize(&pr_ge.x);

        fe_get_b32(c, &pr_ge.x);
        scalar_set_b32(&computed_r, c, NULL);
        return scalar_eq(sigr, &computed_r);
    }
    #else
        scalar_get_b32(c, sigr);
        fe_set_b32(&xr, c);

        /** We now have the recomputed R point in pr, and its claimed x coordinate (modulo n)
         *  in xr. Naively, we would extract the x coordinate from pr (requiring a inversion modulo p),
         *  compute the remainder modulo n, and compare it to xr. However:
         *
         *        xr == X(pr) mod n
         *    <=> exists h. (xr + h * n < p && xr + h * n == X(pr))
         *    [Since 2 * n > p, h can only be 0 or 1]
         *    <=> (xr == X(pr)) || (xr + n < p && xr + n == X(pr))
         *    [In Jacobian coordinates, X(pr) is pr.x / pr.z^2 mod p]
         *    <=> (xr == pr.x / pr.z^2 mod p) || (xr + n < p && xr + n == pr.x / pr.z^2 mod p)
         *    [Multiplying both sides of the equations by pr.z^2 mod p]
         *    <=> (xr * pr.z^2 mod p == pr.x) || (xr + n < p && (xr + n) * pr.z^2 mod p == pr.x)
         *
         *  Thus, we can avoid the inversion, but we have to check both cases separately.
         *  gej_eq_x implements the (xr * pr.z^2 mod p == pr.x) test.
         */
        if (gej_eq_x_var(&xr, &pr)) {
            /* xr * pr.z^2 mod p == pr.x, so the signature is valid. */
            return 1;
        }
        if (fe_cmp_var(&xr, &ecdsa_const_p_minus_order) >= 0) {
            /* xr + n >= p, so we can skip testing the second case. */
            return 0;
        }
        fe_add(&xr, &ecdsa_const_order_as_fe);
        if (gej_eq_x_var(&xr, &pr)) {
            /* (xr + n) * pr.z^2 mod p == pr.x, so the signature is valid. */
            return 1;
        }
        return 0;
    #endif
        */
}

pub fn ecdsa_sig_sign(
        ctx:     *const EcMultGenContext,
        sigr:    *mut Scalar,
        sigs:    *mut Scalar,
        seckey:  *const Scalar,
        message: *const Scalar,
        nonce:   *const Scalar,
        recid:   *mut i32) -> i32 {
    
    todo!();
        /*
            unsigned char b[32];
        gej rp;
        ge r;
        scalar n;
        int overflow = 0;
        int high;

        ecmult_gen(ctx, &rp, nonce);
        ge_set_gej(&r, &rp);
        fe_normalize(&r.x);
        fe_normalize(&r.y);
        fe_get_b32(b, &r.x);
        scalar_set_b32(sigr, b, &overflow);
        if (recid) {
            /* The overflow condition is cryptographically unreachable as hitting it requires finding the discrete log
             * of some P where P.x >= order, and only 1 in about 2^127 points meet this criteria.
             */
            *recid = (overflow << 1) | fe_is_odd(&r.y);
        }
        scalar_mul(&n, sigr, seckey);
        scalar_add(&n, &n, message);
        scalar_inverse(sigs, nonce);
        scalar_mul(sigs, sigs, &n);
        scalar_clear(&n);
        gej_clear(&rp);
        ge_clear(&r);
        high = scalar_is_high(sigs);
        scalar_cond_negate(sigs, high);
        if (recid) {
                *recid ^= high;
        }
        /* P.x = order is on the curve, so technically sig->r could end up being zero, which would be an invalid signature.
         * This is cryptographically unreachable as hitting it requires finding the discrete log of P.x = N.
         */
        return !scalar_is_zero(sigr) & !scalar_is_zero(sigs);
        */
}

/**
  | This function is taken from the libsecp256k1
  | distribution and implements
  | 
  | DER parsing for ECDSA signatures, while
  | supporting an arbitrary subset of format
  | violations.
  | 
  | Supported violations include negative
  | integers, excessive padding, garbage
  | at the end, and overly long length descriptors.
  | This is safe to use in
  | 
  | Bitcoin because since the activation
  | of BIP66, signatures are verified to
  | be strict DER before being passed to
  | this module, and we know it supports
  | all violations present in the blockchain
  | before that point.
  |
  */
pub fn ecdsa_signature_parse_der_lax(
        ctx:      *const Secp256k1Context,
        sig:      *mut Secp256k1EcdsaSignature,
        input:    *const u8,
        inputlen: usize) -> i32 {
    
    todo!();
        /*
            size_t rpos, rlen, spos, slen;
        size_t pos = 0;
        size_t lenbyte;
        unsigned char tmpsig[64] = {0};
        int overflow = 0;

        /* Hack to initialize sig with a correctly-parsed but invalid signature. */
        secp256k1_ecdsa_signature_parse_compact(ctx, sig, tmpsig);

        /* Sequence tag byte */
        if (pos == inputlen || input[pos] != 0x30) {
            return 0;
        }
        pos++;

        /* Sequence length bytes */
        if (pos == inputlen) {
            return 0;
        }
        lenbyte = input[pos++];
        if (lenbyte & 0x80) {
            lenbyte -= 0x80;
            if (lenbyte > inputlen - pos) {
                return 0;
            }
            pos += lenbyte;
        }

        /* Integer tag byte for R */
        if (pos == inputlen || input[pos] != 0x02) {
            return 0;
        }
        pos++;

        /* Integer length for R */
        if (pos == inputlen) {
            return 0;
        }
        lenbyte = input[pos++];
        if (lenbyte & 0x80) {
            lenbyte -= 0x80;
            if (lenbyte > inputlen - pos) {
                return 0;
            }
            while (lenbyte > 0 && input[pos] == 0) {
                pos++;
                lenbyte--;
            }
            const_assert(sizeof(size_t) >= 4, "size_t too small");
            if (lenbyte >= 4) {
                return 0;
            }
            rlen = 0;
            while (lenbyte > 0) {
                rlen = (rlen << 8) + input[pos];
                pos++;
                lenbyte--;
            }
        } else {
            rlen = lenbyte;
        }
        if (rlen > inputlen - pos) {
            return 0;
        }
        rpos = pos;
        pos += rlen;

        /* Integer tag byte for S */
        if (pos == inputlen || input[pos] != 0x02) {
            return 0;
        }
        pos++;

        /* Integer length for S */
        if (pos == inputlen) {
            return 0;
        }
        lenbyte = input[pos++];
        if (lenbyte & 0x80) {
            lenbyte -= 0x80;
            if (lenbyte > inputlen - pos) {
                return 0;
            }
            while (lenbyte > 0 && input[pos] == 0) {
                pos++;
                lenbyte--;
            }
            const_assert(sizeof(size_t) >= 4, "size_t too small");
            if (lenbyte >= 4) {
                return 0;
            }
            slen = 0;
            while (lenbyte > 0) {
                slen = (slen << 8) + input[pos];
                pos++;
                lenbyte--;
            }
        } else {
            slen = lenbyte;
        }
        if (slen > inputlen - pos) {
            return 0;
        }
        spos = pos;

        /* Ignore leading zeroes in R */
        while (rlen > 0 && input[rpos] == 0) {
            rlen--;
            rpos++;
        }
        /* Copy R value */
        if (rlen > 32) {
            overflow = 1;
        } else {
            memcpy(tmpsig + 32 - rlen, input + rpos, rlen);
        }

        /* Ignore leading zeroes in S */
        while (slen > 0 && input[spos] == 0) {
            slen--;
            spos++;
        }
        /* Copy S value */
        if (slen > 32) {
            overflow = 1;
        } else {
            memcpy(tmpsig + 64 - slen, input + spos, slen);
        }

        if (!overflow) {
            overflow = !secp256k1_ecdsa_signature_parse_compact(ctx, sig, tmpsig);
        }
        if (overflow) {
            /* Overwrite the result again with a correctly-parsed but invalid
               signature if parsing failed. */
            memset(tmpsig, 0, 64);
            secp256k1_ecdsa_signature_parse_compact(ctx, sig, tmpsig);
        }
        return 1;
        */
}
