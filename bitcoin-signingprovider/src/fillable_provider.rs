// ---------------- [ File: bitcoin-signingprovider/src/fillable_provider.rs ]
crate::ix!();

pub type FillableSigningProviderKeyMap    = HashMap<KeyID,Key>;
pub type FillableSigningProviderScriptMap = HashMap<ScriptID,Script>;

/**
  | Fillable signing provider that keeps
  | keys in an address->secret map
  |
  */
pub struct FillableSigningProvider<T> {
    base:         SigningProvider,
    cs_key_store: Arc<Mutex<FillableSigningProviderInner<T>>>,
}

pub struct FillableSigningProviderInner<T> {

    /**
      | Map of key id to unencrypted private
      | keys known by the signing provider.
      | 
      | Map may be empty if the provider has another
      | source of keys, like an encrypted store.
      |
      */
    map_keys: FillableSigningProviderKeyMap,

    /**
      | Map of script id to scripts known by the
      | signing provider.
      | 
      | This map originally just held P2SH
      | redeemScripts, and was used by wallet
      | code to look up script ids referenced in
      | "OP_HASH160 <script id> OP_EQUAL" P2SH
      | outputs. 
      |
      | Later in 605e8473a7d it was extended to
      | hold P2WSH witnessScripts as well, and
      | used to look up nested scripts
      | referenced in "OP_0 <script hash>" P2WSH
      | outputs.
      |
      | Later in commits f4691ab3a9d and
      | 248f3a76a82, it was extended once again
      | to hold segwit "OP_0 <key or script
      | hash>" scriptPubKeys, in order to give
      | the wallet a way to distinguish between
      | segwit outputs that it generated
      | addresses for and wanted to receive
      | payments from, and segwit outputs that
      | it never generated addresses for, but it
      | could spend just because of having keys. 
      |
      | (Before segwit activation it was also
      | important to not treat segwit outputs to
      | arbitrary wallet keys as payments,
      | because these could be spent by anyone
      | without even needing to sign with the
      | keys.)
      | 
      | Some of the scripts stored in mapScripts
      | are memory-only and intentionally not
      | saved to disk. 
      |
      | Specifically, scripts added by
      | ImplicitlyLearnRelatedKeyScripts(pubkey)
      | calls are not written to disk so future
      | wallet code can have flexibility to be
      | more selective about what transaction
      | outputs it recognizes as payments,
      | instead of having to treat all outputs
      | spending to keys it knows as payments.
      |
      | By contrast, mapScripts entries added by
      | AddCScript(script),
      | 
      | LearnRelatedScripts(pubkey, type), and
      | LearnAllRelatedScripts(pubkey) calls are
      | saved because they are all intentionally
      | used to receive payments.
      | 
      | The FillableSigningProvider::mapScripts
      | script map should not be confused with
      | LegacyScriptPubKeyMan::setWatchOnly
      | script set. 
      |
      | The two collections can hold the same
      | scripts, but they serve different
      | purposes. 
      |
      | The setWatchOnly script set is intended
      | to expand the set of outputs the wallet
      | considers payments. 
      |
      | Every output with a script it contains
      | is considered to belong to the wallet,
      | regardless of whether the script is
      | solvable or signable.
      |
      | By contrast, the scripts in mapScripts
      | are only used for solving, and to
      | restrict which outputs are considered
      | payments by the wallet. 
      |
      | An output with a script in mapScripts,
      | unlike setWatchOnly, is not
      | automatically considered to belong
      | to the wallet if it can't be solved
      | and signed for.
      |
      */
    map_scripts:  FillableSigningProviderScriptMap,

    item: T,
}

impl<T> AddKey for FillableSigningProvider<T> {

    fn add_key(&mut self, key: &Key) -> bool {
        
        todo!();
        /*
            return AddKeyPubKey(key, key.GetPubKey());
        */
    }
}

impl<T> FillableSigningProvider<T> {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_KeyStore)]
    pub fn implicitly_learn_related_key_scripts(&mut self, pubkey: &PubKey)  {
        
        todo!();
        /*
            AssertLockHeld(cs_KeyStore);
        CKeyID key_id = pubkey.GetID();
        // This adds the redeemscripts necessary to detect P2WPKH and P2SH-P2WPKH
        // outputs. Technically P2WPKH outputs don't have a redeemscript to be
        // spent. However, our current IsMine logic requires the corresponding
        // P2SH-P2WPKH redeemscript to be present in the wallet in order to accept
        // payment even to P2WPKH outputs.
        // Also note that having superfluous scripts in the keystore never hurts.
        // They're only used to guide recursion in signing and IsMine logic - if
        // a script is present but we can't do anything with it, it has no effect.
        // "Implicitly" refers to fact that scripts are derived automatically from
        // existing keys, and are present in memory, even without being explicitly
        // loaded (e.g. from a file).
        if (pubkey.IsCompressed()) {
            CScript script = GetScriptForDestination(WitnessV0KeyHash(key_id));
            // This does not use AddCScript, as it may be overridden.
            CScriptID id(script);
            mapScripts[id] = std::move(script);
        }
        */
    }
    
    pub fn get_pub_key(&self, 
        address:         &KeyID,
        vch_pub_key_out: &mut PubKey) -> bool {
        
        todo!();
        /*
            CKey key;
        if (!GetKey(address, key)) {
            return false;
        }
        vchPubKeyOut = key.GetPubKey();
        return true;
        */
    }
    
    pub fn add_key_pub_key(&mut self, 
        key:    &Key,
        pubkey: &PubKey) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        mapKeys[pubkey.GetID()] = key;
        ImplicitlyLearnRelatedKeyScripts(pubkey);
        return true;
        */
    }
    
    pub fn have_key(&self, address: &KeyID) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        return mapKeys.count(address) > 0;
        */
    }
    
    pub fn get_keys(&self) -> HashSet<KeyID> {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        std::set<CKeyID> set_address;
        for (const auto& mi : mapKeys) {
            set_address.insert(mi.first);
        }
        return set_address;
        */
    }
    
    pub fn get_key(&self, 
        address: &KeyID,
        key_out: &mut Key) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        KeyMap::const_iterator mi = mapKeys.find(address);
        if (mi != mapKeys.end()) {
            keyOut = mi->second;
            return true;
        }
        return false;
        */
    }
    
    pub fn add_script(&mut self, redeem_script: &Script) -> bool {
        
        todo!();
        /*
            if (redeemScript.size() > MAX_SCRIPT_ELEMENT_SIZE)
            return error("FillableSigningProvider::AddCScript(): redeemScripts > %i bytes are invalid", MAX_SCRIPT_ELEMENT_SIZE);

        LOCK(cs_KeyStore);
        mapScripts[CScriptID(redeemScript)] = redeemScript;
        return true;
        */
    }
    
    pub fn have_script(&self, hash: &ScriptID) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        return mapScripts.count(hash) > 0;
        */
    }
    
    pub fn get_scripts(&self) -> HashSet<ScriptID> {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        std::set<CScriptID> set_script;
        for (const auto& mi : mapScripts) {
            set_script.insert(mi.first);
        }
        return set_script;
        */
    }
    
    pub fn get_script(&self, 
        hash:              &ScriptID,
        redeem_script_out: &mut Script) -> bool {
        
        todo!();
        /*
            LOCK(cs_KeyStore);
        ScriptMap::const_iterator mi = mapScripts.find(hash);
        if (mi != mapScripts.end())
        {
            redeemScriptOut = (*mi).second;
            return true;
        }
        return false;
        */
    }
}
