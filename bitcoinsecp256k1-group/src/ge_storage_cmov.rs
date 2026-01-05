// ---------------- [ File: bitcoinsecp256k1-group/src/ge_storage_cmov.rs ]
crate::ix!();

/// If flag is true, set *r equal to *a; otherwise leave it. 
///
/// Constant-time. Both *r and *a must be initialized.
/// 
#[inline]
pub fn ge_storage_cmov(r: *mut GeStorage, a: *const GeStorage, flag: i32) {
    unsafe {
        fe_storage_cmov(core::ptr::addr_of_mut!((*r).x), core::ptr::addr_of!((*a).x), flag);
        fe_storage_cmov(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!((*a).y), flag);
    }
}

#[cfg(test)]
mod ge_storage_cmov_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_storage_cmov_selects_expected_input_based_on_flag() {
        tracing::info!("Validating ge_storage_cmov chooses between r and a based on flag.");

        unsafe {
            let a: GeStorage = ge_storage_const!(
                0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 2
            );

            let mut r: GeStorage = ge_storage_const!(
                0, 0, 0, 0, 0, 0, 0, 3,
                0, 0, 0, 0, 0, 0, 0, 4
            );

            let mut r0: Ge = core::mem::zeroed();
            ge_from_storage(core::ptr::addr_of_mut!(r0), core::ptr::addr_of!(r));

            ge_storage_cmov(core::ptr::addr_of_mut!(r), core::ptr::addr_of!(a), 0);

            let mut r_after0: Ge = core::mem::zeroed();
            ge_from_storage(core::ptr::addr_of_mut!(r_after0), core::ptr::addr_of!(r));
            assert!(secp256k1_group_exhaustive_test_support::ge_eq(&r_after0, &r0));

            ge_storage_cmov(core::ptr::addr_of_mut!(r), core::ptr::addr_of!(a), 1);

            let mut r_after1: Ge = core::mem::zeroed();
            ge_from_storage(core::ptr::addr_of_mut!(r_after1), core::ptr::addr_of!(r));

            let mut a_ge: Ge = core::mem::zeroed();
            ge_from_storage(core::ptr::addr_of_mut!(a_ge), core::ptr::addr_of!(a));

            assert!(secp256k1_group_exhaustive_test_support::ge_eq(&r_after1, &a_ge));
        }
    }
}
