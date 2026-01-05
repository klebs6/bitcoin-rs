// ---------------- [ File: bitcoinsecp256k1-group/src/secp256k1_group_exhaustive_test_support.rs ]
#![cfg(test)]

crate::ix!();

pub(crate) fn fe_int(value: i32) -> Fe {
    unsafe {
        let mut out: Fe = core::mem::zeroed();
        fe_set_int(core::ptr::addr_of_mut!(out), value);
        out
    }
}

pub(crate) fn fe_is_zero_bool(a: &Fe) -> bool {
    unsafe { fe_is_zero(a as *const Fe) != 0 }
}

pub(crate) fn fe_is_one_bool(a: &Fe) -> bool {
    unsafe {
        let one: Fe = fe_int(1);
        fe_equal_var(a as *const Fe, core::ptr::addr_of!(one)) != 0
    }
}

pub(crate) fn fe_eq(a: &Fe, b: &Fe) -> bool {
    unsafe { fe_equal_var(a as *const Fe, b as *const Fe) != 0 }
}

pub(crate) fn ge_is_infinity_bool(a: &Ge) -> bool {
    unsafe { ge_is_infinity(a as *const Ge) != 0 }
}

pub(crate) fn ge_eq(a: &Ge, b: &Ge) -> bool {
    unsafe {
        let a_inf: bool = ge_is_infinity(a as *const Ge) != 0;
        let b_inf: bool = ge_is_infinity(b as *const Ge) != 0;

        if a_inf || b_inf {
            return a_inf == b_inf;
        }

        (fe_equal_var(core::ptr::addr_of!(a.x), core::ptr::addr_of!(b.x)) != 0)
            && (fe_equal_var(core::ptr::addr_of!(a.y), core::ptr::addr_of!(b.y)) != 0)
    }
}

pub(crate) fn gej_is_infinity_bool(a: &Gej) -> bool {
    unsafe { gej_is_infinity(a as *const Gej) != 0 }
}

pub(crate) fn gej_affine_eq(a: &Gej, b: &Gej) -> bool {
    unsafe {
        let a_inf: bool = gej_is_infinity(a as *const Gej) != 0;
        let b_inf: bool = gej_is_infinity(b as *const Gej) != 0;

        if a_inf || b_inf {
            return a_inf == b_inf;
        }

        let mut z1z1: Fe = core::mem::zeroed();
        let mut z2z2: Fe = core::mem::zeroed();
        let mut u1: Fe = core::mem::zeroed();
        let mut u2: Fe = core::mem::zeroed();

        let mut z1z1z1: Fe = core::mem::zeroed();
        let mut z2z2z2: Fe = core::mem::zeroed();
        let mut s1: Fe = core::mem::zeroed();
        let mut s2: Fe = core::mem::zeroed();

        fe_sqr(core::ptr::addr_of_mut!(z1z1), core::ptr::addr_of!(a.z));
        fe_sqr(core::ptr::addr_of_mut!(z2z2), core::ptr::addr_of!(b.z));

        fe_mul(
            core::ptr::addr_of_mut!(u1),
            core::ptr::addr_of!(a.x),
            core::ptr::addr_of!(z2z2),
        );
        fe_mul(
            core::ptr::addr_of_mut!(u2),
            core::ptr::addr_of!(b.x),
            core::ptr::addr_of!(z1z1),
        );

        fe_mul(
            core::ptr::addr_of_mut!(z1z1z1),
            core::ptr::addr_of!(z1z1),
            core::ptr::addr_of!(a.z),
        );
        fe_mul(
            core::ptr::addr_of_mut!(z2z2z2),
            core::ptr::addr_of!(z2z2),
            core::ptr::addr_of!(b.z),
        );

        fe_mul(
            core::ptr::addr_of_mut!(s1),
            core::ptr::addr_of!(a.y),
            core::ptr::addr_of!(z2z2z2),
        );
        fe_mul(
            core::ptr::addr_of_mut!(s2),
            core::ptr::addr_of!(b.y),
            core::ptr::addr_of!(z1z1z1),
        );

        (fe_equal_var(core::ptr::addr_of!(u1), core::ptr::addr_of!(u2)) != 0)
            && (fe_equal_var(core::ptr::addr_of!(s1), core::ptr::addr_of!(s2)) != 0)
    }
}

pub(crate) fn gej_from_ge(a: &Ge) -> Gej {
    unsafe {
        let mut r: Gej = core::mem::zeroed();
        gej_set_ge(core::ptr::addr_of_mut!(r), a as *const Ge);
        r
    }
}

pub(crate) fn ge_from_gej_via_set_gej_var(a: &Gej) -> Ge {
    unsafe {
        let mut tmp: Gej = core::ptr::read(a as *const Gej);
        let mut out: Ge = core::mem::zeroed();
        ge_set_gej_var(core::ptr::addr_of_mut!(out), core::ptr::addr_of_mut!(tmp));
        out
    }
}

pub(crate) fn ge_negate_affine(a: &Ge) -> Ge {
    unsafe {
        let mut r: Ge = core::mem::zeroed();
        ge_neg(core::ptr::addr_of_mut!(r), a as *const Ge);
        r
    }
}

pub(crate) fn gej_negate_jacobian(a: &Gej) -> Gej {
    unsafe {
        let mut r: Gej = core::mem::zeroed();
        gej_neg(core::ptr::addr_of_mut!(r), a as *const Gej);
        r
    }
}

pub(crate) fn gej_add_var_result(a: &Gej, b: &Gej) -> Gej {
    unsafe {
        let mut r: Gej = core::mem::zeroed();
        gej_add_var(
            core::ptr::addr_of_mut!(r),
            a as *const Gej,
            b as *const Gej,
            core::ptr::null_mut(),
        );
        r
    }
}

pub(crate) fn gej_add_ge_var_result(a: &Gej, b: &Ge) -> Gej {
    unsafe {
        let mut r: Gej = core::mem::zeroed();
        gej_add_ge_var(
            core::ptr::addr_of_mut!(r),
            a as *const Gej,
            b as *const Ge,
            core::ptr::null_mut(),
        );
        r
    }
}

pub(crate) fn gej_double_result(a: &Gej) -> Gej {
    unsafe {
        let mut r: Gej = core::mem::zeroed();
        gej_double(core::ptr::addr_of_mut!(r), a as *const Gej);
        r
    }
}

pub(crate) fn gej_set_infinity_value() -> Gej {
    unsafe {
        let mut r: Gej = core::mem::zeroed();
        gej_set_infinity(core::ptr::addr_of_mut!(r));
        r
    }
}

pub(crate) fn ge_set_infinity_value() -> Ge {
    unsafe {
        let mut r: Ge = core::mem::zeroed();
        ge_set_infinity(core::ptr::addr_of_mut!(r));
        r
    }
}

pub(crate) fn fe_inv_var_value(a: &Fe) -> Fe {
    unsafe {
        let mut out: Fe = core::mem::zeroed();
        fe_inv_var(core::ptr::addr_of_mut!(out), a as *const Fe);
        out
    }
}

pub(crate) fn fe_mul_value(a: &Fe, b: &Fe) -> Fe {
    unsafe {
        let mut out: Fe = core::mem::zeroed();
        fe_mul(core::ptr::addr_of_mut!(out), a as *const Fe, b as *const Fe);
        out
    }
}

pub(crate) fn fe_sqr_value(a: &Fe) -> Fe {
    unsafe {
        let mut out: Fe = core::mem::zeroed();
        fe_sqr(core::ptr::addr_of_mut!(out), a as *const Fe);
        out
    }
}

pub(crate) fn generate_gej_multiples_from_affine<const N: usize>(g: &Ge) -> [Gej; N] {
    unsafe {
        let mut points: [Gej; N] = core::array::from_fn(|_| core::mem::zeroed());

        let mut acc: Gej = core::mem::zeroed();
        gej_set_infinity(core::ptr::addr_of_mut!(acc));

        let mut i: usize = 0;
        while i < N {
            core::ptr::copy(
                core::ptr::addr_of!(acc),
                core::ptr::addr_of_mut!(points[i]),
                1,
            );

            let mut next: Gej = core::mem::zeroed();
            gej_add_ge_var(
                core::ptr::addr_of_mut!(next),
                core::ptr::addr_of!(acc),
                g as *const Ge,
                core::ptr::null_mut(),
            );

            acc = next;
            i += 1;
        }

        points
    }
}
