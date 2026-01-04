// ---------------- [ File: bitcoinsecp256k1-group/src/ge_mul_lambda.rs ]
crate::ix!();

/// Set r to be equal to lambda times a, where lambda is chosen in a way such that this is very
/// fast.
/// 
pub fn ge_mul_lambda(r: *mut Ge, a: *const Ge) {
    unsafe {
        static beta: Fe = fe_const!(
            0x7ae96a2b_u32,
            0x657c0710_u32,
            0x6e64479e_u32,
            0xac3434e9_u32,
            0x9cf04975_u32,
            0x12f58995_u32,
            0xc1396c28_u32,
            0x719501ee_u32
        );
        core::ptr::copy(a, r, 1);
        let rx: *mut Fe = core::ptr::addr_of_mut!((*r).x);
        fe_mul(rx, rx as *const Fe, core::ptr::addr_of!(beta));
    }
}
