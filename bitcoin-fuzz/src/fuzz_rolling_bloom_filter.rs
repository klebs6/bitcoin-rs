// ---------------- [ File: bitcoin-fuzz/src/fuzz_rolling_bloom_filter.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/rolling_bloom_filter.cpp]

#[fuzz_test] fn rolling_bloom_filter() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        CRollingBloomFilter rolling_bloom_filter{
            fuzzed_data_provider.ConsumeIntegralInRange<unsigned int>(1, 1000),
            0.999 / fuzzed_data_provider.ConsumeIntegralInRange<unsigned int>(1, std::numeric_limits<unsigned int>::max())};
        LIMITED_WHILE(fuzzed_data_provider.remaining_bytes() > 0, 3000)
        {
            CallOneOf(
                fuzzed_data_provider,
                [&] {
                    const std::vector<unsigned char> b = ConsumeRandomLengthByteVector(fuzzed_data_provider);
                    (c_void)rolling_bloom_filter.contains(b);
                    rolling_bloom_filter.insert(b);
                    const bool present = rolling_bloom_filter.contains(b);
                    assert(present);
                },
                [&] {
                    const uint256 u256{ConsumeUInt256(fuzzed_data_provider)};
                    (c_void)rolling_bloom_filter.contains(u256);
                    rolling_bloom_filter.insert(u256);
                    const bool present = rolling_bloom_filter.contains(u256);
                    assert(present);
                },
                [&] {
                    rolling_bloom_filter.reset();
                });
        }

    */
}
