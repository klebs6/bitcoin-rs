// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_eq.rs ]
crate::ix!();

/// Compare two scalars.
/// 
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_eq(a: *const Scalar, b: *const Scalar) -> i32 {
    unsafe {
        ((((*a).d[0] ^ (*b).d[0])
            | ((*a).d[1] ^ (*b).d[1])
            | ((*a).d[2] ^ (*b).d[2])
            | ((*a).d[3] ^ (*b).d[3]))
            == 0) as i32
    }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_eq(a: *const Scalar, b: *const Scalar) -> i32 {
    unsafe {
        ((((*a).d[0] ^ (*b).d[0])
            | ((*a).d[1] ^ (*b).d[1])
            | ((*a).d[2] ^ (*b).d[2])
            | ((*a).d[3] ^ (*b).d[3])
            | ((*a).d[4] ^ (*b).d[4])
            | ((*a).d[5] ^ (*b).d[5])
            | ((*a).d[6] ^ (*b).d[6])
            | ((*a).d[7] ^ (*b).d[7]))
            == 0) as i32
    }
}

#[cfg(feature="exhaustive-test-order")]
#[inline]
pub fn scalar_eq(a: *const Scalar, b: *const Scalar) -> i32 {
    unsafe { (*a == *b) as i32 }
}

#[cfg(test)]
mod scalar_equality_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_eq_is_reflexive_symmetric_and_detects_difference() {
        info!("validating scalar_eq basic equivalence relation behavior");

        let a = scalar_from_be_bytes(&SCALAR_MAX_U32_BE);
        let b = scalar_clone_via_b32(&a);
        let c = scalar_from_be_bytes(&SCALAR_THREE_BE);

        unsafe {
            assert_eq!(scalar_eq(&a as *const Scalar, &a as *const Scalar), 1);
            assert_eq!(scalar_eq(&a as *const Scalar, &b as *const Scalar), 1);
            assert_eq!(scalar_eq(&b as *const Scalar, &a as *const Scalar), 1);
            assert_eq!(scalar_eq(&a as *const Scalar, &c as *const Scalar), 0);
            assert_eq!(scalar_eq(&c as *const Scalar, &a as *const Scalar), 0);
        }

        debug!(
            a_be = ?scalar_to_be_bytes(&a),
            c_be = ?scalar_to_be_bytes(&c),
            "scalar_eq checked"
        );
    }
}
