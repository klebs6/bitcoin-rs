crate::ix!();

/**
  | A Wallet maintains a set of transactions
  | and balances, and provides the ability
  | to create new transactions.
  |
  */
pub struct Wallet {

    /**
      | WalletFlags set on this wallet.
      |
      */
    wallet_flags:                Atomic<u64>, // default = { 0 }

    /**
      | Interface for accessing chain state.
      |
      */
    chain:                       Rc<RefCell<dyn ChainInterface>>,

    /**
      | Wallet name: relative directory name
      | or "" for default wallet.
      |
      */
    name:                        String,

    /**
      | Internal database handle.
      |
      */
    database:                    Box<WalletDatabase>,

    external_spk_managers:       HashMap<OutputType,*mut ScriptPubKeyMan>,
    internal_spk_managers:       HashMap<OutputType,*mut ScriptPubKeyMan>,

    /**
      | Indexed by a unique identifier produced
      | by each ScriptPubKeyMan using
      | 
      | ScriptPubKeyMan::GetID. In many cases
      | it will be the hash of an internal structure
      |
      */
    spk_managers:                HashMap<u256,Box<ScriptPubKeyMan>>,

    /**
      | Main wallet lock.
      | 
      | This lock protects all the fields added
      | by CWallet.
      |
      */
    cs_wallet:                   Arc<Mutex<WalletInner>>,

    map_master_keys:             WalletMasterKeyMap,
    n_master_key_maxid:          u32, // default = 0

    wtx_ordered:                 WalletTxItems,

    n_accounting_entry_number:   u64, // default = 0

    /**
      | Registered interfaces::Chain::Notifications
      | handler.
      |
      */
    chain_notifications_handler: Box<dyn Handler>,

    /**
      | Used to prevent concurrent calls to
      | walletpassphrase RPC.
      |
      */
    unlock_mutex:                parking_lot::RawMutex,

    pay_tx_fee:                  FeeRate, // default = { DEFAULT_PAY_TX_FEE }
    confirm_target:              u32, // default = { DEFAULT_TX_CONFIRM_TARGET }

    /**
      | Allow Coin Selection to pick unconfirmed
      | UTXOs that were sent from our own wallet
      | if it cannot fund the transaction otherwise.
      |
      */
    spend_zero_conf_change:      bool, // default = { DEFAULT_SPEND_ZEROCONF_CHANGE }

    signal_rbf:                  bool, // default = { DEFAULT_WALLET_RBF }

    /**
      | will be false if -fallbackfee=0
      |
      */
    allow_fallback_fee:          bool, // default = { true }

    /**
      | Override with -mintxfee
      |
      */
    min_fee:                     FeeRate, // default = { DEFAULT_TRANSACTION_MINFEE }

    /**
      | If fee estimation does not have enough
      | data to provide estimates, use this
      | fee instead.
      | 
      | Has no effect if not using fee estimation
      | 
      | Override with -fallbackfee
      |
      */
    fallback_fee:         FeeRate, // default = { DEFAULT_FALLBACK_FEE }

    /**
      | If the cost to spend a change output at
      | this feerate is greater than the value
      | of the output itself, just drop it to
      | fees.
      |
      */
    discard_rate:         FeeRate, // default = { DEFAULT_DISCARD_FEE }

    /**
      | When the actual feerate is less than
      | the consolidate feerate, we will tend
      | to make transactions which consolidate
      | inputs. When the actual feerate is greater
      | than the consolidate feerate, we will
      | tend to make transactions which have
      | the lowest fees.
      |
      */
    consolidate_feerate:  FeeRate, // default = { DEFAULT_CONSOLIDATE_FEERATE }

    /**
      | The maximum fee amount we're willing
      | to pay to prioritize partial spend avoidance.
      |
      */
    max_aps_fee:          Amount, // default = { DEFAULT_MAX_AVOIDPARTIALSPEND_FEE }

    default_address_type: OutputType, // default = { DEFAULT_ADDRESS_TYPE }

    /**
      | Default output type for change outputs.
      | When unset, automatically choose type
      | based on address type setting and the
      | types other of non-change outputs (see
      | -changetype option documentation
      | and implementation in
      | 
      | CWallet::TransactionChangeType
      | for details).
      |
      */
    default_change_type:  Option<OutputType>,

    /**
      | Absolute maximum transaction fee (in
      | satoshis) used by default for the wallet
      |
      */
    default_max_tx_fee:   Amount, // default = { DEFAULT_TRANSACTION_MAXFEE }

    /**
      | Wallet is about to be unloaded
      |
      */
    notify_unload:                    Signal<fn() -> ()>,

    /**
      | Address book entry changed.
      | 
      | -----------
      | @note
      | 
      | called without lock cs_wallet held.
      |
      */
    notify_address_book_changed:      Signal<fn(
            address: &TxDestination,
            label:   &String,
            is_mine: bool,
            purpose: &String,
            status:  ChangeType
    ) -> ()>,

    /**
      | Wallet transaction added, removed
      | or updated.
      | 
      | -----------
      | @note
      | 
      | called with lock cs_wallet held.
      |
      */
    notify_transaction_changed:       Signal<fn(hash_tx: &u256, status: ChangeType) -> ()>,

    /**
      | Show progress e.g. for rescan
      |
      */
    show_progress:                    Signal<fn(title: &String, n_progress: i32) -> ()>,

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

    /**
      | Wallet status (encrypted, locked)
      | changed.
      | 
      | -----------
      | @note
      | 
      | Called without locks held.
      |
      */
    notify_status_changed:            Signal<fn(wallet: *mut Wallet) -> ()>,
}

impl WalletStorage      for Wallet { }
impl ChainNotifications for Wallet { }
impl BlockConnected     for Wallet { }
impl BlockDisconnected  for Wallet { }

pub struct WalletInner {

    /**
      | Used to keep track of spent outpoints,
      | and detect and report conflicts (double-spends
      | or mutated transactions where the mutant
      | gets mined).
      | 
      |
      */
    map_tx_spends:               WalletTxSpends,

    /**
      | The following is used to keep track of
      | how far behind the wallet is from the
      | chain sync, and to allow clients to block
      | on us being caught up.
      | 
      | Processed hash is a pointer on node's
      | tip and doesn't imply that the wallet
      | has scanned sequentially all blocks
      | up to this one.
      | 
      |
      */
    last_block_processed:        u256,

    /**
      | Height of last block processed is used
      | by wallet to know depth of transactions
      | without relying on Chain interface
      | beyond asynchronous updates. For safety,
      | we initialize it to -1. Height is a pointer
      | on node's tip and doesn't imply that
      | the wallet has scanned sequentially
      | all blocks up to this one.
      | 
      |
      */
    last_block_processed_height: i32, // default = -1

    /**
      | Map from txid to CWalletTx for all transactions
      | this wallet is interested in, including
      | received and sent transactions.
      | 
      |
      */
    map_wallet:                  HashMap<u256,WalletTx>,

    n_order_pos_next:            i64, // default = 0

    address_book:                HashMap<TxDestination,AddressBookData>,
    /**
      | Set of Coins owned by this wallet that
      | we won't try to spend from. A
      | 
      | Coin may be locked if it has already been
      | used to fund a transaction that hasn't
      | confirmed yet. We wouldn't consider
      | the Coin spent already, but also shouldn't
      | try to use it again.
      | 
      |
      */
    set_locked_coins:            HashSet<OutPoint>,

    /**
      | Holds a timestamp at which point the
      | wallet is scheduled (externally)
      | to be relocked. Caller must arrange
      | for actual relocking to occur via Lock().
      |
      */
    n_relock_time:               i64, // default = { 0 }

}

impl Drop for Wallet {

    fn drop(&mut self) {
        todo!();
        /*
            // Should not have slots connected at this point.
            assert(NotifyUnload.empty());
        */
    }
}

impl AddToSpendsWithOutpoint for Wallet {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn add_to_spends_with_outpoint(&mut self, 
        outpoint: &OutPoint,
        wtxid:    &u256,
        batch:    Option<*mut WalletBatch>)  {

        todo!();
        /*
        
        */
    }
}

impl AddToSpends for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn add_to_spends(&mut self, 
        wtxid: &u256,
        batch: Option<*mut WalletBatch>)  {

        todo!();
        /*
        
        */
    }
}

impl AddToWalletIfInvolvingMe for Wallet {

    /**
      | Add a transaction to the wallet, or update
      | it. confirm.block_* should be set when
      | the transaction was known to be included
      | in a block. When block_hash.IsNull(),
      | then wallet state is not updated in AddToWallet,
      | but notifications happen and cached
      | balances are marked dirty.
      | 
      | If fUpdate is true, existing transactions
      | will be updated.
      | 
      | TODO: One exception to this is that the
      | abandoned state is cleared under the
      | assumption that any further notification
      | of a transaction that was considered
      | abandoned is an indication that it is
      | not safe to be considered abandoned.
      | 
      | Abandoned state should probably be
      | more carefully tracked via different
      | chain notifications or by checking
      | mempool presence when necessary.
      | 
      | Should be called with rescanning_old_block
      | set to true, if the transaction is not
      | discovered in real time, but during
      | a rescan of old blocks.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn add_to_wallet_if_involving_me(&mut self, 
        tx:                   &TransactionRef,
        confirm:              WalletTxConfirmation,
        update:               bool,
        rescanning_old_block: bool) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl MarkConflicted for Wallet {

    /**
      | Mark a transaction (and its in-wallet
      | descendants) as conflicting with a
      | particular block.
      |
      */
    fn mark_conflicted(&mut self, 
        hash_block:         &u256,
        conflicting_height: i32,
        hash_tx:            &u256)  {
        
        todo!();
        /*
        
        */
    }
}

impl MarkInputsDirty for Wallet {

    /**
      | Mark a transaction's inputs dirty,
      | thus forcing the outputs to be recomputed
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn mark_inputs_dirty(&mut self, tx: &TransactionRef)  {
        
        todo!();
        /*
        
        */
    }
}

impl SyncMetaData for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn sync_meta_data(&mut self, _0: (WalletTxSpendsIterator,WalletTxSpendsIterator))  {
        
        todo!();
        /*
        
        */
    }
}

impl SyncTransaction for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn sync_transaction(&mut self, 
        tx:                   &TransactionRef,
        confirm:              WalletTxConfirmation,
        update_tx:            Option<bool>,
        rescanning_old_block: Option<bool>)  {

        let update_tx:            bool = update_tx.unwrap_or(true);
        let rescanning_old_block: bool = rescanning_old_block.unwrap_or(false);

        todo!();
        /*
        
        */
    }
}

impl SetAddressBookWithDB for Wallet {
    fn set_address_book_withdb(&mut self, 
        batch:       &mut WalletBatch,
        address:     &TxDestination,
        str_name:    &str,
        str_purpose: &str) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl UnsetWalletFlagWithDB for Wallet {

    /**
      | Unsets a wallet flag and saves it to disk
      |
      */
    fn unset_wallet_flag_withdb(&mut self, 
        batch: &mut WalletBatch,
        flag:  u64)  {
        
        todo!();
        /*
        
        */
    }
}

impl UnsetBlankWalletFlag for Wallet {

    /**
      | Unset the blank wallet flag and saves
      | it to disk
      |
      */
    fn unset_blank_wallet_flag(&mut self, batch: &mut WalletBatch)  {
        
        todo!();
        /*
        
        */
    }
}

impl AttachChain for Wallet {

    /**
      | Catch wallet up to current chain, scanning
      | new blocks, updating the best block
      | locator and m_last_block_processed,
      | and registering for notifications
      | about new blocks and transactions.
      |
      */
    fn attach_chain<'a>(
        wallet:          &Arc<Wallet>,
        chain:           &'a mut dyn ChainInterface,
        rescan_required: bool,
        error:           &mut BilingualStr,
        warnings:        &mut Vec<BilingualStr>) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl GetDatabase for Wallet {

    fn get_database(&self) -> &mut WalletDatabase {
        
        todo!();
        /*
            assert(static_cast<bool>(m_database));
            return *m_database;
        */
    }
}

impl GetName for Wallet {

    /**
      | Get a name for this wallet for logging/debugging
      | purposes.
      |
      */
    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return m_name;
        */
    }
}

impl Wallet {

    /**
      | Construct wallet with specified name
      | and database implementation.
      |
      */
    pub fn new<'a>(
        chain:    &'a mut dyn ChainInterface,
        name:     &str,
        database: Box<WalletDatabase>) -> Self {
    
        todo!();
        /*
            : m_chain(chain),
              m_name(name),
              m_database(std::move(database))
        */
    }

    /**
      | Interface for accessing chain state.
      |
      */
    fn chain<'a>(&self) -> &'a mut dyn ChainInterface {
        
        todo!();
        /*
            assert(m_chain); return *m_chain;
        */
    }
}

impl IsCrypted for Wallet {

    fn is_crypted(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl IsLocked for Wallet {

    fn is_locked(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl Lock for Wallet {

    fn lock(&mut self) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl HaveChain for Wallet {

    /**
      | Interface to assert chain access
      |
      */
    fn have_chain(&self) -> bool {
        
        todo!();
        /*
            return m_chain ? true : false;
        */
    }
}

impl FindAddressBookEntry for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn find_address_book_entry(&self, 
        _0:           &TxDestination,
        allow_change: Option<bool>) -> *const AddressBookData {
        let allow_change: bool = allow_change.unwrap_or(false);

        todo!();
        /*
        
        */
    }
}

impl GetWalletTx for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn get_wallet_tx(&self, hash: &u256) -> WalletTx {
        
        todo!();
        /*
        
        */
    }
}

impl GetTxConflicts for Wallet {

    /**
      | TODO: Remove "NO_THREAD_SAFETY_ANALYSIS"
      | and replace it with the correct annotation
      | "EXCLUSIVE_LOCKS_REQUIRED(pwallet->cs_wallet)". 
      |
      | The annotation "NO_THREAD_SAFETY_ANALYSIS"
      | was temporarily added to avoid having to
      | resolve the issue of member access into
      | incomplete type CWallet. 
      |
      | Note that we still have the runtime check
      | "AssertLockHeld(pwallet->cs_wallet)" in
      | place.
      */
    #[NO_THREAD_SAFETY_ANALYSIS]
    fn get_tx_conflicts(&self, wtx: &WalletTx) -> HashSet<u256> {
        
        todo!();
        /*
        
        */
    }
}
impl GetTxDepthInMainChain for Wallet {

    /**
      | Return depth of transaction in blockchain:
      | 
      | <0 : conflicts with a transaction this
      | deep in the blockchain
      | 
      | 0 : in memory pool, waiting to be included
      | in a block
      | 
      | >=1 : this many blocks deep in the main
      | chain
      |
      */
    #[NO_THREAD_SAFETY_ANALYSIS]
    fn get_tx_depth_in_main_chain(&self, wtx: &WalletTx) -> i32 {

        /*
           | TODO: Remove "NO_THREAD_SAFETY_ANALYSIS"
           | and replace it with the correct annotation
           | "EXCLUSIVE_LOCKS_REQUIRED(pwallet->cs_wallet)". The
           | annotation "NO_THREAD_SAFETY_ANALYSIS" was
           | temporarily added to avoid having to
           | resolve the issue of member access into
           | incomplete type CWallet. Note that we still
           | have the runtime check
           | "AssertLockHeld(pwallet->cs_wallet)" in
           | place.
           */

        todo!();
        /*
        
        */
    }
}

impl IsTxInMainChain for Wallet {

    fn is_tx_in_main_chain(&self, wtx: &WalletTx) -> bool {
        
        todo!();
        /*
            return GetTxDepthInMainChain(wtx) > 0;
        */
    }
}

impl GetTxBlocksToMaturity for Wallet {

    /**
      | @return
      | 
      | number of blocks to maturity for this
      | transaction:
      | 
      | 0 : is not a coinbase transaction, or
      | is a mature coinbase transaction
      | 
      | >0 : is a coinbase transaction which
      | matures in this many blocks
      |
      */
    fn get_tx_blocks_to_maturity(&self, wtx: &WalletTx) -> i32 {
        
        todo!();
        /*
        
        */
    }
}

impl IsTxImmatureCoinBase for Wallet {

    fn is_tx_immature_coin_base(&self, wtx: &WalletTx) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl CanSupportFeature for Wallet {

    /**
      | check whether we support the named feature
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn can_support_feature(&self, wf: WalletFeature) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_wallet); return IsFeatureSupported(nWalletVersion, wf);
        */
    }
}

impl IsSpent for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn is_spent(&self, 
        hash: &u256,
        n:    u32) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl IsSpentKey for Wallet {

    /**
      | Whether this or any known UTXO with the
      | same single key has been spent.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn is_spent_key(&self, 
        hash: &u256,
        n:    u32) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl SetSpentKeyState for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn set_spent_key_state(&mut self, 
        batch:           &mut WalletBatch,
        hash:            &u256,
        n:               u32,
        used:            bool,
        tx_destinations: &mut HashSet<TxDestination>)  {
        
        todo!();
        /*
        
        */
    }
}

impl DisplayAddress for Wallet {

    /**
      | Display address on an external signer.
      | Returns false if external signer support
      | is not compiled
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn display_address(&mut self, dest: &TxDestination) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl CheckIsLockedCoinWithHash for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn check_is_locked_coin_with_hash(&self, 
        hash: u256,
        n:    u32) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl LockCoin for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn lock_coin_with_batch(&mut self, 
        output: &OutPoint,
        batch:  Option<*mut WalletBatch>) -> bool {

        todo!();
        /*
        
        */
    }
}

impl UnlockCoinWithBatch for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn unlock_coin_with_batch(&mut self, 
        output: &OutPoint,
        batch:  Option<*mut WalletBatch>) -> bool {

        todo!();
        /*
        
        */
    }
}

impl UnlockAllCoins for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn unlock_all_coins(&mut self) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl ListLockedCoins for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn list_locked_coins(&self, outpts: &mut Vec<OutPoint>)  {
        
        todo!();
        /*
        
        */
    }
}

impl AbortRescan for Wallet {

    fn abort_rescan(&mut self)  {
        
        todo!();
        /*
            fAbortRescan = true;
        */
    }
}

impl IsAbortingRescan for Wallet {

    fn is_aborting_rescan(&self) -> bool {
        
        todo!();
        /*
            return fAbortRescan;
        */
    }
}

impl IsScanning for Wallet {

    fn is_scanning(&self) -> bool {
        
        todo!();
        /*
            return fScanningWallet;
        */
    }
}

impl ScanningDuration for Wallet {
    fn scanning_duration(&self) -> i64 {
        
        todo!();
        /*
            return fScanningWallet ? GetTimeMillis() - m_scanning_start : 0;
        */
    }
}

impl ScanningProgress for Wallet {
    fn scanning_progress(&self) -> f64 {
        
        todo!();
        /*
            return fScanningWallet ? (double) m_scanning_progress : 0;
        */
    }
}

impl UpgradeKeyMetadata for Wallet {

    /**
      | Upgrade stored CKeyMetadata objects
      | to store key origin info as KeyOriginInfo
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn upgrade_key_metadata(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl UpgradeDescriptorCache for Wallet {

    /**
      | Upgrade DescriptorCaches
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn upgrade_descriptor_cache(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl LoadMinVersion for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn load_min_version(&mut self, n_version: i32) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs_wallet); nWalletVersion = nVersion; return true;
        */
    }
}

impl LoadDestData for Wallet {

    /**
      | Adds a destination data tuple to the
      | store, without saving it to disk
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn load_dest_data(&mut self, 
        dest:  &TxDestination,
        key:   &String,
        value: &String)  {
        
        todo!();
        /*
        
        */
    }
}

impl Unlock for Wallet {

    fn unlock(&mut self, 
        str_wallet_passphrase: &SecureString,
        accept_no_keys:        Option<bool>) -> bool {
        let accept_no_keys: bool = accept_no_keys.unwrap_or(false);

        todo!();
        /*
        
        */
    }
}

impl ChangeWalletPassphrase for Wallet {
    fn change_wallet_passphrase(&mut self, 
        str_old_wallet_passphrase: &SecureString,
        str_new_wallet_passphrase: &SecureString) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl EncryptWallet for Wallet {
    fn encrypt_wallet(&mut self, str_wallet_passphrase: &SecureString) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl GetKeyBirthTimes for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn get_key_birth_times(&self, map_key_birth: &mut HashMap<KeyID,i64>)  {
        
        todo!();
        /*
        
        */
    }
}

impl ComputeTimeSmart for Wallet {

    fn compute_time_smart(&self, 
        wtx:                  &WalletTx,
        rescanning_old_block: bool) -> u32 {
        
        todo!();
        /*
        
        */
    }
}

impl IncOrderPosNext for Wallet {

    /**
      | Increment the next transaction order
      | id
      | 
      | -----------
      | @return
      | 
      | next transaction order id
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn inc_order_pos_next(&mut self, batch: Option<*mut WalletBatch>) -> i64 {

        todo!();
        /*
        
        */
    }
}

impl ReorderTransactions for Wallet {

    fn reorder_transactions(&mut self) -> DBErrors {
        
        todo!();
        /*
        
        */
    }
}

impl MarkDirty for Wallet {
    fn mark_dirty(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl AddToWallet for Wallet {

    fn add_to_wallet(&mut self, 
        tx:                   TransactionRef,
        confirm:              &WalletTxConfirmation,
        update_wtx:           Option<&WalletUpdateWalletTxFn>,
        flush_on_close:       Option<bool>,
        rescanning_old_block: Option<bool>) -> *mut WalletTx {

        let flush_on_close: bool = flush_on_close.unwrap_or(true);
        let rescanning_old_block: bool = rescanning_old_block.unwrap_or(false);

        todo!();
        /*
        
        */
    }
}

impl LoadToWallet for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn load_to_wallet(&mut self, 
        hash:     &u256,
        fill_wtx: &WalletUpdateWalletTxFn) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl TransactionAddedToMempool for Wallet {
    fn transaction_added_to_mempool(&mut self, 
        tx:               &TransactionRef,
        mempool_sequence: u64)  {
        
        todo!();
        /*
        
        */
    }
}

impl UpdatedBlockTip for Wallet { }

impl RescanFromTime for Wallet {
    fn rescan_from_time(&mut self, 
        start_time: i64,
        reserver:   &WalletRescanReserver,
        update:     bool) -> i64 {
        
        todo!();
        /*
        
        */
    }
}

impl ScanForWalletTransactions for Wallet {

    fn scan_for_wallet_transactions(&mut self, 
        start_block:  &u256,
        start_height: i32,
        max_height:   Option<i32>,
        reserver:     &WalletRescanReserver,
        update:       bool) -> WalletScanResult {
        
        todo!();
        /*
        
        */
    }
}

impl TransactionRemovedFromMempool for Wallet {

    fn transaction_removed_from_mempool(&mut self, 
        tx:               &TransactionRef,
        reason:           MemPoolRemovalReason,
        mempool_sequence: u64)  {
        
        todo!();
        /*
        
        */
    }
}

impl ReacceptWalletTransactions for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn reaccept_wallet_transactions(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl ResendWalletTransactions for Wallet {

    fn resend_wallet_transactions(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl TransactionChangeType for Wallet {

    fn transaction_change_type(&self, 
        change_type: &Option<OutputType>,
        vec_send:    &Vec<Recipient>) -> OutputType {
        
        todo!();
        /*
        
        */
    }
}

impl WalletSignTransaction for Wallet {

    /**
      | Fetch the inputs and sign with SIGHASH_ALL.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn sign_transaction(&self, tx: &mut MutableTransaction) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl SignTransactionGivenInputCoinsAndSighash for Wallet {

    /**
      | Sign the tx given the input coins and
      | sighash.
      |
      */
    fn sign_transaction_given_input_coins_and_sighash(&self, 
        tx:           &mut MutableTransaction,
        coins:        &HashMap<OutPoint,Coin>,
        sighash:      i32,
        input_errors: &mut HashMap<i32,BilingualStr>) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl SignMessage for Wallet {

    fn sign_message(&self, 
        message: &String,
        pkhash:  &PKHash,
        str_sig: &mut String) -> SigningResult {
        
        todo!();
        /*
        
        */
    }
}

impl WalletFillPSBT for Wallet {

    /**
      | Fills out a PSBT with information from
      | the wallet. Fills in UTXOs if we have
      | them. Tries to sign if sign=true. Sets
      | `complete` if the PSBT is now complete
      | (i.e. has all required signatures or
      | signature-parts, and is ready to finalize.)
      | Sets `error` and returns false if something
      | goes wrong.
      | 
      | -----------
      | @param[in] psbtx
      | 
      | PartiallySignedTransaction to fill
      | in
      | ----------
      | @param[out] complete
      | 
      | indicates whether the PSBT is now complete
      | ----------
      | @param[in] sighash_type
      | 
      | the sighash type to use when signing
      | (if PSBT does not specify)
      | ----------
      | @param[in] sign
      | 
      | whether to sign or not
      | ----------
      | @param[in] bip32derivs
      | 
      | whether to fill in bip32 derivation
      | information if available return error
      |
      */
    fn fill_psbt(&mut self, 
        sighash_type: i32,
        sign:         bool,
        bip_32derivs: bool,
        n_signed:     *mut usize,
        psbtx:        &mut PartiallySignedTransaction,
        complete:     &mut bool) -> TransactionError {

        /*
        let sighash_type:  i32 = sighash_type.unwrap_or(1 ); /* SIGHASH_ALL */
        let sign:         bool = sign.unwrap_or(true);
        let bip_32derivs: bool = bip_32derivs.unwrap_or(true);
        */

        todo!();
        /*
        
        */
    }
}

impl CommitTransaction for Wallet {

    /**
      | Submit the transaction to the node's
      | mempool and then relay to peers.
      | 
      | Should be called after CreateTransaction
      | unless you want to abort broadcasting
      | the transaction.
      | 
      | -----------
      | @param[in] tx
      | 
      | The transaction to be broadcast.
      | ----------
      | @param[in] mapValue
      | 
      | key-values to be set on the transaction.
      | ----------
      | @param[in] orderForm
      | 
      | BIP 70 / BIP 21 order form details to be
      | set on the transaction.
      |
      */
    fn commit_transaction(&mut self, 
        tx:         TransactionRef,
        map_value:  MapValue,
        order_form: Vec<(String,String)>)  {
        
        todo!();
        /*
        
        */
    }
}

impl SubmitTxMemoryPoolAndRelay for Wallet {

    /**
      | Pass this transaction to node for mempool
      | insertion and relay to peers if flag
      | set to true
      |
      */
    fn submit_tx_memory_pool_and_relay(&self, 
        wtx:        &WalletTx,
        err_string: &mut String,
        relay:      bool) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl DummySignTx for Wallet {

    fn dummy_sign_tx(&self, 
        tx_new:       &mut MutableTransaction,
        txouts:       &HashSet<TxOut>,
        coin_control: Option<*const CoinControl>) -> bool {

        todo!();
        /*
            std::vector<CTxOut> v_txouts(txouts.size());
            std::copy(txouts.begin(), txouts.end(), v_txouts.begin());
            return DummySignTx(txNew, v_txouts, coin_control);
        */
    }
}

impl ImportScripts for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn import_scripts(&mut self, 
        scripts:   HashSet<Script>,
        timestamp: i64) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl ImportPrivKeys for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn import_priv_keys(&mut self, 
        privkey_map: &HashMap<KeyID,Key>,
        timestamp:   i64) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl ImportPubKeys for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn import_pub_keys(&mut self, 
        ordered_pubkeys: &Vec<KeyID>,
        pubkey_map:      &HashMap<KeyID,PubKey>,
        key_origins:     &HashMap<KeyID,(PubKey,KeyOriginInfo)>,
        add_keypool:     bool,
        internal:        bool,
        timestamp:       i64) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl ImportScriptPubKeys for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn import_script_pub_keys(&mut self, 
        label:             &String,
        script_pub_keys:   &HashSet<Script>,
        have_solving_data: bool,
        apply_label:       bool,
        timestamp:         i64) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl KeypoolCountExternalKeys for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn keypool_count_external_keys(&self) -> usize {
        
        todo!();
        /*
        
        */
    }
}

impl TopUpKeyPool for Wallet {

    fn top_up_key_pool(&mut self, kp_size: Option<u32>) -> bool {
        let kp_size: u32 = kp_size.unwrap_or(0);

        todo!();
        /*
        
        */
    }
}

impl GetOldestKeyPoolTime for Wallet {

    fn get_oldest_key_pool_time(&self) -> i64 {
        
        todo!();
        /*
        
        */
    }
}

impl GetLabelAddresses for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn get_label_addresses(&self, label: &String) -> HashSet<TxDestination> {
        
        todo!();
        /*
        
        */
    }
}

impl MarkDestinationsDirty for Wallet {

    /**
      | Marks all outputs in each one of the destinations
      | dirty, so their cache is reset and does
      | not return outdated information.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn mark_destinations_dirty(&mut self, destinations: &HashSet<TxDestination>)  {
        
        todo!();
        /*
        
        */
    }
}

impl WalletGetNewDestination for Wallet {
    fn get_new_destination(&mut self, 
        ty:    OutputType,
        label: String,
        dest:  &mut TxDestination,
        error: &mut BilingualStr) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl GetNewChangeDestination for Wallet {
    fn get_new_change_destination(&mut self, 
        ty:    OutputType,
        dest:  &mut TxDestination,
        error: &mut BilingualStr) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl IsMineWithTxDest for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn is_mine_with_tx_dest(&self, dest: &TxDestination) -> IsMineType {
        
        todo!();
        /*
        
        */
    }
}

impl IsMineWithScript for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn is_mine_with_script(&self, script: &Script) -> IsMineType {
        
        todo!();
        /*
        
        */
    }
}

impl GetDebitWithTxinAndFilter for Wallet {

    /**
      | Returns amount of debit if the input
      | matches the filter, otherwise returns
      | 0
      |
      */
    fn get_debit_with_txin_and_filter(&self, 
        txin:   &TxIn,
        filter: &IsMineFilter) -> Amount {
        
        todo!();
        /*
        
        */
    }
}

impl IsMineWithTxout for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn is_mine_with_txout(&self, txout: &TxOut) -> IsMineType {
        
        todo!();
        /*
        
        */
    }
}

impl IsMineWithTx for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn is_mine_with_tx(&self, tx: &Transaction) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl IsFromMe for Wallet {

    /**
      | should probably be renamed to IsRelevantToMe
      |
      */
    fn is_from_me(&self, tx: &Transaction) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl GetDebitWithTxAndFilter for Wallet {

    fn get_debit_with_tx_and_filter(&self, 
        tx:     &Transaction,
        filter: &IsMineFilter) -> Amount {
        
        todo!();
        /*
        
        */
    }
}

impl ChainStateFlushed for Wallet {

    fn chain_state_flushed(&mut self, loc: &BlockLocator)  {
        
        todo!();
        /*
        
        */
    }
}

impl LoadWallet for Wallet {

    fn load_wallet(&mut self) -> DBErrors {
        
        todo!();
        /*
        
        */
    }
}

impl ZapSelectTx for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn zap_select_tx(&mut self, 
        hash_in:  &mut Vec<u256>,
        hash_out: &mut Vec<u256>) -> DBErrors {
        
        todo!();
        /*
        
        */
    }
}

impl SetAddressBook for Wallet {
    fn set_address_book(&mut self, 
        address:  &TxDestination,
        str_name: &String,
        purpose:  &String) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl DelAddressBook for Wallet {
    fn del_address_book(&mut self, address: &TxDestination) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl IsAddressUsed for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn is_address_used(&self, dest: &TxDestination) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl SetAddressUsed for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn set_address_used(&mut self, 
        batch: &mut WalletBatch,
        dest:  &TxDestination,
        used:  bool) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl GetAddressReceiveRequests for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn get_address_receive_requests(&self) -> Vec<String> {
        
        todo!();
        /*
        
        */
    }
}

impl SetAddressReceiveRequestWithBatch for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn set_address_receive_request_with_batch(
        &mut self, 
        batch: &mut WalletBatch,
        dest:  &TxDestination,
        id:    &String,
        value: &String) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl GetKeyPoolSize for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn get_key_pool_size(&self) -> u32 {
        
        todo!();
        /*
        
        */
    }
}

impl SetMinVersion for Wallet {

    /**
      | signify that a particular wallet feature
      | is now used.
      |
      */
    fn set_min_version(&mut self, 
        _0:       WalletFeature,
        batch_in: Option<*mut WalletBatch>)  {

        todo!();
        /*
        
        */
    }
}

impl GetVersion for Wallet {

    /**
      | get the current wallet format (the oldest
      | client version guaranteed to understand
      | this wallet)
      |
      */
    fn get_version(&self) -> i32 {
        
        todo!();
        /*
            LOCK(cs_wallet); return nWalletVersion;
        */
    }
}

impl GetConflicts for Wallet {

    /**
      | Get wallet transactions that conflict
      | with given transaction (spend same
      | outputs)
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn get_conflicts(&self, txid: &u256) -> HashSet<u256> {
        
        todo!();
        /*
        
        */
    }
}

impl HasWalletSpend for Wallet {

    /**
      | Check if a given transaction has any
      | of its outputs spent by another transaction
      | in the wallet
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn has_wallet_spend(&self, txid: &u256) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl Flush for Wallet {

    /**
      | Flush wallet (bitdb flush)
      |
      */
    fn flush(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl Close for Wallet {

    /**
      | Close wallet database
      |
      */
    fn close(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl GetBroadcastTransactions for Wallet {

    /**
      | Inquire whether this wallet broadcasts
      | transactions.
      |
      */
    fn get_broadcast_transactions(&self) -> bool {
        
        todo!();
        /*
            return fBroadcastTransactions;
        */
    }
}

impl SetBroadcastTransactions for Wallet {

    /**
      | Set whether this wallet broadcasts
      | transactions.
      |
      */
    fn set_broadcast_transactions(&mut self, broadcast: bool)  {
        
        todo!();
        /*
            fBroadcastTransactions = broadcast;
        */
    }
}

impl TransactionCanBeAbandoned for Wallet {

    /**
      | Return whether transaction can be abandoned
      |
      */
    fn transaction_can_be_abandoned(&self, hash_tx: &u256) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl AbandonTransaction for Wallet {

    /**
      | Mark a transaction (and it in-wallet
      | descendants) as abandoned so its inputs
      | may be respent.
      |
      */
    fn abandon_transaction(&mut self, hash_tx: &u256) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl MarkReplaced for Wallet {

    /**
      | Mark a transaction as replaced by another
      | transaction (e.g., BIP 125).
      |
      */
    fn mark_replaced(&mut self, 
        original_hash: &u256,
        new_hash:      &u256) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl Create for Wallet {

    /**
      | Initializes the wallet, returns a new
      | CWallet instance or a null pointer in
      | case of an error
      |
      */
    fn create(
        context:               &mut WalletContext,
        name:                  &String,
        database:              Box<WalletDatabase>,
        wallet_creation_flags: u64,
        error:                 &mut BilingualStr,
        warnings:              &mut Vec<BilingualStr>) -> Arc<Wallet> {
        
        todo!();
        /*
        
        */
    }
}

impl PostInitProcess for Wallet {

    /**
      | Wallet post-init setup
      | 
      | Gives the wallet a chance to register
      | repetitive tasks and complete post-init
      | tasks
      |
      */
    fn post_init_process(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl BackupWallet for Wallet {

    fn backup_wallet(&self, str_dest: &String) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl IsHDEnabled for Wallet {

    /**
      | Returns true if HD is enabled
      |
      */
    fn is_hd_enabled(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl CanGetAddresses for Wallet {

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
        
        */
    }
}

impl BlockUntilSyncedToCurrentChain for Wallet {

    /**
      | Blocks until the wallet state is up-to-date
      | to /at least/ the current chain at the
      | time this function is entered
      | 
      | Obviously holding cs_main/cs_wallet
      | when going into this call may cause deadlock
      |
      */
    #[LOCKS_EXCLUDED(::cs_main)]
    #[EXCLUSIVE_LOCKS_REQUIRED(!cs_wallet)]
    fn block_until_synced_to_current_chain(&self)  {
        
        todo!();
        /*
        
        */
    }
}

impl SetWalletFlag for Wallet {

    /**
      | set a single wallet flag
      |
      */
    fn set_wallet_flag(&mut self, flags: u64)  {
        
        todo!();
        /*
        
        */
    }
}

impl UnsetWalletFlag for Wallet {

    /**
      | Unsets a single wallet flag
      |
      */
    fn unset_wallet_flag(&mut self, flag: u64)  {
        
        todo!();
        /*
        
        */
    }
}

impl IsWalletFlagSet for Wallet {

    /**
      | check if a certain wallet flag is set
      |
      */
    fn is_wallet_flag_set(&self, flag: u64) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl AddWalletFlags for Wallet {

    /**
      | overwrite all flags by the given uint64_t
      | returns false if unknown, non-tolerable
      | flags are present
      |
      */
    fn add_wallet_flags(&mut self, flags: u64) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl LoadWalletFlags for Wallet {

    /**
      | Loads the flags into the wallet. (used
      | by LoadWallet)
      |
      */
    fn load_wallet_flags(&mut self, flags: u64) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl IsLegacy for Wallet {

    /**
      | Determine if we are a legacy wallet
      |
      */
    fn is_legacy(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl GetDisplayName for Wallet {

    /**
      | Returns a bracketed wallet name for
      | displaying in logs, will return [default
      | wallet] if the wallet has no name
      |
      */
    fn get_display_name(&self) -> String {
        
        todo!();
        /*
            std::string wallet_name = GetName().length() == 0 ? "default wallet" : GetName();
            return strprintf("[%s]", wallet_name);
        }{
        */
    }
}

impl WalletLogPrintf for Wallet {

    /**
      | Prepends the wallet name in logging
      | output to ease debugging in multi-wallet
      | use cases
      |
      */
    fn wallet_log_printf<Params>(&self, 
        fmt:        String,
        parameters: Params)  {
    
        todo!();
        /*
            LogPrintf(("%s " + fmt).c_str(), GetDisplayName(), parameters...);
        }{
        */
    }
}

impl UpgradeWallet for Wallet {

    /**
      | Upgrade the wallet
      |
      */
    fn upgrade_wallet(&mut self, 
        version: i32,
        error:   &mut BilingualStr) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl GetActiveScriptPubKeyMans for Wallet {

    /**
      | Returns all unique ScriptPubKeyMans
      | in m_internal_spk_managers and m_external_spk_managers
      |
      */
    fn get_active_script_pub_key_mans(&self) -> HashSet<*mut ScriptPubKeyMan> {
        
        todo!();
        /*
        
        */
    }
}

impl GetAllScriptPubKeyMans for Wallet {

    /**
      | Returns all unique ScriptPubKeyMans
      |
      */
    fn get_all_script_pub_key_mans(&self) -> HashSet<*mut ScriptPubKeyMan> {
        
        todo!();
        /*
        
        */
    }
}

impl GetScriptPubKeyMan for Wallet {

    /**
      | Get the ScriptPubKeyMan for the given
      | OutputType and internal/external
      | chain.
      |
      */
    fn get_script_pub_key_man(&self, 
        ty:       &OutputType,
        internal: bool) -> *mut ScriptPubKeyMan {
        
        todo!();
        /*
        
        */
    }
}

impl GetScriptPubKeyManWithScript for Wallet {

    /**
      | Get the ScriptPubKeyMan for a script
      |
      */
    fn get_script_pub_key_man_with_script(&self, script: &Script) -> *mut ScriptPubKeyMan {
        
        todo!();
        /*
        
        */
    }
}

impl GetScriptPubKeyManWithId for Wallet {

    /**
      | Get the ScriptPubKeyMan by id
      |
      */
    fn get_script_pub_key_man_with_id(&self, id: &u256) -> *mut ScriptPubKeyMan {
        
        todo!();
        /*
        
        */
    }
}

impl GetScriptPubKeyMans for Wallet {

    /**
      | Get all of the ScriptPubKeyMans for
      | a script given additional information
      | in sigdata (populated by e.g. a psbt)
      |
      */
    fn get_script_pub_key_mans(&self, 
        script:  &Script,
        sigdata: &mut SignatureData) -> HashSet<*mut ScriptPubKeyMan> {
        
        todo!();
        /*
        
        */
    }
}

impl GetSolvingProvider for Wallet {

    /**
      | Get the SigningProvider for a script
      |
      */
    fn get_solving_provider(&self, script: &Script) -> Box<SigningProvider> {
        
        todo!();
        /*
        
        */
    }
}

impl GetSolvingProviderWithSigdata for Wallet {

    fn get_solving_provider_with_sigdata(&self, 
        script:  &Script,
        sigdata: &mut SignatureData) -> Box<SigningProvider> {
        
        todo!();
        /*
        
        */
    }
}

impl GetLegacyScriptPubKeyMan for Wallet {

    /**
      | Get the LegacyScriptPubKeyMan which
      | is used for all types, internal, and
      | external.
      |
      */
    fn get_legacy_script_pub_key_man(&self) -> *mut LegacyScriptPubKeyMan {
        
        todo!();
        /*
        
        */
    }
}

impl GetOrCreateLegacyScriptPubKeyMan for Wallet {

    fn get_or_create_legacy_script_pub_key_man(&mut self) -> *mut LegacyScriptPubKeyMan {
        
        todo!();
        /*
        
        */
    }
}

impl SetupLegacyScriptPubKeyMan for Wallet {

    /**
      | Make a LegacyScriptPubKeyMan and set
      | it for all types, internal, and external.
      |
      */
    fn setup_legacy_script_pub_key_man(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl GetEncryptionKey for Wallet {

    fn get_encryption_key(&self) -> &KeyingMaterial {
        
        todo!();
        /*
        
        */
    }
}

impl HasEncryptionKeys for Wallet {

    fn has_encryption_keys(&self) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl GetLastBlockHeight for Wallet {

    /**
      | Get last block processed height
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn get_last_block_height(&self) -> i32 {
        
        todo!();
        /*
            AssertLockHeld(cs_wallet);
            assert(m_last_block_processed_height >= 0);
            return m_last_block_processed_height;
        }{
        */
    }
}

impl GetLastBlockHash for Wallet {

    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn get_last_block_hash(&self) -> u256 {
        
        todo!();
        /*
            AssertLockHeld(cs_wallet);
            assert(m_last_block_processed_height >= 0);
            return m_last_block_processed;
        */
    }
}

impl SetLastBlockProcessed for Wallet {

    /**
      | Set last block processed height, currently
      | only use in unit test
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn set_last_block_processed(&mut self, 
        block_height: i32,
        block_hash:   u256)  {
        
        todo!();
        /*
            AssertLockHeld(cs_wallet);
            m_last_block_processed_height = block_height;
            m_last_block_processed = block_hash;
        }{
        */
    }
}

impl ConnectScriptPubKeyManNotifiers for Wallet {

    /**
      | Connect the signals from ScriptPubKeyMans
      | to the signals in CWallet
      |
      */
    fn connect_script_pub_key_man_notifiers(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl LoadDescriptorScriptPubKeyMan for Wallet {

    /**
      | Instantiate a descriptor ScriptPubKeyMan
      | from the WalletDescriptor and load
      | it
      |
      */
    fn load_descriptor_script_pub_key_man(&mut self, 
        id:   u256,
        desc: &mut WalletDescriptor)  {
        
        todo!();
        /*
        
        */
    }
}

impl AddActiveScriptPubKeyMan for Wallet {

    /**
      | Adds the active ScriptPubKeyMan for
      | the specified type and internal. Writes
      | it to the wallet file
      | 
      | -----------
      | @param[in] id
      | 
      | The unique id for the ScriptPubKeyMan
      | ----------
      | @param[in] type
      | 
      | The OutputType this ScriptPubKeyMan
      | provides addresses for
      | ----------
      | @param[in] internal
      | 
      | Whether this ScriptPubKeyMan provides
      | change addresses
      |
      */
    fn add_active_script_pub_key_man(&mut self, 
        id:       u256,
        ty:       OutputType,
        internal: bool)  {
        
        todo!();
        /*
        
        */
    }
}

impl LoadActiveScriptPubKeyMan for Wallet {

    /**
      | Loads an active ScriptPubKeyMan for
      | the specified type and internal. (used
      | by LoadWallet)
      | 
      | -----------
      | @param[in] id
      | 
      | The unique id for the ScriptPubKeyMan
      | ----------
      | @param[in] type
      | 
      | The OutputType this ScriptPubKeyMan
      | provides addresses for
      | ----------
      | @param[in] internal
      | 
      | Whether this ScriptPubKeyMan provides
      | change addresses
      |
      */
    fn load_active_script_pub_key_man(&mut self, 
        id:       u256,
        ty:       OutputType,
        internal: bool)  {
        
        todo!();
        /*
        
        */
    }
}

impl DeactivateScriptPubKeyMan for Wallet {

    /**
      | Remove specified ScriptPubKeyMan
      | from set of active SPK managers. Writes
      | the change to the wallet file.
      | 
      | -----------
      | @param[in] id
      | 
      | The unique id for the ScriptPubKeyMan
      | ----------
      | @param[in] type
      | 
      | The OutputType this ScriptPubKeyMan
      | provides addresses for
      | ----------
      | @param[in] internal
      | 
      | Whether this ScriptPubKeyMan provides
      | change addresses
      |
      */
    fn deactivate_script_pub_key_man(&mut self, 
        id:       u256,
        ty:       OutputType,
        internal: bool)  {
        
        todo!();
        /*
        
        */
    }
}

impl SetupDescriptorScriptPubKeyMans for Wallet {

    /**
      | Create new DescriptorScriptPubKeyMans
      | and add them to the wallet
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn setup_descriptor_script_pub_key_mans(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

impl GetDescriptorScriptPubKeyMan for Wallet {

    /**
      | Return the DescriptorScriptPubKeyMan
      | for a WalletDescriptor if it is already
      | in the wallet
      |
      */
    fn get_descriptor_script_pub_key_man(&self, desc: &WalletDescriptor) -> *mut DescriptorScriptPubKeyMan {
        
        todo!();
        /*
        
        */
    }
}

impl AddWalletDescriptor for Wallet {

    /**
      | Add a descriptor to the wallet, return
      | a ScriptPubKeyMan & associated output
      | type
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs_wallet)]
    fn add_wallet_descriptor(&mut self, 
        desc:             &mut WalletDescriptor,
        signing_provider: &FlatSigningProvider,
        label:            &String,
        internal:         bool) -> *mut ScriptPubKeyMan {
        
        todo!();
        /*
        
        */
    }
}
