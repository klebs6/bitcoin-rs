crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/signature_checker.cpp]

pub fn initialize_signature_checker()  {
    
    todo!();
        /*
            static const auto verify_handle = std::make_unique<ECCVerifyHandle>();
        */
}

pub struct FuzzedSignatureChecker {
    fuzzed_data_provider: Rc<RefCell<FuzzedDataProvider>>,
}

impl BaseSignatureChecker for FuzzedSignatureChecker {

}

impl FuzzedSignatureChecker {

    pub fn new(fuzzed_data_provider: &mut FuzzedDataProvider) -> Self {
    
        todo!();
        /*
        : fuzzed_data_provider(fuzzed_data_provider),

        
        */
    }
    
    pub fn check_ecdsa_signature(&self, 
        script_sig:  &Vec<u8>,
        vch_pub_key: &Vec<u8>,
        script_code: &Script,
        sigversion:  SigVersion) -> bool {
        
        todo!();
        /*
            return m_fuzzed_data_provider.ConsumeBool();
        */
    }
    
    pub fn check_schnorr_signature(&self, 
        sig:        &[u8],
        pubkey:     &[u8],
        sigversion: SigVersion,
        execdata:   &ScriptExecutionData,
        serror:     Option<*mut ScriptError>) -> bool {

        todo!();
        /*
            return m_fuzzed_data_provider.ConsumeBool();
        */
    }
    
    pub fn check_lock_time(&self, n_lock_time: &ScriptNum) -> bool {
        
        todo!();
        /*
            return m_fuzzed_data_provider.ConsumeBool();
        */
    }
    
    pub fn check_sequence(&self, n_sequence: &ScriptNum) -> bool {
        
        todo!();
        /*
            return m_fuzzed_data_provider.ConsumeBool();
        */
    }
}

#[fuzz_test(initializer = "initialize_signature_checker")]
fn signature_checker() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const unsigned int flags = fuzzed_data_provider.ConsumeIntegral<unsigned int>();
        const SigVersion sig_version = fuzzed_data_provider.PickValueInArray({SigVersion::BASE, SigVersion::WITNESS_V0});
        const auto script_1 = ConsumeScript(fuzzed_data_provider, 65536);
        const auto script_2 = ConsumeScript(fuzzed_data_provider, 65536);
        std::vector<std::vector<unsigned char>> stack;
        (c_void)EvalScript(stack, script_1, flags, FuzzedSignatureChecker(fuzzed_data_provider), sig_version, nullptr);
        if (!IsValidFlagCombination(flags)) {
            return;
        }
        (c_void)VerifyScript(script_1, script_2, nullptr, flags, FuzzedSignatureChecker(fuzzed_data_provider), nullptr);

    */
}
