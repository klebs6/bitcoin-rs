// ---------------- [ File: bitcoinsecp256k1-ecmult/src/wnaf.rs ]
crate::ix!();

pub const WNAF_BITS: usize = 128;

#[macro_export]
macro_rules! wnaf_size_bits {
    ($bits:expr, $w:expr) => {
        ((($bits) + (($w) as usize) - 1usize) / (($w) as usize))
    };
}

#[macro_export]
macro_rules! wnaf_size {
    ($w:expr) => {
        wnaf_size_bits!(WNAF_BITS, ($w))
    };
}
