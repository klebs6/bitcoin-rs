// ---------------- [ File: bitcoinsecp256k1-modinv/src/lib.rs ]
/// modinv64 requires 128-bit wide multiplication support
#[cfg(SECP256K1_WIDEMUL_INT128)]
pub use bitcoinsecp256k1_modinv64::*;
pub use bitcoinsecp256k1_modinv32::*;
