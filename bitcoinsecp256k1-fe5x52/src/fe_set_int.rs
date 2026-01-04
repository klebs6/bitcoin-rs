// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_set_int.rs ]
crate::ix!();

#[inline] pub fn fe_set_int(
        r: *mut Fe5x52,
        a: i32)  {

    unsafe {
        (*r).n[0] = a as u64;
        (*r).n[1] = 0;
        (*r).n[2] = 0;
        (*r).n[3] = 0;
        (*r).n[4] = 0;

        #[cfg(feature="secp256k1-verify")]
        {
            (*r).magnitude = 1;
            (*r).normalized = 1;
            fe_verify(r);
        }
    }
}

#[cfg(test)]
mod fe_set_int_rs_exhaustive_tests {
    use super::*;

    unsafe fn fe_get_b32_normalized(fe: &mut Fe5x52) -> [u8; 32] {
        crate::fe_normalize(fe as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), fe as *const Fe5x52);
        out
    }

    fn i32_to_be32(v: i32) -> [u8; 32] {
        let mut out = [0u8; 32];
        let be = (v as u32).to_be_bytes();
        out[28..32].copy_from_slice(&be);
        out
    }

    #[traced_test]
    fn fe_set_int_sets_expected_canonical_values_for_representative_inputs() {
        tracing::info!("testing fe_set_int for representative i32 values");

        unsafe {
            let vals: [i32; 6] = [0, 1, 2, 7, 255, 0x7FFFFFFF];

            for &v in vals.iter() {
                tracing::debug!(value_i32 = v, "setting int");
                let mut fe = Fe5x52::new();
                crate::fe_set_int(&mut fe as *mut Fe5x52, v);
                let got = fe_get_b32_normalized(&mut fe);
                let expected = i32_to_be32(v);
                assert_eq!(got, expected);
            }
        }
    }
}
