// ---------------- [ File: bitcoinleveldb-lru/src/finish_erase_inner.rs ]
crate::ix!();

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
mod finish_erase_inner_test_suite {
    use super::*;
    use core::ffi::c_void;
    use core::sync::atomic::{AtomicUsize, Ordering};

    // Per-test counters to avoid cross-test interference when tests run in parallel.
    static FINISH_ERASE_INNER_LRU_ONLY_DELETER_CALLS: AtomicUsize =
        AtomicUsize::new(0);
    static FINISH_ERASE_INNER_IN_USE_DELETER_CALLS: AtomicUsize =
        AtomicUsize::new(0);

    fn finish_erase_inner_lru_only_test_deleter(
        _: &Slice,
        ptr: *mut c_void,
    ) -> c_void {
        FINISH_ERASE_INNER_LRU_ONLY_DELETER_CALLS.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    fn finish_erase_inner_in_use_test_deleter(
        _: &Slice,
        ptr: *mut c_void,
    ) -> c_void {
        FINISH_ERASE_INNER_IN_USE_DELETER_CALLS.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    unsafe fn finish_erase_inner_make_cache_entry_on_lru(
        inner:     &mut LRUCacheInner,
        key_bytes: &[u8],
        charge:    usize,
        refs:      u32,
        deleter:   fn(&Slice, *mut c_void) -> c_void,
    ) -> *mut LRUHandle {
        let key_len   = key_bytes.len();
        let alloc_len = core::mem::size_of::<LRUHandle>() + key_len.saturating_sub(1);

        let e = libc::malloc(alloc_len) as *mut LRUHandle;
        assert!(
            !e.is_null(),
            "finish_erase_inner_make_cache_entry_on_lru: allocation failed"
        );

        let value_box = Box::new(7_i32);
        let value_ptr = Box::into_raw(value_box) as *mut c_void;

        (*e).set_value_ptr(value_ptr);
        (*e).set_deleter_fn(deleter);
        (*e).set_charge_value(charge);
        (*e).set_key_length(key_len);
        (*e).set_hash_value(0xF00D_F00Du32);
        (*e).set_in_cache(true);
        (*e).set_refs(refs);
        (*e).set_next_hash_ptr(core::ptr::null_mut());
        (*e).set_next_ptr(core::ptr::null_mut());
        (*e).set_prev_ptr(core::ptr::null_mut());

        core::ptr::copy_nonoverlapping(
            key_bytes.as_ptr(),
            (*e).key_data_mut(),
            key_len,
        );

        let lru_head: *mut LRUHandle = inner.lru_head_mut();

        // Insert as the only element on the LRU list.
        (*e).set_next_ptr(lru_head);
        (*e).set_prev_ptr(lru_head);
        (*lru_head).set_next_ptr(e);
        (*lru_head).set_prev_ptr(e);

        inner.add_usage(charge);

        e
    }

    unsafe fn finish_erase_inner_make_cache_entry_on_in_use(
        inner:     &mut LRUCacheInner,
        key_bytes: &[u8],
        charge:    usize,
        refs:      u32,
        deleter:   fn(&Slice, *mut c_void) -> c_void,
    ) -> *mut LRUHandle {
        let key_len   = key_bytes.len();
        let alloc_len = core::mem::size_of::<LRUHandle>() + key_len.saturating_sub(1);

        let e = libc::malloc(alloc_len) as *mut LRUHandle;
        assert!(
            !e.is_null(),
            "finish_erase_inner_make_cache_entry_on_in_use: allocation failed"
        );

        let value_box = Box::new(11_i32);
        let value_ptr = Box::into_raw(value_box) as *mut c_void;

        (*e).set_value_ptr(value_ptr);
        (*e).set_deleter_fn(deleter);
        (*e).set_charge_value(charge);
        (*e).set_key_length(key_len);
        (*e).set_hash_value(0xABCD_DCBAu32);
        (*e).set_in_cache(true);
        (*e).set_refs(refs);
        (*e).set_next_hash_ptr(core::ptr::null_mut());
        (*e).set_next_ptr(core::ptr::null_mut());
        (*e).set_prev_ptr(core::ptr::null_mut());

        core::ptr::copy_nonoverlapping(
            key_bytes.as_ptr(),
            (*e).key_data_mut(),
            key_len,
        );

        let in_use_head: *mut LRUHandle = inner.in_use_head_mut();

        (*e).set_next_ptr(in_use_head);
        (*e).set_prev_ptr(in_use_head);
        (*in_use_head).set_next_ptr(e);
        (*in_use_head).set_prev_ptr(e);

        inner.add_usage(charge);

        e
    }

    #[traced_test]
    fn finish_erase_inner_null_pointer_is_noop() {
        bitcoin_cfg::setup();

        let mut inner = LRUCacheInner::new();
        inner.set_usage(42);

        let erased = unsafe { finish_erase_inner(&mut inner, core::ptr::null_mut()) };
        assert!(
            !erased,
            "finish_erase_inner should return false when given a null pointer"
        );

        assert_eq!(
            inner.usage(),
            42,
            "finish_erase_inner must not modify usage when e is null"
        );
    }

    #[traced_test]
    fn finish_erase_inner_removes_lru_only_entry_and_frees_handle() {
        bitcoin_cfg::setup();
        FINISH_ERASE_INNER_LRU_ONLY_DELETER_CALLS.store(0, Ordering::SeqCst);

        let mut inner = LRUCacheInner::new();
        let charge    = 5usize;

        let e = unsafe {
            finish_erase_inner_make_cache_entry_on_lru(
                &mut inner,
                b"fe-lru-only",
                charge,
                1, // cache ref only
                finish_erase_inner_lru_only_test_deleter,
            )
        };

        let lru_head: *mut LRUHandle = inner.lru_head_mut();

        assert_eq!(
            inner.usage(),
            charge,
            "usage should account for the single cached entry before erase"
        );

        let erased = unsafe { finish_erase_inner(&mut inner, e) };
        assert!(
            erased,
            "finish_erase_inner should return true for non-null entry"
        );

        assert_eq!(
            inner.usage(),
            0,
            "usage should be decremented by the entry's charge"
        );

        unsafe {
            assert!(
                core::ptr::eq((*lru_head).next_ptr(), lru_head),
                "after erase, LRU list should be empty (next points to head)"
            );
            assert!(
                core::ptr::eq((*lru_head).prev_ptr(), lru_head),
                "after erase, LRU list should be empty (prev points to head)"
            );
        }

        assert_eq!(
            FINISH_ERASE_INNER_LRU_ONLY_DELETER_CALLS.load(Ordering::SeqCst),
            1,
            "deleter should be invoked exactly once for erased LRU entry"
        );
    }

    #[traced_test]
    fn finish_erase_inner_preserves_external_reference_for_in_use_entry() {
        bitcoin_cfg::setup();
        FINISH_ERASE_INNER_IN_USE_DELETER_CALLS.store(0, Ordering::SeqCst);

        let mut inner = LRUCacheInner::new();
        let charge    = 3usize;

        let e = unsafe {
            finish_erase_inner_make_cache_entry_on_in_use(
                &mut inner,
                b"fe-in-use",
                charge,
                2, // cache ref + one external client
                finish_erase_inner_in_use_test_deleter,
            )
        };

        let in_use_head: *mut LRUHandle = inner.in_use_head_mut();

        assert_eq!(
            inner.usage(),
            charge,
            "usage should reflect in-use entry before erase"
        );

        let erased = unsafe { finish_erase_inner(&mut inner, e) };
        assert!(
            erased,
            "finish_erase_inner should return true for in-use entry"
        );

        assert_eq!(
            inner.usage(),
            0,
            "finish_erase_inner must subtract the entry's charge from usage"
        );

        unsafe {
            assert!(
                core::ptr::eq((*in_use_head).next_ptr(), in_use_head),
                "in_use_ list should no longer contain the entry after finish_erase_inner"
            );
            assert!(
                core::ptr::eq((*in_use_head).prev_ptr(), in_use_head),
                "in_use_ list should be empty after finish_erase_inner"
            );

            assert!(
                !(*e).is_in_cache(),
                "finish_erase_inner must clear the in_cache flag"
            );
            assert_eq!(
                (*e).refs(),
                1,
                "finish_erase_inner should drop exactly one reference for the cache"
            );
        }

        assert_eq!(
            FINISH_ERASE_INNER_IN_USE_DELETER_CALLS.load(Ordering::SeqCst),
            0,
            "deleter must not run while an external reference remains"
        );

        unsafe {
            // Simulate the last client releasing its handle.
            unref_inner(&mut inner, e);
        }

        assert_eq!(
            FINISH_ERASE_INNER_IN_USE_DELETER_CALLS.load(Ordering::SeqCst),
            1,
            "deleter should run once when the final external reference is released"
        );
    }
}
