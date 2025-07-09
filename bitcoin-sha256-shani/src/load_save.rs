crate::ix!();

#[inline(always)] pub fn load(in_: *const u8)  {
    
    todo!();
        /*
            return _mm_shuffle_epi8(_mm_loadu_si128((const __m128i*)in), _mm_load_si128((const __m128i*)MASK));
        */
}

#[inline(always)] pub fn save(
        out: *mut u8,
        s:   __m128i)  {
    
    todo!();
        /*
            _mm_storeu_si128((__m128i*)out, _mm_shuffle_epi8(s, _mm_load_si128((const __m128i*)MASK)));
        */
}
