crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/addition_overflow.cpp]

pub fn test_addition_overflow<T>(fuzzed_data_provider: &mut FuzzedDataProvider)  {

    todo!();
        /*
            const T i = fuzzed_data_provider.ConsumeIntegral<T>();
        const T j = fuzzed_data_provider.ConsumeIntegral<T>();
        const bool is_addition_overflow_custom = AdditionOverflow(i, j);
    #if defined(HAVE_BUILTIN_ADD_OVERFLOW)
        T result_builtin;
        const bool is_addition_overflow_builtin = __builtin_add_overflow(i, j, &result_builtin);
        assert(is_addition_overflow_custom == is_addition_overflow_builtin);
        if (!is_addition_overflow_custom) {
            assert(i + j == result_builtin);
        }
    #else
        if (!is_addition_overflow_custom) {
            (c_void)(i + j);
        }
    #endif
        */
}

#[fuzz_test] fn addition_overflow() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        TestAdditionOverflow<int64_t>(fuzzed_data_provider);
        TestAdditionOverflow<uint64_t>(fuzzed_data_provider);
        TestAdditionOverflow<int32_t>(fuzzed_data_provider);
        TestAdditionOverflow<uint32_t>(fuzzed_data_provider);
        TestAdditionOverflow<int16_t>(fuzzed_data_provider);
        TestAdditionOverflow<uint16_t>(fuzzed_data_provider);
        TestAdditionOverflow<char>(fuzzed_data_provider);
        TestAdditionOverflow<unsigned char>(fuzzed_data_provider);
        TestAdditionOverflow<signed char>(fuzzed_data_provider);

    */
}
