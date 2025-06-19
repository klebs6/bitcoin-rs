// ---------------- [ File: bitcoin-aes/src/aes256_decrypt.rs ]
crate::ix!();

/**
  | A decryption class for AES-256.
  |
  */
#[derive(Default)]
pub struct AES256Decrypt {
    ctx: AES256_ctx,
}

impl Decrypt for AES256Decrypt {
    fn decrypt(&self, out: &mut [u8; 16], cipher: &[u8; 16]) {
        unsafe { aes256_decrypt(&self.ctx, 1, out.as_mut_ptr(), cipher.as_ptr()); }
    }
}

impl From<[u8; AES256_KEYSIZE]> for AES256Decrypt {
    /// Build a fully‑initialised `AES256Decrypt` from a 32‑byte raw key.
    ///
    /// The key schedule is expanded immediately via `aes256_init`,
    /// guaranteeing that the returned instance is ready for use in
    /// constant‑time decryption operations.
    fn from(key: [u8; AES256_KEYSIZE]) -> Self {
        tracing::info!(target: "aes", "AES256Decrypt::from – initialise (AES‑256)");

        let mut dec = Self::default();
        dec.init(key);

        tracing::debug!(target: "aes", "AES256Decrypt::from – ready");
        dec
    }
}

impl Drop for AES256Decrypt {

    fn drop(&mut self) {
        unsafe {
            libc::memset(
                &mut self.ctx as *mut AES256_ctx as *mut _, 
                0, 
                size_of_val(&self.ctx)
            );
        }
    }
}

impl AES256Decrypt {

    pub fn init(&mut self, key: [u8; 32]) {
    
        aes256_init(&mut self.ctx as *mut _, key.as_ptr() as *mut u8)
    }
}

#[cfg(test)]
mod aes256_decrypt_validation {
    use super::*;

    /// The internal `ctx` produced by [`AES256Decrypt::init`] must decrypt
    /// correctly when paired with the library’s low‑level
    /// [`aes256_encrypt`]/[`aes256_decrypt`](crate::aes256_decrypt) helpers.
    #[traced_test]
    fn decrypts_back_to_original_plaintext() {
        let mut rng = thread_rng();

        for _ in 0..2_000 {
            // --- random key + block -------------------------------------
            let mut key = [0u8; 32];
            let mut plain_in = [0u8; AES_BLOCKSIZE];
            rng.fill(&mut key);
            rng.fill(&mut plain_in);

            // --- produce ciphertext with the “raw” helper ----------------
            let mut enc_ctx = AES256_ctx::default();
            unsafe { aes256_init(&mut enc_ctx as *mut _, key.as_ptr()) };

            let mut cipher = [0u8; AES_BLOCKSIZE];
            unsafe { aes256_encrypt(&enc_ctx as *const _, 1, cipher.as_mut_ptr(), plain_in.as_ptr()) };

            // --- decrypt through the *type under test* -------------------
            let mut dec = AES256Decrypt::default();
            dec.init(key);

            let mut plain_out = [0u8; AES_BLOCKSIZE];
            unsafe {
                crate::aes256_decrypt(
                    &dec.ctx as *const _,
                    1,
                    plain_out.as_mut_ptr(),
                    cipher.as_ptr(),
                );
            }

            info!(target: "test", ?key, ?plain_in, ?cipher, ?plain_out, "AES‑256 decrypt round‑trip");
            assert_eq!(plain_out, plain_in, "AES‑256 decrypt mismatch");
        }
    }
}
