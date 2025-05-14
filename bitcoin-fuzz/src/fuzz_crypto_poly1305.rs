// ---------------- [ File: bitcoin-fuzz/src/fuzz_crypto_poly1305.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/crypto_poly1305.cpp]

#[fuzz_test] fn crypto_poly1305() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};

        const std::vector<uint8_t> key = ConsumeFixedLengthByteVector(fuzzed_data_provider, POLY1305_KEYLEN);
        const std::vector<uint8_t> in = ConsumeRandomLengthByteVector(fuzzed_data_provider);

        std::vector<uint8_t> tag_out(POLY1305_TAGLEN);
        poly1305_auth(tag_out.data(), in.data(), in.size(), key.data());

    */
}
