// ---------------- [ File: bitcoin-compat/src/strnlen.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/strnlen.cpp]

#[cfg(HAVE_DECL_STRNLEN_EQ_0)]
pub fn strnlen(
        start:   *const u8,
        max_len: usize) -> usize {
    
    todo!();
        /*
            const char *end = (const char *)memchr(start, '\0', max_len);

        return end ? (size_t)(end - start) : max_len;
        */
}
