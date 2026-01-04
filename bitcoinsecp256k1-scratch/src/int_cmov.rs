// ---------------- [ File: bitcoinsecp256k1-scratch/src/int_cmov.rs ]
crate::ix!();

/// If flag is true, set *r equal to *a; otherwise leave it. 
///
/// Constant-time. Both *r and *a must be initialized and non-negative.
/// 
#[inline] pub fn int_cmov(
    r:    *mut i32,
    a:    *const i32,
    flag: i32)  {

    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::util",
            r = r as usize,
            a = a as usize,
            flag,
            "int_cmov"
        );

        /* Access flag with a volatile-qualified lvalue.
           This prevents clang from figuring out (after inlining) that flag can
           take only be 0 or 1, which leads to variable time code. */
        let vflag: i32 = core::ptr::read_volatile(&flag);

        /* Casting a negative int to unsigned and back to int is implementation defined behavior */
        verify_check!{ (*r >= 0) && (*a >= 0) };

        let mask0: u32 = (vflag as u32).wrapping_add(!0u32);
        let mask1: u32 = !mask0;

        let r_masked: u32 = (*r as u32) & mask0;
        let a_masked: u32 = (*a as u32) & mask1;

        *r = (r_masked | a_masked) as i32;
    }
}

#[cfg(test)]
mod int_cmov_contract_test_suite {
    use super::*;

    #[traced_test]
    fn int_cmov_leaves_destination_unchanged_when_flag_is_zero() {
        let mut r: i32 = 5;
        let a: i32 = 9;

        info!(
            target: "bitcoinsecp256k1_scratch::tests::int_cmov",
            r,
            a,
            "before int_cmov(flag=0)"
        );

        int_cmov(&mut r as *mut i32, &a as *const i32, 0);

        info!(
            target: "bitcoinsecp256k1_scratch::tests::int_cmov",
            r,
            a,
            "after int_cmov(flag=0)"
        );

        assert_eq!(r, 5);
    }

    #[traced_test]
    fn int_cmov_copies_source_into_destination_when_flag_is_one() {
        let mut r: i32 = 5;
        let a: i32 = 9;

        info!(
            target: "bitcoinsecp256k1_scratch::tests::int_cmov",
            r,
            a,
            "before int_cmov(flag=1)"
        );

        int_cmov(&mut r as *mut i32, &a as *const i32, 1);

        info!(
            target: "bitcoinsecp256k1_scratch::tests::int_cmov",
            r,
            a,
            "after int_cmov(flag=1)"
        );

        assert_eq!(r, 9);
    }

    #[traced_test]
    fn int_cmov_supports_overlapping_pointers_safely() {
        let mut v: i32 = 7;

        debug!(
            target: "bitcoinsecp256k1_scratch::tests::int_cmov",
            v,
            "before overlapping int_cmov(flag=0)"
        );

        int_cmov(&mut v as *mut i32, &v as *const i32, 0);
        assert_eq!(v, 7);

        debug!(
            target: "bitcoinsecp256k1_scratch::tests::int_cmov",
            v,
            "before overlapping int_cmov(flag=1)"
        );

        int_cmov(&mut v as *mut i32, &v as *const i32, 1);
        assert_eq!(v, 7);
    }
}
