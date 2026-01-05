// ---------------- [ File: bitcoinsecp256k1-group/src/ge_clear.rs ]
crate::ix!();

/**
  | Clear a ge to prevent leaking
  | sensitive information.
  |
  */
pub fn ge_clear(r: *mut Ge) {
    unsafe {
        (*r).infinity = 0;
        fe_clear(core::ptr::addr_of_mut!((*r).x));
        fe_clear(core::ptr::addr_of_mut!((*r).y));
    }
}

#[cfg(test)]
mod ge_clear_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_clear_zeroizes_coordinates_and_resets_infinity_flag() {
        tracing::info!("Validating ge_clear() clears x/y and sets infinity=0.");

        unsafe {
            let mut p: Ge = core::mem::zeroed();
            core::ptr::copy(core::ptr::addr_of!(ge_const_g), core::ptr::addr_of_mut!(p), 1);

            assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);
            assert!(ge_is_valid_var(core::ptr::addr_of!(p)) != 0);

            ge_clear(core::ptr::addr_of_mut!(p));

            assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.x)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.y)) != 0);

            assert!(ge_is_valid_var(core::ptr::addr_of!(p)) == 0);
        }
    }

    #[traced_test]
    fn ge_clear_overwrites_infinity_point_into_non_infinity_zero_point() {
        tracing::info!("Validating ge_clear() on an infinity point results in infinity=0 and zero coordinates.");

        unsafe {
            let mut p: Ge = core::mem::zeroed();
            ge_set_infinity(core::ptr::addr_of_mut!(p));
            assert!(ge_is_infinity(core::ptr::addr_of!(p)) != 0);

            ge_clear(core::ptr::addr_of_mut!(p));

            assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.x)) != 0);
            assert!(fe_is_zero(core::ptr::addr_of!(p.y)) != 0);
        }
    }
}
