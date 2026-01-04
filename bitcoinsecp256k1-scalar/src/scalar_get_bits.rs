// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_get_bits.rs ]
crate::ix!();

/// Access bits from a scalar. 
///
/// All requested bits must belong to the same 32-bit limb.
///
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_get_bits(a: *const Scalar, offset: u32, count: u32) -> u32 {
    unsafe {
        verify_check!(((offset + count - 1) >> 6) == (offset >> 6));
        ((((*a).d[(offset >> 6) as usize] >> (offset & 0x3F))
            & (((1u64 << count) - 1) as u64)) as u32)
    }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_get_bits(a: *const Scalar, offset: u32, count: u32) -> u32 {
    unsafe {
        verify_check!(((offset + count - 1) >> 5) == (offset >> 5));
        ((((*a).d[(offset >> 5) as usize] >> (offset & 0x1F)) & ((1u32 << count) - 1)) as u32)
    }
}

#[cfg(feature="exhaustive-test-order")]
#[inline]
pub fn scalar_get_bits(a: *const Scalar, offset: u32, count: u32) -> u32 {
    unsafe {
        if offset < 32 {
            ((*a >> offset) & ((1u32 << count) - 1))
        } else {
            0
        }
    }
}

#[cfg(test)]
mod scalar_get_bits_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_get_bits_matches_reference_extraction_within_single_limb() {
        info!("validating scalar_get_bits against byte-level bit extraction");

        let pattern: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
        ];
        let s = scalar_from_be_bytes(&pattern);

        #[cfg(feature = "widemul-int128")]
        let cases: &[(u32, u32)] = &[
            (0, 1),
            (1, 7),
            (8, 8),
            (16, 16),
            (32, 16),
            (48, 8),
            (60, 4),
            (64, 1),
            (80, 12),
            (120, 8),
            (128, 16),
            (192, 16),
        ];

        #[cfg(feature = "widemul-int64")]
        let cases: &[(u32, u32)] = &[
            (0, 1),
            (1, 7),
            (8, 8),
            (16, 8),
            (24, 7),
            (32, 1),
            (40, 12),
            (56, 8),
            (64, 8),
            (96, 8),
            (128, 8),
            (224, 8),
        ];

        for (idx, &(offset, count)) in cases.iter().enumerate() {
            let got = unsafe { scalar_get_bits(&s as *const Scalar, offset, count) };
            let expected = be_bit_extract_u32(&pattern, offset, count);
            debug!(idx, offset, count, got, expected, "get_bits case");
            assert_eq!(got, expected);
        }
    }
}
