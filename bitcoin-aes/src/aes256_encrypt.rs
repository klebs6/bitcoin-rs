// ---------------- [ File: bitcoin-aes/src/aes256_encrypt.rs ]
crate::ix!();

/**
  | An encryption class for AES-256.
  |
  */
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
