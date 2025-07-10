// ---------------- [ File: bitcoin-sha256-shani/src/quad_round.rs ]
crate::ix!();

#[inline(always)] pub fn quad_round(
        state0: &mut __m128i,
        state1: &mut __m128i,
        k1:     u64,
        k0:     u64)  {
    
    todo!();
        /*
            const __m128i msg = _mm_set_epi64x(k1, k0);
            state1 = _mm_sha256rnds2_epu32(state1, state0, msg);
            state0 = _mm_sha256rnds2_epu32(state0, state1, _mm_shuffle_epi32(msg, 0x0e));
        */
}

#[inline(always)] pub fn quad_round(
        state0: &mut __m128i,
        state1: &mut __m128i,
        m:      __m128i,
        k1:     u64,
        k0:     u64)  {
    
    todo!();
        /*
            const __m128i msg = _mm_add_epi32(m, _mm_set_epi64x(k1, k0));
            state1 = _mm_sha256rnds2_epu32(state1, state0, msg);
            state0 = _mm_sha256rnds2_epu32(state0, state1, _mm_shuffle_epi32(msg, 0x0e));
        */
}
