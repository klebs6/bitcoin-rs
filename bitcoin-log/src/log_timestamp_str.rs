// ---------------- [ File: bitcoin-log/src/log_timestamp_str.rs ]
crate::ix!();

impl Logger {

    pub fn log_timestamp_str(&self, s: &str) -> String {
        if !self.log_timestamps() {
            return s.to_owned();
        }
        let started = self
            .started_new_line()
            .load(std::sync::atomic::Ordering::Relaxed);

        if started {
            let micros = get_time_micros();
            let mut stamp = format_iso8601_datetime(micros / 1_000_000);
            if *self.log_time_micros() {
                if stamp.ends_with('Z') {
                    stamp.pop();
                }
                let us_part = micros % 1_000_000;
                stamp.push_str(&format!(".{:06}Z", us_part));
            }
            format!("{} {}", stamp, s)
        } else {
            s.to_owned()
        }
    }
}

#[cfg(test)]
mod logger_log_timestamp_str_tests {
    use super::*;

    /// Thoroughly tests `log_timestamp_str` behavior when log_timestamps / log_time_micros are on/off.
    #[traced_test]
    #[serial]
    fn test_log_timestamp_str() {
        info!("Testing log_timestamp_str with various logger settings.");

        let mut logger = Logger::default();

        // 1) If log_timestamps=false => returns input `s` unchanged
        logger.set_log_timestamps(false);
        logger.started_new_line().store(true, std::sync::atomic::Ordering::Relaxed);
        let res_no_timestamps = logger.log_timestamp_str("Hello\n");
        assert_eq!(
            res_no_timestamps, "Hello\n",
            "Without log_timestamps, output must match input exactly"
        );

        // 2) Enable timestamps => but no microseconds => we get an ISO8601 second-based prefix
        logger.set_log_timestamps(true);
        logger.set_log_time_micros(false);
        logger.started_new_line().store(true, std::sync::atomic::Ordering::Relaxed);
        let res_with_ts = logger.log_timestamp_str("Line w/ TS\n");
        debug!("res_with_ts => {}", res_with_ts);
        assert!(
            res_with_ts.contains("T"),
            "Must have an RFC3339 date/time"
        );
        assert!(
            res_with_ts.contains("Line w/ TS\n"),
            "Must contain original text"
        );
        // Must have exactly one '.' or zero => we must see 'Z' at the end
        // (But we won't parse the entire format in this test.)

        // 3) Timestamps + microseconds => must see .xxxxxxZ if started_new_line=true
        logger.set_log_time_micros(true);
        logger.started_new_line().store(true, std::sync::atomic::Ordering::Relaxed);
        let res_with_micros = logger.log_timestamp_str("Micros\n");
        debug!("res_with_micros => {}", res_with_micros);
        assert!(
            res_with_micros.contains("."),
            "Should contain '.' if microseconds are enabled"
        );
        assert!(
            res_with_micros.contains("Micros\n"),
            "Must contain original text"
        );

        // 4) If started_new_line=false => it should return the string unchanged
        logger.started_new_line().store(false, std::sync::atomic::Ordering::Relaxed);
        let unchanged = logger.log_timestamp_str("No prefix here\n");
        assert_eq!(
            unchanged,
            "No prefix here\n",
            "Should not have timestamp when started_new_line=false"
        );

        trace!("test_log_timestamp_str passed.");
    }
}
