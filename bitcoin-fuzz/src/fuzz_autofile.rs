crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/autofile.cpp]

#[fuzz_test] fn autofile() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        FuzzedAutoFileProvider fuzzed_auto_file_provider = ConsumeAutoFile(fuzzed_data_provider);
        CAutoFile auto_file = fuzzed_auto_file_provider.open();
        while (fuzzed_data_provider.ConsumeBool()) {
            CallOneOf(
                fuzzed_data_provider,
                [&] {
                    std::array<uint8_t, 4096> arr{};
                    try {
                        auto_file.read((char*)arr.data(), fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, 4096));
                    } catch (const std::ios_base::failure&) {
                    }
                },
                [&] {
                    const std::array<uint8_t, 4096> arr{};
                    try {
                        auto_file.write((const char*)arr.data(), fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, 4096));
                    } catch (const std::ios_base::failure&) {
                    }
                },
                [&] {
                    try {
                        auto_file.ignore(fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, 4096));
                    } catch (const std::ios_base::failure&) {
                    }
                },
                [&] {
                    auto_file.fclose();
                },
                [&] {
                    ReadFromStream(fuzzed_data_provider, auto_file);
                },
                [&] {
                    WriteToStream(fuzzed_data_provider, auto_file);
                });
        }
        (c_void)auto_file.Get();
        (c_void)auto_file.GetType();
        (c_void)auto_file.GetVersion();
        (c_void)auto_file.IsNull();
        if (fuzzed_data_provider.ConsumeBool()) {
            FILE* f = auto_file.release();
            if (f != nullptr) {
                fclose(f);
            }
        }

    */
}
