// ---------------- [ File: bitcoinleveldb-snapshot/src/snapshot_list_create.rs ]
crate::ix!();

impl SnapshotList {

    /// Creates a SnapshotImpl and appends it to the end of the list.
    pub fn new(&mut self, sequence_number: SequenceNumber) -> *mut SnapshotImpl {
        debug!(
            sequence_number = sequence_number,
            "creating new SnapshotImpl in SnapshotList"
        );

        unsafe {
            if !self.empty() {
                let newest_ptr: *mut SnapshotImpl = self.newest();
                let newest_seq_ref: &SequenceNumber = (*newest_ptr).sequence_number();
                debug_assert!(
                    *newest_seq_ref <= sequence_number,
                    "SnapshotList::new must be called with monotonically increasing sequence numbers"
                );
            }

            let snapshot = Box::new(SnapshotImpl::new(sequence_number));

            #[cfg(not(NDEBUG))]
            let mut snapshot = {
                let mut s = snapshot;
                s.set_list(self as *mut SnapshotList);
                s
            };

            #[cfg(NDEBUG)]
            let snapshot = snapshot;

            let snapshot_ptr: *mut SnapshotImpl = Box::into_raw(snapshot);

            // Pointer to the dummy head node (sentinel)
            let head_ptr: *mut SnapshotImpl = self.head_mut_ptr();

            // Previous tail before inserting the new snapshot
            let prev_ptr: *mut SnapshotImpl = {
                let head_ref: &SnapshotImpl = &*head_ptr;
                *head_ref.prev()
            };

            debug_assert!(
                !prev_ptr.is_null(),
                "SnapshotList::new: prev_ptr is null"
            );

            {
                let snapshot_ref: &mut SnapshotImpl = &mut *snapshot_ptr;
                snapshot_ref.set_next(head_ptr);
                snapshot_ref.set_prev(prev_ptr);
            }

            {
                let prev_ref: &mut SnapshotImpl = &mut *prev_ptr;
                prev_ref.set_next(snapshot_ptr);
            }

            {
                let head_ref_mut: &mut SnapshotImpl = &mut *head_ptr;
                head_ref_mut.set_prev(snapshot_ptr);
            }

            trace!(
                snapshot_ptr = ?snapshot_ptr,
                head_ptr = ?head_ptr,
                prev_ptr = ?prev_ptr,
                "linked new SnapshotImpl into SnapshotList"
            );

            snapshot_ptr
        }
    }
}

#[cfg(test)]
mod snapshot_list_creation_behaviour_spec {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[traced_test]
    fn inserting_first_snapshot_initializes_nonempty_list() {
        debug!("inserting first snapshot and validating non-empty state");

        let mut list = SnapshotList::default();

        assert!(
            list.empty(),
            "freshly constructed list must report empty() == true"
        );

        let first = list.new(10 as SequenceNumber);

        assert!(
            !list.empty(),
            "list must report empty() == false after first insertion"
        );

        let oldest = list.oldest();
        let newest = list.newest();

        trace!(
            first_ptr  = ?first,
            oldest_ptr = ?oldest,
            newest_ptr = ?newest,
            "examining pointers after first insertion"
        );

        assert_eq!(oldest, first, "oldest must point to the first element");
        assert_eq!(newest, first, "newest must point to the first element");

        let seq = unsafe { *(*first).sequence_number() };
        assert_eq!(seq, 10, "stored sequence must match insertion value");
    }

    #[traced_test]
    fn inserting_with_monotonically_increasing_sequences_succeeds() {
        debug!("inserting multiple snapshots with increasing sequence numbers");

        let mut list = SnapshotList::default();
        let s1 = list.new(1);
        let s2 = list.new(2);
        let s3 = list.new(3);

        let oldest = list.oldest();
        let newest = list.newest();

        trace!(
            s1 = ?s1, s2 = ?s2, s3 = ?s3, oldest = ?oldest, newest = ?newest,
            "verifying ordering after multiple insertions"
        );

        assert_eq!(oldest, s1, "oldest must be the first inserted");
        assert_eq!(newest, s3, "newest must be the last inserted");

        let oldest_seq = unsafe { *(*oldest).sequence_number() };
        let newest_seq = unsafe { *(*newest).sequence_number() };

        assert_eq!(oldest_seq, 1, "oldest sequence must match first insertion");
        assert_eq!(newest_seq, 3, "newest sequence must match last insertion");
    }

    #[traced_test]
    fn inserting_non_monotonic_sequence_panics_in_debug_builds() {
        debug!("verifying debug assertion triggers on non-monotonic insertion");

        let mut list = SnapshotList::default();
        let _ = list.new(100);

        let res = catch_unwind(AssertUnwindSafe(|| {
            let _ = list.new(99);
        }));

        // In debug builds this should panic due to debug_assert!, in release it may pass.
        if cfg!(debug_assertions) {
            assert!(
                res.is_err(),
                "in debug builds inserting a smaller sequence must panic"
            );
        } else {
            assert!(
                res.is_ok(),
                "in release builds debug assertion is disabled and should not panic"
            );
        }
    }
}
