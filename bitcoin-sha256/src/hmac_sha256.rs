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
