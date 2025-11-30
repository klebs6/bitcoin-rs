// ---------------- [ File: bitcoinleveldb-posixenv/src/sleep_for_microseconds.rs ]
crate::ix!();

impl SleepForMicroseconds for PosixEnv {

    fn sleep_for_microseconds(&mut self, micros: i32) {
        trace!(
            micros,
            "PosixEnv::sleep_for_microseconds: requested sleep"
        );

        if micros <= 0 {
            debug!(
                micros,
                "PosixEnv::sleep_for_microseconds: non-positive duration; returning immediately"
            );
            return;
        }

        let duration = std::time::Duration::from_micros(micros as u64);

        debug!(
            micros,
            "PosixEnv::sleep_for_microseconds: sleeping"
        );
        std::thread::sleep(duration);
        debug!(
            micros,
            "PosixEnv::sleep_for_microseconds: sleep completed"
        );
    }
}

#[cfg(test)]
mod posix_env_sleep_for_microseconds_tests {
    use super::*;

    #[traced_test]
    fn sleep_for_microseconds_returns_immediately_for_non_positive_values() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));

        let start = std::time::Instant::now();
        env.sleep_for_microseconds(0);
        env.sleep_for_microseconds(-10);
        let elapsed = start.elapsed();

        assert!(
            elapsed < std::time::Duration::from_millis(10),
            "sleep_for_microseconds with non-positive values should not block; \
             elapsed = {:?}",
            elapsed
        );
    }

    #[traced_test]
    fn sleep_for_microseconds_blocks_for_requested_duration() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));

        let requested_micros = 50_000;
        let start = std::time::Instant::now();
        env.sleep_for_microseconds(requested_micros);
        let elapsed = start.elapsed();

        assert!(
            elapsed >= std::time::Duration::from_micros((requested_micros / 2) as u64),
            "sleep_for_microseconds should block for at least roughly half of the \
             requested interval; elapsed = {:?}",
            elapsed
        );
    }
}
