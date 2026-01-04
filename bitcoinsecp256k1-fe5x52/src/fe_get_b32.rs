// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_get_b32.rs ]
crate::ix!();

/// Convert a field element to a 32-byte big endian value. Requires the input to be normalized
/// 
pub fn fe_get_b32(
        r: *mut u8,
        a: *const Fe5x52)  {

    unsafe {
        #[cfg(feature="secp256k1-verify")]
        {
            verify_check((*a).normalized != 0);
            fe_verify(a);
        }

        let t = (*a).n;

        *r.add(0)  = ((t[4] >> 40) & 0xFF) as u8;
        *r.add(1)  = ((t[4] >> 32) & 0xFF) as u8;
        *r.add(2)  = ((t[4] >> 24) & 0xFF) as u8;
        *r.add(3)  = ((t[4] >> 16) & 0xFF) as u8;
        *r.add(4)  = ((t[4] >> 8)  & 0xFF) as u8;
        *r.add(5)  = ( t[4]        & 0xFF) as u8;
        *r.add(6)  = ((t[3] >> 44) & 0xFF) as u8;
        *r.add(7)  = ((t[3] >> 36) & 0xFF) as u8;
        *r.add(8)  = ((t[3] >> 28) & 0xFF) as u8;
        *r.add(9)  = ((t[3] >> 20) & 0xFF) as u8;
        *r.add(10) = ((t[3] >> 12) & 0xFF) as u8;
        *r.add(11) = ((t[3] >> 4)  & 0xFF) as u8;
        *r.add(12) = ((((t[2] >> 48) & 0xF) | ((t[3] & 0xF) << 4)) & 0xFF) as u8;
        *r.add(13) = ((t[2] >> 40) & 0xFF) as u8;
        *r.add(14) = ((t[2] >> 32) & 0xFF) as u8;
        *r.add(15) = ((t[2] >> 24) & 0xFF) as u8;
        *r.add(16) = ((t[2] >> 16) & 0xFF) as u8;
        *r.add(17) = ((t[2] >> 8)  & 0xFF) as u8;
        *r.add(18) = ( t[2]        & 0xFF) as u8;
        *r.add(19) = ((t[1] >> 44) & 0xFF) as u8;
        *r.add(20) = ((t[1] >> 36) & 0xFF) as u8;
        *r.add(21) = ((t[1] >> 28) & 0xFF) as u8;
        *r.add(22) = ((t[1] >> 20) & 0xFF) as u8;
        *r.add(23) = ((t[1] >> 12) & 0xFF) as u8;
        *r.add(24) = ((t[1] >> 4)  & 0xFF) as u8;
        *r.add(25) = ((((t[0] >> 48) & 0xF) | ((t[1] & 0xF) << 4)) & 0xFF) as u8;
        *r.add(26) = ((t[0] >> 40) & 0xFF) as u8;
        *r.add(27) = ((t[0] >> 32) & 0xFF) as u8;
        *r.add(28) = ((t[0] >> 24) & 0xFF) as u8;
        *r.add(29) = ((t[0] >> 16) & 0xFF) as u8;
        *r.add(30) = ((t[0] >> 8)  & 0xFF) as u8;
        *r.add(31) = ( t[0]        & 0xFF) as u8;
    }
}

#[cfg(test)]
mod fe_get_b32_rs_exhaustive_tests {
    use super::*;

    const SAMPLE_B32: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF,
        0xFE, 0xED, 0xFA, 0xCE, 0x12, 0x34, 0x56, 0x78,
        0x90, 0xAB, 0xCD, 0xEF, 0xAA, 0x55, 0xAA, 0x55,
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
    ];

    fn u64_to_be32(v: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out
    }

    unsafe fn fe_from_b32_checked(bytes: &[u8; 32]) -> Fe5x52 {
        let mut fe = Fe5x52::new();
        let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, bytes.as_ptr());
        assert_eq!(ret, 1);
        fe
    }

    unsafe fn fe_get_b32_from_normalized(a: &Fe5x52) -> [u8; 32] {
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), a as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_get_b32_matches_original_bytes_for_normalized_inputs() {
        tracing::info!("testing fe_get_b32 against values constructed by fe_set_b32");

        unsafe {
            let samples: [&[u8; 32]; 6] = [
                &u64_to_be32(0),
                &u64_to_be32(1),
                &u64_to_be32(2),
                &u64_to_be32(255),
                &u64_to_be32(1u64 << 52),
                &SAMPLE_B32,
            ];

            for (idx, s) in samples.iter().enumerate() {
                tracing::debug!(sample_index = idx, "fe_set_b32 then fe_get_b32 should roundtrip");
                let mut a = fe_from_b32_checked(s);
                crate::fe_normalize(&mut a as *mut Fe5x52);
                let got = fe_get_b32_from_normalized(&a);
                assert_eq!(got, **s);
            }
        }
    }
}
