// ---------------- [ File: bitcoin-time/src/count.rs ]
crate::ix!();

/// Helper to count the seconds of a duration.
/// 
/// All durations should be using std::chrono and calling this should generally be avoided in
/// code. Though, it is still preferred to an inline t.count() to protect against a reliance on the
/// exact type of t.
/// 
/// This helper is used to convert durations before passing them over an interface that doesn't
/// support std::chrono (e.g. RPC, debug log, or the GUI)
/// 
#[inline]
pub fn count_seconds(t: Seconds) -> i64 {
    let secs = t.as_secs() as i64;
    trace!(seconds = secs, "count_seconds");
    secs
}

/// Helper to count the milliseconds of a duration.
#[inline]
pub fn count_milliseconds(t: Milliseconds) -> i64 {
    let ms = t.as_millis() as i64;
    trace!(milliseconds = ms, "count_milliseconds");
    ms
}

/// Helper to count the microseconds of a duration.
#[inline]
pub fn count_microseconds(t: Microseconds) -> i64 {
    let µs = t.as_micros() as i64;
    trace!(microseconds = µs, "count_microseconds");
    µs
}


/// Helper to count the seconds in any duration type, returning `f64`.
#[inline]
pub fn count_seconds_double(t: SecondsDouble) -> f64 {
    let secs = t.as_secs_f64();
    trace!(seconds = secs, "count_seconds_double");
    secs
}


pub use std::time::Duration as StdDuration;       // NEW

/// Rust equivalents of the C++ `std::chrono` typedefs.             // NEW
pub type Milliseconds  = StdDuration;                              // NEW
pub type Microseconds  = StdDuration;                              // NEW
pub type Seconds       = StdDuration;                              // NEW
pub type SecondsDouble = StdDuration;                              // NEW
//pub type SecondsDouble = Seconds<u64>;
