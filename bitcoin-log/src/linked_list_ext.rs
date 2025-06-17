// ---------------- [ File: bitcoin-log/src/linked_list_ext.rs ]
crate::ix!();

/// A convenience extension: `is_empty()` on LinkedList
pub trait LinkedListExt<T> {
    fn is_empty(&self) -> bool;
}

impl<T> LinkedListExt<T> for LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
