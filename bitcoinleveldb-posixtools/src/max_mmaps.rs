// ---------------- [ File: bitcoinleveldb-posixtools/src/max_mmaps.rs ]
crate::ix!();

/// Return the maximum number of concurrent
/// mmaps.
/// 
pub fn max_mmaps() -> i32 {
    use std::sync::atomic::Ordering;

    let limit = MMAP_LIMIT.load(Ordering::SeqCst);
    trace!(
        limit,
        "max_mmaps: returning configured mmap limit"
    );
    limit
}
