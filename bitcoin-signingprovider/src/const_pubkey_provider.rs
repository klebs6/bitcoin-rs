crate::ix!();

/**
  | An object representing a parsed constant
  | public key in a descriptor.
  |
  */
pub struct ConstPubkeyProvider {
    base:   PubkeyProvider,
    pubkey: crate::PubKey,
    xonly:  bool,
}

impl ConstPubkeyProvider {

    pub fn new(
        exp_index: u32,
        pubkey:    &crate::PubKey,
        xonly:     bool) -> Self {
    
        todo!();
        /*
        : pubkey_provider(exp_index),
        : pubkey(pubkey),
        : xonly(xonly),

        
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
            key = m_pubkey;
            info.path.clear();
            CKeyID keyid = m_pubkey.GetID();
            std::copy(keyid.begin(), keyid.begin() + sizeof(info.fingerprint), info.fingerprint);
            return true;
        */
    }
    
    pub fn get_size(&self) -> usize {
        
        todo!();
        /*
            return m_pubkey.size();
        */
    }
    
    pub fn to_private_string(&self, 
        arg: &SigningProvider,
        ret: &mut String) -> bool {
        
        todo!();
        /*
            CKey key;
            if (!arg.GetKey(m_pubkey.GetID(), key)) return false;
            ret = EncodeSecret(key);
            return true;
        */
    }
    
    pub fn get_priv_key(&self, 
        pos: i32,
        arg: &SigningProvider,
        key: &mut Key) -> bool {
        
        todo!();
        /*
            return arg.GetKey(m_pubkey.GetID(), key);
        */
    }
}

impl IsRange for ConstPubkeyProvider {

    fn is_range(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
    
impl ToString for ConstPubkeyProvider {

    fn to_string(&self) -> String {
        
        todo!();
        /*
            return m_xonly ? HexStr(m_pubkey).substr(2) : HexStr(m_pubkey);
        */
    }
}

impl ToNormalizedString for ConstPubkeyProvider {

    fn to_normalized_string(&self, 
        arg:   &SigningProvider,
        ret:   &mut String,
        cache: *const DescriptorCache) -> bool {
        
        todo!();
        /*
            ret = ToString();
            return true;
        */
    }
}

