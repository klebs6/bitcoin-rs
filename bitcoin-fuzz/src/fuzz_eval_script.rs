// ---------------- [ File: bitcoin-fuzz/src/fuzz_eval_script.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/eval_script.cpp]

pub fn initialize_eval_script()  {
    
    todo!();
        /*
            static const ECCVerifyHandle verify_handle;
        */
}

#[fuzz_test(initializer = "initialize_eval_script")]
fn eval_script() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const unsigned int flags = fuzzed_data_provider.ConsumeIntegral<unsigned int>();
        const std::vector<uint8_t> script_bytes = [&] {
            if (fuzzed_data_provider.remaining_bytes() != 0) {
                return fuzzed_data_provider.ConsumeRemainingBytes<uint8_t>();
            } else {
                // Avoid UBSan warning:
                //   test/fuzz/FuzzedDataProvider.h:212:17: runtime error: null pointer passed as argument 1, which is declared to never be null
                //   /usr/include/string.h:43:28: note: nonnull attribute specified here
                return std::vector<uint8_t>();
            }
        }();
        const CScript script(script_bytes.begin(), script_bytes.end());
        for (const auto sig_version : {SigVersion::BASE, SigVersion::WITNESS_V0}) {
            std::vector<std::vector<unsigned char>> stack;
            (c_void)EvalScript(stack, script, flags, BaseSignatureChecker(), sig_version, nullptr);
        }

    */
}
