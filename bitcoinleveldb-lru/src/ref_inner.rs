// ---------------- [ File: bitcoinleveldb-lru/src/ref_inner.rs ]
crate::ix!();

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

#[cfg(test)]
mod ref_inner_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn ref_inner_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn ref_inner_make_list_node() -> *mut LRUHandle {
        let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "ref_inner_make_list_node: allocation failed"
        );

        (*node).set_value_ptr(core::ptr::null_mut());
        (*node).set_deleter_fn(ref_inner_test_deleter);
        (*node).set_charge_value(0);
        (*node).set_key_length(0);
        (*node).set_hash_value(0);
        (*node).set_in_cache(false);
        (*node).set_refs(0);
        (*node).set_next_hash_ptr(core::ptr::null_mut());
        (*node).set_next_ptr(core::ptr::null_mut());
        (*node).set_prev_ptr(core::ptr::null_mut());

        node
    }

    #[traced_test]
    fn ref_inner_moves_entry_from_lru_to_in_use_when_refs_are_one_and_in_cache() {
        bitcoin_cfg::setup();

        unsafe {
            let mut inner = LRUCacheInner::new_with_sentinels();

            let lru_head: *mut LRUHandle    = inner.lru_head_mut();
            let in_use_head: *mut LRUHandle = inner.in_use_head_mut();

            let node = ref_inner_make_list_node();
            (*node).set_in_cache(true);
            (*node).set_refs(1);

            lru_append_node(lru_head, node);

            ref_inner(&mut inner, node);

            assert_eq!(
                (*node).refs(),
                2,
                "ref_inner must increment reference count"
            );
            assert!(
                core::ptr::eq((*in_use_head).next_ptr(), node),
                "node should be the first element in the in_use_ list"
            );
            assert!(
                core::ptr::eq((*node).next_ptr(), in_use_head),
                "in_use_ list should remain circular after ref_inner"
            );
            assert!(
                core::ptr::eq((*lru_head).next_ptr(), lru_head),
                "lru_ list should be empty after moving node to in_use_"
            );

            libc::free(node as *mut libc::c_void);
        }
    }

    #[traced_test]
    fn ref_inner_only_increments_refs_when_entry_not_in_cache() {
        bitcoin_cfg::setup();

        unsafe {
            let mut inner = LRUCacheInner::new_with_sentinels();

            let node = ref_inner_make_list_node();
            (*node).set_in_cache(false);
            (*node).set_refs(0);

            ref_inner(&mut inner, node);

            assert_eq!(
                (*node).refs(),
                1,
                "ref_inner must still increment refs when entry is not in cache"
            );

            libc::free(node as *mut libc::c_void);
        }
    }
}
