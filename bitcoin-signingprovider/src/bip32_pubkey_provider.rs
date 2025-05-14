// ---------------- [ File: bitcoin-signingprovider/src/bip32_pubkey_provider.rs ]
crate::ix!();

/**
  | An object representing a parsed extended
  | public key in a descriptor.
  |
  */
pub struct BIP32PubkeyProvider {
    base: PubkeyProvider,

    /**
      | Root xpub, path, and final derivation
      | step type being used, if any
      |
      */
    root_extkey: ExtPubKey,

    path:        KeyPath,
    derive:      DeriveType,
}

impl BIP32PubkeyProvider {
    
    pub fn get_ext_key(&self, 
        arg: &SigningProvider,
        ret: &mut ExtKey) -> bool {
        
        todo!();
        /*
            CKey key;
            if (!arg.GetKey(m_root_extkey.pubkey.GetID(), key)) return false;
            ret.nDepth = m_root_extkey.nDepth;
            std::copy(m_root_extkey.vchFingerprint, m_root_extkey.vchFingerprint + sizeof(ret.vchFingerprint), ret.vchFingerprint);
            ret.nChild = m_root_extkey.nChild;
            ret.chaincode = m_root_extkey.chaincode;
            ret.key = key;
            return true;
        */
    }

    /**
      | Derives the last xprv
      |
      */
    pub fn get_derived_ext_key(&self, 
        arg:           &SigningProvider,
        xprv:          &mut ExtKey,
        last_hardened: &mut ExtKey) -> bool {
        
        todo!();
        /*
            if (!GetExtKey(arg, xprv)) return false;
            for (auto entry : m_path) {
                xprv.Derive(xprv, entry);
                if (entry >> 31) {
                    last_hardened = xprv;
                }
            }
            return true;
        */
    }
    
    pub fn is_hardened(&self) -> bool {
        
        todo!();
        /*
            if (m_derive == DeriveType::HARDENED) return true;
            for (auto entry : m_path) {
                if (entry >> 31) return true;
            }
            return false;
        */
    }
    
    pub fn new(
        exp_index: u32,
        extkey:    &ExtPubKey,
        path:      KeyPath,
        derive:    DeriveType) -> Self {
    
        todo!();
        /*
        : pubkey_provider(exp_index),
        : root_extkey(extkey),
        : path(std::move(path)),
        : derive(derive),

        
        */
    }
    
    pub fn get_size(&self) -> usize {
        
        todo!();
        /*
            return 33;
        */
    }
    
    pub fn get_pub_key(&self, 
        pos:            i32,
        arg:            &SigningProvider,
        key_out:        &mut crate::PubKey,
        final_info_out: &mut KeyOriginInfo,
        read_cache:     Option<*const DescriptorCache>,
        write_cache:    Option<*mut DescriptorCache>) -> bool {

        todo!();
        /*
            // Info of parent of the to be derived pubkey
            KeyOriginInfo parent_info;
            CKeyID keyid = m_root_extkey.pubkey.GetID();
            std::copy(keyid.begin(), keyid.begin() + sizeof(parent_info.fingerprint), parent_info.fingerprint);
            parent_info.path = m_path;

            // Info of the derived key itself which is copied out upon successful completion
            KeyOriginInfo final_info_out_tmp = parent_info;
            if (m_derive == DeriveType::UNHARDENED) final_info_out_tmp.path.push_back((uint32_t)pos);
            if (m_derive == DeriveType::HARDENED) final_info_out_tmp.path.push_back(((uint32_t)pos) | 0x80000000L);

            // Derive keys or fetch them from cache
            CExtPubKey final_extkey = m_root_extkey;
            CExtPubKey parent_extkey = m_root_extkey;
            CExtPubKey last_hardened_extkey;
            bool der = true;
            if (read_cache) {
                if (!read_cache->GetCachedDerivedExtPubKey(m_expr_index, pos, final_extkey)) {
                    if (m_derive == DeriveType::HARDENED) return false;
                    // Try to get the derivation parent
                    if (!read_cache->GetCachedParentExtPubKey(m_expr_index, parent_extkey)) return false;
                    final_extkey = parent_extkey;
                    if (m_derive == DeriveType::UNHARDENED) der = parent_extkey.Derive(final_extkey, pos);
                }
            } else if (IsHardened()) {
                CExtKey xprv;
                CExtKey lh_xprv;
                if (!GetDerivedExtKey(arg, xprv, lh_xprv)) return false;
                parent_extkey = xprv.Neuter();
                if (m_derive == DeriveType::UNHARDENED) der = xprv.Derive(xprv, pos);
                if (m_derive == DeriveType::HARDENED) der = xprv.Derive(xprv, pos | 0x80000000UL);
                final_extkey = xprv.Neuter();
                if (lh_xprv.key.IsValid()) {
                    last_hardened_extkey = lh_xprv.Neuter();
                }
            } else {
                for (auto entry : m_path) {
                    der = parent_extkey.Derive(parent_extkey, entry);
                    assert(der);
                }
                final_extkey = parent_extkey;
                if (m_derive == DeriveType::UNHARDENED) der = parent_extkey.Derive(final_extkey, pos);
                assert(m_derive != DeriveType::HARDENED);
            }
            assert(der);

            final_info_out = final_info_out_tmp;
            key_out = final_extkey.pubkey;

            if (write_cache) {
                // Only cache parent if there is any unhardened derivation
                if (m_derive != DeriveType::HARDENED) {
                    write_cache->CacheParentExtPubKey(m_expr_index, parent_extkey);
                    // Cache last hardened xpub if we have it
                    if (last_hardened_extkey.pubkey.IsValid()) {
                        write_cache->CacheLastHardenedExtPubKey(m_expr_index, last_hardened_extkey);
                    }
                } else if (final_info_out.path.size() > 0) {
                    write_cache->CacheDerivedExtPubKey(m_expr_index, pos, final_extkey);
                }
            }

            return true;
        */
    }
    
    pub fn to_private_string(&self, 
        arg: &SigningProvider,
        out: &mut String) -> bool {
        
        todo!();
        /*
            CExtKey key;
            if (!GetExtKey(arg, key)) return false;
            out = EncodeExtKey(key) + FormatHDKeypath(m_path);
            if (IsRange()) {
                out += "/\*";
                if (m_derive == DeriveType::HARDENED) out += '\'';
            }
            return true;
        */
    }
       
    pub fn get_priv_key(&self, 
        pos: i32,
        arg: &SigningProvider,
        key: &mut Key) -> bool {
        
        todo!();
        /*
            CExtKey extkey;
            CExtKey dummy;
            if (!GetDerivedExtKey(arg, extkey, dummy)) return false;
            if (m_derive == DeriveType::UNHARDENED) extkey.Derive(extkey, pos);
            if (m_derive == DeriveType::HARDENED) extkey.Derive(extkey, pos | 0x80000000UL);
            key = extkey.key;
            return true;
        */
    }
}

impl IsRange for BIP32PubkeyProvider {

    fn is_range(&self) -> bool {
        
        todo!();
        /*
            return m_derive != DeriveType::NO;
        */
    }
}

impl ToString for BIP32PubkeyProvider {

    fn to_string(&self) -> String {
        
        todo!();
        /*
            std::string ret = EncodeExtPubKey(m_root_extkey) + FormatHDKeypath(m_path);
            if (IsRange()) {
                ret += "/\*";
                if (m_derive == DeriveType::HARDENED) ret += '\'';
            }
            return ret;
        */
    }
}

impl ToNormalizedString for BIP32PubkeyProvider {

    fn to_normalized_string(&self, 
        arg:   &SigningProvider,
        out:   &mut String,
        cache: *const DescriptorCache) -> bool {
        
        todo!();
        /*
            // For hardened derivation type, just return the typical string, nothing to normalize
            if (m_derive == DeriveType::HARDENED) {
                out = ToString();
                return true;
            }
            // Step backwards to find the last hardened step in the path
            int i = (int)m_path.size() - 1;
            for (; i >= 0; --i) {
                if (m_path.at(i) >> 31) {
                    break;
                }
            }
            // Either no derivation or all unhardened derivation
            if (i == -1) {
                out = ToString();
                return true;
            }
            // Get the path to the last hardened stup
            KeyOriginInfo origin;
            int k = 0;
            for (; k <= i; ++k) {
                // Add to the path
                origin.path.push_back(m_path.at(k));
            }
            // Build the remaining path
            KeyPath end_path;
            for (; k < (int)m_path.size(); ++k) {
                end_path.push_back(m_path.at(k));
            }
            // Get the fingerprint
            CKeyID id = m_root_extkey.pubkey.GetID();
            std::copy(id.begin(), id.begin() + 4, origin.fingerprint);

            CExtPubKey xpub;
            CExtKey lh_xprv;
            // If we have the cache, just get the parent xpub
            if (cache != nullptr) {
                cache->GetCachedLastHardenedExtPubKey(m_expr_index, xpub);
            }
            if (!xpub.pubkey.IsValid()) {
                // Cache miss, or nor cache, or need privkey
                CExtKey xprv;
                if (!GetDerivedExtKey(arg, xprv, lh_xprv)) return false;
                xpub = lh_xprv.Neuter();
            }
            assert(xpub.pubkey.IsValid());

            // Build the string
            std::string origin_str = HexStr(origin.fingerprint) + FormatHDKeypath(origin.path);
            out = "[" + origin_str + "]" + EncodeExtPubKey(xpub) + FormatHDKeypath(end_path);
            if (IsRange()) {
                out += "/\*";
                assert(m_derive == DeriveType::UNHARDENED);
            }
            return true;
        */
    }
}
