// ---------------- [ File: bitcoin-aes/src/cbc_encrypt.rs ]
crate::ix!();

pub fn cbc_encrypt<T: Encrypt>(
    enc:      &T,
    iv:       [u8; AES_BLOCKSIZE],
    mut data: *const u8,
    size:     i32,
    pad:      bool,
    out:      *mut u8

) -> i32 {

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
