// ---------------- [ File: bitcoin-scriptpubkeyman/src/legacy_script_pubkey_man.rs ]
crate::ix!();

///-------------------
pub struct LegacyScriptPubKeyMan {
    base:  ScriptPubKeyMan,
    base2: FillableSigningProvider<LegacyScriptPubkeyManInner>,

    /**
      | keeps track of whether Unlock has run
      | a thorough check before
      |
      */
    decryption_thoroughly_checked: bool, // default = true


    /**
      | the HD chain data model (external chain
      | counters)
      |
      */
    hd_chain:              HDChain,

    inactive_hd_chains:    HashMap<KeyID,HDChain,SaltedSipHasher>,


    pool_key_to_index:     HashMap<KeyID,i64>,

    /**
      | Tracks keypool indexes to CKeyIDs of
      | keys that have been taken out of the keypool
      | but may be returned to it
      |
      */
    index_to_reserved_key: HashMap<i64,KeyID>,
}

pub struct LegacyScriptPubkeyManInner {

    encrypted_batch:       *mut WalletBatch, // default = nullptr
    map_crypted_keys:      LegacyScriptPubkeyManCryptedKeyMap,
    set_watch_only:        LegacyScriptPubkeyManWatchOnlySet,
    map_watch_keys:        LegacyScriptPubkeyManWatchKeyMap,
    n_time_first_key:      i64, // default = 0
    set_internal_key_pool: HashSet<i64>,
    set_external_key_pool: HashSet<i64>,
    set_pre_split_keypool: HashSet<i64>,
    max_keypool_index:     i64, // default = 0

    /**
      | Map from Key ID to key metadata.
      | 
      |
      */
    map_key_metadata:      HashMap<KeyID,KeyMetadata>,

    /**
      | Map from Script ID to key metadata (for
      | watch-only keys).
      | 
      |
      */
    script_metadata:       HashMap<ScriptID,KeyMetadata>,
}

pub type LegacyScriptPubkeyManCryptedKeyMap = HashMap<KeyID,(PubKey,Vec<u8>)>;
pub type LegacyScriptPubkeyManWatchOnlySet  = HashSet<Script>;
pub type LegacyScriptPubkeyManWatchKeyMap   = HashMap<KeyID,PubKey>;

impl LegacyScriptPubKeyMan {

    pub fn get_hd_chain(&self) -> &HDChain {
        
        todo!();
        /*
            return m_hd_chain;
        */
    }
    
    /* ------------ SigningProvider overrides  ------------ */
    pub fn get_all_reserve_keys(&self) -> &HashMap<KeyID,i64> {
        
        todo!();
        /*
            return m_pool_key_to_index;
        */
    }
    
    pub fn get_new_destination(&mut self, 
        ty:    OutputType,
        dest:  &mut TxDestination,
        error: &mut BilingualStr) -> bool {
        
        todo!();
        /*
            if (LEGACY_OUTPUT_TYPES.count(type) == 0) {
            error = _("Error: Legacy wallets only support the \"legacy\", \"p2sh-segwit\", and \"bech32\" address types");
            return false;
        }
        assert(type != OutputType::BECH32M);

        LOCK(cs_KeyStore);
        error.clear();

        // Generate a new key that is added to wallet
        CPubKey new_key;
        if (!GetKeyFromPool(new_key, type)) {
            error = _("Error: Keypool ran out, please call keypoolrefill first");
            return false;
        }
        LearnRelatedScripts(new_key, type);
        dest = GetDestinationForKey(new_key, type);
        return true;
        */
    }

    pub fn is_mine(&self, script: &Script) -> IsMineType {
        
        todo!();
        /*
            switch (IsMineInner(*this, script, IsMineSigVersion::TOP)) {
        case IsMineResult::INVALID:
        case IsMineResult::NO:
            return ISMINE_NO;
        case IsMineResult::WATCH_ONLY:
            return ISMINE_WATCH_ONLY;
        case IsMineResult::SPENDABLE:
            return ISMINE_SPENDABLE;
        }
        assert(false);
        */
    }
    
    pub fn check_decryption_key(&mut self, 
        master_key:     &KeyingMaterial,
        accept_no_keys: Option<bool>) -> bool {

        let accept_no_keys: bool = accept_no_keys.unwrap_or(false);
        
        todo!();
        /*
            {
            LOCK(cs_KeyStore);
            assert(mapKeys.empty());

            bool keyPass = mapCryptedKeys.empty(); // Always pass when there are no encrypted keys
            bool keyFail = false;
            CryptedKeyMap::const_iterator mi = mapCryptedKeys.begin();
            WalletBatch batch(m_storage.GetDatabase());
            for (; mi != mapCryptedKeys.end(); ++mi)
            {
                const CPubKey &vchPubKey = (*mi).second.first;
                const std::vector<unsigned char> &vchCryptedSecret = (*mi).second.second;
                CKey key;
                if (!DecryptKey(master_key, vchCryptedSecret, vchPubKey, key))
                {
                    keyFail = true;
                    break;
                }
                keyPass = true;
                if (fDecryptionThoroughlyChecked)
                    break;
                else {
                    // Rewrite these encrypted keys with checksums
                    batch.WriteCryptedKey(vchPubKey, vchCryptedSecret, mapKeyMetadata[vchPubKey.GetID()]);
                }
            }
            if (keyPass && keyFail)
            {
                LogPrintf("The wallet is probably corrupted: Some keys decrypt but not all.\n");
                throw std::runtime_error("Error unlocking wallet: some keys decrypt but not all. Your wallet file may be corrupt.");
            }
            if (keyFail || (!keyPass && !accept_no_keys))
                return false;
            fDecryptionThoroughlyChecked = true;
        }
        return true;
        */
    }
    
    pub fn encrypt(&mut self, 
        master_key: &KeyingMaterial,
        batch:      *mut WalletBatch) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        encrypted_batch = batch;
        if (!mapCryptedKeys.empty()) {
            encrypted_batch = nullptr;
            return false;
        }

        KeyMap keys_to_encrypt;
        keys_to_encrypt.swap(mapKeys); // Clear mapKeys so AddCryptedKeyInner will succeed.
        for (const KeyMap::value_type& mKey : keys_to_encrypt)
        {
            const CKey &key = mKey.second;
            CPubKey vchPubKey = key.GetPubKey();
            CKeyingMaterial vchSecret(key.begin(), key.end());
            std::vector<unsigned char> vchCryptedSecret;
            if (!EncryptSecret(master_key, vchSecret, vchPubKey.GetHash(), vchCryptedSecret)) {
                encrypted_batch = nullptr;
                return false;
            }
            if (!AddCryptedKey(vchPubKey, vchCryptedSecret)) {
                encrypted_batch = nullptr;
                return false;
            }
        }
        encrypted_batch = nullptr;
        return true;
        */
    }
    
    pub fn get_reserved_destination(&mut self, 
        ty:       OutputType,
        internal: bool,
        address:  &mut TxDestination,
        index:    &mut i64,
        keypool:  &mut KeyPool,
        error:    &mut BilingualStr) -> bool {
        
        todo!();
        /*
            if (LEGACY_OUTPUT_TYPES.count(type) == 0) {
            error = _("Error: Legacy wallets only support the \"legacy\", \"p2sh-segwit\", and \"bech32\" address types");
            return false;
        }
        assert(type != OutputType::BECH32M);

        LOCK(cs_KeyStore);
        if (!CanGetAddresses(internal)) {
            error = _("Error: Keypool ran out, please call keypoolrefill first");
            return false;
        }

        if (!ReserveKeyFromKeyPool(index, keypool, internal)) {
            error = _("Error: Keypool ran out, please call keypoolrefill first");
            return false;
        }
        address = GetDestinationForKey(keypool.vchPubKey, type);
        return true;
        */
    }
    
    /**
      | Like TopUp() but adds keys for inactive
      | HD chains.
      | 
      | Ensures that there are at least -keypool
      | number of keys derived after the given
      | index.
      | 
      | -----------
      | @param seed_id
      | 
      | the CKeyID for the HD seed.
      | ----------
      | @param index
      | 
      | the index to start generating keys from
      | ----------
      | @param internal
      | 
      | whether the internal chain should be
      | used. true for internal chain, false
      | for external chain.
      | 
      | -----------
      | @return
      | 
      | true if seed was found and keys were derived.
      | false if unable to derive seeds
      |
      */
    pub fn top_up_inactive_hd_chain(&mut self, 
        seed_id:  KeyID,
        index:    i64,
        internal: bool) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);

        if (m_storage.IsLocked()) return false;

        auto it = m_inactive_hd_chains.find(seed_id);
        if (it == m_inactive_hd_chains.end()) {
            return false;
        }

        CHDChain& chain = it->second;

        // Top up key pool
        int64_t target_size = std::max(gArgs.GetIntArg("-keypool", DEFAULT_KEYPOOL_SIZE), (int64_t) 1);

        // "size" of the keypools. Not really the size, actually the difference between index and the chain counter
        // Since chain counter is 1 based and index is 0 based, one of them needs to be offset by 1.
        int64_t kp_size = (internal ? chain.nInternalChainCounter : chain.nExternalChainCounter) - (index + 1);

        // make sure the keypool fits the user-selected target (-keypool)
        int64_t missing = std::max(target_size - kp_size, (int64_t) 0);

        if (missing > 0) {
            WalletBatch batch(m_storage.GetDatabase());
            for (int64_t i = missing; i > 0; --i) {
                GenerateNewKey(batch, chain, internal);
            }
            if (internal) {
                WalletLogPrintf("inactive seed with id %s added %d internal keys\n", HexStr(seed_id), missing);
            } else {
                WalletLogPrintf("inactive seed with id %s added %d keys\n", HexStr(seed_id), missing);
            }
        }
        return true;
        */
    }
    
    pub fn mark_unused_addresses(&mut self, script: &Script)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        // extract addresses and check if they match with an unused keypool key
        for (const auto& keyid : GetAffectedKeys(script, *this)) {
            std::map<CKeyID, int64_t>::const_iterator mi = m_pool_key_to_index.find(keyid);
            if (mi != m_pool_key_to_index.end()) {
                WalletLogPrintf("%s: Detected a used keypool key, mark all keypool keys up to this key as used\n", __func__);
                MarkReserveKeysAsUsed(mi->second);

                if (!TopUp()) {
                    WalletLogPrintf("%s: Topping up keypool failed (locked wallet)\n", __func__);
                }
            }

            // Find the key's metadata and check if it's seed id (if it has one) is inactive, i.e. it is not the current m_hd_chain seed id.
            // If so, TopUp the inactive hd chain
            auto it = mapKeyMetadata.find(keyid);
            if (it != mapKeyMetadata.end()){
                CKeyMetadata meta = it->second;
                if (!meta.hd_seed_id.IsNull() && meta.hd_seed_id != m_hd_chain.seed_id) {
                    bool internal = (meta.key_origin.path[1] & ~BIP32_HARDENED_KEY_LIMIT) != 0;
                    int64_t index = meta.key_origin.path[2] & ~BIP32_HARDENED_KEY_LIMIT;

                    if (!TopUpInactiveHDChain(meta.hd_seed_id, index, internal)) {
                        WalletLogPrintf("%s: Adding inactive seed keys failed\n", __func__);
                    }
                }
            }
        }
        */
    }
    
    /**
      | Upgrade stored CKeyMetadata objects
      | to store key origin info as KeyOriginInfo
      |
      */
    pub fn upgrade_key_metadata(&mut self)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        if (m_storage.IsLocked() || m_storage.IsWalletFlagSet(WALLET_FLAG_KEY_ORIGIN_METADATA)) {
            return;
        }

        std::unique_ptr<WalletBatch> batch = std::make_unique<WalletBatch>(m_storage.GetDatabase());
        for (auto& meta_pair : mapKeyMetadata) {
            CKeyMetadata& meta = meta_pair.second;
            if (!meta.hd_seed_id.IsNull() && !meta.has_key_origin && meta.hdKeypath != "s") { // If the hdKeypath is "s", that's the seed and it doesn't have a key origin
                CKey key;
                GetKey(meta.hd_seed_id, key);
                CExtKey masterKey;
                masterKey.SetSeed(key.begin(), key.size());
                // Add to map
                CKeyID master_id = masterKey.key.GetPubKey().GetID();
                std::copy(master_id.begin(), master_id.begin() + 4, meta.key_origin.fingerprint);
                if (!ParseHDKeypath(meta.hdKeypath, meta.key_origin.path)) {
                    throw std::runtime_error("Invalid stored hdKeypath");
                }
                meta.has_key_origin = true;
                if (meta.nVersion < CKeyMetadata::VERSION_WITH_KEY_ORIGIN) {
                    meta.nVersion = CKeyMetadata::VERSION_WITH_KEY_ORIGIN;
                }

                // Write meta to wallet
                CPubKey pubkey;
                if (GetPubKey(meta_pair.first, pubkey)) {
                    batch->WriteKeyMetadata(meta, pubkey, true);
                }
            }
        }
        */
    }
    
    pub fn setup_generation(&mut self, force: Option<bool>) -> bool {
        let force: bool = force.unwrap_or(false);
        
        todo!();
        /*
            if ((CanGenerateKeys() && !force) || m_storage.IsLocked()) {
            return false;
        }

        SetHDSeed(GenerateNewSeed());
        if (!NewKeyPool()) {
            return false;
        }
        return true;
        */
    }
    
    pub fn is_hd_enabled(&self) -> bool {
        
        todo!();
        /*
            return !m_hd_chain.seed_id.IsNull();
        */
    }
    
    pub fn can_get_addresses(&self, internal: Option<bool>) -> bool {

        let internal: bool = internal.unwrap_or(false);
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        // Check if the keypool has keys
        bool keypool_has_keys;
        if (internal && m_storage.CanSupportFeature(FEATURE_HD_SPLIT)) {
            keypool_has_keys = setInternalKeyPool.size() > 0;
        } else {
            keypool_has_keys = KeypoolCountExternalKeys() > 0;
        }
        // If the keypool doesn't have keys, check if we can generate them
        if (!keypool_has_keys) {
            return CanGenerateKeys();
        }
        return keypool_has_keys;
        */
    }
    
    pub fn upgrade(&mut self, 
        prev_version: i32,
        new_version:  i32,
        error:        &mut BilingualStr) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        bool hd_upgrade = false;
        bool split_upgrade = false;
        if (IsFeatureSupported(new_version, FEATURE_HD) && !IsHDEnabled()) {
            WalletLogPrintf("Upgrading wallet to HD\n");
            m_storage.SetMinVersion(FEATURE_HD);

            // generate a new master key
            CPubKey masterPubKey = GenerateNewSeed();
            SetHDSeed(masterPubKey);
            hd_upgrade = true;
        }
        // Upgrade to HD chain split if necessary
        if (!IsFeatureSupported(prev_version, FEATURE_HD_SPLIT) && IsFeatureSupported(new_version, FEATURE_HD_SPLIT)) {
            WalletLogPrintf("Upgrading wallet to use HD chain split\n");
            m_storage.SetMinVersion(FEATURE_PRE_SPLIT_KEYPOOL);
            split_upgrade = FEATURE_HD_SPLIT > prev_version;
            // Upgrade the HDChain
            if (m_hd_chain.nVersion < CHDChain::VERSION_HD_CHAIN_SPLIT) {
                m_hd_chain.nVersion = CHDChain::VERSION_HD_CHAIN_SPLIT;
                if (!WalletBatch(m_storage.GetDatabase()).WriteHDChain(m_hd_chain)) {
                    throw std::runtime_error(std::string(__func__) + ": writing chain failed");
                }
            }
        }
        // Mark all keys currently in the keypool as pre-split
        if (split_upgrade) {
            MarkPreSplitKeys();
        }
        // Regenerate the keypool if upgraded to HD
        if (hd_upgrade) {
            if (!NewKeyPool()) {
                error = _("Unable to generate keys");
                return false;
            }
        }
        return true;
        */
    }
    
    pub fn have_private_keys(&self) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        return !mapKeys.empty() || !mapCryptedKeys.empty();
        */
    }
    
    pub fn rewritedb(&mut self)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        setInternalKeyPool.clear();
        setExternalKeyPool.clear();
        m_pool_key_to_index.clear();
        // Note: can't top-up keypool here, because wallet is locked.
        // User will be prompted to unlock wallet the next operation
        // that requires a new key.
        */
    }
    
    pub fn get_oldest_key_pool_time(&self) -> i64 {
        
        todo!();
        /*
            LOCK(cs_KeyStore);

        WalletBatch batch(m_storage.GetDatabase());

        // load oldest key from keypool, get time and return
        int64_t oldestKey = GetOldestKeyTimeInPool(setExternalKeyPool, batch);
        if (IsHDEnabled() && m_storage.CanSupportFeature(FEATURE_HD_SPLIT)) {
            oldestKey = std::max(GetOldestKeyTimeInPool(setInternalKeyPool, batch), oldestKey);
            if (!set_pre_split_keypool.empty()) {
                oldestKey = std::max(GetOldestKeyTimeInPool(set_pre_split_keypool, batch), oldestKey);
            }
        }

        return oldestKey;
        */
    }
    
    pub fn keypool_count_external_keys(&self) -> usize {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        return setExternalKeyPool.size() + set_pre_split_keypool.size();
        */
    }
    
    pub fn get_key_pool_size(&self) -> u32 {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        return setInternalKeyPool.size() + setExternalKeyPool.size() + set_pre_split_keypool.size();
        */
    }
    
    pub fn get_time_first_key(&self) -> i64 {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        return nTimeFirstKey;
        */
    }
    
    pub fn get_solving_provider(&self, script: &Script) -> Box<SigningProvider> {
        
        todo!();
        /*
            return std::make_unique<LegacySigningProvider>(*this);
        */
    }
    
    pub fn can_provide(&mut self, 
        script:  &Script,
        sigdata: &mut SignatureData) -> bool {
        
        todo!();
        /*
            IsMineResult ismine = IsMineInner(*this, script, IsMineSigVersion::TOP, /* recurse_scripthash= */ false);
        if (ismine == IsMineResult::SPENDABLE || ismine == IsMineResult::WATCH_ONLY) {
            // If ismine, it means we recognize keys or script ids in the script, or
            // are watching the script itself, and we can at least provide metadata
            // or solving information, even if not able to sign fully.
            return true;
        } else {
            // If, given the stuff in sigdata, we could make a valid sigature, then we can provide for this script
            ProduceSignature(*this, DUMMY_SIGNATURE_CREATOR, script, sigdata);
            if (!sigdata.signatures.empty()) {
                // If we could make signatures, make sure we have a private key to actually make a signature
                bool has_privkeys = false;
                for (const auto& key_sig_pair : sigdata.signatures) {
                    has_privkeys |= HaveKey(key_sig_pair.first);
                }
                return has_privkeys;
            }
            return false;
        }
        */
    }
    
    pub fn sign_transaction(&self, 
        tx:           &mut MutableTransaction,
        coins:        &HashMap<OutPoint,Coin>,
        sighash:      i32,
        input_errors: &mut HashMap<i32,BilingualStr>) -> bool {
        
        todo!();
        /*
            return ::SignTransaction(tx, this, coins, sighash, input_errors);
        */
    }
    
    pub fn sign_message(&self, 
        message: &String,
        pkhash:  &PKHash,
        str_sig: &mut String) -> SigningResult {
        
        todo!();
        /*
            CKey key;
        if (!GetKey(ToKeyID(pkhash), key)) {
            return SigningResult::PRIVATE_KEY_NOT_AVAILABLE;
        }

        if (MessageSign(key, message, str_sig)) {
            return SigningResult::OK;
        }
        return SigningResult::SIGNING_FAILED;
        */
    }
    
    pub fn fillpsbt(&self, 
        psbtx:        &mut PartiallySignedTransaction,
        txdata:       &PrecomputedTransactionData,
        sighash_type: Option<i32>,
        sign:         Option<bool>,
        bip_32derivs: Option<bool>,
        n_signed:     *mut i32) -> TransactionError {

        /* SIGHASH_ALL */
        let sighash_type: i32 = sighash_type.unwrap_or(1 );
        let sign: bool = sign.unwrap_or(true);
        let bip_32derivs: bool = bip_32derivs.unwrap_or(false);
        
        todo!();
        /*
            if (n_signed) {
            *n_signed = 0;
        }
        for (unsigned int i = 0; i < psbtx.tx->vin.size(); ++i) {
            const CTxIn& txin = psbtx.tx->vin[i];
            PSBTInput& input = psbtx.inputs.at(i);

            if (PSBTInputSigned(input)) {
                continue;
            }

            // Get the Sighash type
            if (sign && input.sighash_type > 0 && input.sighash_type != sighash_type) {
                return TransactionError::SIGHASH_MISMATCH;
            }

            // Check non_witness_utxo has specified prevout
            if (input.non_witness_utxo) {
                if (txin.prevout.n >= input.non_witness_utxo->vout.size()) {
                    return TransactionError::MISSING_INPUTS;
                }
            } else if (input.witness_utxo.IsNull()) {
                // There's no UTXO so we can just skip this now
                continue;
            }
            SignatureData sigdata;
            input.FillSignatureData(sigdata);
            SignPSBTInput(HidingSigningProvider(this, !sign, !bip32derivs), psbtx, i, &txdata, sighash_type);

            bool signed_one = PSBTInputSigned(input);
            if (n_signed && (signed_one || !sign)) {
                // If sign is false, we assume that we _could_ sign if we get here. This
                // will never have false negatives; it is hard to tell under what i
                // circumstances it could have false positives.
                (*n_signed)++;
            }
        }

        // Fill in the bip32 keypaths and redeemscripts for the outputs so that hardware wallets can identify change
        for (unsigned int i = 0; i < psbtx.tx->vout.size(); ++i) {
            UpdatePSBTOutput(HidingSigningProvider(this, true, !bip32derivs), psbtx, i);
        }

        return TransactionError::OK;
        */
    }
    
    pub fn get_metadata(&self, dest: &TxDestination) -> Box<KeyMetadata> {
        
        todo!();
        /*
            LOCK(cs_KeyStore);

        CKeyID key_id = GetKeyForDestination(*this, dest);
        if (!key_id.IsNull()) {
            auto it = mapKeyMetadata.find(key_id);
            if (it != mapKeyMetadata.end()) {
                return std::make_unique<CKeyMetadata>(it->second);
            }
        }

        CScript scriptPubKey = GetScriptForDestination(dest);
        auto it = m_script_metadata.find(CScriptID(scriptPubKey));
        if (it != m_script_metadata.end()) {
            return std::make_unique<CKeyMetadata>(it->second);
        }

        return nullptr;
        */
    }
    
    pub fn getid(&self) -> u256 {
        
        todo!();
        /*
            return uint256::ONE;
        */
    }

    /**
      | Update wallet first key creation time.
      | This should be called whenever keys
      | are added to the wallet, with the oldest
      | key creation time.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn update_time_first_key(&mut self, n_create_time: i64)  {
        
        todo!();
        /*
            AssertLockHeld(cs_KeyStore);
        if (nCreateTime <= 1) {
            // Cannot determine birthday information, so set the wallet birthday to
            // the beginning of time.
            nTimeFirstKey = 1;
        } else if (!nTimeFirstKey || nCreateTime < nTimeFirstKey) {
            nTimeFirstKey = nCreateTime;
        }
        */
    }
    
    /**
      | Adds a key to the store, without saving
      | it to disk (used by LoadWallet)
      |
      */
    pub fn load_key(&mut self, 
        key:    &Key,
        pubkey: &PubKey) -> bool {
        
        todo!();
        /*
            return AddKeyPubKeyInner(key, pubkey);
        */
    }
    
    /**
      | Adds a key to the store, and saves it to
      | disk.
      |
      */
    pub fn add_key_pub_key(&mut self, 
        secret: &Key,
        pubkey: &PubKey) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        WalletBatch batch(m_storage.GetDatabase());
        return LegacyScriptPubKeyMan::AddKeyPubKeyWithDB(batch, secret, pubkey);
        */
    }
    
    /**
      | Adds a key to the store, and saves it to
      | disk.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn add_key_pub_key_withdb(&mut self, 
        batch:  &mut WalletBatch,
        secret: &Key,
        pubkey: &PubKey) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_KeyStore);

        // Make sure we aren't adding private keys to private key disabled wallets
        assert(!m_storage.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS));

        // FillableSigningProvider has no concept of wallet databases, but calls AddCryptedKey
        // which is overridden below.  To avoid flushes, the database handle is
        // tunneled through to it.
        bool needsDB = !encrypted_batch;
        if (needsDB) {
            encrypted_batch = &batch;
        }
        if (!AddKeyPubKeyInner(secret, pubkey)) {
            if (needsDB) encrypted_batch = nullptr;
            return false;
        }
        if (needsDB) encrypted_batch = nullptr;

        // check if we need to remove from watch-only
        CScript script;
        script = GetScriptForDestination(PKHash(pubkey));
        if (HaveWatchOnly(script)) {
            RemoveWatchOnly(script);
        }
        script = GetScriptForRawPubKey(pubkey);
        if (HaveWatchOnly(script)) {
            RemoveWatchOnly(script);
        }

        if (!m_storage.HasEncryptionKeys()) {
            return batch.WriteKey(pubkey,
                                                     secret.GetPrivKey(),
                                                     mapKeyMetadata[pubkey.GetID()]);
        }
        m_storage.UnsetBlankWalletFlag(batch);
        return true;
        */
    }
    
    /**
      | Adds a CScript to the store
      |
      */
    pub fn load_cscript(&mut self, redeem_script: &Script) -> bool {
        
        todo!();
        /*
            /* A sanity check was added in pull #3843 to avoid adding redeemScripts
         * that never can be redeemed. However, old wallets may still contain
         * these. Do not add them to the wallet and warn. */
        if (redeemScript.size() > MAX_SCRIPT_ELEMENT_SIZE)
        {
            std::string strAddr = EncodeDestination(ScriptHash(redeemScript));
            WalletLogPrintf("%s: Warning: This wallet contains a redeemScript of size %i which exceeds maximum size %i thus can never be redeemed. Do not use address %s.\n", __func__, redeemScript.size(), MAX_SCRIPT_ELEMENT_SIZE, strAddr);
            return true;
        }

        return FillableSigningProvider::AddCScript(redeemScript);
        */
    }
    
    /**
      | Load metadata (used by LoadWallet)
      |
      */
    pub fn load_key_metadata(&mut self, 
        keyid: &KeyID,
        meta:  &KeyMetadata)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        UpdateTimeFirstKey(meta.nCreateTime);
        mapKeyMetadata[keyID] = meta;
        */
    }
    
    pub fn load_script_metadata(&mut self, 
        script_id: &ScriptID,
        meta:      &KeyMetadata)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        UpdateTimeFirstKey(meta.nCreateTime);
        m_script_metadata[script_id] = meta;
        */
    }
    
    pub fn add_key_pub_key_inner(&mut self, 
        key:    &Key,
        pubkey: &PubKey) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        if (!m_storage.HasEncryptionKeys()) {
            return FillableSigningProvider::AddKeyPubKey(key, pubkey);
        }

        if (m_storage.IsLocked()) {
            return false;
        }

        std::vector<unsigned char> vchCryptedSecret;
        CKeyingMaterial vchSecret(key.begin(), key.end());
        if (!EncryptSecret(m_storage.GetEncryptionKey(), vchSecret, pubkey.GetHash(), vchCryptedSecret)) {
            return false;
        }

        if (!AddCryptedKey(pubkey, vchCryptedSecret)) {
            return false;
        }
        return true;
        */
    }
    
    /**
      | Adds an encrypted key to the store, without
      | saving it to disk (used by LoadWallet)
      |
      */
    pub fn load_crypted_key(&mut self, 
        vch_pub_key:        &PubKey,
        vch_crypted_secret: &Vec<u8>,
        checksum_valid:     bool) -> bool {
        
        todo!();
        /*
            // Set fDecryptionThoroughlyChecked to false when the checksum is invalid
        if (!checksum_valid) {
            fDecryptionThoroughlyChecked = false;
        }

        return AddCryptedKeyInner(vchPubKey, vchCryptedSecret);
        */
    }
    
    pub fn add_crypted_key_inner(&mut self, 
        vch_pub_key:        &PubKey,
        vch_crypted_secret: &Vec<u8>) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        assert(mapKeys.empty());

        mapCryptedKeys[vchPubKey.GetID()] = make_pair(vchPubKey, vchCryptedSecret);
        ImplicitlyLearnRelatedKeyScripts(vchPubKey);
        return true;
        */
    }
    
    /**
      | Adds an encrypted key to the store, and
      | saves it to disk.
      |
      */
    pub fn add_crypted_key(&mut self, 
        vch_pub_key:        &PubKey,
        vch_crypted_secret: &Vec<u8>) -> bool {
        
        todo!();
        /*
            if (!AddCryptedKeyInner(vchPubKey, vchCryptedSecret))
            return false;
        {
            LOCK(cs_KeyStore);
            if (encrypted_batch)
                return encrypted_batch->WriteCryptedKey(vchPubKey,
                                                            vchCryptedSecret,
                                                            mapKeyMetadata[vchPubKey.GetID()]);
            else
                return WalletBatch(m_storage.GetDatabase()).WriteCryptedKey(vchPubKey,
                                                                vchCryptedSecret,
                                                                mapKeyMetadata[vchPubKey.GetID()]);
        }
        */
    }
    
    /**
      | Returns whether the watch-only script
      | is in the wallet
      |
      */
    pub fn have_watch_only_with_script(&self, dest: &Script) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        return setWatchOnly.count(dest) > 0;
        */
    }
    
    /**
      | Returns whether there are any watch-only
      | things in the wallet
      |
      */
    pub fn have_watch_only(&self) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        return (!setWatchOnly.empty());
        */
    }
    
    /**
      | Remove a watch only script from the keystore
      |
      */
    pub fn remove_watch_only(&mut self, dest: &Script) -> bool {
        
        todo!();
        /*
            {
            LOCK(cs_KeyStore);
            setWatchOnly.erase(dest);
            CPubKey pubKey;
            if (ExtractPubKey(dest, pubKey)) {
                mapWatchKeys.erase(pubKey.GetID());
            }
            // Related CScripts are not removed; having superfluous scripts around is
            // harmless (see comment in ImplicitlyLearnRelatedKeyScripts).
        }

        if (!HaveWatchOnly())
            NotifyWatchonlyChanged(false);
        if (!WalletBatch(m_storage.GetDatabase()).EraseWatchOnly(dest))
            return false;

        return true;
        */
    }
    
    /**
      | Adds a watch-only address to the store,
      | without saving it to disk (used by
      | LoadWallet)
      |
      */
    pub fn load_watch_only(&mut self, dest: &Script) -> bool {
        
        todo!();
        /*
            return AddWatchOnlyInMem(dest);
        */
    }
    
    pub fn add_watch_only_in_mem(&mut self, dest: &Script) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        setWatchOnly.insert(dest);
        CPubKey pubKey;
        if (ExtractPubKey(dest, pubKey)) {
            mapWatchKeys[pubKey.GetID()] = pubKey;
            ImplicitlyLearnRelatedKeyScripts(pubKey);
        }
        return true;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn add_watch_only_withdb(&mut self, 
        batch: &mut WalletBatch,
        dest:  &Script) -> bool {
        
        todo!();
        /*
            if (!AddWatchOnlyInMem(dest))
            return false;
        const CKeyMetadata& meta = m_script_metadata[CScriptID(dest)];
        UpdateTimeFirstKey(meta.nCreateTime);
        NotifyWatchonlyChanged(true);
        if (batch.WriteWatchOnly(dest, meta)) {
            m_storage.UnsetBlankWalletFlag(batch);
            return true;
        }
        return false;
        */
    }
    
    /**
      | Adds a watch-only address to the store,
      | and saves it to disk.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn add_watch_only_withdb_with_create_time(&mut self, 
        batch:       &mut WalletBatch,
        dest:        &Script,
        create_time: i64) -> bool {
        
        todo!();
        /*
            m_script_metadata[CScriptID(dest)].nCreateTime = create_time;
        return AddWatchOnlyWithDB(batch, dest);
        */
    }
    
    /**
      | Private version of AddWatchOnly method
      | which does not accept a timestamp, and
      | which will reset the wallet's nTimeFirstKey
      | value to 1 if the watch key did not previously
      | have a timestamp associated with it.
      | 
      | Because this is an inherited virtual
      | method, it is accessible despite being
      | marked private, but it is marked private
      | anyway to encourage use of the other
      | AddWatchOnly which accepts a timestamp
      | and sets nTimeFirstKey more intelligently
      | for more efficient rescans.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn add_watch_only(&mut self, dest: &Script) -> bool {
        
        todo!();
        /*
            WalletBatch batch(m_storage.GetDatabase());
        return AddWatchOnlyWithDB(batch, dest);
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn add_watch_only_with_create_time(&mut self, 
        dest:          &Script,
        n_create_time: i64) -> bool {
        
        todo!();
        /*
            m_script_metadata[CScriptID(dest)].nCreateTime = nCreateTime;
        return AddWatchOnly(dest);
        */
    }
    
    /**
      | Load a HD chain model (used by LoadWallet)
      |
      */
    pub fn load_hd_chain(&mut self, chain: &HDChain)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        m_hd_chain = chain;
        */
    }
    
    /**
      | Set the HD chain model (chain child index
      | counters) and writes it to the database
      |
      */
    pub fn add_hd_chain(&mut self, chain: &HDChain)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        // Store the new chain
        if (!WalletBatch(m_storage.GetDatabase()).WriteHDChain(chain)) {
            throw std::runtime_error(std::string(__func__) + ": writing chain failed");
        }
        // When there's an old chain, add it as an inactive chain as we are now rotating hd chains
        if (!m_hd_chain.seed_id.IsNull()) {
            AddInactiveHDChain(m_hd_chain);
        }

        m_hd_chain = chain;
        */
    }
    
    pub fn add_inactive_hd_chain(&mut self, chain: &HDChain)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        assert(!chain.seed_id.IsNull());
        m_inactive_hd_chains[chain.seed_id] = chain;
        */
    }
    
    pub fn have_key(&self, address: &KeyID) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        if (!m_storage.HasEncryptionKeys()) {
            return FillableSigningProvider::HaveKey(address);
        }
        return mapCryptedKeys.count(address) > 0;
        */
    }
    
    pub fn get_key(&self, 
        address: &KeyID,
        key_out: &mut Key) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        if (!m_storage.HasEncryptionKeys()) {
            return FillableSigningProvider::GetKey(address, keyOut);
        }

        CryptedKeyMap::const_iterator mi = mapCryptedKeys.find(address);
        if (mi != mapCryptedKeys.end())
        {
            const CPubKey &vchPubKey = (*mi).second.first;
            const std::vector<unsigned char> &vchCryptedSecret = (*mi).second.second;
            return DecryptKey(m_storage.GetEncryptionKey(), vchCryptedSecret, vchPubKey, keyOut);
        }
        return false;
        */
    }
    
    pub fn get_key_origin(&self, 
        keyid: &KeyID,
        info:  &mut KeyOriginInfo) -> bool {
        
        todo!();
        /*
            CKeyMetadata meta;
        {
            LOCK(cs_KeyStore);
            auto it = mapKeyMetadata.find(keyID);
            if (it != mapKeyMetadata.end()) {
                meta = it->second;
            }
        }
        if (meta.has_key_origin) {
            std::copy(meta.key_origin.fingerprint, meta.key_origin.fingerprint + 4, info.fingerprint);
            info.path = meta.key_origin.path;
        } else { // Single pubkeys get the master fingerprint of themselves
            std::copy(keyID.begin(), keyID.begin() + 4, info.fingerprint);
        }
        return true;
        */
    }
    
    /**
      | Fetches a pubkey from mapWatchKeys
      | if it exists there
      |
      */
    pub fn get_watch_pub_key(&self, 
        address:    &KeyID,
        pubkey_out: &mut PubKey) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        WatchKeyMap::const_iterator it = mapWatchKeys.find(address);
        if (it != mapWatchKeys.end()) {
            pubkey_out = it->second;
            return true;
        }
        return false;
        */
    }
    
    pub fn get_pub_key(&self, 
        address:         &KeyID,
        vch_pub_key_out: &mut PubKey) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        if (!m_storage.HasEncryptionKeys()) {
            if (!FillableSigningProvider::GetPubKey(address, vchPubKeyOut)) {
                return GetWatchPubKey(address, vchPubKeyOut);
            }
            return true;
        }

        CryptedKeyMap::const_iterator mi = mapCryptedKeys.find(address);
        if (mi != mapCryptedKeys.end())
        {
            vchPubKeyOut = (*mi).second.first;
            return true;
        }
        // Check for watch-only pubkeys
        return GetWatchPubKey(address, vchPubKeyOut);
        */
    }
    
    /**
      | Generate a new key
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn generate_new_key(&mut self, 
        batch:    &mut WalletBatch,
        hd_chain: &mut HDChain,
        internal: Option<bool>) -> PubKey {

        let internal: bool = internal.unwrap_or(false);
        
        todo!();
        /*
            assert(!m_storage.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS));
        assert(!m_storage.IsWalletFlagSet(WALLET_FLAG_BLANK_WALLET));
        AssertLockHeld(cs_KeyStore);
        bool fCompressed = m_storage.CanSupportFeature(FEATURE_COMPRPUBKEY); // default to compressed public keys if we want 0.6.0 wallets

        CKey secret;

        // Create new metadata
        int64_t nCreationTime = GetTime();
        CKeyMetadata metadata(nCreationTime);

        // use HD key derivation if HD was enabled during wallet creation and a seed is present
        if (IsHDEnabled()) {
            DeriveNewChildKey(batch, metadata, secret, hd_chain, (m_storage.CanSupportFeature(FEATURE_HD_SPLIT) ? internal : false));
        } else {
            secret.MakeNewKey(fCompressed);
        }

        // Compressed public keys were introduced in version 0.6.0
        if (fCompressed) {
            m_storage.SetMinVersion(FEATURE_COMPRPUBKEY);
        }

        CPubKey pubkey = secret.GetPubKey();
        assert(secret.VerifyPubKey(pubkey));

        mapKeyMetadata[pubkey.GetID()] = metadata;
        UpdateTimeFirstKey(nCreationTime);

        if (!AddKeyPubKeyWithDB(batch, secret, pubkey)) {
            throw std::runtime_error(std::string(__func__) + ": AddKey failed");
        }
        return pubkey;
        */
    }
    
    /**
      | HD derive new child key (on internal
      | or external chain)
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn derive_new_child_key(&mut self, 
        batch:    &mut WalletBatch,
        metadata: &mut KeyMetadata,
        secret:   &mut Key,
        hd_chain: &mut HDChain,
        internal: Option<bool>)  {

        let internal: bool = internal.unwrap_or(false);
        
        todo!();
        /*
            // for now we use a fixed keypath scheme of m/0'/0'/k
        CKey seed;                     //seed (256bit)
        CExtKey masterKey;             //hd master key
        CExtKey accountKey;            //key at m/0'
        CExtKey chainChildKey;         //key at m/0'/0' (external) or m/0'/1' (internal)
        CExtKey childKey;              //key at m/0'/0'/<n>'

        // try to get the seed
        if (!GetKey(hd_chain.seed_id, seed))
            throw std::runtime_error(std::string(__func__) + ": seed not found");

        masterKey.SetSeed(seed.begin(), seed.size());

        // derive m/0'
        // use hardened derivation (child keys >= 0x80000000 are hardened after bip32)
        masterKey.Derive(accountKey, BIP32_HARDENED_KEY_LIMIT);

        // derive m/0'/0' (external chain) OR m/0'/1' (internal chain)
        assert(internal ? m_storage.CanSupportFeature(FEATURE_HD_SPLIT) : true);
        accountKey.Derive(chainChildKey, BIP32_HARDENED_KEY_LIMIT+(internal ? 1 : 0));

        // derive child key at next index, skip keys already known to the wallet
        do {
            // always derive hardened keys
            // childIndex | BIP32_HARDENED_KEY_LIMIT = derive childIndex in hardened child-index-range
            // example: 1 | BIP32_HARDENED_KEY_LIMIT == 0x80000001 == 2147483649
            if (internal) {
                chainChildKey.Derive(childKey, hd_chain.nInternalChainCounter | BIP32_HARDENED_KEY_LIMIT);
                metadata.hdKeypath = "m/0'/1'/" + ToString(hd_chain.nInternalChainCounter) + "'";
                metadata.key_origin.path.push_back(0 | BIP32_HARDENED_KEY_LIMIT);
                metadata.key_origin.path.push_back(1 | BIP32_HARDENED_KEY_LIMIT);
                metadata.key_origin.path.push_back(hd_chain.nInternalChainCounter | BIP32_HARDENED_KEY_LIMIT);
                hd_chain.nInternalChainCounter++;
            }
            else {
                chainChildKey.Derive(childKey, hd_chain.nExternalChainCounter | BIP32_HARDENED_KEY_LIMIT);
                metadata.hdKeypath = "m/0'/0'/" + ToString(hd_chain.nExternalChainCounter) + "'";
                metadata.key_origin.path.push_back(0 | BIP32_HARDENED_KEY_LIMIT);
                metadata.key_origin.path.push_back(0 | BIP32_HARDENED_KEY_LIMIT);
                metadata.key_origin.path.push_back(hd_chain.nExternalChainCounter | BIP32_HARDENED_KEY_LIMIT);
                hd_chain.nExternalChainCounter++;
            }
        } while (HaveKey(childKey.key.GetPubKey().GetID()));
        secret = childKey.key;
        metadata.hd_seed_id = hd_chain.seed_id;
        CKeyID master_id = masterKey.key.GetPubKey().GetID();
        std::copy(master_id.begin(), master_id.begin() + 4, metadata.key_origin.fingerprint);
        metadata.has_key_origin = true;
        // update the chain model in the database
        if (hd_chain.seed_id == m_hd_chain.seed_id && !batch.WriteHDChain(hd_chain))
            throw std::runtime_error(std::string(__func__) + ": writing HD chain model failed");
        */
    }
    
    /**
      | Load a keypool entry
      |
      */
    pub fn load_key_pool(&mut self, 
        n_index: i64,
        keypool: &KeyPool)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        if (keypool.m_pre_split) {
            set_pre_split_keypool.insert(nIndex);
        } else if (keypool.fInternal) {
            setInternalKeyPool.insert(nIndex);
        } else {
            setExternalKeyPool.insert(nIndex);
        }
        m_max_keypool_index = std::max(m_max_keypool_index, nIndex);
        m_pool_key_to_index[keypool.vchPubKey.GetID()] = nIndex;

        // If no metadata exists yet, create a default with the pool key's
        // creation time. Note that this may be overwritten by actually
        // stored metadata for that key later, which is fine.
        CKeyID keyid = keypool.vchPubKey.GetID();
        if (mapKeyMetadata.count(keyid) == 0)
            mapKeyMetadata[keyid] = CKeyMetadata(keypool.nTime);
        */
    }
    
    /**
      | Returns true if the wallet can generate
      | new keys
      |
      */
    pub fn can_generate_keys(&self) -> bool {
        
        todo!();
        /*
            // A wallet can generate keys if it has an HD seed (IsHDEnabled) or it is a non-HD wallet (pre FEATURE_HD)
        LOCK(cs_KeyStore);
        return IsHDEnabled() || !m_storage.CanSupportFeature(FEATURE_HD);
        */
    }
    
    /**
      | Generates a new HD seed (will not be activated)
      |
      */
    pub fn generate_new_seed(&mut self) -> PubKey {
        
        todo!();
        /*
            assert(!m_storage.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS));
        CKey key;
        key.MakeNewKey(true);
        return DeriveNewSeed(key);
        */
    }
    
    /**
      | Derives a new HD seed (will not be activated)
      |
      */
    pub fn derive_new_seed(&mut self, key: &Key) -> PubKey {
        
        todo!();
        /*
            int64_t nCreationTime = GetTime();
        CKeyMetadata metadata(nCreationTime);

        // calculate the seed
        CPubKey seed = key.GetPubKey();
        assert(key.VerifyPubKey(seed));

        // set the hd keypath to "s" -> Seed, refers the seed to itself
        metadata.hdKeypath     = "s";
        metadata.has_key_origin = false;
        metadata.hd_seed_id = seed.GetID();

        {
            LOCK(cs_KeyStore);

            // mem store the metadata
            mapKeyMetadata[seed.GetID()] = metadata;

            // write the key&metadata to the database
            if (!AddKeyPubKey(key, seed))
                throw std::runtime_error(std::string(__func__) + ": AddKeyPubKey failed");
        }

        return seed;
        */
    }
    
    /**
      | Set the current HD seed (will reset the
      | chain child index counters)
      | 
      | Sets the seed's version based on the
      | current wallet version (so the caller
      | must ensure the current wallet version
      | is correct before calling this function).
      |
      */
    pub fn set_hd_seed(&mut self, seed: &PubKey)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        // store the keyid (hash160) together with
        // the child index counter in the database
        // as a hdchain object
        CHDChain newHdChain;
        newHdChain.nVersion = m_storage.CanSupportFeature(FEATURE_HD_SPLIT) ? CHDChain::VERSION_HD_CHAIN_SPLIT : CHDChain::VERSION_HD_BASE;
        newHdChain.seed_id = seed.GetID();
        AddHDChain(newHdChain);
        NotifyCanGetAddressesChanged();
        WalletBatch batch(m_storage.GetDatabase());
        m_storage.UnsetBlankWalletFlag(batch);
        */
    }

    /**
      | Mark old keypool keys as used, and generate
      | all new keys
      |
      */
    pub fn new_key_pool(&mut self) -> bool {
        
        todo!();
        /*
            if (m_storage.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS)) {
            return false;
        }
        {
            LOCK(cs_KeyStore);
            WalletBatch batch(m_storage.GetDatabase());

            for (const int64_t nIndex : setInternalKeyPool) {
                batch.ErasePool(nIndex);
            }
            setInternalKeyPool.clear();

            for (const int64_t nIndex : setExternalKeyPool) {
                batch.ErasePool(nIndex);
            }
            setExternalKeyPool.clear();

            for (const int64_t nIndex : set_pre_split_keypool) {
                batch.ErasePool(nIndex);
            }
            set_pre_split_keypool.clear();

            m_pool_key_to_index.clear();

            if (!TopUp()) {
                return false;
            }
            WalletLogPrintf("LegacyScriptPubKeyMan::NewKeyPool rewrote keypool\n");
        }
        return true;
        */
    }
    
    pub fn top_up(&mut self, kp_size: Option<u32>) -> bool {

        let kp_size: u32 = kp_size.unwrap_or(0);
        
        todo!();
        /*
            if (!CanGenerateKeys()) {
            return false;
        }
        {
            LOCK(cs_KeyStore);

            if (m_storage.IsLocked()) return false;

            // Top up key pool
            unsigned int nTargetSize;
            if (kpSize > 0)
                nTargetSize = kpSize;
            else
                nTargetSize = std::max(gArgs.GetIntArg("-keypool", DEFAULT_KEYPOOL_SIZE), (int64_t) 0);

            // count amount of available keys (internal, external)
            // make sure the keypool of external and internal keys fits the user selected target (-keypool)
            int64_t missingExternal = std::max(std::max((int64_t) nTargetSize, (int64_t) 1) - (int64_t)setExternalKeyPool.size(), (int64_t) 0);
            int64_t missingInternal = std::max(std::max((int64_t) nTargetSize, (int64_t) 1) - (int64_t)setInternalKeyPool.size(), (int64_t) 0);

            if (!IsHDEnabled() || !m_storage.CanSupportFeature(FEATURE_HD_SPLIT))
            {
                // don't create extra internal keys
                missingInternal = 0;
            }
            bool internal = false;
            WalletBatch batch(m_storage.GetDatabase());
            for (int64_t i = missingInternal + missingExternal; i--;)
            {
                if (i < missingInternal) {
                    internal = true;
                }

                CPubKey pubkey(GenerateNewKey(batch, m_hd_chain, internal));
                AddKeypoolPubkeyWithDB(pubkey, internal, batch);
            }
            if (missingInternal + missingExternal > 0) {
                WalletLogPrintf("keypool added %d keys (%d internal), size=%u (%u internal)\n", missingInternal + missingExternal, missingInternal, setInternalKeyPool.size() + setExternalKeyPool.size() + set_pre_split_keypool.size(), setInternalKeyPool.size());
            }
        }
        NotifyCanGetAddressesChanged();
        return true;
        */
    }
    
    pub fn add_keypool_pubkey_withdb(&mut self, 
        pubkey:   &PubKey,
        internal: bool,
        batch:    &mut WalletBatch)  {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        assert(m_max_keypool_index < std::numeric_limits<int64_t>::max()); // How in the hell did you use so many keys?
        int64_t index = ++m_max_keypool_index;
        if (!batch.WritePool(index, CKeyPool(pubkey, internal))) {
            throw std::runtime_error(std::string(__func__) + ": writing imported pubkey failed");
        }
        if (internal) {
            setInternalKeyPool.insert(index);
        } else {
            setExternalKeyPool.insert(index);
        }
        m_pool_key_to_index[pubkey.GetID()] = index;
        */
    }
    
    pub fn keep_destination(&mut self, 
        n_index: i64,
        ty:      &OutputType)  {
        
        todo!();
        /*
            assert(type != OutputType::BECH32M);
        // Remove from key pool
        WalletBatch batch(m_storage.GetDatabase());
        batch.ErasePool(nIndex);
        CPubKey pubkey;
        bool have_pk = GetPubKey(m_index_to_reserved_key.at(nIndex), pubkey);
        assert(have_pk);
        LearnRelatedScripts(pubkey, type);
        m_index_to_reserved_key.erase(nIndex);
        WalletLogPrintf("keypool keep %d\n", nIndex);
        */
    }
    
    pub fn return_destination(&mut self, 
        n_index:  i64,
        internal: bool,
        _2:       &TxDestination)  {
        
        todo!();
        /*
            // Return to key pool
        {
            LOCK(cs_KeyStore);
            if (fInternal) {
                setInternalKeyPool.insert(nIndex);
            } else if (!set_pre_split_keypool.empty()) {
                set_pre_split_keypool.insert(nIndex);
            } else {
                setExternalKeyPool.insert(nIndex);
            }
            CKeyID& pubkey_id = m_index_to_reserved_key.at(nIndex);
            m_pool_key_to_index[pubkey_id] = nIndex;
            m_index_to_reserved_key.erase(nIndex);
            NotifyCanGetAddressesChanged();
        }
        WalletLogPrintf("keypool return %d\n", nIndex);
        */
    }
    
    /**
      | Fetches a key from the keypool
      |
      */
    pub fn get_key_from_pool(&mut self, 
        result:   &mut PubKey,
        ty:       OutputType,
        internal: Option<bool>) -> bool {

        let internal: bool = internal.unwrap_or(false);
        
        todo!();
        /*
            assert(type != OutputType::BECH32M);
        if (!CanGetAddresses(internal)) {
            return false;
        }

        CKeyPool keypool;
        {
            LOCK(cs_KeyStore);
            int64_t nIndex;
            if (!ReserveKeyFromKeyPool(nIndex, keypool, internal) && !m_storage.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS)) {
                if (m_storage.IsLocked()) return false;
                WalletBatch batch(m_storage.GetDatabase());
                result = GenerateNewKey(batch, m_hd_chain, internal);
                return true;
            }
            KeepDestination(nIndex, type);
            result = keypool.vchPubKey;
        }
        return true;
        */
    }
    
    /**
      | Reserves a key from the keypool and sets
      | nIndex to its index
      | 
      | -----------
      | @param[out] nIndex
      | 
      | the index of the key in keypool
      | ----------
      | @param[out] keypool
      | 
      | the keypool the key was drawn from, which
      | could be the the pre-split pool if present,
      | or the internal or external pool
      | ----------
      | @param fRequestedInternal
      | 
      | true if the caller would like the key
      | drawn from the internal keypool, false
      | if external is preferred
      | 
      | -----------
      | @return
      | 
      | true if succeeded, false if failed due
      | to empty keypool @throws std::runtime_error
      | if keypool read failed, key was invalid,
      | was not found in the wallet, or was misclassified
      | in the internal or external keypool
      |
      */
    pub fn reserve_key_from_key_pool(&mut self, 
        n_index:            &mut i64,
        keypool:            &mut KeyPool,
        requested_internal: bool) -> bool {
        
        todo!();
        /*
            nIndex = -1;
        keypool.vchPubKey = CPubKey();
        {
            LOCK(cs_KeyStore);

            bool fReturningInternal = fRequestedInternal;
            fReturningInternal &= (IsHDEnabled() && m_storage.CanSupportFeature(FEATURE_HD_SPLIT)) || m_storage.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS);
            bool use_split_keypool = set_pre_split_keypool.empty();
            std::set<int64_t>& setKeyPool = use_split_keypool ? (fReturningInternal ? setInternalKeyPool : setExternalKeyPool) : set_pre_split_keypool;

            // Get the oldest key
            if (setKeyPool.empty()) {
                return false;
            }

            WalletBatch batch(m_storage.GetDatabase());

            auto it = setKeyPool.begin();
            nIndex = *it;
            setKeyPool.erase(it);
            if (!batch.ReadPool(nIndex, keypool)) {
                throw std::runtime_error(std::string(__func__) + ": read failed");
            }
            CPubKey pk;
            if (!GetPubKey(keypool.vchPubKey.GetID(), pk)) {
                throw std::runtime_error(std::string(__func__) + ": unknown key in key pool");
            }
            // If the key was pre-split keypool, we don't care about what type it is
            if (use_split_keypool && keypool.fInternal != fReturningInternal) {
                throw std::runtime_error(std::string(__func__) + ": keypool entry misclassified");
            }
            if (!keypool.vchPubKey.IsValid()) {
                throw std::runtime_error(std::string(__func__) + ": keypool entry invalid");
            }

            assert(m_index_to_reserved_key.count(nIndex) == 0);
            m_index_to_reserved_key[nIndex] = keypool.vchPubKey.GetID();
            m_pool_key_to_index.erase(keypool.vchPubKey.GetID());
            WalletLogPrintf("keypool reserve %d\n", nIndex);
        }
        NotifyCanGetAddressesChanged();
        return true;
        */
    }
    
    /**
      | Explicitly make the wallet learn the
      | related scripts for outputs to the given
      | key. This is purely to make the wallet
      | file compatible with older software,
      | as FillableSigningProvider automatically
      | does this implicitly for all keys now.
      |
      */
    pub fn learn_related_scripts(&mut self, 
        key: &PubKey,
        ty:  OutputType)  {
        
        todo!();
        /*
            assert(type != OutputType::BECH32M);
        if (key.IsCompressed() && (type == OutputType::P2SH_SEGWIT || type == OutputType::BECH32)) {
            TxDestination witdest = WitnessV0KeyHash(key.GetID());
            CScript witprog = GetScriptForDestination(witdest);
            // Make sure the resulting program is solvable.
            assert(IsSolvable(*this, witprog));
            AddCScript(witprog);
        }
        */
    }
    
    /**
      | Same as LearnRelatedScripts, but when
      | the OutputType is not known (and could
      | be anything).
      |
      */
    pub fn learn_all_related_scripts(&mut self, key: &PubKey)  {
        
        todo!();
        /*
            // OutputType::P2SH_SEGWIT always adds all necessary scripts for all types.
        LearnRelatedScripts(key, OutputType::P2SH_SEGWIT);
        */
    }
    
    /**
      | Marks all keys in the keypool up to and
      | including reserve_key as used.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn mark_reserve_keys_as_used(&mut self, keypool_id: i64)  {
        
        todo!();
        /*
            AssertLockHeld(cs_KeyStore);
        bool internal = setInternalKeyPool.count(keypool_id);
        if (!internal) assert(setExternalKeyPool.count(keypool_id) || set_pre_split_keypool.count(keypool_id));
        std::set<int64_t> *setKeyPool = internal ? &setInternalKeyPool : (set_pre_split_keypool.empty() ? &setExternalKeyPool : &set_pre_split_keypool);
        auto it = setKeyPool->begin();

        WalletBatch batch(m_storage.GetDatabase());
        while (it != std::end(*setKeyPool)) {
            const int64_t& index = *(it);
            if (index > keypool_id) break; // set*KeyPool is ordered

            CKeyPool keypool;
            if (batch.ReadPool(index, keypool)) { //TODO: This should be unnecessary
                m_pool_key_to_index.erase(keypool.vchPubKey.GetID());
            }
            LearnAllRelatedScripts(keypool.vchPubKey);
            batch.ErasePool(index);
            WalletLogPrintf("keypool index %d removed\n", index);
            it = setKeyPool->erase(it);
        }
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn mark_pre_split_keys(&mut self)  {
        
        todo!();
        /*
            WalletBatch batch(m_storage.GetDatabase());
        for (auto it = setExternalKeyPool.begin(); it != setExternalKeyPool.end();) {
            int64_t index = *it;
            CKeyPool keypool;
            if (!batch.ReadPool(index, keypool)) {
                throw std::runtime_error(std::string(__func__) + ": read keypool entry failed");
            }
            keypool.m_pre_split = true;
            if (!batch.WritePool(index, keypool)) {
                throw std::runtime_error(std::string(__func__) + ": writing modified keypool entry failed");
            }
            set_pre_split_keypool.insert(index);
            it = setExternalKeyPool.erase(it);
        }
        */
    }
    
    pub fn add_cscript(&mut self, redeem_script: &Script) -> bool {
        
        todo!();
        /*
            WalletBatch batch(m_storage.GetDatabase());
        return AddCScriptWithDB(batch, redeemScript);
        */
    }
    
    /**
      | Adds a script to the store and saves it
      | to disk
      |
      */
    pub fn add_cscript_withdb(&mut self, 
        batch:         &mut WalletBatch,
        redeem_script: &Script) -> bool {
        
        todo!();
        /*
            if (!FillableSigningProvider::AddCScript(redeemScript))
            return false;
        if (batch.WriteCScript(Hash160(redeemScript), redeemScript)) {
            m_storage.UnsetBlankWalletFlag(batch);
            return true;
        }
        return false;
        */
    }
    
    /**
      | Add a KeyOriginInfo to the wallet
      |
      */
    pub fn add_key_origin_withdb(&mut self, 
        batch:  &mut WalletBatch,
        pubkey: &PubKey,
        info:   &KeyOriginInfo) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        std::copy(info.fingerprint, info.fingerprint + 4, mapKeyMetadata[pubkey.GetID()].key_origin.fingerprint);
        mapKeyMetadata[pubkey.GetID()].key_origin.path = info.path;
        mapKeyMetadata[pubkey.GetID()].has_key_origin = true;
        mapKeyMetadata[pubkey.GetID()].hdKeypath = WriteHDKeypath(info.path);
        return batch.WriteKeyMetadata(mapKeyMetadata[pubkey.GetID()], pubkey, true);
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn import_scripts(&mut self, 
        scripts:   HashSet<Script>,
        timestamp: i64) -> bool {
        
        todo!();
        /*
            WalletBatch batch(m_storage.GetDatabase());
        for (const auto& entry : scripts) {
            CScriptID id(entry);
            if (HaveCScript(id)) {
                WalletLogPrintf("Already have script %s, skipping\n", HexStr(entry));
                continue;
            }
            if (!AddCScriptWithDB(batch, entry)) {
                return false;
            }

            if (timestamp > 0) {
                m_script_metadata[CScriptID(entry)].nCreateTime = timestamp;
            }
        }
        if (timestamp > 0) {
            UpdateTimeFirstKey(timestamp);
        }

        return true;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn import_priv_keys(&mut self, 
        privkey_map: &HashMap<KeyID,Key>,
        timestamp:   i64) -> bool {
        
        todo!();
        /*
            WalletBatch batch(m_storage.GetDatabase());
        for (const auto& entry : privkey_map) {
            const CKey& key = entry.second;
            CPubKey pubkey = key.GetPubKey();
            const CKeyID& id = entry.first;
            assert(key.VerifyPubKey(pubkey));
            // Skip if we already have the key
            if (HaveKey(id)) {
                WalletLogPrintf("Already have key with pubkey %s, skipping\n", HexStr(pubkey));
                continue;
            }
            mapKeyMetadata[id].nCreateTime = timestamp;
            // If the private key is not present in the wallet, insert it.
            if (!AddKeyPubKeyWithDB(batch, key, pubkey)) {
                return false;
            }
            UpdateTimeFirstKey(timestamp);
        }
        return true;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn import_pub_keys(&mut self, 
        ordered_pubkeys: &Vec<KeyID>,
        pubkey_map:      &HashMap<KeyID,PubKey>,
        key_origins:     &HashMap<KeyID,(PubKey,KeyOriginInfo)>,
        add_keypool:     bool,
        internal:        bool,
        timestamp:       i64) -> bool {
        
        todo!();
        /*
            WalletBatch batch(m_storage.GetDatabase());
        for (const auto& entry : key_origins) {
            AddKeyOriginWithDB(batch, entry.second.first, entry.second.second);
        }
        for (const CKeyID& id : ordered_pubkeys) {
            auto entry = pubkey_map.find(id);
            if (entry == pubkey_map.end()) {
                continue;
            }
            const CPubKey& pubkey = entry->second;
            CPubKey temp;
            if (GetPubKey(id, temp)) {
                // Already have pubkey, skipping
                WalletLogPrintf("Already have pubkey %s, skipping\n", HexStr(temp));
                continue;
            }
            if (!AddWatchOnlyWithDB(batch, GetScriptForRawPubKey(pubkey), timestamp)) {
                return false;
            }
            mapKeyMetadata[id].nCreateTime = timestamp;

            // Add to keypool only works with pubkeys
            if (add_keypool) {
                AddKeypoolPubkeyWithDB(pubkey, internal, batch);
                NotifyCanGetAddressesChanged();
            }
        }
        return true;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn import_script_pub_keys(&mut self, 
        script_pub_keys:   &HashSet<Script>,
        have_solving_data: bool,
        timestamp:         i64) -> bool {
        
        todo!();
        /*
            WalletBatch batch(m_storage.GetDatabase());
        for (const CScript& script : script_pub_keys) {
            if (!have_solving_data || !IsMine(script)) { // Always call AddWatchOnly for non-solvable watch-only, so that watch timestamp gets updated
                if (!AddWatchOnlyWithDB(batch, script, timestamp)) {
                    return false;
                }
            }
        }
        return true;
        */
    }
    
    pub fn get_keys(&self) -> HashSet<KeyID> {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        if (!m_storage.HasEncryptionKeys()) {
            return FillableSigningProvider::GetKeys();
        }
        std::set<CKeyID> set_address;
        for (const auto& mi : mapCryptedKeys) {
            set_address.insert(mi.first);
        }
        return set_address;
        */
    }
}

