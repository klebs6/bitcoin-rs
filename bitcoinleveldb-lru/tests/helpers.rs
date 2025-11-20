// ---------------- [ File: bitcoinleveldb-lru/tests/helpers.rs ]
use core::ffi::c_void;
use bitcoinleveldb_lru::*;
use bitcoin_imports::*;

#[traced_test]
fn helpers_lru_append_and_remove_maintain_circular_list_invariants() {
    bitcoin_cfg::setup();

    unsafe {
        let sentinel = mock_make_heap_sentinel_node();
        let node     = mock_make_list_node();

        lru_append_node(sentinel, node);

        assert_eq!((*sentinel).next_ptr(), node);
        assert_eq!((*sentinel).prev_ptr(), node);
        assert_eq!((*node).next_ptr(), sentinel);
        assert_eq!((*node).prev_ptr(), sentinel);

        lru_remove_node(node);

        assert_eq!((*sentinel).next_ptr(), sentinel);
        assert_eq!((*sentinel).prev_ptr(), sentinel);

        libc::free(node as *mut libc::c_void);
        libc::free(sentinel as *mut libc::c_void);
    }
}

#[traced_test]
fn helpers_ref_inner_moves_from_lru_to_in_use() {
    bitcoin_cfg::setup();

    unsafe {
        let mut inner = LRUCacheInner::new_with_sentinels();

        let lru_head: *mut LRUHandle    = inner.lru_head_mut();
        let in_use_head: *mut LRUHandle = inner.in_use_head_mut();

        let node = mock_make_list_node();
        (*node).set_in_cache(true);
        (*node).set_refs(1);

        lru_append_node(lru_head, node);

        ref_inner(&mut inner, node);

        assert_eq!((*in_use_head).next_ptr(), node);
        assert_eq!((*node).refs(), 2);

        libc::free(node as *mut libc::c_void);
    }
}
