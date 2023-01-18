crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/primitives_transaction.cpp]

#[fuzz_test] fn primitives_transaction() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const CScript script = ConsumeScript(fuzzed_data_provider);
        const std::optional<OutPoint> out_point = ConsumeDeserializable<OutPoint>(fuzzed_data_provider);
        if (out_point) {
            const CTxIn tx_in{*out_point, script, fuzzed_data_provider.ConsumeIntegral<uint32_t>()};
            (c_void)tx_in;
        }
        const CTxOut tx_out_1{ConsumeMoney(fuzzed_data_provider), script};
        const CTxOut tx_out_2{ConsumeMoney(fuzzed_data_provider), ConsumeScript(fuzzed_data_provider)};
        assert((tx_out_1 == tx_out_2) != (tx_out_1 != tx_out_2));
        const std::optional<CMutableTransaction> mutable_tx_1 = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
        const std::optional<CMutableTransaction> mutable_tx_2 = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
        if (mutable_tx_1 && mutable_tx_2) {
            const CTransaction tx_1{*mutable_tx_1};
            const CTransaction tx_2{*mutable_tx_2};
            assert((tx_1 == tx_2) != (tx_1 != tx_2));
        }

    */
}
