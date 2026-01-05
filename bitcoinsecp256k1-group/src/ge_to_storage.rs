// ---------------- [ File: bitcoinsecp256k1-group/src/ge_to_storage.rs ]
crate::ix!();

/**
  | Convert a group element to the storage
  | type.
  |
  */
pub fn ge_to_storage(r: *mut GeStorage, a: *const Ge) {
    unsafe {
        let mut x: Fe = core::mem::zeroed();
        let mut y: Fe = core::mem::zeroed();

        verify_check!((*a).infinity == 0);

        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!(x), 1);
        fe_normalize(core::ptr::addr_of_mut!(x));

        core::ptr::copy(core::ptr::addr_of!((*a).y), core::ptr::addr_of_mut!(y), 1);
        fe_normalize(core::ptr::addr_of_mut!(y));

        fe_to_storage(core::ptr::addr_of_mut!((*r).x), core::ptr::addr_of!(x));
        fe_to_storage(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(y));
    }
}

#[cfg(test)]
mod ge_to_storage_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_to_storage_then_ge_from_storage_roundtrips_generator() {
        tracing::info!("Validating ge_to_storage + ge_from_storage roundtrip for generator.");

        unsafe {
            let mut st: GeStorage = core::mem::zeroed();
            ge_to_storage(core::ptr::addr_of_mut!(st), core::ptr::addr_of!(ge_const_g));

            let mut out: Ge = core::mem::zeroed();
            ge_from_storage(core::ptr::addr_of_mut!(out), core::ptr::addr_of!(st));

            assert!(secp256k1_group_exhaustive_test_support::ge_eq(&out, &ge_const_g));
        }
    }
}
