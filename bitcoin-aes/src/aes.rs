// ---------------- [ File: bitcoin-aes/src/aes.rs ]
/*!
  | C++ wrapper around ctaes, a constant-time
  | AES implementation
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/aes.h]

pub const AES_BLOCKSIZE:  usize = 16;
pub const AES256_KEYSIZE: usize = 32;

//-------------------------------------------[.cpp/bitcoin/src/crypto/aes.cpp]

pub trait Decrypt {

    fn decrypt(&self, 
        plaintext:  [u8; 16],
        ciphertext: [u8; 16]);
}

pub trait Encrypt {

    fn encrypt(&self, 
        ciphertext: [u8; 16],
        plaintext:  [u8; 16]);
}

