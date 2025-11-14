// ---------------- [ File: bitcoin-random/src/rand_add_static_env.rs ]
crate::ix!();

pub fn rand_add_static_env(hasher: &mut Sha512) {
    // --- Some compile-time static properties (exact order) ------------------
    // hasher << (CHAR_MIN < 0)
    // Serialize as a single byte 0/1 to match C++ bool layout.
    let char_signed: u8 = if ((-1i8) as libc::c_char) < (0 as libc::c_char) { 1 } else { 0 };
    unsafe { hasher.write(&char_signed as *const u8, core::mem::size_of::<u8>()) };

    // hasher << sizeof(c_void*) << sizeof(long) << sizeof(int)
    let sz_ptr  = core::mem::size_of::<*const libc::c_void>() as usize;
    let sz_long = core::mem::size_of::<libc::c_long>() as usize;
    let sz_int  = core::mem::size_of::<libc::c_int>() as usize;
    unsafe {
        hasher.write(&sz_ptr  as *const _ as *const u8, core::mem::size_of::<usize>());
        hasher.write(&sz_long as *const _ as *const u8, core::mem::size_of::<usize>());
        hasher.write(&sz_int  as *const _ as *const u8, core::mem::size_of::<usize>());
    }

    // --- Compiler/version macros block (skipped unless you expose them) -----
    // C++ does:
    //   hasher << __GNUC__ << __GNUC_MINOR__ << __GNUC_PATCHLEVEL__;
    //   hasher << _MSC_VER; hasher << __cplusplus; hasher << _XOPEN_VERSION;
    //   Write __VERSION__ string + NUL.
    // If you surface equivalents via your cfg layer, write them here exactly
    // as integers and a NUL-terminated string, in this order.
    /*
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
    */

    // --- Bitcoin client version (optional, only if you export it) -----------
    // If your cfg layer exposes CLIENT_VERSION (i32/u32), write it raw here.
    #[cfg(any())] // replace with an appropriate cfg if you export the symbol
    unsafe {
        extern "C" { static CLIENT_VERSION: libc::c_int; }
        let v: libc::c_int = CLIENT_VERSION;
        hasher.write(&v as *const _ as *const u8, core::mem::size_of::<libc::c_int>());
    }

    // --- getauxval() (Linux/Android) ---------------------------------------
    #[cfg(any(target_os = "linux", target_os = "android"))]
    unsafe {
        // AT_HWCAP / AT_HWCAP2
        let hwcap  = libc::getauxval(libc::AT_HWCAP);
        let hwcap2 = libc::getauxval(libc::AT_HWCAP2);
        hasher.write(&hwcap  as *const _ as *const u8, core::mem::size_of::<libc::c_ulong>());
        hasher.write(&hwcap2 as *const _ as *const u8, core::mem::size_of::<libc::c_ulong>());

        // AT_RANDOM (16 bytes)
        let random_aux = libc::getauxval(libc::AT_RANDOM) as *const u8;
        if !random_aux.is_null() {
            hasher.write(random_aux, 16);
        }

        // AT_PLATFORM (string + NUL)
        let platform = libc::getauxval(libc::AT_PLATFORM) as *const libc::c_char;
        if !platform.is_null() {
            let n = libc::strlen(platform) + 1;
            hasher.write(platform as *const u8, n);
        }

        // AT_EXECFN (string + NUL)
        let execfn = libc::getauxval(libc::AT_EXECFN) as *const libc::c_char;
        if !execfn.is_null() {
            let n = libc::strlen(execfn) + 1;
            hasher.write(execfn as *const u8, n);
        }
    }

    // --- CPU features via CPUID (exactly like Core) -------------------------
    #[cfg(have_getcpuid)]
    {
        add_allcpuid(hasher);
    }

    // --- Memory locations (order preserved) ---------------------------------
    unsafe {
        // &hasher
        let p_hasher = hasher as *mut Sha512;
        hasher.write(
            &p_hasher as *const _ as *const u8,
            core::mem::size_of::<*mut Sha512>(),
        );

        // &RandAddStaticEnv (function pointer)
        let fptr = rand_add_static_env as usize;
        hasher.write(&fptr as *const _ as *const u8, core::mem::size_of::<usize>());

        // &malloc (function pointer)
        let mptr = libc::malloc as usize;
        hasher.write(&mptr as *const _ as *const u8, core::mem::size_of::<usize>());

        // &errno  -> pointer to the TLS errno int
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            let errno_ptr = libc::__errno_location();
            hasher.write(
                &errno_ptr as *const _ as *const u8,
                core::mem::size_of::<*mut libc::c_int>(),
            );
        }
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly"
        ))]
        {
            let errno_ptr = libc::__error();
            hasher.write(
                &errno_ptr as *const _ as *const u8,
                core::mem::size_of::<*mut libc::c_int>(),
            );
        }

        // &environ (address of the global)
        #[cfg(not(WIN32))]
        {
            extern "C" {
                static mut environ: *mut *mut libc::c_char;
            }
            let penv = &environ as *const _; // address of the global symbol
            hasher.write(
                &penv as *const _ as *const u8,
                core::mem::size_of::<*const *mut *mut libc::c_char>(),
            );
        }
    }

    // --- Hostname -----------------------------------------------------------
    #[cfg(not(WIN32))]
    unsafe {
        let mut hname = [0u8; 256];
        if libc::gethostname(hname.as_mut_ptr() as *mut libc::c_char, hname.len()) == 0 {
            // strnlen up to 256
            let n = libc::strnlen(hname.as_ptr() as *const libc::c_char, hname.len());
            hasher.write(hname.as_ptr(), n);
        }
    }

    // --- Network interfaces -------------------------------------------------
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    unsafe {
        let mut ifap: *mut libc::ifaddrs = core::ptr::null_mut();
        if libc::getifaddrs(&mut ifap) == 0 {
            let mut it = ifap;
            while !it.is_null() {
                // hasher.Write((unsigned char*)&ifit, sizeof(ifit))  -> pointer value
                hasher.write(
                    &it as *const _ as *const u8,
                    core::mem::size_of::<*mut libc::ifaddrs>(),
                );

                // hasher.Write(ifa_name, strlen+1)
                let name = (*it).ifa_name;
                if !name.is_null() {
                    let n = libc::strlen(name) + 1;
                    hasher.write(name as *const u8, n);
                }

                // hasher.Write(&ifa_flags, sizeof(ifa_flags))
                hasher.write(
                    &(*it).ifa_flags as *const _ as *const u8,
                    core::mem::size_of_val(&(*it).ifa_flags),
                );

                // AddSockaddr(hasher, ifa_addr/netmask/dstaddr)
                add_sockaddr(hasher, (*it).ifa_addr as *const nix::sys::socket::sockaddr);
                add_sockaddr(hasher, (*it).ifa_netmask as *const nix::sys::socket::sockaddr);
                add_sockaddr(hasher, (*it).ifa_dstaddr as *const nix::sys::socket::sockaddr);

                it = (*it).ifa_next;
            }
            libc::freeifaddrs(ifap);
        }
    }

    // --- UNIX kernel information + basic paths/files ------------------------
    #[cfg(not(WIN32))]
    unsafe {
        // uname
        let mut name: libc::utsname = core::mem::zeroed();
        if libc::uname(&mut name) != -1 {
            // Write strings + NUL, in exact order
            let sysname  = name.sysname.as_ptr();
            let nodename = name.nodename.as_ptr();
            let release  = name.release.as_ptr();
            let version  = name.version.as_ptr();
            let machine  = name.machine.as_ptr();

            let mut w = |p: *const libc::c_char| {
                if !p.is_null() {
                    let n = libc::strlen(p) + 1;
                    hasher.write(p as *const u8, n);
                }
            };
            w(sysname);
            w(nodename);
            w(release);
            w(version);
            w(machine);
        }

        // AddPath (exact list/order)
        add_path(hasher, b"/\0".as_ptr());
        add_path(hasher, b".\0".as_ptr());
        add_path(hasher, b"/tmp\0".as_ptr());
        add_path(hasher, b"/home\0".as_ptr());
        add_path(hasher, b"/proc\0".as_ptr());

        // Linux /proc files (exact list/order)
        #[cfg(any(target_os = "linux", target_os = "android"))]
        {
            use std::ffi::CString;
            for p in ["/proc/cmdline", "/proc/cpuinfo", "/proc/version"] {
                if let Ok(cs) = CString::new(p) {
                    add_file(hasher, cs.as_ptr());
                }
            }
        }

        // Common/etc files (exact list/order)
        {
            use std::ffi::CString;
            for p in [
                "/etc/passwd",
                "/etc/group",
                "/etc/hosts",
                "/etc/resolv.conf",
                "/etc/timezone",
                "/etc/localtime",
            ] {
                if let Ok(cs) = CString::new(p) {
                    add_file(hasher, cs.as_ptr());
                }
            }
        }
    }

    // --- sysctl blocks for Mac/BSDs (defer until add_sysctl is implemented) -
    // See your commented C++ list. Add them back here once add_sysctl<T...>()
    // is available, preserving the same order.
    /*
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
    */

    // --- Environment variables ---------------------------------------------
    #[cfg(not(WIN32))]
    unsafe {
        extern "C" { static mut environ: *mut *mut libc::c_char; }
        let mut p = environ;
        if !p.is_null() {
            loop {
                let q = *p;
                if q.is_null() { break; }
                let n = libc::strlen(q);
                hasher.write(q as *const u8, n); // no terminating NUL (matches Core)
                p = p.add(1);
            }
        }
    }

    // --- Process/thread/user/session/group IDs ------------------------------
    #[cfg(WIN32)]
    unsafe {
        use winapi::um::processthreadsapi::{GetCurrentProcessId, GetCurrentThreadId};
        let pid = GetCurrentProcessId();
        let tid = GetCurrentThreadId();
        hasher.write(&pid as *const _ as *const u8, core::mem::size_of_val(&pid));
        hasher.write(&tid as *const _ as *const u8, core::mem::size_of_val(&tid));
    }

    #[cfg(not(WIN32))]
    unsafe {
        // getpid, getppid, getsid(0), getpgid(0), getuid, geteuid, getgid, getegid
        let pid   = libc::getpid();
        let ppid  = libc::getppid();
        let sid   = libc::getsid(0);
        let pgid  = libc::getpgid(0);
        let uid   = libc::getuid();
        let euid  = libc::geteuid();
        let gid   = libc::getgid();
        let egid  = libc::getegid();
        hasher.write(&pid  as *const _ as *const u8, core::mem::size_of_val(&pid));
        hasher.write(&ppid as *const _ as *const u8, core::mem::size_of_val(&ppid));
        hasher.write(&sid  as *const _ as *const u8, core::mem::size_of_val(&sid));
        hasher.write(&pgid as *const _ as *const u8, core::mem::size_of_val(&pgid));
        hasher.write(&uid  as *const _ as *const u8, core::mem::size_of_val(&uid));
        hasher.write(&euid as *const _ as *const u8, core::mem::size_of_val(&euid));
        hasher.write(&gid  as *const _ as *const u8, core::mem::size_of_val(&gid));
        hasher.write(&egid as *const _ as *const u8, core::mem::size_of_val(&egid));

        // std::this_thread::get_id() equivalent: pthread_self() raw bytes
        let tid = libc::pthread_self();
        hasher.write(&tid as *const _ as *const u8, core::mem::size_of_val(&tid));
    }
}

#[cfg(test)]
mod rand_add_static_env_spec {
    use super::*;

    #[traced_test]
    fn increases_hasher_size_at_all() {
        let mut h = Sha512::default();
        let before = h.size();
        rand_add_static_env(&mut h);
        let after = h.size();
        assert!(after > before, "rand_add_static_env did not add any bytes");
    }

    // This test asserts the *minimal* set of bytes we know we write on all platforms:
    //   - 1 byte for (CHAR_MIN < 0) bool
    //   - sizeof(void*) + sizeof(long) + sizeof(int)
    //   - pointers: &hasher, &rand_add_static_env, &malloc
    //
    // We *also* write errno/environ pointers on non-Windows; those are included
    // in a separate OS-gated test below to avoid false assumptions on Windows.
    #[traced_test]
    fn includes_core_minimum_sizes_and_pointers() {
        let mut h = Sha512::default();
        let before = h.size();

        rand_add_static_env(&mut h);

        let after = h.size();

        let bool_byte = 1usize;
        let sz_ptr  = core::mem::size_of::<*const libc::c_void>();
        let sz_long = core::mem::size_of::<libc::c_long>();
        let sz_int  = core::mem::size_of::<libc::c_int>();
        let base_sizes = bool_byte + sz_ptr + sz_long + sz_int;

        let ptr_sz = core::mem::size_of::<*const libc::c_void>();
        let core_ptrs = 3 * ptr_sz; // &hasher, &rand_add_static_env, &malloc

        let min_expected: u64 = (base_sizes + core_ptrs) as u64;

        assert!(
            after >= before + min_expected,
            "expected at least {} bytes, got {}",
            min_expected,
            after - before
        );
    }

    // On POSIX we also write:
    //   - &errno pointer (pointer size)
    //   - &environ *address of the global symbol* (pointer size)
    //   - hostname bytes (strnlen result, no trailing NUL)
    //
    // We only add these to the lower bound if we can measure their sizes right now.
    #[traced_test]
    #[cfg(not(WIN32))]
    fn posix_adds_errno_environ_and_hostname_lower_bound() {
        let mut h = Sha512::default();
        let before = h.size();

        // Compute lower bound contributions we *know* will be written.
        let mut extra_expected = 0usize;

        // &errno pointer value
        #[cfg(any(target_os = "linux", target_os = "android"))]
        unsafe {
            let _errno_ptr = libc::__errno_location();
            let ptr_sz = core::mem::size_of::<*mut libc::c_int>();
            extra_expected += ptr_sz;
        }
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly"
        ))]
        unsafe {
            let _errno_ptr = libc::__error();
            let ptr_sz = core::mem::size_of::<*mut libc::c_int>();
            extra_expected += ptr_sz;
        }

        // &environ address-of-global (pointer size)
        unsafe {
            let ptr_sz_env = core::mem::size_of::<*const *mut *mut libc::c_char>();
            extra_expected += ptr_sz_env;
        }

        // Hostname: number of bytes we will write (no terminating NUL)
        unsafe {
            let mut hname = [0u8; 256];
            let mut host_contrib = 0usize;
            if libc::gethostname(hname.as_mut_ptr() as *mut libc::c_char, hname.len()) == 0 {
                let n = libc::strnlen(hname.as_ptr() as *const libc::c_char, hname.len());
                host_contrib = n as usize;
            }
            extra_expected += host_contrib;
        }

        rand_add_static_env(&mut h);

        let after = h.size();

        assert!(
            after >= before + (extra_expected as u64),
            "expected at least {} bytes (POSIX extras), got {}",
            extra_expected,
            after - before
        );
    }

    // Linux: getauxval() adds several lower-bounded contributions *if present*.
    // We only count what we can safely measure right now:
    //   - AT_HWCAP (c_ulong), AT_HWCAP2 (c_ulong)
    //   - AT_RANDOM (16 bytes) if non-null
    //   - AT_PLATFORM (strlen + 1) if non-null
    //   - AT_EXECFN (strlen + 1) if non-null
    #[traced_test]
    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn linux_getauxval_lower_bound() {
        unsafe {
            let mut h = Sha512::default();
            let before = h.size();

            let mut lb = 0usize;

            // AT_HWCAP and AT_HWCAP2 are always written (no null checks in our impl)
            let ul_sz = core::mem::size_of::<libc::c_ulong>();
            lb += ul_sz; // HWCAP
            lb += ul_sz; // HWCAP2

            // AT_RANDOM: 16 bytes if pointer is non-null
            let rnd = libc::getauxval(libc::AT_RANDOM) as *const u8;
            if !rnd.is_null() {
                lb += 16;
            }

            // AT_PLATFORM
            let platform = libc::getauxval(libc::AT_PLATFORM) as *const libc::c_char;
            if !platform.is_null() {
                let n = libc::strlen(platform) + 1; // string + NUL
                lb += n as usize;
            }

            // AT_EXECFN
            let execfn = libc::getauxval(libc::AT_EXECFN) as *const libc::c_char;
            if !execfn.is_null() {
                let n = libc::strlen(execfn) + 1; // string + NUL
                lb += n as usize;
            }

            rand_add_static_env(&mut h);
            let after = h.size();

            assert!(
                after >= before + lb,
                "expected at least {} bytes from getauxval block, got {}",
                lb,
                after - before
            );
        }
    }

    // Multiple invocations should keep increasing the size (there are always some
    // dynamic components like addresses and environment content).
    #[traced_test]
    fn multiple_calls_accumulate() {
        let mut h = Sha512::default();
        let b0 = h.size();
        rand_add_static_env(&mut h);
        let b1 = h.size();
        rand_add_static_env(&mut h);
        let b2 = h.size();

        assert!(b1 > b0, "first call did not add bytes");
        assert!(b2 > b1, "second call did not add bytes");
    }
}
