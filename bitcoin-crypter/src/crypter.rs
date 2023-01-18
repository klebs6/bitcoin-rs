/*! 
  | Private key encryption is done based on
  | a CMasterKey, which holds a salt and random
  | encryption key.
  |
  | CMasterKeys are encrypted using AES-256-CBC
  | using a key derived using derivation method
  | nDerivationMethod (0 == EVP_sha512()) and
  | derivation iterations nDeriveIterations.
  | vchOtherDerivationParameters is provided for
  | alternative algorithms which may require more
  | parameters (such as scrypt).
  |
  | Wallet Private Keys are then encrypted using
  | AES-256-CBC with the double-sha256 of the
  | public key as the IV, and the master key's key
  | as the encryption key (see keystore.[ch]).
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/crypter.h]
//-------------------------------------------[.cpp/bitcoin/src/wallet/crypter.cpp]

pub const WALLET_CRYPTO_KEY_SIZE:  u32 = 32;
pub const WALLET_CRYPTO_SALT_SIZE: u32 = 8;
pub const WALLET_CRYPTO_IV_SIZE:   u32 = 16;

pub type KeyingMaterial = Vec<u8,SecureAllocator>;

/**
  | Encryption/decryption context with
  | key information
  |
  */
pub struct Crypter {
    vch_key: Vec<u8,SecureAllocator>,
    vchiv:   Vec<u8,SecureAllocator>,
    key_set: bool,
}

impl Default for Crypter {
    
    fn default() -> Self {
        todo!();
        /*


            fKeySet = false;
            vchKey.resize(WALLET_CRYPTO_KEY_SIZE);
            vchIV.resize(WALLET_CRYPTO_IV_SIZE);
        */
    }
}

impl Drop for Crypter {
    fn drop(&mut self) {
        todo!();
        /*
            CleanKey();
        */
    }
}

impl Crypter {
    
    pub fn clean_key(&mut self)  {
        
        todo!();
        /*
            memory_cleanse(vchKey.data(), vchKey.size());
            memory_cleanse(vchIV.data(), vchIV.size());
            fKeySet = false;
        */
    }
    
    pub fn bytes_to_keysha512aes(&self, 
        ch_salt:      &Vec<u8>,
        str_key_data: &SecureString,
        count:        i32,
        key:          *mut u8,
        iv:           *mut u8) -> i32 {
        
        todo!();
        /*
            // This mimics the behavior of openssl's EVP_BytesToKey with an aes256cbc
        // cipher and sha512 message digest. Because sha512's output size (64b) is
        // greater than the aes256 block size (16b) + aes256 key size (32b),
        // there's no need to process more than once (D_0).

        if(!count || !key || !iv)
            return 0;

        unsigned char buf[CSHA512::OUTPUT_SIZE];
        CSHA512 di;

        di.Write((const unsigned char*)strKeyData.data(), strKeyData.size());
        di.Write(chSalt.data(), chSalt.size());
        di.Finalize(buf);

        for(int i = 0; i != count - 1; i++)
            di.Reset().Write(buf, sizeof(buf)).Finalize(buf);

        memcpy(key, buf, WALLET_CRYPTO_KEY_SIZE);
        memcpy(iv, buf + WALLET_CRYPTO_KEY_SIZE, WALLET_CRYPTO_IV_SIZE);
        memory_cleanse(buf, sizeof(buf));
        return WALLET_CRYPTO_KEY_SIZE;
        */
    }
    
    pub fn set_key_from_passphrase(&mut self, 
        str_key_data:        &SecureString,
        ch_salt:             &Vec<u8>,
        n_rounds:            u32,
        n_derivation_method: u32) -> bool {
        
        todo!();
        /*
            if (nRounds < 1 || chSalt.size() != WALLET_CRYPTO_SALT_SIZE)
            return false;

        int i = 0;
        if (nDerivationMethod == 0)
            i = BytesToKeySHA512AES(chSalt, strKeyData, nRounds, vchKey.data(), vchIV.data());

        if (i != (int)WALLET_CRYPTO_KEY_SIZE)
        {
            memory_cleanse(vchKey.data(), vchKey.size());
            memory_cleanse(vchIV.data(), vchIV.size());
            return false;
        }

        fKeySet = true;
        return true;
        */
    }
    
    pub fn set_key(&mut self, 
        ch_new_key: &KeyingMaterial,
        ch_newiv:   &Vec<u8>) -> bool {
        
        todo!();
        /*
            if (chNewKey.size() != WALLET_CRYPTO_KEY_SIZE || chNewIV.size() != WALLET_CRYPTO_IV_SIZE)
            return false;

        memcpy(vchKey.data(), chNewKey.data(), chNewKey.size());
        memcpy(vchIV.data(), chNewIV.data(), chNewIV.size());

        fKeySet = true;
        return true;
        */
    }
    
    pub fn encrypt(&self, 
        vch_plaintext:  &KeyingMaterial,
        vch_ciphertext: &mut Vec<u8>) -> bool {
        
        todo!();
        /*
            if (!fKeySet)
            return false;

        // max ciphertext len for a n bytes of plaintext is
        // n + AES_BLOCKSIZE bytes
        vchCiphertext.resize(vchPlaintext.size() + AES_BLOCKSIZE);

        AES256CBCEncrypt enc(vchKey.data(), vchIV.data(), true);
        size_t nLen = enc.Encrypt(vchPlaintext.data(), vchPlaintext.size(), vchCiphertext.data());
        if(nLen < vchPlaintext.size())
            return false;
        vchCiphertext.resize(nLen);

        return true;
        */
    }
    
    pub fn decrypt(&self, 
        vch_ciphertext: &Vec<u8>,
        vch_plaintext:  &mut KeyingMaterial) -> bool {
        
        todo!();
        /*
            if (!fKeySet)
            return false;

        // plaintext will always be equal to or lesser than length of ciphertext
        int nLen = vchCiphertext.size();

        vchPlaintext.resize(nLen);

        AES256CBCDecrypt dec(vchKey.data(), vchIV.data(), true);
        nLen = dec.Decrypt(vchCiphertext.data(), vchCiphertext.size(), vchPlaintext.data());
        if(nLen == 0)
            return false;
        vchPlaintext.resize(nLen);
        return true;
        */
    }
}

pub fn encrypt_secret(
        master_key:     &KeyingMaterial,
        vch_plaintext:  &KeyingMaterial,
        niv:            &u256,
        vch_ciphertext: &mut Vec<u8>) -> bool {
    
    todo!();
        /*
            CCrypter cKeyCrypter;
        std::vector<unsigned char> chIV(WALLET_CRYPTO_IV_SIZE);
        memcpy(chIV.data(), &nIV, WALLET_CRYPTO_IV_SIZE);
        if(!cKeyCrypter.SetKey(vMasterKey, chIV))
            return false;
        return cKeyCrypter.Encrypt(*((const CKeyingMaterial*)&vchPlaintext), vchCiphertext);
        */
}

pub fn decrypt_secret(
        master_key:     &KeyingMaterial,
        vch_ciphertext: &Vec<u8>,
        niv:            &u256,
        vch_plaintext:  &mut KeyingMaterial) -> bool {
    
    todo!();
        /*
            CCrypter cKeyCrypter;
        std::vector<unsigned char> chIV(WALLET_CRYPTO_IV_SIZE);
        memcpy(chIV.data(), &nIV, WALLET_CRYPTO_IV_SIZE);
        if(!cKeyCrypter.SetKey(vMasterKey, chIV))
            return false;
        return cKeyCrypter.Decrypt(vchCiphertext, vchPlaintext);
        */
}

pub fn decrypt_key(
        master_key:         &KeyingMaterial,
        vch_crypted_secret: &Vec<u8>,
        vch_pub_key:        &PubKey,
        key:                &mut Key) -> bool {
    
    todo!();
        /*
            CKeyingMaterial vchSecret;
        if(!DecryptSecret(vMasterKey, vchCryptedSecret, vchPubKey.GetHash(), vchSecret))
            return false;

        if (vchSecret.size() != 32)
            return false;

        key.Set(vchSecret.begin(), vchSecret.end(), vchPubKey.IsCompressed());
        return key.VerifyPubKey(vchPubKey);
        */
}
