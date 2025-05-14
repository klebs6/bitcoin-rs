// ---------------- [ File: bitcoin-fuzz/src/fuzz_base_encode_decode.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/base_encode_decode.cpp]

pub fn initialize_base_encode_decode()  {
    
    todo!();
        /*
            static const ECCVerifyHandle verify_handle;
        */
}

#[fuzz_test(initializer = "initialize_base_encode_decode")]
fn base_encode_decode() {
    todo!();
    /*
    
        const std::string random_encoded_string(buffer.begin(), buffer.end());

        std::vector<unsigned char> decoded;
        if (DecodeBase58(random_encoded_string, decoded, 100)) {
            const std::string encoded_string = EncodeBase58(decoded);
            assert(encoded_string == TrimString(encoded_string));
            assert(ToLower(encoded_string) == ToLower(TrimString(random_encoded_string)));
        }

        if (DecodeBase58Check(random_encoded_string, decoded, 100)) {
            const std::string encoded_string = EncodeBase58Check(decoded);
            assert(encoded_string == TrimString(encoded_string));
            assert(ToLower(encoded_string) == ToLower(TrimString(random_encoded_string)));
        }

        bool pf_invalid;
        std::string decoded_string = DecodeBase32(random_encoded_string, &pf_invalid);
        if (!pf_invalid) {
            const std::string encoded_string = EncodeBase32(decoded_string);
            assert(encoded_string == TrimString(encoded_string));
            assert(ToLower(encoded_string) == ToLower(TrimString(random_encoded_string)));
        }

        decoded_string = DecodeBase64(random_encoded_string, &pf_invalid);
        if (!pf_invalid) {
            const std::string encoded_string = EncodeBase64(decoded_string);
            assert(encoded_string == TrimString(encoded_string));
            assert(ToLower(encoded_string) == ToLower(TrimString(random_encoded_string)));
        }

        PartiallySignedTransaction psbt;
        std::string error;
        (c_void)DecodeBase64PSBT(psbt, random_encoded_string, error);

    */
}
