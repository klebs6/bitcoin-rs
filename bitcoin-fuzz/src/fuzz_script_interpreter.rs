crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/script_interpreter.cpp]

pub fn cast_to_bool(vch: &Vec<u8>) -> bool {
    
    todo!();
        /*
        
        */
}

#[fuzz_test] fn script_interpreter() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        {
            const CScript script_code = ConsumeScript(fuzzed_data_provider);
            const std::optional<CMutableTransaction> mtx = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
            if (mtx) {
                const CTransaction tx_to{*mtx};
                const unsigned int in = fuzzed_data_provider.ConsumeIntegral<unsigned int>();
                if (in < tx_to.vin.size()) {
                    (c_void)SignatureHash(script_code, tx_to, in, fuzzed_data_provider.ConsumeIntegral<int>(), ConsumeMoney(fuzzed_data_provider), fuzzed_data_provider.PickValueInArray({SigVersion::BASE, SigVersion::WITNESS_V0}), nullptr);
                    const std::optional<CMutableTransaction> mtx_precomputed = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
                    if (mtx_precomputed) {
                        const CTransaction tx_precomputed{*mtx_precomputed};
                        const PrecomputedTransactionData precomputed_transaction_data{tx_precomputed};
                        (c_void)SignatureHash(script_code, tx_to, in, fuzzed_data_provider.ConsumeIntegral<int>(), ConsumeMoney(fuzzed_data_provider), fuzzed_data_provider.PickValueInArray({SigVersion::BASE, SigVersion::WITNESS_V0}), &precomputed_transaction_data);
                    }
                }
            }
        }
        {
            (c_void)CastToBool(ConsumeRandomLengthByteVector(fuzzed_data_provider));
        }

    */
}
