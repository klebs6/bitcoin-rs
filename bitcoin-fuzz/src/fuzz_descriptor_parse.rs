crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/descriptor_parse.cpp]

pub fn initialize_descriptor_parse()  {
    
    todo!();
        /*
            static const ECCVerifyHandle verify_handle;
        ECC_Start();
        SelectParams(CBaseChainParams::MAIN);
        */
}

#[fuzz_test(initializer = "initialize_descriptor_parse")]
fn descriptor_parse() {
    todo!();
    /*
    
        const std::string descriptor(buffer.begin(), buffer.end());
        FlatSigningProvider signing_provider;
        std::string error;
        for (const bool require_checksum : {true, false}) {
            const auto desc = Parse(descriptor, signing_provider, error, require_checksum);
            if (desc) {
                (c_void)desc->ToString();
                (c_void)desc->IsRange();
                (c_void)desc->IsSolvable();
            }
        }

    */
}
