// ---------------- [ File: bitcoin-sha256-hmac/src/bitcoin_sha256_hmac.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/hmac_sha256.h]

/**
  | A hasher class for HMAC-SHA-256.
  |
  */
pub struct HmacSha256 {
    outer: Sha256,
    inner: Sha256,
}

pub const HMAC_SHA256_OUTPUT_SIZE: usize = 32;

//-------------------------------------------[.cpp/bitcoin/src/crypto/hmac_sha256.cpp]
impl HmacSha256 {

    pub fn write(&mut self, 
        data: *const u8,
        len:  usize) -> &mut HmacSha256 {
        
        todo!();
        /*
            inner.Write(data, len);
            return *this;
        */
    }
    
    pub fn new(
        key:    *const u8,
        keylen: usize) -> Self {
    
        todo!();
        /*


            unsigned char rkey[64];
        if (keylen <= 64) {
            memcpy(rkey, key, keylen);
            memset(rkey + keylen, 0, 64 - keylen);
        } else {
            CSHA256().Write(key, keylen).Finalize(rkey);
            memset(rkey + 32, 0, 32);
        }

        for (int n = 0; n < 64; n++)
            rkey[n] ^= 0x5c;
        outer.Write(rkey, 64);

        for (int n = 0; n < 64; n++)
            rkey[n] ^= 0x5c ^ 0x36;
        inner.Write(rkey, 64);
        */
    }
    
    pub fn finalize(&mut self, hash: [u8; HMAC_SHA256_OUTPUT_SIZE])  {
        
        todo!();
        /*
            unsigned char temp[32];
        inner.Finalize(temp);
        outer.Write(temp, 32).Finalize(hash);
        */
    }
}
pub struct Rfc6979HmacSha256 {
    v:     [u8; 32],
    k:     [u8; 32],
    retry: i32,
}


pub fn hmac_sha256_initialize(
        hash:   *mut HmacSha256,
        key:    *const u8,
        keylen: usize)  {
    
    todo!();
        /*
            size_t n;
        unsigned char rkey[64];
        if (keylen <= sizeof(rkey)) {
            memcpy(rkey, key, keylen);
            memset(rkey + keylen, 0, sizeof(rkey) - keylen);
        } else {
            sha256 sha256;
            sha256_initialize(&sha256);
            sha256_write(&sha256, key, keylen);
            sha256_finalize(&sha256, rkey);
            memset(rkey + 32, 0, 32);
        }

        sha256_initialize(&hash->outer);
        for (n = 0; n < sizeof(rkey); n++) {
            rkey[n] ^= 0x5c;
        }
        sha256_write(&hash->outer, rkey, sizeof(rkey));

        sha256_initialize(&hash->inner);
        for (n = 0; n < sizeof(rkey); n++) {
            rkey[n] ^= 0x5c ^ 0x36;
        }
        sha256_write(&hash->inner, rkey, sizeof(rkey));
        memset(rkey, 0, sizeof(rkey));
        */
}

pub fn hmac_sha256_write(
        hash: *mut HmacSha256,
        data: *const u8,
        size: usize)  {
    
    todo!();
        /*
            sha256_write(&hash->inner, data, size);
        */
}

pub fn hmac_sha256_finalize(
        hash:  *mut HmacSha256,
        out32: *mut u8)  {
    
    todo!();
        /*
            unsigned char temp[32];
        sha256_finalize(&hash->inner, temp);
        sha256_write(&hash->outer, temp, 32);
        memset(temp, 0, 32);
        sha256_finalize(&hash->outer, out32);
        */
}

pub fn rfc6979_hmac_sha256_initialize(
        rng:    *mut Rfc6979HmacSha256,
        key:    *const u8,
        keylen: usize)  {
    
    todo!();
        /*
            hmac_sha256 hmac;
        static const unsigned char zero[1] = {0x00};
        static const unsigned char one[1] = {0x01};

        memset(rng->v, 0x01, 32); /* RFC6979 3.2.b. */
        memset(rng->k, 0x00, 32); /* RFC6979 3.2.c. */

        /* RFC6979 3.2.d. */
        hmac_sha256_initialize(&hmac, rng->k, 32);
        hmac_sha256_write(&hmac, rng->v, 32);
        hmac_sha256_write(&hmac, zero, 1);
        hmac_sha256_write(&hmac, key, keylen);
        hmac_sha256_finalize(&hmac, rng->k);
        hmac_sha256_initialize(&hmac, rng->k, 32);
        hmac_sha256_write(&hmac, rng->v, 32);
        hmac_sha256_finalize(&hmac, rng->v);

        /* RFC6979 3.2.f. */
        hmac_sha256_initialize(&hmac, rng->k, 32);
        hmac_sha256_write(&hmac, rng->v, 32);
        hmac_sha256_write(&hmac, one, 1);
        hmac_sha256_write(&hmac, key, keylen);
        hmac_sha256_finalize(&hmac, rng->k);
        hmac_sha256_initialize(&hmac, rng->k, 32);
        hmac_sha256_write(&hmac, rng->v, 32);
        hmac_sha256_finalize(&hmac, rng->v);
        rng->retry = 0;
        */
}

pub fn rfc6979_hmac_sha256_generate(
        rng:    *mut Rfc6979HmacSha256,
        out:    *mut u8,
        outlen: usize)  {
    
    todo!();
        /*
            /* RFC6979 3.2.h. */
        static const unsigned char zero[1] = {0x00};
        if (rng->retry) {
            hmac_sha256 hmac;
            hmac_sha256_initialize(&hmac, rng->k, 32);
            hmac_sha256_write(&hmac, rng->v, 32);
            hmac_sha256_write(&hmac, zero, 1);
            hmac_sha256_finalize(&hmac, rng->k);
            hmac_sha256_initialize(&hmac, rng->k, 32);
            hmac_sha256_write(&hmac, rng->v, 32);
            hmac_sha256_finalize(&hmac, rng->v);
        }

        while (outlen > 0) {
            hmac_sha256 hmac;
            int now = outlen;
            hmac_sha256_initialize(&hmac, rng->k, 32);
            hmac_sha256_write(&hmac, rng->v, 32);
            hmac_sha256_finalize(&hmac, rng->v);
            if (now > 32) {
                now = 32;
            }
            memcpy(out, rng->v, now);
            out += now;
            outlen -= now;
        }

        rng->retry = 1;
        */
}

pub fn rfc6979_hmac_sha256_finalize(rng: *mut Rfc6979HmacSha256)  {
    
    todo!();
        /*
            memset(rng->k, 0, 32);
        memset(rng->v, 0, 32);
        rng->retry = 0;
        */
}
