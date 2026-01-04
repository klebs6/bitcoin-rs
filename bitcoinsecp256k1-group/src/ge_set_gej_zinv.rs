// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_gej_zinv.rs ]
crate::ix!();

pub fn ge_set_gej_zinv(r: *mut Ge, a: *const Gej, zi: *const Fe) {
    unsafe {
        let mut zi2: Fe = core::mem::zeroed();
        let mut zi3: Fe = core::mem::zeroed();
        fe_sqr(core::ptr::addr_of_mut!(zi2), zi);
        fe_mul(
            core::ptr::addr_of_mut!(zi3),
            core::ptr::addr_of!(zi2),
            zi,
        );
        fe_mul(
            core::ptr::addr_of_mut!((*r).x),
            core::ptr::addr_of!((*a).x),
            core::ptr::addr_of!(zi2),
        );
        fe_mul(
            core::ptr::addr_of_mut!((*r).y),
            core::ptr::addr_of!((*a).y),
            core::ptr::addr_of!(zi3),
        );
        (*r).infinity = (*a).infinity;
    }
}
