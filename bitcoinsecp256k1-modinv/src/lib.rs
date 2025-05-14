// ---------------- [ File: bitcoinsecp256k1-modinv/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{modinv32}

/**
  | modinv64 requires 128-bit wide multiplication
  | support
  |
  */
#[cfg(SECP256K1_WIDEMUL_INT128)]
x!{modinv64}
