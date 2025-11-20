// ---------------- [ File: bitcoinleveldb-lru/src/lru_remove_node.rs ]
crate::ix!();

pub unsafe fn lru_remove_node(e: *mut LRUHandle) {
    trace!("lru_remove_node: e={:p}", e);

    let next = (*e).next_ptr();
    let prev = (*e).prev_ptr();

    (*next).set_prev_ptr(prev);
    (*prev).set_next_ptr(next);
}

#[cfg(test)]
mod lru_remove_node_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn lru_remove_node_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn lru_remove_node_make_heap_sentinel() -> *mut LRUHandle {
        let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "lru_remove_node_make_heap_sentinel: allocation failed"
        );

        (*node).set_value_ptr(core::ptr::null_mut());
        (*node).set_deleter_fn(lru_remove_node_test_deleter);
        (*node).set_charge_value(0);
        (*node).set_key_length(0);
        (*node).set_hash_value(0);
        (*node).set_in_cache(false);
        (*node).set_refs(0);
        (*node).set_next_hash_ptr(core::ptr::null_mut());
        (*node).set_next_ptr(node);
        (*node).set_prev_ptr(node);

        node
    }

    unsafe fn lru_remove_node_make_list_node() -> *mut LRUHandle {
        let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "lru_remove_node_make_list_node: allocation failed"
        );

        (*node).set_value_ptr(core::ptr::null_mut());
        (*node).set_deleter_fn(lru_remove_node_test_deleter);
        (*node).set_charge_value(0);
        (*node).set_key_length(0);
        (*node).set_hash_value(0);
        (*node).set_in_cache(false);
        (*node).set_refs(1);
        (*node).set_next_hash_ptr(core::ptr::null_mut());
        (*node).set_next_ptr(core::ptr::null_mut());
        (*node).set_prev_ptr(core::ptr::null_mut());

        node
    }

    #[traced_test]
    fn lru_remove_node_single_entry_restores_empty_circular_list() {
        bitcoin_cfg::setup();

        unsafe {
            let sentinel = lru_remove_node_make_heap_sentinel();
            let node     = lru_remove_node_make_list_node();

            lru_append_node(sentinel, node);

            lru_remove_node(node);

            assert_eq!(
                (*sentinel).next_ptr(),
                sentinel,
                "after removing single node, sentinel.next must point to sentinel"
            );
            assert_eq!(
                (*sentinel).prev_ptr(),
                sentinel,
                "after removing single node, sentinel.prev must point to sentinel"
            );

            libc::free(node as *mut libc::c_void);
            libc::free(sentinel as *mut libc::c_void);
        }
    }

    #[traced_test]
    fn lru_remove_node_middle_entry_in_three_node_list_relinks_neighbors() {
        bitcoin_cfg::setup();

        unsafe {
            let sentinel = lru_remove_node_make_heap_sentinel();
            let first    = lru_remove_node_make_list_node();
            let middle   = lru_remove_node_make_list_node();

            lru_append_node(sentinel, first);
            lru_append_node(sentinel, middle);

            // List is: sentinel <-> first <-> middle <-> sentinel
            lru_remove_node(first);

            assert_eq!(
                (*sentinel).next_ptr(),
                middle,
                "sentinel.next should point directly to the remaining node"
            );
            assert_eq!(
                (*middle).prev_ptr(),
                sentinel,
                "middle.prev should point back to sentinel after removal"
            );
            assert_eq!(
                (*middle).next_ptr(),
                sentinel,
                "middle.next should still close the list at sentinel"
            );

            libc::free(middle as *mut libc::c_void);
            libc::free(first as *mut libc::c_void);
            libc::free(sentinel as *mut libc::c_void);
        }
    }
}
