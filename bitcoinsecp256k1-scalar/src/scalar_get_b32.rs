// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_get_b32.rs ]
crate::ix!();

/**
  | Convert a scalar to a byte array.
  |
  */
#[cfg(feature="widemul-int128")]
pub fn scalar_get_b32(bin: *mut u8, a: *const Scalar) {
    unsafe {
        *bin.add(0) = ((*a).d[3] >> 56) as u8;
        *bin.add(1) = ((*a).d[3] >> 48) as u8;
        *bin.add(2) = ((*a).d[3] >> 40) as u8;
        *bin.add(3) = ((*a).d[3] >> 32) as u8;
        *bin.add(4) = ((*a).d[3] >> 24) as u8;
        *bin.add(5) = ((*a).d[3] >> 16) as u8;
        *bin.add(6) = ((*a).d[3] >> 8) as u8;
        *bin.add(7) = (*a).d[3] as u8;

        *bin.add(8) = ((*a).d[2] >> 56) as u8;
        *bin.add(9) = ((*a).d[2] >> 48) as u8;
        *bin.add(10) = ((*a).d[2] >> 40) as u8;
        *bin.add(11) = ((*a).d[2] >> 32) as u8;
        *bin.add(12) = ((*a).d[2] >> 24) as u8;
        *bin.add(13) = ((*a).d[2] >> 16) as u8;
        *bin.add(14) = ((*a).d[2] >> 8) as u8;
        *bin.add(15) = (*a).d[2] as u8;

        *bin.add(16) = ((*a).d[1] >> 56) as u8;
        *bin.add(17) = ((*a).d[1] >> 48) as u8;
        *bin.add(18) = ((*a).d[1] >> 40) as u8;
        *bin.add(19) = ((*a).d[1] >> 32) as u8;
        *bin.add(20) = ((*a).d[1] >> 24) as u8;
        *bin.add(21) = ((*a).d[1] >> 16) as u8;
        *bin.add(22) = ((*a).d[1] >> 8) as u8;
        *bin.add(23) = (*a).d[1] as u8;

        *bin.add(24) = ((*a).d[0] >> 56) as u8;
        *bin.add(25) = ((*a).d[0] >> 48) as u8;
        *bin.add(26) = ((*a).d[0] >> 40) as u8;
        *bin.add(27) = ((*a).d[0] >> 32) as u8;
        *bin.add(28) = ((*a).d[0] >> 24) as u8;
        *bin.add(29) = ((*a).d[0] >> 16) as u8;
        *bin.add(30) = ((*a).d[0] >> 8) as u8;
        *bin.add(31) = (*a).d[0] as u8;
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_get_b32(bin: *mut u8, a: *const Scalar) {
    unsafe {
        *bin.add(0) = ((*a).d[7] >> 24) as u8;
        *bin.add(1) = ((*a).d[7] >> 16) as u8;
        *bin.add(2) = ((*a).d[7] >> 8) as u8;
        *bin.add(3) = (*a).d[7] as u8;

        *bin.add(4) = ((*a).d[6] >> 24) as u8;
        *bin.add(5) = ((*a).d[6] >> 16) as u8;
        *bin.add(6) = ((*a).d[6] >> 8) as u8;
        *bin.add(7) = (*a).d[6] as u8;

        *bin.add(8) = ((*a).d[5] >> 24) as u8;
        *bin.add(9) = ((*a).d[5] >> 16) as u8;
        *bin.add(10) = ((*a).d[5] >> 8) as u8;
        *bin.add(11) = (*a).d[5] as u8;

        *bin.add(12) = ((*a).d[4] >> 24) as u8;
        *bin.add(13) = ((*a).d[4] >> 16) as u8;
        *bin.add(14) = ((*a).d[4] >> 8) as u8;
        *bin.add(15) = (*a).d[4] as u8;

        *bin.add(16) = ((*a).d[3] >> 24) as u8;
        *bin.add(17) = ((*a).d[3] >> 16) as u8;
        *bin.add(18) = ((*a).d[3] >> 8) as u8;
        *bin.add(19) = (*a).d[3] as u8;

        *bin.add(20) = ((*a).d[2] >> 24) as u8;
        *bin.add(21) = ((*a).d[2] >> 16) as u8;
        *bin.add(22) = ((*a).d[2] >> 8) as u8;
        *bin.add(23) = (*a).d[2] as u8;

        *bin.add(24) = ((*a).d[1] >> 24) as u8;
        *bin.add(25) = ((*a).d[1] >> 16) as u8;
        *bin.add(26) = ((*a).d[1] >> 8) as u8;
        *bin.add(27) = (*a).d[1] as u8;

        *bin.add(28) = ((*a).d[0] >> 24) as u8;
        *bin.add(29) = ((*a).d[0] >> 16) as u8;
        *bin.add(30) = ((*a).d[0] >> 8) as u8;
        *bin.add(31) = (*a).d[0] as u8;
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_get_b32(bin: *mut u8, a: *const Scalar) {
    unsafe {
        core::ptr::write_bytes(bin, 0u8, 32);
        *bin.add(28) = (*a >> 24) as u8;
        *bin.add(29) = (*a >> 16) as u8;
        *bin.add(30) = (*a >> 8) as u8;
        *bin.add(31) = (*a) as u8;
    }
}

#[cfg(test)]
mod scalar_get_b32_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_get_b32_roundtrips_known_values_below_order() {
        info!("validating scalar_get_b32 roundtrip for values < n");

        for (i, be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let s = scalar_from_be_bytes(be);
            let got = scalar_to_be_bytes(&s);
            debug!(i, ?be, ?got, "roundtrip");
            assert_eq!(&got, be);
        }
    }
}
