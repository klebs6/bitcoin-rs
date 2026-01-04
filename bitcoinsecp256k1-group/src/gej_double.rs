// ---------------- [ File: bitcoinsecp256k1-group/src/gej_double.rs ]
crate::ix!();

/// Set r equal to the double of a. Constant time.
/// 
#[inline]
pub fn gej_double(r: *mut Gej, a: *const Gej) {
    unsafe {
        /* Operations: 3 mul, 4 sqr, 0 normalize, 12 mul_int/add/negate.
         *
         * Note that there is an implementation described at
         *     https://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l
         * which trades a multiply for a square, but in practice this is actually slower,
         * mainly because it requires more normalizations.
         */
        let mut t1: Fe = core::mem::zeroed();
        let mut t2: Fe = core::mem::zeroed();
        let mut t3: Fe = core::mem::zeroed();
        let mut t4: Fe = core::mem::zeroed();

        (*r).infinity = (*a).infinity;

        fe_mul(
            core::ptr::addr_of_mut!((*r).z),
            core::ptr::addr_of!((*a).z),
            core::ptr::addr_of!((*a).y),
        );

        /* Z' = 2*Y*Z (2) */
        fe_mul_int(core::ptr::addr_of_mut!((*r).z), 2); 

        fe_sqr(core::ptr::addr_of_mut!(t1), core::ptr::addr_of!((*a).x));

        /* T1 = 3*X^2 (3) */
        fe_mul_int(core::ptr::addr_of_mut!(t1), 3); 

        /* T2 = 9*X^4 (1) */
        fe_sqr(core::ptr::addr_of_mut!(t2), core::ptr::addr_of!(t1)); 

        fe_sqr(core::ptr::addr_of_mut!(t3), core::ptr::addr_of!((*a).y));

        /* T3 = 2*Y^2 (2) */
        fe_mul_int(core::ptr::addr_of_mut!(t3), 2); 

        fe_sqr(core::ptr::addr_of_mut!(t4), core::ptr::addr_of!(t3));

        /* T4 = 8*Y^4 (2) */
        fe_mul_int(core::ptr::addr_of_mut!(t4), 2); 

        /* T3 = 2*X*Y^2 (1) */
        fe_mul(
            core::ptr::addr_of_mut!(t3),
            core::ptr::addr_of!(t3),
            core::ptr::addr_of!((*a).x),
        ); 

        core::ptr::copy(core::ptr::addr_of!(t3), core::ptr::addr_of_mut!((*r).x), 1);

        /* X' = 8*X*Y^2 (4) */
        fe_mul_int(core::ptr::addr_of_mut!((*r).x), 4); 

        let rx: *mut Fe = core::ptr::addr_of_mut!((*r).x);

        /* X' = -8*X*Y^2 (5) */
        fe_negate(rx, rx as *const Fe, 4); 

        /* X' = 9*X^4 - 8*X*Y^2 (6) */
        fe_add(rx, core::ptr::addr_of!(t2)); 

        let t2_ptr: *mut Fe = core::ptr::addr_of_mut!(t2);

        /* T2 = -9*X^4 (2) */
        fe_negate(t2_ptr, t2_ptr as *const Fe, 1); 

        /* T3 = 12*X*Y^2 (6) */
        fe_mul_int(core::ptr::addr_of_mut!(t3), 6); 

        /* T3 = 12*X*Y^2 - 9*X^4 (8) */
        fe_add(core::ptr::addr_of_mut!(t3), core::ptr::addr_of!(t2)); 

        /* Y' = 36*X^3*Y^2 - 27*X^6 (1) */
        fe_mul(
            core::ptr::addr_of_mut!((*r).y),
            core::ptr::addr_of!(t1),
            core::ptr::addr_of!(t3),
        ); 

        /* T2 = -8*Y^4 (3) */
        fe_negate(t2_ptr, core::ptr::addr_of!(t4), 2); 

        /* Y' = 36*X^3*Y^2 - 27*X^6 - 8*Y^4 (4) */
        fe_add(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(t2)); 
    }
}
