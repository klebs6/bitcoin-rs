// ---------------- [ File: bitcoin-signingprovider/src/flat_provider.rs ]
crate::ix!();

pub struct FlatSigningProvider {
    base:         SigningProvider,
    scripts:      HashMap<ScriptID,Script>,
    pubkeys:      HashMap<KeyID,PubKey>,
    origins:      HashMap<KeyID,(PubKey,KeyOriginInfo)>,
    keys:         HashMap<KeyID,Key>,

    /**
      | Map from output key to spend data.
      |
      */
    tr_spenddata: HashMap<XOnlyPubKey,TaprootSpendData>,
}

impl FlatSigningProvider {
    
    pub fn get_script(&self, 
        scriptid: &ScriptID,
        script:   &mut Script) -> bool {
        
        todo!();
        /*
            return LookupHelper(scripts, scriptid, script);
        */
    }
    
    pub fn get_pub_key(&self, 
        keyid:  &KeyID,
        pubkey: &mut PubKey) -> bool {
        
        todo!();
        /*
            return LookupHelper(pubkeys, keyid, pubkey);
        */
    }
    
    pub fn get_key_origin(&self, 
        keyid: &KeyID,
        info:  &mut KeyOriginInfo) -> bool {
        
        todo!();
        /*
            std::pair<CPubKey, KeyOriginInfo> out;
        bool ret = LookupHelper(origins, keyid, out);
        if (ret) info = std::move(out.second);
        return ret;
        */
    }
    
    pub fn get_key(&self, 
        keyid: &KeyID,
        key:   &mut Key) -> bool {
        
        todo!();
        /*
            return LookupHelper(keys, keyid, key);
        */
    }
    
    pub fn get_taproot_spend_data(&self, 
        output_key: &XOnlyPubKey,
        spenddata:  &mut TaprootSpendData) -> bool {
        
        todo!();
        /*
            return LookupHelper(tr_spenddata, output_key, spenddata);
        */
    }
}

pub fn merge(
        a: &FlatSigningProvider,
        b: &FlatSigningProvider) -> FlatSigningProvider {
    
    todo!();
        /*
            FlatSigningProvider ret;
        ret.scripts = a.scripts;
        ret.scripts.insert(b.scripts.begin(), b.scripts.end());
        ret.pubkeys = a.pubkeys;
        ret.pubkeys.insert(b.pubkeys.begin(), b.pubkeys.end());
        ret.keys = a.keys;
        ret.keys.insert(b.keys.begin(), b.keys.end());
        ret.origins = a.origins;
        ret.origins.insert(b.origins.begin(), b.origins.end());
        ret.tr_spenddata = a.tr_spenddata;
        for (const auto& [output_key, spenddata] : b.tr_spenddata) {
            ret.tr_spenddata[output_key].Merge(spenddata);
        }
        return ret;
        */
}
