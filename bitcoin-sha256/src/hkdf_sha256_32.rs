// ---------------- [ File: bitcoin-sha256/src/hkdf_sha256_32.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/hkdf_sha256_32.h]

/**
  | A rfc5869 HKDF implementation with
  | HMAC_SHA256 and fixed key output length
  | of 32 bytes (L=32)
  |
  */
pub struct CHKDF_HMAC_SHA256_L32 {
    prk: [u8; 32],
}

pub const CHKDF_HMAC_SHA256_L32_OUTPUT_SIZE: usize = 32;

//-------------------------------------------[.cpp/bitcoin/src/crypto/hkdf_sha256_32.cpp]
impl CHKDF_HMAC_SHA256_L32 {

    pub fn new(
        ikm:    *const u8,
        ikmlen: usize,
        salt:   &String) -> Self {
    
        todo!();
        /*
            CHMAC_SHA256((const unsigned char*)salt.data(), salt.size()).Write(ikm, ikmlen).Finalize(m_prk);
        */
    }
    
    pub fn expand32(&mut self, 
        info: &String,
        hash: [u8; CHKDF_HMAC_SHA256_L32_OUTPUT_SIZE])  {
        
        todo!();
        /*
            // expand a 32byte key (single round)
        assert(info.size() <= 128);
        static const unsigned char one[1] = {1};
        CHMAC_SHA256(m_prk, 32).Write((const unsigned char*)info.data(), info.size()).Write(one, 1).Finalize(hash);
        */
    }
}
