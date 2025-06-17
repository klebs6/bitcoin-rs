// ---------------- [ File: bitcoin-time/src/mock_time.rs ]
crate::ix!();

lazy_static! {
    /// Mock‑time value in *seconds* since the Unix epoch.  
    /// A value of `0` means “no mock time”.
    pub static ref MOCK_TIME: AtomicI64 = AtomicI64::new(0);
}

/// For testing – set mock time (in *seconds* since the Unix epoch).
pub fn set_mock_time(mock_time_in: Instant) {
    let secs = mock_time_in.elapsed().as_secs() as i64;
    MOCK_TIME.store(secs, atomic::Ordering::Relaxed);
    info!(mock_seconds = secs, "set_mock_time");
}
