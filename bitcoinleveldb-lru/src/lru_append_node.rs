// ---------------- [ File: bitcoinleveldb-lru/src/lru_append_node.rs ]
crate::ix!();

pub unsafe fn lru_append_node(list: *mut LRUHandle, e: *mut LRUHandle) {
    trace!("lru_append_node: list={:p}, e={:p}", list, e);

    // Make "e" newest entry by inserting just before *list
    let last = (*list).prev_ptr();

    (*e).set_next_ptr(list);
    (*e).set_prev_ptr(last);
    (*last).set_next_ptr(e);
    (*list).set_prev_ptr(e);
}

#[cfg(test)]
mod lru_append_node_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn lru_append_node_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn lru_append_node_make_heap_sentinel() -> *mut LRUHandle {
        let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "lru_append_node_make_heap_sentinel: allocation failed"
        );

        (*node).set_value_ptr(core::ptr::null_mut());
        (*node).set_deleter_fn(lru_append_node_test_deleter);
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

    unsafe fn lru_append_node_make_list_node() -> *mut LRUHandle {
        let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "lru_append_node_make_list_node: allocation failed"
        );

        (*node).set_value_ptr(core::ptr::null_mut());
        (*node).set_deleter_fn(lru_append_node_test_deleter);
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
    fn lru_append_node_single_entry_forms_two_node_cycle() {
        bitcoin_cfg::setup();

        unsafe {
            let sentinel = lru_append_node_make_heap_sentinel();
            let node     = lru_append_node_make_list_node();

            lru_append_node(sentinel, node);

            assert_eq!(
                (*sentinel).next_ptr(),
                node,
                "sentinel.next should point to appended node"
            );
            assert_eq!(
                (*sentinel).prev_ptr(),
                node,
                "sentinel.prev should point to appended node"
            );
            assert_eq!(
                (*node).next_ptr(),
                sentinel,
                "node.next should close the circular list back to sentinel"
            );
            assert_eq!(
                (*node).prev_ptr(),
                sentinel,
                "node.prev should point back to sentinel"
            );

            libc::free(node as *mut libc::c_void);
            libc::free(sentinel as *mut libc::c_void);
        }
    }

    #[traced_test]
    fn lru_append_node_appends_newest_before_sentinel_in_multi_node_list() {
        bitcoin_cfg::setup();

        unsafe {
            let sentinel = lru_append_node_make_heap_sentinel();
            let first    = lru_append_node_make_list_node();
            let second   = lru_append_node_make_list_node();

            lru_append_node(sentinel, first);
            lru_append_node(sentinel, second);

            assert_eq!(
                (*sentinel).next_ptr(),
                first,
                "first node should remain the oldest (sentinel.next)"
            );
            assert_eq!(
                (*sentinel).prev_ptr(),
                second,
                "second node should be the newest (sentinel.prev)"
            );
            assert_eq!(
                (*first).next_ptr(),
                second,
                "first.next should point to second"
            );
            assert_eq!(
                (*second).prev_ptr(),
                first,
                "second.prev should point back to first"
            );
            assert_eq!(
                (*second).next_ptr(),
                sentinel,
                "second.next should close the circular list at sentinel"
            );

            libc::free(second as *mut libc::c_void);
            libc::free(first as *mut libc::c_void);
            libc::free(sentinel as *mut libc::c_void);
        }
    }
}
