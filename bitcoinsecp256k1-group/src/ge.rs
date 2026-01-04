// ---------------- [ File: bitcoinsecp256k1-group/src/ge.rs ]
crate::ix!();

/**
  | A group element of the secp256k1 curve,
  | in affine coordinates.
  |
  */
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
