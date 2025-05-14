// ---------------- [ File: bitcoin-random/src/dev_urandom.rs ]
crate::ix!();

/**
  | Fallback: get 32 bytes of system entropy
  | from /dev/urandom. The most compatible
  | way to get cryptographic randomness
  | on UNIX-ish platforms.
  |
  */
#[cfg(not_windows)]
pub fn get_dev_urandom(ent32: *mut u8)  {
    
    let f: i32 = unsafe {
        libc::open(
            "/dev/urandom".as_ptr() as *const i8, 
            libc::O_RDONLY)
    };

    if f == -1 {
        rand_failure();
    }

    let mut have: isize = 0;

    loop {

        let count: usize = 
            (NUM_OS_RANDOM_BYTES - (have as i32)).try_into().unwrap();

        let n: isize = unsafe {
            libc::read(
                f, 
                ent32.offset(have) as *mut c_void, 
                count)
        };

        if n <= 0 || (n + have) as i32 > NUM_OS_RANDOM_BYTES {

            unsafe {
                libc::close(f);
            }

            rand_failure();
        }

        have += n;

        if have as i32 >= NUM_OS_RANDOM_BYTES {
            break;
        }
    }

    unsafe {
        libc::close(f);
    }
}
