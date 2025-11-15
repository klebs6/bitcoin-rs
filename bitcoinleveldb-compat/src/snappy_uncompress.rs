// ---------------- [ File: bitcoinleveldb-compat/src/snappy_uncompress.rs ]
crate::ix!();

/**
  | Attempt to snappy uncompress
  | input[0,input_length-1] into *output.
  |
  | Returns true if successful, false if the input
  | is invalid lightweight compressed data.
  |
  | REQUIRES: at least the first "n" bytes of
  | output[] must be writable where "n" is the
  | result of a successful call to
  | Snappy_GetUncompressedLength.
  */
#[inline]
#[cfg(feature = "leveldb_snappy")]
#[instrument(level = "trace", skip(input, output), fields(length = length))]
pub fn snappy_uncompress(
    input:  *const u8,
    length: usize,
    output: *mut u8,
) -> bool {
    if input.is_null() {
        warn!("snappy_uncompress called with null input pointer");
        return false;
    }
    if output.is_null() {
        error!("snappy_uncompress called with null output pointer");
        return false;
    }

    unsafe {
        let input_slice = std::slice::from_raw_parts(input, length);
        debug!(
            input_len = input_slice.len(),
            "snappy_uncompress: starting decompression"
        );

        let mut decoder = snap::raw::Decoder::new();

        match decoder.decompress_vec(input_slice) {
            Ok(decompressed) => {
                let out_len = decompressed.len();
                debug!(
                    uncompressed_len = out_len,
                    "snappy_uncompress: decompression succeeded"
                );

                // Caller must ensure that `output` points to a writable buffer
                // of at least `out_len` bytes, as per the original contract.
                std::ptr::copy_nonoverlapping(decompressed.as_ptr(), output, out_len);
                true
            }
            Err(err) => {
                debug!(
                    ?err,
                    "snappy_uncompress: invalid or corrupt Snappy input"
                );
                false
            }
        }
    }
}

#[inline]
#[cfg(not(feature = "leveldb_snappy"))]
#[instrument(level = "trace", skip(input, output), fields(length = length))]
pub fn snappy_uncompress(
    input:  *const u8,
    length: usize,
    output: *mut u8,
) -> bool {
    debug!(
        input_ptr = ?input,
        output_ptr = ?output,
        "snappy_uncompress: leveldb_snappy feature disabled; returning false"
    );
    let _ = input;
    let _ = length;
    let _ = output;
    false
}
