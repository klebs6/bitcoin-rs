// ---------------- [ File: bitcoin-fuzz/src/fuzz_rbf.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/rbf.cpp]

#[fuzz_test] fn rbf() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        SetMockTime(ConsumeTime(fuzzed_data_provider));
        std::optional<CMutableTransaction> mtx = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
        if (!mtx) {
            return;
        }
        CTxMemPool pool;
        while (fuzzed_data_provider.ConsumeBool()) {
            const std::optional<CMutableTransaction> another_mtx = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
            if (!another_mtx) {
                break;
            }
            const CTransaction another_tx{*another_mtx};
            if (fuzzed_data_provider.ConsumeBool() && !mtx->vin.empty()) {
                mtx->vin[0].prevout = OutPoint{another_tx.GetHash(), 0};
            }
            LOCK2(cs_main, pool.cs);
            pool.addUnchecked(ConsumeTxMemPoolEntry(fuzzed_data_provider, another_tx));
        }
        const CTransaction tx{*mtx};
        if (fuzzed_data_provider.ConsumeBool()) {
            LOCK2(cs_main, pool.cs);
            pool.addUnchecked(ConsumeTxMemPoolEntry(fuzzed_data_provider, tx));
        }
        {
            LOCK(pool.cs);
            (c_void)IsRBFOptIn(tx, pool);
        }

    */
}
