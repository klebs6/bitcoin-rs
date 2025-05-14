// ---------------- [ File: bitcoin-scriptpubkeyman/src/scriptpubkeyman.rs ]
crate::ix!();

/**
  | A class implementing ScriptPubKeyMan
  | manages some (or all) scriptPubKeys
  | used in a wallet.
  | 
  | It contains the scripts and keys related
  | to the scriptPubKeys it manages.
  | 
  | A ScriptPubKeyMan will be able to give
  | out scriptPubKeys to be used, as well
  | as marking when a scriptPubKey has been
  | used. It also handles when and how to
  | store a scriptPubKey and its related
  | scripts and keys, including encryption.
  |
  */
pub struct ScriptPubKeyMan {

    storage:                          Rc<RefCell<dyn WalletStorage>>,

    /**
      | Watch-only address added
      |
      */
    notify_watchonly_changed:         Signal<fn(have_watch_only: bool) -> ()>,

    /**
      | Keypool has new keys
      |
      */
    notify_can_get_addresses_changed: Signal<fn() -> ()>,
}

impl GetNewDestination for ScriptPubKeyMan {

    fn get_new_destination(&mut self, 
        ty:    OutputType,
        dest:  &mut TxDestination,
        error: &mut BilingualStr) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl IsMine for ScriptPubKeyMan {

    fn is_mine(&self, script: &Script) -> IsMineType {
        
        todo!();
        /*
            return ISMINE_NO;
        */
    }
}

impl CheckDecryptionKey for ScriptPubKeyMan {

    /**
      | Check that the given decryption key
      | is valid for this ScriptPubKeyMan,
      | i.e. it decrypts all of the keys handled
      | by it.
      |
      */
    fn check_decryption_key(&mut self, 
        master_key:     &KeyingMaterial,
        accept_no_keys: Option<bool>) -> bool {
        let accept_no_keys: bool = accept_no_keys.unwrap_or(false);

        todo!();
        /*
            return false;
        */
    }
}

impl Encrypt for ScriptPubKeyMan {

    fn encrypt(&mut self, 
        master_key: &KeyingMaterial,
        batch:      *mut WalletBatch) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl GetReservedDestination for ScriptPubKeyMan {

    fn get_reserved_destination(&mut self, 
        ty:       OutputType,
        internal: bool,
        address:  &mut TxDestination,
        index:    &mut i64,
        keypool:  &mut KeyPool,
        error:    &mut BilingualStr) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl KeepDestination for ScriptPubKeyMan {

    fn keep_destination(&mut self, 
        index: i64,
        ty:    &OutputType)  {
        
        todo!();
        /*
        
        */
    }
}

impl ReturnDestination for ScriptPubKeyMan {

    fn return_destination(&mut self, 
        index:    i64,
        internal: bool,
        addr:     &TxDestination)  {
        
        todo!();
        /*
        
        */
    }
}

impl TopUp for ScriptPubKeyMan {

    /**
      | Fills internal address pool. Use within
      | ScriptPubKeyMan implementations
      | should be used sparingly and only when
      | something from the address pool is removed,
      | excluding GetNewDestination and GetReservedDestination.
      | 
      | External wallet code is primarily responsible
      | for topping up prior to fetching new
      | addresses
      |
      */
    fn top_up(&mut self, size: Option<u32>) -> bool {
        let size: u32 = size.unwrap_or(0);

        todo!();
        /*
            return false;
        */
    }
}

impl MarkUnusedAddresses for ScriptPubKeyMan {

    /**
      | Mark unused addresses as being used
      |
      */
    fn mark_unused_addresses(&mut self, script: &Script)  { }
}

impl SetupGeneration for ScriptPubKeyMan {

    /**
      | Sets up the key generation stuff, i.e.
      | generates new HD seeds and sets them
      | as active.
      | 
      | Returns false if already setup or setup
      | fails, true if setup is successful
      | 
      | Set force=true to make it re-setup if
      | already setup, used for upgrades
      |
      */
    fn setup_generation(&mut self, force: Option<bool>) -> bool {
        let force: bool = force.unwrap_or(false);

        todo!();
        /*
            return false;
        */
    }
}

impl IsHDEnabled for ScriptPubKeyMan {

    /**
      | Returns true if HD is enabled
      |
      */
    fn is_hd_enabled(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl CanGetAddresses for ScriptPubKeyMan {

    /**
      | Returns true if the wallet can give out
      | new addresses. This means it has keys
      | in the keypool or can generate new keys
      |
      */
    fn can_get_addresses(&self, internal: Option<bool>) -> bool {
        let internal: bool = internal.unwrap_or(false);

        todo!();
        /*
            return false;
        */
    }
}

impl Upgrade for ScriptPubKeyMan {

    /**
      | Upgrades the wallet to the specified
      | version
      |
      */
    fn upgrade(&mut self, 
        prev_version: i32,
        new_version:  i32,
        error:        &mut BilingualStr) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl HavePrivateKeys for ScriptPubKeyMan {

    fn have_private_keys(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl RewriteDB for ScriptPubKeyMan {

    /**
      | The action to do when the DB needs rewrite
      |
      */
    fn rewritedb(&mut self)  {
        
    }
}

impl GetOldestKeyPoolTime for ScriptPubKeyMan {

    fn get_oldest_key_pool_time(&self) -> i64 {
        
        todo!();
        /*
            return GetTime();
        */
    }
}

impl GetKeyPoolSize for ScriptPubKeyMan {

    fn get_key_pool_size(&self) -> u32 {
        
        todo!();
        /*
            return 0;
        */
    }
}

impl GetTimeFirstKey for ScriptPubKeyMan {

    fn get_time_first_key(&self) -> i64 {
        
        todo!();
        /*
            return 0;
        */
    }
}

impl GetMetadata for ScriptPubKeyMan {

    fn get_metadata(&self, dest: &TxDestination) -> Box<KeyMetadata> {
        
        todo!();
        /*
            return nullptr;
        */
    }
}

impl GetSolvingProvider for ScriptPubKeyMan {

    fn get_solving_provider(&self, script: &Script) -> Box<SigningProvider> {
        
        todo!();
        /*
            return nullptr;
        */
    }
}

impl CanProvide for ScriptPubKeyMan {

    /**
      | Whether this ScriptPubKeyMan can provide
      | a SigningProvider (via GetSolvingProvider)
      | that, combined with sigdata, can produce
      | solving data.
      |
      */
    fn can_provide(&mut self, 
        script:  &Script,
        sigdata: &mut SignatureData) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl SignTransaction for ScriptPubKeyMan {

    /**
      | Creates new signatures and adds them
      | to the transaction. Returns whether
      | all inputs were signed
      |
      */
    fn sign_transaction(&self, 
        tx:           &mut MutableTransaction,
        coins:        &HashMap<OutPoint,Coin>,
        sighash:      i32,
        input_errors: &mut HashMap<i32,BilingualStr>) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl SignMessage for ScriptPubKeyMan {

    /**
      | Sign a message with the given script
      |
      */
    fn sign_message(&self, 
        message: &String,
        pkhash:  &PKHash,
        str_sig: &mut String) -> SigningResult {
        
        todo!();
        /*
            return SigningResult::SIGNING_FAILED; }{
        */
    }
}

impl FillPSBT for ScriptPubKeyMan {

    /**
      | Adds script and derivation path information
      | to a PSBT, and optionally signs it.
      |
      */
    fn fill_psbt(&self, 
        psbt:         &mut PartiallySignedTransaction,
        txdata:       &PrecomputedTransactionData,
        sighash_type: Option<i32>,
        sign:         Option<bool>,
        bip_32derivs: Option<bool>,
        n_signed:     Option<*mut i32>) -> TransactionError {

        let sighash_type:  i32 = sighash_type.unwrap_or(1); // SIGHASH_ALL 
        let sign:         bool = sign.unwrap_or(true);
        let bip_32derivs: bool = bip_32derivs.unwrap_or(false);

        todo!();
        /*
            return TransactionError::INVALID_PSBT;
        */
    }
}

impl GetID for ScriptPubKeyMan {

    fn getid(&self) -> u256 {
        
        todo!();
        /*
            return uint256();
        */
    }
}

impl ScriptPubKeyMan {
    
    pub fn new<'a>(storage: &'a mut dyn WalletStorage) -> Self {
    
        todo!();
        /*
        : storage(storage),

        
        */
    }
    
    /**
      | Prepends the wallet name in logging
      | output to ease debugging in multi-wallet
      | use cases
      |
      */
    pub fn wallet_log_printf<Params>(&self, 
        fmt:        String,
        parameters: Params)  {
    
        todo!();
        /*
            LogPrintf(("%s " + fmt).c_str(), m_storage.GetDisplayName(), parameters...);
        */
    }
}
