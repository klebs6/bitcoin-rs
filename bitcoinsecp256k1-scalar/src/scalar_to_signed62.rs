// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_to_signed62.rs ]
crate::ix!();

#[cfg(feature="widemul-int128")]
pub fn scalar_to_signed62(r: *mut ModInv64Signed62, a: *const Scalar) {
    unsafe {
        const M62: u64 = u64::MAX >> 2;

        let a0: u64 = (*a).d[0];
        let a1: u64 = (*a).d[1];
        let a2: u64 = (*a).d[2];
        let a3: u64 = (*a).d[3];

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!(scalar_check_overflow(a) == 0);
        }

        (*r).v[0] = (a0 & M62) as _;
        (*r).v[1] = (((a0 >> 62) | (a1 << 2)) & M62) as _;
        (*r).v[2] = (((a1 >> 60) | (a2 << 4)) & M62) as _;
        (*r).v[3] = (((a2 >> 58) | (a3 << 6)) & M62) as _;
        (*r).v[4] = (a3 >> 56) as _;
    }
}

#[cfg(test)]
#[cfg(feature = "widemul-int128")]
mod scalar_signed62_roundtrip_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_to_signed62_and_from_signed62_roundtrip_for_canonical_vectors() {
        info!("validating scalar_to_signed62 <-> scalar_from_signed62 roundtrip");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);

            let mut s62: ModInv64Signed62 = unsafe { core::mem::zeroed() };
            unsafe {
                scalar_to_signed62(&mut s62 as *mut ModInv64Signed62, &a as *const Scalar);
            }

            // Range checks: limbs 0..3 in [0,2^62), limb 4 in [0,2^8).
            for limb in 0..4usize {
                let v = s62.v[limb] as i128;
                debug!(i, limb, v, "signed62 limb");
                assert!(v >= 0);
                assert!((v as u128) < (1u128 << 62));
            }
            let top = s62.v[4] as i128;
            assert!(top >= 0);
            assert!((top as u128) < (1u128 << 8));

            let mut back = scalar_zero_value();
            unsafe {
                scalar_from_signed62(&mut back as *mut Scalar, &s62 as *const ModInv64Signed62);
            }
            let back_be = scalar_to_be_bytes(&back);

            debug!(i, ?a_be, ?back_be, "roundtrip");
            assert_eq!(back_be, *a_be);
        }
    }
}
