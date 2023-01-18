crate::ix!();

/**
  | Cache for single descriptor's derived
  | extended pubkeys
  |
  */
#[derive(Default)]
pub struct DescriptorCache {

    /**
      | Map key expression index -> map of (key
      | derivation index -> xpub)
      |
      */
    derived_xpubs:       HashMap<u32,ExtPubKeyMap>,


    /**
      | Map key expression index -> parent xpub
      |
      */
    parent_xpubs:        ExtPubKeyMap,


    /**
      | Map key expression index -> last hardened
      | xpub
      |
      */
    last_hardened_xpubs: ExtPubKeyMap,
}

impl DescriptorCache {

    /**
      | Cache a parent xpub
      | 
      | -----------
      | @param[in] key_exp_pos
      | 
      | Position of the key expression within
      | the descriptor
      | ----------
      | @param[in] xpub
      | 
      | The CExtPubKey to cache
      |
      */
    pub fn cache_parent_ext_pub_key(&mut self, 
        key_exp_pos: u32,
        xpub:        &ExtPubKey)  {
        
        todo!();
        /*
            m_parent_xpubs[key_exp_pos] = xpub;
        */
    }
    
    /**
      | Cache an xpub derived at an index
      | 
      | -----------
      | @param[in] key_exp_pos
      | 
      | Position of the key expression within
      | the descriptor
      | ----------
      | @param[in] der_index
      | 
      | Derivation index of the xpub
      | ----------
      | @param[in] xpub
      | 
      | The CExtPubKey to cache
      |
      */
    pub fn cache_derived_ext_pub_key(&mut self, 
        key_exp_pos: u32,
        der_index:   u32,
        xpub:        &ExtPubKey)  {
        
        todo!();
        /*
            auto& xpubs = m_derived_xpubs[key_exp_pos];
        xpubs[der_index] = xpub;
        */
    }
    
    /**
      | Cache a last hardened xpub
      | 
      | -----------
      | @param[in] key_exp_pos
      | 
      | Position of the key expression within
      | the descriptor
      | ----------
      | @param[in] xpub
      | 
      | The CExtPubKey to cache
      |
      */
    pub fn cache_last_hardened_ext_pub_key(&mut self, 
        key_exp_pos: u32,
        xpub:        &ExtPubKey)  {
        
        todo!();
        /*
            m_last_hardened_xpubs[key_exp_pos] = xpub;
        */
    }
    
    /**
      | Retrieve a cached parent xpub
      | 
      | -----------
      | @param[in] key_exp_pos
      | 
      | Position of the key expression within
      | the descriptor
      | ----------
      | @param[in] xpub
      | 
      | The CExtPubKey to get from cache
      |
      */
    pub fn get_cached_parent_ext_pub_key(&self, 
        key_exp_pos: u32,
        xpub:        &mut ExtPubKey) -> bool {
        
        todo!();
        /*
            const auto& it = m_parent_xpubs.find(key_exp_pos);
        if (it == m_parent_xpubs.end()) return false;
        xpub = it->second;
        return true;
        */
    }
    
    /**
      | Retrieve a cached xpub derived at an
      | index
      | 
      | -----------
      | @param[in] key_exp_pos
      | 
      | Position of the key expression within
      | the descriptor
      | ----------
      | @param[in] der_index
      | 
      | Derivation index of the xpub
      | ----------
      | @param[in] xpub
      | 
      | The CExtPubKey to get from cache
      |
      */
    pub fn get_cached_derived_ext_pub_key(&self, 
        key_exp_pos: u32,
        der_index:   u32,
        xpub:        &mut ExtPubKey) -> bool {
        
        todo!();
        /*
            const auto& key_exp_it = m_derived_xpubs.find(key_exp_pos);
        if (key_exp_it == m_derived_xpubs.end()) return false;
        const auto& der_it = key_exp_it->second.find(der_index);
        if (der_it == key_exp_it->second.end()) return false;
        xpub = der_it->second;
        return true;
        */
    }
    
    /**
      | Retrieve a cached last hardened xpub
      | 
      | -----------
      | @param[in] key_exp_pos
      | 
      | Position of the key expression within
      | the descriptor
      | ----------
      | @param[in] xpub
      | 
      | The CExtPubKey to get from cache
      |
      */
    pub fn get_cached_last_hardened_ext_pub_key(&self, 
        key_exp_pos: u32,
        xpub:        &mut ExtPubKey) -> bool {
        
        todo!();
        /*
            const auto& it = m_last_hardened_xpubs.find(key_exp_pos);
        if (it == m_last_hardened_xpubs.end()) return false;
        xpub = it->second;
        return true;
        */
    }
    
    /**
      | Combine another DescriptorCache into
      | this one.
      | 
      | Returns a cache containing the items
      | from the other cache unknown to current
      | cache
      |
      */
    pub fn merge_and_diff(&mut self, other: &DescriptorCache) -> DescriptorCache {
        
        todo!();
        /*
            DescriptorCache diff;
        for (const auto& parent_xpub_pair : other.GetCachedParentExtPubKeys()) {
            CExtPubKey xpub;
            if (GetCachedParentExtPubKey(parent_xpub_pair.first, xpub)) {
                if (xpub != parent_xpub_pair.second) {
                    throw std::runtime_error(std::string(__func__) + ": New cached parent xpub does not match already cached parent xpub");
                }
                continue;
            }
            CacheParentExtPubKey(parent_xpub_pair.first, parent_xpub_pair.second);
            diff.CacheParentExtPubKey(parent_xpub_pair.first, parent_xpub_pair.second);
        }
        for (const auto& derived_xpub_map_pair : other.GetCachedDerivedExtPubKeys()) {
            for (const auto& derived_xpub_pair : derived_xpub_map_pair.second) {
                CExtPubKey xpub;
                if (GetCachedDerivedExtPubKey(derived_xpub_map_pair.first, derived_xpub_pair.first, xpub)) {
                    if (xpub != derived_xpub_pair.second) {
                        throw std::runtime_error(std::string(__func__) + ": New cached derived xpub does not match already cached derived xpub");
                    }
                    continue;
                }
                CacheDerivedExtPubKey(derived_xpub_map_pair.first, derived_xpub_pair.first, derived_xpub_pair.second);
                diff.CacheDerivedExtPubKey(derived_xpub_map_pair.first, derived_xpub_pair.first, derived_xpub_pair.second);
            }
        }
        for (const auto& lh_xpub_pair : other.GetCachedLastHardenedExtPubKeys()) {
            CExtPubKey xpub;
            if (GetCachedLastHardenedExtPubKey(lh_xpub_pair.first, xpub)) {
                if (xpub != lh_xpub_pair.second) {
                    throw std::runtime_error(std::string(__func__) + ": New cached last hardened xpub does not match already cached last hardened xpub");
                }
                continue;
            }
            CacheLastHardenedExtPubKey(lh_xpub_pair.first, lh_xpub_pair.second);
            diff.CacheLastHardenedExtPubKey(lh_xpub_pair.first, lh_xpub_pair.second);
        }
        return diff;
        */
    }
    
    /**
      | Retrieve all cached parent xpubs
      |
      */
    pub fn get_cached_parent_ext_pub_keys(&self) -> ExtPubKeyMap {
        
        todo!();
        /*
            return m_parent_xpubs;
        */
    }
    
    /**
      | Retrieve all cached derived xpubs
      |
      */
    pub fn get_cached_derived_ext_pub_keys(&self) -> HashMap<u32,ExtPubKeyMap> {
        
        todo!();
        /*
            return m_derived_xpubs;
        */
    }
    
    /**
      | Retrieve all cached last hardened xpubs
      |
      */
    pub fn get_cached_last_hardened_ext_pub_keys(&self) -> ExtPubKeyMap {
        
        todo!();
        /*
            return m_last_hardened_xpubs;
        */
    }
}
