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
