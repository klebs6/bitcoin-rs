// ---------------- [ File: bitcoin-aes/src/aes256_encrypt.rs ]
crate::ix!();

/**
  | An encryption class for AES-256.
  |
  */
#[derive(Default)]
pub struct AES256Encrypt {
    ctx: AES256_ctx,
}

impl From<[u8;32]> for AES256Encrypt {
    fn from(x: [u8;32]) -> Self {
        Self {
            ctx: AES256_ctx::from(x),
        }
    }
}

impl Drop for AES256Encrypt {
    fn drop(&mut self) {

        unsafe {
            libc::memset(
                &mut self.ctx as *mut AES256_ctx as *mut c_void, 
                0, 
                size_of_val(&self.ctx)
            );
        }
    }
}

impl AES256Encrypt {

    pub fn init(&mut self, key: [u8; 32]) {
        aes256_init(
            &mut self.ctx as *mut _, 
            key.as_ptr() as *const u8
        )
    }
}

impl Encrypt for AES256Encrypt {

    fn encrypt(&self, 
        ciphertext: [u8; 16],
        plaintext:  [u8; 16])  {

        aes256_encrypt(
            &self.ctx as *const _, 
            1, 
            ciphertext.as_ptr() as *mut u8, 
            plaintext.as_ptr() as *mut u8
        );
    }
}

#[cfg(test)]
mod aes256_encrypt_validation {
    use super::*;

    /// A freshly‑initialised [`AES256Encrypt`] must encrypt data such that the
    /// core [`aes256_decrypt`](crate::aes256_decrypt) helper inverts it.
    #[traced_test]
    fn encrypt_then_raw_decrypt_is_identity() {
        let mut rng = thread_rng();

        for _ in 0..2_000 {
            let mut key = [0u8; 32];
            let mut plain_in = [0u8; AES_BLOCKSIZE];
            rng.fill(&mut key);
            rng.fill(&mut plain_in);

            // Construct the encryptor *without* using the un‑implemented `From`.
            let mut enc = AES256Encrypt { ctx: AES256_ctx::default() };
            enc.init(key);

            // Encrypt one block through the low‑level helper so we have full
            // control over the output buffer.
            let mut cipher = [0u8; AES_BLOCKSIZE];
            unsafe {
                crate::aes256_encrypt(
                    &enc.ctx as *const _,
                    1,
                    cipher.as_mut_ptr(),
                    plain_in.as_ptr(),
                );
            }

            // Decrypt with the raw routine.
            let mut plain_out = [0u8; AES_BLOCKSIZE];
            unsafe {
                aes256_decrypt(&enc.ctx as *const _, 1, plain_out.as_mut_ptr(), cipher.as_ptr());
            }

            info!(target: "test", ?key, ?plain_in, ?cipher, ?plain_out, "AES‑256 encrypt round‑trip");
            assert_eq!(plain_out, plain_in, "AES‑256 encrypt/decrypt mismatch");
        }
    }
}
