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
        libc::open(b"/dev/urandom\0".as_ptr() as *const i8, libc::O_RDONLY)
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

#[cfg(test)]
mod dev_urandom_spec {
    use super::*;

    #[traced_test]
    #[cfg(not_windows)]
    fn fills_exactly_32_bytes_and_changes_between_calls() {
        let mut a = [0u8; NUM_OS_RANDOM_BYTES as usize];
        let mut b = [0u8; NUM_OS_RANDOM_BYTES as usize];

        // Two independent draws from the OS RNG.
        get_dev_urandom(a.as_mut_ptr());
        get_dev_urandom(b.as_mut_ptr());

        // Sanity: wrote 32 bytes (shape check)
        assert_eq!(a.len(), 32);
        assert_eq!(b.len(), 32);

        // Almost-surely not all zeros (if it is, subsequent equality check will also fail)
        assert_ne!(a, [0u8; 32], "first draw from /dev/urandom was all zeros — indicates OS RNG failure");

        // Vanishingly unlikely: two identical 32-byte draws.
        assert_ne!(a, b, "two /dev/urandom draws were identical (2^-256 chance); re-run if fluke");
    }

    #[traced_test]
    fn fill_bytes_fills_entire_slice() {
        let mut ctx = FastRandomContext::new(true);
        let mut buf = [0u8; 64];
        ctx.fill_bytes(&mut buf);
        assert!(buf.iter().any(|&x| x != 0));      // not all zeros
        assert!(buf.iter().filter(|&&x| x == 0).count() < 64); // very likely not all zeros
    }
}
