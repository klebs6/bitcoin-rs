// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_normalize_weak.rs ]
crate::ix!();

pub fn fe_normalize_weak(r: *mut Fe5x52)  {

    unsafe {
        let mut t0: u64 = (*r).n[0];
        let mut t1: u64 = (*r).n[1];
        let mut t2: u64 = (*r).n[2];
        let mut t3: u64 = (*r).n[3];
        let mut t4: u64 = (*r).n[4];

        /* Reduce t4 at the start so there will be at most a single carry from the first pass */
        let x: u64 = t4 >> 48;
        t4 &= 0x0FFFFFFFFFFFF_u64;

        /* The first pass ensures the magnitude is 1, ... */
        t0 = t0.wrapping_add(x.wrapping_mul(0x1000003D1_u64));
        t1 = t1.wrapping_add(t0 >> 52); t0 &= 0xFFFFFFFFFFFFF_u64;
        t2 = t2.wrapping_add(t1 >> 52); t1 &= 0xFFFFFFFFFFFFF_u64;
        t3 = t3.wrapping_add(t2 >> 52); t2 &= 0xFFFFFFFFFFFFF_u64;
        t4 = t4.wrapping_add(t3 >> 52); t3 &= 0xFFFFFFFFFFFFF_u64;

        /* ... except for a possible carry at bit 48 of t4 (i.e. bit 256 of the field element) */
        verify_check!((t4 >> 49) == 0);

        (*r).n[0] = t0; (*r).n[1] = t1; (*r).n[2] = t2; (*r).n[3] = t3; (*r).n[4] = t4;

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_normalize_weak_rs_exhaustive_tests {
    use super::*;

    const P0: u64 = 0xFFFFEFFFFFC2F_u64;
    const P1: u64 = 0xFFFFFFFFFFFFF_u64;
    const P2: u64 = 0xFFFFFFFFFFFFF_u64;
    const P3: u64 = 0xFFFFFFFFFFFFF_u64;
    const P4: u64 = 0x0FFFFFFFFFFFF_u64;

    unsafe fn fe_get_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_normalize_weak_preserves_value_after_full_normalization_and_masks_limb_ranges() {
        tracing::info!("testing fe_normalize_weak limb masking and compatibility with full normalization");

        unsafe {
            let mut x = Fe5x52::new();
            x.n[0] = P0; x.n[1] = P1; x.n[2] = P2; x.n[3] = P3; x.n[4] = P4; /* raw p */

            let mut full = x;
            crate::fe_normalize(&mut full as *mut Fe5x52);
            let full_b = {
                let mut tmp = full;
                let mut out = [0u8; 32];
                crate::fe_get_b32(out.as_mut_ptr(), &tmp as *const Fe5x52);
                out
            };

            let mut weak = x;
            crate::fe_normalize_weak(&mut weak as *mut Fe5x52);

            tracing::debug!(
                n0 = weak.n[0],
                n1 = weak.n[1],
                n2 = weak.n[2],
                n3 = weak.n[3],
                n4 = weak.n[4],
                "post-weak limb values"
            );

            assert!((weak.n[0] >> 52) == 0);
            assert!((weak.n[1] >> 52) == 0);
            assert!((weak.n[2] >> 52) == 0);
            assert!((weak.n[3] >> 52) == 0);
            assert!((weak.n[4] >> 49) == 0);

            let weak_full_b = fe_get_b32_normalized(&mut weak);
            assert_eq!(weak_full_b, full_b);

            tracing::debug!("additional case: propagate carries from an overfull limb");
            let mut y = Fe5x52::new();
            y.n[0] = (1u64 << 60) - 1;
            y.n[1] = (1u64 << 60) - 1;
            y.n[2] = (1u64 << 60) - 1;
            y.n[3] = (1u64 << 60) - 1;
            y.n[4] = (1u64 << 56) - 1;

            let mut y_full = y;
            crate::fe_normalize(&mut y_full as *mut Fe5x52);
            let y_full_b = {
                let mut out = [0u8; 32];
                crate::fe_get_b32(out.as_mut_ptr(), &y_full as *const Fe5x52);
                out
            };

            crate::fe_normalize_weak(&mut y as *mut Fe5x52);
            assert!((y.n[0] >> 52) == 0);
            assert!((y.n[1] >> 52) == 0);
            assert!((y.n[2] >> 52) == 0);
            assert!((y.n[3] >> 52) == 0);
            assert!((y.n[4] >> 49) == 0);

            let y_weak_full_b = fe_get_b32_normalized(&mut y);
            assert_eq!(y_weak_full_b, y_full_b);
        }
    }
}
