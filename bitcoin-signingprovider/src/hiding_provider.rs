// ---------------- [ File: bitcoin-signingprovider/src/hiding_provider.rs ]
crate::ix!();

pub struct HidingSigningProvider {
    base:        SigningProvider,
    hide_secret: bool,
    hide_origin: bool,
    provider:    *const SigningProvider,
}

impl HidingSigningProvider {

    pub fn new(
        provider:    *const SigningProvider,
        hide_secret: bool,
        hide_origin: bool) -> Self {
    
        todo!();
        /*
        : hide_secret(hide_secret),
        : hide_origin(hide_origin),
        : provider(provider),

        
        */
    }

    pub fn get_script(&self, 
        scriptid: &ScriptID,
        script:   &mut Script) -> bool {
        
        todo!();
        /*
            return m_provider->GetCScript(scriptid, script);
        */
    }
    
    pub fn get_pub_key(&self, 
        keyid:  &KeyID,
        pubkey: &mut PubKey) -> bool {
        
        todo!();
        /*
            return m_provider->GetPubKey(keyid, pubkey);
        */
    }
    
    pub fn get_key(&self, 
        keyid: &KeyID,
        key:   &mut Key) -> bool {
        
        todo!();
        /*
            if (m_hide_secret) return false;
        return m_provider->GetKey(keyid, key);
        */
    }
    
    pub fn get_key_origin(&self, 
        keyid: &KeyID,
        info:  &mut KeyOriginInfo) -> bool {
        
        todo!();
        /*
            if (m_hide_origin) return false;
        return m_provider->GetKeyOrigin(keyid, info);
        */
    }
    
    pub fn get_taproot_spend_data(&self, 
        output_key: &XOnlyPubKey,
        spenddata:  &mut TaprootSpendData) -> bool {
        
        todo!();
        /*
            return m_provider->GetTaprootSpendData(output_key, spenddata);
        */
    }
}
