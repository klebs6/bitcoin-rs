// ---------------- [ File: bitcoinchain-interface/src/traits.rs ]
crate::ix!();

pub trait ChainHeight {

    /**
      | Get current chain height, not including
      | genesis block (returns 0 if chain only
      | contains genesis block, nullopt if chain
      | does not contain any blocks)
      */
    fn height(&self) -> Option<usize>;
}

pub trait GetBlockHash {

    /**
       Get block hash. Height must be valid or
       this function will abort.
      */
    fn get_block_hash(&mut self, height: i32) -> u256;
}

pub trait HaveBlockOnDisk {

    /**
      | Check that the block is available on disk
      | (i.e. has not been pruned), and contains
      | transactions.
      */
    fn have_block_on_disk(&mut self, height: i32) -> bool;
}

pub trait GetTipLocator {

    /**
      | Get locator for the current chain tip.
      |
      */
    fn get_tip_locator(&mut self) -> BlockLocator;
}

pub trait Tip {

    /**
      | Returns the index entry for the tip of
      | this chain, or nullptr if none.
      |
      */
    fn tip(&self) -> Option<Arc<BlockIndex>>;
}

pub trait Contains<T> {

    fn contains(&self, item: Option<T>) -> bool;
}

pub trait GetLocator<T> {

    type LocatorType;

    fn get_locator(&self, item: Option<T>) -> Self::LocatorType;
}

pub trait FindLocatorFork {

    /**
      | Return height of the highest block on
      | chain in common with the locator, which
      | will either be the original block used to
      | create the locator, or one of its
      | ancestors.
      */
    fn find_locator_fork(&mut self, locator: &BlockLocator) -> Option<i32>;
}

pub trait CheckFinalTx {

    /**
      | Check if transaction will be final given
      | chain height current time.
      |
      */
    fn check_final_tx(&mut self, tx: &Transaction) -> bool;
}

pub trait FindCoins {

    /**
      | Look up unspent output
      | information. Returns coins in the mempool
      | and in the current chain UTXO
      | set. Iterates through all the keys in the
      | map and populates the values.
      */
    fn find_coins(&mut self, coins: &mut HashMap<OutPoint,Coin>);
}

pub trait GuessVerificationProgress {

    /**
      | Estimate fraction of total transactions
      | verified if blocks up to the specified
      | block hash are verified.
      |
      */
    fn guess_verification_progress(&mut self, block_hash: &u256) -> f64;
}

pub trait HasBlocks {

    /**
      | Return true if data is available for all
      | blocks in the specified range of
      | blocks. This checks all blocks that are
      | ancestors of block_hash in the height
      | range from min_height to max_height,
      | inclusive.
      */
    fn has_blocks(&mut self, 
            block_hash: &u256,
            min_height: i32,
            max_height: Option<i32>) -> bool;
}

pub trait IsInMempool {

    /**
      | Check if transaction is in mempool.
      |
      */
    fn is_in_mempool(&mut self, txid: &u256) -> bool;
}

pub trait HasDescendantsInMempool {

    /**
      | Check if transaction has descendants
      | in mempool.
      |
      */
    fn has_descendants_in_mempool(&mut self, txid: &u256) -> bool;
}

pub trait BroadcastTransaction {

    /**
      | Transaction is added to memory pool, if
      | the transaction fee is below the amount
      | specified by max_tx_fee, and broadcast to
      | all peers if relay is set to true.  Return
      | false if the transaction could not be
      | added due to the fee or for another
      | reason.
      */
    fn broadcast_transaction(&mut self, 
        tx:         &TransactionRef,
        max_tx_fee: &Amount,
        relay:      bool,
        err_string: &mut String) -> bool;
}

pub trait GetTransactionAncestry  {

    /**
      | Calculate mempool ancestor and descendant
      | counts for the given transaction.
      |
      */
    fn get_transaction_ancestry(&mut self, 
        txid:         &u256,
        ancestors:    &mut usize,
        descendants:  &mut usize,
        ancestorsize: *mut usize,
        ancestorfees: *mut Amount);
}

pub trait GetPackageLimits {

    /**
      | Get the node's package limits.
      |
      | Currently only returns the ancestor and
      | descendant count limits, but could be
      | enhanced to return more policy settings.
      */
    fn get_package_limits(&mut self, 
        limit_ancestor_count:   &mut u32,
        limit_descendant_count: &mut u32);
}

pub trait CheckChainLimits {

    /**
      | Check if transaction will pass the mempool's
      | chain limits.
      |
      */
    fn check_chain_limits(&mut self, tx: &TransactionRef) -> bool;
}

pub trait EstimateSmartFee {

    /**
      | Estimate smart fee.
      |
      */
    fn estimate_smart_fee(&mut self, 
            num_blocks:   i32,
            conservative: bool,
            calc:         *mut FeeCalculation) -> FeeRate;
}

pub trait EstimateMaxBlocks {

    /**
      | Fee estimator max target.
      |
      */
    fn estimate_max_blocks(&mut self) -> u32;
}

pub trait MemPoolMinFee {

    /**
      | Mempool minimum fee.
      |
      */
    fn mempool_min_fee(&mut self) -> FeeRate;
}

pub trait RelayMinFee {

    /**
      | Relay current minimum fee (from
      | -minrelaytxfee and -incrementalrelayfee
      | settings).
      |
      */
    fn relay_min_fee(&mut self) -> FeeRate;
}

pub trait RelayIncrementalFee {

    /**
      | Relay incremental fee setting (-incrementalrelayfee),
      | reflecting cost of relay.
      |
      */
    fn relay_incremental_fee(&mut self) -> FeeRate;
}

pub trait RelayDustFee {

    /**
      | Relay dust fee setting (-dustrelayfee),
      | reflecting lowest rate it's economical
      | to spend.
      |
      */
    fn relay_dust_fee(&mut self) -> FeeRate;
}

pub trait HavePruned {

    /**
      | Check if any block has been pruned.
      |
      */
    fn have_pruned(&mut self) -> bool;
}

pub trait IsReadyToBroadcast {

    /**
      | Check if the node is ready to broadcast
      | transactions.
      |
      */
    fn is_ready_to_broadcast(&mut self) -> bool;
}

pub trait ShutdownRequested {

    /**
      | Check if shutdown requested.
      |
      */
    fn shutdown_requested(&mut self) -> bool;
}

pub trait GetAdjustedTime {

    /**
      | Get adjusted time.
      |
      */
    fn get_adjusted_time(&mut self) -> i64;
}

pub trait InitMessage {

    /**
      | Send init message.
      |
      */
    fn init_message(&mut self, message: &String);
}

pub trait InitWarning {

    /**
      | Send init warning.
      |
      */
    fn init_warning(&mut self, message: &BilingualStr);
}

pub trait InitError {

    /**
      | Send init error.
      |
      */
    fn init_error(&mut self, message: &BilingualStr);
}

pub trait ShowProgress {

    /**
      | Send progress indicator.
      |
      */
    fn show_progress(&mut self, 
        title:           &String,
        progress:        i32,
        resume_possible: bool);
}

pub trait HandleNotifications {

    /**
      | Register handler for notifications.
      |
      */
    fn handle_notifications(
        &mut self, 
        notifications: Arc<dyn ChainNotifications>) -> Box<dyn Handler>;
}

pub trait WaitForNotificationsIfTipChanged {

    /**
      | Wait for pending notifications to be
      | processed unless block hash points
      | to the current chain tip.
      |
      */
    fn wait_for_notifications_if_tip_changed(&mut self, old_tip: &u256);
}

pub trait HandleRpc {

    /**
      | Register handler for RPC. Command is not
      | copied, so reference needs to remain valid
      | until Handler is disconnected.
      */
    fn handle_rpc(&mut self, command: &RPCCommand) -> Box<dyn Handler>;
}

pub trait RpcEnableDeprecated {

    /**
      | Check if deprecated RPC is enabled.
      |
      */
    fn rpc_enable_deprecated(&mut self, method: &String) -> bool;
}

pub trait RpcRunLater {

    /**
      | Run function after given number of seconds.
      | Cancel any previous calls with same
      | name.
      |
      */
    fn rpc_run_later(&mut self, 
            name:    &String,
            fn_:     fn() -> (),
            seconds: i64);
}

pub trait RpcSerializationFlags {

    /**
      | Current RPC serialization flags.
      |
      */
    fn rpc_serialization_flags(&mut self) -> i32;
}

pub trait GetSetting {

    /**
      | Get settings value.
      |
      */
    fn get_setting(&mut self, arg: &String) -> SettingsValue;
}

pub trait GetSettingsList {

    /**
      | Get list of settings values.
      |
      */
    fn get_settings_list(&mut self, arg: &String) -> Vec<SettingsValue>;
}

pub trait GetRwSetting {

    /**
      | Return <datadir>/settings.json setting
      | value.
      |
      */
    fn get_rw_setting(&mut self, name: &String) -> SettingsValue;
}

pub trait UpdateRwSetting {

    /**
      | Write a setting to
      | <datadir>/settings.json. Optionally just
      | update the setting in memory and do not
      | write the file.
      */
    fn update_rw_setting(
        &mut self, 
        name:  &String,
        value: &SettingsValue,
        write: bool) -> bool;
}

pub trait RequestMempoolTransactions {

    /**
      | Synchronously send
      | transactionAddedToMempool notifications
      | about all current mempool transactions to
      | the specified handler and return after the
      | last one is sent. These notifications
      | aren't coordinated with async
      | notifications sent by handleNotifications,
      | so out of date async notifications from
      | handleNotifications can arrive during and
      | after synchronous notifications from
      | requestMempoolTransactions. Clients need
      | to be prepared to handle this by ignoring
      | notifications about unknown removed
      | transactions and already added new
      | transactions.
      */
    fn request_mempool_transactions(
        &mut self, 
        notifications: Rc<RefCell<dyn ChainNotifications>>);
}

pub trait IsTaprootActive {

    /**
      | Check if Taproot has activated
      |
      */
    fn is_taproot_active(&mut self) -> bool;
}

pub trait ChainNext {

    fn next(&self, pindex: Option<Arc<BlockIndex>>) 
        -> Option<Arc<BlockIndex>>;
}
