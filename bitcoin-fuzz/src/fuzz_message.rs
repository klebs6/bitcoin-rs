crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/message.cpp]

pub fn initialize_message()  {
    
    todo!();
        /*
            static const ECCVerifyHandle ecc_verify_handle;
        ECC_Start();
        SelectParams(CBaseChainParams::REGTEST);
        */
}

#[fuzz_test(initializer = "initialize_message")]
fn message() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::string random_message = fuzzed_data_provider.ConsumeRandomLengthString(1024);
        {
            const std::vector<uint8_t> random_bytes = ConsumeRandomLengthByteVector(fuzzed_data_provider);
            CKey private_key;
            private_key.Set(random_bytes.begin(), random_bytes.end(), fuzzed_data_provider.ConsumeBool());
            std::string signature;
            const bool message_signed = MessageSign(private_key, random_message, signature);
            if (private_key.IsValid()) {
                assert(message_signed);
                const MessageVerificationResult verification_result = MessageVerify(EncodeDestination(PKHash(private_key.GetPubKey().GetID())), signature, random_message);
                assert(verification_result == MessageVerificationResult::OK);
            }
        }
        {
            (c_void)MessageHash(random_message);
            (c_void)MessageVerify(fuzzed_data_provider.ConsumeRandomLengthString(1024), fuzzed_data_provider.ConsumeRandomLengthString(1024), random_message);
            (c_void)SigningResultString(fuzzed_data_provider.PickValueInArray({SigningResult::OK, SigningResult::PRIVATE_KEY_NOT_AVAILABLE, SigningResult::SIGNING_FAILED}));
        }

    */
}
