/*!
  | Please do not link this file directly.
  | It is not part of the libsecp256k1 project
  | and does not promise any stability in
  | its API, functionality or presence.
  | Projects which use this code should
  | instead copy this header and its accompanying
  | .c file directly into their codebase.
  | **
  |
  | This file contains code snippets that
  | parse DER private keys with various
  | errors and violations. This is not a
  | part of the library itself, because
  | the allowed violations are chosen arbitrarily
  | and do not follow or establish any standard.
  | 
  | It also contains code to serialize private
  | keys in a compatible manner.
  | 
  | These functions are meant for compatibility
  | with applications that require BER
  | encoded keys. When working with secp256k1-specific
  | code, the simple 32-byte private keys
  | normally used by the library are sufficient.
  |
  */

crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/secp256k1/contrib/lax_der_privatekey_parsing.h]

/** 
  | Export a private key in DER format.
  |
  |  Returns: 1 if the private key was valid.
  |
  |  Args: ctx:        pointer to a context object,
  |                    initialized for signing
  |                    (cannot be NULL)
  |
  |  Out: privkey:     pointer to an array for
  |                    storing the private key in
  |                    BER.  Should have space for
  |                    279 bytes, and cannot be
  |                    NULL.
  |
  |       privkeylen:  Pointer to an int where the
  |                    length of the private key in
  |                    privkey will be stored.
  |
  |  In:  seckey:      pointer to a 32-byte secret
  |                    key to export.
  |
  |       compressed:  1 if the key should be
  |                    exported in compressed
  |                    format, 0 otherwise
  |
  |  This function is purely meant for
  |  compatibility with applications that require
  |  BER encoded keys. When working with
  |  secp256k1-specific code, the simple 32-byte
  |  private keys are sufficient.
  |
  |  Note that this function does not guarantee
  |  correct DER output. It is guaranteed to be
  |  parsable by ec_privkey_import_der
  */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_privkey_export_der(
        const context* ctx,
        unsigned char *privkey,
        size_t *privkeylen,
        const unsigned char *seckey,
        int compressed
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3) ARG_NONNULL(4);
    */
}

/** 
  | Import a private key in DER format.
  |
  | Returns: 1 if a private key was extracted.
  |
  | Args: ctx:        pointer to a context object
  |                   (cannot be NULL).
  |
  | Out:  seckey:     pointer to a 32-byte array
  |                   for storing the private key.
  |                   (cannot be NULL).
  |
  | In:   privkey:    pointer to a private key in
  |                   DER format (cannot be NULL).
  |
  |       privkeylen: length of the DER private key
  |                   pointed to be privkey.
  |
  | This function will accept more than just strict
  | DER, and even allow some BER violations. The
  | public key stored inside the DER-encoded
  | private key is not verified for correctness,
  | nor are the curve parameters. Use this function
  | only if you know in advance it is supposed to
  | contain a secp256k1 private key.
  */
lazy_static!{
    /*
    WARN_UNUSED_RESULT int ec_privkey_import_der(
        const context* ctx,
        unsigned char *seckey,
        const unsigned char *privkey,
        size_t privkeylen
    ) ARG_NONNULL(1) ARG_NONNULL(2) ARG_NONNULL(3);
    */
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/contrib/lax_der_privatekey_parsing.c]

pub fn ec_privkey_import_der(
        ctx:        *const Secp256k1Context,
        out32:      *mut u8,
        privkey:    *const u8,
        privkeylen: usize) -> i32 {
    
    todo!();
        /*
        const unsigned char *end = privkey + privkeylen;
        int lenb = 0;
        int len = 0;
        memset(out32, 0, 32);
        /* sequence header */
        if (end < privkey+1 || *privkey != 0x30) {
            return 0;
        }
        privkey++;
        /* sequence length constructor */
        if (end < privkey+1 || !(*privkey & 0x80)) {
            return 0;
        }
        lenb = *privkey & ~0x80; privkey++;
        if (lenb < 1 || lenb > 2) {
            return 0;
        }
        if (end < privkey+lenb) {
            return 0;
        }
        /* sequence length */
        len = privkey[lenb-1] | (lenb > 1 ? privkey[lenb-2] << 8 : 0);
        privkey += lenb;
        if (end < privkey+len) {
            return 0;
        }
        /* sequence element 0: version number (=1) */
        if (end < privkey+3 || privkey[0] != 0x02 || privkey[1] != 0x01 || privkey[2] != 0x01) {
            return 0;
        }
        privkey += 3;
        /* sequence element 1: octet string, up to 32 bytes */
        if (end < privkey+2 || privkey[0] != 0x04 || privkey[1] > 0x20 || end < privkey+2+privkey[1]) {
            return 0;
        }
        if (privkey[1]) memcpy(out32 + 32 - privkey[1], privkey + 2, privkey[1]);
        if (!ec_seckey_verify(ctx, out32)) {
            memset(out32, 0, 32);
            return 0;
        }
        return 1;
        */
}

pub fn ec_privkey_export_der(
        ctx:        *const Secp256k1Context,
        privkey:    *mut u8,
        privkeylen: *mut usize,
        key32:      *const u8,
        compressed: i32) -> i32 {
    
    todo!();
        /*
            pubkey pubkey;
        size_t pubkeylen = 0;
        if (!ec_pubkey_create(ctx, &pubkey, key32)) {
            *privkeylen = 0;
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
            unsigned char *ptr = privkey;
            memcpy(ptr, begin, sizeof(begin)); ptr += sizeof(begin);
            memcpy(ptr, key32, 32); ptr += 32;
            memcpy(ptr, middle, sizeof(middle)); ptr += sizeof(middle);
            pubkeylen = 33;
            ec_pubkey_serialize(ctx, ptr, &pubkeylen, &pubkey, EC_COMPRESSED);
            ptr += pubkeylen;
            *privkeylen = ptr - privkey;
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
            unsigned char *ptr = privkey;
            memcpy(ptr, begin, sizeof(begin)); ptr += sizeof(begin);
            memcpy(ptr, key32, 32); ptr += 32;
            memcpy(ptr, middle, sizeof(middle)); ptr += sizeof(middle);
            pubkeylen = 65;
            ec_pubkey_serialize(ctx, ptr, &pubkeylen, &pubkey, EC_UNCOMPRESSED);
            ptr += pubkeylen;
            *privkeylen = ptr - privkey;
        }
        return 1;
        */
}
