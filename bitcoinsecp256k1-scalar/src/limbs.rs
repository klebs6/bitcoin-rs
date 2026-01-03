// ---------------- [ File: bitcoinsecp256k1-scalar/src/limbs.rs ]
crate::ix!();

/**
  | Limbs of the secp256k1 order.
  |
  */
#[cfg(WIDEMUL_INT128)] pub const N_0: u64 = 0xBFD25E8CD0364141;
#[cfg(WIDEMUL_INT128)] pub const N_1: u64 = 0xBAAEDCE6AF48A03B;
#[cfg(WIDEMUL_INT128)] pub const N_2: u64 = 0xFFFFFFFFFFFFFFFE;
#[cfg(WIDEMUL_INT128)] pub const N_3: u64 = 0xFFFFFFFFFFFFFFFF;

/**
  | Limbs of 2^256 minus the secp256k1 order.
  |
  */
#[cfg(WIDEMUL_INT128)] pub const N_C_0: u64 = !N_0 + 1;
#[cfg(WIDEMUL_INT128)] pub const N_C_1: u64 = !N_1;
#[cfg(WIDEMUL_INT128)] pub const N_C_2: u64 = 1;

/**
  | Limbs of half the secp256k1 order.
  |
  */
#[cfg(WIDEMUL_INT128)] pub const N_H_0: u64 = 0xDFE92F46681B20A0;
#[cfg(WIDEMUL_INT128)] pub const N_H_1: u64 = 0x5D576E7357A4501D;
#[cfg(WIDEMUL_INT128)] pub const N_H_2: u64 = 0xFFFFFFFFFFFFFFFF;
#[cfg(WIDEMUL_INT128)] pub const N_H_3: u64 = 0x7FFFFFFFFFFFFFFF;

//-----------------------------

/**
  | Limbs of the secp256k1 order.
  |
  */
#[cfg(WIDEMUL_INT64)] pub const N_0: u32 = 0xD0364141;
#[cfg(WIDEMUL_INT64)] pub const N_1: u32 = 0xBFD25E8C;
#[cfg(WIDEMUL_INT64)] pub const N_2: u32 = 0xAF48A03B;
#[cfg(WIDEMUL_INT64)] pub const N_3: u32 = 0xBAAEDCE6;
#[cfg(WIDEMUL_INT64)] pub const N_4: u32 = 0xFFFFFFFE;
#[cfg(WIDEMUL_INT64)] pub const N_5: u32 = 0xFFFFFFFF;
#[cfg(WIDEMUL_INT64)] pub const N_6: u32 = 0xFFFFFFFF;
#[cfg(WIDEMUL_INT64)] pub const N_7: u32 = 0xFFFFFFFF;

/**
  | Limbs of 2^256 minus the secp256k1 order.
  |
  */
#[cfg(WIDEMUL_INT64)] pub const N_C_0: u32 = !N_0 + 1;
#[cfg(WIDEMUL_INT64)] pub const N_C_1: u32 = !N_1;
#[cfg(WIDEMUL_INT64)] pub const N_C_2: u32 = !N_2;
#[cfg(WIDEMUL_INT64)] pub const N_C_3: u32 = !N_3;
#[cfg(WIDEMUL_INT64)] pub const N_C_4: u32 = 1;

/**
  | Limbs of half the secp256k1 order.
  |
  */
#[cfg(WIDEMUL_INT64)] pub const N_H_0: u32 = 0x681B20A0;
#[cfg(WIDEMUL_INT64)] pub const N_H_1: u32 = 0xDFE92F46;
#[cfg(WIDEMUL_INT64)] pub const N_H_2: u32 = 0x57A4501D;
#[cfg(WIDEMUL_INT64)] pub const N_H_3: u32 = 0x5D576E73;
#[cfg(WIDEMUL_INT64)] pub const N_H_4: u32 = 0xFFFFFFFF;
#[cfg(WIDEMUL_INT64)] pub const N_H_5: u32 = 0xFFFFFFFF;
#[cfg(WIDEMUL_INT64)] pub const N_H_6: u32 = 0xFFFFFFFF;
#[cfg(WIDEMUL_INT64)] pub const N_H_7: u32 = 0x7FFFFFFF;
