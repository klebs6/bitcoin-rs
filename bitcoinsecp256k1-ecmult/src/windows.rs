// ---------------- [ File: bitcoinsecp256k1-ecmult/src/windows.rs ]
crate::ix!();

/// We need to lower these values for exhaustive tests because the tables cannot have infinities in
/// them (this breaks the affine-isomorphism stuff which tracks z-ratios)
///
#[cfg(EXHAUSTIVE_TEST_ORDER)]
lazy_static!{
    /*
    #  if EXHAUSTIVE_TEST_ORDER > 128
    #    define WINDOW_A 5
    #    define WINDOW_G 8
    #  elif EXHAUSTIVE_TEST_ORDER > 8
    #    define WINDOW_A 4
    #    define WINDOW_G 4
    #  else
    #    define WINDOW_A 2
    #    define WINDOW_G 2
    #  endif
    */
}

/// optimal for 128-bit and 256-bit exponents.
#[cfg(not(EXHAUSTIVE_TEST_ORDER))]
pub const WINDOW_A: usize = 5;

/// Larger values for ECMULT_WINDOW_SIZE result in possibly better performance at the cost of an
/// exponentially larger precomputed table. The exact table size is
///
///```ignore
///      (1 << (WINDOW_G - 2)) * sizeof(ge_storage)  bytes,
///```
///
///  where sizeof(ge_storage) is typically 64 bytes but can be larger due to platform-specific
///  padding and alignment.
///
///  Two tables of this size are used (due to the endomorphism optimization).
///
#[cfg(not(EXHAUSTIVE_TEST_ORDER))]
pub const WINDOW_G: usize = ECMULT_WINDOW_SIZE;

/// Noone will ever need more than a window size of 24. 
///
/// The code might be correct for larger values of ECMULT_WINDOW_SIZE but this is not not tested.
///
/// The following limitations are known, and there are probably more:
///
/// If WINDOW_G > 27 and size_t has 32 bits, then the code is incorrect because the size of the
/// memory object that we allocate (in bytes) will not fit in a size_t.
///
/// If WINDOW_G > 31 and int has 32 bits, then the code is incorrect because certain expressions
/// will overflow.
///
///  error msg: "Set ECMULT_WINDOW_SIZE to an integer in range [2..24]"
const_assert!{
    2 <= ECMULT_WINDOW_SIZE &&
        25 > ECMULT_WINDOW_SIZE
}
