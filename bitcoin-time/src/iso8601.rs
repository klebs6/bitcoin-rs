// ---------------- [ File: bitcoin-time/src/iso8601.rs ]
crate::ix!();

use chrono::format::parse;

/// Format an epoch value (`n_time`) as ISO‑8601 `YYYY‑MM‑DDTHH:MM:SSZ`.
pub fn format_iso8601date_time(n_time: i64) -> String {
    let fmt = parse("[year]-[month]-[day]T[hour]:[minute]:[second]Z").unwrap();
    let out = OffsetDateTime::from_unix_timestamp(n_time)
        .map(|dt| dt.format(&fmt).unwrap_or_default())
        .unwrap_or_default();
    trace!(epoch = n_time, iso = %out, "format_iso8601date_time");
    out
}

/// Format an epoch value (`n_time`) as ISO‑8601 `YYYY‑MM‑DD`.
pub fn format_iso8601date(n_time: i64) -> String {
    let fmt = parse("[year]-[month]-[day]").unwrap();
    let out = OffsetDateTime::from_unix_timestamp(n_time)
        .map(|dt| dt.format(&fmt).unwrap_or_default())
        .unwrap_or_default();
    trace!(epoch = n_time, iso = %out, "format_iso8601date");
    out
}

/// Parse an ISO‑8601 `YYYY‑MM‑DDTHH:MM:SSZ` string into epoch seconds.
/// Returns `0` on failure.
pub fn parse_iso8601date_time(s: &str) -> i64 {
    let fmt = parse("[year]-[month]-[day]T[hour]:[minute]:[second]Z").unwrap();
    match OffsetDateTime::parse(s, &fmt) {
        Ok(dt) => {
            let ts = dt.unix_timestamp();
            trace!(iso = s, epoch = ts, "parse_iso8601date_time");
            ts
        }
        Err(e) => {
            warn!(iso = s, error = %e, "parse_iso8601date_time_failed");
            0
        }
    }
}
