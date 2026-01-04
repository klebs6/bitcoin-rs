// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_set_b32_seckey.rs ]
crate::ix!();

/**
  | Set a scalar from a big endian byte array
  | and returns 1 if it is a valid seckey and
  | 0 otherwise.
  |
  */
#[cfg(feature="widemul-int128")]
pub fn scalar_set_b32_seckey(r: *mut Scalar, bin: *const u8) -> i32 {
    unsafe {
        let mut overflow: i32 = 0;
        scalar_set_b32(r, bin, &mut overflow);
        ((overflow == 0) as i32) & ((scalar_is_zero(r) == 0) as i32)
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_set_b32_seckey(r: *mut Scalar, bin: *const u8) -> i32 {
    unsafe {
        let mut overflow: i32 = 0;
        scalar_set_b32(r, bin, &mut overflow);
        ((overflow == 0) as i32) & ((scalar_is_zero(r) == 0) as i32)
    }
}

#[cfg(test)]
mod scalar_set_b32_seckey_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_set_b32_seckey_accepts_nonzero_values_below_order_and_rejects_invalid() {
        info!("validating scalar_set_b32_seckey acceptance rules");

        let mut out = scalar_zero_value();

        let zero = SCALAR_ZERO_BE;
        let ok1 = unsafe { scalar_set_b32_seckey(&mut out as *mut Scalar, zero.as_ptr()) };
        debug!(ok1, "zero must be rejected");
        assert_eq!(ok1, 0);

        let one = SCALAR_ONE_BE;
        let ok2 = unsafe { scalar_set_b32_seckey(&mut out as *mut Scalar, one.as_ptr()) };
        debug!(ok2, "one must be accepted");
        assert_eq!(ok2, 1);

        let n = SECP256K1_ORDER_BE;
        let ok3 = unsafe { scalar_set_b32_seckey(&mut out as *mut Scalar, n.as_ptr()) };
        debug!(ok3, "n must be rejected (overflow)");
        assert_eq!(ok3, 0);

        let nm1 = SECP256K1_ORDER_MINUS_1_BE;
        let ok4 = unsafe { scalar_set_b32_seckey(&mut out as *mut Scalar, nm1.as_ptr()) };
        debug!(ok4, "n-1 must be accepted");
        assert_eq!(ok4, 1);
    }
}
