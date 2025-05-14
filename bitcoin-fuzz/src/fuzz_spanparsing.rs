// ---------------- [ File: bitcoin-fuzz/src/fuzz_spanparsing.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/spanparsing.cpp]

#[fuzz_test] fn spanparsing() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const size_t query_size = fuzzed_data_provider.ConsumeIntegral<size_t>();
        const std::string query = fuzzed_data_provider.ConsumeBytesAsString(std::min<size_t>(query_size, 1024 * 1024));
        const std::string span_str = fuzzed_data_provider.ConsumeRemainingBytesAsString();
        const Span<const char> const_span{span_str};

        Span<const char> mut_span = const_span;
        (c_void)spanparsing::Const(query, mut_span);

        mut_span = const_span;
        (c_void)spanparsing::Func(query, mut_span);

        mut_span = const_span;
        (c_void)spanparsing::Expr(mut_span);

        if (!query.empty()) {
            mut_span = const_span;
            (c_void)spanparsing::Split(mut_span, query.front());
        }

    */
}
