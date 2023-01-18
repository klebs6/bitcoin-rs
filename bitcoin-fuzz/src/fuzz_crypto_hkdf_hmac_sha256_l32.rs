crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/crypto_hkdf_hmac_sha256_l32.cpp]

#[fuzz_test] fn crypto_hkdf_hmac_sha256_l32() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};

        const std::vector<uint8_t> initial_key_material = ConsumeRandomLengthByteVector(fuzzed_data_provider);

        CHKDF_HMAC_SHA256_L32 hkdf_hmac_sha256_l32(initial_key_material.data(), initial_key_material.size(), fuzzed_data_provider.ConsumeRandomLengthString(1024));
        while (fuzzed_data_provider.ConsumeBool()) {
            std::vector<uint8_t> out(32);
            hkdf_hmac_sha256_l32.Expand32(fuzzed_data_provider.ConsumeRandomLengthString(128), out.data());
        }

    */
}
