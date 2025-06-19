// ---------------- [ File: bitcoin-aes/src/aes256cbc_decrypt.rs ]
crate::ix!();

#[derive(Default)]
pub struct AES256CBCDecrypt {
    dec: AES256Decrypt,
    pad: bool,
    iv:  [u8; AES_BLOCKSIZE],
}

impl Drop for AES256CBCDecrypt {
    fn drop(&mut self) {

        unsafe {
            libc::memset(
                self.iv.as_mut_ptr() as *mut c_void, 
                0, 
                size_of_val(&self.iv)
            );
        }
    }
}

impl AES256CBCDecrypt {

    pub fn new(
        key:    [u8; AES256_KEYSIZE],
        iv_in:  [u8; AES_BLOCKSIZE],
        pad_in: bool) -> Self {
    
        let mut x = Self {
            dec: AES256Decrypt::from(key),
            pad: pad_in,
            ..Default::default()
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
    
    pub fn decrypt(&self, 
        data: *const u8,
        size: i32,
        out:  *mut u8) -> i32 {
        
        cbc_decrypt(
            &self.dec,
            self.iv,
            data,
            size,
            self.pad,
            out
        )
    }
}

#[cfg(test)]
mod aes256cbc_decrypt_validation {
    use super::*;

    /// The *default* value of [`AES256CBCDecrypt`] must be a clean slate
    /// (zero IV, padding `false`) so that callers cannot inherit stale data.
    #[traced_test]
    fn default_is_zeroised_and_unpadded() {
        let cbc = AES256CBCDecrypt::default();
        debug!(?cbc.iv, pad = cbc.pad);
        assert_eq!(cbc.iv, [0u8; AES_BLOCKSIZE], "IV must start as all‑zeros");
        assert!(!cbc.pad, "padding flag must default to false");
    }
}
