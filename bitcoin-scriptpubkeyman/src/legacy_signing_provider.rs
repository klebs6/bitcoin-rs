// ---------------- [ File: bitcoin-scriptpubkeyman/src/legacy_signing_provider.rs ]
crate::ix!();

/**
  | Wraps a LegacyScriptPubKeyMan so that
  | it can be returned in a new unique_ptr.
  | Does not provide privkeys
  |
  */
pub struct LegacySigningProvider {
    base:    SigningProvider,
    spk_man: Rc<LegacyScriptPubKeyMan>,
}

impl LegacySigningProvider {
    
    pub fn new(spk_man: &LegacyScriptPubKeyMan) -> Self {
    
        todo!();
        /*
        : spk_man(spk_man),

        
        */
    }
    
    pub fn get_cscript(&self, 
        scriptid: &ScriptID,
        script:   &mut Script) -> bool {
        
        todo!();
        /*
            return m_spk_man.GetCScript(scriptid, script);
        */
    }
    
    pub fn have_cscript(&self, scriptid: &ScriptID) -> bool {
        
        todo!();
        /*
            return m_spk_man.HaveCScript(scriptid);
        */
    }
    
    pub fn get_pub_key(&self, 
        address: &KeyID,
        pubkey:  &mut PubKey) -> bool {
        
        todo!();
        /*
            return m_spk_man.GetPubKey(address, pubkey);
        */
    }
    
    pub fn get_key(&self, 
        address: &KeyID,
        key:     &mut Key) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn have_key(&self, address: &KeyID) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_key_origin(&self, 
        keyid: &KeyID,
        info:  &mut KeyOriginInfo) -> bool {
        
        todo!();
        /*
            return m_spk_man.GetKeyOrigin(keyid, info);
        */
    }
}
