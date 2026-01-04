// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_split.rs ]
crate::ix!();

/// Find r1 and r2 such that r1+r2*2^128 = k.
/// 
#[cfg(feature="widemul-int128")]
pub fn scalar_split_128(r1: *mut Scalar, r2: *mut Scalar, k: *const Scalar) {
    unsafe {
        (*r1).d[0] = (*k).d[0];
        (*r1).d[1] = (*k).d[1];
        (*r1).d[2] = 0;
        (*r1).d[3] = 0;

        (*r2).d[0] = (*k).d[2];
        (*r2).d[1] = (*k).d[3];
        (*r2).d[2] = 0;
        (*r2).d[3] = 0;
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_split_128(r1: *mut Scalar, r2: *mut Scalar, k: *const Scalar) {
    unsafe {
        (*r1).d[0] = (*k).d[0];
        (*r1).d[1] = (*k).d[1];
        (*r1).d[2] = (*k).d[2];
        (*r1).d[3] = (*k).d[3];
        (*r1).d[4] = 0;
        (*r1).d[5] = 0;
        (*r1).d[6] = 0;
        (*r1).d[7] = 0;

        (*r2).d[0] = (*k).d[4];
        (*r2).d[1] = (*k).d[5];
        (*r2).d[2] = (*k).d[6];
        (*r2).d[3] = (*k).d[7];
        (*r2).d[4] = 0;
        (*r2).d[5] = 0;
        (*r2).d[6] = 0;
        (*r2).d[7] = 0;
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_split_128(r1: *mut Scalar, r2: *mut Scalar, a: *const Scalar) {
    unsafe {
        *r1 = *a;
        *r2 = 0;
    }
}

#[cfg(test)]
mod scalar_split_128_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    fn be_shl_128(a: &[u8; 32]) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[0..16].copy_from_slice(&a[16..32]);
        out
    }

    fn be_add_256_no_mod(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let (sum, carry) = be_add_32(a, b);
        debug_assert!(carry == 0, "unexpected carry in be_add_256_no_mod");
        sum
    }

    #[traced_test]
    fn scalar_split_128_recombines_to_original_scalar_as_integer() {
        info!("validating scalar_split_128 recombines to original scalar (integer sense)");

        for (i, k_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let k = scalar_from_be_bytes(k_be);

            let mut r1 = scalar_zero_value();
            let mut r2 = scalar_zero_value();

            unsafe {
                scalar_split_128(
                    &mut r1 as *mut Scalar,
                    &mut r2 as *mut Scalar,
                    &k as *const Scalar,
                );
            }

            let r1_be = scalar_to_be_bytes(&r1);
            let r2_be = scalar_to_be_bytes(&r2);

            // k == r1 + (r2 << 128)
            let shifted_r2 = be_shl_128(&r2_be);
            let recomposed = be_add_256_no_mod(&r1_be, &shifted_r2);

            debug!(i, ?k_be, ?r1_be, ?r2_be, ?recomposed, "split/recompose");
            assert_eq!(recomposed, *k_be);
        }
    }

}
