// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_set_b32.rs ]
crate::ix!();

pub fn fe_set_b32(
    r: *mut Fe10x26,
    a: *const u8) -> i32 {

    unsafe {
        (*r).n[0] = (*a.add(31) as u32) | ((*a.add(30) as u32) << 8) | ((*a.add(29) as u32) << 16) | (((*a.add(28) as u32) & 0x3) << 24);
        (*r).n[1] = (((*a.add(28) as u32) >> 2) & 0x3f) | ((*a.add(27) as u32) << 6) | ((*a.add(26) as u32) << 14) | (((*a.add(25) as u32) & 0xf) << 22);
        (*r).n[2] = (((*a.add(25) as u32) >> 4) & 0xf) | ((*a.add(24) as u32) << 4) | ((*a.add(23) as u32) << 12) | (((*a.add(22) as u32) & 0x3f) << 20);
        (*r).n[3] = (((*a.add(22) as u32) >> 6) & 0x3) | ((*a.add(21) as u32) << 2) | ((*a.add(20) as u32) << 10) | ((*a.add(19) as u32) << 18);
        (*r).n[4] = (*a.add(18) as u32) | ((*a.add(17) as u32) << 8) | ((*a.add(16) as u32) << 16) | (((*a.add(15) as u32) & 0x3) << 24);
        (*r).n[5] = (((*a.add(15) as u32) >> 2) & 0x3f) | ((*a.add(14) as u32) << 6) | ((*a.add(13) as u32) << 14) | (((*a.add(12) as u32) & 0xf) << 22);
        (*r).n[6] = (((*a.add(12) as u32) >> 4) & 0xf) | ((*a.add(11) as u32) << 4) | ((*a.add(10) as u32) << 12) | (((*a.add(9) as u32) & 0x3f) << 20);
        (*r).n[7] = (((*a.add(9) as u32) >> 6) & 0x3) | ((*a.add(8) as u32) << 2) | ((*a.add(7) as u32) << 10) | ((*a.add(6) as u32) << 18);
        (*r).n[8] = (*a.add(5) as u32) | ((*a.add(4) as u32) << 8) | ((*a.add(3) as u32) << 16) | (((*a.add(2) as u32) & 0x3) << 24);
        (*r).n[9] = (((*a.add(2) as u32) >> 2) & 0x3f) | ((*a.add(1) as u32) << 6) | ((*a.add(0) as u32) << 14);

        let cond: u32 =
            ((((*r).n[9] == 0x3FFFFFu32) as u32)
             & ((((*r).n[8] & (*r).n[7] & (*r).n[6] & (*r).n[5] & (*r).n[4] & (*r).n[3] & (*r).n[2]) == 0x3FFFFFFu32) as u32)
             & ((((*r).n[1].wrapping_add(0x40u32).wrapping_add(((*r).n[0].wrapping_add(0x3D1u32)) >> 26)) > 0x3FFFFFFu32) as u32));

        let ret: i32 = if cond != 0 { 0 } else { 1 };

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            if ret != 0 {
                (*r).normalized = 1;
                fe_verify(r);
            } else {
                (*r).normalized = 0;
            }
        }

        ret
    }
}

#[cfg(test)]
mod fe_set_b32_interface_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn fe_set_b32_accepts_values_below_p_and_roundtrips() {
        info!("fe_set_b32 should accept <p and allow roundtrip through fe_get_b32");
        let vectors: [&[u8; 32]; 6] = [
            &BYTES_ZERO,
            &BYTES_ONE,
            &BYTES_TWO,
            &BYTES_PATTERN_A,
            &BYTES_2_POW_255,
            &FIELD_PRIME_MINUS_ONE_BYTES_BE,
        ];

        for v in vectors {
            let mut r = Fe10x26::new();
            let ret = unsafe { fe_set_b32(&mut r as *mut Fe10x26, v.as_ptr()) };
            debug!(ret, "fe_set_b32 ret");
            assert_ne!(ret, 0);

            fe_normalize_in_place(&mut r);
            let mut out = [0u8; 32];
            unsafe { fe_get_b32(out.as_mut_ptr(), &r as *const Fe10x26) };

            trace!(?out, "roundtrip output");
            assert_eq!(&out, v);

            #[cfg(feature = "secp256k1-verify")]
            {
                assert_eq!(r.magnitude, 1);
                assert_eq!(r.normalized, 1);
            }
        }
    }

    #[traced_test]
    fn fe_set_b32_rejects_p_and_above() {
        info!("fe_set_b32 should reject p, p+1, and max(2^256-1)");
        let reject_vectors: [&[u8; 32]; 3] = [
            &FIELD_PRIME_BYTES_BE,
            &FIELD_PRIME_PLUS_ONE_BYTES_BE,
            &BYTES_MAX,
        ];

        for v in reject_vectors {
            let mut r = Fe10x26::new();
            let ret = unsafe { fe_set_b32(&mut r as *mut Fe10x26, v.as_ptr()) };
            debug!(ret, "fe_set_b32 ret for rejected value");
            assert_eq!(ret, 0);

            #[cfg(feature = "secp256k1-verify")]
            {
                assert_eq!(r.normalized, 0);
                assert_eq!(r.magnitude, 1);
            }
        }
    }
}

#[cfg(test)]
mod fe_set_b32_additional_boundary_vectors_suite {
    use super::*;
    use crate::fe10x26_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn fe_set_b32_accepts_p_minus_two_and_roundtrips() {
        info!("fe_set_b32 should accept p-2 and roundtrip via fe_get_b32 after normalization");
        let mut r = Fe10x26::new();
        let ret = unsafe { fe_set_b32(&mut r as *mut Fe10x26, FIELD_PRIME_MINUS_TWO_BYTES_BE.as_ptr()) };
        debug!(ret, "fe_set_b32(p-2) ret");
        assert_ne!(ret, 0);

        fe_normalize_in_place(&mut r);

        let mut out = [0u8; 32];
        unsafe { fe_get_b32(out.as_mut_ptr(), &r as *const Fe10x26) };

        trace!(?out, "p-2 roundtrip bytes");
        assert_eq!(out, FIELD_PRIME_MINUS_TWO_BYTES_BE);
    }
}
