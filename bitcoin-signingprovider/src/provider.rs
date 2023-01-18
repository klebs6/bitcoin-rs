crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/script/signingprovider.h]

/**
  | An interface to be implemented by keystores
  | that support signing.
  |
  */
#[derive(Default)]
pub struct SigningProvider { }

impl GetScript for SigningProvider {

    fn get_script(&self, 
        scriptid: &ScriptID,
        script:   &mut Script) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl HaveScript for SigningProvider {

    fn have_script(&self, scriptid: &ScriptID) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl GetPubKeyWithKeyId for SigningProvider {

    fn get_pub_key_with_key_id(&self, 
        address: &KeyID,
        pubkey:  &mut PubKey) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl GetKey for SigningProvider {

    fn get_key(&self, 
        address: &KeyID,
        key:     &mut Key) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl HaveKey for SigningProvider {

    fn have_key(&self, address: &KeyID) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl GetKeyOrigin for SigningProvider {

    fn get_key_origin(&self, 
        keyid: &KeyID,
        info:  &mut KeyOriginInfo) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl GetTaprootSpendData for SigningProvider {

    fn get_taproot_spend_data(&self, 
        output_key: &XOnlyPubKey,
        spenddata:  &mut TaprootSpendData) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl SigningProvider {

    pub fn get_key_by_xonly(&self, 
        pubkey: &XOnlyPubKey,
        key:    &mut Key) -> bool {
        
        todo!();
        /*
            for (const auto& id : pubkey.GetKeyIDs()) {
                if (GetKey(id, key)) return true;
            }
            return false;
        */
    }
    
    pub fn get_pub_key_by_xonly(&self, 
        pubkey: &XOnlyPubKey,
        out:    &mut PubKey) -> bool {
        
        todo!();
        /*
            for (const auto& id : pubkey.GetKeyIDs()) {
                if (GetPubKey(id, out)) return true;
            }
            return false;
        */
    }
    
    pub fn get_key_origin_by_xonly(&self, 
        pubkey: &XOnlyPubKey,
        info:   &mut KeyOriginInfo) -> bool {
        
        todo!();
        /*
            for (const auto& id : pubkey.GetKeyIDs()) {
                if (GetKeyOrigin(id, info)) return true;
            }
            return false;
        */
    }
}

lazy_static!{
    /*
    extern const SigningProvider& DUMMY_SIGNING_PROVIDER;
    */
}

//-------------------------------------------[.cpp/bitcoin/src/script/signingprovider.cpp]

pub const DUMMY_SIGNING_PROVIDER: SigningProvider = SigningProvider{};

pub fn lookup_helper<M, K, V>(
        map:   &M,
        key:   &K,
        value: &mut V) -> bool {

    todo!();
        /*
            auto it = map.find(key);
        if (it != map.end()) {
            value = it->second;
            return true;
        }
        return false;
        */
}

/**
  | Return the CKeyID of the key involved
  | in a script (if there is a unique one).
  |
  */
pub fn get_key_for_destination(
        store: &SigningProvider,
        dest:  &TxDestination) -> KeyID {
    
    todo!();
        /*
            // Only supports destinations which map to single public keys, i.e. P2PKH,
        // P2WPKH, and P2SH-P2WPKH.
        if (auto id = std::get_if<PKHash>(&dest)) {
            return ToKeyID(*id);
        }
        if (auto witness_id = std::get_if<WitnessV0KeyHash>(&dest)) {
            return ToKeyID(*witness_id);
        }
        if (auto script_hash = std::get_if<ScriptHash>(&dest)) {
            CScript script;
            CScriptID script_id(*script_hash);
            TxDestination inner_dest;
            if (store.GetCScript(script_id, script) && ExtractDestination(script, inner_dest)) {
                if (auto inner_witness_id = std::get_if<WitnessV0KeyHash>(&inner_dest)) {
                    return ToKeyID(*inner_witness_id);
                }
            }
        }
        return CKeyID();
        */
}
