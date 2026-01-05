// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_xy.rs ]
crate::ix!();

/**
  | Set a group element equal to the point
  | with given X and Y coordinates
  |
  */
pub fn ge_set_xy(
    r: *mut Ge,
    x: *const Fe,
    y: *const Fe
) {
    unsafe {
        (*r).infinity = 0;
        core::ptr::copy(x, core::ptr::addr_of_mut!((*r).x), 1);
        core::ptr::copy(y, core::ptr::addr_of_mut!((*r).y), 1);
    }
}

#[cfg(test)]
mod ge_set_xy_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_set_xy_sets_coordinates_and_resets_infinity() {
        tracing::info!("Validating ge_set_xy sets x/y and infinity=0.");

        unsafe {
            let mut p: Ge = core::mem::zeroed();
            ge_set_infinity(core::ptr::addr_of_mut!(p));
            assert!(ge_is_infinity(core::ptr::addr_of!(p)) != 0);

            let x: Fe = secp256k1_group_exhaustive_test_support::fe_int(1);
            let y: Fe = secp256k1_group_exhaustive_test_support::fe_int(2);

            ge_set_xy(
                core::ptr::addr_of_mut!(p),
                core::ptr::addr_of!(x),
                core::ptr::addr_of!(y),
            );

            assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);
            assert!(
                fe_equal_var(core::ptr::addr_of!(p.x), core::ptr::addr_of!(x)) != 0
            );
            assert!(
                fe_equal_var(core::ptr::addr_of!(p.y), core::ptr::addr_of!(y)) != 0
            );
        }
    }
}
