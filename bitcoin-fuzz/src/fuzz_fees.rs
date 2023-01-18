crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/fees.cpp]

#[fuzz_test] fn fees() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const CFeeRate minimal_incremental_fee{ConsumeMoney(fuzzed_data_provider)};
        FeeFilterRounder fee_filter_rounder{minimal_incremental_fee};
        while (fuzzed_data_provider.ConsumeBool()) {
            const CAmount current_minimum_fee = ConsumeMoney(fuzzed_data_provider);
            const CAmount rounded_fee = fee_filter_rounder.round(current_minimum_fee);
            assert(MoneyRange(rounded_fee));
        }
        const FeeReason fee_reason = fuzzed_data_provider.PickValueInArray({FeeReason::NONE, FeeReason::HALF_ESTIMATE, FeeReason::FULL_ESTIMATE, FeeReason::DOUBLE_ESTIMATE, FeeReason::CONSERVATIVE, FeeReason::MEMPOOL_MIN, FeeReason::PAYTXFEE, FeeReason::FALLBACK, FeeReason::REQUIRED});
        (c_void)StringForFeeReason(fee_reason);

    */
}
