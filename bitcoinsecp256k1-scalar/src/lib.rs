// ---------------- [ File: bitcoinsecp256k1-scalar/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

///Need to select a scalar format
#[cfg(SECP256K1_WIDEMUL_INT128)]
x!{scalar_4x64}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
x!{scalar_low}
x!{scalar}

#[cfg(SECP256K1_WIDEMUL_INT64)]
x!{scalar_8x32}

x!{scalar_split_lambda}
