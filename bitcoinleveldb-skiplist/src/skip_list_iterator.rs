// ---------------- [ File: bitcoinleveldb-skiplist/src/skip_list_iterator.rs ]
crate::ix!();

/// Skiplist iterator.
///
/// Intentionally copyable. It holds an immutable reference to the list
/// and a raw pointer to the current node.
pub struct SkipListIterator<'a, K, C>
where
    K: core::fmt::Debug + Copy + Default,
    C: SkipListComparator<K>,
{
    list: &'a SkipList<K, C>,
    node: *mut SkipListNode<K>,
}

impl<'a, K, C> Clone for SkipListIterator<'a, K, C>
where
    K: core::fmt::Debug + Copy + Default,
    C: SkipListComparator<K>,
{
    fn clone(&self) -> Self {
        trace!(
            "SkipListIterator::clone: list={:p}, node={:p}",
            self.list,
            self.node
        );
        Self {
            list: self.list,
            node: self.node,
        }
    }
}

impl<'a, K, C> SkipListIterator<'a, K, C>
where
    K: core::fmt::Debug + Copy + Default,
    C: SkipListComparator<K>,
{
    /// Initialize an iterator over the specified list.
    /// The returned iterator is not valid.
    pub fn new(list: &'a SkipList<K, C>) -> Self {
        trace!("SkipListIterator::new: list={:p}", list);
        Self {
            list,
            node: core::ptr::null_mut(),
        }
    }

    /// Returns true iff the iterator is positioned at a valid node.
    #[inline]
    pub fn valid(&self) -> bool {
        let v = !self.node.is_null();
        trace!("SkipListIterator::valid -> {}", v);
        v
    }

    /// Returns the key at the current position.
    /// REQUIRES: Valid()
    #[inline]
    pub fn key(&self) -> K {
        assert!(
            self.valid(),
            "SkipListIterator::key called on invalid iterator"
        );
        unsafe {
            let v = (*self.node).key_ref();
            trace!("SkipListIterator::key -> {:?}", v);
            *v
        }
    }

    /// Advances to the next position.
    /// REQUIRES: Valid()
    #[inline]
    pub fn next(&mut self) {
        assert!(
            self.valid(),
            "SkipListIterator::next called on invalid iterator"
        );
        unsafe {
            // Call `next` on a shared reference to the current node.
            let node_ref: &SkipListNode<K> = &*self.node;
            self.node = node_ref.next(0);
            trace!("SkipListIterator::next: node={:p}", self.node);
        }
    }

    /// Advances to the previous position.
    /// REQUIRES: Valid()
    #[inline]
    pub fn prev(&mut self) {
        assert!(
            self.valid(),
            "SkipListIterator::prev called on invalid iterator"
        );
        let cur = self.key();
        unsafe {
            let mut x = self.list.find_less_than(&cur);
            let head_ptr: *mut SkipListNode<K> = *self.list.head();
            if x == head_ptr {
                x = core::ptr::null_mut();
            }
            self.node = x;
            trace!(
                "SkipListIterator::prev: cur={:?}, node={:p}",
                cur,
                self.node
            );
        }
    }

    /// Advance to the first entry with a key >= target
    #[inline]
    pub fn seek(&mut self, target: &K) {
        trace!("SkipListIterator::seek: target={:?}", target);
        let x = self.list.find_greater_or_equal(target, None);
        self.node = x;
        trace!("SkipListIterator::seek: node={:p}", self.node);
    }

    /// Position at the first entry in list.
    /// Final state of iterator is Valid() iff list is not empty.
    #[inline]
    pub fn seek_to_first(&mut self) {
        trace!("SkipListIterator::seek_to_first");
        unsafe {
            let head_ptr: *mut SkipListNode<K> = *self.list.head();
            let next_ptr = match head_ptr.as_ref() {
                Some(head_node) => {
                    let next = head_node.next(0);
                    trace!(
                        "SkipListIterator::seek_to_first: head={:p}, next={:p}",
                        head_ptr,
                        next
                    );
                    next
                }
                None => {
                    warn!(
                        "SkipListIterator::seek_to_first: head pointer is null; iterator will be invalid"
                    );
                    core::ptr::null_mut()
                }
            };
            self.node = next_ptr;
            trace!("SkipListIterator::seek_to_first: node={:p}", self.node);
        }
    }

    /// Position at the last entry in list.
    /// Final state of iterator is Valid() iff list is not empty.
    #[inline]
    pub fn seek_to_last(&mut self) {
        trace!("SkipListIterator::seek_to_last");
        unsafe {
            let mut x = self.list.find_last();
            let head_ptr: *mut SkipListNode<K> = *self.list.head();
            if x == head_ptr {
                x = core::ptr::null_mut();
            }
            self.node = x;
            trace!("SkipListIterator::seek_to_last: node={:p}", self.node);
        }
    }
}

#[cfg(test)]
mod iterator_behavior_suite {
    use super::*;
    use bitcoin_imports::*;
    use bitcoinleveldb_arena::Arena;

    struct U64Cmp;
    impl SkipListComparator<u64> for U64Cmp {
        #[inline]
        fn compare(&self, a: &u64, b: &u64) -> i32 {
            if a < b { -1 } else if a > b { 1 } else { 0 }
        }
    }

    fn setup_list_with_keys(keys: &[u64]) -> (Arena, SkipList<u64, U64Cmp>) {
        let mut arena = Arena::default();
        let mut list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);
        for &k in keys {
            list.insert(k);
        }
        (arena, list)
    }

    #[traced_test]
    fn iterator_on_empty_list_is_always_invalid() {
        info!("iterator_on_empty_list_is_always_invalid: start");

        let mut arena = Arena::default();
        let list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);

        let mut it = SkipListIterator::new(&list);
        assert!(!it.valid(), "fresh iterator must be invalid on empty list");

        it.seek_to_first();
        assert!(!it.valid(), "seek_to_first must remain invalid on empty list");

        it.seek(&100);
        assert!(!it.valid(), "seek must remain invalid on empty list");

        it.seek_to_last();
        assert!(!it.valid(), "seek_to_last must remain invalid on empty list");

        info!("iterator_on_empty_list_is_always_invalid: done");
    }

    #[traced_test]
    fn iterator_forward_walks_in_sorted_order() {
        info!("iterator_forward_walks_in_sorted_order: start");

        let (_arena, list) = setup_list_with_keys(&[1, 3, 4, 7, 9]);

        let mut it = SkipListIterator::new(&list);
        it.seek_to_first();
        assert!(it.valid(), "iterator must be valid at first element");
        assert_eq!(it.key(), 1);

        it.next(); assert!(it.valid()); assert_eq!(it.key(), 3);
        it.next(); assert!(it.valid()); assert_eq!(it.key(), 4);
        it.next(); assert!(it.valid()); assert_eq!(it.key(), 7);
        it.next(); assert!(it.valid()); assert_eq!(it.key(), 9);
        it.next(); assert!(!it.valid(), "iterator must be invalid after last element");

        info!("iterator_forward_walks_in_sorted_order: done");
    }

    #[traced_test]
    fn iterator_backward_from_last_to_before_first() {
        info!("iterator_backward_from_last_to_before_first: start");

        let (_arena, list) = setup_list_with_keys(&[2, 5, 8]);

        let mut it = SkipListIterator::new(&list);
        it.seek_to_last();
        assert!(it.valid(), "seek_to_last must position at last element");
        assert_eq!(it.key(), 8);

        it.prev(); assert!(it.valid()); assert_eq!(it.key(), 5);
        it.prev(); assert!(it.valid()); assert_eq!(it.key(), 2);
        it.prev(); assert!(!it.valid(), "prev past first must invalidate iterator");

        info!("iterator_backward_from_last_to_before_first: done");
    }

    #[traced_test]
    fn iterator_seek_finds_first_ge_target() {
        info!("iterator_seek_finds_first_ge_target: start");

        let (_arena, list) = setup_list_with_keys(&[10, 20, 30, 40]);

        let mut it = SkipListIterator::new(&list);

        it.seek(&5);  assert!(it.valid());  assert_eq!(it.key(), 10);
        it.seek(&10); assert!(it.valid());  assert_eq!(it.key(), 10);
        it.seek(&25); assert!(it.valid());  assert_eq!(it.key(), 30);
        it.seek(&40); assert!(it.valid());  assert_eq!(it.key(), 40);
        it.seek(&41); assert!(!it.valid(), "seek past max must invalidate iterator");

        info!("iterator_seek_finds_first_ge_target: done");
    }
}
