// ---------------- [ File: bitcoin-sock/src/compute_bounded_wait.rs ]
crate::ix!();

/// Compute the bounded wait interval until `deadline`,
/// clamped to `MAX_WAIT_FOR_IO`.
#[inline]
pub fn compute_bounded_wait(deadline: Instant) -> Duration {
    let now = Instant::now();

    // Remaining time in nanoseconds (std time API)
    let remaining_ns = deadline.0.saturating_duration_since(now.into()).as_nanos();

    // Convert MAX_WAIT_FOR_IO (a `time::Duration`) into nanoseconds
    let max_ns_time: i128 = MAX_WAIT_FOR_IO.whole_nanoseconds();
    let max_ns_u = if max_ns_time > 0 { max_ns_time as u128 } else { 0u128 };

    // Clamp remaining time to MAX_WAIT_FOR_IO
    let wait_ns_u = core::cmp::min(remaining_ns, max_ns_u);

    // Bound to i64 range and return as `time::Duration`
    let wait_ns_i64 = if wait_ns_u > i64::MAX as u128 {
        i64::MAX
    } else {
        wait_ns_u as i64
    };

    Duration::nanoseconds(wait_ns_i64)
}
