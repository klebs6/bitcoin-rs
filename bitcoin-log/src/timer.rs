// ---------------- [ File: bitcoin-log/src/timer.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/logging/timer.h]

/**
  | RAII-style object that outputs timing
  | information to logs.
  |
  */
#[derive(Getters,MutGetters,Setters,Builder)]
#[builder(setter(into))]
#[getset(get="pub",set="pub")]
pub struct Timer {

    start_t:      Instant,

    /**
      | Log prefix; usually the name of the function
      | this was created in.
      |
      */
    prefix:       String,

    /**
      | A descriptive message of what is being
      | timed.
      |
      */
    title:        String,

    /**
      | Forwarded on to LogPrint if specified
      |
      | Has the effect of only outputting the
      | timing log when a particular debug=
      | category is specified.
      |
      */
    log_category: LogFlags,
}

impl Drop for Timer {
    fn drop(&mut self) {
        // Was logging internal details via `trace!`; removed to prevent re-entrancy.
        self.log(&format!("{} completed", self.title()));
    }
}

impl Timer {
    pub fn new(prefix: &str, end_msg: &str, log_category: Option<LogFlags>) -> Self {
        let final_cat = log_category.unwrap_or(LogFlags::ALL);
        let mut timer = Self {
            start_t: Instant::now(),
            prefix: prefix.to_string(),
            title: end_msg.to_string(),
            log_category: final_cat,
        };
        timer.log(&format!("{} started", timer.title()));
        timer
    }

    pub fn log(&mut self, msg: &str) {
        // Removed `trace!` calls to avoid re-entrant logging.
        let full_msg = self.log_msg::<()>(msg);
        let cat = *self.log_category();
        // If log_category is ALL or is enabled, proceed:
        let will_log = cat == LogFlags::ALL || log_instance().will_log_category(cat);
        if will_log {
            let with_newline = format!("{}\n", full_msg);
            log_instance().log_print_str(
                &with_newline,
                "Timer::log",
                file!(),
                line!() as i32
            );
        }
    }

    pub fn log_msg<TimeType>(&mut self, msg: &str) -> String {
        // Removed `trace!` calls to avoid re-entrant logging.
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
