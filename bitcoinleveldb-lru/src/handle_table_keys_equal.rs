// ---------------- [ File: bitcoinleveldb-lru/src/handle_table_keys_equal.rs ]
crate::ix!();

pub fn handle_table_keys_equal(entry: *const LRUHandle, key_: &Slice) -> bool {
    trace!("handle_table_keys_equal: entry={:p}", entry);

    unsafe {
        let entry_len = (*entry).key_len();
        let key_len   = *key_.size();

        if entry_len != key_len {
            trace!(
                "handle_table_keys_equal: length mismatch (entry_len={}, key_len={})",
                entry_len,
                key_len
            );
            return false;
        }

        if entry_len == 0 {
            trace!("handle_table_keys_equal: both keys empty, treating as equal");
            return true;
        }

        let entry_ptr = (*entry).key_data_ptr();
        let key_ptr   = *key_.data();

        if entry_ptr.is_null() || key_ptr.is_null() {
            debug!(
                "handle_table_keys_equal: null pointer(s) encountered (entry_ptr={:p}, key_ptr={:p})",
                entry_ptr,
                key_ptr
            );
            return false;
        }

        let entry_slice = core::slice::from_raw_parts(entry_ptr, entry_len);
        let key_slice   = core::slice::from_raw_parts(key_ptr,   key_len);

        let equal = entry_slice == key_slice;

        trace!(
            "handle_table_keys_equal: compared {} bytes, equal={}",
            entry_len,
            equal
        );

        equal
    }
}

#[cfg(test)]
mod handle_table_keys_equal_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn handle_table_keys_equal_test_deleter(_: &Slice, _: *mut c_void) -> c_void {
        unsafe { core::mem::zeroed() }
    }

    unsafe fn handle_table_keys_equal_make_handle(
        key_bytes: &[u8],
    ) -> *mut LRUHandle {
        let key_len   = key_bytes.len();
        let alloc_len = core::mem::size_of::<LRUHandle>() + key_len.saturating_sub(1);

        let handle = libc::malloc(alloc_len) as *mut LRUHandle;
        assert!(
            !handle.is_null(),
            "handle_table_keys_equal_make_handle: allocation failed"
        );

        (*handle).set_value_ptr(core::ptr::null_mut());
        (*handle).set_deleter_fn(handle_table_keys_equal_test_deleter);
        (*handle).set_charge_value(0);
        (*handle).set_key_length(key_len);
        (*handle).set_hash_value(0xC0FF_EEu32);
        (*handle).set_in_cache(false);
        (*handle).set_refs(1);
        (*handle).set_next_hash_ptr(core::ptr::null_mut());
        (*handle).set_next_ptr(core::ptr::null_mut());
        (*handle).set_prev_ptr(core::ptr::null_mut());

        core::ptr::copy_nonoverlapping(
            key_bytes.as_ptr(),
            (*handle).key_data_mut(),
            key_len,
        );

        handle
    }

    fn handle_table_keys_equal_slice_from_bytes(bytes: &[u8]) -> Slice {
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len())
    }

    #[traced_test]
    fn handle_table_keys_equal_reports_true_for_identical_keys() {
        bitcoin_cfg::setup();

        let key_bytes = b"ht-eq-identical";

        unsafe {
            let handle = handle_table_keys_equal_make_handle(key_bytes);
            let slice  = handle_table_keys_equal_slice_from_bytes(key_bytes);

            let equal = handle_table_keys_equal(handle as *const LRUHandle, &slice);
            assert!(
                equal,
                "handle_table_keys_equal should return true for identical key bytes"
            );

            libc::free(handle as *mut libc::c_void);
        }
    }

    #[traced_test]
    fn handle_table_keys_equal_reports_false_for_length_mismatch() {
        bitcoin_cfg::setup();

        let entry_key = b"abcd";
        let slice_key = b"abc";

        unsafe {
            let handle = handle_table_keys_equal_make_handle(entry_key);
            let slice  = handle_table_keys_equal_slice_from_bytes(slice_key);

            let equal = handle_table_keys_equal(handle as *const LRUHandle, &slice);
            assert!(
                !equal,
                "length mismatch between entry and slice must yield false"
            );

            libc::free(handle as *mut libc::c_void);
        }
    }

    #[traced_test]
    fn handle_table_keys_equal_reports_false_for_same_length_different_bytes() {
        bitcoin_cfg::setup();

        let entry_key = b"abcd";
        let slice_key = b"abce";

        unsafe {
            let handle = handle_table_keys_equal_make_handle(entry_key);
            let slice  = handle_table_keys_equal_slice_from_bytes(slice_key);

            let equal = handle_table_keys_equal(handle as *const LRUHandle, &slice);
            assert!(
                !equal,
                "keys of equal length but different bytes must not be considered equal"
            );

            libc::free(handle as *mut libc::c_void);
        }
    }

    #[traced_test]
    fn handle_table_keys_equal_treats_both_empty_keys_as_equal() {
        bitcoin_cfg::setup();

        let mut entry = LRUHandle::make_sentinel();
        entry.set_key_length(0);

        let bytes = b"";
        let empty_slice = Slice::from(bytes.as_slice());

        let equal = unsafe {
            handle_table_keys_equal(&entry as *const LRUHandle, &empty_slice)
        };

        assert!(
            equal,
            "when both entry and slice have zero length, keys should be treated as equal"
        );
    }
}
