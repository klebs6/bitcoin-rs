// ---------------- [ File: bitcoinsecp256k1-ec/src/eckey.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/eckey.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/eckey_impl.h]

pub fn eckey_pubkey_parse(
        elem: *mut Ge,
        pub_: *const u8,
        size: usize) -> i32 {
    
    todo!();
        /*
        if (size == 33 && (pub[0] == TAG_PUBKEY_EVEN || pub[0] == TAG_PUBKEY_ODD)) {
            fe x;
            return fe_set_b32(&x, pub+1) && ge_set_xo_var(elem, &x, pub[0] == TAG_PUBKEY_ODD);
        } else if (size == 65 && (pub[0] == TAG_PUBKEY_UNCOMPRESSED || pub[0] == TAG_PUBKEY_HYBRID_EVEN || pub[0] == TAG_PUBKEY_HYBRID_ODD)) {
            fe x, y;
            if (!fe_set_b32(&x, pub+1) || !fe_set_b32(&y, pub+33)) {
                return 0;
            }
            ge_set_xy(elem, &x, &y);
            if ((pub[0] == TAG_PUBKEY_HYBRID_EVEN || pub[0] == TAG_PUBKEY_HYBRID_ODD) &&
                fe_is_odd(&y) != (pub[0] == TAG_PUBKEY_HYBRID_ODD)) {
                return 0;
            }
            return ge_is_valid_var(elem);
        } else {
            return 0;
        }
        */
}

pub fn eckey_pubkey_serialize(
        elem:       *mut Ge,
        pub_:       *mut u8,
        size:       *mut usize,
        compressed: i32) -> i32 {
    
    todo!();
        /*
            if (ge_is_infinity(elem)) {
            return 0;
        }
        fe_normalize_var(&elem->x);
        fe_normalize_var(&elem->y);
        fe_get_b32(&pub[1], &elem->x);
        if (compressed) {
            *size = 33;
            pub[0] = fe_is_odd(&elem->y) ? TAG_PUBKEY_ODD : TAG_PUBKEY_EVEN;
        } else {
            *size = 65;
            pub[0] = TAG_PUBKEY_UNCOMPRESSED;
            fe_get_b32(&pub[33], &elem->y);
        }
        return 1;
        */
}

pub fn eckey_privkey_tweak_add(
        key:   *mut Scalar,
        tweak: *const Scalar) -> i32 {
    
    todo!();
        /*
        scalar_add(key, key, tweak);
        return !scalar_is_zero(key);
        */
}

pub fn eckey_pubkey_tweak_add(
        ctx:   *const EcMultContext,
        key:   *mut Ge,
        tweak: *const Scalar) -> i32 {
    
    todo!();
        /*
            gej pt;
        scalar one;
        gej_set_ge(&pt, key);
        scalar_set_int(&one, 1);
        ecmult(ctx, &pt, &pt, &one, tweak);

        if (gej_is_infinity(&pt)) {
            return 0;
        }
        ge_set_gej(key, &pt);
        return 1;
        */
}

pub fn eckey_privkey_tweak_mul(
        key:   *mut Scalar,
        tweak: *const Scalar) -> i32 {
    
    todo!();
        /*
        int ret;
        ret = !scalar_is_zero(tweak);

        scalar_mul(key, key, tweak);
        return ret;
        */
}

pub fn eckey_pubkey_tweak_mul(
        ctx:   *const EcMultContext,
        key:   *mut Ge,
        tweak: *const Scalar) -> i32 {
    
    todo!();
        /*
            scalar zero;
        gej pt;
        if (scalar_is_zero(tweak)) {
            return 0;
        }

        scalar_set_int(&zero, 0);
        gej_set_ge(&pt, key);
        ecmult(ctx, &pt, &pt, tweak, &zero);
        ge_set_gej(key, &pt);
        return 1;
        */
}

/**
  | This parses a format loosely based on
  | a DER encoding of the ECPrivateKey type
  | from section C.4 of SEC 1 <https://www.secg.org/sec1-v2.pdf>,
  | with the following caveats:
  | 
  | - The octet-length of the SEQUENCE must
  | be encoded as 1 or 2 octets. It is not required
  | to be encoded as one octet if it is less
  | than 256, as DER would require.
  | 
  | - The octet-length of the SEQUENCE must
  | not be greater than the remaining length
  | of the key encoding, but need not match
  | it (i.e. the encoding may contain junk
  | after the encoded SEQUENCE).
  | 
  | - The privateKey OCTET STRING is zero-filled
  | on the left to 32 octets.
  | 
  | - Anything after the encoding of the
  | privateKey OCTET STRING is ignored,
  | whether or not it is validly encoded
  | DER. out32 must point to an output buffer
  | of length at least 32 bytes.
  |
  */
pub fn ec_seckey_import_der(
        ctx:       *const Secp256k1Context,
        out32:     *mut u8,
        seckey:    *const u8,
        seckeylen: usize) -> i32 {
    
    todo!();
        /*
            const unsigned char *end = seckey + seckeylen;
        memset(out32, 0, 32);
        /* sequence header */
        if (end - seckey < 1 || *seckey != 0x30u) {
            return 0;
        }
        seckey++;
        /* sequence length constructor */
        if (end - seckey < 1 || !(*seckey & 0x80u)) {
            return 0;
        }
        ptrdiff_t lenb = *seckey & ~0x80u; seckey++;
        if (lenb < 1 || lenb > 2) {
            return 0;
        }
        if (end - seckey < lenb) {
            return 0;
        }
        /* sequence length */
        ptrdiff_t len = seckey[lenb-1] | (lenb > 1 ? seckey[lenb-2] << 8 : 0u);
        seckey += lenb;
        if (end - seckey < len) {
            return 0;
        }
        /* sequence element 0: version number (=1) */
        if (end - seckey < 3 || seckey[0] != 0x02u || seckey[1] != 0x01u || seckey[2] != 0x01u) {
            return 0;
        }
        seckey += 3;
        /* sequence element 1: octet string, up to 32 bytes */
        if (end - seckey < 2 || seckey[0] != 0x04u) {
            return 0;
        }
        ptrdiff_t oslen = seckey[1];
        seckey += 2;
        if (oslen > 32 || end - seckey < oslen) {
            return 0;
        }
        memcpy(out32 + (32 - oslen), seckey, oslen);
        if (!secp256k1_ec_seckey_verify(ctx, out32)) {
            memset(out32, 0, 32);
            return 0;
        }
        return 1;
        */
}

/**
  | This serializes to a DER encoding of
  | the ECPrivateKey type from section
  | C.4 of SEC 1 <https://www.secg.org/sec1-v2.pdf>.
  | The optional parameters and publicKey
  | fields are included.
  | 
  | seckey must point to an output buffer
  | of length at least CKey::SIZE bytes.
  | seckeylen must initially be set to the
  | size of the seckey buffer. Upon return
  | it will be set to the number of bytes used
  | in the buffer. key32 must point to a 32-byte
  | raw private key.
  |
  */
pub fn ec_seckey_export_der(
        ctx:        *const Secp256k1Context,
        seckey:     *mut u8,
        seckeylen:  *mut usize,
        key32:      *const u8,
        compressed: bool) -> i32 {
    
    todo!();
        /*
            assert(*seckeylen >= CKey::SIZE);
        secp256k1_pubkey pubkey;
        size_t pubkeylen = 0;
        if (!secp256k1_ec_pubkey_create(ctx, &pubkey, key32)) {
            *seckeylen = 0;
            return 0;
        }
        if (compressed) {
            static const unsigned char begin[] = {
                0x30,0x81,0xD3,0x02,0x01,0x01,0x04,0x20
            };
            static const unsigned char middle[] = {
                0xA0,0x81,0x85,0x30,0x81,0x82,0x02,0x01,0x01,0x30,0x2C,0x06,0x07,0x2A,0x86,0x48,
                0xCE,0x3D,0x01,0x01,0x02,0x21,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
                0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
                0xFF,0xFF,0xFE,0xFF,0xFF,0xFC,0x2F,0x30,0x06,0x04,0x01,0x00,0x04,0x01,0x07,0x04,
                0x21,0x02,0x79,0xBE,0x66,0x7E,0xF9,0xDC,0xBB,0xAC,0x55,0xA0,0x62,0x95,0xCE,0x87,
                0x0B,0x07,0x02,0x9B,0xFC,0xDB,0x2D,0xCE,0x28,0xD9,0x59,0xF2,0x81,0x5B,0x16,0xF8,
                0x17,0x98,0x02,0x21,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
                0xFF,0xFF,0xFF,0xFF,0xFE,0xBA,0xAE,0xDC,0xE6,0xAF,0x48,0xA0,0x3B,0xBF,0xD2,0x5E,
                0x8C,0xD0,0x36,0x41,0x41,0x02,0x01,0x01,0xA1,0x24,0x03,0x22,0x00
            };
            unsigned char *ptr = seckey;
            memcpy(ptr, begin, sizeof(begin)); ptr += sizeof(begin);
            memcpy(ptr, key32, 32); ptr += 32;
            memcpy(ptr, middle, sizeof(middle)); ptr += sizeof(middle);
            pubkeylen = crate::PubKey::COMPRESSED_SIZE;
            secp256k1_ec_pubkey_serialize(ctx, ptr, &pubkeylen, &pubkey, SECP256K1_EC_COMPRESSED);
            ptr += pubkeylen;
            *seckeylen = ptr - seckey;
            assert(*seckeylen == CKey::COMPRESSED_SIZE);
        } else {
            static const unsigned char begin[] = {
                0x30,0x82,0x01,0x13,0x02,0x01,0x01,0x04,0x20
            };
            static const unsigned char middle[] = {
                0xA0,0x81,0xA5,0x30,0x81,0xA2,0x02,0x01,0x01,0x30,0x2C,0x06,0x07,0x2A,0x86,0x48,
                0xCE,0x3D,0x01,0x01,0x02,0x21,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
                0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
                0xFF,0xFF,0xFE,0xFF,0xFF,0xFC,0x2F,0x30,0x06,0x04,0x01,0x00,0x04,0x01,0x07,0x04,
                0x41,0x04,0x79,0xBE,0x66,0x7E,0xF9,0xDC,0xBB,0xAC,0x55,0xA0,0x62,0x95,0xCE,0x87,
                0x0B,0x07,0x02,0x9B,0xFC,0xDB,0x2D,0xCE,0x28,0xD9,0x59,0xF2,0x81,0x5B,0x16,0xF8,
                0x17,0x98,0x48,0x3A,0xDA,0x77,0x26,0xA3,0xC4,0x65,0x5D,0xA4,0xFB,0xFC,0x0E,0x11,
                0x08,0xA8,0xFD,0x17,0xB4,0x48,0xA6,0x85,0x54,0x19,0x9C,0x47,0xD0,0x8F,0xFB,0x10,
                0xD4,0xB8,0x02,0x21,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
                0xFF,0xFF,0xFF,0xFF,0xFE,0xBA,0xAE,0xDC,0xE6,0xAF,0x48,0xA0,0x3B,0xBF,0xD2,0x5E,
                0x8C,0xD0,0x36,0x41,0x41,0x02,0x01,0x01,0xA1,0x44,0x03,0x42,0x00
            };
            unsigned char *ptr = seckey;
            memcpy(ptr, begin, sizeof(begin)); ptr += sizeof(begin);
            memcpy(ptr, key32, 32); ptr += 32;
            memcpy(ptr, middle, sizeof(middle)); ptr += sizeof(middle);
            pubkeylen = crate::PubKey::SIZE;
            secp256k1_ec_pubkey_serialize(ctx, ptr, &pubkeylen, &pubkey, SECP256K1_EC_UNCOMPRESSED);
            ptr += pubkeylen;
            *seckeylen = ptr - seckey;
            assert(*seckeylen == CKey::SIZE);
        }
        return 1;
        */
}
