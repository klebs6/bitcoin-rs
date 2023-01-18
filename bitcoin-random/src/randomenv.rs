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

/**
  | Gather non-cryptographic environment
  | data that changes over time.
  |
  */
pub fn rand_add_dynamic_env(hasher: &mut Sha512)  {
    
    todo!();
        /*
            RandAddSeedPerfmon(hasher);

        // Various clocks
    #ifdef WIN32
        FILETIME ftime;
        GetSystemTimeAsFileTime(&ftime);
        hasher << ftime;
    #else
        struct timespec ts = {};
    #    ifdef CLOCK_MONOTONIC
        clock_gettime(CLOCK_MONOTONIC, &ts);
        hasher << ts;
    #    endif
    #    ifdef CLOCK_REALTIME
        clock_gettime(CLOCK_REALTIME, &ts);
        hasher << ts;
    #    endif
    #    ifdef CLOCK_BOOTTIME
        clock_gettime(CLOCK_BOOTTIME, &ts);
        hasher << ts;
    #    endif
        // gettimeofday is available on all UNIX systems, but only has microsecond precision.
        struct timeval tv = {};
        gettimeofday(&tv, nullptr);
        hasher << tv;
    #endif
        // Probably redundant, but also use all the clocks C++11 provides:
        hasher << std::chrono::system_clock::now().time_since_epoch().count();
        hasher << std::chrono::steady_clock::now().time_since_epoch().count();
        hasher << std::chrono::high_resolution_clock::now().time_since_epoch().count();

    #ifndef WIN32
        // Current resource usage.
        struct rusage usage = {};
        if (getrusage(RUSAGE_SELF, &usage) == 0) hasher << usage;
    #endif

    #ifdef __linux__
        AddFile(hasher, "/proc/diskstats");
        AddFile(hasher, "/proc/vmstat");
        AddFile(hasher, "/proc/schedstat");
        AddFile(hasher, "/proc/zoneinfo");
        AddFile(hasher, "/proc/meminfo");
        AddFile(hasher, "/proc/softirqs");
        AddFile(hasher, "/proc/stat");
        AddFile(hasher, "/proc/self/schedstat");
        AddFile(hasher, "/proc/self/status");
    #endif

    #if HAVE_SYSCTL
    #  ifdef CTL_KERN
    #    if defined(KERN_PROC) && defined(KERN_PROC_ALL)
        AddSysctl<CTL_KERN, KERN_PROC, KERN_PROC_ALL>(hasher);
    #    endif
    #  endif
    #  ifdef CTL_HW
    #    ifdef HW_DISKSTATS
        AddSysctl<CTL_HW, HW_DISKSTATS>(hasher);
    #    endif
    #  endif
    #  ifdef CTL_VM
    #    ifdef VM_LOADAVG
        AddSysctl<CTL_VM, VM_LOADAVG>(hasher);
    #    endif
    #    ifdef VM_TOTAL
        AddSysctl<CTL_VM, VM_TOTAL>(hasher);
    #    endif
    #    ifdef VM_METER
        AddSysctl<CTL_VM, VM_METER>(hasher);
    #    endif
    #  endif
    #endif

        // Stack and heap location
        c_void* addr = malloc(4097);
        hasher << &addr << addr;
        free(addr);
        */
}

/**
  | Gather non-cryptographic environment
  | data that does not change over time.
  |
  */
pub fn rand_add_static_env(hasher: &mut Sha512)  {
    
    todo!();
        /*
            // Some compile-time static properties
        hasher << (CHAR_MIN < 0) << sizeof(c_void*) << sizeof(long) << sizeof(int);
    #if defined(__GNUC__) && defined(__GNUC_MINOR__) && defined(__GNUC_PATCHLEVEL__)
        hasher << __GNUC__ << __GNUC_MINOR__ << __GNUC_PATCHLEVEL__;
    #endif
    #ifdef _MSC_VER
        hasher << _MSC_VER;
    #endif
        hasher << __cplusplus;
    #ifdef _XOPEN_VERSION
        hasher << _XOPEN_VERSION;
    #endif
    #ifdef __VERSION__
        const char* COMPILER_VERSION = __VERSION__;
        hasher.Write((const unsigned char*)COMPILER_VERSION, strlen(COMPILER_VERSION) + 1);
    #endif

        // Bitcoin client version
        hasher << CLIENT_VERSION;

    #if defined(HAVE_STRONG_GETAUXVAL)
        // Information available through getauxval()
    #  ifdef AT_HWCAP
        hasher << getauxval(AT_HWCAP);
    #  endif
    #  ifdef AT_HWCAP2
        hasher << getauxval(AT_HWCAP2);
    #  endif
    #  ifdef AT_RANDOM
        const unsigned char* random_aux = (const unsigned char*)getauxval(AT_RANDOM);
        if (random_aux) hasher.Write(random_aux, 16);
    #  endif
    #  ifdef AT_PLATFORM
        const char* platform_str = (const char*)getauxval(AT_PLATFORM);
        if (platform_str) hasher.Write((const unsigned char*)platform_str, strlen(platform_str) + 1);
    #  endif
    #  ifdef AT_EXECFN
        const char* exec_str = (const char*)getauxval(AT_EXECFN);
        if (exec_str) hasher.Write((const unsigned char*)exec_str, strlen(exec_str) + 1);
    #  endif
    #endif // HAVE_STRONG_GETAUXVAL

    #ifdef HAVE_GETCPUID
        AddAllCPUID(hasher);
    #endif

        // Memory locations
        hasher << &hasher << &RandAddStaticEnv << &malloc << &errno << &environ;

        // Hostname
        char hname[256];
        if (gethostname(hname, 256) == 0) {
            hasher.Write((const unsigned char*)hname, strnlen(hname, 256));
        }

    #if HAVE_DECL_GETIFADDRS && HAVE_DECL_FREEIFADDRS
        // Network interfaces
        struct ifaddrs *ifad = NULL;
        getifaddrs(&ifad);
        struct ifaddrs *ifit = ifad;
        while (ifit != NULL) {
            hasher.Write((const unsigned char*)&ifit, sizeof(ifit));
            hasher.Write((const unsigned char*)ifit->ifa_name, strlen(ifit->ifa_name) + 1);
            hasher.Write((const unsigned char*)&ifit->ifa_flags, sizeof(ifit->ifa_flags));
            AddSockaddr(hasher, ifit->ifa_addr);
            AddSockaddr(hasher, ifit->ifa_netmask);
            AddSockaddr(hasher, ifit->ifa_dstaddr);
            ifit = ifit->ifa_next;
        }
        freeifaddrs(ifad);
    #endif

    #ifndef WIN32
        // UNIX kernel information
        struct utsname name;
        if (uname(&name) != -1) {
            hasher.Write((const unsigned char*)&name.sysname, strlen(name.sysname) + 1);
            hasher.Write((const unsigned char*)&name.nodename, strlen(name.nodename) + 1);
            hasher.Write((const unsigned char*)&name.release, strlen(name.release) + 1);
            hasher.Write((const unsigned char*)&name.version, strlen(name.version) + 1);
            hasher.Write((const unsigned char*)&name.machine, strlen(name.machine) + 1);
        }

        /* Path and filesystem provided data */
        AddPath(hasher, "/");
        AddPath(hasher, ".");
        AddPath(hasher, "/tmp");
        AddPath(hasher, "/home");
        AddPath(hasher, "/proc");
    #ifdef __linux__
        AddFile(hasher, "/proc/cmdline");
        AddFile(hasher, "/proc/cpuinfo");
        AddFile(hasher, "/proc/version");
    #endif
        AddFile(hasher, "/etc/passwd");
        AddFile(hasher, "/etc/group");
        AddFile(hasher, "/etc/hosts");
        AddFile(hasher, "/etc/resolv.conf");
        AddFile(hasher, "/etc/timezone");
        AddFile(hasher, "/etc/localtime");
    #endif

        // For MacOS/BSDs, gather data through sysctl instead of /proc. Not all of these
        // will exist on every system.
    #if HAVE_SYSCTL
    #  ifdef CTL_HW
    #    ifdef HW_MACHINE
        AddSysctl<CTL_HW, HW_MACHINE>(hasher);
    #    endif
    #    ifdef HW_MODEL
        AddSysctl<CTL_HW, HW_MODEL>(hasher);
    #    endif
    #    ifdef HW_NCPU
        AddSysctl<CTL_HW, HW_NCPU>(hasher);
    #    endif
    #    ifdef HW_PHYSMEM
        AddSysctl<CTL_HW, HW_PHYSMEM>(hasher);
    #    endif
    #    ifdef HW_USERMEM
        AddSysctl<CTL_HW, HW_USERMEM>(hasher);
    #    endif
    #    ifdef HW_MACHINE_ARCH
        AddSysctl<CTL_HW, HW_MACHINE_ARCH>(hasher);
    #    endif
    #    ifdef HW_REALMEM
        AddSysctl<CTL_HW, HW_REALMEM>(hasher);
    #    endif
    #    ifdef HW_CPU_FREQ
        AddSysctl<CTL_HW, HW_CPU_FREQ>(hasher);
    #    endif
    #    ifdef HW_BUS_FREQ
        AddSysctl<CTL_HW, HW_BUS_FREQ>(hasher);
    #    endif
    #    ifdef HW_CACHELINE
        AddSysctl<CTL_HW, HW_CACHELINE>(hasher);
    #    endif
    #  endif
    #  ifdef CTL_KERN
    #    ifdef KERN_BOOTFILE
         AddSysctl<CTL_KERN, KERN_BOOTFILE>(hasher);
    #    endif
    #    ifdef KERN_BOOTTIME
         AddSysctl<CTL_KERN, KERN_BOOTTIME>(hasher);
    #    endif
    #    ifdef KERN_CLOCKRATE
         AddSysctl<CTL_KERN, KERN_CLOCKRATE>(hasher);
    #    endif
    #    ifdef KERN_HOSTID
         AddSysctl<CTL_KERN, KERN_HOSTID>(hasher);
    #    endif
    #    ifdef KERN_HOSTUUID
         AddSysctl<CTL_KERN, KERN_HOSTUUID>(hasher);
    #    endif
    #    ifdef KERN_HOSTNAME
         AddSysctl<CTL_KERN, KERN_HOSTNAME>(hasher);
    #    endif
    #    ifdef KERN_OSRELDATE
         AddSysctl<CTL_KERN, KERN_OSRELDATE>(hasher);
    #    endif
    #    ifdef KERN_OSRELEASE
         AddSysctl<CTL_KERN, KERN_OSRELEASE>(hasher);
    #    endif
    #    ifdef KERN_OSREV
         AddSysctl<CTL_KERN, KERN_OSREV>(hasher);
    #    endif
    #    ifdef KERN_OSTYPE
         AddSysctl<CTL_KERN, KERN_OSTYPE>(hasher);
    #    endif
    #    ifdef KERN_POSIX1
         AddSysctl<CTL_KERN, KERN_OSREV>(hasher);
    #    endif
    #    ifdef KERN_VERSION
         AddSysctl<CTL_KERN, KERN_VERSION>(hasher);
    #    endif
    #  endif
    #endif

        // Env variables
        if (environ) {
            for (size_t i = 0; environ[i]; ++i) {
                hasher.Write((const unsigned char*)environ[i], strlen(environ[i]));
            }
        }

        // Process, thread, user, session, group, ... ids.
    #ifdef WIN32
        hasher << GetCurrentProcessId() << GetCurrentThreadId();
    #else
        hasher << getpid() << getppid() << getsid(0) << getpgid(0) << getuid() << geteuid() << getgid() << getegid();
    #endif
        hasher << std::this_thread::get_id();
        */
}
