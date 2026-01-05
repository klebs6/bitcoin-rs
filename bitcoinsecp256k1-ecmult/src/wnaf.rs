// ---------------- [ File: bitcoinsecp256k1-ecmult/src/wnaf.rs ]
crate::ix!();

pub const WNAF_BITS: usize = 128;

#[macro_export]
macro_rules! wnaf_size_bits {
    ($bits:ident, $w:ident) => {
        /*
                (((bits) + (w) - 1) / (w))
        */
    }
}

#[macro_export]
macro_rules! wnaf_size {
    ($w:ident) => {
        /*
                WNAF_SIZE_BITS(WNAF_BITS, w)
        */
    }
}
