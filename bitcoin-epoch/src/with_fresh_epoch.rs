// ---------------- [ File: bitcoin-epoch/src/with_fresh_epoch.rs ]
crate::ix!();

/// Convenience macro mirroring the original C++
/// `WITH_FRESH_EPOCH` helper.
///
/// ```text
/// {
///     with_fresh_epoch!(mempool_epoch);
///     … // traversal
/// }   // guard dropped here
/// ```
#[macro_export]
macro_rules! with_fresh_epoch {
    ($epoch:expr) => {
        let _epoch_guard = $crate::EpochGuard::new($epoch.clone());
    };
}
