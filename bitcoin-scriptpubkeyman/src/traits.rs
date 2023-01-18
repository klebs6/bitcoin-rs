crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/scriptpubkeyman.cpp]

pub trait GetActiveScriptPubKeyMans {

    fn get_active_script_pub_key_mans(&self) -> HashSet<*mut ScriptPubKeyMan>;
}

pub trait GetAllScriptPubKeyMans {

    fn get_all_script_pub_key_mans(&self) -> HashSet<*mut ScriptPubKeyMan>;
}

pub trait GetScriptPubKeyMan {

    fn get_script_pub_key_man(&self, 
        ty:       &OutputType,
        internal: bool) -> *mut ScriptPubKeyMan;
}

pub trait GetScriptPubKeyManWithScript {

    fn get_script_pub_key_man_with_script(&self, script: &Script) -> *mut ScriptPubKeyMan;
}


pub trait GetScriptPubKeyManWithId {

    fn get_script_pub_key_man_with_id(&self, id: &u256) -> *mut ScriptPubKeyMan;
}

pub trait GetScriptPubKeyMans {

    fn get_script_pub_key_mans(&self, 
        script:  &Script,
        sigdata: &mut SignatureData) -> HashSet<*mut ScriptPubKeyMan>;
}

pub trait GetLegacyScriptPubKeyMan {

    fn get_legacy_script_pub_key_man(&self) -> *mut LegacyScriptPubKeyMan;
}

pub trait GetOrCreateLegacyScriptPubKeyMan {
    fn get_or_create_legacy_script_pub_key_man(&mut self) -> *mut LegacyScriptPubKeyMan;
}

pub trait SetupLegacyScriptPubKeyMan {

    fn setup_legacy_script_pub_key_man(&mut self);
}

pub trait ConnectScriptPubKeyManNotifiers {

    fn connect_script_pub_key_man_notifiers(&mut self);
}

pub trait LoadDescriptorScriptPubKeyMan {

    fn load_descriptor_script_pub_key_man(&mut self, 
        id:   u256,
        desc: &mut WalletDescriptor);
}

pub trait AddActiveScriptPubKeyMan {

    fn add_active_script_pub_key_man(&mut self, 
        id:       u256,
        ty:       OutputType,
        internal: bool);
}

pub trait LoadActiveScriptPubKeyMan {

    fn load_active_script_pub_key_man(&mut self, 
        id:       u256,
        ty:       OutputType,
        internal: bool);
}

pub trait DeactivateScriptPubKeyMan {

    fn deactivate_script_pub_key_man(&mut self, 
        id:       u256,
        ty:       OutputType,
        internal: bool);
}

pub trait SetupDescriptorScriptPubKeyMans {

    fn setup_descriptor_script_pub_key_mans(&mut self);
}

pub trait GetDescriptorScriptPubKeyMan {

    fn get_descriptor_script_pub_key_man(&self, desc: &WalletDescriptor) -> *mut DescriptorScriptPubKeyMan;
}

pub trait AddWalletDescriptor {

    fn add_wallet_descriptor(&mut self, 
        desc:             &mut WalletDescriptor,
        signing_provider: &FlatSigningProvider,
        label:            &String,
        internal:         bool) -> *mut ScriptPubKeyMan;
}
