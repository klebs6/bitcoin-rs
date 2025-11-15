// ---------------- [ File: bitcoinleveldb-compat/src/snappy_get_uncompressed_length.rs ]
crate::ix!();

/**
  | If input[0,input_length-1] looks like a valid
  | snappy compressed buffer, store the size of the
  | uncompressed data in *result and return true.
  | Else return false.
  */
#[inline]
#[cfg(feature = "leveldb_snappy")]
#[instrument(level = "trace", skip(input, result), fields(length = length))]
pub fn snappy_get_uncompressed_length(
    input:  *const u8,
    length: usize,
    result: *mut usize,
) -> bool {
    if input.is_null() {
        warn!("snappy_get_uncompressed_length called with null input pointer");
        return false;
    }
    if result.is_null() {
        error!("snappy_get_uncompressed_length called with null result pointer");
        return false;
    }

    unsafe {
        let input_slice = std::slice::from_raw_parts(input, length);
        debug!(
            input_len = input_slice.len(),
            "snappy_get_uncompressed_length: inspecting compressed buffer"
        );

        let mut decoder = snap::raw::Decoder::new();

        match decoder.decompress_vec(input_slice) {
            Ok(decompressed) => {
                let len = decompressed.len();
                debug!(
                    uncompressed_len = len,
                    "snappy_get_uncompressed_length: decompression for length succeeded"
                );
                *result = len;
                true
            }
            Err(err) => {
                debug!(
                    ?err,
                    "snappy_get_uncompressed_length: invalid or corrupt Snappy input"
                );
                false
            }
        }
    }
}

#[inline]
#[cfg(not(feature = "leveldb_snappy"))]
#[instrument(level = "trace", skip(input, result), fields(length = length))]
pub fn snappy_get_uncompressed_length(
    input:  *const u8,
    length: usize,
    result: *mut usize,
) -> bool {
    debug!(
        input_ptr = ?input,
        result_ptr = ?result,
        "snappy_get_uncompressed_length: leveldb_snappy feature disabled; returning false"
    );
    let _ = input;
    let _ = length;
    let _ = result;
    false
}
