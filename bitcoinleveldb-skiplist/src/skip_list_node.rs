// ---------------- [ File: bitcoinleveldb-skiplist/src/skip_list_node.rs ]
crate::ix!();

/// Internal node type used by `SkipList`.
///
/// This struct is laid out so that we can store a variable number of
/// `next` pointers in arena-allocated memory, exactly like the C++
/// implementation does with a flexible array member.
#[repr(C)]
pub struct SkipListNode<K>
where
    K: Copy + Default,
{
    key:  K,

    /// Array of length equal to the node height.
    /// next_[0] is lowest level link.
    next: [AtomicPtr<SkipListNode<K>>; 1],
}

impl<K> SkipListNode<K>
where
    K: Copy + Default,
{
    #[inline]
    pub(crate) unsafe fn write_key_at(node: *mut Self, key: K) {
        trace!("SkipListNode::write_key_at: node={:p}", node);
        core::ptr::write(&mut (*node).key, key);
    }

    #[inline]
    pub fn key_ref(&self) -> &K {
        &self.key
    }

    #[inline]
    pub unsafe fn next_base_ptr(node: *mut Self) -> *mut AtomicPtr<Self> {
        (&mut (*node).next[0]) as *mut AtomicPtr<Self>
    }

    #[inline]
    pub unsafe fn next_slot(node: *mut Self, level: i32) -> *mut AtomicPtr<Self> {
        debug_assert!(level >= 0);
        Self::next_base_ptr(node).add(level as usize)
    }

    /// Load the `next` pointer at a given level with acquire semantics.
    #[inline]
    pub fn next(&self, level: i32) -> *mut Self {
        assert!(level >= 0, "SkipListNode::next: negative level {}", level);
        unsafe {
            let slot =
                Self::next_slot(self as *const Self as *mut Self, level);

            // Use an 'acquire load' so that we observe a fully initialized
            // version of the returned SkipListNode.
            (*slot).load(atomic::Ordering::Acquire)
        }
    }

    /// Store the `next` pointer at a given level with release semantics.
    #[inline]
    pub fn set_next(&self, level: i32, node: *mut Self) {
        assert!(
            level >= 0,
            "SkipListNode::set_next: negative level {}",
            level
        );
        unsafe {
            let slot =
                Self::next_slot(self as *const Self as *mut Self, level);
            // Use a 'release store' so that anybody who reads through this
            // pointer observes a fully initialized version of the inserted node.
            (*slot).store(node, atomic::Ordering::Release);
        }
    }

    /// Load the `next` pointer at a given level with relaxed semantics.
    #[inline]
    pub fn no_barrier_next(&self, level: i32) -> *mut Self {
        assert!(
            level >= 0,
            "SkipListNode::no_barrier_next: negative level {}",
            level
        );
        unsafe {
            let slot =
                Self::next_slot(self as *const Self as *mut Self, level);
            (*slot).load(atomic::Ordering::Relaxed)
        }
    }

    /// Store the `next` pointer at a given level with relaxed semantics.
    #[inline]
    pub fn no_barrier_set_next(&self, level: i32, node: *mut Self) {
        assert!(
            level >= 0,
            "SkipListNode::no_barrier_set_next: negative level {}",
            level
        );
        unsafe {
            let slot =
                Self::next_slot(self as *const Self as *mut Self, level);
            (*slot).store(node, atomic::Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod node_api_validation_suite {
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

    #[traced_test]
    fn node_key_write_and_read_back() {
        info!("node_key_write_and_read_back: start");

        let mut arena = Arena::default();
        let mut list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);

        let key = 123_u64;
        let height = 3_i32;
        let node = list.new_node(key, height);

        unsafe {
            let kref = (*node).key_ref();
            assert_eq!(*kref, key, "key_ref must return the key previously written");
        }

        info!("node_key_write_and_read_back: done");
    }

    #[traced_test]
    fn node_next_pointers_are_null_after_allocation() {
        info!("node_next_pointers_are_null_after_allocation: start");

        let mut arena = Arena::default();
        let mut list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);

        let node = list.new_node(11_u64, 4);

        for level in 0..4 {
            let next = unsafe { (&*node).no_barrier_next(level) };
            assert!(next.is_null(), "freshly allocated node must have null next at level {}", level);
        }

        info!("node_next_pointers_are_null_after_allocation: done");
    }

    #[traced_test]
    fn node_set_next_and_load_semantics_round_trip() {
        info!("node_set_next_and_load_semantics_round_trip: start");

        let mut arena = Arena::default();
        let mut list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);

        let a = list.new_node(1, 3);
        let b = list.new_node(2, 3);

        unsafe {
            (&*a).set_next(1, b);
            let acq = (&*a).next(1);
            let rlx = (&*a).no_barrier_next(1);

            assert_eq!(acq, b, "acquire load must see the link");
            assert_eq!(rlx, b, "relaxed load must see the link written by set_next");
        }

        info!("node_set_next_and_load_semantics_round_trip: done");
    }
}
