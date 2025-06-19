// ---------------- [ File: bitcoin-chacha/src/chacha20.rs ]
crate::ix!();

/// ChaCha20 “expand 32‑byte k”/“expand 16‑byte k” constants
pub const SIGMA_BYTES: &[u8; 16] = b"expand 32-byte k";
pub const TAU_BYTES:   &[u8; 16] = b"expand 16-byte k";

//-------------------------------------------[.cpp/bitcoin/src/crypto/chacha20.h]
//-------------------------------------------[.cpp/bitcoin/src/crypto/chacha20.cpp]

/**
  | A class for ChaCha20 256-bit stream
  | cipher developed by Daniel J. Bernstein
  | https://cr.yp.to/chacha/chacha-20080128.pdf
  |
  */
#[derive(Clone,MutGetters,Debug,Getters)]
#[getset(get="pub",get_mut="pub")]
pub struct ChaCha20 {
    input: [u32; 16],
}

impl Default for ChaCha20 {
    fn default() -> Self {
        trace!("ChaCha20::default");
        Self { input: [0u32; 16] }
    }
}
    
impl ChaCha20 {
    
    pub fn new(k: *const u8, keylen: usize) -> Self {
        let mut c = Self::default();
        c.set_key(k, keylen);
        c
    }

    /**
      | set the 64bit nonce
      |
      */
    pub fn setiv(&mut self, iv: u64) {
        trace!(iv, "ChaCha20::setiv");
        self.input[14] = (iv & 0xFFFF_FFFF) as u32;
        self.input[15] = (iv >> 32)       as u32;
    }
    
    /**
      | set the 64bit block counter
      |
      */
    pub fn seek(&mut self, pos: u64) {
        trace!(pos, "ChaCha20::seek");
        self.input[12] = (pos & 0xFFFF_FFFF) as u32;
        self.input[13] = (pos >> 32)       as u32;
    }
}
