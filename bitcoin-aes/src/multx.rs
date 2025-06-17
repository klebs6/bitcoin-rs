crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ctaes/ctaes.c]

/// Multiply the cells in s by x, as polynomials over GF(2) mod x^8 + x^4 + x^3 + x + 1
///
/// Multiply state by x in GF(2⁸) mod x⁸ + x⁴ + x³ + x + 1.
///
#[inline(always)]
pub fn multx(s: *mut AESState) {
    tracing::trace!(target: "aes", "multx – entry {:p}", s);

    unsafe {
        let top = (*s).slice[7];
        (*s).slice[7] = (*s).slice[6];
        (*s).slice[6] = (*s).slice[5];
        (*s).slice[5] = (*s).slice[4];
        (*s).slice[4] = (*s).slice[3] ^ top;
        (*s).slice[3] = (*s).slice[2] ^ top;
        (*s).slice[2] = (*s).slice[1];
        (*s).slice[1] = (*s).slice[0] ^ top;
        (*s).slice[0] = top;
    }

    tracing::trace!(target: "aes", "multx – exit");
}
