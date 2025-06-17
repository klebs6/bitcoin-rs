// ---------------- [ File: bitcoin-time/src/sleep.rs ]
crate::ix!();

/// Sleep for the given duration without interruption.
pub fn uninterruptible_sleep(n: std::time::Duration) {             // CHANGED SIGNATURE
    debug!(?n, "uninterruptible_sleep");
    let std_dur: std::time::Duration = n
        .try_into()
        .expect("negative duration passed to uninterruptible_sleep");
    std::thread::sleep(std_dur);
}

