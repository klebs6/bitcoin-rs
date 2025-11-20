// ---------------- [ File: bitcoinleveldb-lru/src/lru_debug_verify_inner.rs ]
crate::ix!();

pub unsafe fn lru_debug_verify_inner(inner: &mut LRUCacheInner) {
    trace!(
        "lru_debug_verify_inner: usage={}, lru_head={:p}, in_use_head={:p}",
        inner.usage(),
        inner.lru_head_mut(),
        inner.in_use_head_mut()
    );

    let lru_head: *mut LRUHandle    = inner.lru_head_mut();
    let in_use_head: *mut LRUHandle = inner.in_use_head_mut();

    lru_debug_verify_list(lru_head, "lru_");
    lru_debug_verify_list(in_use_head, "in_use_");
}

#[cfg(test)]
mod lru_debug_verify_inner_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn lru_debug_verify_inner_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn lru_debug_verify_inner_make_node() -> *mut LRUHandle {
        let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "lru_debug_verify_inner_make_node: allocation failed"
        );

        (*node).set_value_ptr(core::ptr::null_mut());
        (*node).set_deleter_fn(lru_debug_verify_inner_test_deleter);
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
    fn lru_debug_verify_inner_accepts_pristine_inner_state() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new_with_sentinels();

        unsafe {
            lru_debug_verify_inner(&mut inner);
        }
    }

    #[traced_test]
    fn lru_debug_verify_inner_accepts_inner_with_entries_in_both_lists() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new_with_sentinels();

        unsafe {
            let lru_head: *mut LRUHandle    = inner.lru_head_mut();
            let in_use_head: *mut LRUHandle = inner.in_use_head_mut();

            let lru_node: *mut LRUHandle    = lru_debug_verify_inner_make_node();
            let in_use_node: *mut LRUHandle = lru_debug_verify_inner_make_node();

            lru_append_node(lru_head, lru_node);
            lru_append_node(in_use_head, in_use_node);

            lru_debug_verify_inner(&mut inner);

            lru_remove_node(lru_node);
            lru_remove_node(in_use_node);

            libc::free(lru_node as *mut libc::c_void);
            libc::free(in_use_node as *mut libc::c_void);
        }
    }
}
