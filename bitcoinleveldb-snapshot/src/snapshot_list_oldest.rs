// ---------------- [ File: bitcoinleveldb-snapshot/src/snapshot_list_oldest.rs ]
crate::ix!();

impl SnapshotList {
    
    pub fn oldest(&self) -> *mut SnapshotImpl {
        debug!("requesting oldest SnapshotImpl from SnapshotList");

        debug_assert!(
            !self.empty(),
            "SnapshotList::oldest called on an empty list"
        );

        let head_ptr: *const SnapshotImpl = self.head_const_ptr();

        let oldest_ptr: *mut SnapshotImpl = unsafe {
            let head_ref: &SnapshotImpl = &*head_ptr;
            *head_ref.next()
        };

        trace!(
            oldest_ptr = ?oldest_ptr,
            "returning oldest SnapshotImpl pointer"
        );

        oldest_ptr
    }

    pub fn newest(&self) -> *mut SnapshotImpl {
        debug!("requesting newest SnapshotImpl from SnapshotList");

        debug_assert!(
            !self.empty(),
            "SnapshotList::newest called on an empty list"
        );

        let head_ptr: *const SnapshotImpl = self.head_const_ptr();

        let newest_ptr: *mut SnapshotImpl = unsafe {
            let head_ref: &SnapshotImpl = &*head_ptr;
            *head_ref.prev()
        };

        trace!(
            newest_ptr = ?newest_ptr,
            "returning newest SnapshotImpl pointer"
        );

        newest_ptr
    }
}

#[cfg(test)]
mod snapshot_list_ordering_semantics_spec {
    use super::*;

    #[traced_test]
    fn oldest_and_newest_coincide_for_singleton_list() {
        debug!("verifying oldest/newest coincide when only one element exists");

        let mut list = SnapshotList::default();
        let s = list.new(77);

        let oldest = list.oldest();
        let newest = list.newest();

        trace!(s = ?s, oldest = ?oldest, newest = ?newest, "singleton pointers");

        assert_eq!(oldest, s, "oldest must equal the only element");
        assert_eq!(newest, s, "newest must equal the only element");
    }

    #[traced_test]
    fn oldest_points_to_first_and_newest_points_to_last_after_multiple_inserts() {
        debug!("verifying ordering semantics after multiple insertions");

        let mut list = SnapshotList::default();
        let a = list.new(1);
        let _ = list.new(2);
        let c = list.new(3);

        let oldest = list.oldest();
        let newest = list.newest();

        trace!(a = ?a, c = ?c, oldest = ?oldest, newest = ?newest, "ordering pointers");

        assert_eq!(oldest, a, "oldest must be the first inserted snapshot");
        assert_eq!(newest, c, "newest must be the last inserted snapshot");
    }
}
