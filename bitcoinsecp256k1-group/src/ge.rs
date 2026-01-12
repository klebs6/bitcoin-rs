// ---------------- [ File: bitcoinsecp256k1-group/src/ge.rs ]
crate::ix!();

/**
  | A group element of the secp256k1 curve,
  | in affine coordinates.
  |
  */
#[derive(Debug,Copy,Clone)]
pub struct Ge {
    pub x:        Fe,
    pub y:        Fe,

    /**
      | whether this represents the point at
      | infinity
      |
      */
    pub infinity: i32,
}

impl Ge {
    pub const fn new() -> Self {
        Self {
            x:        Fe::new(),
            y:        Fe::new(),
            infinity: 0,
        }
    }
}

#[cfg(test)]
mod ge_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_new_initializes_to_zero_and_non_infinity() {
        tracing::info!("Validating Ge::new() initializes to (0,0) with infinity=0.");

        let p: Ge = Ge::new();

        assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);
        assert!(fe_is_zero(core::ptr::addr_of!(p.x)) != 0);
        assert!(fe_is_zero(core::ptr::addr_of!(p.y)) != 0);

        tracing::debug!("Ge::new() should not represent a valid curve point for secp256k1-style curves.");
        assert!(ge_is_valid_var(core::ptr::addr_of!(p)) == 0);
    }
}

#[inline(always)]
pub fn ge_negate_in_place(p: *mut Ge) {
    if ge_is_infinity(p) != 0 {
        return;
    }
    fe_negate(
        ge_y_mut(p),
        ge_y(p),
        1,
    );
}

#[inline(always)]
pub fn ge_x(this: *const Ge) -> *const Fe {
    unsafe { core::ptr::addr_of!((*this).x) }
}

#[inline(always)]
pub fn ge_x_mut(this: *mut Ge) -> *mut Fe {
    unsafe { core::ptr::addr_of_mut!((*this).x) }
}

#[inline(always)]
pub fn ge_y(this: *const Ge) -> *const Fe {
    unsafe { core::ptr::addr_of!((*this).y) }
}

#[inline(always)]
pub fn ge_y_mut(this: *mut Ge) -> *mut Fe {
    unsafe { core::ptr::addr_of_mut!((*this).y) }
}

#[inline(always)]
pub fn ge_infinity(this: *const Ge) -> *const i32 {
    unsafe { core::ptr::addr_of!((*this).infinity) }
}

#[inline(always)]
pub fn ge_infinity_mut(this: *mut Ge) -> *mut i32 {
    unsafe { core::ptr::addr_of_mut!((*this).infinity) }
}
