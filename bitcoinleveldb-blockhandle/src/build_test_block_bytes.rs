// ---------------- [ File: bitcoinleveldb-blockhandle/src/build_test_block_bytes.rs ]
crate::ix!();

#[cfg(test)]
pub fn build_test_block_bytes(payload: &[u8], block_type: u8) -> Vec<u8> {
    // Layout: payload[n] + type[1] + crc[4]
    let mut block = Vec::with_capacity(
        payload.len() + BLOCK_TRAILER_SIZE,
    );
    block.extend_from_slice(payload);
    block.push(block_type);

    let crc_input_len = payload.len() + 1;
    let crc = unsafe {
        crc32c_value(
            block.as_ptr(),
            crc_input_len,
        )
    };
    let masked = crc32c_mask(crc);

    let mut crc_bytes = [0u8; 4];
    bitcoinleveldb_coding::encode_fixed32(
        crc_bytes.as_mut_ptr(),
        masked,
    );
    block.extend_from_slice(&crc_bytes);

    assert_eq!(
        block.len(),
        payload.len() + BLOCK_TRAILER_SIZE,
        "build_test_block_bytes: unexpected block length"
    );

    trace!(
        "build_test_block_bytes: payload_len={}, block_type={}, total_len={}",
        payload.len(),
        block_type,
        block.len()
    );

    block
}
