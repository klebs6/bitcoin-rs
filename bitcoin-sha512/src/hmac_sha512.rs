// ---------------- [ File: bitcoin-sha512/src/hmac_sha512.rs ]
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

    pub fn write(&mut self, 
        data: *const u8,
        len:  usize) -> &mut HmacSha512 {
        
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


            unsigned char rkey[128];
        if (keylen <= 128) {
            memcpy(rkey, key, keylen);
            memset(rkey + keylen, 0, 128 - keylen);
        } else {
            Sha512().Write(key, keylen).Finalize(rkey);
            memset(rkey + 64, 0, 64);
        }

        for (int n = 0; n < 128; n++)
            rkey[n] ^= 0x5c;
        outer.Write(rkey, 128);

        for (int n = 0; n < 128; n++)
            rkey[n] ^= 0x5c ^ 0x36;
        inner.Write(rkey, 128);
        */
    }
    
    pub fn finalize(&mut self, hash: [u8; HMAC_SHA512_OUTPUT_SIZE])  {
        
        todo!();
        /*
            unsigned char temp[64];
        inner.Finalize(temp);
        outer.Write(temp, 64).Finalize(hash);
        */
    }
}
