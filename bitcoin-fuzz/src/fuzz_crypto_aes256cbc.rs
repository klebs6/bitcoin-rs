// ---------------- [ File: bitcoin-fuzz/src/fuzz_crypto_aes256cbc.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/crypto_aes256cbc.cpp]

#[fuzz_test] fn crypto_aes256cbc() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        const std::vector<uint8_t> key = ConsumeFixedLengthByteVector(fuzzed_data_provider, AES256_KEYSIZE);
        const std::vector<uint8_t> iv = ConsumeFixedLengthByteVector(fuzzed_data_provider, AES_BLOCKSIZE);
        const bool pad = fuzzed_data_provider.ConsumeBool();

        AES256CBCEncrypt encrypt{key.data(), iv.data(), pad};
        AES256CBCDecrypt decrypt{key.data(), iv.data(), pad};

        while (fuzzed_data_provider.ConsumeBool()) {
            const std::vector<uint8_t> plaintext = ConsumeRandomLengthByteVector(fuzzed_data_provider);
            std::vector<uint8_t> ciphertext(plaintext.size() + AES_BLOCKSIZE);
            const int encrypt_ret = encrypt.Encrypt(plaintext.data(), plaintext.size(), ciphertext.data());
            ciphertext.resize(encrypt_ret);
            std::vector<uint8_t> decrypted_plaintext(ciphertext.size());
            const int decrypt_ret = decrypt.Decrypt(ciphertext.data(), ciphertext.size(), decrypted_plaintext.data());
            decrypted_plaintext.resize(decrypt_ret);
            assert(decrypted_plaintext == plaintext || (!pad && plaintext.size() % AES_BLOCKSIZE != 0 && encrypt_ret == 0 && decrypt_ret == 0));
        }

    */
}
