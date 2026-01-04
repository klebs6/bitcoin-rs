// ---------------- [ File: bitcoinsecp256k1-group/src/gej.rs ]
crate::ix!();

/// A group element of the secp256k1 curve,
/// in jacobian coordinates.
/// 
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
