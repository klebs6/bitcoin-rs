// ---------------- [ File: bitcoin-sha256/src/hash.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/hash.h]

pub struct HmacSha256 {
    inner: Sha256,
    outer: Sha256,
}

pub struct Rfc6979HmacSha256 {
    v:     [u8; 32],
    k:     [u8; 32],
    retry: i32,
}

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/hash_impl.h]

macro_rules! ch {
    ($x:ident, 
     $y:ident, 
     $z:ident) => {
        /*
                ((z) ^ ((x) & ((y) ^ (z))))
        */
    }
}

macro_rules! maj {
    ($x:ident, 
     $y:ident, 
     $z:ident) => {
        /*
                (((x) & (y)) | ((z) & ((x) | (y))))
        */
    }
}

macro_rules! sigma0 {
    ($x:ident) => {
        /*
                (((x) >> 2 | (x) << 30) ^ ((x) >> 13 | (x) << 19) ^ ((x) >> 22 | (x) << 10))
        */
    }
}

macro_rules! sigma1 {
    ($x:ident) => {
        /*
                (((x) >> 6 | (x) << 26) ^ ((x) >> 11 | (x) << 21) ^ ((x) >> 25 | (x) << 7))
        */
    }
}

macro_rules! sigma0 {
    ($x:ident) => {
        /*
                (((x) >> 7 | (x) << 25) ^ ((x) >> 18 | (x) << 14) ^ ((x) >> 3))
        */
    }
}

macro_rules! sigma1 {
    ($x:ident) => {
        /*
                (((x) >> 17 | (x) << 15) ^ ((x) >> 19 | (x) << 13) ^ ((x) >> 10))
        */
    }
}

macro_rules! round {
    ($a:ident, 
     $b:ident, 
     $c:ident, 
     $d:ident, 
     $e:ident, 
     $f:ident, 
     $g:ident, 
     $h:ident, 
     $k:ident, 
     $w:ident) => {
        /*
                do { 
            uint32_t t1 = (h) + Sigma1(e) + Ch((e), (f), (g)) + (k) + (w); 
            uint32_t t2 = Sigma0(a) + Maj((a), (b), (c)); 
            (d) += t1; 
            (h) = t1 + t2; 
        } while(0)
        */
    }
}

#[cfg(BIG_ENDIAN)]
macro_rules! be32 {
    ($x:ident) => {
        /*
                (x)
        */
    }
}

#[cfg(LITTLE_ENDIAN)]
macro_rules! be32 {
    ($p:ident) => {
        /*
                ((((p) & 0xFF) << 24) | (((p) & 0xFF00) << 8) | (((p) & 0xFF0000) >> 8) | (((p) & 0xFF000000) >> 24))
        */
    }
}

pub fn sha256_initialize(hash: *mut Sha256)  {
    
    todo!();
        /*
            hash->s[0] = 0x6a09e667ul;
        hash->s[1] = 0xbb67ae85ul;
        hash->s[2] = 0x3c6ef372ul;
        hash->s[3] = 0xa54ff53aul;
        hash->s[4] = 0x510e527ful;
        hash->s[5] = 0x9b05688cul;
        hash->s[6] = 0x1f83d9abul;
        hash->s[7] = 0x5be0cd19ul;
        hash->bytes = 0;
        */
}

/**
  | Perform one SHA-256 transformation,
  | processing 16 big endian 32-bit words.
  |
  */
pub fn sha256_transform(
        s:     *mut u32,
        chunk: *const u32)  {
    
    todo!();
        /*
            uint32_t a = s[0], b = s[1], c = s[2], d = s[3], e = s[4], f = s[5], g = s[6], h = s[7];
        uint32_t w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, w10, w11, w12, w13, w14, w15;

        Round(a, b, c, d, e, f, g, h, 0x428a2f98, w0 = BE32(chunk[0]));
        Round(h, a, b, c, d, e, f, g, 0x71374491, w1 = BE32(chunk[1]));
        Round(g, h, a, b, c, d, e, f, 0xb5c0fbcf, w2 = BE32(chunk[2]));
        Round(f, g, h, a, b, c, d, e, 0xe9b5dba5, w3 = BE32(chunk[3]));
        Round(e, f, g, h, a, b, c, d, 0x3956c25b, w4 = BE32(chunk[4]));
        Round(d, e, f, g, h, a, b, c, 0x59f111f1, w5 = BE32(chunk[5]));
        Round(c, d, e, f, g, h, a, b, 0x923f82a4, w6 = BE32(chunk[6]));
        Round(b, c, d, e, f, g, h, a, 0xab1c5ed5, w7 = BE32(chunk[7]));
        Round(a, b, c, d, e, f, g, h, 0xd807aa98, w8 = BE32(chunk[8]));
        Round(h, a, b, c, d, e, f, g, 0x12835b01, w9 = BE32(chunk[9]));
        Round(g, h, a, b, c, d, e, f, 0x243185be, w10 = BE32(chunk[10]));
        Round(f, g, h, a, b, c, d, e, 0x550c7dc3, w11 = BE32(chunk[11]));
        Round(e, f, g, h, a, b, c, d, 0x72be5d74, w12 = BE32(chunk[12]));
        Round(d, e, f, g, h, a, b, c, 0x80deb1fe, w13 = BE32(chunk[13]));
        Round(c, d, e, f, g, h, a, b, 0x9bdc06a7, w14 = BE32(chunk[14]));
        Round(b, c, d, e, f, g, h, a, 0xc19bf174, w15 = BE32(chunk[15]));

        Round(a, b, c, d, e, f, g, h, 0xe49b69c1, w0 += sigma1(w14) + w9 + sigma0(w1));
        Round(h, a, b, c, d, e, f, g, 0xefbe4786, w1 += sigma1(w15) + w10 + sigma0(w2));
        Round(g, h, a, b, c, d, e, f, 0x0fc19dc6, w2 += sigma1(w0) + w11 + sigma0(w3));
        Round(f, g, h, a, b, c, d, e, 0x240ca1cc, w3 += sigma1(w1) + w12 + sigma0(w4));
        Round(e, f, g, h, a, b, c, d, 0x2de92c6f, w4 += sigma1(w2) + w13 + sigma0(w5));
        Round(d, e, f, g, h, a, b, c, 0x4a7484aa, w5 += sigma1(w3) + w14 + sigma0(w6));
        Round(c, d, e, f, g, h, a, b, 0x5cb0a9dc, w6 += sigma1(w4) + w15 + sigma0(w7));
        Round(b, c, d, e, f, g, h, a, 0x76f988da, w7 += sigma1(w5) + w0 + sigma0(w8));
        Round(a, b, c, d, e, f, g, h, 0x983e5152, w8 += sigma1(w6) + w1 + sigma0(w9));
        Round(h, a, b, c, d, e, f, g, 0xa831c66d, w9 += sigma1(w7) + w2 + sigma0(w10));
        Round(g, h, a, b, c, d, e, f, 0xb00327c8, w10 += sigma1(w8) + w3 + sigma0(w11));
        Round(f, g, h, a, b, c, d, e, 0xbf597fc7, w11 += sigma1(w9) + w4 + sigma0(w12));
        Round(e, f, g, h, a, b, c, d, 0xc6e00bf3, w12 += sigma1(w10) + w5 + sigma0(w13));
        Round(d, e, f, g, h, a, b, c, 0xd5a79147, w13 += sigma1(w11) + w6 + sigma0(w14));
        Round(c, d, e, f, g, h, a, b, 0x06ca6351, w14 += sigma1(w12) + w7 + sigma0(w15));
        Round(b, c, d, e, f, g, h, a, 0x14292967, w15 += sigma1(w13) + w8 + sigma0(w0));

        Round(a, b, c, d, e, f, g, h, 0x27b70a85, w0 += sigma1(w14) + w9 + sigma0(w1));
        Round(h, a, b, c, d, e, f, g, 0x2e1b2138, w1 += sigma1(w15) + w10 + sigma0(w2));
        Round(g, h, a, b, c, d, e, f, 0x4d2c6dfc, w2 += sigma1(w0) + w11 + sigma0(w3));
        Round(f, g, h, a, b, c, d, e, 0x53380d13, w3 += sigma1(w1) + w12 + sigma0(w4));
        Round(e, f, g, h, a, b, c, d, 0x650a7354, w4 += sigma1(w2) + w13 + sigma0(w5));
        Round(d, e, f, g, h, a, b, c, 0x766a0abb, w5 += sigma1(w3) + w14 + sigma0(w6));
        Round(c, d, e, f, g, h, a, b, 0x81c2c92e, w6 += sigma1(w4) + w15 + sigma0(w7));
        Round(b, c, d, e, f, g, h, a, 0x92722c85, w7 += sigma1(w5) + w0 + sigma0(w8));
        Round(a, b, c, d, e, f, g, h, 0xa2bfe8a1, w8 += sigma1(w6) + w1 + sigma0(w9));
        Round(h, a, b, c, d, e, f, g, 0xa81a664b, w9 += sigma1(w7) + w2 + sigma0(w10));
        Round(g, h, a, b, c, d, e, f, 0xc24b8b70, w10 += sigma1(w8) + w3 + sigma0(w11));
        Round(f, g, h, a, b, c, d, e, 0xc76c51a3, w11 += sigma1(w9) + w4 + sigma0(w12));
        Round(e, f, g, h, a, b, c, d, 0xd192e819, w12 += sigma1(w10) + w5 + sigma0(w13));
        Round(d, e, f, g, h, a, b, c, 0xd6990624, w13 += sigma1(w11) + w6 + sigma0(w14));
        Round(c, d, e, f, g, h, a, b, 0xf40e3585, w14 += sigma1(w12) + w7 + sigma0(w15));
        Round(b, c, d, e, f, g, h, a, 0x106aa070, w15 += sigma1(w13) + w8 + sigma0(w0));

        Round(a, b, c, d, e, f, g, h, 0x19a4c116, w0 += sigma1(w14) + w9 + sigma0(w1));
        Round(h, a, b, c, d, e, f, g, 0x1e376c08, w1 += sigma1(w15) + w10 + sigma0(w2));
        Round(g, h, a, b, c, d, e, f, 0x2748774c, w2 += sigma1(w0) + w11 + sigma0(w3));
        Round(f, g, h, a, b, c, d, e, 0x34b0bcb5, w3 += sigma1(w1) + w12 + sigma0(w4));
        Round(e, f, g, h, a, b, c, d, 0x391c0cb3, w4 += sigma1(w2) + w13 + sigma0(w5));
        Round(d, e, f, g, h, a, b, c, 0x4ed8aa4a, w5 += sigma1(w3) + w14 + sigma0(w6));
        Round(c, d, e, f, g, h, a, b, 0x5b9cca4f, w6 += sigma1(w4) + w15 + sigma0(w7));
        Round(b, c, d, e, f, g, h, a, 0x682e6ff3, w7 += sigma1(w5) + w0 + sigma0(w8));
        Round(a, b, c, d, e, f, g, h, 0x748f82ee, w8 += sigma1(w6) + w1 + sigma0(w9));
        Round(h, a, b, c, d, e, f, g, 0x78a5636f, w9 += sigma1(w7) + w2 + sigma0(w10));
        Round(g, h, a, b, c, d, e, f, 0x84c87814, w10 += sigma1(w8) + w3 + sigma0(w11));
        Round(f, g, h, a, b, c, d, e, 0x8cc70208, w11 += sigma1(w9) + w4 + sigma0(w12));
        Round(e, f, g, h, a, b, c, d, 0x90befffa, w12 += sigma1(w10) + w5 + sigma0(w13));
        Round(d, e, f, g, h, a, b, c, 0xa4506ceb, w13 += sigma1(w11) + w6 + sigma0(w14));
        Round(c, d, e, f, g, h, a, b, 0xbef9a3f7, w14 + sigma1(w12) + w7 + sigma0(w15));
        Round(b, c, d, e, f, g, h, a, 0xc67178f2, w15 + sigma1(w13) + w8 + sigma0(w0));

        s[0] += a;
        s[1] += b;
        s[2] += c;
        s[3] += d;
        s[4] += e;
        s[5] += f;
        s[6] += g;
        s[7] += h;
        */
}

pub fn sha256_write(
        hash: *mut Sha256,
        data: *const u8,
        len:  usize)  {
    
    todo!();
        /*
            size_t bufsize = hash->bytes & 0x3F;
        hash->bytes += len;
        VERIFY_CHECK(hash->bytes >= len);
        while (len >= 64 - bufsize) {
            /* Fill the buffer, and process it. */
            size_t chunk_len = 64 - bufsize;
            memcpy(((unsigned char*)hash->buf) + bufsize, data, chunk_len);
            data += chunk_len;
            len -= chunk_len;
            sha256_transform(hash->s, hash->buf);
            bufsize = 0;
        }
        if (len) {
            /* Fill the buffer with what remains. */
            memcpy(((unsigned char*)hash->buf) + bufsize, data, len);
        }
        */
}

pub fn sha256_finalize(
        hash:  *mut Sha256,
        out32: *mut u8)  {
    
    todo!();
        /*
            static const unsigned char pad[64] = {0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
        uint32_t sizedesc[2];
        uint32_t out[8];
        int i = 0;
        sizedesc[0] = BE32(hash->bytes >> 29);
        sizedesc[1] = BE32(hash->bytes << 3);
        sha256_write(hash, pad, 1 + ((119 - (hash->bytes % 64)) % 64));
        sha256_write(hash, (const unsigned char*)sizedesc, 8);
        for (i = 0; i < 8; i++) {
            out[i] = BE32(hash->s[i]);
            hash->s[i] = 0;
        }
        memcpy(out32, (const unsigned char*)out, 32);
        */
}

/**
  | Initializes a sha256 struct and writes
  | the 64 byte string
  | SHA256(tag)||SHA256(tag) into it.
  |
  */
pub fn sha256_initialize_tagged(
        hash:   *mut Sha256,
        tag:    *const u8,
        taglen: usize)  {
    
    todo!();
        /*
            unsigned char buf[32];
        sha256_initialize(hash);
        sha256_write(hash, tag, taglen);
        sha256_finalize(hash, buf);

        sha256_initialize(hash);
        sha256_write(hash, buf, 32);
        sha256_write(hash, buf, 32);
        */
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
