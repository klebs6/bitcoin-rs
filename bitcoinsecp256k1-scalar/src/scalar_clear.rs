// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_clear.rs ]
crate::ix!();

/**
  | Clear a scalar to prevent the leak of
  | sensitive data.
  |
  */
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_clear(r: *mut Scalar) {
    unsafe {
        (*r).d[0] = 0;
        (*r).d[1] = 0;
        (*r).d[2] = 0;
        (*r).d[3] = 0;
    }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_clear(r: *mut Scalar) {
    unsafe {
        (*r).d[0] = 0;
        (*r).d[1] = 0;
        (*r).d[2] = 0;
        (*r).d[3] = 0;
        (*r).d[4] = 0;
        (*r).d[5] = 0;
        (*r).d[6] = 0;
        (*r).d[7] = 0;
    }
}

#[cfg(feature="exhaustive-test-order")]
#[inline]
pub fn scalar_clear(r: *mut Scalar) {
    unsafe {
        *r = 0;
    }
}

#[cfg(test)]
mod scalar_clear_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_clear_zeroes_all_limbs_and_is_detected_as_zero() {
        info!("validating scalar_clear behavior");

        let mut s = scalar_from_be_bytes(&SECP256K1_ORDER_MINUS_1_BE);
        let before = scalar_to_be_bytes(&s);

        unsafe {
            scalar_clear(&mut s as *mut Scalar);
        }
        let after = scalar_to_be_bytes(&s);

        debug!(?before, ?after, "scalar_clear");
        assert_eq!(after, SCALAR_ZERO_BE);
        unsafe {
            assert_eq!(scalar_is_zero(&s as *const Scalar), 1);
        }
    }
}
