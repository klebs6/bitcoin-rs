// ---------------- [ File: bitcoinsecp256k1-group/src/ge_is_in_correct_subgroup.rs ]
crate::ix!();

/// Determine if a point (which is assumed to be on the curve) is in the correct (sub)group of the
/// curve.
/// 
/// In normal mode, the used group is secp256k1, which has cofactor=1 meaning that every point on
/// the curve is in the group, and this function returns always true.
/// 
/// When compiling in exhaustive test mode, a slightly different curve equation is used, leading to
/// a group with a (very) small subgroup, and that subgroup is what is used for all cryptographic
/// operations. In that mode, this function checks whether a point that is on the curve is in fact
/// also in that subgroup.
/// 
pub fn ge_is_in_correct_subgroup(ge: *const Ge) -> i32 {
    #[cfg(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199"))]
    unsafe {
        let mut out: Gej = core::mem::zeroed();
        let out_ptr: *mut Gej = core::ptr::addr_of_mut!(out);

        let mut i: i32 = 0;

        /* A very simple EC multiplication ladder that avoids a dependency on ecmult. */
        gej_set_infinity(out_ptr);
        while i < 32 {
            gej_double_var(out_ptr, out_ptr as *const Gej, core::ptr::null_mut());
            if (((EXHAUSTIVE_TEST_ORDER_U32) >> (31 - (i as u32))) & 1) != 0 {
                gej_add_ge_var(out_ptr, out_ptr as *const Gej, ge, core::ptr::null_mut());
            }
            i += 1;
        }
        gej_is_infinity(out_ptr as *const Gej)
    }

    #[cfg(not(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199")))]
    {
        let _ = ge;
        /* The real secp256k1 group has cofactor 1, so the subgroup is the entire curve. */
        1
    }
}
