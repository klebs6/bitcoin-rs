crate::ix!();

pub unsafe fn lru_remove_node(e: *mut LRUHandle) {
    trace!("lru_remove_node: e={:p}", e);

    let next = (*e).next_ptr();
    let prev = (*e).prev_ptr();

    (*next).set_prev_ptr(prev);
    (*prev).set_next_ptr(next);
}

pub unsafe fn lru_append_node(list: *mut LRUHandle, e: *mut LRUHandle) {
    trace!("lru_append_node: list={:p}, e={:p}", list, e);

    // Make "e" newest entry by inserting just before *list
    let last = (*list).prev_ptr();

    (*e).set_next_ptr(list);
    (*e).set_prev_ptr(last);
    (*last).set_next_ptr(e);
    (*list).set_prev_ptr(e);
}

pub unsafe fn ref_inner(inner: &mut LRUCacheInner, e: *mut LRUHandle) {
    trace!("ref_inner: e={:p}, refs={}", e, (*e).refs());

    if (*e).refs() == 1 && (*e).is_in_cache() {
        // If on lru_ list, move to in_use_ list.
        lru_remove_node(e);
        let in_use_head: *mut LRUHandle = inner.in_use_head_mut();
        lru_append_node(in_use_head, e);
    }

    (*e).increment_refs();
}

pub unsafe fn unref_inner(inner: &mut LRUCacheInner, e: *mut LRUHandle) {
    trace!("unref_inner: e={:p}, refs={}", e, (*e).refs());

    assert!((*e).refs() > 0);
    (*e).decrement_refs();

    if (*e).refs() == 0 {
        // Deallocate.
        assert!(!(*e).is_in_cache());

        let deleter = (*e).deleter_fn();
        let key     = (*e).key();
        let value   = (*e).value_ptr();

        trace!("unref_inner: deallocating e={:p}", e);

        deleter(&key, value);
        libc::free(e as *mut libc::c_void);
    } else if (*e).is_in_cache() && (*e).refs() == 1 {
        // No longer in use; move to lru_ list.
        lru_remove_node(e);
        let lru_head: *mut LRUHandle = inner.lru_head_mut();
        lru_append_node(lru_head, e);
    }
}

pub unsafe fn finish_erase_inner(inner: &mut LRUCacheInner, e: *mut LRUHandle) -> bool {
    trace!("finish_erase_inner: e={:p}", e);

    if !e.is_null() {
        assert!((*e).is_in_cache());
        lru_remove_node(e);
        (*e).set_in_cache(false);

        let charge = (*e).charge_value();
        inner.sub_usage(charge);

        unref_inner(inner, e);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod helpers_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn helpers_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn helpers_make_heap_sentinel_node() -> *mut LRUHandle {
        let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "helpers_make_heap_sentinel_node: alloc failed"
        );

        (*node).set_value_ptr(core::ptr::null_mut());
        (*node).set_deleter_fn(helpers_test_deleter);
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

    unsafe fn helpers_make_list_node() -> *mut LRUHandle {
        let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "helpers_make_list_node: allocation failed"
        );

        (*node).set_value_ptr(core::ptr::null_mut());
        (*node).set_deleter_fn(helpers_test_deleter);
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
    fn helpers_lru_append_and_remove_maintain_circular_list_invariants() {
        bitcoin_cfg::setup();

        unsafe {
            let sentinel = helpers_make_heap_sentinel_node();
            let node     = helpers_make_list_node();

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

            let node = helpers_make_list_node();
            (*node).set_in_cache(true);
            (*node).set_refs(1);

            // Place node on the LRU list with refs == 1
            lru_append_node(lru_head, node);

            // Now RefInner should move it to in_use_ and bump refs
            ref_inner(&mut inner, node);

            assert_eq!(
                (*in_use_head).next_ptr(),
                node,
                "node should be first in in_use_ list after ref_inner"
            );
            assert_eq!(
                (*node).refs(),
                2,
                "ref_inner should increment refs from 1 to 2"
            );

            libc::free(node as *mut libc::c_void);
        }
    }
}
