// ---------------- [ File: bitcoinsecp256k1-ecmult/src/config.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/basic-config.h]
#[cfg(feature="secp256k1-use-basic-config")] 
pub const ECMULT_WINDOW_SIZE:   usize = 15;

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/ecmult_impl.h]

/// The number of entries a table with precomputed multiples needs to have.
///
#[macro_export]
macro_rules! ecmult_table_size {
    ($w:expr) => {
        (1usize << (($w as usize) - 2usize))
    };
}

/// The number of objects allocated on the scratch space for ecmult_multi algorithms
///
pub const PIPPENGER_SCRATCH_OBJECTS:   usize = 6;
pub const STRAUSS_SCRATCH_OBJECTS:     usize = 6;
pub const PIPPENGER_MAX_BUCKET_WINDOW: usize = 12;

/// Minimum number of points for which pippenger_wnaf is faster than strauss wnaf
///
pub const ECMULT_PIPPENGER_THRESHOLD:  usize = 88;
pub const ECMULT_MAX_POINTS_PER_BATCH: usize = 5000000;

lazy_static!{
    pub static ref ECMULT_CONTEXT_PREALLOCATED_SIZE: usize = {
        round_to_align!(size_of::<GeStorage>() * ecmult_table_size!(WINDOW_G))
            + round_to_align!(size_of::<GeStorage>() * ecmult_table_size!(WINDOW_G))
    };
}
