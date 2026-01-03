// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar.rs ]
crate::ix!();

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
#[cfg(WIDEMUL_INT64)]
pub struct Scalar {
    pub d: [u32; 8],
}

#[cfg(WIDEMUL_INT64)]
impl Scalar {
    pub const fn new() -> Self {
        Self {
            d: [0; 8],
        }
    }
}

//-----------------------

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
#[cfg(WIDEMUL_INT128)]
pub struct Scalar {
    pub d: [u64; 4],
}

#[cfg(WIDEMUL_INT128)]
impl Scalar {

    pub const fn new() -> Self {
        Self {
            d: [0; 4],
        }
    }
}

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub type Scalar = u32;
