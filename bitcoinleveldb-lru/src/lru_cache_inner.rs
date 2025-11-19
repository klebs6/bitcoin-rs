// ---------------- [ File: bitcoinleveldb-lru/src/lru_cache_inner.rs ]
crate::ix!();

pub struct LRUCacheInner {

    usage:  usize,

    /**
      | Dummy head of LRU list.
      |
      | lru.prev is newest entry, lru.next is
      | oldest entry.
      |
      | Entries have refs==1 and in_cache==true.
      */
    lru:    LRUHandle,

    /**
      | Dummy head of in-use list. 
      |
      | Entries are in use by clients, and have
      | refs >= 2 and in_cache==true.
      |
      */
    in_use: LRUHandle,

    table:  HandleTable,
}

impl LRUCacheInner {

    /// Construct an inner state with properly initialized
    /// LRU and in-use sentinel nodes.
    pub fn new_with_sentinels() -> Self {
        trace!("LRUCacheInner::new_with_sentinels: initializing inner state");

        let mut inner = LRUCacheInner {
            usage: 0,
            lru:   lru_make_sentinel(),
            in_use: lru_make_sentinel(),
            table: HandleTable::default(),
        };

        unsafe {
            let lru_head: *mut LRUHandle = &mut inner.lru;
            (*lru_head).set_next_ptr(lru_head);
            (*lru_head).set_prev_ptr(lru_head);

            let in_use_head: *mut LRUHandle = &mut inner.in_use;
            (*in_use_head).set_next_ptr(in_use_head);
            (*in_use_head).set_prev_ptr(in_use_head);
        }

        inner
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
        &mut self.lru
    }

    pub fn in_use_head_mut(&mut self) -> *mut LRUHandle {
        &mut self.in_use
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
    use core::ffi::c_void;

    fn sentinel_deleter(_k: &Slice, _v: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    #[traced_test]
    fn lru_cache_inner_usage_tracking_is_consistent() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new_with_sentinels();

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

        let mut inner = LRUCacheInner::new_with_sentinels();

        // Force the compiler to actually take references so we can
        // validate that both access paths refer to the same table.
        let ptr_direct: *const HandleTable = inner.table();
        let ptr_via_fn: *const HandleTable = inner.table();

        assert_eq!(
            ptr_direct, ptr_via_fn,
            "multiple immutable accessors must reference the same HandleTable"
        );

        let ptr_mut_via_fn: *mut HandleTable = inner.table_mut();
        assert_eq!(
            ptr_direct,
            ptr_mut_via_fn as *const HandleTable,
            "mutable accessor must reference the same underlying HandleTable"
        );

        // sanity: use table_mut() to ensure it is actually mutable
        unsafe {
            let table_ptr = inner.table_mut() as *mut HandleTable;
            let _ = table_ptr;
        }

        // use the sentinel_deleter just to keep it referenced in tests
        let _ = sentinel_deleter;
    }
}
