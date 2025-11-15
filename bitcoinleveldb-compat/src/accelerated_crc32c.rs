// ---------------- [ File: bitcoinleveldb-compat/src/accelerated_crc32c.rs ]
crate::ix!();

/**
  | Extend the CRC to include the first n bytes of
  | buf.
  |
  | Returns zero if the CRC cannot be extended
  | using acceleration, else returns the newly
  | extended CRC value (which may also be zero).
  */
#[inline]
#[instrument(level = "trace", skip(buf), fields(size = size))]
pub fn acceleratedcrc32c(
    crc:  u32,
    buf:  *const u8,
    size: usize,
) -> u32 {
    if buf.is_null() || size == 0 {
        trace!(
            original_crc = crc,
            "acceleratedcrc32c: null buffer or empty input; returning original CRC"
        );
        return crc;
    }

    unsafe {
        let data = std::slice::from_raw_parts(buf, size);
        let new_crc = crc32c_extend(crc, data);
        trace!(
            original_crc = crc,
            new_crc,
            "acceleratedcrc32c: CRC32C computed"
        );
        new_crc
    }
}

/// Software implementation of CRC32C (Castagnoli) "Extend", matching
/// the LevelDB semantics: given an existing CRC and additional bytes,
/// return the CRC of the concatenation.
#[inline]
fn crc32c_extend(mut crc: u32, data: &[u8]) -> u32 {
    const CRC32C_POLY: u32 = 0x82f6_3b78;

    // Convert from "public" CRC value into the internal state.
    crc ^= 0xffff_ffff;

    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ CRC32C_POLY;
            } else {
                crc >>= 1;
            }
        }
    }

    // Convert back to the public representation.
    crc ^ 0xffff_ffff
}

#[cfg(test)]
mod crc32c_port_spec {
    use super::*;

    #[traced_test]
    fn crc32c_empty_input_is_identity() {
        let seed = 0x1234_5678u32;
        let buf: [u8; 0] = [];
        let crc = acceleratedcrc32c(seed, buf.as_ptr(), buf.len());
        assert_eq!(crc, seed);
    }

    #[traced_test]
    fn crc32c_chunked_vs_one_shot_match() {
        let data = b"leveldb-crc32c-port-consistency-check";
        let mid = data.len() / 2;

        let full = acceleratedcrc32c(0, data.as_ptr(), data.len());

        let first = acceleratedcrc32c(0, data.as_ptr(), mid);
        let second = unsafe {
            acceleratedcrc32c(
                first,
                data.as_ptr().add(mid),
                data.len() - mid,
            )
        };

        assert_eq!(full, second);
    }
}
