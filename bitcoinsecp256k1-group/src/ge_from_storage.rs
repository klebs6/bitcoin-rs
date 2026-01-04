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
