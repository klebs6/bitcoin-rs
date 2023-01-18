crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/secp256k1_ecdsa_signature_parse_der_lax.cpp]

pub fn sig_has_lowr(sig: *const Secp256k1EcdsaSignature) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn ecdsa_signature_parse_der_lax(
        ctx:      *const Secp256k1Context,
        sig:      *mut Secp256k1EcdsaSignature,
        input:    *const u8,
        inputlen: usize) -> i32 {
    
    todo!();
        /*
        
        */
}

#[fuzz_test] fn secp256k1_ecdsa_signature_parse_der_lax() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        const std::vector<uint8_t> signature_bytes = ConsumeRandomLengthByteVector(fuzzed_data_provider);
        if (signature_bytes.data() == nullptr) {
            return;
        }
        secp256k1_context* secp256k1_context_verify = secp256k1_context_create(SECP256K1_CONTEXT_VERIFY);
        secp256k1_ecdsa_signature sig_der_lax;
        const bool parsed_der_lax = ecdsa_signature_parse_der_lax(secp256k1_context_verify, &sig_der_lax, signature_bytes.data(), signature_bytes.size()) == 1;
        if (parsed_der_lax) {
            ECC_Start();
            (c_void)SigHasLowR(&sig_der_lax);
            ECC_Stop();
        }
        secp256k1_context_destroy(secp256k1_context_verify);

    */
}
