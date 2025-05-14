// ---------------- [ File: bitcoin-fuzz/src/fuzz_crypto_aes256.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/crypto_aes256.cpp]

#[fuzz_test] fn crypto_aes256() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        const std::vector<uint8_t> key = ConsumeFixedLengthByteVector(fuzzed_data_provider, AES256_KEYSIZE);

        AES256Encrypt encrypt{key.data()};
        AES256Decrypt decrypt{key.data()};

        while (fuzzed_data_provider.ConsumeBool()) {
            const std::vector<uint8_t> plaintext = ConsumeFixedLengthByteVector(fuzzed_data_provider, AES_BLOCKSIZE);
            std::vector<uint8_t> ciphertext(AES_BLOCKSIZE);
            encrypt.Encrypt(ciphertext.data(), plaintext.data());
            std::vector<uint8_t> decrypted_plaintext(AES_BLOCKSIZE);
            decrypt.Decrypt(decrypted_plaintext.data(), ciphertext.data());
            assert(decrypted_plaintext == plaintext);
        }

    */
}
