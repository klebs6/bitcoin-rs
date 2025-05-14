// ---------------- [ File: bitcoin-secp256k1/src/selftest.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/selftest.h]

pub fn secp_256k1_selftest_sha256() -> i32 {
    
    todo!();
        /*
            static const char *input63 = "For this sample, this 63-byte string will be used as input data";
        static const unsigned char output32[32] = {
            0xf0, 0x8a, 0x78, 0xcb, 0xba, 0xee, 0x08, 0x2b, 0x05, 0x2a, 0xe0, 0x70, 0x8f, 0x32, 0xfa, 0x1e,
            0x50, 0xc5, 0xc4, 0x21, 0xaa, 0x77, 0x2b, 0xa5, 0xdb, 0xb4, 0x06, 0xa2, 0xea, 0x6b, 0xe3, 0x42,
        };
        unsigned char out[32];
        secp256k1_sha256 hasher;
        secp256k1_sha256_initialize(&hasher);
        secp256k1_sha256_write(&hasher, (const unsigned char*)input63, 63);
        secp256k1_sha256_finalize(&hasher, out);
        return secp256k1_memcmp_var(out, output32, 32) == 0;
        */
}

pub fn secp_256k1_selftest() -> i32 {
    
    todo!();
        /*
            return secp256k1_selftest_sha256();
        */
}
