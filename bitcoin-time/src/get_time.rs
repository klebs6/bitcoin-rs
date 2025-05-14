// ---------------- [ File: bitcoin-time/src/get_time.rs ]
crate::ix!();

pub fn get_time() -> Instant {
    Instant::now()
}

#[inline] pub fn max_unix_timestamp() -> i64 {

    let time = unsafe { Time::__from_hms_nanos_unchecked(23, 59, 59, 999_999_999) };

    Date::MAX
        .with_time(time)
        .assume_utc()
        .unix_timestamp()
}

pub fn get_datetime() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

/**
  | Return system time (or mocked time,
  | if set)
  |
  */
pub fn get_time_since_epoch<T>() -> T {

    todo!();
        /*
            const seconds mocktime{nMockTime.load(std::memory_order_relaxed)};

        return duration_cast<T>(
            mocktime.count() ?
                mocktime :
                microseconds{GetTimeMicros()});
        */
}

pub fn get_system_time_since_epoch<T>() -> T {

    todo!();
        /*
            const auto now = duration_cast<T>(system_clock::now().time_since_epoch());
        assert(now.count() > 0);
        return now;
        */
}

/**
  | For testing
  |
  */
pub fn get_mock_time_since_epoch() -> Duration {
    
    todo!();
        /*
            return seconds(nMockTime.load(std::memory_order_relaxed));
        */
}

/**
  | Returns the system time (not mockable)
  |
  */
pub fn get_time_millis_since_epoch() -> i64 {
    
    todo!();
        /*
            return int64_t{GetSystemTime<milliseconds>().count()};
        */
}

/**
  | Returns the system time (not mockable)
  |
  */
pub fn get_time_micros_since_epoch() -> i64 {
    
    todo!();
        /*
            return int64_t{GetSystemTime<microseconds>().count()};
        */
}

/**
  | Returns the system time (not mockable)
  |
  | Like GetTime(), but not mockable
  */
pub fn get_time_seconds_since_epoch() -> i64 {
    
    todo!();
        /*
            return int64_t{GetSystemTime<seconds>().count()};
        */
}
