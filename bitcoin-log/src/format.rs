// ---------------- [ File: bitcoin-log/src/format.rs ]
crate::ix!();

pub fn format_iso8601_datetime(secs: i64) -> String {
    use chrono::{TimeZone, Utc, LocalResult};

    // New approach: match on `timestamp_opt`
    let dt = match Utc.timestamp_opt(secs, 0) {
        LocalResult::Single(x) => x,
        // If None or Ambiguous, just fallback to epoch
        _ => Utc.timestamp(0, 0),
    };

    // Format as RFC3339 with "Z" at the end
    dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}
