// ---------------- [ File: bitcoinleveldb-snapshot/src/snapshot_list_empty.rs ]
crate::ix!();

impl SnapshotList {
    
    pub fn empty(&self) -> bool {
        let head_ptr: *const SnapshotImpl = self.head_const_ptr();

        let next_ptr: *mut SnapshotImpl = unsafe {
            let head_ref: &SnapshotImpl = &*head_ptr;
            *head_ref.next()
        };

        let is_empty: bool = core::ptr::eq(next_ptr as *const SnapshotImpl, head_ptr);

        trace!(
            head_ptr = ?head_ptr,
            next_ptr = ?next_ptr,
            is_empty,
            "evaluated SnapshotList::empty"
        );

        is_empty
    }
}

#[cfg(test)]
mod snapshot_list_emptiness_semantics_spec {
    use super::*;

    #[traced_test]
    fn default_list_reports_empty() {
        debug!("verifying empty() on default list");

        let list = SnapshotList::default();
        assert!(list.empty(), "default list must be empty()");
    }

    #[traced_test]
    fn list_reports_non_empty_after_insertion_and_empty_after_all_deletions() {
        debug!("verifying empty() transitions with insertions and deletions");

        let mut list = SnapshotList::default();

        let a = list.new(10);
        assert!(!list.empty(), "list must become non-empty after insertion");

        list.delete(a as *const SnapshotImpl);
        assert!(list.empty(), "list must become empty after removing the only element");

        let b = list.new(20);
        let c = list.new(30);

        assert!(!list.empty(), "list must be non-empty with multiple elements");

        list.delete(b as *const SnapshotImpl);
        assert!(!list.empty(), "list must remain non-empty with one element left");

        list.delete(c as *const SnapshotImpl);
        assert!(list.empty(), "list must be empty after removing the last element");
    }
}
