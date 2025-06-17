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

    fn decrypt(&self, 
        mut plaintext: [u8; 16],
        ciphertext:    [u8; 16])  {
        
        aes256_decrypt(&self.ctx, 1, plaintext.as_mut_ptr(), ciphertext.as_ptr());
    }
}

impl From<[u8; AES256_KEYSIZE]> for AES256Decrypt {
    fn from(x: [u8; AES256_KEYSIZE]) -> Self {
        todo!();
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
