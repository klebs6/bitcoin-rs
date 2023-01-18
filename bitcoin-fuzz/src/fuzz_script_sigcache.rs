crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/script_sigcache.cpp]

pub fn initialize_script_sigcache()  {
    
    todo!();
        /*
            static const ECCVerifyHandle ecc_verify_handle;
        ECC_Start();
        SelectParams(CBaseChainParams::REGTEST);
        InitSignatureCache();
        */
}

#[fuzz_test(initializer = "initialize_script_sigcache")]
fn script_sigcache() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        const std::optional<CMutableTransaction> mutable_transaction = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
        const CTransaction tx{mutable_transaction ? *mutable_transaction : CMutableTransaction{}};
        const unsigned int n_in = fuzzed_data_provider.ConsumeIntegral<unsigned int>();
        const CAmount amount = ConsumeMoney(fuzzed_data_provider);
        const bool store = fuzzed_data_provider.ConsumeBool();
        PrecomputedTransactionData tx_data;
        CachingTransactionSignatureChecker caching_transaction_signature_checker{mutable_transaction ? &tx : nullptr, n_in, amount, store, tx_data};
        if (fuzzed_data_provider.ConsumeBool()) {
            const auto random_bytes = fuzzed_data_provider.ConsumeBytes<unsigned char>(64);
            const XOnlyPubKey pub_key(ConsumeUInt256(fuzzed_data_provider));
            if (random_bytes.size() == 64) {
                (c_void)caching_transaction_signature_checker.VerifySchnorrSignature(random_bytes, pub_key, ConsumeUInt256(fuzzed_data_provider));
            }
        } else {
            const auto random_bytes = ConsumeRandomLengthByteVector(fuzzed_data_provider);
            const auto pub_key = ConsumeDeserializable<CPubKey>(fuzzed_data_provider);
            if (pub_key) {
                if (!random_bytes.empty()) {
                    (c_void)caching_transaction_signature_checker.VerifyECDSASignature(random_bytes, *pub_key, ConsumeUInt256(fuzzed_data_provider));
                }
            }
        }

    */
}
