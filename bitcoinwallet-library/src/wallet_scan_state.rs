// ---------------- [ File: bitcoinwallet-library/src/wallet_scan_state.rs ]
crate::ix!();

#[derive(Default)]
pub struct WalletScanState {
    n_keys:                u32, // default = { 0 }
    n_ckeys:               u32, // default = { 0 }
    n_watch_keys:          u32, // default = { 0 }
    n_key_meta:            u32, // default = { 0 }
    unknown_records:       u32, // default = { 0 }
    is_encrypted:          bool, // default = { false }
    any_unordered:         bool, // default = { false }
    wallet_upgrade:        Vec<u256>,
    active_external_spks:  HashMap<OutputType,u256>,
    active_internal_spks:  HashMap<OutputType,u256>,
    descriptor_caches:     HashMap<u256,DescriptorCache>,
    descriptor_keys:       HashMap<(u256,KeyID),Key>,
    descriptor_crypt_keys: HashMap<(u256,KeyID),(PubKey,Vec<u8>)>,
    hd_chains:             HashMap<u160,HDChain>,
    tx_corrupt:            bool, // default = { false }
}

#[EXCLUSIVE_LOCKS_REQUIRED(pwallet->cs_wallet)]
pub fn read_key_value_with_wallet_scan_state(
        pwallet:   *mut Wallet,
        ss_key:    &mut DataStream,
        ss_value:  &mut DataStream,
        wss:       &mut WalletScanState,
        str_type:  &mut String,
        str_err:   &mut String,
        filter_fn: Option<&KeyFilterFn>) -> bool {

    todo!();
        /*
            try {
            // Unserialize
            // Taking advantage of the fact that pair serialization
            // is just the two items serialized one after the other
            ssKey >> strType;
            // If we have a filter, check if this matches the filter
            if (filter_fn && !filter_fn(strType)) {
                return true;
            }
            if (strType == DBKeys::NAME) {
                std::string strAddress;
                ssKey >> strAddress;
                std::string label;
                ssValue >> label;
                pwallet->m_address_book[DecodeDestination(strAddress)].SetLabel(label);
            } else if (strType == DBKeys::PURPOSE) {
                std::string strAddress;
                ssKey >> strAddress;
                ssValue >> pwallet->m_address_book[DecodeDestination(strAddress)].purpose;
            } else if (strType == DBKeys::TX) {
                uint256 hash;
                ssKey >> hash;
                // LoadToWallet call below creates a new CWalletTx that fill_wtx
                // callback fills with transaction metadata.
                auto fill_wtx = [&](CWalletTx& wtx, bool new_tx) {
                    if(!new_tx) {
                        // There's some corruption here since the tx we just tried to load was already in the wallet.
                        // We don't consider this type of corruption critical, and can fix it by removing tx data and
                        // rescanning.
                        wss.tx_corrupt = true;
                        return false;
                    }
                    ssValue >> wtx;
                    if (wtx.GetHash() != hash)
                        return false;

                    // Undo serialize changes in 31600
                    if (31404 <= wtx.fTimeReceivedIsTxTime && wtx.fTimeReceivedIsTxTime <= 31703)
                    {
                        if (!ssValue.empty())
                        {
                            uint8_t fTmp;
                            uint8_t fUnused;
                            std::string unused_string;
                            ssValue >> fTmp >> fUnused >> unused_string;
                            strErr = strprintf("LoadWallet() upgrading tx ver=%d %d %s",
                                               wtx.fTimeReceivedIsTxTime, fTmp, hash.ToString());
                            wtx.fTimeReceivedIsTxTime = fTmp;
                        }
                        else
                        {
                            strErr = strprintf("LoadWallet() repairing tx ver=%d %s", wtx.fTimeReceivedIsTxTime, hash.ToString());
                            wtx.fTimeReceivedIsTxTime = 0;
                        }
                        wss.vWalletUpgrade.push_back(hash);
                    }

                    if (wtx.nOrderPos == -1)
                        wss.fAnyUnordered = true;

                    return true;
                };
                if (!pwallet->LoadToWallet(hash, fill_wtx)) {
                    return false;
                }
            } else if (strType == DBKeys::WATCHS) {
                wss.nWatchKeys++;
                CScript script;
                ssKey >> script;
                uint8_t fYes;
                ssValue >> fYes;
                if (fYes == '1') {
                    pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadWatchOnly(script);
                }
            } else if (strType == DBKeys::KEY) {
                CPubKey vchPubKey;
                ssKey >> vchPubKey;
                if (!vchPubKey.IsValid())
                {
                    strErr = "Error reading wallet database: CPubKey corrupt";
                    return false;
                }
                CKey key;
                CPrivKey pkey;
                uint256 hash;

                wss.nKeys++;
                ssValue >> pkey;

                // Old wallets store keys as DBKeys::KEY [pubkey] => [privkey]
                // ... which was slow for wallets with lots of keys, because the public key is re-derived from the private key
                // using EC operations as a checksum.
                // Newer wallets store keys as DBKeys::KEY [pubkey] => [privkey][hash(pubkey,privkey)], which is much faster while
                // remaining backwards-compatible.
                try
                {
                    ssValue >> hash;
                }
                catch (const std::ios_base::failure&) {}

                bool fSkipCheck = false;

                if (!hash.IsNull())
                {
                    // hash pubkey/privkey to accelerate wallet load
                    std::vector<unsigned char> vchKey;
                    vchKey.reserve(vchPubKey.size() + pkey.size());
                    vchKey.insert(vchKey.end(), vchPubKey.begin(), vchPubKey.end());
                    vchKey.insert(vchKey.end(), pkey.begin(), pkey.end());

                    if (Hash(vchKey) != hash)
                    {
                        strErr = "Error reading wallet database: CPubKey/CPrivKey corrupt";
                        return false;
                    }

                    fSkipCheck = true;
                }

                if (!key.Load(pkey, vchPubKey, fSkipCheck))
                {
                    strErr = "Error reading wallet database: CPrivKey corrupt";
                    return false;
                }
                if (!pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadKey(key, vchPubKey))
                {
                    strErr = "Error reading wallet database: LegacyScriptPubKeyMan::LoadKey failed";
                    return false;
                }
            } else if (strType == DBKeys::MASTER_KEY) {
                // Master encryption key is loaded into only the wallet and not any of the ScriptPubKeyMans.
                unsigned int nID;
                ssKey >> nID;
                CMasterKey kMasterKey;
                ssValue >> kMasterKey;
                if(pwallet->mapMasterKeys.count(nID) != 0)
                {
                    strErr = strprintf("Error reading wallet database: duplicate CMasterKey id %u", nID);
                    return false;
                }
                pwallet->mapMasterKeys[nID] = kMasterKey;
                if (pwallet->nMasterKeyMaxID < nID)
                    pwallet->nMasterKeyMaxID = nID;
            } else if (strType == DBKeys::CRYPTED_KEY) {
                CPubKey vchPubKey;
                ssKey >> vchPubKey;
                if (!vchPubKey.IsValid())
                {
                    strErr = "Error reading wallet database: CPubKey corrupt";
                    return false;
                }
                std::vector<unsigned char> vchPrivKey;
                ssValue >> vchPrivKey;

                // Get the checksum and check it
                bool checksum_valid = false;
                if (!ssValue.eof()) {
                    uint256 checksum;
                    ssValue >> checksum;
                    if ((checksum_valid = Hash(vchPrivKey) != checksum)) {
                        strErr = "Error reading wallet database: Encrypted key corrupt";
                        return false;
                    }
                }

                wss.nCKeys++;

                if (!pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadCryptedKey(vchPubKey, vchPrivKey, checksum_valid))
                {
                    strErr = "Error reading wallet database: LegacyScriptPubKeyMan::LoadCryptedKey failed";
                    return false;
                }
                wss.fIsEncrypted = true;
            } else if (strType == DBKeys::KEYMETA) {
                CPubKey vchPubKey;
                ssKey >> vchPubKey;
                CKeyMetadata keyMeta;
                ssValue >> keyMeta;
                wss.nKeyMeta++;
                pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadKeyMetadata(vchPubKey.GetID(), keyMeta);

                // Extract some CHDChain info from this metadata if it has any
                if (keyMeta.nVersion >= CKeyMetadata::VERSION_WITH_HDDATA && !keyMeta.hd_seed_id.IsNull() && keyMeta.hdKeypath.size() > 0) {
                    // Get the path from the key origin or from the path string
                    // Not applicable when path is "s" or "m" as those indicate a seed
                    // See https://github.com/bitcoin/bitcoin/pull/12924
                    bool internal = false;
                    uint32_t index = 0;
                    if (keyMeta.hdKeypath != "s" && keyMeta.hdKeypath != "m") {
                        std::vector<uint32_t> path;
                        if (keyMeta.has_key_origin) {
                            // We have a key origin, so pull it from its path vector
                            path = keyMeta.key_origin.path;
                        } else {
                            // No key origin, have to parse the string
                            if (!ParseHDKeypath(keyMeta.hdKeypath, path)) {
                                strErr = "Error reading wallet database: keymeta with invalid HD keypath";
                                return false;
                            }
                        }

                        // Extract the index and internal from the path
                        // Path string is m/0'/k'/i'
                        // Path vector is [0', k', i'] (but as ints OR'd with the hardened bit
                        // k == 0 for external, 1 for internal. i is the index
                        if (path.size() != 3) {
                            strErr = "Error reading wallet database: keymeta found with unexpected path";
                            return false;
                        }
                        if (path[0] != 0x80000000) {
                            strErr = strprintf("Unexpected path index of 0x%08x (expected 0x80000000) for the element at index 0", path[0]);
                            return false;
                        }
                        if (path[1] != 0x80000000 && path[1] != (1 | 0x80000000)) {
                            strErr = strprintf("Unexpected path index of 0x%08x (expected 0x80000000 or 0x80000001) for the element at index 1", path[1]);
                            return false;
                        }
                        if ((path[2] & 0x80000000) == 0) {
                            strErr = strprintf("Unexpected path index of 0x%08x (expected to be greater than or equal to 0x80000000)", path[2]);
                            return false;
                        }
                        internal = path[1] == (1 | 0x80000000);
                        index = path[2] & ~0x80000000;
                    }

                    // Insert a new CHDChain, or get the one that already exists
                    auto ins = wss.m_hd_chains.emplace(keyMeta.hd_seed_id, CHDChain());
                    CHDChain& chain = ins.first->second;
                    if (ins.second) {
                        // For new chains, we want to default to VERSION_HD_BASE until we see an internal
                        chain.nVersion = CHDChain::VERSION_HD_BASE;
                        chain.seed_id = keyMeta.hd_seed_id;
                    }
                    if (internal) {
                        chain.nVersion = CHDChain::VERSION_HD_CHAIN_SPLIT;
                        chain.nInternalChainCounter = std::max(chain.nInternalChainCounter, index);
                    } else {
                        chain.nExternalChainCounter = std::max(chain.nExternalChainCounter, index);
                    }
                }
            } else if (strType == DBKeys::WATCHMETA) {
                CScript script;
                ssKey >> script;
                CKeyMetadata keyMeta;
                ssValue >> keyMeta;
                wss.nKeyMeta++;
                pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadScriptMetadata(CScriptID(script), keyMeta);
            } else if (strType == DBKeys::DEFAULTKEY) {
                // We don't want or need the default key, but if there is one set,
                // we want to make sure that it is valid so that we can detect corruption
                CPubKey vchPubKey;
                ssValue >> vchPubKey;
                if (!vchPubKey.IsValid()) {
                    strErr = "Error reading wallet database: Default Key corrupt";
                    return false;
                }
            } else if (strType == DBKeys::POOL) {
                int64_t nIndex;
                ssKey >> nIndex;
                CKeyPool keypool;
                ssValue >> keypool;

                pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadKeyPool(nIndex, keypool);
            } else if (strType == DBKeys::CSCRIPT) {
                u160 hash;
                ssKey >> hash;
                CScript script;
                ssValue >> script;
                if (!pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadCScript(script))
                {
                    strErr = "Error reading wallet database: LegacyScriptPubKeyMan::LoadCScript failed";
                    return false;
                }
            } else if (strType == DBKeys::ORDERPOSNEXT) {
                ssValue >> pwallet->nOrderPosNext;
            } else if (strType == DBKeys::DESTDATA) {
                std::string strAddress, strKey, strValue;
                ssKey >> strAddress;
                ssKey >> strKey;
                ssValue >> strValue;
                pwallet->LoadDestData(DecodeDestination(strAddress), strKey, strValue);
            } else if (strType == DBKeys::HDCHAIN) {
                CHDChain chain;
                ssValue >> chain;
                pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadHDChain(chain);
            } else if (strType == DBKeys::OLD_KEY) {
                strErr = "Found unsupported 'wkey' record, try loading with version 0.18";
                return false;
            } else if (strType == DBKeys::ACTIVEEXTERNALSPK || strType == DBKeys::ACTIVEINTERNALSPK) {
                uint8_t type;
                ssKey >> type;
                uint256 id;
                ssValue >> id;

                bool internal = strType == DBKeys::ACTIVEINTERNALSPK;
                auto& spk_mans = internal ? wss.m_active_internal_spks : wss.m_active_external_spks;
                if (spk_mans.count(static_cast<OutputType>(type)) > 0) {
                    strErr = "Multiple ScriptPubKeyMans specified for a single type";
                    return false;
                }
                spk_mans[static_cast<OutputType>(type)] = id;
            } else if (strType == DBKeys::WALLETDESCRIPTOR) {
                uint256 id;
                ssKey >> id;
                WalletDescriptor desc;
                ssValue >> desc;
                if (wss.m_descriptor_caches.count(id) == 0) {
                    wss.m_descriptor_caches[id] = DescriptorCache();
                }
                pwallet->LoadDescriptorScriptPubKeyMan(id, desc);
            } else if (strType == DBKeys::WALLETDESCRIPTORCACHE) {
                bool parent = true;
                uint256 desc_id;
                uint32_t key_exp_index;
                uint32_t der_index;
                ssKey >> desc_id;
                ssKey >> key_exp_index;

                // if the der_index exists, it's a derived xpub
                try
                {
                    ssKey >> der_index;
                    parent = false;
                }
                catch (...) {}

                std::vector<unsigned char> ser_xpub(BIP32_EXTKEY_SIZE);
                ssValue >> ser_xpub;
                CExtPubKey xpub;
                xpub.Decode(ser_xpub.data());
                if (parent) {
                    wss.m_descriptor_caches[desc_id].CacheParentExtPubKey(key_exp_index, xpub);
                } else {
                    wss.m_descriptor_caches[desc_id].CacheDerivedExtPubKey(key_exp_index, der_index, xpub);
                }
            } else if (strType == DBKeys::WALLETDESCRIPTORLHCACHE) {
                uint256 desc_id;
                uint32_t key_exp_index;
                ssKey >> desc_id;
                ssKey >> key_exp_index;

                std::vector<unsigned char> ser_xpub(BIP32_EXTKEY_SIZE);
                ssValue >> ser_xpub;
                CExtPubKey xpub;
                xpub.Decode(ser_xpub.data());
                wss.m_descriptor_caches[desc_id].CacheLastHardenedExtPubKey(key_exp_index, xpub);
            } else if (strType == DBKeys::WALLETDESCRIPTORKEY) {
                uint256 desc_id;
                CPubKey pubkey;
                ssKey >> desc_id;
                ssKey >> pubkey;
                if (!pubkey.IsValid())
                {
                    strErr = "Error reading wallet database: CPubKey corrupt";
                    return false;
                }
                CKey key;
                CPrivKey pkey;
                uint256 hash;

                wss.nKeys++;
                ssValue >> pkey;
                ssValue >> hash;

                // hash pubkey/privkey to accelerate wallet load
                std::vector<unsigned char> to_hash;
                to_hash.reserve(pubkey.size() + pkey.size());
                to_hash.insert(to_hash.end(), pubkey.begin(), pubkey.end());
                to_hash.insert(to_hash.end(), pkey.begin(), pkey.end());

                if (Hash(to_hash) != hash)
                {
                    strErr = "Error reading wallet database: CPubKey/CPrivKey corrupt";
                    return false;
                }

                if (!key.Load(pkey, pubkey, true))
                {
                    strErr = "Error reading wallet database: CPrivKey corrupt";
                    return false;
                }
                wss.m_descriptor_keys.insert(std::make_pair(std::make_pair(desc_id, pubkey.GetID()), key));
            } else if (strType == DBKeys::WALLETDESCRIPTORCKEY) {
                uint256 desc_id;
                CPubKey pubkey;
                ssKey >> desc_id;
                ssKey >> pubkey;
                if (!pubkey.IsValid())
                {
                    strErr = "Error reading wallet database: CPubKey corrupt";
                    return false;
                }
                std::vector<unsigned char> privkey;
                ssValue >> privkey;
                wss.nCKeys++;

                wss.m_descriptor_crypt_keys.insert(std::make_pair(std::make_pair(desc_id, pubkey.GetID()), std::make_pair(pubkey, privkey)));
                wss.fIsEncrypted = true;
            } else if (strType == DBKeys::LOCKED_UTXO) {
                uint256 hash;
                uint32_t n;
                ssKey >> hash;
                ssKey >> n;
                pwallet->LockCoin(OutPoint(hash, n));
            } else if (strType != DBKeys::BESTBLOCK && strType != DBKeys::BESTBLOCK_NOMERKLE &&
                       strType != DBKeys::MINVERSION && strType != DBKeys::ACENTRY &&
                       strType != DBKeys::VERSION && strType != DBKeys::SETTINGS &&
                       strType != DBKeys::FLAGS) {
                wss.m_unknown_records++;
            }
        } catch (const std::exception& e) {
            if (strErr.empty()) {
                strErr = e.what();
            }
            return false;
        } catch (...) {
            if (strErr.empty()) {
                strErr = "Caught unknown exception in ReadKeyValue";
            }
            return false;
        }
        return true;
        */
}
