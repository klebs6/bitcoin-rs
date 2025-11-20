// ---------------- [ File: bitcoinleveldb-snapshot/src/snapshot_list.rs ]
crate::ix!();

#[derive(Getters,Setters)]
#[getset(get="pub",set="pub")]
pub struct SnapshotList {
    /// Dummy head of doubly-linked list of snapshots.
    ///
    /// We store the sentinel on the heap so that its address is **stable across moves**.
    /// This mirrors the C++ self-referential sentinel while remaining sound in Rust.
    head: Box<SnapshotImpl>,
}

impl Default for SnapshotList {
    fn default() -> Self {
        debug!("initializing empty SnapshotList (heap-pinned sentinel)");

        // Allocate the sentinel on the heap so its address will not change if `SnapshotList` moves.
        let mut list = SnapshotList {
            head: Box::new(SnapshotImpl::new(0 as SequenceNumber)),
        };

        // SAFELY obtain a mutable raw pointer to the heap-allocated head
        let head_ptr: *mut SnapshotImpl = &mut *list.head;

        unsafe {
            // Self-link the sentinel
            (*head_ptr).set_prev(head_ptr);
            (*head_ptr).set_next(head_ptr);
        }

        trace!(
            head_ptr = ?head_ptr,
            "initialized SnapshotList head pointers (stable heap address)"
        );

        list
    }

}

impl SnapshotList {

    /// Returns a mutable raw pointer to the dummy head node of this list.
    ///
    /// The head node is heap-allocated to ensure a **stable address**
    /// even if `SnapshotList` is moved. This avoids self-reference invalidation.
    pub fn head_mut_ptr(&mut self) -> *mut SnapshotImpl {
        let ptr: *mut SnapshotImpl = &mut *self.head;
        trace!(
            head_ptr = ?ptr,
            "SnapshotList::head_mut_ptr returning mutable raw pointer to list head"
        );
        ptr
    }

    /// Returns a const raw pointer to the dummy head node of this list.
    pub fn head_const_ptr(&self) -> *const SnapshotImpl {
        let ptr: *const SnapshotImpl = &*self.head;
        trace!(
            head_ptr = ?ptr,
            "SnapshotList::head_const_ptr returning const raw pointer to list head"
        );
        ptr
    }

}

#[cfg(test)]
mod snapshot_list_behaviour_tests {
    use super::*;

    #[traced_test]
    fn snapshot_list_default_state_is_empty() {
        debug!("starting snapshot_list_default_state_is_empty");

        let list = SnapshotList::default();

        assert!(
            list.empty(),
            "a newly created SnapshotList should report as empty"
        );

        debug!("completed snapshot_list_default_state_is_empty");
    }

    #[traced_test]
    fn snapshot_list_append_single_snapshot_behaves_as_expected() {
        debug!("starting snapshot_list_append_single_snapshot_behaves_as_expected");

        let mut list = SnapshotList::default();
        let snapshot_ptr = list.new(10);

        assert!(
            !list.empty(),
            "SnapshotList should not be empty after inserting a snapshot"
        );

        let oldest_ptr = list.oldest();
        let newest_ptr = list.newest();

        assert_eq!(
            oldest_ptr, snapshot_ptr,
            "oldest snapshot pointer should match inserted snapshot"
        );
        assert_eq!(
            newest_ptr, snapshot_ptr,
            "newest snapshot pointer should match inserted snapshot"
        );

        let sequence = unsafe { (*snapshot_ptr).sequence_number() };

        assert_eq!(
            *sequence, 10,
            "sequence number stored in snapshot should match inserted value"
        );

        list.delete(snapshot_ptr as *const SnapshotImpl);

        assert!(
            list.empty(),
            "SnapshotList should be empty again after deleting the only snapshot"
        );

        debug!("completed snapshot_list_append_single_snapshot_behaves_as_expected");
    }

    #[traced_test]
    fn snapshot_list_append_multiple_snapshots_preserves_ordering() {
        debug!("starting snapshot_list_append_multiple_snapshots_preserves_ordering");

        let mut list = SnapshotList::default();

        let first_ptr = list.new(10);
        let second_ptr = list.new(20);
        let third_ptr = list.new(30);

        let oldest_ptr = list.oldest();
        let newest_ptr = list.newest();

        let oldest_sequence = unsafe { (*oldest_ptr).sequence_number() };
        let newest_sequence = unsafe { (*newest_ptr).sequence_number() };

        assert_eq!(
            oldest_ptr, first_ptr,
            "oldest snapshot should be the first one inserted"
        );
        assert_eq!(
            newest_ptr, third_ptr,
            "newest snapshot should be the last one inserted"
        );
        assert_eq!(
            *oldest_sequence, 10,
            "oldest snapshot sequence number should match first inserted value"
        );
        assert_eq!(
            *newest_sequence, 30,
            "newest snapshot sequence number should match last inserted value"
        );

        list.delete(first_ptr as *const SnapshotImpl);
        list.delete(second_ptr as *const SnapshotImpl);
        list.delete(third_ptr as *const SnapshotImpl);

        assert!(
            list.empty(),
            "SnapshotList should be empty after deleting all snapshots"
        );

        debug!("completed snapshot_list_append_multiple_snapshots_preserves_ordering");
    }

    #[traced_test]
    fn snapshot_list_delete_relinks_neighbors_correctly() {
        debug!("starting snapshot_list_delete_relinks_neighbors_correctly");

        let mut list = SnapshotList::default();

        let first_ptr = list.new(1);
        let second_ptr = list.new(2);
        let third_ptr = list.new(3);

        let oldest_before_delete = list.oldest();
        assert_eq!(
            oldest_before_delete, first_ptr,
            "expected first snapshot to be oldest before delete"
        );

        list.delete(second_ptr as *const SnapshotImpl);

        let oldest_after_delete = list.oldest();
        let newest_after_delete = list.newest();

        let oldest_sequence_after = unsafe { (*oldest_after_delete).sequence_number() };
        let newest_sequence_after = unsafe { (*newest_after_delete).sequence_number() };

        assert_eq!(
            oldest_after_delete, first_ptr,
            "oldest snapshot should still be the first one after deleting the middle snapshot"
        );
        assert_eq!(
            newest_after_delete, third_ptr,
            "newest snapshot should still be the last one after deleting the middle snapshot"
        );
        assert_eq!(
            *oldest_sequence_after, 1,
            "sequence number of oldest snapshot should remain unchanged"
        );
        assert_eq!(
            *newest_sequence_after, 3,
            "sequence number of newest snapshot should remain unchanged"
        );

        list.delete(first_ptr as *const SnapshotImpl);
        list.delete(third_ptr as *const SnapshotImpl);

        assert!(
            list.empty(),
            "SnapshotList should be empty after deleting remaining snapshots"
        );

        debug!("completed snapshot_list_delete_relinks_neighbors_correctly");
    }
}
