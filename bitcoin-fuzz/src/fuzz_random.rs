// ---------------- [ File: bitcoin-fuzz/src/fuzz_random.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/random.cpp]

#[fuzz_test] fn random() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        FastRandomContext fast_random_context{ConsumeUInt256(fuzzed_data_provider)};
        (c_void)fast_random_context.rand64();
        (c_void)fast_random_context.randbits(fuzzed_data_provider.ConsumeIntegralInRange<int>(0, 64));
        (c_void)fast_random_context.randrange(fuzzed_data_provider.ConsumeIntegralInRange<uint64_t>(FastRandomContext::min() + 1, FastRandomContext::max()));
        (c_void)fast_random_context.randbytes(fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, 1024));
        (c_void)fast_random_context.rand32();
        (c_void)fast_random_context.rand256();
        (c_void)fast_random_context.randbool();
        (c_void)fast_random_context();

        std::vector<int64_t> integrals = ConsumeRandomLengthIntegralVector<int64_t>(fuzzed_data_provider);
        Shuffle(integrals.begin(), integrals.end(), fast_random_context);
        std::shuffle(integrals.begin(), integrals.end(), fast_random_context);

    */
}
