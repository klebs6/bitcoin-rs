crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/parse_univalue.cpp]

pub fn initialize_parse_univalue()  {
    
    todo!();
        /*
            static const ECCVerifyHandle verify_handle;
        SelectParams(CBaseChainParams::REGTEST);
        */
}

#[fuzz_test(initializer = "initialize_parse_univalue")]
fn parse_univalue() {
    todo!();
    /*
    
        const std::string random_string(buffer.begin(), buffer.end());
        bool valid = true;
        const UniValue univalue = [&] {
            try {
                return ParseNonRFCJSONValue(random_string);
            } catch (const std::runtime_error&) {
                valid = false;
                return NullUniValue;
            }
        }();
        if (!valid) {
            return;
        }
        try {
            (c_void)ParseHashO(univalue, "A");
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)ParseHashO(univalue, random_string);
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)ParseHashV(univalue, "A");
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)ParseHashV(univalue, random_string);
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)ParseHexO(univalue, "A");
        } catch (const UniValue&) {
        }
        try {
            (c_void)ParseHexO(univalue, random_string);
        } catch (const UniValue&) {
        }
        try {
            (c_void)ParseHexUV(univalue, "A");
            (c_void)ParseHexUV(univalue, random_string);
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)ParseHexV(univalue, "A");
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)ParseHexV(univalue, random_string);
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)ParseSighashString(univalue);
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)AmountFromValue(univalue);
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            FlatSigningProvider provider;
            (c_void)EvalDescriptorStringOrObject(univalue, provider);
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)ParseConfirmTarget(univalue, std::numeric_limits<unsigned int>::max());
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }
        try {
            (c_void)ParseDescriptorRange(univalue);
        } catch (const UniValue&) {
        } catch (const std::runtime_error&) {
        }

    */
}
