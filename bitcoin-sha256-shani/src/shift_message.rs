// ---------------- [ File: bitcoin-sha256-shani/src/shift_message.rs ]
crate::ix!();

#[inline(always)] pub fn shift_messagea(
        m0: &mut __m128i,
        m1: __m128i)  {
    
    todo!();
        /*
            m0 = _mm_sha256msg1_epu32(m0, m1);
        */
}

#[inline(always)] pub fn shift_messagec(
        m0: &mut __m128i,
        m1: __m128i,
        m2: &mut __m128i)  {
    
    todo!();
        /*
            m2 = _mm_sha256msg2_epu32(_mm_add_epi32(m2, _mm_alignr_epi8(m1, m0, 4)), m1);
        */
}

#[inline(always)] pub fn shift_messageb(
        m0: &mut __m128i,
        m1: __m128i,
        m2: &mut __m128i)  {
    
    todo!();
        /*
            ShiftMessageC(m0, m1, m2);
            ShiftMessageA(m0, m1);
        */
}
