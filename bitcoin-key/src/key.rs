crate::ix!();

/**
  | An encapsulated private key.
  |
  */
pub struct Key {

    /**
      | Whether this private key is valid. We
      | check for correctness when modifying the
      | key data, so fValid should always
      | correspond to the actual state.
      */
    valid:      bool,

    /**
      Whether the public key corresponding to this
      private key is (to be) compressed.
      */
    compressed: bool,

    /**
      The actual byte data
      */
    keydata:    Vec<u8,SecureAllocator>,
}

impl Key {

    /**
      | Initialize using begin and end iterators
      | to byte data.
      |
      */
    pub fn set<T>(&mut self, 
        pbegin:        T,
        pend:          T,
        compressed_in: bool)  {
    
        todo!();
        /*
            if (size_t(pend - pbegin) != keydata.size()) {
                fValid = false;
            } else if (Check(&pbegin[0])) {
                memcpy(keydata.data(), (unsigned char*)&pbegin[0], keydata.size());
                fValid = true;
                fCompressed = fCompressedIn;
            } else {
                fValid = false;
            }
        */
    }

    /**
      | Simple read-only vector-like interface.
      |
      */
    pub fn size(&self) -> u32 {
        
        todo!();
        /*
            return (fValid ? keydata.size() : 0);
        */
    }
    
    pub fn begin(&self) -> *const u8 {
        
        todo!();
        /*
            return keydata.data();
        */
    }
    
    pub fn end(&self) -> *const u8 {
        
        todo!();
        /*
            return keydata.data() + size();
        */
    }

    /**
      | Check whether this private key is valid.
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return fValid;
        */
    }

    /**
      | Check whether the public key corresponding
      | to this private key is (to be) compressed.
      |
      */
    pub fn is_compressed(&self) -> bool {
        
        todo!();
        /*
            return fCompressed;
        */
    }

    /**
      | Check whether the 32-byte array pointed
      | to by vch is valid keydata.
      |
      */
    pub fn check(&mut self, vch: *const u8) -> bool {
        
        todo!();
        /*
            return secp256k1_ec_seckey_verify(secp256k1_context_sign, vch);
        */
    }
    
    /**
      | Generate a new private key using a cryptographic
      | PRNG.
      |
      */
    pub fn make_new_key(&mut self, compressed_in: bool)  {
        
        todo!();
        /*
            do {
            GetStrongRandBytes(keydata.data(), keydata.size());
        } while (!Check(keydata.data()));
        fValid = true;
        fCompressed = fCompressedIn;
        */
    }
    
    /**
      | Negate private key
      |
      */
    pub fn negate(&mut self) -> bool {
        
        todo!();
        /*
            assert(fValid);
        return secp256k1_ec_seckey_negate(secp256k1_context_sign, keydata.data());
        */
    }
    
    /**
      | Convert the private key to a CPrivKey
      | (serialized OpenSSL private key data).
      | 
      | This is expensive.
      |
      */
    pub fn get_priv_key(&self) -> PrivKey {
        
        todo!();
        /*
            assert(fValid);
        CPrivKey seckey;
        int ret;
        size_t seckeylen;
        seckey.resize(SIZE);
        seckeylen = SIZE;
        ret = ec_seckey_export_der(secp256k1_context_sign, seckey.data(), &seckeylen, begin(), fCompressed);
        assert(ret);
        seckey.resize(seckeylen);
        return seckey;
        */
    }
    
    /**
      | Compute the public key from a private
      | key.
      | 
      | This is expensive.
      |
      */
    pub fn get_pub_key(&self) -> crate::PubKey {
        
        todo!();
        /*
            assert(fValid);
        secp256k1_pubkey pubkey;
        size_t clen = CPubKey::SIZE;
        CPubKey result;
        int ret = secp256k1_ec_pubkey_create(secp256k1_context_sign, &pubkey, begin());
        assert(ret);
        secp256k1_ec_pubkey_serialize(secp256k1_context_sign, (unsigned char*)result.begin(), &clen, &pubkey, fCompressed ? SECP256K1_EC_COMPRESSED : SECP256K1_EC_UNCOMPRESSED);
        assert(result.size() == clen);
        assert(result.IsValid());
        return result;
        */
    }

    /**
      | Create a DER-serialized signature.
      | 
      | The test_case parameter tweaks the
      | deterministic nonce.
      |
      */
    pub fn sign(&self, 
        hash:      &u256,
        vch_sig:   &mut Vec<u8>,
        grind:     Option<bool>,
        test_case: Option<u32>) -> bool {

        let grind:     bool = grind.unwrap_or(true);
        let test_case:  u32 = test_case.unwrap_or(0);
        
        todo!();
        /*
            if (!fValid)
            return false;
        vchSig.resize(CPubKey::SIGNATURE_SIZE);
        size_t nSigLen = CPubKey::SIGNATURE_SIZE;
        unsigned char extra_entropy[32] = {0};
        WriteLE32(extra_entropy, test_case);
        secp256k1_ecdsa_signature sig;
        uint32_t counter = 0;
        int ret = secp256k1_ecdsa_sign(secp256k1_context_sign, &sig, hash.begin(), begin(), secp256k1_nonce_function_rfc6979, (!grind && test_case) ? extra_entropy : nullptr);

        // Grind for low R
        while (ret && !SigHasLowR(&sig) && grind) {
            WriteLE32(extra_entropy, ++counter);
            ret = secp256k1_ecdsa_sign(secp256k1_context_sign, &sig, hash.begin(), begin(), secp256k1_nonce_function_rfc6979, extra_entropy);
        }
        assert(ret);
        secp256k1_ecdsa_signature_serialize_der(secp256k1_context_sign, vchSig.data(), &nSigLen, &sig);
        vchSig.resize(nSigLen);
        return true;
        */
    }
    
    /**
      | Verify thoroughly whether a private
      | key and a public key match.
      | 
      | This is done using a different mechanism
      | than just regenerating it.
      |
      */
    pub fn verify_pub_key(&self, pubkey: &crate::PubKey) -> bool {
        
        todo!();
        /*
            if (pubkey.IsCompressed() != fCompressed) {
            return false;
        }
        unsigned char rnd[8];
        std::string str = "Bitcoin key verification\n";
        GetRandBytes(rnd, sizeof(rnd));
        uint256 hash;
        CHash256().Write(MakeUCharSpan(str)).Write(rnd).Finalize(hash);
        std::vector<unsigned char> vchSig;
        Sign(hash, vchSig);
        return pubkey.Verify(hash, vchSig);
        */
    }
    
    /**
      | Create a compact signature (65 bytes),
      | which allows reconstructing the used
      | public key.
      | 
      | The format is one header byte, followed
      | by two times 32 bytes for the serialized
      | r and s values.
      | 
      | The header byte: 0x1B = first key with
      | even y, 0x1C = first key with odd y, 0x1D
      | = second key with even y, 0x1E = second
      | key with odd y, add 0x04 for compressed
      | keys.
      |
      */
    pub fn sign_compact(&self, 
        hash:    &u256,
        vch_sig: &mut Vec<u8>) -> bool {
        
        todo!();
        /*
            if (!fValid)
            return false;
        vchSig.resize(CPubKey::COMPACT_SIGNATURE_SIZE);
        int rec = -1;
        secp256k1_ecdsa_recoverable_signature sig;
        int ret = secp256k1_ecdsa_sign_recoverable(secp256k1_context_sign, &sig, hash.begin(), begin(), secp256k1_nonce_function_rfc6979, nullptr);
        assert(ret);
        ret = secp256k1_ecdsa_recoverable_signature_serialize_compact(secp256k1_context_sign, &vchSig[1], &rec, &sig);
        assert(ret);
        assert(rec != -1);
        vchSig[0] = 27 + rec + (fCompressed ? 4 : 0);
        return true;
        */
    }
    
    /**
      | Create a BIP-340 Schnorr signature,
      | for the xonly-pubkey corresponding
      | to *this, optionally tweaked by *merkle_root.
      | Additional nonce entropy can be provided
      | through aux. merkle_root is used to
      | optionally perform tweaking of the
      | private key, as specified in BIP341:
      | 
      | - If merkle_root == nullptr: no tweaking
      | is done, sign with key directly (this
      | is used for signatures in BIP342 script).
      | 
      | - If merkle_root->IsNull(): sign with
      | key + H_TapTweak(pubkey) (this is used
      | for key path spending when no scripts
      | are present).
      | 
      | - Otherwise: sign with key + H_TapTweak(pubkey
      | || *merkle_root) (this is used for key
      | path spending, with specific
      | 
      | Merkle root of the script tree).
      |
      */
    pub fn sign_schnorr(&self, 
        hash:        &u256,
        sig:         &[u8],
        merkle_root: *const u256,
        aux:         *const u256) -> bool {
        
        todo!();
        /*
            assert(sig.size() == 64);
        secp256k1_keypair keypair;
        if (!secp256k1_keypair_create(secp256k1_context_sign, &keypair, begin())) return false;
        if (merkle_root) {
            secp256k1_xonly_pubkey pubkey;
            if (!secp256k1_keypair_xonly_pub(secp256k1_context_sign, &pubkey, nullptr, &keypair)) return false;
            unsigned char pubkey_bytes[32];
            if (!secp256k1_xonly_pubkey_serialize(secp256k1_context_sign, pubkey_bytes, &pubkey)) return false;
            uint256 tweak = XOnlyPubKey(pubkey_bytes).ComputeTapTweakHash(merkle_root->IsNull() ? nullptr : merkle_root);
            if (!secp256k1_keypair_xonly_tweak_add(GetVerifyContext(), &keypair, tweak.data())) return false;
        }
        bool ret = secp256k1_schnorrsig_sign(secp256k1_context_sign, sig.data(), hash.data(), &keypair, aux ? (unsigned char*)aux->data() : nullptr);
        memory_cleanse(&keypair, sizeof(keypair));
        return ret;
        */
    }
    
    /**
      | Load private key and check that public
      | key matches.
      |
      */
    pub fn load(&mut self, 
        seckey:      &PrivKey,
        vch_pub_key: &crate::PubKey,
        skip_check:  Option<bool>) -> bool {
        let skip_check: bool = skip_check.unwrap_or(false);

        todo!();
        /*
            if (!ec_seckey_import_der(secp256k1_context_sign, (unsigned char*)begin(), seckey.data(), seckey.size()))
            return false;
        fCompressed = vchPubKey.IsCompressed();
        fValid = true;

        if (fSkipCheck)
            return true;

        return VerifyPubKey(vchPubKey);
        */
    }
    
    /**
      | Derive BIP32 child key.
      |
      */
    pub fn derive(&self, 
        key_child: &mut Key,
        cc_child:  &mut ChainCode,
        n_child:   u32,
        cc:        &ChainCode) -> bool {
        
        todo!();
        /*
            assert(IsValid());
        assert(IsCompressed());
        std::vector<unsigned char, secure_allocator<unsigned char>> vout(64);
        if ((nChild >> 31) == 0) {
            CPubKey pubkey = GetPubKey();
            assert(pubkey.size() == CPubKey::COMPRESSED_SIZE);
            BIP32Hash(cc, nChild, *pubkey.begin(), pubkey.begin()+1, vout.data());
        } else {
            assert(size() == 32);
            BIP32Hash(cc, nChild, 0, begin(), vout.data());
        }
        memcpy(ccChild.begin(), vout.data()+32, 32);
        memcpy((unsigned char*)keyChild.begin(), begin(), 32);
        bool ret = secp256k1_ec_seckey_tweak_add(secp256k1_context_sign, (unsigned char*)keyChild.begin(), vout.data());
        keyChild.fCompressed = true;
        keyChild.fValid = ret;
        return ret;
        */
    }
}

impl Default for Key {
    
    /**
       Construct an invalid private key.
      */
    fn default() -> Self {
        todo!();
        /*
        : valid(false),
        : compressed(false),

            // Important: vch must be 32 bytes in length to not break serialization
            keydata.resize(32);
        */
    }
}

impl PartialEq<Key> for Key {
    
    #[inline] fn eq(&self, other: &Key) -> bool {
        todo!();
        /*
            return a.fCompressed == b.fCompressed &&
                a.size() == b.size() &&
                memcmp(a.keydata.data(), b.keydata.data(), a.size()) == 0;
        */
    }
}

impl Eq for Key {}

