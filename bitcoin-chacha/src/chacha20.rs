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

#[cfg(test)]
mod chacha20_exhaustive_tests {
    use super::*;

    // RFC 8439 / DJB test vector (counter 0, nonce 0)
    const KEY:      [u8; 32] = [0u8; 32];
    const NONCE:    u64      = 0;
    const STREAM0:  [u8; 64] = [
        0x76,0xB8,0xE0,0xAD,0xA0,0xF1,0x3D,0x90,0x40,0x5D,0x6A,0xE5,0x53,0x86,0xBD,0x28,
        0xBD,0xD2,0x19,0xB8,0xA0,0x8D,0xED,0x1A,0xA8,0x36,0xEF,0xCC,0x8B,0x77,0x0D,0xC7,
        0xDA,0x41,0x59,0x7C,0x51,0x57,0x48,0x8D,0x77,0x24,0xE0,0x3F,0xB8,0xD8,0x4A,0x37,
        0x6A,0x43,0xB8,0xF4,0x15,0x18,0xA1,0x1C,0xC3,0x87,0xB6,0x69,0xB2,0xEE,0x65,0x86,
    ];

    #[traced_test]
    fn keystream_matches_reference() {
        let mut ks = [0u8; 64];
        let mut c  = ChaCha20::new(KEY.as_ptr(), 32);
        c.setiv(NONCE);
        c.seek(0);
        c.keystream(ks.as_mut_ptr(), ks.len());
        assert_eq!(&ks, &STREAM0, "first 64‑byte block must match spec");
    }

    #[traced_test]
    fn encrypt_then_decrypt_roundtrip() {
        let plaintext  = b"The quick brown fox jumps over the lazy dog";
        let mut cipher = ChaCha20::new(KEY.as_ptr(), 32);
        cipher.setiv(1234);

        let mut buf = plaintext.to_vec();
        let mut out = vec![0u8; buf.len()];
        cipher.crypt(buf.as_ptr(), out.as_mut_ptr(), buf.len());

        // decrypt
        let mut decipher = ChaCha20::new(KEY.as_ptr(), 32);
        decipher.setiv(1234);
        decipher.crypt(out.as_ptr(), buf.as_mut_ptr(), out.len());

        assert_eq!(&buf, plaintext, "round‑trip must restore plaintext");
    }

    #[traced_test]
    fn seek_advances_stream() {
        let mut c0 = ChaCha20::new(KEY.as_ptr(), 32);
        c0.setiv(NONCE);
        c0.seek(0);

        let mut c1 = ChaCha20::new(KEY.as_ptr(), 32);
        c1.setiv(NONCE);
        c1.seek(1); // skip first 64 bytes

        let mut block0 = [0u8; 64];
        let mut block1 = [0u8; 64];
        c0.keystream(block0.as_mut_ptr(), 64);
        c1.keystream(block1.as_mut_ptr(), 64);

        assert_ne!(block0, block1, "different counters yield different blocks");
    }
}
