// ---------------- [ File: bitcoin-time/src/get_time.rs ]
crate::ix!();

use std::time::{UNIX_EPOCH,SystemTime};

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

/// Return system time (or mocked time, if set) expressed as an arbitrary
/// `Duration`‑convertible representation.
pub fn get_time_since_epoch<T>() -> T
where
    T: From<StdDuration>,                                              // CHANGED
{
    let mock = mock_time::MOCK_TIME.load(atomic::Ordering::Relaxed);    // CHANGED
    let d: StdDuration = if mock != 0 {                                // CHANGED
        StdDuration::from_secs(mock as u64)                            // CHANGED
    } else {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
    };
    trace!(?d, "get_time_since_epoch");
    T::from(d)
}

/// Return *real* system time expressed as an arbitrary `Duration`‑convertible
/// representation (never mockable).
pub fn get_system_time_since_epoch<T>() -> T
where
    T: From<StdDuration>,                                              // CHANGED
{
    let d = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");
    debug_assert!(d > StdDuration::ZERO, "system clock before Unix epoch"); // CHANGED
    trace!(?d, "get_system_time_since_epoch");
    T::from(d)
}

/// For testing – return the currently‑configured mock time, or `StdDuration::ZERO`.
pub fn get_mock_time_since_epoch() -> StdDuration {                   // CHANGED
    let secs = mock_time::MOCK_TIME.load(atomic::Ordering::Relaxed);   // CHANGED
    trace!(mock_seconds = secs, "get_mock_time_since_epoch");
    if secs == 0 {
        StdDuration::ZERO
    } else {
        StdDuration::from_secs(secs as u64)
    }
}

/// Returns **physical** milliseconds since the Unix epoch (not mockable).
pub fn get_time_millis_since_epoch() -> i64 {
    get_system_time_since_epoch::<StdDuration>().as_millis() as i64   // CHANGED
}

/// Returns **physical** microseconds since the Unix epoch (not mockable).
pub fn get_time_micros_since_epoch() -> i64 {
    get_system_time_since_epoch::<StdDuration>().as_micros() as i64   // CHANGED
}

/// Returns **physical** seconds since the Unix epoch (not mockable).
pub fn get_time_seconds_since_epoch() -> i64 {
    get_system_time_since_epoch::<StdDuration>().as_secs() as i64     // CHANGED
}
