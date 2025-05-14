// ---------------- [ File: bitcoin-signingprovider/src/origin_pubkey_provider.rs ]
crate::ix!();

///----------------------
pub struct OriginPubkeyProvider {
    base:     PubkeyProvider,
    origin:   KeyOriginInfo,
    provider: Box<PubkeyProvider>,
}

impl OriginPubkeyProvider {
    
    pub fn origin_string(&self) -> String {
        
        todo!();
        /*
            return HexStr(m_origin.fingerprint) + FormatHDKeypath(m_origin.path);
        */
    }
    
    pub fn new(
        exp_index: u32,
        info:      KeyOriginInfo,
        provider:  Box<PubkeyProvider>) -> Self {
    
        todo!();
        /*
        : pubkey_provider(exp_index),
        : origin(std::move(info)),
        : provider(std::move(provider)),

        
        */
    }
    
    pub fn get_pub_key(&self, 
        pos:         i32,
        arg:         &SigningProvider,
        key:         &mut crate::PubKey,
        info:        &mut KeyOriginInfo,
        read_cache:  Option<*const DescriptorCache>,
        write_cache: Option<*mut DescriptorCache>) -> bool {

        todo!();
        /*
            if (!m_provider->GetPubKey(pos, arg, key, info, read_cache, write_cache)) return false;
            std::copy(std::begin(m_origin.fingerprint), std::end(m_origin.fingerprint), info.fingerprint);
            info.path.insert(info.path.begin(), m_origin.path.begin(), m_origin.path.end());
            return true;
        */
    }
    
    pub fn get_size(&self) -> usize {
        
        todo!();
        /*
            return m_provider->GetSize();
        */
    }
    
    pub fn to_private_string(&self, 
        arg: &SigningProvider,
        ret: &mut String) -> bool {
        
        todo!();
        /*
            std::string sub;
            if (!m_provider->ToPrivateString(arg, sub)) return false;
            ret = "[" + OriginString() + "]" + std::move(sub);
            return true;
        */
    }
    
    pub fn get_priv_key(&self, 
        pos: i32,
        arg: &SigningProvider,
        key: &mut Key) -> bool {
        
        todo!();
        /*
            return m_provider->GetPrivKey(pos, arg, key);
        */
    }
}

impl IsRange for OriginPubkeyProvider {

    fn is_range(&self) -> bool {
        
        todo!();
        /*
            return m_provider->IsRange();
        */
    }
}

impl ToString for OriginPubkeyProvider {

    fn to_string(&self) -> String {
        
        todo!();
        /*
            return "[" + OriginString() + "]" + m_provider->ToString();
        */
    }
}

impl ToNormalizedString for OriginPubkeyProvider {

    fn to_normalized_string(&self, 
        arg:   &SigningProvider,
        ret:   &mut String,
        cache: *const DescriptorCache) -> bool {
        
        todo!();
        /*
            std::string sub;
            if (!m_provider->ToNormalizedString(arg, sub, cache)) return false;
            // If m_provider is a BIP32PubkeyProvider, we may get a string formatted like a OriginPubkeyProvider
            // In that case, we need to strip out the leading square bracket and fingerprint from the substring,
            // and append that to our own origin string.
            if (sub[0] == '[') {
                sub = sub.substr(9);
                ret = "[" + OriginString() + std::move(sub);
            } else {
                ret = "[" + OriginString() + "]" + std::move(sub);
            }
            return true;
        */
    }
}
 
