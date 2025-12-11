// ---------------- [ File: bitcoinleveldb-versionsetutil/src/get_file_iterator.rs ]
crate::ix!();

pub fn get_file_iterator(
    arg:        *mut c_void,
    options:    &ReadOptions,
    file_value: &Slice,
) -> *mut LevelDBIterator {
    unsafe {
        let size = *file_value.size();
        trace!(
            file_value_size = size,
            "get_file_iterator: invoked"
        );

        if size != 16 {
            error!(
                "get_file_iterator: unexpected file_value size {}; expected 16",
                size
            );
            let msg_slice =
                Slice::from("FileReader invoked with unexpected value");
            let status = Status::corruption(&msg_slice, None);
            return new_error_iterator(&status);
        }

        if arg.is_null() {
            error!("get_file_iterator: null TableCache pointer");
            let msg_slice =
                Slice::from("FileReader invoked with null TableCache pointer");
            let status = Status::corruption(&msg_slice, None);
            return new_error_iterator(&status);
        }

        let data_ptr = *file_value.data();
        debug!(
            data_ptr = ?data_ptr,
            "get_file_iterator: decoding file number and size"
        );

        let file_number = decode_fixed64_le(data_ptr);
        let file_size   = decode_fixed64_le(data_ptr.add(8));

        let cache: *mut TableCache = arg as *mut TableCache;
        let cache_ref: &mut TableCache = &mut *cache;

        trace!(
            file_number,
            file_size,
            "get_file_iterator: delegating to TableCache::new_iterator"
        );

        cache_ref.new_iterator(
            options,
            file_number,
            file_size,
            core::ptr::null_mut(),
        )
    }
}

#[cfg(test)]
mod get_file_iterator_spec {
    use super::*;

    #[traced_test]
    fn verify_get_file_iterator_with_invalid_value_size_returns_error_iterator() {
        let options = ReadOptions::default();
        let bytes = [0u8; 8];
        let file_value = Slice::from(&bytes[..]);

        trace!(
            file_value_len = *file_value.size(),
            "verify_get_file_iterator_with_invalid_value_size_returns_error_iterator: invoking"
        );

        let iter_ptr = get_file_iterator(core::ptr::null_mut(), &options, &file_value);

        debug!(
            "verify_get_file_iterator_with_invalid_value_size_returns_error_iterator: iter_ptr={:p}",
            iter_ptr
        );

        assert!(
            !iter_ptr.is_null(),
            "get_file_iterator should still return a non-null error iterator"
        );

        // We deliberately do not dereference the iterator here; it is enough
        // to ensure that the error-path allocation succeeds.
    }

    #[traced_test]
    fn verify_get_file_iterator_with_null_table_cache_pointer_on_valid_size() {
        let options = ReadOptions::default();

        // Build a 16-byte value, representing (file_number, file_size).
        let mut bytes = [0u8; 16];
        // file_number = 1, file_size = 2 (arbitrary but non-zero)
        let num = encode_fixed64_le(1);
        let size = encode_fixed64_le(2);
        bytes[..8].copy_from_slice(&num);
        bytes[8..].copy_from_slice(&size);

        let file_value = Slice::from(&bytes[..]);

        trace!(
            file_value_len = *file_value.size(),
            "verify_get_file_iterator_with_null_table_cache_pointer_on_valid_size: invoking"
        );

        // With a null TableCache pointer, we expect the second error-path
        // inside get_file_iterator to be taken.
        let iter_ptr = get_file_iterator(core::ptr::null_mut(), &options, &file_value);

        debug!(
            "verify_get_file_iterator_with_null_table_cache_pointer_on_valid_size: iter_ptr={:p}",
            iter_ptr
        );

        assert!(
            !iter_ptr.is_null(),
            "get_file_iterator must still return a non-null iterator on error paths"
        );
    }
}
