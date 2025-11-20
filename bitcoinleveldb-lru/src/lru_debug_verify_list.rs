// ---------------- [ File: bitcoinleveldb-lru/src/lru_debug_verify_list.rs ]
crate::ix!();

pub unsafe fn lru_debug_verify_list(head: *mut LRUHandle, list_name: &str) {
    trace!(
        "lru_debug_verify_list: list_name={}, head={:p}",
        list_name,
        head
    );

    assert!(
        !head.is_null(),
        "lru_debug_verify_list({}): list head pointer is null",
        list_name
    );

    let align = core::mem::align_of::<LRUHandle>();
    let head_addr = head as usize;

    assert!(
        head_addr % align == 0,
        "lru_debug_verify_list({}): head pointer {:p} is misaligned (align={})",
        list_name,
        head,
        align
    );

    let mut node: *mut LRUHandle = (*head).next_ptr();
    let mut steps: usize         = 0;

    while !core::ptr::eq(node, head) {
        let addr = node as usize;

        assert!(
            addr % align == 0,
            "lru_debug_verify_list({}): node pointer {:p} is misaligned (align={}) at step {}",
            list_name,
            node,
            align,
            steps
        );

        let next = (*node).next_ptr();
        let prev = (*node).prev_ptr();

        assert!(
            !next.is_null() && !prev.is_null(),
            "lru_debug_verify_list({}): node {:p} has null next/prev (next={:p}, prev={:p})",
            list_name,
            node,
            next,
            prev
        );

        assert!(
            core::ptr::eq((*next).prev_ptr(), node),
            "lru_debug_verify_list({}): next.prev inconsistency at node {:p} (next={:p}, next.prev={:p})",
            list_name,
            node,
            next,
            (*next).prev_ptr()
        );

        assert!(
            core::ptr::eq((*prev).next_ptr(), node),
            "lru_debug_verify_list({}): prev.next inconsistency at node {:p} (prev={:p}, prev.next={:p})",
            list_name,
            node,
            prev,
            (*prev).next_ptr()
        );

        steps = steps.wrapping_add(1);

        assert!(
            steps < 1_000_000,
            "lru_debug_verify_list({}): list appears excessively long; possible cycle without head",
            list_name
        );

        node = next;
    }

    debug!(
        "lru_debug_verify_list: list_name={}, head={:p}, nodes_traversed={}",
        list_name,
        head,
        steps
    );
}

#[cfg(test)]
mod lru_debug_verify_list_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn lru_debug_verify_list_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn lru_debug_verify_list_make_node() -> *mut LRUHandle {
        let node = libc::malloc(core::mem::size_of::<LRUHandle>()) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "lru_debug_verify_list_make_node: allocation failed"
        );

        (*node).set_value_ptr(core::ptr::null_mut());
        (*node).set_deleter_fn(lru_debug_verify_list_test_deleter);
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
    fn lru_debug_verify_list_accepts_empty_list_with_only_sentinel() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new_with_sentinels();

        unsafe {
            let lru_head: *mut LRUHandle = inner.lru_head_mut();
            lru_debug_verify_list(lru_head, "lru_empty");
        }
    }

    #[traced_test]
    fn lru_debug_verify_list_accepts_single_element_list() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new_with_sentinels();

        unsafe {
            let lru_head: *mut LRUHandle = inner.lru_head_mut();
            let node: *mut LRUHandle     = lru_debug_verify_list_make_node();

            lru_append_node(lru_head, node);

            lru_debug_verify_list(lru_head, "lru_single");

            lru_remove_node(node);
            libc::free(node as *mut libc::c_void);
        }
    }
}
