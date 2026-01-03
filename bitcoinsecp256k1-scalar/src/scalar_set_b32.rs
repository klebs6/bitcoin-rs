// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_set_b32.rs ]
crate::ix!();

/// Set a scalar from a big endian byte array. The
/// scalar will be reduced modulo group order `n`.
/// 
/// In:      bin:        pointer to a 32-byte array.
/// 
/// Out:     r:          scalar to be set.
/// 
///          overflow:   non-zero if the scalar was
///          bigger or equal to `n` before
///          reduction, zero otherwise (can be
///          NULL).
///
#[cfg(WIDEMUL_INT128)]
pub fn scalar_set_b32(
        r:        *mut Scalar,
        b32:      *const u8,
        overflow: *mut i32)  {
    
    todo!();
        /*
            int over;
        r->d[0] = (uint64_t)b32[31] | (uint64_t)b32[30] << 8 | (uint64_t)b32[29] << 16 | (uint64_t)b32[28] << 24 | (uint64_t)b32[27] << 32 | (uint64_t)b32[26] << 40 | (uint64_t)b32[25] << 48 | (uint64_t)b32[24] << 56;
        r->d[1] = (uint64_t)b32[23] | (uint64_t)b32[22] << 8 | (uint64_t)b32[21] << 16 | (uint64_t)b32[20] << 24 | (uint64_t)b32[19] << 32 | (uint64_t)b32[18] << 40 | (uint64_t)b32[17] << 48 | (uint64_t)b32[16] << 56;
        r->d[2] = (uint64_t)b32[15] | (uint64_t)b32[14] << 8 | (uint64_t)b32[13] << 16 | (uint64_t)b32[12] << 24 | (uint64_t)b32[11] << 32 | (uint64_t)b32[10] << 40 | (uint64_t)b32[9] << 48 | (uint64_t)b32[8] << 56;
        r->d[3] = (uint64_t)b32[7] | (uint64_t)b32[6] << 8 | (uint64_t)b32[5] << 16 | (uint64_t)b32[4] << 24 | (uint64_t)b32[3] << 32 | (uint64_t)b32[2] << 40 | (uint64_t)b32[1] << 48 | (uint64_t)b32[0] << 56;
        over = scalar_reduce(r, scalar_check_overflow(r));
        if (overflow) {
            *overflow = over;
        }
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_set_b32(
        r:        *mut Scalar,
        b32:      *const u8,
        overflow: *mut i32)  {
    
    todo!();
        /*
            int over;
        r->d[0] = (uint32_t)b32[31] | (uint32_t)b32[30] << 8 | (uint32_t)b32[29] << 16 | (uint32_t)b32[28] << 24;
        r->d[1] = (uint32_t)b32[27] | (uint32_t)b32[26] << 8 | (uint32_t)b32[25] << 16 | (uint32_t)b32[24] << 24;
        r->d[2] = (uint32_t)b32[23] | (uint32_t)b32[22] << 8 | (uint32_t)b32[21] << 16 | (uint32_t)b32[20] << 24;
        r->d[3] = (uint32_t)b32[19] | (uint32_t)b32[18] << 8 | (uint32_t)b32[17] << 16 | (uint32_t)b32[16] << 24;
        r->d[4] = (uint32_t)b32[15] | (uint32_t)b32[14] << 8 | (uint32_t)b32[13] << 16 | (uint32_t)b32[12] << 24;
        r->d[5] = (uint32_t)b32[11] | (uint32_t)b32[10] << 8 | (uint32_t)b32[9] << 16 | (uint32_t)b32[8] << 24;
        r->d[6] = (uint32_t)b32[7] | (uint32_t)b32[6] << 8 | (uint32_t)b32[5] << 16 | (uint32_t)b32[4] << 24;
        r->d[7] = (uint32_t)b32[3] | (uint32_t)b32[2] << 8 | (uint32_t)b32[1] << 16 | (uint32_t)b32[0] << 24;
        over = scalar_reduce(r, scalar_check_overflow(r));
        if (overflow) {
            *overflow = over;
        }
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_set_b32(
        r:        *mut Scalar,
        b32:      *const u8,
        overflow: *mut i32)  {
    
    todo!();
        /*
            int i;
        int over = 0;
        *r = 0;
        for (i = 0; i < 32; i++) {
            *r = (*r * 0x100) + b32[i];
            if (*r >= EXHAUSTIVE_TEST_ORDER) {
                over = 1;
                *r %= EXHAUSTIVE_TEST_ORDER;
            }
        }
        if (overflow) *overflow = over;
        */
}


