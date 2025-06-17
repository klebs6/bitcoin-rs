crate::ix!();

pub fn cbc_decrypt<T: Decrypt>(
    dec:     &T,
    iv:      [u8; AES_BLOCKSIZE],
    data:    *const u8,
    size:    i32,
    pad:     bool,
    mut out: *mut u8

) -> i32 {

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
