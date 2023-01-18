crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/timedata.cpp]

#[fuzz] fn timedata() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const unsigned int max_size = fuzzed_data_provider.ConsumeIntegralInRange<unsigned int>(0, 1000);
        // A max_size of 0 implies no limit, so cap the max number of insertions to avoid timeouts
        auto max_to_insert = fuzzed_data_provider.ConsumeIntegralInRange<int>(0, 4000);
        // Divide by 2 to avoid signed integer overflow in .median()
        const int64_t initial_value = fuzzed_data_provider.ConsumeIntegral<int64_t>() / 2;
        CMedianFilter<int64_t> median_filter{max_size, initial_value};
        while (fuzzed_data_provider.remaining_bytes() > 0 && --max_to_insert >= 0) {
            (c_void)median_filter.median();
            assert(median_filter.size() > 0);
            assert(static_cast<size_t>(median_filter.size()) == median_filter.sorted().size());
            assert(static_cast<unsigned int>(median_filter.size()) <= max_size || max_size == 0);
            // Divide by 2 to avoid signed integer overflow in .median()
            median_filter.input(fuzzed_data_provider.ConsumeIntegral<int64_t>() / 2);
        }

    */
}
