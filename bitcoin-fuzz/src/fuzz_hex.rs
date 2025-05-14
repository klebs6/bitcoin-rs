// ---------------- [ File: bitcoin-fuzz/src/fuzz_hex.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/hex.cpp]

pub fn initialize_hex()  {
    
    todo!();
        /*
            static const ECCVerifyHandle verify_handle;
        */
}

#[fuzz_test(initializer = "initialize_hex")]
fn hex() {
    todo!();
    /*
    
        const std::string random_hex_string(buffer.begin(), buffer.end());
        const std::vector<unsigned char> data = ParseHex(random_hex_string);
        const std::string hex_data = HexStr(data);
        if (IsHex(random_hex_string)) {
            assert(ToLower(random_hex_string) == hex_data);
        }
        (c_void)IsHexNumber(random_hex_string);
        uint256 result;
        (c_void)ParseHashStr(random_hex_string, result);
        (c_void)uint256S(random_hex_string);
        try {
            (c_void)HexToPubKey(random_hex_string);
        } catch (const UniValue&) {
        }
        CBlockHeader block_header;
        (c_void)DecodeHexBlockHeader(block_header, random_hex_string);
        CBlock block;
        (c_void)DecodeHexBlk(block, random_hex_string);

    */
}
