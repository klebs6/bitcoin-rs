crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/fee_rate.cpp]

#[fuzz_test] fn fee_rate() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const CAmount satoshis_per_k = ConsumeMoney(fuzzed_data_provider);
        const CFeeRate fee_rate{satoshis_per_k};

        (c_void)fee_rate.GetFeePerK();
        const auto bytes = fuzzed_data_provider.ConsumeIntegral<uint32_t>();
        if (!MultiplicationOverflow(int64_t{bytes}, satoshis_per_k)) {
            (c_void)fee_rate.GetFee(bytes);
        }
        (c_void)fee_rate.ToString();

        const CAmount another_satoshis_per_k = ConsumeMoney(fuzzed_data_provider);
        CFeeRate larger_fee_rate{another_satoshis_per_k};
        larger_fee_rate += fee_rate;
        if (satoshis_per_k != 0 && another_satoshis_per_k != 0) {
            assert(fee_rate < larger_fee_rate);
            assert(!(fee_rate > larger_fee_rate));
            assert(!(fee_rate == larger_fee_rate));
            assert(fee_rate <= larger_fee_rate);
            assert(!(fee_rate >= larger_fee_rate));
            assert(fee_rate != larger_fee_rate);
        }

    */
}
