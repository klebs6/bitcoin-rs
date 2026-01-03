// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_get_b32.rs ]
crate::ix!();

/**
  | Convert a scalar to a byte array.
  |
  */
#[cfg(WIDEMUL_INT128)]
pub fn scalar_get_b32(
        bin: *mut u8,
        a:   *const Scalar)  {
    
    todo!();
        /*
            bin[0] = a->d[3] >> 56; bin[1] = a->d[3] >> 48; bin[2] = a->d[3] >> 40; bin[3] = a->d[3] >> 32; bin[4] = a->d[3] >> 24; bin[5] = a->d[3] >> 16; bin[6] = a->d[3] >> 8; bin[7] = a->d[3];
        bin[8] = a->d[2] >> 56; bin[9] = a->d[2] >> 48; bin[10] = a->d[2] >> 40; bin[11] = a->d[2] >> 32; bin[12] = a->d[2] >> 24; bin[13] = a->d[2] >> 16; bin[14] = a->d[2] >> 8; bin[15] = a->d[2];
        bin[16] = a->d[1] >> 56; bin[17] = a->d[1] >> 48; bin[18] = a->d[1] >> 40; bin[19] = a->d[1] >> 32; bin[20] = a->d[1] >> 24; bin[21] = a->d[1] >> 16; bin[22] = a->d[1] >> 8; bin[23] = a->d[1];
        bin[24] = a->d[0] >> 56; bin[25] = a->d[0] >> 48; bin[26] = a->d[0] >> 40; bin[27] = a->d[0] >> 32; bin[28] = a->d[0] >> 24; bin[29] = a->d[0] >> 16; bin[30] = a->d[0] >> 8; bin[31] = a->d[0];
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_get_b32(
        bin: *mut u8,
        a:   *const Scalar)  {
    
    todo!();
        /*
            bin[0] = a->d[7] >> 24; bin[1] = a->d[7] >> 16; bin[2] = a->d[7] >> 8; bin[3] = a->d[7];
        bin[4] = a->d[6] >> 24; bin[5] = a->d[6] >> 16; bin[6] = a->d[6] >> 8; bin[7] = a->d[6];
        bin[8] = a->d[5] >> 24; bin[9] = a->d[5] >> 16; bin[10] = a->d[5] >> 8; bin[11] = a->d[5];
        bin[12] = a->d[4] >> 24; bin[13] = a->d[4] >> 16; bin[14] = a->d[4] >> 8; bin[15] = a->d[4];
        bin[16] = a->d[3] >> 24; bin[17] = a->d[3] >> 16; bin[18] = a->d[3] >> 8; bin[19] = a->d[3];
        bin[20] = a->d[2] >> 24; bin[21] = a->d[2] >> 16; bin[22] = a->d[2] >> 8; bin[23] = a->d[2];
        bin[24] = a->d[1] >> 24; bin[25] = a->d[1] >> 16; bin[26] = a->d[1] >> 8; bin[27] = a->d[1];
        bin[28] = a->d[0] >> 24; bin[29] = a->d[0] >> 16; bin[30] = a->d[0] >> 8; bin[31] = a->d[0];
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_get_b32(
        bin: *mut u8,
        a:   *const Scalar)  {
    
    todo!();
        /*
            memset(bin, 0, 32);
        bin[28] = *a >> 24; bin[29] = *a >> 16; bin[30] = *a >> 8; bin[31] = *a;
        */
}
