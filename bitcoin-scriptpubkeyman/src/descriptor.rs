crate::ix!();

///-----------------------
pub struct DescriptorScriptPubKeyMan {
    base: ScriptPubKeyMan,

    max_cached_index:              i32, // default = -1

    /**
      | keeps track of whether Unlock has run
      | a thorough check before
      |
      */
    decryption_thoroughly_checked: bool, // default = false

    cs_desc_man:                   Arc<Mutex<DescriptorScriptPubkeyManInner>>,
}

pub struct DescriptorScriptPubkeyManInner {
    map_script_pub_keys:           DescriptorScriptPubkeyManScriptPubKeyMap,
    map_pubkeys:                   DescriptorScriptPubkeyManPubKeyMap,
    map_keys:                      DescriptorScriptPubkeyManKeyMap,
    map_crypted_keys:              DescriptorScriptPubkeyManCryptedKeyMap,
    wallet_descriptor:             WalletDescriptor,
}

/**
  | Map of scripts to descriptor range index
  |
  */
pub type DescriptorScriptPubkeyManScriptPubKeyMap = HashMap<Script,i32>;

/**
  | Map of pubkeys involved in scripts to
  | descriptor range index
  |
  */
pub type DescriptorScriptPubkeyManPubKeyMap       = HashMap<PubKey,i32>;

pub type DescriptorScriptPubkeyManCryptedKeyMap   = HashMap<KeyID,(PubKey,Vec<u8>)>;
pub type DescriptorScriptPubkeyManKeyMap          = HashMap<KeyID,Key>;

impl DescriptorScriptPubKeyMan {

    pub fn new_with_descriptor(
        storage:    &mut dyn WalletStorage,
        descriptor: &mut WalletDescriptor) -> Self {
    
        todo!();
        /*
        : script_pub_key_man(storage),
        : wallet_descriptor(descriptor),
        */
    }
    
    pub fn new<'a>(storage: &'a mut dyn WalletStorage) -> Self {
    
        todo!();
        /*
        : script_pub_key_man(storage),
        */
    }
    
    /**
      | Provide a descriptor at setup time
      | 
      | Returns false if already setup or setup
      | fails, true if setup is successful
      |
      */
    pub fn setup_descriptor(&mut self, desc: Box<dyn Descriptor>) -> bool {
        
        todo!();
        /*
        
        */
    }

    pub fn is_mine(&self, script: &Script) -> IsMineType {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        if (m_map_script_pub_keys.count(script) > 0) {
            return ISMINE_SPENDABLE;
        }
        return ISMINE_NO;
        */
    }
    
    pub fn check_decryption_key(&mut self, 
        master_key:     &KeyingMaterial,
        accept_no_keys: Option<bool>) -> bool {

        let accept_no_keys: bool = accept_no_keys.unwrap_or(false);
        
        todo!();
        /*
            LOCK(cs_desc_man);
        if (!m_map_keys.empty()) {
            return false;
        }

        bool keyPass = m_map_crypted_keys.empty(); // Always pass when there are no encrypted keys
        bool keyFail = false;
        for (const auto& mi : m_map_crypted_keys) {
            const CPubKey &pubkey = mi.second.first;
            const std::vector<unsigned char> &crypted_secret = mi.second.second;
            CKey key;
            if (!DecryptKey(master_key, crypted_secret, pubkey, key)) {
                keyFail = true;
                break;
            }
            keyPass = true;
            if (m_decryption_thoroughly_checked)
                break;
        }
        if (keyPass && keyFail) {
            LogPrintf("The wallet is probably corrupted: Some keys decrypt but not all.\n");
            throw std::runtime_error("Error unlocking wallet: some keys decrypt but not all. Your wallet file may be corrupt.");
        }
        if (keyFail || (!keyPass && !accept_no_keys)) {
            return false;
        }
        m_decryption_thoroughly_checked = true;
        return true;
        */
    }
    
    pub fn encrypt(&mut self, 
        master_key: &KeyingMaterial,
        batch:      *mut WalletBatch) -> bool {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        if (!m_map_crypted_keys.empty()) {
            return false;
        }

        for (const KeyMap::value_type& key_in : m_map_keys)
        {
            const CKey &key = key_in.second;
            CPubKey pubkey = key.GetPubKey();
            CKeyingMaterial secret(key.begin(), key.end());
            std::vector<unsigned char> crypted_secret;
            if (!EncryptSecret(master_key, secret, pubkey.GetHash(), crypted_secret)) {
                return false;
            }
            m_map_crypted_keys[pubkey.GetID()] = make_pair(pubkey, crypted_secret);
            batch->WriteCryptedDescriptorKey(GetID(), pubkey, crypted_secret);
        }
        m_map_keys.clear();
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
            LOCK(cs_desc_man);
        bool result = GetNewDestination(type, address, error);
        index = m_wallet_descriptor.next_index - 1;
        return result;
        */
    }
    
    pub fn return_destination(&mut self, 
        index:    i64,
        internal: bool,
        addr:     &TxDestination)  {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        // Only return when the index was the most recent
        if (m_wallet_descriptor.next_index - 1 == index) {
            m_wallet_descriptor.next_index--;
        }
        WalletBatch(m_storage.GetDatabase()).WriteDescriptor(GetID(), m_wallet_descriptor);
        NotifyCanGetAddressesChanged();
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_desc_man)]
    pub fn get_keys(&self) -> HashMap<KeyID,Key> {
        
        todo!();
        /*
            AssertLockHeld(cs_desc_man);
        if (m_storage.HasEncryptionKeys() && !m_storage.IsLocked()) {
            KeyMap keys;
            for (auto key_pair : m_map_crypted_keys) {
                const CPubKey& pubkey = key_pair.second.first;
                const std::vector<unsigned char>& crypted_secret = key_pair.second.second;
                CKey key;
                DecryptKey(m_storage.GetEncryptionKey(), crypted_secret, pubkey, key);
                keys[pubkey.GetID()] = key;
            }
            return keys;
        }
        return m_map_keys;
        */
    }
    
    /**
      | Tops up the descriptor cache and
      | m_map_script_pub_keys. The cache is stored
      | in the wallet file and is used to expand
      | the descriptor in
      | GetNewDestination. DescriptorScriptPubKeyMan
      | relies more on ephemeral data than
      | LegacyScriptPubKeyMan. For wallets using
      | unhardened derivation (with or without
      | private keys), the "keypool" is a single
      | xpub.
      */
    pub fn top_up(&mut self, size: Option<u32>) -> bool {

        let size: u32 = size.unwrap_or(0);
        
        todo!();
        /*
            LOCK(cs_desc_man);
        unsigned int target_size;
        if (size > 0) {
            target_size = size;
        } else {
            target_size = std::max(gArgs.GetIntArg("-keypool", DEFAULT_KEYPOOL_SIZE), (int64_t) 1);
        }

        // Calculate the new range_end
        int32_t new_range_end = std::max(m_wallet_descriptor.next_index + (int32_t)target_size, m_wallet_descriptor.range_end);

        // If the descriptor is not ranged, we actually just want to fill the first cache item
        if (!m_wallet_descriptor.descriptor->IsRange()) {
            new_range_end = 1;
            m_wallet_descriptor.range_end = 1;
            m_wallet_descriptor.range_start = 0;
        }

        FlatSigningProvider provider;
        provider.keys = GetKeys();

        WalletBatch batch(m_storage.GetDatabase());
        uint256 id = GetID();
        for (int32_t i = m_max_cached_index + 1; i < new_range_end; ++i) {
            FlatSigningProvider out_keys;
            std::vector<CScript> scripts_temp;
            DescriptorCache temp_cache;
            // Maybe we have a cached xpub and we can expand from the cache first
            if (!m_wallet_descriptor.descriptor->ExpandFromCache(i, m_wallet_descriptor.cache, scripts_temp, out_keys)) {
                if (!m_wallet_descriptor.descriptor->Expand(i, provider, scripts_temp, out_keys, &temp_cache)) return false;
            }
            // Add all of the scriptPubKeys to the scriptPubKey set
            for (const CScript& script : scripts_temp) {
                m_map_script_pub_keys[script] = i;
            }
            for (const auto& pk_pair : out_keys.pubkeys) {
                const CPubKey& pubkey = pk_pair.second;
                if (m_map_pubkeys.count(pubkey) != 0) {
                    // We don't need to give an error here.
                    // It doesn't matter which of many valid indexes the pubkey has, we just need an index where we can derive it and it's private key
                    continue;
                }
                m_map_pubkeys[pubkey] = i;
            }
            // Merge and write the cache
            DescriptorCache new_items = m_wallet_descriptor.cache.MergeAndDiff(temp_cache);
            if (!batch.WriteDescriptorCacheItems(id, new_items)) {
                throw std::runtime_error(std::string(__func__) + ": writing cache items failed");
            }
            m_max_cached_index++;
        }
        m_wallet_descriptor.range_end = new_range_end;
        batch.WriteDescriptor(GetID(), m_wallet_descriptor);

        // By this point, the cache size should be the size of the entire range
        assert(m_wallet_descriptor.range_end - 1 == m_max_cached_index);

        NotifyCanGetAddressesChanged();
        return true;
        */
    }
    
    pub fn mark_unused_addresses(&mut self, script: &Script)  {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        if (IsMine(script)) {
            int32_t index = m_map_script_pub_keys[script];
            if (index >= m_wallet_descriptor.next_index) {
                WalletLogPrintf("%s: Detected a used keypool item at index %d, mark all keypool items up to this item as used\n", __func__, index);
                m_wallet_descriptor.next_index = index + 1;
            }
            if (!TopUp()) {
                WalletLogPrintf("%s: Topping up keypool failed (locked wallet)\n", __func__);
            }
        }
        */
    }
    
    pub fn add_descriptor_key(&mut self, 
        key:    &Key,
        pubkey: &PubKey)  {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        WalletBatch batch(m_storage.GetDatabase());
        if (!AddDescriptorKeyWithDB(batch, key, pubkey)) {
            throw std::runtime_error(std::string(__func__) + ": writing descriptor private key failed");
        }
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_desc_man)]
    pub fn add_descriptor_key_withdb(&mut self, 
        batch:  &mut WalletBatch,
        key:    &Key,
        pubkey: &PubKey) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_desc_man);
        assert(!m_storage.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS));

        // Check if provided key already exists
        if (m_map_keys.find(pubkey.GetID()) != m_map_keys.end() ||
            m_map_crypted_keys.find(pubkey.GetID()) != m_map_crypted_keys.end()) {
            return true;
        }

        if (m_storage.HasEncryptionKeys()) {
            if (m_storage.IsLocked()) {
                return false;
            }

            std::vector<unsigned char> crypted_secret;
            CKeyingMaterial secret(key.begin(), key.end());
            if (!EncryptSecret(m_storage.GetEncryptionKey(), secret, pubkey.GetHash(), crypted_secret)) {
                return false;
            }

            m_map_crypted_keys[pubkey.GetID()] = make_pair(pubkey, crypted_secret);
            return batch.WriteCryptedDescriptorKey(GetID(), pubkey, crypted_secret);
        } else {
            m_map_keys[pubkey.GetID()] = key;
            return batch.WriteDescriptorKey(GetID(), pubkey, key.GetPrivKey());
        }
        */
    }
    
    /**
      | Setup descriptors based on the given
      | Extkey
      |
      */
    pub fn setup_descriptor_generation(&mut self, 
        master_key: &ExtKey,
        addr_type:  OutputType,
        internal:   bool) -> bool {
        
        todo!();
        /*
            if (addr_type == OutputType::BECH32M) {
            // Don't allow setting up taproot descriptors yet
            // TODO: Allow setting up taproot descriptors
            return false;
        }

        LOCK(cs_desc_man);
        assert(m_storage.IsWalletFlagSet(WALLET_FLAG_DESCRIPTORS));

        // Ignore when there is already a descriptor
        if (m_wallet_descriptor.descriptor) {
            return false;
        }

        int64_t creation_time = GetTime();

        std::string xpub = EncodeExtPubKey(master_key.Neuter());

        // Build descriptor string
        std::string desc_prefix;
        std::string desc_suffix = "/\*)";
        switch (addr_type) {
        case OutputType::LEGACY: {
            desc_prefix = "pkh(" + xpub + "/44'";
            break;
        }
        case OutputType::P2SH_SEGWIT: {
            desc_prefix = "sh(wpkh(" + xpub + "/49'";
            desc_suffix += ")";
            break;
        }
        case OutputType::BECH32: {
            desc_prefix = "wpkh(" + xpub + "/84'";
            break;
        }
        case OutputType::BECH32M: assert(false); // TODO: Setup taproot descriptor
        } // no default case, so the compiler can warn about missing cases
        assert(!desc_prefix.empty());

        // Mainnet derives at 0', testnet and regtest derive at 1'
        if (Params().IsTestChain()) {
            desc_prefix += "/1'";
        } else {
            desc_prefix += "/0'";
        }

        std::string internal_path = internal ? "/1" : "/0";
        std::string desc_str = desc_prefix + "/0'" + internal_path + desc_suffix;

        // Make the descriptor
        FlatSigningProvider keys;
        std::string error;
        std::unique_ptr<Descriptor> desc = Parse(desc_str, keys, error, false);
        WalletDescriptor w_desc(std::move(desc), creation_time, 0, 0, 0);
        m_wallet_descriptor = w_desc;

        // Store the master private key, and descriptor
        WalletBatch batch(m_storage.GetDatabase());
        if (!AddDescriptorKeyWithDB(batch, master_key.key, master_key.key.GetPubKey())) {
            throw std::runtime_error(std::string(__func__) + ": writing descriptor master private key failed");
        }
        if (!batch.WriteDescriptor(GetID(), m_wallet_descriptor)) {
            throw std::runtime_error(std::string(__func__) + ": writing descriptor failed");
        }

        // TopUp
        TopUp();

        m_storage.UnsetBlankWalletFlag(batch);
        return true;
        */
    }
    
    pub fn is_hd_enabled(&self) -> bool {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        return m_wallet_descriptor.descriptor->IsRange();
        */
    }
    
    pub fn can_get_addresses(&self, internal: Option<bool>) -> bool {
        let internal: bool = internal.unwrap_or(false);
        
        todo!();
        /*
            // We can only give out addresses from descriptors that are single type (not combo), ranged,
        // and either have cached keys or can generate more keys (ignoring encryption)
        LOCK(cs_desc_man);
        return m_wallet_descriptor.descriptor->IsSingleType() &&
               m_wallet_descriptor.descriptor->IsRange() &&
               (HavePrivateKeys() || m_wallet_descriptor.next_index < m_wallet_descriptor.range_end);
        */
    }
    
    pub fn have_private_keys(&self) -> bool {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        return m_map_keys.size() > 0 || m_map_crypted_keys.size() > 0;
        */
    }
    
    pub fn get_oldest_key_pool_time(&self) -> i64 {
        
        todo!();
        /*
            // This is only used for getwalletinfo output and isn't relevant to descriptor wallets.
        // The magic number 0 indicates that it shouldn't be displayed so that's what we return.
        return 0;
        */
    }
    
    pub fn get_key_pool_size(&self) -> u32 {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        return m_wallet_descriptor.range_end - m_wallet_descriptor.next_index;
        */
    }
    
    pub fn get_time_first_key(&self) -> i64 {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        return m_wallet_descriptor.creation_time;
        */
    }
    
    /**
      | Fetch the SigningProvider for the given
      | script and optionally include private
      | keys
      |
      */
    pub fn get_signing_provider_with_script(&self, 
        script:          &Script,
        include_private: Option<bool>) -> Box<FlatSigningProvider> {

        let include_private: bool = include_private.unwrap_or(false);
        
        todo!();
        /*
            LOCK(cs_desc_man);

        // Find the index of the script
        auto it = m_map_script_pub_keys.find(script);
        if (it == m_map_script_pub_keys.end()) {
            return nullptr;
        }
        int32_t index = it->second;

        return GetSigningProvider(index, include_private);
        */
    }
    
    /**
      | Fetch the SigningProvider for the given
      | pubkey and always include private keys.
      | This should only be called by signing
      | code.
      |
      */
    pub fn get_signing_provider_with_pubkey(&self, pubkey: &PubKey) -> Box<FlatSigningProvider> {
        
        todo!();
        /*
            LOCK(cs_desc_man);

        // Find index of the pubkey
        auto it = m_map_pubkeys.find(pubkey);
        if (it == m_map_pubkeys.end()) {
            return nullptr;
        }
        int32_t index = it->second;

        // Always try to get the signing provider with private keys. This function should only be called during signing anyways
        return GetSigningProvider(index, true);
        */
    }
    
    /**
      | Fetch the SigningProvider for a given index
      | and optionally include private keys. Called
      | by the above functions.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_desc_man)]
    pub fn get_signing_provider_with_index(&self, 
        index:           i32,
        include_private: Option<bool>) -> Box<FlatSigningProvider> {

        let include_private: bool = include_private.unwrap_or(false);
        
        todo!();
        /*
            AssertLockHeld(cs_desc_man);
        // Get the scripts, keys, and key origins for this script
        std::unique_ptr<FlatSigningProvider> out_keys = std::make_unique<FlatSigningProvider>();
        std::vector<CScript> scripts_temp;
        if (!m_wallet_descriptor.descriptor->ExpandFromCache(index, m_wallet_descriptor.cache, scripts_temp, *out_keys)) return nullptr;

        if (HavePrivateKeys() && include_private) {
            FlatSigningProvider master_provider;
            master_provider.keys = GetKeys();
            m_wallet_descriptor.descriptor->ExpandPrivate(index, master_provider, *out_keys);
        }

        return out_keys;
        */
    }
    
    pub fn get_solving_provider(&self, script: &Script) -> Box<SigningProvider> {
        
        todo!();
        /*
            return GetSigningProvider(script, false);
        */
    }
    
    pub fn can_provide(&mut self, 
        script:  &Script,
        sigdata: &mut SignatureData) -> bool {
        
        todo!();
        /*
            return IsMine(script);
        */
    }
    
    pub fn sign_transaction(&self, 
        tx:           &mut MutableTransaction,
        coins:        &HashMap<OutPoint,Coin>,
        sighash:      i32,
        input_errors: &mut HashMap<i32,BilingualStr>) -> bool {
        
        todo!();
        /*
            std::unique_ptr<FlatSigningProvider> keys = std::make_unique<FlatSigningProvider>();
        for (const auto& coin_pair : coins) {
            std::unique_ptr<FlatSigningProvider> coin_keys = GetSigningProvider(coin_pair.second.out.scriptPubKey, true);
            if (!coin_keys) {
                continue;
            }
            *keys = Merge(*keys, *coin_keys);
        }

        return ::SignTransaction(tx, keys.get(), coins, sighash, input_errors);
        */
    }
    
    pub fn sign_message(&self, 
        message: &String,
        pkhash:  &PKHash,
        str_sig: &mut String) -> SigningResult {
        
        todo!();
        /*
            std::unique_ptr<FlatSigningProvider> keys = GetSigningProvider(GetScriptForDestination(pkhash), true);
        if (!keys) {
            return SigningResult::PRIVATE_KEY_NOT_AVAILABLE;
        }

        CKey key;
        if (!keys->GetKey(ToKeyID(pkhash), key)) {
            return SigningResult::PRIVATE_KEY_NOT_AVAILABLE;
        }

        if (!MessageSign(key, message, str_sig)) {
            return SigningResult::SIGNING_FAILED;
        }
        return SigningResult::OK;
        */
    }
    
    pub fn fillpsbt(&self, 
        psbtx:        &mut PartiallySignedTransaction,
        txdata:       &PrecomputedTransactionData,
        sighash_type: Option<i32>,
        sign:         Option<bool>,
        bip_32derivs: Option<bool>,
        n_signed:     *mut i32) -> TransactionError {

        let sighash_type:  i32 = sighash_type.unwrap_or(1 );//SIGHASH_ALL
        let sign:         bool = sign.unwrap_or(true);
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

            // Get the scriptPubKey to know which SigningProvider to use
            CScript script;
            if (!input.witness_utxo.IsNull()) {
                script = input.witness_utxo.scriptPubKey;
            } else if (input.non_witness_utxo) {
                if (txin.prevout.n >= input.non_witness_utxo->vout.size()) {
                    return TransactionError::MISSING_INPUTS;
                }
                script = input.non_witness_utxo->vout[txin.prevout.n].scriptPubKey;
            } else {
                // There's no UTXO so we can just skip this now
                continue;
            }
            SignatureData sigdata;
            input.FillSignatureData(sigdata);

            std::unique_ptr<FlatSigningProvider> keys = std::make_unique<FlatSigningProvider>();
            std::unique_ptr<FlatSigningProvider> script_keys = GetSigningProvider(script, sign);
            if (script_keys) {
                *keys = Merge(*keys, *script_keys);
            } else {
                // Maybe there are pubkeys listed that we can sign for
                script_keys = std::make_unique<FlatSigningProvider>();
                for (const auto& pk_pair : input.hd_keypaths) {
                    const CPubKey& pubkey = pk_pair.first;
                    std::unique_ptr<FlatSigningProvider> pk_keys = GetSigningProvider(pubkey);
                    if (pk_keys) {
                        *keys = Merge(*keys, *pk_keys);
                    }
                }
            }

            SignPSBTInput(HidingSigningProvider(keys.get(), !sign, !bip32derivs), psbtx, i, &txdata, sighash_type);

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
            std::unique_ptr<SigningProvider> keys = GetSolvingProvider(psbtx.tx->vout.at(i).scriptPubKey);
            if (!keys) {
                continue;
            }
            UpdatePSBTOutput(HidingSigningProvider(keys.get(), true, !bip32derivs), psbtx, i);
        }

        return TransactionError::OK;
        */
    }
    
    pub fn get_metadata(&self, dest: &TxDestination) -> Box<KeyMetadata> {
        
        todo!();
        /*
            std::unique_ptr<SigningProvider> provider = GetSigningProvider(GetScriptForDestination(dest));
        if (provider) {
            KeyOriginInfo orig;
            CKeyID key_id = GetKeyForDestination(*provider, dest);
            if (provider->GetKeyOrigin(key_id, orig)) {
                LOCK(cs_desc_man);
                std::unique_ptr<CKeyMetadata> meta = std::make_unique<CKeyMetadata>();
                meta->key_origin = orig;
                meta->has_key_origin = true;
                meta->nCreateTime = m_wallet_descriptor.creation_time;
                return meta;
            }
        }
        return nullptr;
        */
    }
    
    pub fn getid(&self) -> u256 {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        std::string desc_str = m_wallet_descriptor.descriptor->ToString();
        uint256 id;
        CSHA256().Write((unsigned char*)desc_str.data(), desc_str.size()).Finalize(id.begin());
        return id;
        */
    }
    
    pub fn set_cache(&mut self, cache: &DescriptorCache)  {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        m_wallet_descriptor.cache = cache;
        for (int32_t i = m_wallet_descriptor.range_start; i < m_wallet_descriptor.range_end; ++i) {
            FlatSigningProvider out_keys;
            std::vector<CScript> scripts_temp;
            if (!m_wallet_descriptor.descriptor->ExpandFromCache(i, m_wallet_descriptor.cache, scripts_temp, out_keys)) {
                throw std::runtime_error("Error: Unable to expand wallet descriptor from cache");
            }
            // Add all of the scriptPubKeys to the scriptPubKey set
            for (const CScript& script : scripts_temp) {
                if (m_map_script_pub_keys.count(script) != 0) {
                    throw std::runtime_error(strprintf("Error: Already loaded script at index %d as being at index %d", i, m_map_script_pub_keys[script]));
                }
                m_map_script_pub_keys[script] = i;
            }
            for (const auto& pk_pair : out_keys.pubkeys) {
                const CPubKey& pubkey = pk_pair.second;
                if (m_map_pubkeys.count(pubkey) != 0) {
                    // We don't need to give an error here.
                    // It doesn't matter which of many valid indexes the pubkey has, we just need an index where we can derive it and it's private key
                    continue;
                }
                m_map_pubkeys[pubkey] = i;
            }
            m_max_cached_index++;
        }
        */
    }
    
    pub fn add_key(&mut self, 
        key_id: &KeyID,
        key:    &Key) -> bool {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        m_map_keys[key_id] = key;
        return true;
        */
    }
    
    pub fn add_crypted_key(&mut self, 
        key_id:      &KeyID,
        pubkey:      &PubKey,
        crypted_key: &Vec<u8>) -> bool {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        if (!m_map_keys.empty()) {
            return false;
        }

        m_map_crypted_keys[key_id] = make_pair(pubkey, crypted_key);
        return true;
        */
    }
    
    pub fn has_wallet_descriptor(&self, desc: &WalletDescriptor) -> bool {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        return m_wallet_descriptor.descriptor != nullptr && desc.descriptor != nullptr && m_wallet_descriptor.descriptor->ToString() == desc.descriptor->ToString();
        */
    }
    
    pub fn write_descriptor(&mut self)  {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        WalletBatch batch(m_storage.GetDatabase());
        if (!batch.WriteDescriptor(GetID(), m_wallet_descriptor)) {
            throw std::runtime_error(std::string(__func__) + ": writing descriptor failed");
        }
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_desc_man)]
    pub fn get_wallet_descriptor(&self) -> WalletDescriptor {
        
        todo!();
        /*
            return m_wallet_descriptor;
        */
    }
    
    pub fn get_script_pub_keys(&self) -> Vec<Script> {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        std::vector<CScript> script_pub_keys;
        script_pub_keys.reserve(m_map_script_pub_keys.size());

        for (auto const& script_pub_key: m_map_script_pub_keys) {
            script_pub_keys.push_back(script_pub_key.first);
        }
        return script_pub_keys;
        */
    }
    
    pub fn get_descriptor_string(&self, 
        out:   &mut String,
        priv_: bool) -> bool {
        
        todo!();
        /*
            LOCK(cs_desc_man);

        FlatSigningProvider provider;
        provider.keys = GetKeys();

        if (priv) {
            // For the private version, always return the master key to avoid
            // exposing child private keys. The risk implications of exposing child
            // private keys together with the parent xpub may be non-obvious for users.
            return m_wallet_descriptor.descriptor->ToPrivateString(provider, out);
        }

        return m_wallet_descriptor.descriptor->ToNormalizedString(provider, out, &m_wallet_descriptor.cache);
        */
    }
    
    pub fn upgrade_descriptor_cache(&mut self)  {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        if (m_storage.IsLocked() || m_storage.IsWalletFlagSet(WALLET_FLAG_LAST_HARDENED_XPUB_CACHED)) {
            return;
        }

        // Skip if we have the last hardened xpub cache
        if (m_wallet_descriptor.cache.GetCachedLastHardenedExtPubKeys().size() > 0) {
            return;
        }

        // Expand the descriptor
        FlatSigningProvider provider;
        provider.keys = GetKeys();
        FlatSigningProvider out_keys;
        std::vector<CScript> scripts_temp;
        DescriptorCache temp_cache;
        if (!m_wallet_descriptor.descriptor->Expand(0, provider, scripts_temp, out_keys, &temp_cache)){
            throw std::runtime_error("Unable to expand descriptor");
        }

        // Cache the last hardened xpubs
        DescriptorCache diff = m_wallet_descriptor.cache.MergeAndDiff(temp_cache);
        if (!WalletBatch(m_storage.GetDatabase()).WriteDescriptorCacheItems(GetID(), diff)) {
            throw std::runtime_error(std::string(__func__) + ": writing cache items failed");
        }
        */
    }
    
    pub fn update_wallet_descriptor(&mut self, descriptor: &mut WalletDescriptor)  {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        std::string error;
        if (!CanUpdateToWalletDescriptor(descriptor, error)) {
            throw std::runtime_error(std::string(__func__) + ": " + error);
        }

        m_map_pubkeys.clear();
        m_map_script_pub_keys.clear();
        m_max_cached_index = -1;
        m_wallet_descriptor = descriptor;
        */
    }
    
    pub fn can_update_to_wallet_descriptor(&mut self, 
        descriptor: &WalletDescriptor,
        error:      &mut String) -> bool {
        
        todo!();
        /*
            LOCK(cs_desc_man);
        if (!HasWalletDescriptor(descriptor)) {
            error = "can only update matching descriptor";
            return false;
        }

        if (descriptor.range_start > m_wallet_descriptor.range_start ||
            descriptor.range_end < m_wallet_descriptor.range_end) {
            // Use inclusive range for error
            error = strprintf("new range must include current range = [%d,%d]",
                              m_wallet_descriptor.range_start,
                              m_wallet_descriptor.range_end - 1);
            return false;
        }

        return true;
        */
    }
}

impl GetNewDestination for DescriptorScriptPubKeyMan {
    
    fn get_new_destination(&mut self, 
        ty:    OutputType,
        dest:  &mut TxDestination,
        error: &mut BilingualStr) -> bool {
        
        todo!();
        /*
            // Returns true if this descriptor supports getting new addresses. Conditions where we may be unable to fetch them (e.g. locked) are caught later
        if (!CanGetAddresses()) {
            error = _("No addresses available");
            return false;
        }
        {
            LOCK(cs_desc_man);
            assert(m_wallet_descriptor.descriptor->IsSingleType()); // This is a combo descriptor which should not be an active descriptor
            std::optional<OutputType> desc_addr_type = m_wallet_descriptor.descriptor->GetOutputType();
            assert(desc_addr_type);
            if (type != *desc_addr_type) {
                throw std::runtime_error(std::string(__func__) + ": Types are inconsistent");
            }

            TopUp();

            // Get the scriptPubKey from the descriptor
            FlatSigningProvider out_keys;
            std::vector<CScript> scripts_temp;
            if (m_wallet_descriptor.range_end <= m_max_cached_index && !TopUp(1)) {
                // We can't generate anymore keys
                error = _("Error: Keypool ran out, please call keypoolrefill first");
                return false;
            }
            if (!m_wallet_descriptor.descriptor->ExpandFromCache(m_wallet_descriptor.next_index, m_wallet_descriptor.cache, scripts_temp, out_keys)) {
                // We can't generate anymore keys
                error = _("Error: Keypool ran out, please call keypoolrefill first");
                return false;
            }

            std::optional<OutputType> out_script_type = m_wallet_descriptor.descriptor->GetOutputType();
            if (out_script_type && out_script_type == type) {
                ExtractDestination(scripts_temp[0], dest);
            } else {
                throw std::runtime_error(std::string(__func__) + ": Types are inconsistent. Stored type does not match type of newly generated address");
            }
            m_wallet_descriptor.next_index++;
            WalletBatch(m_storage.GetDatabase()).WriteDescriptor(GetID(), m_wallet_descriptor);
            return true;
        }
        */
    }
}
