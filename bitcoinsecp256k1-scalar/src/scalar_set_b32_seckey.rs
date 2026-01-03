// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_set_b32_seckey.rs ]
crate::ix!();

/**
  | Set a scalar from a big endian byte array
  | and returns 1 if it is a valid seckey and
  | 0 otherwise.
  |
  */
#[cfg(WIDEMUL_INT128)]
pub fn scalar_set_b32_seckey(
        r:   *mut Scalar,
        bin: *const u8) -> i32 {
    
    todo!();
        /*
            int overflow;
        scalar_set_b32(r, bin, &overflow);
        return (!overflow) & (!scalar_is_zero(r));
        */
}
