// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_mul_shift.rs ]
crate::ix!();

/// Multiply a and b (without taking the modulus!), divide by 2**shift, and round to the nearest
/// integer. Shift must be at least 256.
/// 
#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_mul_shift_var(
        r:     *mut Scalar,
        a:     *const Scalar,
        b:     *const Scalar,
        shift: u32)  {
    
    todo!();
        /*
            uint64_t l[8];
        unsigned int shiftlimbs;
        unsigned int shiftlow;
        unsigned int shifthigh;
        VERIFY_CHECK(shift >= 256);
        scalar_mul_512(l, a, b);
        shiftlimbs = shift >> 6;
        shiftlow = shift & 0x3F;
        shifthigh = 64 - shiftlow;
        r->d[0] = shift < 512 ? (l[0 + shiftlimbs] >> shiftlow | (shift < 448 && shiftlow ? (l[1 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[1] = shift < 448 ? (l[1 + shiftlimbs] >> shiftlow | (shift < 384 && shiftlow ? (l[2 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[2] = shift < 384 ? (l[2 + shiftlimbs] >> shiftlow | (shift < 320 && shiftlow ? (l[3 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[3] = shift < 320 ? (l[3 + shiftlimbs] >> shiftlow) : 0;
        scalar_cadd_bit(r, 0, (l[(shift - 1) >> 6] >> ((shift - 1) & 0x3f)) & 1);
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_mul_shift_var(
        r:     *mut Scalar,
        a:     *const Scalar,
        b:     *const Scalar,
        shift: u32)  {
    
    todo!();
        /*
            uint32_t l[16];
        unsigned int shiftlimbs;
        unsigned int shiftlow;
        unsigned int shifthigh;
        VERIFY_CHECK(shift >= 256);
        scalar_mul_512(l, a, b);
        shiftlimbs = shift >> 5;
        shiftlow = shift & 0x1F;
        shifthigh = 32 - shiftlow;
        r->d[0] = shift < 512 ? (l[0 + shiftlimbs] >> shiftlow | (shift < 480 && shiftlow ? (l[1 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[1] = shift < 480 ? (l[1 + shiftlimbs] >> shiftlow | (shift < 448 && shiftlow ? (l[2 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[2] = shift < 448 ? (l[2 + shiftlimbs] >> shiftlow | (shift < 416 && shiftlow ? (l[3 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[3] = shift < 416 ? (l[3 + shiftlimbs] >> shiftlow | (shift < 384 && shiftlow ? (l[4 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[4] = shift < 384 ? (l[4 + shiftlimbs] >> shiftlow | (shift < 352 && shiftlow ? (l[5 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[5] = shift < 352 ? (l[5 + shiftlimbs] >> shiftlow | (shift < 320 && shiftlow ? (l[6 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[6] = shift < 320 ? (l[6 + shiftlimbs] >> shiftlow | (shift < 288 && shiftlow ? (l[7 + shiftlimbs] << shifthigh) : 0)) : 0;
        r->d[7] = shift < 288 ? (l[7 + shiftlimbs] >> shiftlow)  : 0;
        scalar_cadd_bit(r, 0, (l[(shift - 1) >> 5] >> ((shift - 1) & 0x1f)) & 1);
        */
}

