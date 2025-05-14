// ---------------- [ File: bitcoin-fuzz/src/fuzz_multiplication_overflow.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/multiplication_overflow.cpp]

pub fn test_multiplication_overflow<T>(fuzzed_data_provider: &mut FuzzedDataProvider)  {

    todo!();
        /*
            const T i = fuzzed_data_provider.ConsumeIntegral<T>();
        const T j = fuzzed_data_provider.ConsumeIntegral<T>();
        const bool is_multiplication_overflow_custom = MultiplicationOverflow(i, j);
    #if defined(HAVE_BUILTIN_MUL_OVERFLOW)
        T result_builtin;
        const bool is_multiplication_overflow_builtin = __builtin_mul_overflow(i, j, &result_builtin);
        assert(is_multiplication_overflow_custom == is_multiplication_overflow_builtin);
        if (!is_multiplication_overflow_custom) {
            assert(i * j == result_builtin);
        }
    #else
        if (!is_multiplication_overflow_custom) {
            (c_void)(i * j);
        }
    #endif
        */
}

#[fuzz_test] fn multiplication_overflow() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        TestMultiplicationOverflow<int64_t>(fuzzed_data_provider);
        TestMultiplicationOverflow<uint64_t>(fuzzed_data_provider);
        TestMultiplicationOverflow<int32_t>(fuzzed_data_provider);
        TestMultiplicationOverflow<uint32_t>(fuzzed_data_provider);
        TestMultiplicationOverflow<int16_t>(fuzzed_data_provider);
        TestMultiplicationOverflow<uint16_t>(fuzzed_data_provider);
        TestMultiplicationOverflow<char>(fuzzed_data_provider);
        TestMultiplicationOverflow<unsigned char>(fuzzed_data_provider);
        TestMultiplicationOverflow<signed char>(fuzzed_data_provider);

    */
}
