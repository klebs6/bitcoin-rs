// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_set_b32.rs ]
crate::ix!();

pub fn fe_set_b32(
        r: *mut Fe5x52,
        a: *const u8) -> i32 {

    unsafe {
        let ret: i32;

        let b0  = *a.add(0)  as u64;
        let b1  = *a.add(1)  as u64;
        let b2  = *a.add(2)  as u64;
        let b3  = *a.add(3)  as u64;
        let b4  = *a.add(4)  as u64;
        let b5  = *a.add(5)  as u64;
        let b6  = *a.add(6)  as u64;
        let b7  = *a.add(7)  as u64;
        let b8  = *a.add(8)  as u64;
        let b9  = *a.add(9)  as u64;
        let b10 = *a.add(10) as u64;
        let b11 = *a.add(11) as u64;
        let b12 = *a.add(12) as u64;
        let b13 = *a.add(13) as u64;
        let b14 = *a.add(14) as u64;
        let b15 = *a.add(15) as u64;
        let b16 = *a.add(16) as u64;
        let b17 = *a.add(17) as u64;
        let b18 = *a.add(18) as u64;
        let b19 = *a.add(19) as u64;
        let b20 = *a.add(20) as u64;
        let b21 = *a.add(21) as u64;
        let b22 = *a.add(22) as u64;
        let b23 = *a.add(23) as u64;
        let b24 = *a.add(24) as u64;
        let b25 = *a.add(25) as u64;
        let b26 = *a.add(26) as u64;
        let b27 = *a.add(27) as u64;
        let b28 = *a.add(28) as u64;
        let b29 = *a.add(29) as u64;
        let b30 = *a.add(30) as u64;
        let b31 = *a.add(31) as u64;

        (*r).n[0] = b31
            | (b30 << 8)
            | (b29 << 16)
            | (b28 << 24)
            | (b27 << 32)
            | (b26 << 40)
            | (((b25 & 0xF) as u64) << 48);

        (*r).n[1] = ((b25 >> 4) & 0xF)
            | (b24 << 4)
            | (b23 << 12)
            | (b22 << 20)
            | (b21 << 28)
            | (b20 << 36)
            | (b19 << 44);

        (*r).n[2] = b18
            | (b17 << 8)
            | (b16 << 16)
            | (b15 << 24)
            | (b14 << 32)
            | (b13 << 40)
            | (((b12 & 0xF) as u64) << 48);

        (*r).n[3] = ((b12 >> 4) & 0xF)
            | (b11 << 4)
            | (b10 << 12)
            | (b9  << 20)
            | (b8  << 28)
            | (b7  << 36)
            | (b6  << 44);

        (*r).n[4] = b5
            | (b4 << 8)
            | (b3 << 16)
            | (b2 << 24)
            | (b1 << 32)
            | (b0 << 40);

        let cond: u64 =
            ((((*r).n[4] == 0x0FFFFFFFFFFFF_u64) as u64)
                & ((((*r).n[3] & (*r).n[2] & (*r).n[1]) == 0xFFFFFFFFFFFFF_u64) as u64)
                & (((*r).n[0] >= 0xFFFFEFFFFFC2F_u64) as u64));

        ret = ((cond ^ 1u64) & 1u64) as i32;

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
mod fe_set_b32_rs_exhaustive_tests {
    use super::*;

    const FIELD_P_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
    ];

    const FIELD_P_MINUS_1_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2E,
    ];

    fn u64_to_be32(v: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[24..32].copy_from_slice(&v.to_be_bytes());
        out
    }

    fn add_one_be32(x: &[u8; 32]) -> [u8; 32] {
        let mut out = *x;
        let mut i: i32 = 31;
        let mut carry: u16 = 1;
        while i >= 0 && carry != 0 {
            let v = out[i as usize] as u16 + carry;
            out[i as usize] = (v & 0xFF) as u8;
            carry = v >> 8;
            i -= 1;
        }
        out
    }

    unsafe fn fe_get_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_set_b32_accepts_values_below_p_and_rejects_p_and_above() {
        tracing::info!("testing fe_set_b32 accept/reject boundaries");

        unsafe {
            let valid: [&[u8; 32]; 3] = [&u64_to_be32(0), &u64_to_be32(1), &FIELD_P_MINUS_1_B32];

            for (idx, v) in valid.iter().enumerate() {
                tracing::debug!(valid_index = idx, "expect accept");
                let mut fe = Fe5x52::new();
                let ret = crate::fe_set_b32(&mut fe as *mut Fe5x52, v.as_ptr());
                assert_eq!(ret, 1);
                let got = fe_get_b32_normalized(&mut fe);
                assert_eq!(got, **v);
            }

            tracing::debug!("p should be rejected");
            let mut fe_p = Fe5x52::new();
            let ret_p = crate::fe_set_b32(&mut fe_p as *mut Fe5x52, FIELD_P_B32.as_ptr());
            assert_eq!(ret_p, 0);

            tracing::debug!("p+1 should be rejected");
            let p_plus_1 = add_one_be32(&FIELD_P_B32);
            let mut fe_p1 = Fe5x52::new();
            let ret_p1 = crate::fe_set_b32(&mut fe_p1 as *mut Fe5x52, p_plus_1.as_ptr());
            assert_eq!(ret_p1, 0);
        }
    }
}
