// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_capture_current_time_components.rs ]
crate::ix!();

impl PosixLogger {

    pub fn capture_current_time_components(&self) -> (libc::timeval, libc::tm) {
        unsafe {
            // Record the time as close to the Logv() call as possible.
            let mut now_timeval: libc::timeval = std::mem::zeroed();
            if libc::gettimeofday(
                &mut now_timeval as *mut libc::timeval,
                std::ptr::null_mut(),
            ) != 0
            {
                error!(
                    "PosixLogger::capture_current_time_components: gettimeofday failed"
                );
            }

            let mut now_seconds: libc::time_t = now_timeval.tv_sec;
            let mut now_components: libc::tm = std::mem::zeroed();
            if libc::localtime_r(
                &mut now_seconds as *mut libc::time_t,
                &mut now_components as *mut libc::tm,
            )
            .is_null()
            {
                error!(
                    "PosixLogger::capture_current_time_components: localtime_r failed"
                );
            }

            trace!(
                "PosixLogger::capture_current_time_components: sec={} usec={}",
                now_timeval.tv_sec,
                now_timeval.tv_usec
            );

            (now_timeval, now_components)
        }
    }
}

#[cfg(test)]
mod posix_logger_time_capture_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> PosixLogger {
        info!("create_logger_with_tmpfile: creating logger for time capture tests");
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            PosixLogger::new(fp)
        }
    }

    #[traced_test]
    fn capture_current_time_components_returns_reasonable_values() {
        info!("capture_current_time_components_returns_reasonable_values: start");
        let logger = create_logger_with_tmpfile();

        let (now_tv, mut now_tm) = logger.capture_current_time_components();
        debug!(
            "capture_current_time_components_returns_reasonable_values: tv_sec={} tv_usec={}",
            now_tv.tv_sec,
            now_tv.tv_usec
        );

        assert!(
            now_tv.tv_usec >= 0,
            "tv_usec should be non-negative in timeval"
        );
        assert!(
            now_tv.tv_usec < 1_000_000,
            "tv_usec should be less than 1_000_000"
        );

        unsafe {
            let mktime_result = libc::mktime(&mut now_tm as *mut libc::tm);
            debug!(
                "capture_current_time_components_returns_reasonable_values: mktime_result={}",
                mktime_result
            );

            let tv_sec_i64 = now_tv.tv_sec as i64;
            let mktime_sec_i64 = mktime_result as i64;
            let diff = (tv_sec_i64 - mktime_sec_i64).abs();
            trace!(
                "capture_current_time_components_returns_reasonable_values: diff_between_tv_sec_and_mktime={}",
                diff
            );

            assert!(
                diff <= 1,
                "Difference between timeval seconds and mktime result should be at most 1 second"
            );
        }

        assert!(
            now_tm.tm_year >= 70,
            "tm_year should be at least 70 (corresponding to year 1970)"
        );

        info!("capture_current_time_components_returns_reasonable_values: end");
    }
}
