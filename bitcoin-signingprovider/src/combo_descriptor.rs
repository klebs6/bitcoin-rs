// ---------------- [ File: bitcoin-signingprovider/src/combo_descriptor.rs ]
crate::ix!();

/**
  | A parsed combo(P) descriptor.
  |
  */
pub struct ComboDescriptor {
    base: DescriptorImpl,
}

impl From<Box<PubkeyProvider>> for ComboDescriptor {

    fn from(prov: Box<PubkeyProvider>) -> Self {
    
        todo!();
        /*
            : DescriptorImpl(Vector(std::move(prov)), "combo")
        */
    }
}
    
impl ComboDescriptor {
    
    pub fn make_scripts(&self, 
        keys: &Vec<crate::PubKey>,
        _1:   &[Script],
        out:  &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            std::vector<CScript> ret;
            CKeyID id = keys[0].GetID();
            out.pubkeys.emplace(id, keys[0]);
            ret.emplace_back(GetScriptForRawPubKey(keys[0])); // P2PK
            ret.emplace_back(GetScriptForDestination(PKHash(id))); // P2PKH
            if (keys[0].IsCompressed()) {
                CScript p2wpkh = GetScriptForDestination(WitnessV0KeyHash(id));
                out.scripts.emplace(CScriptID(p2wpkh), p2wpkh);
                ret.emplace_back(p2wpkh);
                ret.emplace_back(GetScriptForDestination(ScriptHash(p2wpkh))); // P2SH-P2WPKH
            }
            return ret;
        */
    }
}

impl IsSingleType for ComboDescriptor {
    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
