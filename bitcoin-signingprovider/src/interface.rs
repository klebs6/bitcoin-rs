// ---------------- [ File: bitcoin-signingprovider/src/interface.rs ]
crate::ix!();

pub trait HaveKey {
    fn have_key(&self, address: &KeyID) -> bool;
}

pub trait GetKeyOrigin {
    fn get_key_origin(&self, 
        keyid: &KeyID,
        info:  &mut KeyOriginInfo) -> bool;
}

pub trait GetTaprootSpendData {
    fn get_taproot_spend_data(&self, 
        output_key: &XOnlyPubKey,
        spenddata:  &mut TaprootSpendData) -> bool;
}

pub trait AddKeyPubKey {
    fn add_key_pub_key(&mut self, 
        key:    &Key,
        pubkey: &PubKey) -> bool;
}

pub trait AddKey {
    fn add_key(&mut self, key: &Key) -> bool;
}

pub trait GetPubKeyWithKeyId {
    fn get_pub_key_with_key_id(&self, 
        address:         &KeyID,
        vch_pub_key_out: &mut PubKey) -> bool;
}

pub trait GetKeys {
    fn get_keys(&self) -> HashSet<KeyID>;
}

pub trait GetKey {
    fn get_key(&self, 
        address: &KeyID,
        key_out: &mut Key) -> bool;
}

pub trait AddScript {
    fn add_script(&mut self, redeem_script: &Script) -> bool;
}

pub trait HaveScript {
    fn have_script(&self, hash: &ScriptID) -> bool;
}

pub trait GetScripts {
    fn get_scripts(&self) -> HashSet<ScriptID>;
}

pub trait GetScript {
    fn get_script(&self, 
        hash:              &ScriptID,
        redeem_script_out: &mut Script) -> bool;
}
