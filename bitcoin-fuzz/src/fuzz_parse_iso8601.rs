// ---------------- [ File: bitcoin-fuzz/src/fuzz_parse_iso8601.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/parse_iso8601.cpp]

#[fuzz_test] fn parse_iso8601() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        const int64_t random_time = fuzzed_data_provider.ConsumeIntegral<int32_t>();
        const std::string random_string = fuzzed_data_provider.ConsumeRemainingBytesAsString();

        const std::string iso8601_datetime = FormatISO8601DateTime(random_time);
        const int64_t parsed_time_1 = ParseISO8601DateTime(iso8601_datetime);
        if (random_time >= 0) {
            assert(parsed_time_1 >= 0);
            if (iso8601_datetime.length() == 20) {
                assert(parsed_time_1 == random_time);
            }
        }

        const int64_t parsed_time_2 = ParseISO8601DateTime(random_string);
        assert(parsed_time_2 >= 0);

    */
}
