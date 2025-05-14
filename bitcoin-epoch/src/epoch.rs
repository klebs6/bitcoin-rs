// ---------------- [ File: bitcoin-epoch/src/epoch.rs ]
crate::ix!();

/** 
 | Epoch: RAII-style guard for using epoch-based
 | graph traversal algorithms.
 |
 | When walking ancestors or descendants, we
 | generally want to avoid visiting the same
 | transactions twice. Some traversal algorithms
 | use std::set (or setEntries) to deduplicate the
 | transaction we visit.
 |
 | However, use of std::set is algorithmically
 | undesirable because it both adds an asymptotic
 | factor of O(log n) to traversals cost and
 | triggers O(n) more dynamic memory allocations.
 |
 | In many algorithms we can replace std::set with
 | an internal mempool counter to track the time
 | (or, "epoch") that we began a traversal, and
 | check + update a per-transaction epoch for each
 | transaction we look at to determine if that
 | transaction has not yet been visited during the
 | current traversal's epoch.
 |
 | Algorithms using std::set can be replaced on
 | a one by one basis.  Both techniques are not
 | fundamentally incompatible across the codebase.
 | Generally speaking, however, the remaining use
 | of std::set for mempool traversal should be
 | viewed as a TODO for replacement with an epoch
 | based traversal, rather than a preference for
 | std::set over epochs in that algorithm.
 */
#[LOCKABLE]
pub struct Epoch {
    pub(crate) raw_epoch: u64,
    pub(crate) guarded:   bool,
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
    
    pub fn guarded(&self) -> bool {
        self.guarded
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(*this)]
    pub fn visited(&self, marker: &mut EpochMarker) -> bool {
        
        assert!(self.guarded);

        if marker.marker < self.raw_epoch {

            // marker is from a previous epoch, so
            // this is its first visit
            marker.marker = self.raw_epoch;

            false

        } else {

            true
        }
    }
}
