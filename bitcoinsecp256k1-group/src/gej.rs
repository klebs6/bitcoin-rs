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
