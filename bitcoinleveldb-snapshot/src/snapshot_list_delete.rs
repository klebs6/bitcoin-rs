// ---------------- [ File: bitcoinleveldb-snapshot/src/snapshot_list_delete.rs ]
crate::ix!();

impl SnapshotList {

    /// Removes a SnapshotImpl from this list.
    ///
    /// The snapshot must have been created by calling New() on this list.
    ///
    /// The snapshot pointer should not be const, because its memory is
    /// deallocated. However, that would force us to change
    /// DB::ReleaseSnapshot(), which is in the API, and currently takes a const
    /// Snapshot.
    pub fn delete(&mut self, snapshot: *const SnapshotImpl) {
        debug!(snapshot_ptr = ?snapshot, "deleting SnapshotImpl from SnapshotList");

        unsafe {
            if snapshot.is_null() {
                warn!("SnapshotList::delete called with null snapshot pointer");
                return;
            }

            let head_addr: *const SnapshotImpl = self.head_const_ptr();

            debug_assert!(
                snapshot != head_addr,
                "SnapshotList::delete must not be called on the list head"
            );

            let snapshot_mut: *mut SnapshotImpl = snapshot as *mut SnapshotImpl;

            #[cfg(not(NDEBUG))]
            {
                let expected_list: *mut SnapshotList = self as *mut SnapshotList;
                let actual_list: *mut SnapshotList = *(*snapshot_mut).list();
                debug_assert_eq!(
                    actual_list,
                    expected_list,
                    "SnapshotList::delete: snapshot list pointer does not match"
                );
            }

            let prev_ptr: *mut SnapshotImpl = *(*snapshot_mut).prev();
            let next_ptr: *mut SnapshotImpl = *(*snapshot_mut).next();

            debug_assert!(
                !prev_ptr.is_null(),
                "SnapshotList::delete: prev_ptr is null"
            );
            debug_assert!(
                !next_ptr.is_null(),
                "SnapshotList::delete: next_ptr is null"
            );

            {
                let prev_ref: &mut SnapshotImpl = &mut *prev_ptr;
                prev_ref.set_next(next_ptr);
            }

            {
                let next_ref: &mut SnapshotImpl = &mut *next_ptr;
                next_ref.set_prev(prev_ptr);
            }

            {
                let snapshot_ref: &mut SnapshotImpl = &mut *snapshot_mut;

                #[cfg(not(NDEBUG))]
                {
                    snapshot_ref.set_list(core::ptr::null_mut());
                }

                snapshot_ref.set_prev(core::ptr::null_mut());
                snapshot_ref.set_next(core::ptr::null_mut());
            }

            trace!(
                snapshot_mut_ptr = ?snapshot_mut,
                prev_ptr = ?prev_ptr,
                next_ptr = ?next_ptr,
                "unlinked SnapshotImpl from SnapshotList, deallocating"
            );

            drop(Box::from_raw(snapshot_mut));
        }
    }
}

#[cfg(test)]
mod snapshot_list_deletion_behaviour_spec {
    use super::*;

    #[traced_test]
    fn deleting_null_pointer_is_a_noop() {
        debug!("calling delete with a null pointer should be a no-op");

        let mut list = SnapshotList::default();
        assert!(list.empty(), "list should start empty");

        // SAFETY: API accepts a raw pointer; we intentionally pass null.
        let null_ptr: *const SnapshotImpl = core::ptr::null();
        list.delete(null_ptr);

        assert!(list.empty(), "list must remain empty after null delete");
    }

    #[traced_test]
    fn deleting_only_element_restores_empty_state() {
        debug!("inserting a single element then deleting it to restore empty state");

        let mut list = SnapshotList::default();
        let s = list.new(5);

        assert!(!list.empty(), "list must be non-empty after insertion");

        list.delete(s as *const SnapshotImpl);

        assert!(list.empty(), "list must be empty after removing the only element");
    }

    #[traced_test]
    fn deleting_middle_element_relinks_neighbors() {
        debug!("inserting three elements and deleting the middle one");

        let mut list = SnapshotList::default();
        let s1 = list.new(1);
        let s2 = list.new(2);
        let s3 = list.new(3);

        list.delete(s2 as *const SnapshotImpl);

        let oldest = list.oldest();
        let newest = list.newest();

        trace!(oldest = ?oldest, newest = ?newest, "pointers after middle deletion");

        assert_eq!(oldest, s1, "oldest must remain first element");
        assert_eq!(newest, s3, "newest must remain last element");

        let oldest_seq = unsafe { *(*oldest).sequence_number() };
        let newest_seq = unsafe { *(*newest).sequence_number() };

        assert_eq!(oldest_seq, 1, "oldest sequence remains unchanged");
        assert_eq!(newest_seq, 3, "newest sequence remains unchanged");
    }
}
