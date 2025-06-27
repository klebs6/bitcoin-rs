// ---------------- [ File: bitcoin-ripemd/src/r_star.rs ]
crate::ix!();

#[inline]
pub fn ripemd160_r11(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f1(b, *c, d);
    trace!(target: "ripemd160::r", "R11 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0x0000_0000, r);
    trace!(target: "ripemd160::r", "R11 finish a={:#010x} c={:#010x}", *a, *c);
}

#[inline]
pub fn ripemd160_r21(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f2(b, *c, d);
    trace!(target: "ripemd160::r", "R21 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0x5A82_7999, r);
    trace!(target: "ripemd160::r", "R21 finish a={:#010x} c={:#010x}", *a, *c);
}

#[inline]
pub fn ripemd160_r31(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f3(b, *c, d);
    trace!(target: "ripemd160::r", "R31 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0x6ED9_EBA1, r);
    trace!(target: "ripemd160::r", "R31 finish a={:#010x} c={:#010x}", *a, *c);
}

#[inline]
pub fn ripemd160_r41(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f4(b, *c, d);
    trace!(target: "ripemd160::r", "R41 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0x8F1B_BCDC, r);
    trace!(target: "ripemd160::r", "R41 finish a={:#010x} c={:#010x}", *a, *c);
}

#[inline]
pub fn ripemd160_r51(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f5(b, *c, d);
    trace!(target: "ripemd160::r", "R51 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0xA953_FD4E, r);
    trace!(target: "ripemd160::r", "R51 finish a={:#010x} c={:#010x}", *a, *c);
}

#[inline]
pub fn ripemd160_r12(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f5(b, *c, d);
    trace!(target: "ripemd160::r", "R12 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0x50A2_8BE6, r);
    trace!(target: "ripemd160::r", "R12 finish a={:#010x} c={:#010x}", *a, *c);
}

#[inline]
pub fn ripemd160_r22(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f4(b, *c, d);
    trace!(target: "ripemd160::r", "R22 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0x5C4D_D124, r);
    trace!(target: "ripemd160::r", "R22 finish a={:#010x} c={:#010x}", *a, *c);
}

#[inline]
pub fn ripemd160_r32(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f3(b, *c, d);
    trace!(target: "ripemd160::r", "R32 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0x6D70_3EF3, r);
    trace!(target: "ripemd160::r", "R32 finish a={:#010x} c={:#010x}", *a, *c);
}

#[inline]
pub fn ripemd160_r42(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f2(b, *c, d);
    trace!(target: "ripemd160::r", "R42 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0x7A6D_76E9, r);
    trace!(target: "ripemd160::r", "R42 finish a={:#010x} c={:#010x}", *a, *c);
}

#[inline]
pub fn ripemd160_r52(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
) {
    let f = ripemd160_f1(b, *c, d);
    trace!(target: "ripemd160::r", "R52 start  a={:#010x} c={:#010x}", *a, *c);
    ripemd160_round(a, b, c, d, e, f, x, 0x0000_0000, r);
    trace!(target: "ripemd160::r", "R52 finish a={:#010x} c={:#010x}", *a, *c);
}

#[cfg(test)]
mod spec_r_star {
    use super::*;
    use crate::{
        ripemd160_f1, ripemd160_f2, ripemd160_f3, ripemd160_f4, ripemd160_f5, ripemd160_round,
    };

    /// Generic helper that validates any R‑function
    /// against an independently computed reference
    /// using `ripemd160_round`.
    #[allow(clippy::too_many_arguments)]
    fn check_r<F>(
        mut a: u32,
        b: u32,
        mut c: u32,
        d: u32,
        e: u32,
        x: u32,
        k: u32,
        r: i32,
        f: fn(u32, u32, u32) -> u32,
        call_r: F,
    ) where
        F: Fn(&mut u32, u32, &mut u32, u32, u32, u32, i32),
    {
        // Reference transition.
        let mut a_ref = a;
        let mut c_ref = c;
        let f_val = f(b, c_ref, d);
        ripemd160_round(&mut a_ref, b, &mut c_ref, d, e, f_val, x, k, r);

        // DUT transition.
        call_r(&mut a, b, &mut c, d, e, x, r);

        assert_eq!(a, a_ref, "a mismatch");
        assert_eq!(c, c_ref, "c mismatch");
    }

    #[traced_test]
    fn r_wrappers_match_reference() {
        // Fixed but non‑trivial state & inputs.
        let a0 = 0x6745_2301;
        let b0 = 0xefcd_ab89;
        let c0 = 0x98ba_dcfe;
        let d0 = 0x1032_5476;
        let e0 = 0xc3d2_e1f0;
        let x = 0x0bad_cafe;
        let r = 11; // rotation count

        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0x0000_0000,
            r,
            ripemd160_f1,
            ripemd160_r11,
        );
        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0x5A82_7999,
            r,
            ripemd160_f2,
            ripemd160_r21,
        );
        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0x6ED9_EBA1,
            r,
            ripemd160_f3,
            ripemd160_r31,
        );
        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0x8F1B_BCDC,
            r,
            ripemd160_f4,
            ripemd160_r41,
        );
        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0xA953_FD4E,
            r,
            ripemd160_f5,
            ripemd160_r51,
        );
        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0x50A2_8BE6,
            r,
            ripemd160_f5,
            ripemd160_r12,
        );
        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0x5C4D_D124,
            r,
            ripemd160_f4,
            ripemd160_r22,
        );
        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0x6D70_3EF3,
            r,
            ripemd160_f3,
            ripemd160_r32,
        );
        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0x7A6D_76E9,
            r,
            ripemd160_f2,
            ripemd160_r42,
        );
        check_r(
            a0,
            b0,
            c0,
            d0,
            e0,
            x,
            0x0000_0000,
            r,
            ripemd160_f1,
            ripemd160_r52,
        );
    }
}
