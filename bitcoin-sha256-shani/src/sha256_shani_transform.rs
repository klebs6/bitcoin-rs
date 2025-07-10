// ---------------- [ File: bitcoin-sha256-shani/src/sha256_shani_transform.rs ]
crate::ix!();

pub fn sha256_shani_transform(
    s:      *mut u32,
    chunk:  *const u8,
    blocks: usize)  {

    todo!();
    /*
            __m128i m0, m1, m2, m3, s0, s1, so0, so1;

        /* Load state */
        s0 = _mm_loadu_si128((const __m128i*)s);
        s1 = _mm_loadu_si128((const __m128i*)(s + 4));
        Shuffle(s0, s1);

        while (blocks--) {
            /* Remember old state */
            so0 = s0;
            so1 = s1;

            /* Load data and transform */
            m0 = Load(chunk);
            QuadRound(s0, s1, m0, 0xe9b5dba5b5c0fbcfull, 0x71374491428a2f98ull);
            m1 = Load(chunk + 16);
            QuadRound(s0, s1, m1, 0xab1c5ed5923f82a4ull, 0x59f111f13956c25bull);
            ShiftMessageA(m0, m1);
            m2 = Load(chunk + 32);
            QuadRound(s0, s1, m2, 0x550c7dc3243185beull, 0x12835b01d807aa98ull);
            ShiftMessageA(m1, m2);
            m3 = Load(chunk + 48);
            QuadRound(s0, s1, m3, 0xc19bf1749bdc06a7ull, 0x80deb1fe72be5d74ull);
            ShiftMessageB(m2, m3, m0);
            QuadRound(s0, s1, m0, 0x240ca1cc0fc19dc6ull, 0xefbe4786E49b69c1ull);
            ShiftMessageB(m3, m0, m1);
            QuadRound(s0, s1, m1, 0x76f988da5cb0a9dcull, 0x4a7484aa2de92c6full);
            ShiftMessageB(m0, m1, m2);
            QuadRound(s0, s1, m2, 0xbf597fc7b00327c8ull, 0xa831c66d983e5152ull);
            ShiftMessageB(m1, m2, m3);
            QuadRound(s0, s1, m3, 0x1429296706ca6351ull, 0xd5a79147c6e00bf3ull);
            ShiftMessageB(m2, m3, m0);
            QuadRound(s0, s1, m0, 0x53380d134d2c6dfcull, 0x2e1b213827b70a85ull);
            ShiftMessageB(m3, m0, m1);
            QuadRound(s0, s1, m1, 0x92722c8581c2c92eull, 0x766a0abb650a7354ull);
            ShiftMessageB(m0, m1, m2);
            QuadRound(s0, s1, m2, 0xc76c51A3c24b8b70ull, 0xa81a664ba2bfe8a1ull);
            ShiftMessageB(m1, m2, m3);
            QuadRound(s0, s1, m3, 0x106aa070f40e3585ull, 0xd6990624d192e819ull);
            ShiftMessageB(m2, m3, m0);
            QuadRound(s0, s1, m0, 0x34b0bcb52748774cull, 0x1e376c0819a4c116ull);
            ShiftMessageB(m3, m0, m1);
            QuadRound(s0, s1, m1, 0x682e6ff35b9cca4full, 0x4ed8aa4a391c0cb3ull);
            ShiftMessageC(m0, m1, m2);
            QuadRound(s0, s1, m2, 0x8cc7020884c87814ull, 0x78a5636f748f82eeull);
            ShiftMessageC(m1, m2, m3);
            QuadRound(s0, s1, m3, 0xc67178f2bef9A3f7ull, 0xa4506ceb90befffaull);

            /* Combine with old state */
            s0 = _mm_add_epi32(s0, so0);
            s1 = _mm_add_epi32(s1, so1);

            /* Advance */
            chunk += 64;
        }

    Unshuffle(s0, s1);
    _mm_storeu_si128((__m128i*)s, s0);
    _mm_storeu_si128((__m128i*)(s + 4), s1);
    */
}
