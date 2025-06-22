// ---------------- [ File: bitcoin-epoch/src/epoch.rs ]
crate::ix!();

/// RAII-style epoch tracker used to de‑duplicate mempool graph traversals.
///
/// See the original C++ documentation for the algorithmic rationale.
///
/// Epoch: RAII-style guard for using epoch-based graph traversal algorithms.
/// 
/// When walking ancestors or descendants, we generally want to avoid visiting the same
/// transactions twice. Some traversal algorithms use std::set (or setEntries) to deduplicate the
/// transaction we visit.
/// 
/// However, use of std::set is algorithmically undesirable because it both adds an asymptotic
/// factor of O(log n) to traversals cost and triggers O(n) more dynamic memory allocations.
/// 
/// In many algorithms we can replace std::set with an internal mempool counter to track the time
/// (or, "epoch") that we began a traversal, and check + update a per-transaction epoch for each
/// transaction we look at to determine if that transaction has not yet been visited during the
/// current traversal's epoch.
/// 
/// Algorithms using std::set can be replaced on a one by one basis.  Both techniques are not
/// fundamentally incompatible across the codebase. Generally speaking, however, the remaining use
/// of std::set for mempool traversal should be viewed as a TODO for replacement with an epoch
/// based traversal, rather than a preference for std::set over epochs in that algorithm.
#[derive(Getters, Builder)]
#[getset(get = "pub")]
#[builder(setter(into), default)]
#[LOCKABLE]
pub struct Epoch {
    /// Monotonically‑increasing traversal counter.
    raw_epoch: u64,

    /// `true` while an `EpochGuard` is alive.
    guarded: bool,
}

impl Default for Epoch {

    fn default() -> Self {
        Self {
            raw_epoch: 0,
            guarded:   false,
        }
    }
}

impl Epoch {
    
    /// `true` if `marker` was already visited *during
    /// the **current** epoch*.
    #[EXCLUSIVE_LOCKS_REQUIRED(*this)]
    pub fn visited(&self, marker: &mut EpochMarker) -> bool {
        trace!(
            target: "epoch",
            "visited? epoch={}, marker={}",
            self.raw_epoch,
            marker.marker()
        );

        assert!(
            self.guarded,
            "visited() called outside of an EpochGuard scope"
        );

        if *marker.marker() < self.raw_epoch {
            // first visit this epoch
            marker.update(self.raw_epoch);
            trace!(target: "epoch", "first visit this epoch");
            false
        } else {
            true
        }
    }

    /// Increment the internal counter; crate‑internal.
    pub(crate) fn increment_epoch(&mut self) {
        self.raw_epoch = self
            .raw_epoch
            .checked_add(1)
            .expect("epoch counter overflow");
        trace!(target: "epoch", "epoch incremented → {}", self.raw_epoch);
    }

    /// Set/clear the guarded flag; crate‑internal.
    pub(crate) fn set_guarded(&mut self, value: bool) {
        trace!(target: "epoch", "guarded flag set → {}", value);
        self.guarded = value;
    }
}
