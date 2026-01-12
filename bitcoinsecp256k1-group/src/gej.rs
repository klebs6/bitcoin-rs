// ---------------- [ File: bitcoinsecp256k1-group/src/gej.rs ]
crate::ix!();

/// A group element of the secp256k1 curve,
/// in jacobian coordinates.
/// 
#[derive(Debug,Copy,Clone)]
pub struct Gej {

    /// actual X: x/z^2
    /// 
    pub x:        Fe,

    /// actual Y: y/z^3
    /// 
    pub y:        Fe,

    pub z:        Fe,

    /// whether this represents the point at
    /// infinity
    /// 
    pub infinity: i32,
}

impl Gej {
    pub const fn new() -> Self {
        Self {
            x: Fe::new(),
            y: Fe::new(),
            z: Fe::new(),
            infinity: 0,
        }
    }
}

#[cfg(test)]
mod gej_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_new_initializes_to_zero_and_non_infinity() {
        tracing::info!("Validating Gej::new() initializes x/y/z to zero and infinity=0.");

        let p: Gej = Gej::new();

        assert!(gej_is_infinity(core::ptr::addr_of!(p)) == 0);
        assert!(fe_is_zero(core::ptr::addr_of!(p.x)) != 0);
        assert!(fe_is_zero(core::ptr::addr_of!(p.y)) != 0);
        assert!(fe_is_zero(core::ptr::addr_of!(p.z)) != 0);
    }
}

#[inline(always)]
pub fn gej_x(this: *const Gej) -> *const Fe {
    unsafe { core::ptr::addr_of!((*this).x) }
}

#[inline(always)]
pub fn gej_x_mut(this: *mut Gej) -> *mut Fe {
    unsafe { core::ptr::addr_of_mut!((*this).x) }
}

#[inline(always)]
pub fn gej_y(this: *const Gej) -> *const Fe {
    unsafe { core::ptr::addr_of!((*this).y) }
}

#[inline(always)]
pub fn gej_y_mut(this: *mut Gej) -> *mut Fe {
    unsafe { core::ptr::addr_of_mut!((*this).y) }
}

#[inline(always)]
pub fn gej_z(this: *const Gej) -> *const Fe {
    unsafe { core::ptr::addr_of!((*this).z) }
}

#[inline(always)]
pub fn gej_z_mut(this: *mut Gej) -> *mut Fe {
    unsafe { core::ptr::addr_of_mut!((*this).z) }
}

#[inline(always)]
pub fn gej_infinity(this: *const Gej) -> *const i32 {
    unsafe { core::ptr::addr_of!((*this).infinity) }
}

#[inline(always)]
pub fn gej_infinity_mut(this: *mut Gej) -> *mut i32 {
    unsafe { core::ptr::addr_of_mut!((*this).infinity) }
}

#[inline(always)]
pub fn gej_negate_in_place(p: *mut Gej) {
    if gej_is_infinity(p) != 0 {
        return;
    }
    fe_negate(
        gej_y_mut(p),
        gej_y(p),
        1,
    );
}
