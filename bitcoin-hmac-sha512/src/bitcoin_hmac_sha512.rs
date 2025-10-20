// ---------------- [ File: bitcoin-hmac-sha512/src/bitcoin_hmac_sha512.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/hmac_sha512.h]

/**
  | A hasher class for HMAC-SHA-512.
  |
  */
pub struct HmacSha512 {
    outer: Sha512,
    inner: Sha512,
}

pub const HMAC_SHA512_OUTPUT_SIZE: usize = 64;

//-------------------------------------------[.cpp/bitcoin/src/crypto/hmac_sha512.cpp]
impl HmacSha512 {

    pub fn new(key: *const u8, keylen: usize) -> Self {
        // Prepare rkey (block-sized key, 128 bytes)
        let mut rkey = [0u8; 128];

        if keylen <= 128 {
            if keylen > 0 {
                unsafe { ptr::copy_nonoverlapping(key, rkey.as_mut_ptr(), keylen); }
            }
            // rest already zeroed
        } else {
            // rkey = SHA512(key), then pad to 128 with zeros
            let mut t = [0u8; 64];
            let mut sh = Sha512::new();
            unsafe { sh.write(key, keylen); }
            sh.finalize(&mut t);
            rkey[..64].copy_from_slice(&t);
            // rkey[64..] already zero
        }

        // outer key = rkey ^ 0x5c
        for n in 0..128 {
            rkey[n] ^= 0x5c;
        }
        let mut outer = Sha512::new();
        outer.write(rkey.as_ptr(), 128);

        // inner key = (rkey ^ 0x5c) ^ 0x36  == rkey ^ (0x5c ^ 0x36)
        for n in 0..128 {
            rkey[n] ^= 0x5c ^ 0x36;
        }
        let mut inner = Sha512::new();
        inner.write(rkey.as_ptr(), 128);

        HmacSha512 { outer, inner }
    }

    pub fn write(&mut self, data: *const u8, len: usize) -> &mut HmacSha512 {
        self.inner.write(data, len);
        self
    }

    pub fn finalize(&mut self, hash: &mut [u8; HMAC_SHA512_OUTPUT_SIZE]) {
        let mut temp = [0u8; 64];
        self.inner.finalize(&mut temp);
        self.outer.write(temp.as_ptr(), 64).finalize(hash);
    }

    pub fn finalize_to_array(mut self) -> [u8; 64] {
        let mut out = [0u8; 64];
        self.finalize(&mut out);
        out
    }
}
