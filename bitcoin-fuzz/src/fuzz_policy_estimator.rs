// ---------------- [ File: bitcoin-fuzz/src/fuzz_policy_estimator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/policy_estimator.cpp]

pub fn initialize_policy_estimator()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<>();
        */
}

#[fuzz_test(initializer = "initialize_policy_estimator")]
fn policy_estimator() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        CBlockPolicyEstimator block_policy_estimator;
        while (fuzzed_data_provider.ConsumeBool()) {
            CallOneOf(
                fuzzed_data_provider,
                [&] {
                    const std::optional<CMutableTransaction> mtx = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
                    if (!mtx) {
                        return;
                    }
                    const CTransaction tx{*mtx};
                    block_policy_estimator.processTransaction(ConsumeTxMemPoolEntry(fuzzed_data_provider, tx), fuzzed_data_provider.ConsumeBool());
                    if (fuzzed_data_provider.ConsumeBool()) {
                        (c_void)block_policy_estimator.removeTx(tx.GetHash(), /* inBlock */ fuzzed_data_provider.ConsumeBool());
                    }
                },
                [&] {
                    std::vector<CTxMemPoolEntry> mempool_entries;
                    while (fuzzed_data_provider.ConsumeBool()) {
                        const std::optional<CMutableTransaction> mtx = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
                        if (!mtx) {
                            break;
                        }
                        const CTransaction tx{*mtx};
                        mempool_entries.push_back(ConsumeTxMemPoolEntry(fuzzed_data_provider, tx));
                    }
                    std::vector<const CTxMemPoolEntry*> ptrs;
                    ptrs.reserve(mempool_entries.size());
                    for (const CTxMemPoolEntry& mempool_entry : mempool_entries) {
                        ptrs.push_back(&mempool_entry);
                    }
                    block_policy_estimator.processBlock(fuzzed_data_provider.ConsumeIntegral<unsigned int>(), ptrs);
                },
                [&] {
                    (c_void)block_policy_estimator.removeTx(ConsumeUInt256(fuzzed_data_provider), /* inBlock */ fuzzed_data_provider.ConsumeBool());
                },
                [&] {
                    block_policy_estimator.FlushUnconfirmed();
                });
            (c_void)block_policy_estimator.estimateFee(fuzzed_data_provider.ConsumeIntegral<int>());
            EstimationResult result;
            (c_void)block_policy_estimator.estimateRawFee(fuzzed_data_provider.ConsumeIntegral<int>(), fuzzed_data_provider.ConsumeFloatingPoint<double>(), fuzzed_data_provider.PickValueInArray(ALL_FEE_ESTIMATE_HORIZONS), fuzzed_data_provider.ConsumeBool() ? &result : nullptr);
            FeeCalculation fee_calculation;
            (c_void)block_policy_estimator.estimateSmartFee(fuzzed_data_provider.ConsumeIntegral<int>(), fuzzed_data_provider.ConsumeBool() ? &fee_calculation : nullptr, fuzzed_data_provider.ConsumeBool());
            (c_void)block_policy_estimator.HighestTargetTracked(fuzzed_data_provider.PickValueInArray(ALL_FEE_ESTIMATE_HORIZONS));
        }
        {
            FuzzedAutoFileProvider fuzzed_auto_file_provider = ConsumeAutoFile(fuzzed_data_provider);
            CAutoFile fuzzed_auto_file = fuzzed_auto_file_provider.open();
            block_policy_estimator.Write(fuzzed_auto_file);
            block_policy_estimator.Read(fuzzed_auto_file);
        }

    */
}
