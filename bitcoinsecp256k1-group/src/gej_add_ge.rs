// ---------------- [ File: bitcoinsecp256k1-group/src/gej_add_ge.rs ]
crate::ix!();

/**
  | Set r equal to the sum of a and b (with b
  | given in affine coordinates, and not
  | infinity).
  |
  */
pub fn gej_add_ge(r: *mut Gej, a: *const Gej, b: *const Ge) {
    unsafe {
        /* Operations: 7 mul, 5 sqr, 4 normalize, 21 mul_int/add/negate/cmov */
        static fe_1: Fe = fe_const!(0, 0, 0, 0, 0, 0, 0, 1);

        let mut zz: Fe = core::mem::zeroed();
        let mut u1: Fe = core::mem::zeroed();
        let mut u2: Fe = core::mem::zeroed();
        let mut s1: Fe = core::mem::zeroed();
        let mut s2: Fe = core::mem::zeroed();
        let mut t: Fe = core::mem::zeroed();
        let mut tt: Fe = core::mem::zeroed();
        let mut m: Fe = core::mem::zeroed();
        let mut n: Fe = core::mem::zeroed();
        let mut q: Fe = core::mem::zeroed();
        let mut rr: Fe = core::mem::zeroed();

        let mut m_alt: Fe = core::mem::zeroed();
        let mut rr_alt: Fe = core::mem::zeroed();

        let mut infinity: i32;
        let mut degenerate: i32;

        verify_check!((*b).infinity == 0);
        verify_check!((*a).infinity == 0 || (*a).infinity == 1);

        /** In:
         *    Eric Brier and Marc Joye, Weierstrass Elliptic Curves and Side-Channel Attacks.
         *    In D. Naccache and P. Paillier, Eds., Public Key Cryptography, vol. 2274 of Lecture Notes in Computer Science, pages 335-345. Springer-Verlag, 2002.
         *  we find as solution for a unified addition/doubling formula:
         *    lambda = ((x1 + x2)^2 - x1 * x2 + a) / (y1 + y2), with a = 0 for secp256k1's curve equation.
         *    x3 = lambda^2 - (x1 + x2)
         *    2*y3 = lambda * (x1 + x2 - 2 * x3) - (y1 + y2).
         *
         *  Substituting x_i = Xi / Zi^2 and yi = Yi / Zi^3, for i=1,2,3, gives:
         *    U1 = X1*Z2^2, U2 = X2*Z1^2
         *    S1 = Y1*Z2^3, S2 = Y2*Z1^3
         *    Z = Z1*Z2
         *    T = U1+U2
         *    M = S1+S2
         *    Q = T*M^2
         *    R = T^2-U1*U2
         *    X3 = 4*(R^2-Q)
         *    Y3 = 4*(R*(3*Q-2*R^2)-M^4)
         *    Z3 = 2*M*Z
         *  (Note that the paper uses xi = Xi / Zi and yi = Yi / Zi instead.)
         *
         *  This formula has the benefit of being the same for both addition
         *  of distinct points and doubling. However, it breaks down in the
         *  case that either point is infinity, or that y1 = -y2. We handle
         *  these cases in the following ways:
         *
         *    - If b is infinity we simply bail by means of a VERIFY_CHECK.
         *
         *    - If a is infinity, we detect this, and at the end of the
         *      computation replace the result (which will be meaningless,
         *      but we compute to be constant-time) with b.x : b.y : 1.
         *
         *    - If a = -b, we have y1 = -y2, which is a degenerate case.
         *      But here the answer is infinity, so we simply set the
         *      infinity flag of the result, overriding the computed values
         *      without even needing to cmov.
         *
         *    - If y1 = -y2 but x1 != x2, which does occur thanks to certain
         *      properties of our curve (specifically, 1 has nontrivial cube
         *      roots in our field, and the curve equation has no x coefficient)
         *      then the answer is not infinity but also not given by the above
         *      equation. In this case, we cmov in place an alternate expression
         *      for lambda. Specifically (y1 - y2)/(x1 - x2). Where both these
         *      expressions for lambda are defined, they are equal, and can be
         *      obtained from each other by multiplication by (y1 + y2)/(y1 + y2)
         *      then substitution of x^3 + 7 for y^2 (using the curve equation).
         *      For all pairs of nonzero points (a, b) at least one is defined,
         *      so this covers everything.
         */
        fe_sqr(core::ptr::addr_of_mut!(zz), core::ptr::addr_of!((*a).z)); /* z = Z1^2 */

        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!(u1), 1);
        fe_normalize_weak(core::ptr::addr_of_mut!(u1)); /* u1 = U1 = X1*Z2^2 (1) */

        fe_mul(
            core::ptr::addr_of_mut!(u2),
            core::ptr::addr_of!((*b).x),
            core::ptr::addr_of!(zz),
        ); /* u2 = U2 = X2*Z1^2 (1) */

        core::ptr::copy(core::ptr::addr_of!((*a).y), core::ptr::addr_of_mut!(s1), 1);
        fe_normalize_weak(core::ptr::addr_of_mut!(s1)); /* s1 = S1 = Y1*Z2^3 (1) */

        fe_mul(
            core::ptr::addr_of_mut!(s2),
            core::ptr::addr_of!((*b).y),
            core::ptr::addr_of!(zz),
        ); /* s2 = Y2*Z1^2 (1) */
        fe_mul(
            core::ptr::addr_of_mut!(s2),
            core::ptr::addr_of!(s2),
            core::ptr::addr_of!((*a).z),
        ); /* s2 = S2 = Y2*Z1^3 (1) */

        core::ptr::copy(core::ptr::addr_of!(u1), core::ptr::addr_of_mut!(t), 1);
        fe_add(core::ptr::addr_of_mut!(t), core::ptr::addr_of!(u2)); /* t = T = U1+U2 (2) */

        core::ptr::copy(core::ptr::addr_of!(s1), core::ptr::addr_of_mut!(m), 1);
        fe_add(core::ptr::addr_of_mut!(m), core::ptr::addr_of!(s2)); /* m = M = S1+S2 (2) */

        fe_sqr(core::ptr::addr_of_mut!(rr), core::ptr::addr_of!(t)); /* rr = T^2 (1) */
        fe_negate(core::ptr::addr_of_mut!(m_alt), core::ptr::addr_of!(u2), 1); /* Malt = -X2*Z1^2 */
        fe_mul(
            core::ptr::addr_of_mut!(tt),
            core::ptr::addr_of!(u1),
            core::ptr::addr_of!(m_alt),
        ); /* tt = -U1*U2 (2) */
        fe_add(core::ptr::addr_of_mut!(rr), core::ptr::addr_of!(tt)); /* rr = R = T^2-U1*U2 (3) */

        degenerate =
            fe_normalizes_to_zero(core::ptr::addr_of!(m)) & fe_normalizes_to_zero(core::ptr::addr_of!(rr));

        core::ptr::copy(core::ptr::addr_of!(s1), core::ptr::addr_of_mut!(rr_alt), 1);
        fe_mul_int(core::ptr::addr_of_mut!(rr_alt), 2); /* rr = Y1*Z2^3 - Y2*Z1^3 (2) */
        fe_add(core::ptr::addr_of_mut!(m_alt), core::ptr::addr_of!(u1)); /* Malt = X1*Z2^2 - X2*Z1^2 */

        let not_degenerate: i32 = if degenerate != 0 { 0 } else { 1 };
        fe_cmov(
            core::ptr::addr_of_mut!(rr_alt),
            core::ptr::addr_of!(rr),
            not_degenerate,
        );
        fe_cmov(
            core::ptr::addr_of_mut!(m_alt),
            core::ptr::addr_of!(m),
            not_degenerate,
        );

        fe_sqr(core::ptr::addr_of_mut!(n), core::ptr::addr_of!(m_alt)); /* n = Malt^2 (1) */
        fe_mul(
            core::ptr::addr_of_mut!(q),
            core::ptr::addr_of!(n),
            core::ptr::addr_of!(t),
        ); /* q = Q = T*Malt^2 (1) */

        let n_ptr: *mut Fe = core::ptr::addr_of_mut!(n);
        fe_sqr(n_ptr, n_ptr as *const Fe);
        fe_cmov(n_ptr, core::ptr::addr_of!(m), degenerate); /* n = M^3 * Malt (2) */

        fe_sqr(core::ptr::addr_of_mut!(t), core::ptr::addr_of!(rr_alt)); /* t = Ralt^2 (1) */

        fe_mul(
            core::ptr::addr_of_mut!((*r).z),
            core::ptr::addr_of!((*a).z),
            core::ptr::addr_of!(m_alt),
        ); /* r->z = Malt*Z (1) */
        infinity =
            fe_normalizes_to_zero(core::ptr::addr_of!((*r).z)) & (!(*a).infinity);
        fe_mul_int(core::ptr::addr_of_mut!((*r).z), 2); /* r->z = Z3 = 2*Malt*Z (2) */

        let q_ptr: *mut Fe = core::ptr::addr_of_mut!(q);
        fe_negate(q_ptr, q_ptr as *const Fe, 1); /* q = -Q (2) */
        fe_add(core::ptr::addr_of_mut!(t), core::ptr::addr_of!(q)); /* t = Ralt^2-Q (3) */
        fe_normalize_weak(core::ptr::addr_of_mut!(t));

        core::ptr::copy(core::ptr::addr_of!(t), core::ptr::addr_of_mut!((*r).x), 1); /* r->x = Ralt^2-Q (1) */

        fe_mul_int(core::ptr::addr_of_mut!(t), 2); /* t = 2*x3 (2) */
        fe_add(core::ptr::addr_of_mut!(t), core::ptr::addr_of!(q)); /* t = 2*x3 - Q: (4) */
        fe_mul(
            core::ptr::addr_of_mut!(t),
            core::ptr::addr_of!(t),
            core::ptr::addr_of!(rr_alt),
        ); /* t = Ralt*(2*x3 - Q) (1) */
        fe_add(core::ptr::addr_of_mut!(t), core::ptr::addr_of!(n)); /* t = Ralt*(2*x3 - Q) + M^3*Malt (3) */

        fe_negate(
            core::ptr::addr_of_mut!((*r).y),
            core::ptr::addr_of!(t),
            3,
        ); /* r->y = Ralt*(Q - 2x3) - M^3*Malt (4) */
        fe_normalize_weak(core::ptr::addr_of_mut!((*r).y));

        fe_mul_int(core::ptr::addr_of_mut!((*r).x), 4); /* r->x = X3 = 4*(Ralt^2-Q) */
        fe_mul_int(core::ptr::addr_of_mut!((*r).y), 4); /* r->y = Y3 = 4*Ralt*(Q - 2x3) - 4*M^3*Malt (4) */

        fe_cmov(
            core::ptr::addr_of_mut!((*r).x),
            core::ptr::addr_of!((*b).x),
            (*a).infinity,
        );
        fe_cmov(
            core::ptr::addr_of_mut!((*r).y),
            core::ptr::addr_of!((*b).y),
            (*a).infinity,
        );
        fe_cmov(
            core::ptr::addr_of_mut!((*r).z),
            core::ptr::addr_of!(fe_1),
            (*a).infinity,
        );
        (*r).infinity = infinity;
    }
}
