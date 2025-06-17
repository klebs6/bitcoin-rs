// ---------------- [ File: bitcoin-time/src/sleep.rs ]
crate::ix!();

/// Sleep for the given duration without interruption.
pub fn uninterruptible_sleep(n: Duration) {
    debug!(?n, "uninterruptible_sleep");
    std::thread::sleep(n);
}
