// ---------------- [ File: bitcoin-sha256d/src/transform_d64_wrapper.rs ]
crate::ix!();

pub fn transform_d64wrapper(
        tr:  TransformType,
        out: *mut u8,
        in_: *const u8)  {

    todo!();
        /*
            uint32_t s[8];
        static const unsigned char padding1[64] = {
            0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0
        };
        unsigned char buffer2[64] = {
            0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0
        };
        sha256::Initialize(s);
        tr(s, in, 1);
        tr(s, padding1, 1);
        WriteBE32(buffer2 + 0, s[0]);
        WriteBE32(buffer2 + 4, s[1]);
        WriteBE32(buffer2 + 8, s[2]);
        WriteBE32(buffer2 + 12, s[3]);
        WriteBE32(buffer2 + 16, s[4]);
        WriteBE32(buffer2 + 20, s[5]);
        WriteBE32(buffer2 + 24, s[6]);
        WriteBE32(buffer2 + 28, s[7]);
        sha256::Initialize(s);
        tr(s, buffer2, 1);
        WriteBE32(out + 0, s[0]);
        WriteBE32(out + 4, s[1]);
        WriteBE32(out + 8, s[2]);
        WriteBE32(out + 12, s[3]);
        WriteBE32(out + 16, s[4]);
        WriteBE32(out + 20, s[5]);
        WriteBE32(out + 24, s[6]);
        WriteBE32(out + 28, s[7]);
        */
}
