// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_infinity.rs ]
crate::ix!();

/**
  | Set a group element (affine) equal to
  | the point at infinity.
  |
  */
pub fn ge_set_infinity(r: *mut Ge) {
    unsafe {
        (*r).infinity = 1;
        fe_clear(core::ptr::addr_of_mut!((*r).x));
        fe_clear(core::ptr::addr_of_mut!((*r).y));
    }
}

#[cfg(test)]
mod ge_set_infinity_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_set_infinity_sets_flag_and_clears_coordinates() {
        tracing::info!("Validating ge_set_infinity sets infinity=1 and clears x/y.");

        unsafe {
            let mut p: Ge = core::mem::zeroed();
            core::ptr::copy(core::ptr::addr_of!(ge_const_g), core::ptr::addr_of_mut!(p), 1);
            assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);

            ge_set_infinity(core::ptr::addr_of_mut!(p));

            assert!(ge_is_infinity(core::ptr::addr_of!(p)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.x)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.y)) != 0);

            assert!(ge_is_valid_var(core::ptr::addr_of!(p)) == 0);
        }
    }
}
