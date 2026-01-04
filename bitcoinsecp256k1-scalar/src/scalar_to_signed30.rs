// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_to_signed30.rs ]
crate::ix!();

#[cfg(feature="widemul-int64")]
pub fn scalar_to_signed30(r: *mut ModInv32Signed30, a: *const Scalar) {
    unsafe {
        const M30: u32 = u32::MAX >> 2;

        let a0: u32 = (*a).d[0];
        let a1: u32 = (*a).d[1];
        let a2: u32 = (*a).d[2];
        let a3: u32 = (*a).d[3];
        let a4: u32 = (*a).d[4];
        let a5: u32 = (*a).d[5];
        let a6: u32 = (*a).d[6];
        let a7: u32 = (*a).d[7];

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!(scalar_check_overflow(a) == 0);
        }

        (*r).v[0] = (a0 & M30) as _;
        (*r).v[1] = ((a0 >> 30) | (a1 << 2) & M30) as _;
        (*r).v[2] = ((a1 >> 28) | (a2 << 4) & M30) as _;
        (*r).v[3] = ((a2 >> 26) | (a3 << 6) & M30) as _;
        (*r).v[4] = ((a3 >> 24) | (a4 << 8) & M30) as _;
        (*r).v[5] = ((a4 >> 22) | (a5 << 10) & M30) as _;
        (*r).v[6] = ((a5 >> 20) | (a6 << 12) & M30) as _;
        (*r).v[7] = ((a6 >> 18) | (a7 << 14) & M30) as _;
        (*r).v[8] = (a7 >> 16) as _;
    }
}

#[cfg(test)]
#[cfg(feature = "widemul-int64")]
mod scalar_signed30_roundtrip_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_to_signed30_and_from_signed30_roundtrip_for_canonical_vectors() {
        info!("validating scalar_to_signed30 <-> scalar_from_signed30 roundtrip");

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);

            let mut s30: ModInv32Signed30 = unsafe { core::mem::zeroed() };
            unsafe {
                scalar_to_signed30(&mut s30 as *mut ModInv32Signed30, &a as *const Scalar);
            }

            // Range checks: limbs 0..7 in [0,2^30), limb 8 in [0,2^16).
            for limb in 0..8usize {
                let v = s30.v[limb] as i64;
                debug!(i, limb, v, "signed30 limb");
                assert!(v >= 0);
                assert!((v as u64) < (1u64 << 30));
            }
            let top = s30.v[8] as i64;
            assert!(top >= 0);
            assert!((top as u64) < (1u64 << 16));

            let mut back = scalar_zero_value();
            unsafe {
                scalar_from_signed30(&mut back as *mut Scalar, &s30 as *const ModInv32Signed30);
            }
            let back_be = scalar_to_be_bytes(&back);

            debug!(i, ?a_be, ?back_be, "roundtrip");
            assert_eq!(back_be, *a_be);
        }
    }
}
