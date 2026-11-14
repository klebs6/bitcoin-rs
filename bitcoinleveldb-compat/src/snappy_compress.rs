// ---------------- [ File: bitcoinleveldb-compat/src/snappy_compress.rs ]
crate::ix!();

/**
  | Store the snappy compression of
  | "input[0,input_length-1]" in *output.
  |
  | Replaces the destination only on success; false leaves it unchanged.
  | Returns false if snappy is not supported by this port or a null
  | pointer is supplied. Non-null input must describe `length` readable
  | bytes; output must point to a live, exclusively accessible Vec<u8>.
  | The input may view the destination's existing allocation.
  */
#[inline]
#[cfg(feature = "leveldb_snappy")]
#[instrument(level = "trace", skip(input, output), fields(length = length))]
pub fn snappy_compress(
    input:  *const u8,
    length: usize,
    output: *mut Vec<u8>,
) -> bool {
    if input.is_null() {
        warn!("snappy_compress called with null input pointer");
        return false;
    }
    if output.is_null() {
        error!("snappy_compress called with null output pointer");
        return false;
    }

    // Finish all reads before replacing the destination owner: input may be
    // a live view into output's existing allocation. The encoder returns its
    // own byte vector, so no mutable output reference overlaps those reads.
    let result = {
        let input_slice = unsafe { std::slice::from_raw_parts(input, length) };
        debug!(
            input_len = input_slice.len(),
            "snappy_compress: starting compression"
        );
        let mut encoder = snap::raw::Encoder::new();
        encoder.compress_vec(input_slice)
    };

    match result {
        Ok(compressed) => {
            debug!(
                compressed_len = compressed.len(),
                "snappy_compress: compression succeeded"
            );
            // SAFETY: output must point to a live, exclusively accessible
            // Vec<u8>; the input view above is no longer used.
            unsafe { *output = compressed; }
            true
        }
        Err(err) => {
            error!(?err, "snappy_compress: compression failed");
            false
        }
    }
}

#[inline]
#[cfg(not(feature = "leveldb_snappy"))]
#[instrument(level = "trace", skip(input, output), fields(length = length))]
pub fn snappy_compress(
    input:  *const u8,
    length: usize,
    output: *mut Vec<u8>,
) -> bool {
    debug!(
        input_ptr = ?input,
        output_ptr = ?output,
        "snappy_compress: leveldb_snappy feature disabled; returning false"
    );
    let _ = input;
    let _ = length;
    let _ = output;
    false
}

#[cfg(all(test, feature = "leveldb_snappy"))]
mod snappy_port_roundtrip_spec {
    use super::*;

    #[instrument(level = "trace", skip(payload))]
    fn snappy_roundtrip_via_port(payload: &[u8]) -> Option<Vec<u8>> {
        unsafe {
            let mut compressed = Vec::<u8>::new();
            let ok_compress = snappy_compress(
                payload.as_ptr(),
                payload.len(),
                &mut compressed as *mut Vec<u8>,
            );
            if !ok_compress {
                warn!("snappy_roundtrip_via_port: compression failed");
                return None;
            }

            let compressed_bytes = compressed;

            let mut uncompressed_len: usize = 0;
            let ok_len = snappy_get_uncompressed_length(
                compressed_bytes.as_ptr(),
                compressed_bytes.len(),
                &mut uncompressed_len as *mut usize,
            );
            if !ok_len {
                warn!("snappy_roundtrip_via_port: length probe failed");
                return None;
            }

            let mut uncompressed = vec![0u8; uncompressed_len];
            let ok_uncompress = snappy_uncompress(
                compressed_bytes.as_ptr(),
                compressed_bytes.len(),
                uncompressed.as_mut_ptr(),
            );
            if !ok_uncompress {
                warn!("snappy_roundtrip_via_port: decompression failed");
                return None;
            }

            Some(uncompressed)
        }
    }

    #[traced_test]
    fn snappy_roundtrip_text_payload() {
        let payload = b"hello snappy from leveldb";
        let roundtripped = snappy_roundtrip_via_port(payload)
            .expect("snappy_roundtrip_text_payload: expected Snappy to be available");
        assert_eq!(roundtripped.as_slice(), payload);
    }

    #[traced_test]
    fn snappy_roundtrip_binary_payload() {
        let payload: &[u8] = &[
            0x00, 0xFF, 0x10, 0x42, 0x7F, 0x80, 0xAA, 0x55, 0x01, 0x02, 0x03,
        ];
        let roundtripped = snappy_roundtrip_via_port(payload)
            .expect("snappy_roundtrip_binary_payload: expected Snappy to be available");
        assert_eq!(roundtripped.as_slice(), payload);
    }

    #[traced_test]
    fn snappy_get_uncompressed_length_matches_actual() {
        let payload = b"some payload that compresses reasonably well";
        unsafe {
            let mut compressed = Vec::<u8>::new();
            assert!(snappy_compress(
                payload.as_ptr(),
                payload.len(),
                &mut compressed as *mut Vec<u8>,
            ));

            let compressed_bytes = compressed;

            let mut len_probe: usize = 0;
            assert!(snappy_get_uncompressed_length(
                compressed_bytes.as_ptr(),
                compressed_bytes.len(),
                &mut len_probe as *mut usize,
            ));

            let mut out = vec![0u8; len_probe];
            assert!(snappy_uncompress(
                compressed_bytes.as_ptr(),
                compressed_bytes.len(),
                out.as_mut_ptr(),
            ));
            assert_eq!(out.as_slice(), payload);
        }
    }
}
