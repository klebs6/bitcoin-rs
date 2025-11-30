// ---------------- [ File: bitcoinleveldb-posixenv/src/now_micros.rs ]
crate::ix!();

impl NowMicros for PosixEnv {

    fn now_micros(&mut self) -> u64 {
        const USECONDS_PER_SECOND: u64 = 1_000_000;

        // Primary path: use libc::gettimeofday for close equivalence
        // with the original C++ implementation.
        unsafe {
            let mut tv: libc::timeval = std::mem::zeroed();
            let rc = libc::gettimeofday(&mut tv as *mut libc::timeval, std::ptr::null_mut());

            if rc == 0 {
                let secs  = tv.tv_sec as u64;
                let usecs = tv.tv_usec as u64;
                let micros = secs
                    .saturating_mul(USECONDS_PER_SECOND)
                    .saturating_add(usecs);

                trace!(
                    tv_sec  = tv.tv_sec,
                    tv_usec = tv.tv_usec,
                    micros,
                    "PosixEnv::now_micros: obtained time via gettimeofday"
                );

                return micros;
            }

            // Fallback: SystemTime if gettimeofday fails for any reason.
            warn!(
                rc,
                "PosixEnv::now_micros: gettimeofday failed; falling back to SystemTime"
            );
        }

        let now = std::time::SystemTime::now();
        let micros = match now.duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => d.as_secs()
                .saturating_mul(USECONDS_PER_SECOND)
                .saturating_add(u64::from(d.subsec_micros())),
            Err(err) => {
                error!(
                    error = ?err,
                    "PosixEnv::now_micros: SystemTime before UNIX_EPOCH; \
                     returning 0 as a conservative fallback"
                );
                0
            }
        };

        trace!(
            micros,
            "PosixEnv::now_micros: computed time via SystemTime fallback"
        );

        micros
    }
}
