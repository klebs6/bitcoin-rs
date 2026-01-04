// ---------------- [ File: bitcoinsecp256k1-group/src/gej_double_var.rs ]
crate::ix!();

/// Set r equal to the double of a. 
///
/// If rzr is not-NULL this sets *rzr such that r->z == a->z * *rzr (where infinity means an
/// implicit z = 0).
/// 
pub fn gej_double_var(r: *mut Gej, a: *const Gej, rzr: *mut Fe) {
    unsafe {
        /** For secp256k1, 2Q is infinity if and only if Q is infinity. This is because if 2Q = infinity,
         *  Q must equal -Q, or that Q.y == -(Q.y), or Q.y is 0. For a point on y^2 = x^3 + 7 to have
         *  y=0, x^3 must be -7 mod p. However, -7 has no cube root mod p.
         *
         *  Having said this, if this function receives a point on a sextic twist, e.g. by
         *  a fault attack, it is possible for y to be 0. This happens for y^2 = x^3 + 6,
         *  since -6 does have a cube root mod p. For this point, this function will not set
         *  the infinity flag even though the point doubles to infinity, and the result
         *  point will be gibberish (z = 0 but infinity = 0).
         */
        if (*a).infinity != 0 {
            gej_set_infinity(r);
            if !rzr.is_null() {
                fe_set_int(rzr, 1);
            }
            return;
        }

        if !rzr.is_null() {
            core::ptr::copy(core::ptr::addr_of!((*a).y), rzr, 1);
            fe_normalize_weak(rzr);
            fe_mul_int(rzr, 2);
        }

        gej_double(r, a);
    }
}
