crate::ix!();

#[inline(always)] pub fn shuffle(
        s0: &mut __m128i,
        s1: &mut __m128i)  {
    
    todo!();
        /*
            const __m128i t1 = _mm_shuffle_epi32(s0, 0xB1);
            const __m128i t2 = _mm_shuffle_epi32(s1, 0x1B);
            s0 = _mm_alignr_epi8(t1, t2, 0x08);
            s1 = _mm_blend_epi16(t2, t1, 0xF0);
        */
}

#[inline(always)] pub fn unshuffle(
        s0: &mut __m128i,
        s1: &mut __m128i)  {
    
    todo!();
        /*
            const __m128i t1 = _mm_shuffle_epi32(s0, 0x1B);
            const __m128i t2 = _mm_shuffle_epi32(s1, 0xB1);
            s0 = _mm_blend_epi16(t1, t2, 0xF0);
            s1 = _mm_alignr_epi8(t2, t1, 0x08);
        */
}
