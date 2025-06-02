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
