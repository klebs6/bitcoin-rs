// ---------------- [ File: bitcoin-random/src/get_os_rand.rs ]
crate::ix!();

/**
  | Get 32 bytes of system entropy. Do not
  | use this in application code: use
  | 
  | GetStrongRandBytes instead.
  |
  */
#[cfg(windows)]
pub fn get_os_rand(ent32: *mut u8)  {

    let hProvider = winapi::wincrypt::HCRYPTPROV::default();

    let ret: i32 = CryptAcquireContextW(
        &hProvider, 
        null_mut(), 
        null_mut(), 
        PROV_RSA_FULL, 
        CRYPT_VERIFYCONTEXT
    );

    if ret == 0 {
        rand_failure();
    }

    ret = crypt_gen_random(
        hProvider, 
        NUM_OS_RANDOM_BYTES, 
        ent32);

    if ret == 0 {
        rand_failure();
    }

    crypt_release_context(hProvider, 0);
}

#[cfg(HAVE_SYS_GETRANDOM)]
pub fn get_os_rand(ent32: *mut u8)  {

    /*
       | Linux. From the getrandom(2) man page:
       | "If the urandom source has been initialized,
       | reads of up to 256 bytes will always return
       | as many bytes as requested and will not
       | be interrupted by signals."
       |
       */
    let rv: i32 = libc::syscall(
        libc::SYS_getrandom, 
        ent32, 
        NUM_OS_RANDOM_BYTES, 
        0);

    if rv != NUM_OS_RANDOM_BYTES {

        if rv < 0 && libc::errno() == libc::ENOSYS {

            /*
              | Fallback for kernel <3.17: the return
              | value will be -1 and errno ENOSYS if the
              | syscall is not available, in that case
              | fall back to /dev/urandom.
              |
              */
            get_dev_urandom(ent32);

        } else {
            rand_failure();
        }
    }
}

#[cfg(all(HAVE_GETENTROPY,openbsd))]
pub fn get_os_rand(ent32: *mut u8)  {

    /*
      | On OpenBSD this can return up to 256 bytes
      | of entropy, will return an error if more
      | are requested.
      | 
      | The call cannot return less than the
      | requested number of bytes. getentropy
      | is explicitly limited to openbsd here,
      | as a similar (but not the same) function
      | may exist on other platforms via glibc.
      |
      */
    if libc::getentropy(ent32, NUM_OS_RANDOM_BYTES) != 0 {
        rand_failure();
    }
}

#[cfg(all(HAVE_GETENTROPY_RAND,MAC_OSX))]
pub fn get_os_rand(ent32: *mut u8)  {

    /*
      | libc::getentropy() is available on macOS
      | 10.12 and later.
      |
      */
    if libc::getentropy(ent32, NUM_OS_RANDOM_BYTES) != 0 {
        rand_failure();
    }
}

#[cfg(HAVE_SYSCTL_ARND)]
pub fn get_os_rand(ent32: *mut u8)  {

    /*
      | FreeBSD, NetBSD and similar. It is possible
      | for the call to return less bytes than
      | requested, so need to read in a loop.
      |
      */
    static mut name: [i32; 2] = [libc::CTL_KERN, libc::KERN_ARND];

    let have: i32 = 0;

    loop {

        let len: usize = (NUM_OS_RANDOM_BYTES - have).try_into().unwrap();

        if libc::sysctl(
            name.as_mut_ptr(), 
            std::size(name), 
            ent32.offset(have.try_into().unwrap()), 
            &mut len, 
            null_mut(), 
            0) != 0 
        {
            rand_failure();
        }

        have += len as i32;

        if have >= NUM_OS_RANDOM_BYTES {
            break;
        }
    }
}

/**
   | Fall back to /dev/urandom if there is
   | no specific method implemented to get
   | system entropy for this OS.
   |
   */
#[cfg(not(any(
    windows,
    HAVE_SYS_GETRANDOM,
    all(HAVE_GETENTROPY,openbsd),
    all(HAVE_GETENTROPY_RAND,MAC_OSX),
    HAVE_SYSCTL_ARND,
)))]
pub fn get_os_rand(ent32: *mut u8)  {

    get_dev_urandom(ent32);
}

#[cfg(test)]
mod get_os_rand_spec {
    use super::*;

    #[traced_test]
    #[cfg(not(windows))]
    fn get_os_rand_produces_32_bytes_and_varies() {
        let mut a = [0u8; NUM_OS_RANDOM_BYTES as usize];
        let mut b = [0u8; NUM_OS_RANDOM_BYTES as usize];

        get_os_rand(a.as_mut_ptr());
        get_os_rand(b.as_mut_ptr());

        assert_eq!(a.len(), 32);
        assert_ne!(a, [0u8; 32], "first OS rand draw was all zeros");
        assert_ne!(a, b, "two OS rand draws were identical (extremely unlikely)");
    }
}
