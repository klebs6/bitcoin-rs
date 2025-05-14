// ---------------- [ File: bitcoin-fuzz/src/fuzz_float.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/float.cpp]

#[fuzz_test] fn float() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        {
            const double d{[&] {
                double tmp;
                CallOneOf(
                    fuzzed_data_provider,
                    // an actual number
                    [&] { tmp = fuzzed_data_provider.ConsumeFloatingPoint<double>(); },
                    // special numbers and NANs
                    [&] { tmp = fuzzed_data_provider.PickValueInArray({
                              std::numeric_limits<double>::infinity(),
                              -std::numeric_limits<double>::infinity(),
                              std::numeric_limits<double>::min(),
                              -std::numeric_limits<double>::min(),
                              std::numeric_limits<double>::max(),
                              -std::numeric_limits<double>::max(),
                              std::numeric_limits<double>::lowest(),
                              -std::numeric_limits<double>::lowest(),
                              std::numeric_limits<double>::quiet_NaN(),
                              -std::numeric_limits<double>::quiet_NaN(),
                              std::numeric_limits<double>::signaling_NaN(),
                              -std::numeric_limits<double>::signaling_NaN(),
                              std::numeric_limits<double>::denorm_min(),
                              -std::numeric_limits<double>::denorm_min(),
                          }); },
                    // Anything from raw memory (also checks that DecodeDouble doesn't crash on any input)
                    [&] { tmp = DecodeDouble(fuzzed_data_provider.ConsumeIntegral<uint64_t>()); });
                return tmp;
            }()};
            (c_void)memusage::DynamicUsage(d);

            uint64_t encoded = EncodeDouble(d);
            if constexpr (std::numeric_limits<double>::is_iec559) {
                if (!std::isnan(d)) {
                    uint64_t encoded_in_memory;
                    std::copy((const unsigned char*)&d, (const unsigned char*)(&d + 1), (unsigned char*)&encoded_in_memory);
                    assert(encoded_in_memory == encoded);
                }
            }
            double d_deserialized = DecodeDouble(encoded);
            assert(std::isnan(d) == std::isnan(d_deserialized));
            assert(std::isnan(d) || d == d_deserialized);
        }

    */
}
