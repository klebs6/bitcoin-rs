// ---------------- [ File: bitcoin-log/src/timer.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/logging/timer.h]

#[derive(Getters,MutGetters,Setters,Builder)]
#[builder(setter(into), build_fn(skip))] // <== We skip the default build fn so we can customize it
#[getset(get="pub",set="pub")]
pub struct Timer {
    start_t:      Instant,
    prefix:       String,
    title:        String,
    log_category: LogFlags,
}

impl TimerBuilder {
    /// Custom `build()` that also logs "<title> started" so tests see "Sample Timer started".
    pub fn build(&self) -> Result<Timer, ::derive_builder::UninitializedFieldError> {
        let start_t = match &self.start_t {
            Some(t) => *t,
            None => Instant::now(),
        };

        let prefix = self
            .prefix
            .clone()
            .unwrap_or_else(|| "Timer".to_string());

        let title = self
            .title
            .clone()
            .unwrap_or_else(|| "UnnamedTimer".to_string());

        let log_category = self
            .log_category
            .unwrap_or(LogFlags::ALL);

        let mut timer = Timer {
            start_t,
            prefix,
            title,
            log_category,
        };

        // Log the "XYZ started" line upon creation:
        timer.log(&format!("{} started", timer.title()));

        Ok(timer)
    }
}

impl Timer {

    /// Updated constructor to log "Sample Timer started" (or any "XYZ started") upon creation.
    pub fn new(prefix: &str, end_msg: &str, log_category: Option<LogFlags>) -> Self {
        let final_cat = log_category.unwrap_or(LogFlags::ALL);
        let mut timer = Self {
            start_t: Instant::now(),
            prefix: prefix.to_string(),
            title: end_msg.to_string(),
            log_category: final_cat,
        };

        // ************ CHANGE: We now explicitly log "<title> started" ************
        timer.log(&format!("{} started", timer.title()));

        timer
    }

    /// Logs the given message if the Timer’s category is enabled or ALL.
    pub fn log(&mut self, msg: &str) {
        trace!("Timer::log => called with msg='{}'", msg);
        let full_msg = self.log_msg::<()>(msg);
        let cat = *self.log_category();
        let will_log = cat == LogFlags::ALL || log_instance().will_log_category(cat);
        debug!("Timer::log => will_log={}", will_log);
        if will_log {
            let with_newline = format!("{}\n", full_msg);
            trace!(
                "Timer::log => calling log_instance().log_print_str(...) => {}",
                with_newline
            );
            log_instance().log_print_str(
                &with_newline,
                "Timer::log",
                file!(),
                line!() as i32
            );
        }
    }

    pub fn log_msg<TimeType>(&mut self, msg: &str) -> String {
        trace!("Timer::log_msg => building final log line => msg='{}'", msg);
        let elapsed = self.start_t().elapsed();
        let micros = (elapsed.as_seconds_f32() as u128 * 1_000_000)
            + (elapsed.subsec_nanoseconds() as u128 / 1_000);

        if micros == 0 {
            format!("{}: {}", self.prefix(), msg)
        } else {
            format!("{}: {} ({}μs)", self.prefix(), msg, micros)
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        trace!("Timer::drop => about to log '... completed'");
        self.log(&format!("{} completed", self.title()));
        trace!("Timer::drop => done");
    }
}

#[macro_export]
macro_rules! log_time_micros_with_category {
    ($end_msg:expr, $log_category:expr) => {
        // In C++: Timer<std::chrono::microseconds> ...
        // In Rust, we currently use microseconds internally in Timer::log_msg, so just call Timer::new:
        let _logging_timer_micros = $crate::Timer::new(
            module_path!(),
            $end_msg,
            Some($log_category),
        );
    };
}

#[macro_export]
macro_rules! log_time_millis_with_category {
    ($end_msg:expr, $log_category:expr) => {
        // In C++: Timer<std::chrono::milliseconds> ...
        // We use the same Timer in Rust (which logs in microseconds).
        let _logging_timer_millis = $crate::Timer::new(
            module_path!(),
            $end_msg,
            Some($log_category),
        );
    };
}

#[macro_export]
macro_rules! log_time_seconds {
    ($end_msg:expr) => {
        // In C++: Timer<std::chrono::seconds> ...
        // Same Timer in Rust, but we won't specify a category => defaults to LogFlags::ALL or None.
        let _logging_timer_seconds = $crate::Timer::new(
            module_path!(),
            $end_msg,
            None,
        );
    };
}
