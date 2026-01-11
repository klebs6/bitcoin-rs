crate::ix!();

pub(crate) fn time_duration_to_time_point_duration(delta: Duration) -> time_point::Duration {
    let nanos: i128 = delta.whole_nanoseconds();
    let nanos: i64 = nanos
        .try_into()
        .expect("time::Duration out of range for time_point::Duration");
    time_point::Duration::new(nanos)
}
