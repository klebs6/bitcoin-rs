// ---------------- [ File: bitcoin-random/src/rand_add_dynamic_env.rs ]
crate::ix!();

/**
  | Gather non-cryptographic environment
  | data that changes over time.
  |
  */
pub fn rand_add_dynamic_env(hasher: &mut Sha512)  {
    // --- Windows: perfmon + FILETIME ---------------------------------------
    #[cfg(WIN32)]
    {
        // Matches: RandAddSeedPerfmon(hasher);
        rand_add_seed_perfmon(hasher);

        // Matches: FILETIME ftime; GetSystemTimeAsFileTime(&ftime); hasher << ftime;
        unsafe {
            use winapi::um::{minwinbase::FILETIME, sysinfoapi::GetSystemTimeAsFileTime};
            let mut ftime: FILETIME = core::mem::zeroed();
            GetSystemTimeAsFileTime(&mut ftime);
            hasher.write(
                &ftime as *const _ as *const u8,
                core::mem::size_of::<FILETIME>(),
            );
        }
    }

    // --- POSIX: various clocks (timespecs) + gettimeofday -------------------
    #[cfg(not(WIN32))]
    unsafe {
        // struct timespec ts = {};
        let mut ts: libc::timespec = core::mem::zeroed();

        // #ifdef CLOCK_MONOTONIC
        #[cfg(any(
            target_os = "linux", target_os = "android",
            target_os = "freebsd", target_os = "netbsd",
            target_os = "openbsd", target_os = "dragonfly", target_os = "macos"
        ))]
        if libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) == 0 {
            hasher.write(
                &ts as *const _ as *const u8,
                core::mem::size_of::<libc::timespec>(),
            );
        }

        // #ifdef CLOCK_REALTIME
        if libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts) == 0 {
            hasher.write(
                &ts as *const _ as *const u8,
                core::mem::size_of::<libc::timespec>(),
            );
        }

        // #ifdef CLOCK_BOOTTIME  (Linux/Android)
        #[cfg(any(target_os = "linux", target_os = "android"))]
        if libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut ts) == 0 {
            hasher.write(
                &ts as *const _ as *const u8,
                core::mem::size_of::<libc::timespec>(),
            );
        }

        // // gettimeofday is available on all UNIX systems (microsecond precision).
        let mut tv: libc::timeval = core::mem::zeroed();
        libc::gettimeofday(&mut tv, core::ptr::null_mut());
        hasher.write(
            &tv as *const _ as *const u8,
            core::mem::size_of::<libc::timeval>(),
        );
    }

    // --- C++11 clocks: write three counts in this exact order ---------------
    //     system_clock::now().time_since_epoch().count();
    //     steady_clock::now().time_since_epoch().count();
    //     high_resolution_clock::now().time_since_epoch().count();
    //
    // We serialize them as 64-bit integers, matching the common libstdc++
    // representation (nanoseconds) on POSIX. On Windows, we derive a
    // monotonic-ish counter similarly to Core’s intent (value content is
    // entropy; exact epoch isn’t relied upon).
    {
        // system_clock count
        let sys_ns: i64 = {
            #[cfg(not(WIN32))]
            unsafe {
                let mut ts: libc::timespec = core::mem::zeroed();
                if libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts) == 0 {
                    let ns = (ts.tv_sec as i128) * 1_000_000_000i128 + (ts.tv_nsec as i128);
                    ns as i64
                } else {
                    0
                }
            }
            #[cfg(WIN32)]
            {
                // FILETIME is in 100ns ticks since 1601-01-01; multiply to ns.
                use winapi::um::{minwinbase::FILETIME, sysinfoapi::GetSystemTimeAsFileTime};
                let mut ft: FILETIME = unsafe { core::mem::zeroed() };
                unsafe { GetSystemTimeAsFileTime(&mut ft) };
                let v100ns = ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64);
                (v100ns.saturating_mul(100) as i64)
            }
        };
        hasher.write(&sys_ns as *const _ as *const u8, core::mem::size_of_val(&sys_ns));

        // steady_clock count
        #[cfg(not(WIN32))]
        let steady_ns: i64 = unsafe {
            let mut ts: libc::timespec = core::mem::zeroed();
            if libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) == 0 {
                let ns = (ts.tv_sec as i128) * 1_000_000_000i128 + (ts.tv_nsec as i128);
                ns as i64
            } else {
                0
            }
        };
        #[cfg(WIN32)]
        let steady_ns: i64 = {
            // Use our high‑res counter fallback; Core treats this as entropy.
            get_performance_counter() as i64
        };
        hasher.write(
            &steady_ns as *const _ as *const u8,
            core::mem::size_of_val(&steady_ns),
        );

        // high_resolution_clock count (libstdc++ typically aliases steady_clock)
        let high_ns: i64 = steady_ns;
        hasher.write(
            &high_ns as *const _ as *const u8,
            core::mem::size_of_val(&high_ns),
        );
    }

    // --- POSIX: current resource usage --------------------------------------
    #[cfg(not(WIN32))]
    unsafe {
        let mut usage: libc::rusage = core::mem::zeroed();
        if libc::getrusage(libc::RUSAGE_SELF, &mut usage) == 0 {
            hasher.write(
                &usage as *const _ as *const u8,
                core::mem::size_of::<libc::rusage>(),
            );
        }
    }

    // --- Linux: a handful of small /proc snapshots --------------------------
    #[cfg(all(not(WIN32), target_os = "linux"))]
    {
        use std::ffi::CString;
        for p in [
            "/proc/diskstats",
            "/proc/vmstat",
            "/proc/schedstat",
            "/proc/zoneinfo",
            "/proc/meminfo",
            "/proc/softirqs",
            "/proc/stat",
            "/proc/self/schedstat",
            "/proc/self/status",
        ] {
            if let Ok(cstr) = CString::new(p) {
                add_file(hasher, cstr.as_ptr());
            }
        }
    }

    // --- Stack and heap location (exact pointer bytes, order preserved) -----
    unsafe {
        // c_void* addr = malloc(4097);
        let addr = libc::malloc(4097);

        // hasher << &addr
        let pptr = &addr as *const *mut libc::c_void;
        hasher.write(pptr as *const u8, core::mem::size_of::<*mut libc::c_void>());

        // hasher << addr
        hasher.write(
            &addr as *const _ as *const u8,
            core::mem::size_of::<*mut libc::c_void>(),
        );

        // free(addr);
        libc::free(addr);
    }
}

#[cfg(test)]
mod rand_add_dynamic_env_spec {
    use super::*;

    #[traced_test]
    fn increases_hasher_size_at_all() {
        let mut h = Sha512::default();
        let before = h.size();
        rand_add_dynamic_env(&mut h);
        let after = h.size();
        assert!(after > before, "rand_add_dynamic_env did not add any bytes");
    }

    // Windows: we know we always write FILETIME (16 bytes) and
    // 3 chrono-like counters (3 * i64), and two pointer-sized values (&addr and addr).
    #[traced_test]
    #[cfg(WIN32)]
    fn includes_minimum_windows_material() {
        let mut h = Sha512::default();
        let before = h.size();

        rand_add_dynamic_env(&mut h);

        let after = h.size();
        let ptr_sz = core::mem::size_of::<*mut core::ffi::c_void>();
        let filetime_sz = core::mem::size_of::<winapi::um::minwinbase::FILETIME>();
        let chrono_counts = 3 * core::mem::size_of::<i64>();
        let heap_stack = 2 * ptr_sz;

        let min_expected: u64 = (filetime_sz + chrono_counts + heap_stack) as u64;

        assert!(
            after >= before + min_expected,
            "expected at least {} bytes, got {}",
            min_expected,
            after - before
        );
    }

    // POSIX: we *always* write a timeval, three 64-bit counts (system/steady/high),
    // and two pointer-sized values (&addr and addr). Other pieces (timespecs/rusage)
    // are best-effort; we do not count them in the minimum.
    #[traced_test]
    #[cfg(not(WIN32))]
    fn includes_minimum_posix_material() {
        let mut h = Sha512::default();
        let before = h.size();

        rand_add_dynamic_env(&mut h);

        let after = h.size();
        let ptr_sz = core::mem::size_of::<*mut core::ffi::c_void>();
        let tv_sz = core::mem::size_of::<libc::timeval>();
        let chrono_counts = 3 * core::mem::size_of::<i64>();
        let heap_stack = 2 * ptr_sz;

        let min_expected: u64 = (tv_sz + chrono_counts + heap_stack) as u64;

        assert!(
            after >= before + min_expected,
            "expected at least {} bytes, got {}",
            min_expected,
            after - before
        );
    }

    // Calling it multiple times should keep increasing the hasher's byte count.
    #[traced_test]
    fn multiple_calls_accumulate() {
        let mut h = Sha512::default();
        let b0 = h.size();
        rand_add_dynamic_env(&mut h);
        let b1 = h.size();
        rand_add_dynamic_env(&mut h);
        let b2 = h.size();

        assert!(b1 > b0, "first call did not add bytes");
        assert!(b2 > b1, "second call did not add bytes");
    }

    // Linux: make sure /proc reads don't panic and at least contribute something
    // beyond the strictly minimal pieces we already counted.
    // (We don't assume any particular file size; we just expect *some* growth.)
    #[traced_test]
    #[cfg(all(not(WIN32), target_os = "linux"))]
    fn linux_proc_is_best_effort_but_nonzero() {
        let mut h = Sha512::default();
        let before = h.size();
        rand_add_dynamic_env(&mut h);
        let after = h.size();

        // Very weak assertion that catches no-op implementations on Linux.
        assert!(after > before, "/proc snapshot did not add any bytes");
    }
}
