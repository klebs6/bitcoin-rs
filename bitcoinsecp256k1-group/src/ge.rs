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
