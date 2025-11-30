// ---------------- [ File: bitcoinleveldb-posixtools/src/max_open_files.rs ]
crate::ix!();

/// Return the maximum number of read-only
/// files to keep open.
/// 
pub fn max_open_files() -> i32 {
    use std::sync::atomic::Ordering;

    trace!("max_open_files: start");

    let cached = OPEN_READ_ONLY_FILE_LIMIT.load(Ordering::SeqCst);
    if cached >= 0 {
        debug!(
            value = cached,
            "max_open_files: using cached open-file limit"
        );
        return cached;
    }

    let mut computed: i32;

    unsafe {
        let mut rlim: libc::rlimit = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };

        let rc = libc::getrlimit(libc::RLIMIT_NOFILE, &mut rlim as *mut libc::rlimit);

        if rc != 0 {
            // getrlimit failed, fall back to a conservative default.
            computed = 50;
            warn!(
                rc,
                "max_open_files: getrlimit(RLIMIT_NOFILE) failed, using fallback"
            );
        } else if rlim.rlim_cur == libc::RLIM_INFINITY {
            computed = i32::MAX;
            debug!(
                rlim_cur = rlim.rlim_cur as u64,
                "max_open_files: RLIM_INFINITY, using i32::MAX"
            );
        } else {
            let cur = rlim.rlim_cur;
            // Allow use of 20% of available descriptors for read-only files.
            let twenty_percent = cur / 5;
            if twenty_percent == 0 {
                computed = 50;
                debug!(
                    rlim_cur = cur as u64,
                    "max_open_files: RLIMIT_NOFILE too small, using fallback"
                );
            } else if twenty_percent > i32::MAX as u64 {
                computed = i32::MAX;
                debug!(
                    rlim_cur = cur as u64,
                    computed,
                    "max_open_files: capped computed limit at i32::MAX"
                );
            } else {
                computed = twenty_percent as i32;
                debug!(
                    rlim_cur = cur as u64,
                    computed,
                    "max_open_files: computed open-file limit from RLIMIT_NOFILE"
                );
            }
        }
    }

    OPEN_READ_ONLY_FILE_LIMIT.store(computed, Ordering::SeqCst);

    debug!(
        value = computed,
        "max_open_files: finalized open-file limit"
    );
    computed
}
