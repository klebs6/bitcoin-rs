// ---------------- [ File: bitcoin-ripemd/src/transform.rs ]
crate::ix!();

/**
  | Perform a RIPEMD-160 transformation,
  | processing a 64-byte chunk.
  |
  */
pub fn ripemd160_transform(
        s:     *mut u32,
        chunk: *const u8)  {
    
    todo!();
        /*
            uint32_t a1 = s[0], b1 = s[1], c1 = s[2], d1 = s[3], e1 = s[4];
        uint32_t a2 = a1, b2 = b1, c2 = c1, d2 = d1, e2 = e1;
        uint32_t w0 = ReadLE32(chunk + 0), w1 = ReadLE32(chunk + 4), w2 = ReadLE32(chunk + 8), w3 = ReadLE32(chunk + 12);
        uint32_t w4 = ReadLE32(chunk + 16), w5 = ReadLE32(chunk + 20), w6 = ReadLE32(chunk + 24), w7 = ReadLE32(chunk + 28);
        uint32_t w8 = ReadLE32(chunk + 32), w9 = ReadLE32(chunk + 36), w10 = ReadLE32(chunk + 40), w11 = ReadLE32(chunk + 44);
        uint32_t w12 = ReadLE32(chunk + 48), w13 = ReadLE32(chunk + 52), w14 = ReadLE32(chunk + 56), w15 = ReadLE32(chunk + 60);

        R11(a1, b1, c1, d1, e1, w0, 11);
        R12(a2, b2, c2, d2, e2, w5, 8);
        R11(e1, a1, b1, c1, d1, w1, 14);
        R12(e2, a2, b2, c2, d2, w14, 9);
        R11(d1, e1, a1, b1, c1, w2, 15);
        R12(d2, e2, a2, b2, c2, w7, 9);
        R11(c1, d1, e1, a1, b1, w3, 12);
        R12(c2, d2, e2, a2, b2, w0, 11);
        R11(b1, c1, d1, e1, a1, w4, 5);
        R12(b2, c2, d2, e2, a2, w9, 13);
        R11(a1, b1, c1, d1, e1, w5, 8);
        R12(a2, b2, c2, d2, e2, w2, 15);
        R11(e1, a1, b1, c1, d1, w6, 7);
        R12(e2, a2, b2, c2, d2, w11, 15);
        R11(d1, e1, a1, b1, c1, w7, 9);
        R12(d2, e2, a2, b2, c2, w4, 5);
        R11(c1, d1, e1, a1, b1, w8, 11);
        R12(c2, d2, e2, a2, b2, w13, 7);
        R11(b1, c1, d1, e1, a1, w9, 13);
        R12(b2, c2, d2, e2, a2, w6, 7);
        R11(a1, b1, c1, d1, e1, w10, 14);
        R12(a2, b2, c2, d2, e2, w15, 8);
        R11(e1, a1, b1, c1, d1, w11, 15);
        R12(e2, a2, b2, c2, d2, w8, 11);
        R11(d1, e1, a1, b1, c1, w12, 6);
        R12(d2, e2, a2, b2, c2, w1, 14);
        R11(c1, d1, e1, a1, b1, w13, 7);
        R12(c2, d2, e2, a2, b2, w10, 14);
        R11(b1, c1, d1, e1, a1, w14, 9);
        R12(b2, c2, d2, e2, a2, w3, 12);
        R11(a1, b1, c1, d1, e1, w15, 8);
        R12(a2, b2, c2, d2, e2, w12, 6);

        R21(e1, a1, b1, c1, d1, w7, 7);
        R22(e2, a2, b2, c2, d2, w6, 9);
        R21(d1, e1, a1, b1, c1, w4, 6);
        R22(d2, e2, a2, b2, c2, w11, 13);
        R21(c1, d1, e1, a1, b1, w13, 8);
        R22(c2, d2, e2, a2, b2, w3, 15);
        R21(b1, c1, d1, e1, a1, w1, 13);
        R22(b2, c2, d2, e2, a2, w7, 7);
        R21(a1, b1, c1, d1, e1, w10, 11);
        R22(a2, b2, c2, d2, e2, w0, 12);
        R21(e1, a1, b1, c1, d1, w6, 9);
        R22(e2, a2, b2, c2, d2, w13, 8);
        R21(d1, e1, a1, b1, c1, w15, 7);
        R22(d2, e2, a2, b2, c2, w5, 9);
        R21(c1, d1, e1, a1, b1, w3, 15);
        R22(c2, d2, e2, a2, b2, w10, 11);
        R21(b1, c1, d1, e1, a1, w12, 7);
        R22(b2, c2, d2, e2, a2, w14, 7);
        R21(a1, b1, c1, d1, e1, w0, 12);
        R22(a2, b2, c2, d2, e2, w15, 7);
        R21(e1, a1, b1, c1, d1, w9, 15);
        R22(e2, a2, b2, c2, d2, w8, 12);
        R21(d1, e1, a1, b1, c1, w5, 9);
        R22(d2, e2, a2, b2, c2, w12, 7);
        R21(c1, d1, e1, a1, b1, w2, 11);
        R22(c2, d2, e2, a2, b2, w4, 6);
        R21(b1, c1, d1, e1, a1, w14, 7);
        R22(b2, c2, d2, e2, a2, w9, 15);
        R21(a1, b1, c1, d1, e1, w11, 13);
        R22(a2, b2, c2, d2, e2, w1, 13);
        R21(e1, a1, b1, c1, d1, w8, 12);
        R22(e2, a2, b2, c2, d2, w2, 11);

        R31(d1, e1, a1, b1, c1, w3, 11);
        R32(d2, e2, a2, b2, c2, w15, 9);
        R31(c1, d1, e1, a1, b1, w10, 13);
        R32(c2, d2, e2, a2, b2, w5, 7);
        R31(b1, c1, d1, e1, a1, w14, 6);
        R32(b2, c2, d2, e2, a2, w1, 15);
        R31(a1, b1, c1, d1, e1, w4, 7);
        R32(a2, b2, c2, d2, e2, w3, 11);
        R31(e1, a1, b1, c1, d1, w9, 14);
        R32(e2, a2, b2, c2, d2, w7, 8);
        R31(d1, e1, a1, b1, c1, w15, 9);
        R32(d2, e2, a2, b2, c2, w14, 6);
        R31(c1, d1, e1, a1, b1, w8, 13);
        R32(c2, d2, e2, a2, b2, w6, 6);
        R31(b1, c1, d1, e1, a1, w1, 15);
        R32(b2, c2, d2, e2, a2, w9, 14);
        R31(a1, b1, c1, d1, e1, w2, 14);
        R32(a2, b2, c2, d2, e2, w11, 12);
        R31(e1, a1, b1, c1, d1, w7, 8);
        R32(e2, a2, b2, c2, d2, w8, 13);
        R31(d1, e1, a1, b1, c1, w0, 13);
        R32(d2, e2, a2, b2, c2, w12, 5);
        R31(c1, d1, e1, a1, b1, w6, 6);
        R32(c2, d2, e2, a2, b2, w2, 14);
        R31(b1, c1, d1, e1, a1, w13, 5);
        R32(b2, c2, d2, e2, a2, w10, 13);
        R31(a1, b1, c1, d1, e1, w11, 12);
        R32(a2, b2, c2, d2, e2, w0, 13);
        R31(e1, a1, b1, c1, d1, w5, 7);
        R32(e2, a2, b2, c2, d2, w4, 7);
        R31(d1, e1, a1, b1, c1, w12, 5);
        R32(d2, e2, a2, b2, c2, w13, 5);

        R41(c1, d1, e1, a1, b1, w1, 11);
        R42(c2, d2, e2, a2, b2, w8, 15);
        R41(b1, c1, d1, e1, a1, w9, 12);
        R42(b2, c2, d2, e2, a2, w6, 5);
        R41(a1, b1, c1, d1, e1, w11, 14);
        R42(a2, b2, c2, d2, e2, w4, 8);
        R41(e1, a1, b1, c1, d1, w10, 15);
        R42(e2, a2, b2, c2, d2, w1, 11);
        R41(d1, e1, a1, b1, c1, w0, 14);
        R42(d2, e2, a2, b2, c2, w3, 14);
        R41(c1, d1, e1, a1, b1, w8, 15);
        R42(c2, d2, e2, a2, b2, w11, 14);
        R41(b1, c1, d1, e1, a1, w12, 9);
        R42(b2, c2, d2, e2, a2, w15, 6);
        R41(a1, b1, c1, d1, e1, w4, 8);
        R42(a2, b2, c2, d2, e2, w0, 14);
        R41(e1, a1, b1, c1, d1, w13, 9);
        R42(e2, a2, b2, c2, d2, w5, 6);
        R41(d1, e1, a1, b1, c1, w3, 14);
        R42(d2, e2, a2, b2, c2, w12, 9);
        R41(c1, d1, e1, a1, b1, w7, 5);
        R42(c2, d2, e2, a2, b2, w2, 12);
        R41(b1, c1, d1, e1, a1, w15, 6);
        R42(b2, c2, d2, e2, a2, w13, 9);
        R41(a1, b1, c1, d1, e1, w14, 8);
        R42(a2, b2, c2, d2, e2, w9, 12);
        R41(e1, a1, b1, c1, d1, w5, 6);
        R42(e2, a2, b2, c2, d2, w7, 5);
        R41(d1, e1, a1, b1, c1, w6, 5);
        R42(d2, e2, a2, b2, c2, w10, 15);
        R41(c1, d1, e1, a1, b1, w2, 12);
        R42(c2, d2, e2, a2, b2, w14, 8);

        R51(b1, c1, d1, e1, a1, w4, 9);
        R52(b2, c2, d2, e2, a2, w12, 8);
        R51(a1, b1, c1, d1, e1, w0, 15);
        R52(a2, b2, c2, d2, e2, w15, 5);
        R51(e1, a1, b1, c1, d1, w5, 5);
        R52(e2, a2, b2, c2, d2, w10, 12);
        R51(d1, e1, a1, b1, c1, w9, 11);
        R52(d2, e2, a2, b2, c2, w4, 9);
        R51(c1, d1, e1, a1, b1, w7, 6);
        R52(c2, d2, e2, a2, b2, w1, 12);
        R51(b1, c1, d1, e1, a1, w12, 8);
        R52(b2, c2, d2, e2, a2, w5, 5);
        R51(a1, b1, c1, d1, e1, w2, 13);
        R52(a2, b2, c2, d2, e2, w8, 14);
        R51(e1, a1, b1, c1, d1, w10, 12);
        R52(e2, a2, b2, c2, d2, w7, 6);
        R51(d1, e1, a1, b1, c1, w14, 5);
        R52(d2, e2, a2, b2, c2, w6, 8);
        R51(c1, d1, e1, a1, b1, w1, 12);
        R52(c2, d2, e2, a2, b2, w2, 13);
        R51(b1, c1, d1, e1, a1, w3, 13);
        R52(b2, c2, d2, e2, a2, w13, 6);
        R51(a1, b1, c1, d1, e1, w8, 14);
        R52(a2, b2, c2, d2, e2, w14, 5);
        R51(e1, a1, b1, c1, d1, w11, 11);
        R52(e2, a2, b2, c2, d2, w0, 15);
        R51(d1, e1, a1, b1, c1, w6, 8);
        R52(d2, e2, a2, b2, c2, w3, 13);
        R51(c1, d1, e1, a1, b1, w15, 5);
        R52(c2, d2, e2, a2, b2, w9, 11);
        R51(b1, c1, d1, e1, a1, w13, 6);
        R52(b2, c2, d2, e2, a2, w11, 11);

        uint32_t t = s[0];
        s[0] = s[1] + c1 + d2;
        s[1] = s[2] + d1 + e2;
        s[2] = s[3] + e1 + a2;
        s[3] = s[4] + a1 + b2;
        s[4] = t + b1 + c2;
        */
}
