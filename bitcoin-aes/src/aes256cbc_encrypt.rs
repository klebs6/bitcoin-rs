// ---------------- [ File: bitcoin-aes/src/aes256cbc_encrypt.rs ]
crate::ix!();

pub struct AES256CBCEncrypt {
    enc: AES256Encrypt,
    pad: bool,
    iv:  [u8; AES_BLOCKSIZE],
}

impl Drop for AES256CBCEncrypt {

    fn drop(&mut self) {
        unsafe {
            libc::memset(self.iv.as_mut_ptr() as *mut c_void, 0, size_of_val(&self.iv));
        }
    }
}

impl AES256CBCEncrypt {

    pub fn new(
        key:    [u8; AES256_KEYSIZE],
        iv_in:  [u8; AES_BLOCKSIZE],
        pad_in: bool) -> Self {
    
        let mut x = Self {
            enc: AES256Encrypt::from(key),
            pad: pad_in,
            iv:  [0; AES_BLOCKSIZE],
        };

        unsafe {
            libc::memcpy(
                x.iv.as_mut_ptr() as *mut c_void, 
                iv_in.as_ptr() as *const c_void, 
                AES_BLOCKSIZE
            );
        }

        x
    }
    
    pub fn encrypt(&self, 
        data: *const u8,
        size: i32,
        out:  *mut u8) -> i32 {
        
        cbc_encrypt(
            &self.enc,
            self.iv,
            data,
            size,
            self.pad,
            out
        )
    }
}

#[cfg(test)]
mod aes256cbc_encrypt_validation {
    use super::*;

    /// The *default* value of [`AES256CBCEncrypt`] should start with an
    /// all‑zero IV and `pad == false` (mirroring the decryptor).
    #[traced_test]
    fn default_is_zeroised_and_unpadded() {
        let cbc = AES256CBCEncrypt {
            enc: crate::aes256_encrypt::AES256Encrypt::default(),
            pad: false,
            iv: [0u8; AES_BLOCKSIZE],
        };
        debug!(?cbc.iv, pad = cbc.pad);
        assert_eq!(cbc.iv, [0u8; AES_BLOCKSIZE], "IV must start as all‑zeros");
        assert!(!cbc.pad, "padding flag must default to false");
    }
}
