// ---------------- [ File: bitcoinleveldb-lru/src/lru_cache_inner.rs ]
crate::ix!();

pub struct LRUCacheInner {

    usage: usize,

    /// Dummy head of LRU list.
    ///
    /// lru.prev is newest entry, lru.next is oldest entry.
    ///
    /// Entries have refs==1 and in_cache==true.
    lru:   Box<LRUHandle>,

    /// Dummy head of in-use list.
    ///
    /// Entries are in use by clients, and have
    /// refs >= 2 and in_cache==true.
    in_use: Box<LRUHandle>,

    table: HandleTable,
}

impl Default for LRUCacheInner {

    fn default() -> Self {
        LRUCacheInner::new()
    }
}

impl LRUCacheInner {

    /// Construct an inner state with properly initialized
    /// LRU and in-use sentinel nodes.
    pub fn new_with_sentinels() -> Self {
        trace!("LRUCacheInner::new_with_sentinels: initializing inner state");
        LRUCacheInner::new()
    }

    pub fn new() -> Self {
        trace!("LRUCacheInner::new: constructing inner state");

        let mut lru_sentinel    = Box::new(LRUHandle::make_sentinel());
        let mut in_use_sentinel = Box::new(LRUHandle::make_sentinel());

        unsafe {
            let lru_head: *mut LRUHandle     = lru_sentinel.as_mut();
            let in_use_head: *mut LRUHandle  = in_use_sentinel.as_mut();

            (*lru_head).set_next_ptr(lru_head);
            (*lru_head).set_prev_ptr(lru_head);

            (*in_use_head).set_next_ptr(in_use_head);
            (*in_use_head).set_prev_ptr(in_use_head);

            debug!(
                "LRUCacheInner::new: initialized sentinels lru_head={:p}, in_use_head={:p}",
                lru_head,
                in_use_head
            );
        }

        LRUCacheInner {
            usage: 0,
            lru:   lru_sentinel,
            in_use: in_use_sentinel,
            table: HandleTable::default(),
        }
    }

    pub fn usage(&self) -> usize {
        self.usage
    }

    pub fn set_usage(&mut self, usage: usize) {
        self.usage = usage;
    }

    pub fn add_usage(&mut self, delta: usize) {
        self.usage = self.usage.wrapping_add(delta);
    }

    pub fn sub_usage(&mut self, delta: usize) {
        self.usage = self.usage.wrapping_sub(delta);
    }

    pub fn lru_head_mut(&mut self) -> *mut LRUHandle {
        &mut *self.lru
    }

    pub fn in_use_head_mut(&mut self) -> *mut LRUHandle {
        &mut *self.in_use
    }

    pub fn table(&self) -> &HandleTable {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut HandleTable {
        &mut self.table
    }
}

// ---------------- [ File: bitcoinleveldb-lru/src/lru_cache_inner.rs ]  (replace the test module)

#[cfg(test)]
mod lru_cache_inner_test_suite {
    use super::*;

    #[traced_test]
    fn lru_cache_inner_usage_tracking_is_consistent() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new();

        assert_eq!(inner.usage(), 0);

        inner.set_usage(10);
        assert_eq!(inner.usage(), 10);

        inner.add_usage(5);
        assert_eq!(inner.usage(), 15);

        inner.sub_usage(3);
        assert_eq!(inner.usage(), 12);
    }

    #[traced_test]
    fn lru_cache_inner_table_accessors_reference_same_table() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new();

        let ptr_direct: *const HandleTable = inner.table();
        let ptr_via_fn: *const HandleTable = inner.table();
        assert_eq!(ptr_direct, ptr_via_fn);

        let ptr_mut_via_fn: *mut HandleTable = inner.table_mut();
        assert_eq!(ptr_direct, ptr_mut_via_fn);
    }

    #[traced_test]
    fn lru_cache_inner_new_initializes_empty_lists() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new();

        unsafe {
            let lru_head: *mut LRUHandle    = inner.lru_head_mut();
            let in_use_head: *mut LRUHandle = inner.in_use_head_mut();

            assert!(
                !lru_head.is_null(),
                "lru_cache_inner_new_initializes_empty_lists: lru_head is null"
            );
            assert!(
                !in_use_head.is_null(),
                "lru_cache_inner_new_initializes_empty_lists: in_use_head is null"
            );

            assert!(
                core::ptr::eq((*lru_head).next_ptr(), lru_head),
                "lru_cache_inner_new_initializes_empty_lists: lru_head.next does not point to head"
            );
            assert!(
                core::ptr::eq((*lru_head).prev_ptr(), lru_head),
                "lru_cache_inner_new_initializes_empty_lists: lru_head.prev does not point to head"
            );
            assert!(
                core::ptr::eq((*in_use_head).next_ptr(), in_use_head),
                "lru_cache_inner_new_initializes_empty_lists: in_use_head.next does not point to head"
            );
            assert!(
                core::ptr::eq((*in_use_head).prev_ptr(), in_use_head),
                "lru_cache_inner_new_initializes_empty_lists: in_use_head.prev does not point to head"
            );
        }
    }

    #[traced_test]
    fn lru_cache_inner_new_with_sentinels_initializes_empty_lists() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new_with_sentinels();

        unsafe {
            let lru_head: *mut LRUHandle    = inner.lru_head_mut();
            let in_use_head: *mut LRUHandle = inner.in_use_head_mut();

            assert!(
                !lru_head.is_null(),
                "lru_cache_inner_new_with_sentinels_initializes_empty_lists: lru_head is null"
            );
            assert!(
                !in_use_head.is_null(),
                "lru_cache_inner_new_with_sentinels_initializes_empty_lists: in_use_head is null"
            );

            assert!(
                core::ptr::eq((*lru_head).next_ptr(), lru_head),
                "lru_cache_inner_new_with_sentinels_initializes_empty_lists: lru_head.next does not point to head"
            );
            assert!(
                core::ptr::eq((*lru_head).prev_ptr(), lru_head),
                "lru_cache_inner_new_with_sentinels_initializes_empty_lists: lru_head.prev does not point to head"
            );
            assert!(
                core::ptr::eq((*in_use_head).next_ptr(), in_use_head),
                "lru_cache_inner_new_with_sentinels_initializes_empty_lists: in_use_head.next does not point to head"
            );
            assert!(
                core::ptr::eq((*in_use_head).prev_ptr(), in_use_head),
                "lru_cache_inner_new_with_sentinels_initializes_empty_lists: in_use_head.prev does not point to head"
            );
        }
    }
}
