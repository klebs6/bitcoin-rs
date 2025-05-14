// ---------------- [ File: bitcoin-fuzz/src/fuzz_key_io.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/key_io.cpp]

pub fn initialize_key_io()  {
    
    todo!();
        /*
            static const ECCVerifyHandle verify_handle;
        ECC_Start();
        SelectParams(CBaseChainParams::MAIN);
        */
}

#[fuzz_test(initializer = "initialize_key_io")]
fn key_io() {
    todo!();
    /*
    
        const std::string random_string(buffer.begin(), buffer.end());

        const CKey key = DecodeSecret(random_string);
        if (key.IsValid()) {
            assert(key == DecodeSecret(EncodeSecret(key)));
        }

        const CExtKey ext_key = DecodeExtKey(random_string);
        if (ext_key.key.size() == 32) {
            assert(ext_key == DecodeExtKey(EncodeExtKey(ext_key)));
        }

        const CExtPubKey ext_pub_key = DecodeExtPubKey(random_string);
        if (ext_pub_key.pubkey.size() == CPubKey::COMPRESSED_SIZE) {
            assert(ext_pub_key == DecodeExtPubKey(EncodeExtPubKey(ext_pub_key)));
        }

    */
}
