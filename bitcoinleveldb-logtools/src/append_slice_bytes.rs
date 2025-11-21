crate::ix!();

/// Append the bytes referenced by `source` into `destination`,
/// using the underlying raw pointer from `Slice`.
pub fn append_slice_bytes(source: &Slice, destination: &mut Vec<u8>) {
    let len = *source.size();

    if len == 0 {
        trace!(
            "append_slice_bytes: source slice length is zero; nothing to append"
        );
        return;
    }

    let data_ptr = *source.data();
    if data_ptr.is_null() {
        warn!(
            "append_slice_bytes: source slice has null data pointer (len={})",
            len
        );
        return;
    }

    unsafe {
        let bytes = std::slice::from_raw_parts(data_ptr, len);
        destination.extend_from_slice(bytes);
    }

    trace!(
        appended = len,
        total    = destination.len(),
        "append_slice_bytes: appended slice bytes to destination Vec"
    );
}


