// ---------------- [ File: bitcoin-fuzz/src/fuzz_script_consensus.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/script_bitcoin_consensus.cpp]

#[fuzz_test] fn script_bitcoin_consensus() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::vector<uint8_t> random_bytes_1 = ConsumeRandomLengthByteVector(fuzzed_data_provider);
        const std::vector<uint8_t> random_bytes_2 = ConsumeRandomLengthByteVector(fuzzed_data_provider);
        const CAmount money = ConsumeMoney(fuzzed_data_provider);
        bitcoinconsensus_error err;
        bitcoinconsensus_error* err_p = fuzzed_data_provider.ConsumeBool() ? &err : nullptr;
        const unsigned int n_in = fuzzed_data_provider.ConsumeIntegral<unsigned int>();
        const unsigned int flags = fuzzed_data_provider.ConsumeIntegral<unsigned int>();
        assert(bitcoinconsensus_version() == BITCOINCONSENSUS_API_VER);
        if ((flags & SCRIPT_VERIFY_WITNESS) != 0 && (flags & SCRIPT_VERIFY_P2SH) == 0) {
            return;
        }
        (c_void)bitcoinconsensus_verify_script(random_bytes_1.data(), random_bytes_1.size(), random_bytes_2.data(), random_bytes_2.size(), n_in, flags, err_p);
        (c_void)bitcoinconsensus_verify_script_with_amount(random_bytes_1.data(), random_bytes_1.size(), money, random_bytes_2.data(), random_bytes_2.size(), n_in, flags, err_p);

    */
}
