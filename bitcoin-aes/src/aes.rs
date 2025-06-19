// ---------------- [ File: bitcoin-aes/src/aes.rs ]
/*!
 | Constant time, unoptimized, concise, plain C,
 | AES implementation Based On:
 |
 |   Emilia Kasper and Peter Schwabe, Faster and
 |   Timing-Attack Resistant AES-GCM
 |   http://www.iacr.org/archive/ches2009/57470001/57470001.pdf
 |
 | But using 8 16-bit integers representing
 | a single AES state rather than 8 128-bit
 | integers representing 8 AES states.
 |
 |
 | Slice variable slice_i contains the i'th bit of
 | the 16 state variables in this order:
 |
 |  0  1  2  3
 |  4  5  6  7
 |  8  9 10 11
 | 12 13 14 15
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

#[cfg(test)]
mod aes_constants_validation {
    use super::*;

    /// Ensure public constants match the AES specification.
    #[traced_test]
    fn constants_match_fips_197() {
        debug!(target: "test", AES_BLOCKSIZE, AES256_KEYSIZE);
        assert_eq!(AES_BLOCKSIZE, 16, "AES block size must be 128 bits (16 bytes)");
        assert_eq!(AES256_KEYSIZE, 32, "AES‑256 key size must be 256 bits (32 bytes)");
    }
}
