crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/parse_hd_keypath.cpp]

#[fuzz_test] fn parse_hd_keypath() {
    todo!();
    /*
    
        const std::string keypath_str(buffer.begin(), buffer.end());
        std::vector<uint32_t> keypath;
        (c_void)ParseHDKeypath(keypath_str, keypath);

        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::vector<uint32_t> random_keypath = ConsumeRandomLengthIntegralVector<uint32_t>(fuzzed_data_provider);
        (c_void)FormatHDKeypath(random_keypath);
        (c_void)WriteHDKeypath(random_keypath);

    */
}

