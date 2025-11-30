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
