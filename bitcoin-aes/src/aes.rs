/*!
  | C++ wrapper around ctaes, a constant-time
  | AES implementation
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/aes.h]

pub const AES_BLOCKSIZE:  usize = 16;
pub const AES256_KEYSIZE: usize = 32;

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

///--------------------------
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

///--------------------------
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

//-------------------------------------------[.cpp/bitcoin/src/crypto/aes.cpp]

pub fn cbc_encrypt<T: Encrypt>(
        enc:      &T,
        iv:       [u8; AES_BLOCKSIZE],
        mut data: *const u8,
        size:     i32,
        pad:      bool,
        out:      *mut u8) -> i32 {

    let mut written: i32 = 0;

    let padsize: i32 = size % i32::try_from(AES_BLOCKSIZE).unwrap();

    let mut mixed: [u8; AES_BLOCKSIZE] = [0; AES_BLOCKSIZE];

    if data == null_mut() || size == 0 || out == null_mut() {
        return 0;
    }

    if !pad && padsize != 0 {
        return 0;
    }

    unsafe {
        libc::memcpy(
            mixed.as_mut_ptr() as *mut c_void, 
            iv.as_ptr() as *const c_void, 
            AES_BLOCKSIZE
        );
    }

    // Write all but the last block
    while written + i32::try_from(AES_BLOCKSIZE).unwrap() <= size {

        let mut i: usize = 0;

        while i != AES_BLOCKSIZE.try_into().unwrap() {

            unsafe {
                mixed[i] ^= *{

                    let old = data;

                    data = data.add(1);

                    old
                };
            }

            i += 1;
        }

        unsafe {
            enc.encrypt(
                std::slice::from_raw_parts(out.offset(written as isize), 16)
                    .try_into()
                    .unwrap(), 
                mixed
            );

            libc::memcpy(
                mixed.as_mut_ptr() as *mut c_void, 
                out.offset(written as isize) as *const c_void, 
                AES_BLOCKSIZE
            );
        }

        written += i32::try_from(AES_BLOCKSIZE).unwrap();
    }

    if pad {

        // For all that remains, pad each byte
        // with the value of the remaining
        // space. If there is none, pad by a full
        // block.
        let mut i: usize = 0;

        while i != padsize.try_into().unwrap() {

            unsafe {
                mixed[i] ^= *{
                    let old = data;

                    data = data.add(1);

                    old
                };
            }

            i += 1;
        }

        let mut i: usize = padsize.try_into().unwrap();

        while i != AES_BLOCKSIZE.try_into().unwrap() {

            mixed[i] ^= u8::try_from(i32::try_from(AES_BLOCKSIZE).unwrap() - padsize).unwrap();

            i += 1;
        }

        unsafe {
            enc.encrypt(
                std::slice::from_raw_parts(out.offset(written as isize), 16)
                    .try_into()
                    .unwrap(), 
                mixed
            );
        }

        written += i32::try_from(AES_BLOCKSIZE).unwrap();
    }

    written
}

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

pub fn cbc_decrypt<T: Decrypt>(
        dec:     &T,
        iv:      [u8; AES_BLOCKSIZE],
        data:    *const u8,
        size:    i32,
        pad:     bool,
        mut out: *mut u8) -> i32 {

    let mut written: i32 = 0;

    let mut fail: bool = false;

    let mut prev: *const u8 = iv.as_ptr();

    if data == null_mut() || size == 0 || out == null_mut() {
        return 0;
    }

    if size % i32::try_from(AES_BLOCKSIZE).unwrap() != 0 {
        return 0;
    }

    // Decrypt all data. Padding will be checked
    // in the output.
    while written != size {

        unsafe {
            dec.decrypt(

                std::slice::from_raw_parts(out, 16)
                    .try_into()
                    .unwrap(), 

                std::slice::from_raw_parts(
                    data.offset(written as isize), 
                    16
                ).try_into().unwrap()
            );
        }

        let mut i: usize = 0;

        while i != AES_BLOCKSIZE.try_into().unwrap() {

            unsafe {
                *{
                    let old = out;

                    out = out.add(1);

                    old

                } ^= *prev.add(i);
            }

            i += 1;
        }

        unsafe {
            prev = data.offset(written.try_into().unwrap());
        }

        written += i32::try_from(AES_BLOCKSIZE).unwrap();
    }

    // When decrypting padding, attempt to run in
    // constant-time
    if pad {

        // If used, padding size is the value of
        // the last decrypted byte. For it to be
        // valid, It must be between 1 and
        // AES_BLOCKSIZE.
        let mut padsize: i8 = unsafe {

            i8::try_from(*{

                out = out.offset(-1);

                out

            }).unwrap()
        };

        fail = {

            let bigpad = padsize > AES_BLOCKSIZE.try_into().unwrap();

            (padsize == 0) | bigpad 
        };

        // If not well-formed, treat it as though
        // there's no padding.
        padsize *= match fail { true => 0, false => 1 };

        // All padding must equal the last byte
        // otherwise it's not well-formed
        let mut i: i32 = AES_BLOCKSIZE.try_into().unwrap();

        while i != 0 {

            unsafe {
                fail |= ((i > (AES_BLOCKSIZE - usize::try_from(padsize).unwrap()).try_into().unwrap()) & (*{
                    let old = out;

                    unsafe {
                        out = out.offset(-1);
                    }

                    old
                } != padsize.try_into().unwrap()));
            }

            i -= 1;
        }

        written -= padsize as i32;
    }

    written * match fail { true => 0, false => 1 }
}
