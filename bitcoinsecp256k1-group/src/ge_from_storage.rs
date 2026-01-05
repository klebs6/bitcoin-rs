// ---------------- [ File: bitcoinsecp256k1-group/src/ge_from_storage.rs ]
crate::ix!();

/// Convert a group element back from the storage type.
/// 
pub fn ge_from_storage(r: *mut Ge, a: *const GeStorage) {
    unsafe {
        fe_from_storage(core::ptr::addr_of_mut!((*r).x), core::ptr::addr_of!((*a).x));
        fe_from_storage(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!((*a).y));
        (*r).infinity = 0;
    }
}

#[cfg(test)]
mod ge_from_storage_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_from_storage_roundtrips_generator_via_ge_to_storage() {
        tracing::info!("Validating ge_to_storage + ge_from_storage roundtrip on generator.");

        unsafe {
            let mut st: GeStorage = core::mem::zeroed();
            ge_to_storage(core::ptr::addr_of_mut!(st), core::ptr::addr_of!(ge_const_g));

            let mut out: Ge = core::mem::zeroed();
            ge_from_storage(core::ptr::addr_of_mut!(out), core::ptr::addr_of!(st));

            assert!(ge_is_infinity(core::ptr::addr_of!(out)) == 0);
            assert!(
                secp256k1_group_exhaustive_test_support::ge_eq(&out, &ge_const_g)
            );
        }
    }
}
