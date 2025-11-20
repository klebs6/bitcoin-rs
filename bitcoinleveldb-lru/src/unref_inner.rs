// ---------------- [ File: bitcoinleveldb-lru/src/unref_inner.rs ]
crate::ix!();

pub unsafe fn unref_inner(inner: &mut LRUCacheInner, e: *mut LRUHandle) {
    let refs_before = (*e).refs();
    let in_cache    = (*e).is_in_cache();
    let hash        = (*e).hash_value();
    let charge      = (*e).charge_value();

    trace!(
        "unref_inner: e={:p}, refs_before={}, in_cache={}, hash=0x{:x}, charge={}",
        e,
        refs_before,
        in_cache,
        hash,
        charge
    );

    assert!(
        refs_before > 0,
        "unref_inner: attempt to decrement reference count below zero"
    );

    (*e).decrement_refs();
    let refs_after     = (*e).refs();
    let in_cache_after = (*e).is_in_cache();

    trace!(
        "unref_inner: e={:p}, refs_after_decrement={}, in_cache={}",
        e,
        refs_after,
        in_cache_after
    );

    if refs_after == 0 {
        // Exact LevelDB semantics: an entry is only destroyed when its refcount
        // drops to zero and it is no longer in the cache.
        assert!(
            !in_cache_after,
            "unref_inner: entry with refs==0 must not be marked in_cache"
        );

        let deleter = (*e).deleter_fn();
        let key     = (*e).key();
        let value   = (*e).value_ptr();

        trace!(
            "unref_inner: deallocating e={:p}, hash=0x{:x}, charge={}",
            e,
            hash,
            charge
        );

        deleter(&key, value);
        libc::free(e as *mut libc::c_void);
    } else if in_cache_after && refs_after == 1 {
        // Exact LevelDB semantics: when the cache still owns the only remaining
        // reference, move the entry from the in_use_ list back to the lru_ list.
        trace!(
            "unref_inner: moving e={:p} from in_use_ list to lru_ list (refs now 1)",
            e
        );

        lru_remove_node(e);
        let lru_head: *mut LRUHandle = inner.lru_head_mut();
        lru_append_node(lru_head, e);
    } else {
        // Multiple references remain, or the entry is not in the cache.
        trace!(
            "unref_inner: e={:p} remains with refs={} (in_cache={}), no list movement performed",
            e,
            refs_after,
            in_cache_after
        );
    }
}

#[cfg(test)]
mod unref_inner_test_suite {
    use super::*;
    use core::ffi::c_void;
    use core::sync::atomic::{AtomicUsize, Ordering};

    static UNREF_INNER_TEST_DELETER_CALLS: AtomicUsize = AtomicUsize::new(0);

    fn unref_inner_test_deleter(_: &Slice, ptr: *mut c_void) -> c_void {
        UNREF_INNER_TEST_DELETER_CALLS.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    unsafe fn unref_inner_make_entry(
        key_bytes: &[u8],
        refs:      u32,
        in_cache:  bool,
    ) -> *mut LRUHandle {
        let key_len   = key_bytes.len();
        let alloc_len = core::mem::size_of::<LRUHandle>() + key_len.saturating_sub(1);

        let node = libc::malloc(alloc_len) as *mut LRUHandle;
        assert!(
            !node.is_null(),
            "unref_inner_make_entry: allocation failed"
        );

        let value_box = Box::new(13_i32);
        let value_ptr = Box::into_raw(value_box) as *mut c_void;

        (*node).set_value_ptr(value_ptr);
        (*node).set_deleter_fn(unref_inner_test_deleter);
        (*node).set_charge_value(0);
        (*node).set_key_length(key_len);
        (*node).set_hash_value(0xF0F0_F0F0u32);
        (*node).set_in_cache(in_cache);
        (*node).set_refs(refs);
        (*node).set_next_hash_ptr(core::ptr::null_mut());
        (*node).set_next_ptr(core::ptr::null_mut());
        (*node).set_prev_ptr(core::ptr::null_mut());

        core::ptr::copy_nonoverlapping(
            key_bytes.as_ptr(),
            (*node).key_data_mut(),
            key_len,
        );

        node
    }

    // ---- Per-test counters to avoid cross-test interference in parallel runs ----
    static UNREF_INNER_TEST_DELETER_CALLS_FREE:  AtomicUsize = AtomicUsize::new(0);
    static UNREF_INNER_TEST_DELETER_CALLS_MOVE:  AtomicUsize = AtomicUsize::new(0);
    static UNREF_INNER_TEST_DELETER_CALLS_MULTI: AtomicUsize = AtomicUsize::new(0);

    // ---- Per-test deleters (each increments its own counter) ----
    fn unref_inner_test_deleter_free(_: &Slice, ptr: *mut c_void) -> c_void {
        UNREF_INNER_TEST_DELETER_CALLS_FREE.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe { drop(Box::from_raw(ptr as *mut i32)); }
        }
        unsafe { core::mem::zeroed() }
    }

    fn unref_inner_test_deleter_move(_: &Slice, ptr: *mut c_void) -> c_void {
        UNREF_INNER_TEST_DELETER_CALLS_MOVE.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe { drop(Box::from_raw(ptr as *mut i32)); }
        }
        unsafe { core::mem::zeroed() }
    }

    fn unref_inner_test_deleter_multi(_: &Slice, ptr: *mut c_void) -> c_void {
        UNREF_INNER_TEST_DELETER_CALLS_MULTI.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe { drop(Box::from_raw(ptr as *mut i32)); }
        }
        unsafe { core::mem::zeroed() }
    }

    // ---- Helper: make an entry with an explicit deleter (avoids shared globals) ----
    unsafe fn unref_inner_make_entry_with_deleter(
        key_bytes: &[u8],
        refs:      u32,
        in_cache:  bool,
        deleter:   fn(&Slice, *mut c_void) -> c_void,
    ) -> *mut LRUHandle {
        let key_len   = key_bytes.len();
        let alloc_len = core::mem::size_of::<LRUHandle>() + key_len.saturating_sub(1);

        let node = libc::malloc(alloc_len) as *mut LRUHandle;
        assert!(!node.is_null(), "unref_inner_make_entry_with_deleter: allocation failed");

        let value_box = Box::new(13_i32);
        let value_ptr = Box::into_raw(value_box) as *mut c_void;

        (*node).set_value_ptr(value_ptr);
        (*node).set_deleter_fn(deleter);
        (*node).set_charge_value(0);
        (*node).set_key_length(key_len);
        (*node).set_hash_value(0xF0F0_F0F0u32);
        (*node).set_in_cache(in_cache);
        (*node).set_refs(refs);
        (*node).set_next_hash_ptr(core::ptr::null_mut());
        (*node).set_next_ptr(core::ptr::null_mut());
        (*node).set_prev_ptr(core::ptr::null_mut());

        core::ptr::copy_nonoverlapping(
            key_bytes.as_ptr(),
            (*node).key_data_mut(),
            key_len,
        );

        node
    }



    #[traced_test]
    fn unref_inner_frees_entry_when_reference_count_reaches_zero_and_not_in_cache() {
        bitcoin_cfg::setup();
        UNREF_INNER_TEST_DELETER_CALLS_FREE.store(0, Ordering::SeqCst);

        let mut inner = LRUCacheInner::new_with_sentinels();

        unsafe {
            // refs=1, in_cache=false -> single unref should call deleter once and free
            let node = unref_inner_make_entry_with_deleter(
                b"unref-free",
                1,
                false,
                unref_inner_test_deleter_free,
            );

            trace!(
                "BEFORE unref: node={:p}, refs={}, in_cache={}",
                node,
                (*node).refs(),
                (*node).is_in_cache()
            );

            unref_inner(&mut inner, node);

            let calls = UNREF_INNER_TEST_DELETER_CALLS_FREE.load(Ordering::SeqCst);
            trace!("AFTER unref: deleter_calls_free={}", calls);

            assert_eq!(
                calls, 1,
                "deleter must be invoked exactly once when refs drop to zero"
            );
        }
    }

    #[traced_test]
    fn unref_inner_moves_entry_from_in_use_to_lru_when_refs_drop_to_one() {
        bitcoin_cfg::setup();
        UNREF_INNER_TEST_DELETER_CALLS_MOVE.store(0, Ordering::SeqCst);

        trace!("TEST: unref_inner_moves_entry_from_in_use_to_lru_when_refs_drop_to_one -- begin");

        let mut inner = LRUCacheInner::new_with_sentinels();

        unsafe {
            let in_use_head: *mut LRUHandle = inner.in_use_head_mut();
            let lru_head:    *mut LRUHandle = inner.lru_head_mut();

            trace!(
                "sentinels: in_use_head={:p}, in_use_head.next={:p}, in_use_head.prev={:p}, \
                 lru_head={:p}, lru_head.next={:p}, lru_head.prev={:p}",
                in_use_head,
                (*in_use_head).next_ptr(),
                (*in_use_head).prev_ptr(),
                lru_head,
                (*lru_head).next_ptr(),
                (*lru_head).prev_ptr(),
            );

            // refs=2, in_cache=true -> unref should move from in_use_ to lru_
            let node = unref_inner_make_entry_with_deleter(
                b"unref-move",
                2,
                true,
                unref_inner_test_deleter_move,
            );

            // Ensure the intended preconditions explicitly (guards against helper drift)
            (*node).set_in_cache(true);
            (*node).set_refs(2);

            trace!(
                "allocated node={:p}, initial refs={}, in_cache={}, next={:p}, prev={:p}",
                node,
                (*node).refs(),
                (*node).is_in_cache(),
                (*node).next_ptr(),
                (*node).prev_ptr()
            );

            lru_append_node(in_use_head, node);

            trace!(
                "after lru_append_node(in_use): in_use_head.next={:p}, in_use_head.prev={:p}, \
                 node.next={:p}, node.prev={:p}",
                (*in_use_head).next_ptr(),
                (*in_use_head).prev_ptr(),
                (*node).next_ptr(),
                (*node).prev_ptr()
            );

            trace!(
                "BEFORE unref_inner: node={:p}, refs={}, in_cache={}, \
                 in_use_head.next={:p}, in_use_head.prev={:p}, \
                 lru_head.next={:p}, lru_head.prev={:p}",
                node,
                (*node).refs(),
                (*node).is_in_cache(),
                (*in_use_head).next_ptr(),
                (*in_use_head).prev_ptr(),
                (*lru_head).next_ptr(),
                (*lru_head).prev_ptr()
            );

            unref_inner(&mut inner, node);

            trace!(
                "AFTER unref_inner: node={:p}, refs={}, in_cache={}, \
                 in_use_head.next={:p}, in_use_head.prev={:p}, \
                 lru_head.next={:p}, lru_head.prev={:p}, \
                 node.next={:p}, node.prev={:p}",
                node,
                (*node).refs(),
                (*node).is_in_cache(),
                (*in_use_head).next_ptr(),
                (*in_use_head).prev_ptr(),
                (*lru_head).next_ptr(),
                (*lru_head).prev_ptr(),
                (*node).next_ptr(),
                (*node).prev_ptr()
            );

            // Assertions: moved from in_use_ to lru_
            assert_eq!(
                (*node).refs(),
                1,
                "unref_inner must decrement reference count"
            );
            assert!(
                core::ptr::eq((*in_use_head).next_ptr(), in_use_head),
                "in_use_ list should be empty after moving node to lru_"
            );
            assert!(
                core::ptr::eq((*lru_head).next_ptr(), node),
                "node should become the first element on the lru_ list"
            );
            assert!(
                core::ptr::eq((*node).next_ptr(), lru_head),
                "lru_ list should remain circular after move"
            );
            assert_eq!(
                UNREF_INNER_TEST_DELETER_CALLS_MOVE.load(Ordering::SeqCst),
                0,
                "deleter must not be called when refs remain non-zero"
            );

            // Clean shutdown of the node (respecting invariants)
            trace!(
                "cleanup: detaching node={:p}, setting in_cache=false, performing final unref",
                node
            );
            lru_remove_node(node);
            (*node).set_in_cache(false);
            unref_inner(&mut inner, node);

            assert_eq!(
                UNREF_INNER_TEST_DELETER_CALLS_MOVE.load(Ordering::SeqCst),
                1,
                "deleter should be called exactly once during cleanup"
            );

            trace!("TEST: unref_inner_moves_entry_from_in_use_to_lru_when_refs_drop_to_one -- end");
        }
    }

    #[traced_test]
    fn unref_inner_decrements_refs_without_moving_when_multiple_external_refs_remain() {
        bitcoin_cfg::setup();
        UNREF_INNER_TEST_DELETER_CALLS_MULTI.store(0, Ordering::SeqCst);

        let mut inner = LRUCacheInner::new_with_sentinels();

        unsafe {
            let in_use_head: *mut LRUHandle = inner.in_use_head_mut();

            // refs=3, in_cache=true -> unref should decrement to 2 and keep on in_use_
            let node = unref_inner_make_entry_with_deleter(
                b"unref-multi",
                3,
                true,
                unref_inner_test_deleter_multi,
            );

            // Ensure invariants explicitly regardless of helper
            (*node).set_in_cache(true);
            (*node).set_refs(3);

            lru_append_node(in_use_head, node);

            trace!(
                "BEFORE unref: node={:p}, refs={}, in_cache={}, in_use_head.next={:p}, in_use_head.prev={:p}",
                node,
                (*node).refs(),
                (*node).is_in_cache(),
                (*in_use_head).next_ptr(),
                (*in_use_head).prev_ptr()
            );

            unref_inner(&mut inner, node);

            trace!(
                "AFTER unref: node={:p}, refs={}, in_cache={}, in_use_head.next={:p}, in_use_head.prev={:p}",
                node,
                (*node).refs(),
                (*node).is_in_cache(),
                (*in_use_head).next_ptr(),
                (*in_use_head).prev_ptr()
            );

            assert_eq!(
                (*node).refs(),
                2,
                "unref_inner must reduce the reference count by one"
            );
            assert!(
                core::ptr::eq((*in_use_head).next_ptr(), node),
                "entry should remain on in_use_ list when refs > 1"
            );
            assert_eq!(
                UNREF_INNER_TEST_DELETER_CALLS_MULTI.load(Ordering::SeqCst),
                0,
                "deleter must not run while references remain"
            );

            // Clean up: detach and release both remaining references.
            lru_remove_node(node);
            (*node).set_in_cache(false);
            unref_inner(&mut inner, node); // refs: 2 -> 1
            unref_inner(&mut inner, node); // refs: 1 -> 0 => deleter once

            assert_eq!(
                UNREF_INNER_TEST_DELETER_CALLS_MULTI.load(Ordering::SeqCst),
                1,
                "deleter should have been called exactly once during cleanup"
            );
        }
    }

}
