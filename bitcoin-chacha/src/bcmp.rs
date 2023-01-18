crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/chacha_poly_aead.cpp]

#[cfg(not(HAVE_TIMINGSAFE_BCMP))]
pub fn timingsafe_bcmp(
        b1: *const u8,
        b2: *const u8,
        n:  usize) -> i32 {
    
    todo!();
        /*
            const unsigned char *p1 = b1, *p2 = b2;
        int ret = 0;

        for (; n > 0; n--)
            ret |= *p1++ ^ *p2++;
        return (ret != 0);
        */
}

