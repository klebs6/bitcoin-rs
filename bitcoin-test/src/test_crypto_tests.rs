// ---------------- [ File: bitcoin-test/src/test_crypto_tests.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/wallet/test/wallet_crypto_tests.cpp]

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod wallet_crypto_tests {

    pub struct TestCrypter { }

    impl TestCrypter {

        pub fn test_passphrase_single(
            vch_salt:    &Vec<u8>,
            passphrase:  &SecureString,
            rounds:      u32,
            correct_key: &Vec<u8>,
            correctiv:   &Vec<u8>)  {
            
            todo!();
            /*
                CCrypter crypt;
            crypt.SetKeyFromPassphrase(passphrase, vchSalt, rounds, 0);

            if(!correctKey.empty())
                BOOST_CHECK_MESSAGE(memcmp(crypt.vchKey.data(), correctKey.data(), crypt.vchKey.size()) == 0, \
                    HexStr(crypt.vchKey) + std::string(" != ") + HexStr(correctKey));
            if(!correctIV.empty())
                BOOST_CHECK_MESSAGE(memcmp(crypt.vchIV.data(), correctIV.data(), crypt.vchIV.size()) == 0,
                    HexStr(crypt.vchIV) + std::string(" != ") + HexStr(correctIV));
            */
        }
        
        pub fn test_passphrase(
            vch_salt:    &Vec<u8>,
            passphrase:  &SecureString,
            rounds:      u32,
            correct_key: &Vec<u8>,
            correctiv:   &Vec<u8>)  {
            
            todo!();
            /*
                TestPassphraseSingle(vchSalt, passphrase, rounds, correctKey, correctIV);
            for(SecureString::const_iterator i(passphrase.begin()); i != passphrase.end(); ++i)
                TestPassphraseSingle(vchSalt, SecureString(i, passphrase.end()), rounds);
            */
        }
        
        pub fn test_decrypt(
            crypt:          &Crypter,
            vch_ciphertext: &Vec<u8>,
            vch_plaintext:  &Vec<u8>)  {
            
            todo!();
            /*
                CKeyingMaterial vchDecrypted;
            crypt.Decrypt(vchCiphertext, vchDecrypted);
            if (vchPlaintext.size())
                BOOST_CHECK(CKeyingMaterial(vchPlaintext.begin(), vchPlaintext.end()) == vchDecrypted);
            */
        }
        
        pub fn test_encrypt_single(
            crypt:                  &Crypter,
            vch_plaintext:          &KeyingMaterial,
            vch_ciphertext_correct: &Vec<u8>)  {
            
            todo!();
            /*
                std::vector<unsigned char> vchCiphertext;
            crypt.Encrypt(vchPlaintext, vchCiphertext);

            if (!vchCiphertextCorrect.empty())
                BOOST_CHECK(vchCiphertext == vchCiphertextCorrect);

            const std::vector<unsigned char> vchPlaintext2(vchPlaintext.begin(), vchPlaintext.end());
            TestDecrypt(crypt, vchCiphertext, vchPlaintext2);
            */
        }
        
        pub fn test_encrypt(
            crypt:                  &Crypter,
            vch_plaintext_in:       &Vec<u8>,
            vch_ciphertext_correct: &Vec<u8>)  {
            
            todo!();
            /*
                TestEncryptSingle(crypt, CKeyingMaterial(vchPlaintextIn.begin(), vchPlaintextIn.end()), vchCiphertextCorrect);
            for(std::vector<unsigned char>::const_iterator i(vchPlaintextIn.begin()); i != vchPlaintextIn.end(); ++i)
                TestEncryptSingle(crypt, CKeyingMaterial(i, vchPlaintextIn.end()));
            */
        }
    }

    #[test] fn passphrase() {
        todo!();
        /*
        
            // These are expensive.

            TestCrypter::TestPassphrase(ParseHex("0000deadbeef0000"), "test", 25000, \
                                        ParseHex("fc7aba077ad5f4c3a0988d8daa4810d0d4a0e3bcb53af662998898f33df0556a"), \
                                        ParseHex("cf2f2691526dd1aa220896fb8bf7c369"));

            std::string hash(GetRandHash().ToString());
            std::vector<unsigned char> vchSalt(8);
            GetRandBytes(vchSalt.data(), vchSalt.size());
            uint32_t rounds = InsecureRand32();
            if (rounds > 30000)
                rounds = 30000;
            TestCrypter::TestPassphrase(vchSalt, SecureString(hash.begin(), hash.end()), rounds);

        */
    }

    #[test] fn encrypt() {
        todo!();
        /*
        
            std::vector<unsigned char> vchSalt = ParseHex("0000deadbeef0000");
            BOOST_CHECK(vchSalt.size() == WALLET_CRYPTO_SALT_SIZE);
            CCrypter crypt;
            crypt.SetKeyFromPassphrase("passphrase", vchSalt, 25000, 0);
            TestCrypter::TestEncrypt(crypt, ParseHex("22bcade09ac03ff6386914359cfe885cfeb5f77ff0d670f102f619687453b29d"));

            for (int i = 0; i != 100; i++)
            {
                uint256 hash(GetRandHash());
                TestCrypter::TestEncrypt(crypt, std::vector<unsigned char>(hash.begin(), hash.end()));
            }

        */
    }

    #[test] fn decrypt() {
        todo!();
        /*
        
            std::vector<unsigned char> vchSalt = ParseHex("0000deadbeef0000");
            BOOST_CHECK(vchSalt.size() == WALLET_CRYPTO_SALT_SIZE);
            CCrypter crypt;
            crypt.SetKeyFromPassphrase("passphrase", vchSalt, 25000, 0);

            // Some corner cases the came up while testing
            TestCrypter::TestDecrypt(crypt,ParseHex("795643ce39d736088367822cdc50535ec6f103715e3e48f4f3b1a60a08ef59ca"));
            TestCrypter::TestDecrypt(crypt,ParseHex("de096f4a8f9bd97db012aa9d90d74de8cdea779c3ee8bc7633d8b5d6da703486"));
            TestCrypter::TestDecrypt(crypt,ParseHex("32d0a8974e3afd9c6c3ebf4d66aa4e6419f8c173de25947f98cf8b7ace49449c"));
            TestCrypter::TestDecrypt(crypt,ParseHex("e7c055cca2faa78cb9ac22c9357a90b4778ded9b2cc220a14cea49f931e596ea"));
            TestCrypter::TestDecrypt(crypt,ParseHex("b88efddd668a6801d19516d6830da4ae9811988ccbaf40df8fbb72f3f4d335fd"));
            TestCrypter::TestDecrypt(crypt,ParseHex("8cae76aa6a43694e961ebcb28c8ca8f8540b84153d72865e8561ddd93fa7bfa9"));

            for (int i = 0; i != 100; i++)
            {
                uint256 hash(GetRandHash());
                TestCrypter::TestDecrypt(crypt, std::vector<unsigned char>(hash.begin(), hash.end()));
            }

        */
    }
}
