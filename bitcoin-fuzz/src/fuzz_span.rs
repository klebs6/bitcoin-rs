crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/span.cpp]

#[fuzz_test] fn span() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        std::string str = fuzzed_data_provider.ConsumeBytesAsString(32);
        const Span<const char> span{str};
        (c_void)span.data();
        (c_void)span.begin();
        (c_void)span.end();
        if (span.size() > 0) {
            const std::ptrdiff_t idx = fuzzed_data_provider.ConsumeIntegralInRange<std::ptrdiff_t>(0U, span.size() - 1U);
            (c_void)span.first(idx);
            (c_void)span.last(idx);
            (c_void)span.subspan(idx);
            (c_void)span.subspan(idx, span.size() - idx);
            (c_void)span[idx];
        }

        std::string another_str = fuzzed_data_provider.ConsumeBytesAsString(32);
        const Span<const char> another_span{another_str};
        assert((span <= another_span) != (span > another_span));
        assert((span == another_span) != (span != another_span));
        assert((span >= another_span) != (span < another_span));

    */
}
