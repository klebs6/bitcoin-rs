crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/logging/timer.h]

/**
  | RAII-style object that outputs timing
  | information to logs.
  |
  */
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
        todo!();
        /*
            this->Log(strprintf("%s completed", m_title));
        */
    }
}

impl Timer {

    /**
      | If log_category is left as the default,
      | end_msg will log unconditionally (instead
      | of being filtered by category).
      |
      */
    pub fn new(
        prefix:       &str,
        end_msg:      &str,
        log_category: Option<LogFlags>) -> Self {

        let log_category: LogFlags =
                 log_category.unwrap_or(LogFlags::ALL);

        todo!();
        /*


            :
                m_prefix(std::move(prefix)),
                m_title(std::move(end_msg)),
                m_log_category(log_category)

            this->Log(strprintf("%s started", m_title));
            m_start_t = GetTime<std::chrono::microseconds>();
        */
    }
    
    pub fn log(&mut self, msg: &str)  {
        
        todo!();
        /*
            const std::string full_msg = this->LogMsg(msg);

            if (m_log_category == LogFlags::ALL) {
                LogPrintf("%s\n", full_msg);
            } else {
                LogPrint(m_log_category, "%s\n", full_msg);
            }
        */
    }
    
    pub fn log_msg<TimeType>(&mut self, msg: &str) -> String {
        
        todo!();
        /*
            const auto end_time = GetTime<std::chrono::microseconds>() - m_start_t;
            if (m_start_t.count() <= 0) {
                return strprintf("%s: %s", m_prefix, msg);
            }

            if constexpr (std::is_same<TimeType, std::chrono::microseconds>::value) {
                return strprintf("%s: %s (%iμs)", m_prefix, msg, end_time.count());
            } else if constexpr (std::is_same<TimeType, std::chrono::milliseconds>::value) {
                return strprintf("%s: %s (%.2fms)", m_prefix, msg, end_time.count() * 0.001);
            } else if constexpr (std::is_same<TimeType, std::chrono::seconds>::value) {
                return strprintf("%s: %s (%.2fs)", m_prefix, msg, end_time.count() * 0.000001);
            } else {
                const_assert(ALWAYS_FALSE<TimeType>, "Error: unexpected time type");
            }
        */
    }
}

#[macro_export] macro_rules! log_time_micros_with_category {
    ($end_msg:expr, 
     $log_category:ident) => {
        /*
        
            Timer<std::chrono::microseconds> PASTE2(logging_timer, __COUNTER__)(__func__, end_msg, log_category)
        */
    }
}

#[macro_export] macro_rules! log_time_millis_with_category {
    ($end_msg:expr, 
     $log_category:ident) => {
        /*
        
            Timer<std::chrono::milliseconds> PASTE2(logging_timer, __COUNTER__)(__func__, end_msg, log_category)
        */
    }
}

#[macro_export] macro_rules! log_time_seconds {
    ($end_msg:expr) => {
        /*
        
            Timer<std::chrono::seconds> PASTE2(logging_timer, __COUNTER__)(__func__, end_msg)
        */
    }
}
