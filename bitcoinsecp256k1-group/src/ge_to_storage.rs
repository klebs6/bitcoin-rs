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
