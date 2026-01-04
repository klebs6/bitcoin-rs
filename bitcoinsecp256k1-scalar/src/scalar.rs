// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar.rs ]
crate::ix!();

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
#[cfg(feature="widemul-int64")]
pub struct Scalar {
    pub(crate) d: [u32; 8],
}

#[cfg(feature="widemul-int64")]
impl Scalar {
    pub const fn new() -> Self {
        Self { d: [0; 8] }
    }
}
//-----------------------

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
#[cfg(feature="widemul-int128")]
pub struct Scalar {
    pub(crate) d: [u64; 4],
}

#[cfg(feature="widemul-int128")]
impl Scalar {
    pub const fn new() -> Self {
        Self { d: [0; 4] }
    }
}

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
#[cfg(feature="exhaustive-test-order")]
pub type Scalar = u32;

#[cfg(test)]
#[cfg(any(feature = "widemul-int64", feature = "widemul-int128"))]
mod scalar_type_layout_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_new_is_zero_and_scalar_is_32_bytes() {
        info!("validating Scalar::new() semantics and size");

        let s = Scalar::new();
        let be = scalar_to_be_bytes(&s);

        debug!(size = core::mem::size_of::<Scalar>(), ?be, "Scalar layout/bytes");
        assert_eq!(core::mem::size_of::<Scalar>(), 32);
        assert_eq!(be, SCALAR_ZERO_BE);

        unsafe {
            assert_eq!(scalar_is_zero(&s as *const Scalar), 1);
            assert_eq!(scalar_is_one(&s as *const Scalar), 0);
        }
    }
}
