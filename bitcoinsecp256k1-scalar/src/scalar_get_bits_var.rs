// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_get_bits_var.rs ]
crate::ix!();


/// Access bits from a scalar. Not constant time.
/// 
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_get_bits_var(a: *const Scalar, offset: u32, count: u32) -> u32 {
    unsafe {
        verify_check!(count < 32);
        verify_check!(offset + count <= 256);
        if ((offset + count - 1) >> 6) == (offset >> 6) {
            scalar_get_bits(a, offset, count)
        } else {
            verify_check!(((offset >> 6) + 1) < 4);
            let limb0 = (*a).d[(offset >> 6) as usize];
            let limb1 = (*a).d[((offset >> 6) + 1) as usize];
            ((((limb0 >> (offset & 0x3F)) | (limb1 << (64 - (offset & 0x3F))))
                & ((1u64 << count) - 1)) as u32)
        }
    }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_get_bits_var(a: *const Scalar, offset: u32, count: u32) -> u32 {
    unsafe {
        verify_check!(count < 32);
        verify_check!(offset + count <= 256);
        if ((offset + count - 1) >> 5) == (offset >> 5) {
            scalar_get_bits(a, offset, count)
        } else {
            verify_check!(((offset >> 5) + 1) < 8);
            let limb0 = (*a).d[(offset >> 5) as usize];
            let limb1 = (*a).d[((offset >> 5) + 1) as usize];
            ((((limb0 >> (offset & 0x1F)) | (limb1 << (32 - (offset & 0x1F))))
                & ((1u32 << count) - 1)) as u32)
        }
    }
}

#[cfg(feature="exhaustive-test-order")]
#[inline]
pub fn scalar_get_bits_var(a: *const Scalar, offset: u32, count: u32) -> u32 {
    scalar_get_bits(a, offset, count)
}

#[cfg(test)]
mod scalar_get_bits_var_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_get_bits_var_matches_reference_extraction_across_limb_boundary() {
        info!("validating scalar_get_bits_var against byte-level bit extraction for cross-limb cases");

        let pattern: [u8; 32] = [
            0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0,
            0x0F, 0x1E, 0x2D, 0x3C, 0x4B, 0x5A, 0x69, 0x78, 0x87, 0x96, 0xA5, 0xB4, 0xC3, 0xD2, 0xE1, 0xF0,
        ];
        let s = scalar_from_be_bytes(&pattern);

        #[cfg(feature = "widemul-int128")]
        let cases: &[(u32, u32)] = &[(60, 8), (63, 2), (61, 10), (124, 12), (127, 8), (190, 16), (255 - 15, 16)];

        #[cfg(feature = "widemul-int64")]
        let cases: &[(u32, u32)] = &[(28, 8), (31, 2), (29, 10), (60, 12), (63, 8), (92, 16), (255 - 15, 16)];

        for (idx, &(offset, count)) in cases.iter().enumerate() {
            let got = unsafe { scalar_get_bits_var(&s as *const Scalar, offset, count) };
            let expected = be_bit_extract_u32(&pattern, offset, count);
            debug!(idx, offset, count, got, expected, "get_bits_var case");
            assert_eq!(got, expected);
        }
    }
}
