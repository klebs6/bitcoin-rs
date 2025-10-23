// ---------------- [ File: bitcoin-random/src/randomenv.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/randomenv.h]
//-------------------------------------------[.cpp/bitcoin/src/randomenv.cpp]

pub const TEN_MINUTES:     Duration = Duration::seconds(10 * 60);
pub const ONE_MILLISECOND: Duration = Duration::milliseconds(1);

/**
  | Seed with the entire set of perfmon data
  |
  */
#[cfg(WIN32)]
pub fn rand_add_seed_perfmon(hasher: &mut Sha512)  {

    /*
       | This can take up to 2 seconds, so only
       | do it every 10 minutes.
       |
       | Initialize last_perfmon to 0 seconds,
       | we don't skip the first call.
       */
    lazy_static!{
        static ref last_perfmon: Mutex<Duration> = Mutex::new(Duration::ZERO);
    }

    let last_time:    Duration = *last_perfmon.lock().unwrap();
    let current_time: Duration = get_time();

    if current_time < last_time + TEN_MINUTES {
        return;
    }

    *last_perfmon.lock().unwrap() = current_time;

    //-------------------------------------

    let v_data: Vec<u8> = Vec::with_capacity(250000);

    let ret:    i64 = 0;
    let n_size: u64 = 0;

    // Bail out at more than 10MB of performance data
    const n_max_size: usize = 10000000; 

    while true {

        n_size = v_data.len().try_into().unwrap();

        ret = winreg::RegQueryValueExA(
            winreg::HKEY_PERFORMANCE_DATA, 
            "Global", 
            null_mut(), 
            null_mut(), 
            v_data.as_mut_ptr(), 
            &n_size);

        if ret != winerror::ERROR_MORE_DATA || v_data.len() >= n_max_size {
            break;
        }

        // Grow size of buffer exponentially
        v_data.resize(
            std::cmp::min((v_data.len() * 3) / 2, n_max_size),
            0
        ); 
    }

    winreg::RegCloseKey(winreg::HKEY_PERFORMANCE_DATA);

    if ret == winerror::ERROR_SUCCESS {

        hasher.write(
            v_data.as_mut_ptr(), 
            n_size.try_into().unwrap());

        memory_cleanse(
            v_data.as_mut_ptr() as *mut c_void, 
            n_size.try_into().unwrap());

    } else {

        // Performance data is only a best-effort
        // attempt at improving the situation when
        // the OS randomness (and other sources)
        // aren't adequate. As a result, failure
        // to read it is isn't considered
        // critical, so we don't call
        // RandFailure().
        //
        // TODO: Add logging when the logger is
        // made functional before global
        // constructors have been invoked.
    }
}

#[cfg(not(WIN32))]
pub fn add_sockaddr(
        hasher: &mut Sha512,
        addr:   *const nix::sys::socket::sockaddr)  {

    if addr == null_mut() {
        return;
    }

    unsafe {
        match (*addr).sa_family {

            AF_INET  => hasher.write(
                addr as *mut u8, size_of::<libc::sockaddr_in>()
            ),

            AF_INET6 => hasher.write(
                addr as *mut u8, size_of::<libc::sockaddr_in6>()
            ),

            _ => hasher.write(
                &(*addr).sa_family as *const _ as *const u8, 
                size_of_val(&(*addr).sa_family)
            )
        };
    }
}

#[cfg(not(WIN32))]
pub fn add_file(
        hasher: &mut Sha512,
        path:   *const i8)  {

    let mut sb: libc::stat = unsafe { std::mem::zeroed() };

    let f: i32 = unsafe { libc::open(path, libc::O_RDONLY) };

    let mut total: usize = 0;

    if f != -1 {

        let mut fbuf: [u8; 4096] = [0; 4096];

        let mut n: i32 = 0;

        hasher.write(
            &f as *const _ as *const u8, 
            size_of_val(&f)
        );

        if unsafe { libc::fstat(f, &mut sb) } == 0 {
            hasher.feed_data_in(&mut sb);
        }

        loop {

            n = unsafe {

                libc::read(
                    f, 
                    fbuf.as_mut_ptr() as *mut c_void, 
                    size_of_val(&fbuf)
                ).try_into().unwrap()
            };

            if n > 0 {
                hasher.write(fbuf.as_mut_ptr(), n.try_into().unwrap());
            }

            let offset: usize = n.try_into().unwrap();

            total += offset;

            /* not bothering with EINTR handling. */

            /*
              | Read only the first 1 Mbyte
              |
              */
            let onward: bool = 
                n == (size_of_val(&fbuf) as i32) && total < 1048576; 

            if !onward {
                break;
            }
        } 

        unsafe {
            libc::close(f);
        }
    }
}

#[cfg(not(WIN32))]
pub fn add_path(
        hasher: &mut Sha512,
        path:   *const u8)  {
    
    let mut sb: libc::stat = unsafe { std::mem::zeroed() };

    if unsafe { libc::stat(path as *const i8, &mut sb) } == 0 {

        let len = unsafe { libc::strlen(path as *const i8) + 1 };

        hasher.write(path, len);
        hasher.feed_data_in(&mut sb);
    }
}

#[cfg(HAVE_SYSCTL)]
pub fn add_sysctl<const S: i32>(hasher: &mut Sha512)  {

    todo!();
    /*
    int CTL[sizeof...(S)] = {S...};
    unsigned char buffer[65536];
    size_t siz = 65536;
    int ret = sysctl(CTL, sizeof...(S), buffer, &siz, nullptr, 0);
    if (ret == 0 || (ret == -1 && errno == ENOMEM)) {
        hasher << sizeof(CTL);
        hasher.Write((const unsigned char*)CTL, sizeof(CTL));
        if (siz > sizeof(buffer)) siz = sizeof(buffer);
        hasher << siz;
        hasher.Write(buffer, siz);
    }
    */
}

#[cfg(have_getcpuid)]
#[inline] pub fn addcpuid(
        hasher:  &mut Sha512,
        leaf:    u32,
        subleaf: u32,
        ax:      &mut u32,
        bx:      &mut u32,
        cx:      &mut u32,
        dx:      &mut u32)  {
    
    getcpuid(leaf, subleaf, ax, bx, cx, dx);

    *hasher <<= leaf;
    *hasher <<= subleaf;
    *hasher <<= ax;
    *hasher <<= bx;
    *hasher <<= cx;
    *hasher <<= dx;
}

#[cfg(have_getcpuid)]
pub fn add_allcpuid(hasher: &mut Sha512)  {

    let mut ax: u32 = 0;
    let mut bx: u32 = 0;
    let mut cx: u32 = 0;
    let mut dx: u32 = 0;

    /*
      | Iterate over all standard leaves
      |
      */
    addcpuid(
        hasher, 
        0, 
        0, 
        &mut ax, 
        &mut bx, 
        &mut cx, 
        &mut dx
    ); // Returns max leaf in ax

    let max: u32 = ax;

    for leaf in 1..=std::cmp::min(max,0xFF) {

        let mut maxsub: u32 = 0;

        for subleaf in 0..=0xFF {

            addcpuid(
                hasher, 
                leaf, 
                subleaf, 
                &mut ax, 
                &mut bx, 
                &mut cx, 
                &mut dx);

            /*
              | Iterate subleafs for leaf values 4,
              | 7, 11, 13
              |
              */
            if leaf == 4 {

                if (ax & 0x1f) == 0 {
                    break;
                }

            } else if leaf == 7 {

                if subleaf == 0 {
                    maxsub = ax;
                }

                if subleaf == maxsub {
                    break;
                }

            } else if leaf == 11 {

                if (cx & 0xff00) == 0 {
                    break;
                }

            } else if leaf == 13 {

                if ax == 0 
                    && bx == 0 
                    && cx == 0 
                    && dx == 0 
                {
                    break;
                }

            } else {

                /*
                  | For any other leaf, stop after subleaf
                  | 0.
                  |
                  */
                break;
            }
        }
    }

    // Iterate over all extended leaves
    addcpuid(hasher, 
        0x80000000, 
        0, 
        &mut ax, 
        &mut bx, 
        &mut cx, 
        &mut dx); // Returns max extended leaf in ax

    let ext_max: u32 = ax;

    for leaf in 0x80000001..=std::cmp::min(ext_max,0x800000FF) {

        addcpuid(
            hasher, 
            leaf, 
            0, 
            &mut ax, 
            &mut bx, 
            &mut cx, 
            &mut dx
        );
    }
}

#[cfg(test)]
mod randomenv_spec {
    use super::*;

    #[traced_test]
    #[cfg(not(WIN32))]
    fn add_sockaddr_accepts_null_and_specific_families() {
        let mut hasher = Sha512::default();
        // null is a no-op
        add_sockaddr(&mut hasher, core::ptr::null());

        // IPv4 sockaddr
        let mut v4: libc::sockaddr_in = unsafe { core::mem::zeroed() };
        v4.sin_family = libc::AF_INET as _;
        let before = hasher.size();
        add_sockaddr(&mut hasher, &v4 as *const _ as *const libc::sockaddr);
        assert!(hasher.size() >= before);
    }

    #[traced_test]
    #[cfg(not(WIN32))]
    fn add_file_and_add_path_are_best_effort() {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;
        let tmp = tempfile::NamedTempFile::new().expect("tmpfile");
        std::fs::write(tmp.path(), b"hello world").unwrap();

        let mut hasher = Sha512::default();
        let before = hasher.size();

        // add_file takes *const i8 (C string)
        let cpath = CString::new(tmp.path().as_os_str().as_bytes()).unwrap();
        add_file(&mut hasher, cpath.as_ptr());

        let after_file = hasher.size();
        assert!(after_file >= before);

        // add_path takes *const u8
        let cpath_u8 = CString::new(&b"/"[..]).unwrap();
        let before2 = hasher.size();
        add_path(&mut hasher, cpath_u8.as_ptr() as *const u8);
        let after2 = hasher.size();
        assert!(after2 >= before2);
    }
}
