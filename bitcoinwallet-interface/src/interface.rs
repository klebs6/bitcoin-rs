crate::ix!();

/**
  | Error statuses for the wallet database
  |
  */
pub enum DBErrors
{
    LOAD_OK,
    CORRUPT,
    NONCRITICAL_ERROR,
    TOO_NEW,
    LOAD_FAIL,
    NEED_REWRITE,
    NEED_RESCAN
}

pub type WalletOrderForm  = Vec<(String,String)>;
pub type WalletValueMap   = HashMap<String,String>;
pub type CoinsList        = HashMap<TxDestination,Vec<(OutPoint,WalletTxOut)>>;

pub type MapValue = HashMap<String,String>;

/**
  | Wallet transaction information.
  |
  */
pub struct WalletTxInfo
{
    tx:                    TransactionRef,
    txin_is_mine:          Vec<IsMineType>,
    txout_is_mine:         Vec<IsMineType>,
    txout_address:         Vec<TxDestination>,
    txout_address_is_mine: Vec<IsMineType>,
    credit:                Amount,
    debit:                 Amount,
    change:                Amount,
    time:                  i64,
    value_map:             HashMap<String,String>,
    is_coinbase:           bool,
}

/**
  | Updated transaction status.
  |
  */
pub struct WalletTxStatus
{
    block_height:        i32,
    blocks_to_maturity:  i32,
    depth_in_main_chain: i32,
    time_received:       u32,
    lock_time:           u32,
    is_final:            bool,
    is_trusted:          bool,
    is_abandoned:        bool,
    is_coinbase:         bool,
    is_in_main_chain:    bool,
}

/**
  | Wallet transaction output.
  |
  */
pub struct WalletTxOut
{
    txout:               TxOut,
    time:                i64,
    depth_in_main_chain: i32, // default = -1
    is_spent:            bool, // default = false
}

/**
  | A transaction with a bunch of additional
  | info that only the owner cares about.
  | 
  | It includes any unrecorded transactions
  | needed to link it back to the block chain.
  |
  | Disable copying of CWalletTx objects to
  | prevent bugs where instances get copied in
  | and out of the mapWallet map, and fields
  | are updated in the wrong copy.
  */
#[no_copy]
pub struct WalletTx {

    /**
     | Key/value map with information about the
     | transaction.
     |
     | The following keys can be read and written
     | through the map and are serialized in the
     | wallet database:
     |
     | "comment", "to"   - comment strings
     |                     provided to
     |                     sendtoaddress, and
     |                     sendmany wallet RPCs
     |
     | "replaces_txid"   - txid (as HexStr) of
     |                     transaction replaced by
     |                     bumpfee on transaction
     |                     created by bumpfee
     |
     | "replaced_by_txid" - txid (as HexStr) of
     |                      transaction created by
     |                      bumpfee on transaction
     |                      replaced by bumpfee
     |
     | "from", "message" - obsolete fields that
     |                     could be set in UI
     |                     prior to
     |                     2011 (removed in commit
     |                     4d9b223)
     |
     | The following keys are serialized in the
     | wallet database, but shouldn't be read or
     | written through the map (they will be
     | temporarily added and removed from the map
     | during serialization):
     |
     | "fromaccount"     - serialized strFromAccount value
     | "n"               - serialized nOrderPos value
     | "timesmart"       - serialized nTimeSmart value
     | "spent"           - serialized vfSpent value that 
     |                     existed prior to 2014
     |                     (removed in commit
     |                     93a18a3)
     */
    map_value:                MapValue,
    order_form:               Vec<(String,String)>,
    time_received_is_tx_time: u32,

    /**
      | time received by this node
      |
      */
    n_time_received:          u32,

    /**
     | Stable timestamp that never changes, and
     | reflects the order a transaction was added
     | to the wallet. 
     |
     | Timestamp is based on the block time for
     | a transaction added as part of a block, or
     | else the time when the transaction was
     | received if it wasn't part of a block, with
     | the timestamp adjusted in both cases so
     | timestamp order matches the order
     | transactions were added to the wallet. 
     |
     | More details can be found in
     | CWallet::ComputeTimeSmart().
     */
    n_time_smart: u32,

    /**
      | From me flag is set to 1 for transactions
      | that were created by the wallet on this
      | bitcoin node, and set to 0 for transactions
      | that were created externally and came
      | in through the network or sendrawtransaction
      | RPC.
      |
      */
    from_me:      bool,


    /**
      | position in ordered transaction list
      |
      */
    n_order_pos:  i64,

    it_wtx_ordered:  Box<dyn Iterator<Item = (i64, *mut WalletTx)>>,
    amounts:         [RefCell<CachableAmount>; WalletTxAmountType::AMOUNTTYPE_ENUM_ELEMENTS as usize],

    /**
      | This flag is true if all m_amounts caches
      | are empty. This is particularly useful
      | in places where MarkDirty is conditionally
      | called and the condition can be expensive
      | and thus can be skipped if the flag is
      | true.
      | 
      | See MarkDestinationsDirty.
      |
      */
    is_cache_empty:  RefCell<bool>, // default = { true }

    change_cached:   RefCell<bool>,
    in_mempool:      RefCell<bool>,
    n_change_cached: RefCell<Amount>,
    tx:              TransactionRef,
    confirm:         WalletTxConfirmation,
}

/**
  | Constant used in hashBlock to indicate
  | tx has been abandoned, only used at serialization/deserialization
  | to avoid ambiguity with conflicted.
  |
  */
pub const WALLET_TX_ABANDON_HASH: u256 = u256::ONE;

/**
  | memory only
  |
  */
pub enum WalletTxAmountType { 
    DEBIT, 
    CREDIT, 
    IMMATURE_CREDIT, 
    AVAILABLE_CREDIT, 
    AMOUNTTYPE_ENUM_ELEMENTS 
}

/**
  | New transactions start as UNCONFIRMED.
  | 
  | At BlockConnected, they will transition
  | to CONFIRMED.
  | 
  | In case of reorg, at BlockDisconnected,
  | they roll back to UNCONFIRMED.
  | 
  | If we detect a conflicting transaction
  | at block connection, we update conflicted
  | tx and its dependencies as CONFLICTED.
  | 
  | If tx isn't confirmed and outside of
  | mempool, the user may switch it to ABANDONED
  | by using the abandontransaction call.
  | 
  | This last status may be override by a
  | CONFLICTED or CONFIRMED transition.
  |
  */
pub enum WalletTxStatusCode {
    UNCONFIRMED,
    CONFIRMED,
    CONFLICTED,
    ABANDONED
}

/**
  | Confirmation includes tx status and
  | a triplet of {block height/block hash/tx
  | index in block} at which tx has been confirmed.
  | 
  | All three are set to 0 if tx is unconfirmed
  | or abandoned.
  | 
  | Meaning of these fields changes with
  | CONFLICTED state where they instead
  | point to block hash and block height
  | of the deepest conflicting tx.
  |
  */
pub struct WalletTxConfirmation {
    status:       WalletTxStatus,
    block_height: i32,
    hash_block:   u256,
    n_index:      i32,
}

impl WalletTxConfirmation {
    
    pub fn new(
        status:       Option<WalletTxStatusCode>,
        block_height: Option<i32>,
        block_hash:   Option<u256>,
        block_index:  Option<i32>) -> Self {

        let status:        WalletTxStatusCode = status.unwrap_or(WalletTxStatusCode::UNCONFIRMED);

        let block_height:  i32  = block_height.unwrap_or(0);
        let block_hash:    u256 = block_hash.unwrap_or(u256::ZERO);
        let block_index:   i32  = block_index.unwrap_or(0);

        todo!();
        /*


            : status{status}, block_height{block_height}, hashBlock{block_hash}, nIndex{block_index}
        */
    }
}

impl WalletTx {

    pub fn new(arg: TransactionRef) -> Self {
    
        todo!();
        /*
            : tx(std::move(arg))
            Init();
        */
    }
    
    pub fn init(&mut self)  {
        
        todo!();
        /*
            mapValue.clear();
            vOrderForm.clear();
            fTimeReceivedIsTxTime = false;
            nTimeReceived = 0;
            nTimeSmart = 0;
            fFromMe = false;
            fChangeCached = false;
            fInMempool = false;
            nChangeCached = 0;
            nOrderPos = -1;
            m_confirm = Confirmation{};
        */
    }
    
    
    pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            mapValue_t mapValueCopy = mapValue;

            mapValueCopy["fromaccount"] = "";
            if (nOrderPos != -1) {
                mapValueCopy["n"] = ToString(nOrderPos);
            }
            if (nTimeSmart) {
                mapValueCopy["timesmart"] = strprintf("%u", nTimeSmart);
            }

            std::vector<uint8_t> dummy_vector1; /// Used to be vMerkleBranch
            std::vector<uint8_t> dummy_vector2; /// Used to be vtxPrev
            bool dummy_bool = false; /// Used to be fSpent
            uint256 serializedHash = isAbandoned() ? ABANDON_HASH : m_confirm.hashBlock;
            int serializedIndex = isAbandoned() || isConflicted() ? -1 : m_confirm.nIndex;
            s << tx << serializedHash << dummy_vector1 << serializedIndex << dummy_vector2 << mapValueCopy << vOrderForm << fTimeReceivedIsTxTime << nTimeReceived << fFromMe << dummy_bool;
        */
    }
    
    
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            Init();

            std::vector<uint256> dummy_vector1; /// Used to be vMerkleBranch
            std::vector<CMerkleTx> dummy_vector2; /// Used to be vtxPrev
            bool dummy_bool; /// Used to be fSpent
            int serializedIndex;
            s >> tx >> m_confirm.hashBlock >> dummy_vector1 >> serializedIndex >> dummy_vector2 >> mapValue >> vOrderForm >> fTimeReceivedIsTxTime >> nTimeReceived >> fFromMe >> dummy_bool;

            /* At serialization/deserialization, an nIndex == -1 means that hashBlock refers to
             * the earliest block in the chain we know this or any in-wallet ancestor conflicts
             * with. If nIndex == -1 and hashBlock is ABANDON_HASH, it means transaction is abandoned.
             * In same context, an nIndex >= 0 refers to a confirmed transaction (if hashBlock set) or
             * unconfirmed one. Older clients interpret nIndex == -1 as unconfirmed for backward
             * compatibility (pre-commit 9ac63d6).
             */
            if (serializedIndex == -1 && m_confirm.hashBlock == ABANDON_HASH) {
                setAbandoned();
            } else if (serializedIndex == -1) {
                setConflicted();
            } else if (!m_confirm.hashBlock.IsNull()) {
                m_confirm.nIndex = serializedIndex;
                setConfirmed();
            }

            const auto it_op = mapValue.find("n");
            nOrderPos = (it_op != mapValue.end()) ? LocaleIndependentAtoi<int64_t>(it_op->second) : -1;
            const auto it_ts = mapValue.find("timesmart");
            nTimeSmart = (it_ts != mapValue.end()) ? static_cast<unsigned int>(LocaleIndependentAtoi<int64_t>(it_ts->second)) : 0;

            mapValue.erase("fromaccount");
            mapValue.erase("spent");
            mapValue.erase("n");
            mapValue.erase("timesmart");
        */
    }
    
    pub fn set_tx(&mut self, arg: TransactionRef)  {
        
        todo!();
        /*
            tx = std::move(arg);
        */
    }

    /**
      | make sure balances are recalculated
      |
      */
    pub fn mark_dirty(&mut self)  {
        
        todo!();
        /*
            m_amounts[DEBIT].Reset();
            m_amounts[CREDIT].Reset();
            m_amounts[IMMATURE_CREDIT].Reset();
            m_amounts[AVAILABLE_CREDIT].Reset();
            fChangeCached = false;
            m_is_cache_empty = true;
        */
    }

    pub fn is_abandoned(&self) -> bool {
        
        todo!();
        /*
            return m_confirm.status == CWalletTx::ABANDONED;
        */
    }
    
    pub fn set_abandoned(&mut self)  {
        
        todo!();
        /*
            m_confirm.status = CWalletTx::ABANDONED;
            m_confirm.hashBlock = uint256();
            m_confirm.block_height = 0;
            m_confirm.nIndex = 0;
        */
    }
    
    pub fn is_conflicted(&self) -> bool {
        
        todo!();
        /*
            return m_confirm.status == CWalletTx::CONFLICTED;
        */
    }
    
    pub fn set_conflicted(&mut self)  {
        
        todo!();
        /*
            m_confirm.status = CWalletTx::CONFLICTED;
        */
    }
    
    pub fn is_unconfirmed(&self) -> bool {
        
        todo!();
        /*
            return m_confirm.status == CWalletTx::UNCONFIRMED;
        */
    }
    
    pub fn set_unconfirmed(&mut self)  {
        
        todo!();
        /*
            m_confirm.status = CWalletTx::UNCONFIRMED;
        */
    }
    
    pub fn is_confirmed(&self) -> bool {
        
        todo!();
        /*
            return m_confirm.status == CWalletTx::CONFIRMED;
        */
    }
    
    pub fn set_confirmed(&mut self)  {
        
        todo!();
        /*
            m_confirm.status = CWalletTx::CONFIRMED;
        */
    }
    
    pub fn get_hash(&self) -> &u256 {
        
        todo!();
        /*
            return tx->GetHash();
        */
    }
    
    pub fn is_coinbase(&self) -> bool {
        
        todo!();
        /*
            return tx->IsCoinBase();
        */
    }
    
    /**
      | True if only scriptSigs are different
      |
      */
    pub fn is_equivalent_to(&self, tx: &WalletTx) -> bool {
        
        todo!();
        /*
            CMutableTransaction tx1 {*this->tx};
            CMutableTransaction tx2 {*_tx.tx};
            for (auto& txin : tx1.vin) txin.scriptSig = CScript();
            for (auto& txin : tx2.vin) txin.scriptSig = CScript();
            return CTransaction(tx1) == CTransaction(tx2);
        */
    }
    
    pub fn in_mempool(&self) -> bool {
        
        todo!();
        /*
            return fInMempool;
        */
    }
    
    pub fn get_tx_time(&self) -> i64 {
        
        todo!();
        /*
            int64_t n = nTimeSmart;
        return n ? n : nTimeReceived;
        */
    }
}

/**
  | Interface for accessing a wallet.
  |
  */
pub trait WalletInterface:
EncryptWallet
+ IsCrypted
+ Lock
+ Unlock
+ IsLocked
+ ChangeWalletPassphrase
+ AbortRescan
+ BackupWallet
+ GetWalletName
+ WalletGetNewDestination
+ GetPubKeyWithScriptAndAddress
+ SignMessage
+ IsSpendable
+ HaveWatchOnly
+ SetAddressBook
+ DelAddressBook
+ GetAddress
+ GetAddresses
+ GetAddressReceiveRequests
+ SetAddressReceiveRequest
+ DisplayAddress
+ LockCoin
+ UnlockCoin
+ IsLockedCoin
+ ListLockedCoins
+ CreateTransaction
+ CommitTransaction
+ TransactionCanBeAbandoned
+ AbandonTransaction
+ TransactionCanBeBumped
+ CreateBumpTransaction
+ SignBumpTransaction
+ CommitBumpTransaction
+ GetTx
+ GetWalletTx
+ GetWalletTxs
+ TryGetTxStatus
+ GetWalletTxDetails
+ WalletFillPSBT
+ GetBalances
+ TryGetBalances
+ GetBalance
+ GetAvailableBalance
+ TxinIsMine
+ TxoutIsMine
+ GetDebit
+ GetCredit
+ ListCoins
+ GetCoins
+ GetRequiredFee
+ GetMinimumFee
+ GetConfirmTarget
+ HdEnabled
+ CanGetAddresses
+ PrivateKeysDisabled
+ HasExternalSigner
+ GetDefaultAddressType
+ GetDefaultMaxTxFee
+ Remove
+ IsLegacy
+ HandleUnload
+ HandleShowProgress<Callback = WalletShowProgressFn>
+ HandleStatusChanged
+ HandleAddressBookChanged
+ HandleTransactionChanged
+ HandleWatchOnlyChanged
+ HandleCanGetAddressesChanged
+ GetWallet { }

pub trait EncryptWallet {
    fn encrypt_wallet(&mut self, wallet_passphrase: &SecureString) -> bool;
}

pub trait IsCrypted {

    /**
      | Return whether wallet is encrypted.
      |
      */
    fn is_crypted(&self) -> bool;
}

pub trait Lock {

    /**
      | Lock wallet.
      |
      */
    fn lock(&mut self) -> bool;
}

pub trait Unlock {

    /**
      | Unlock wallet.
      |
      */
    fn unlock(&mut self, 
        wallet_passphrase: &SecureString,
        accept_no_keys:    Option<bool>) -> bool;
}

pub trait IsLocked {

    /**
      | Return whether wallet is locked.
      |
      */
    fn is_locked(&self) -> bool;
}

pub trait ChangeWalletPassphrase {

    fn change_wallet_passphrase(&mut self, 
        old_wallet_passphrase: &SecureString,
        new_wallet_passphrase: &SecureString) -> bool;
}

pub trait AbortRescan {

    fn abort_rescan(&mut self);
}

pub trait BackupWallet {

    fn backup_wallet(&self, filename: &String) -> bool;
}

pub trait GetWalletName {

    fn get_wallet_name(&mut self) -> String;
}

pub trait WalletGetNewDestination {

    /**
      | Get a new address.
      |
      */
    fn get_new_destination(&mut self, 
        ty:    OutputType,
        label: String,
        dest:  &mut TxDestination,
        error: &mut BilingualStr) -> bool;
}

pub trait GetPubKeyWithScriptAndAddress {

    /**
      | Get public key.
      |
      */
    fn get_pub_key_with_script_and_address(&mut self, 
        script:  &Script,
        address: &KeyID,
        pub_key: &mut PubKey) -> bool;
}

pub trait SignMessage {

    /**
      | Sign a message with the given script
      |
      */
    fn sign_message(&self, 
        message: &String,
        pkhash:  &PKHash,
        str_sig: &mut String) -> SigningResult;
}

pub trait IsSpendable {

    /**
      | Return whether wallet has private key.
      |
      */
    fn is_spendable(&mut self, dest: &TxDestination) -> bool;
}

pub trait HaveWatchOnly {

    /**
      | Return whether wallet has watch only
      | keys.
      |
      */
    fn have_watch_only(&mut self) -> bool;
}

pub trait SetAddressBook {

    /**
      | Add or update address.
      |
      */
    fn set_address_book(&mut self, 
        dest:    &TxDestination,
        name:    &String,
        purpose: &String) -> bool;
}
    
pub trait DelAddressBook {

    /**
      | Remove address.
      |
      */
    fn del_address_book(&mut self, address: &TxDestination) -> bool;
}

pub trait GetAddress {

    /**
      | Look up address in wallet, return whether
      | exists.
      |
      */
    fn get_address(&mut self, 
        dest:    &TxDestination,
        name:    *mut String,
        is_mine: *mut IsMineType,
        purpose: *mut String) -> bool;
}

pub trait GetAddresses {

    /**
      | Get wallet address list.
      |
      */
    fn get_addresses(&mut self) -> Vec<WalletAddress>;
}

pub trait GetAddressReceiveRequests {

    /**
      | Get receive requests.
      |
      */
    fn get_address_receive_requests(&self) -> Vec<String>;
}

pub trait SetAddressReceiveRequest {

    /**
      | Save or remove receive request.
      |
      */
    fn set_address_receive_request(&mut self, 
        dest:  &TxDestination,
        id:    &String,
        value: &String) -> bool;
}

pub trait DisplayAddress {

    /**
      | Display address on external signer
      |
      */
    fn display_address(&mut self, dest: &TxDestination) -> bool;
}

pub trait LockCoin {

    fn lock_coin(&mut self, 
        output:      &OutPoint,
        write_to_db: bool) -> bool {
        false
    }

    fn lock_coin_with_batch(&mut self, 
        output: &OutPoint,
        batch:  Option<*mut WalletBatch>) -> bool {
        false
    }
}

pub trait UnlockCoin {

    fn unlock_coin(&mut self, output: &OutPoint) -> bool;
}

pub trait IsLockedCoin {

    /**
      | Return whether coin is locked.
      |
      */
    fn is_locked_coin(&mut self, output: &OutPoint) -> bool;
}

pub trait ListLockedCoins {

    fn list_locked_coins(&self, outpts: &mut Vec<OutPoint>);
}

pub trait CreateTransaction {

    fn create_transaction(&mut self, 
        recipients:   &Vec<Recipient>,
        coin_control: &CoinControl,
        sign:         bool,
        change_pos:   &mut i32,
        fee:          &mut Amount,
        fail_reason:  &mut BilingualStr) -> TransactionRef;

}

pub trait CommitTransaction {

    fn commit_transaction(&mut self, 
        tx:         TransactionRef,
        value_map:  WalletValueMap,
        order_form: WalletOrderForm);

    /*
    fn commit_transaction(&mut self, 
        tx:         TransactionRef,
        map_value:  MapValue,
        order_form: Vec<(String,String)>);
    */
}

pub trait TransactionCanBeAbandoned {

    /**
      | Return whether transaction can be abandoned.
      |
      */
    fn transaction_can_be_abandoned(&self, txid: &u256) -> bool;
}

pub trait AbandonTransaction {

    fn abandon_transaction(&mut self, hash_tx: &u256) -> bool;
}

pub trait TransactionCanBeBumped {

    /**
      | Return whether transaction can be bumped.
      |
      */
    fn transaction_can_be_bumped(&self, txid: &u256) -> bool;
}

pub trait CreateBumpTransaction {

    fn create_bump_transaction(&mut self, 
        txid:         &u256,
        coin_control: &CoinControl,
        errors:       &mut Vec<BilingualStr>,
        old_fee:      &mut Amount,
        new_fee:      &mut Amount,
        mtx:          &mut MutableTransaction) -> bool;
}

pub trait SignBumpTransaction {
    fn sign_bump_transaction(&mut self, mtx: &mut MutableTransaction) -> bool;
}

pub trait CommitBumpTransaction {

    fn commit_bump_transaction(&mut self, 
        txid:        &u256,
        mtx:         MutableTransaction,
        errors:      &mut Vec<BilingualStr>,
        bumped_txid: &mut u256) -> bool;

}

pub trait GetTx {

    /**
      | Get a transaction.
      |
      */
    fn get_tx(&mut self, txid: &u256) -> TransactionRef;
}

pub trait GetWalletTx {

    /**
      | Get transaction information.
      |
      */
    fn get_wallet_tx(&self, txid: &u256) -> WalletTx;
}

pub trait GetWalletTxs {

    /**
      | Get list of all wallet transactions.
      |
      */
    fn get_wallet_txs(&mut self) -> Vec<WalletTx>;
}

pub trait TryGetTxStatus {

    /**
      | Try to get updated status for a particular
      | transaction, if possible without blocking.
      |
      */
    fn try_get_tx_status(&mut self, 
        txid:       &u256,
        tx_status:  &mut WalletTxStatus,
        num_blocks: &mut i32,
        block_time: &mut i64) -> bool;
}

pub trait GetWalletTxDetails {

    /**
      | Get transaction details.
      |
      */
    fn get_wallet_tx_details(&mut self, 
        txid:       &u256,
        tx_status:  &mut WalletTxStatus,
        order_form: &mut WalletOrderForm,
        in_mempool: &mut bool,
        num_blocks: &mut i32) -> WalletTx;
}

pub trait WalletFillPSBT {

    fn fill_psbt(&mut self, 
        sighash_type: i32,
        sign:         bool,
        bip_32derivs: bool,
        n_signed:     *mut usize,
        psbtx:        &mut PartiallySignedTransaction,
        complete:     &mut bool) -> TransactionError;
}

pub trait GetBalances {

    fn get_balances(&mut self) -> WalletBalances;
}

pub trait TryGetBalances {

    /**
      | Get balances if possible without blocking.
      |
      */
    fn try_get_balances(&mut self, 
        balances:   &mut WalletBalances,
        block_hash: &mut u256) -> bool;
}

pub trait GetBalance {
    fn get_balance(&mut self) -> Amount;
}

pub trait GetAvailableBalance {
    fn get_available_balance(&mut self, coin_control: &CoinControl) -> Amount;
}

pub trait TxinIsMine {

    /**
      | Return whether transaction input belongs
      | to wallet.
      |
      */
    fn txin_is_mine(&mut self, txin: &TxIn) -> IsMineType;
}

pub trait TxoutIsMine {

    /**
      | Return whether transaction output
      | belongs to wallet.
      |
      */
    fn txout_is_mine(&mut self, txout: &TxOut) -> IsMineType;
}

pub trait GetDebit {

    /**
      | Return debit amount if transaction
      | input belongs to wallet.
      |
      */
    fn get_debit(&mut self, 
        txin:   &TxIn,
        filter: IsMineFilter) -> Amount;
}

pub trait GetCredit {

    /**
      | Return credit amount if transaction
      | input belongs to wallet.
      |
      */
    fn get_credit(&mut self, 
        txout:  &TxOut,
        filter: IsMineFilter) -> Amount;
}

pub trait ListCoins {

    /**
      | Return AvailableCoins + LockedCoins
      | grouped by wallet address. (put change
      | in one group with wallet address)
      |
      */
    fn list_coins(&mut self) -> CoinsList;
}

pub trait GetCoins {

    /**
      | Return wallet transaction output information.
      |
      */
    fn get_coins(&mut self, outputs: &Vec<OutPoint>) -> Vec<WalletTxOut>;
}

pub trait GetRequiredFee {

    fn get_required_fee(&mut self, tx_bytes: u32) -> Amount;
}

pub trait GetMinimumFee {

    fn get_minimum_fee(&mut self, 
        tx_bytes:        u32,
        coin_control:    &CoinControl,
        returned_target: *mut i32,
        reason:          *mut FeeReason) -> Amount;
}

pub trait GetConfirmTarget {

    /**
      | Get tx confirm target.
      |
      */
    fn get_confirm_target(&mut self) -> u32;
}

pub trait HdEnabled {

    /**
      | Return whether HD enabled.
      |
      */
    fn hd_enabled(&mut self) -> bool;
}

pub trait CanGetAddresses {

    /**
      | Returns true if the wallet can give out
      | new addresses. This means it has keys
      | in the keypool or can generate new keys
      |
      */
    fn can_get_addresses(&self, internal: Option<bool>) -> bool;
}

pub trait PrivateKeysDisabled {

    /**
      | Return whether private keys enabled.
      |
      */
    fn private_keys_disabled(&mut self) -> bool;
}

pub trait HasExternalSigner {

    /**
      | Return whether wallet uses an external
      | signer.
      |
      */
    fn has_external_signer(&mut self) -> bool;
}

pub trait GetDefaultAddressType {

    /**
      | Get default address type.
      |
      */
    fn get_default_address_type(&mut self) -> OutputType;
}

pub trait GetDefaultMaxTxFee {

    /**
      | Get max tx fee.
      |
      */
    fn get_default_max_tx_fee(&mut self) -> Amount;
}

pub trait Remove {

    /**
      | Remove wallet.
      |
      */
    fn remove(&mut self);
}

pub trait IsLegacy {

    /**
      | Return whether is a legacy wallet
      |
      */
    fn is_legacy(&self) -> bool;
}

pub trait HandleUnload {

    /**
      | Register handler for unload message.
      |
      */
    fn handle_unload(&mut self, fn_: WalletUnloadFn) -> Box<dyn Handler>;
}

pub trait HandleShowProgress {

    type Callback;

    /**
      | Register handler for progress messages.
      |
      */
    fn handle_show_progress(&mut self, fn_: Self::Callback) -> Box<dyn Handler>;
}

pub trait HandleStatusChanged {

    /**
      | Register handler for status changed
      | messages.
      |
      */
    fn handle_status_changed(&mut self, fn_: WalletStatusChangedFn) -> Box<dyn Handler>;
}

pub trait HandleAddressBookChanged {

    /**
      | Register handler for address book changed
      | messages.
      |
      */
    fn handle_address_book_changed(&mut self, fn_: WalletAddressBookChangedFn) -> Box<dyn Handler>;
}

pub trait HandleTransactionChanged {

    /**
      | Register handler for transaction changed
      | messages.
      |
      */
    fn handle_transaction_changed(&mut self, fn_: WalletTransactionChangedFn) -> Box<dyn Handler>;
}

pub trait HandleWatchOnlyChanged {

    /**
      | Register handler for watchonly changed
      | messages.
      |
      */
    fn handle_watch_only_changed(&mut self, fn_: WalletWatchOnlyChangedFn) -> Box<dyn Handler>;
}

pub trait HandleCanGetAddressesChanged {

    /**
      | Register handler for keypool changed
      | messages.
      |
      */
    fn handle_can_get_addresses_changed(&mut self, fn_: WalletCanGetAddressesChangedFn) -> Box<dyn Handler>;
}

pub trait GetWallet {

    /**
      | Return pointer to internal wallet class,
      | useful for testing.
      |
      */
    fn wallet(&mut self) -> Rc<RefCell<dyn WalletInterface>> {

        todo!();
        /*
           return nullptr;
           */
    }
}

/**
  | General change type (added, updated,
  | removed).
  |
  */
pub enum ChangeType {
    CT_NEW,
    CT_UPDATED,
    CT_DELETED
}

impl Default for ChangeType { 

    fn default() -> Self { 
        ChangeType::CT_NEW 
    }
}

/**
  | Access to the wallet database.
  | 
  | Opens the database and provides read
  | and write access to it. Each read and
  | write is its own transaction.
  | 
  | Multiple operation transactions can
  | be started using TxnBegin() and committed
  | using TxnCommit()
  | 
  | Otherwise the transaction will be committed
  | when the object goes out of scope.
  | 
  | Optionally (on by default) it will flush
  | to disk on close.
  | 
  | Every 1000 writes will automatically
  | trigger a flush to disk.
  |
  */
pub struct WalletBatch {
    batch:    Box<DatabaseBatch>,
    database: Rc<RefCell<WalletDatabase>>,
}

impl WalletBatch {
    
    pub fn writeic<K, T>(&mut self, 
        key:       &K,
        value:     &T,
        overwrite: Option<bool>) -> bool {

        let overwrite: bool = overwrite.unwrap_or(true);

        todo!();
        /*
            if (!m_batch->Write(key, value, fOverwrite)) {
                return false;
            }
            m_database.IncrementUpdateCounter();
            if (m_database.nUpdateCounter % 1000 == 0) {
                m_batch->Flush();
            }
            return true;
        */
    }
    
    
    pub fn eraseic<K>(&mut self, key: &K) -> bool {
    
        todo!();
        /*
            if (!m_batch->Erase(key)) {
                return false;
            }
            m_database.IncrementUpdateCounter();
            if (m_database.nUpdateCounter % 1000 == 0) {
                m_batch->Flush();
            }
            return true;
        */
    }
    
    pub fn new(
        database:       &mut WalletDatabase,
        flush_on_close: Option<bool>) -> Self {

        let flush_on_close: bool = flush_on_close.unwrap_or(true);

        todo!();
        /*
        : batch(database.MakeBatch(_fFlushOnClose)),
        : database(database),
        */
    }
    
    pub fn write_name(&mut self, 
        str_address: &str,
        str_name:    &str) -> bool {
        
        todo!();
        /*
            return WriteIC(std::make_pair(DBKeys::NAME, strAddress), strName);
        */
    }
    
    pub fn erase_name(&mut self, str_address: &String) -> bool {
        
        todo!();
        /*
            // This should only be used for sending addresses, never for receiving addresses,
        // receiving addresses must always have an address book entry if they're not change return.
        return EraseIC(std::make_pair(DBKeys::NAME, strAddress));
        */
    }
    
    pub fn write_purpose(&mut self, 
        str_address: &String,
        str_purpose: &String) -> bool {
        
        todo!();
        /*
            return WriteIC(std::make_pair(DBKeys::PURPOSE, strAddress), strPurpose);
        */
    }
    
    pub fn erase_purpose(&mut self, str_address: &String) -> bool {
        
        todo!();
        /*
            return EraseIC(std::make_pair(DBKeys::PURPOSE, strAddress));
        */
    }
    
    pub fn write_tx(&mut self, wtx: &WalletTx) -> bool {
        
        todo!();
        /*
            return WriteIC(std::make_pair(DBKeys::TX, wtx.GetHash()), wtx);
        */
    }
    
    pub fn erase_tx(&mut self, hash: u256) -> bool {
        
        todo!();
        /*
            return EraseIC(std::make_pair(DBKeys::TX, hash));
        */
    }
    
    pub fn write_key_metadata(&mut self, 
        meta:      &KeyMetadata,
        pubkey:    &PubKey,
        overwrite: bool) -> bool {
        
        todo!();
        /*
            return WriteIC(std::make_pair(DBKeys::KEYMETA, pubkey), meta, overwrite);
        */
    }
    
    pub fn write_key(&mut self, 
        vch_pub_key:  &PubKey,
        vch_priv_key: &PrivKey,
        key_meta:     &KeyMetadata) -> bool {
        
        todo!();
        /*
            if (!WriteKeyMetadata(keyMeta, vchPubKey, false)) {
            return false;
        }

        // hash pubkey/privkey to accelerate wallet load
        std::vector<unsigned char> vchKey;
        vchKey.reserve(vchPubKey.size() + vchPrivKey.size());
        vchKey.insert(vchKey.end(), vchPubKey.begin(), vchPubKey.end());
        vchKey.insert(vchKey.end(), vchPrivKey.begin(), vchPrivKey.end());

        return WriteIC(std::make_pair(DBKeys::KEY, vchPubKey), std::make_pair(vchPrivKey, Hash(vchKey)), false);
        */
    }
    
    pub fn write_crypted_key(&mut self, 
        vch_pub_key:        &PubKey,
        vch_crypted_secret: &Vec<u8>,
        key_meta:           &KeyMetadata) -> bool {
        
        todo!();
        /*
            if (!WriteKeyMetadata(keyMeta, vchPubKey, true)) {
            return false;
        }

        // Compute a checksum of the encrypted key
        uint256 checksum = Hash(vchCryptedSecret);

        const auto key = std::make_pair(DBKeys::CRYPTED_KEY, vchPubKey);
        if (!WriteIC(key, std::make_pair(vchCryptedSecret, checksum), false)) {
            // It may already exist, so try writing just the checksum
            std::vector<unsigned char> val;
            if (!m_batch->Read(key, val)) {
                return false;
            }
            if (!WriteIC(key, std::make_pair(val, checksum), true)) {
                return false;
            }
        }
        EraseIC(std::make_pair(DBKeys::KEY, vchPubKey));
        return true;
        */
    }
    
    pub fn write_master_key(&mut self, 
        nid:          u32,
        k_master_key: &MasterKey) -> bool {
        
        todo!();
        /*
            return WriteIC(std::make_pair(DBKeys::MASTER_KEY, nID), kMasterKey, true);
        */
    }
    
    pub fn write_cscript(&mut self, 
        hash:          &u160,
        redeem_script: &Script) -> bool {
        
        todo!();
        /*
            return WriteIC(std::make_pair(DBKeys::CSCRIPT, hash), redeemScript, false);
        */
    }
    
    pub fn write_watch_only(&mut self, 
        dest:     &Script,
        key_meta: &KeyMetadata) -> bool {
        
        todo!();
        /*
            if (!WriteIC(std::make_pair(DBKeys::WATCHMETA, dest), keyMeta)) {
            return false;
        }
        return WriteIC(std::make_pair(DBKeys::WATCHS, dest), uint8_t{'1'});
        */
    }
    
    pub fn erase_watch_only(&mut self, dest: &Script) -> bool {
        
        todo!();
        /*
            if (!EraseIC(std::make_pair(DBKeys::WATCHMETA, dest))) {
            return false;
        }
        return EraseIC(std::make_pair(DBKeys::WATCHS, dest));
        */
    }
    
    pub fn write_best_block(&mut self, locator: &BlockLocator) -> bool {
        
        todo!();
        /*
            WriteIC(DBKeys::BESTBLOCK, CBlockLocator()); // Write empty block locator so versions that require a merkle branch automatically rescan
        return WriteIC(DBKeys::BESTBLOCK_NOMERKLE, locator);
        */
    }
    
    pub fn read_best_block(&mut self, locator: &mut BlockLocator) -> bool {
        
        todo!();
        /*
            if (m_batch->Read(DBKeys::BESTBLOCK, locator) && !locator.vHave.empty()) return true;
        return m_batch->Read(DBKeys::BESTBLOCK_NOMERKLE, locator);
        */
    }
    
    pub fn write_order_pos_next(&mut self, n_order_pos_next: i64) -> bool {
        
        todo!();
        /*
            return WriteIC(DBKeys::ORDERPOSNEXT, nOrderPosNext);
        */
    }
    
    pub fn read_pool(&mut self, 
        n_pool:  i64,
        keypool: &mut KeyPool) -> bool {
        
        todo!();
        /*
            return m_batch->Read(std::make_pair(DBKeys::POOL, nPool), keypool);
        */
    }
    
    pub fn write_pool(&mut self, 
        n_pool:  i64,
        keypool: &KeyPool) -> bool {
        
        todo!();
        /*
            return WriteIC(std::make_pair(DBKeys::POOL, nPool), keypool);
        */
    }
    
    pub fn erase_pool(&mut self, n_pool: i64) -> bool {
        
        todo!();
        /*
            return EraseIC(std::make_pair(DBKeys::POOL, nPool));
        */
    }
    
    pub fn write_min_version(&mut self, n_version: i32) -> bool {
        
        todo!();
        /*
            return WriteIC(DBKeys::MINVERSION, nVersion);
        */
    }
    
    pub fn write_active_script_pub_key_man(&mut self, 
        ty:       u8,
        id:       &u256,
        internal: bool) -> bool {
        
        todo!();
        /*
            std::string key = internal ? DBKeys::ACTIVEINTERNALSPK : DBKeys::ACTIVEEXTERNALSPK;
        return WriteIC(make_pair(key, type), id);
        */
    }
    
    pub fn erase_active_script_pub_key_man(&mut self, 
        ty:       u8,
        internal: bool) -> bool {
        
        todo!();
        /*
            const std::string key{internal ? DBKeys::ACTIVEINTERNALSPK : DBKeys::ACTIVEEXTERNALSPK};
        return EraseIC(make_pair(key, type));
        */
    }
    
    pub fn write_descriptor_key(&mut self, 
        desc_id: &u256,
        pubkey:  &PubKey,
        privkey: &PrivKey) -> bool {
        
        todo!();
        /*
            // hash pubkey/privkey to accelerate wallet load
        std::vector<unsigned char> key;
        key.reserve(pubkey.size() + privkey.size());
        key.insert(key.end(), pubkey.begin(), pubkey.end());
        key.insert(key.end(), privkey.begin(), privkey.end());

        return WriteIC(std::make_pair(DBKeys::WALLETDESCRIPTORKEY, std::make_pair(desc_id, pubkey)), std::make_pair(privkey, Hash(key)), false);
        */
    }
    
    pub fn write_crypted_descriptor_key(&mut self, 
        desc_id: &u256,
        pubkey:  &PubKey,
        secret:  &Vec<u8>) -> bool {
        
        todo!();
        /*
            if (!WriteIC(std::make_pair(DBKeys::WALLETDESCRIPTORCKEY, std::make_pair(desc_id, pubkey)), secret, false)) {
            return false;
        }
        EraseIC(std::make_pair(DBKeys::WALLETDESCRIPTORKEY, std::make_pair(desc_id, pubkey)));
        return true;
        */
    }
    
    pub fn write_descriptor(&mut self, 
        desc_id:    &u256,
        descriptor: &WalletDescriptor) -> bool {
        
        todo!();
        /*
            return WriteIC(make_pair(DBKeys::WALLETDESCRIPTOR, desc_id), descriptor);
        */
    }
    
    pub fn write_descriptor_derived_cache(&mut self, 
        xpub:          &ExtPubKey,
        desc_id:       &u256,
        key_exp_index: u32,
        der_index:     u32) -> bool {
        
        todo!();
        /*
            std::vector<unsigned char> ser_xpub(BIP32_EXTKEY_SIZE);
        xpub.Encode(ser_xpub.data());
        return WriteIC(std::make_pair(std::make_pair(DBKeys::WALLETDESCRIPTORCACHE, desc_id), std::make_pair(key_exp_index, der_index)), ser_xpub);
        */
    }
    
    pub fn write_descriptor_parent_cache(&mut self, 
        xpub:          &ExtPubKey,
        desc_id:       &u256,
        key_exp_index: u32) -> bool {
        
        todo!();
        /*
            std::vector<unsigned char> ser_xpub(BIP32_EXTKEY_SIZE);
        xpub.Encode(ser_xpub.data());
        return WriteIC(std::make_pair(std::make_pair(DBKeys::WALLETDESCRIPTORCACHE, desc_id), key_exp_index), ser_xpub);
        */
    }
    
    pub fn write_descriptor_last_hardened_cache(&mut self, 
        xpub:          &ExtPubKey,
        desc_id:       &u256,
        key_exp_index: u32) -> bool {
        
        todo!();
        /*
            std::vector<unsigned char> ser_xpub(BIP32_EXTKEY_SIZE);
        xpub.Encode(ser_xpub.data());
        return WriteIC(std::make_pair(std::make_pair(DBKeys::WALLETDESCRIPTORLHCACHE, desc_id), key_exp_index), ser_xpub);
        */
    }
    
    pub fn write_descriptor_cache_items(&mut self, 
        desc_id: &u256,
        cache:   &DescriptorCache) -> bool {
        
        todo!();
        /*
            for (const auto& parent_xpub_pair : cache.GetCachedParentExtPubKeys()) {
            if (!WriteDescriptorParentCache(parent_xpub_pair.second, desc_id, parent_xpub_pair.first)) {
                return false;
            }
        }
        for (const auto& derived_xpub_map_pair : cache.GetCachedDerivedExtPubKeys()) {
            for (const auto& derived_xpub_pair : derived_xpub_map_pair.second) {
                if (!WriteDescriptorDerivedCache(derived_xpub_pair.second, desc_id, derived_xpub_map_pair.first, derived_xpub_pair.first)) {
                    return false;
                }
            }
        }
        for (const auto& lh_xpub_pair : cache.GetCachedLastHardenedExtPubKeys()) {
            if (!WriteDescriptorLastHardenedCache(lh_xpub_pair.second, desc_id, lh_xpub_pair.first)) {
                return false;
            }
        }
        return true;
        */
    }
    
    pub fn write_lockedutxo(&mut self, output: &OutPoint) -> bool {
        
        todo!();
        /*
            return WriteIC(std::make_pair(DBKeys::LOCKED_UTXO, std::make_pair(output.hash, output.n)), uint8_t{'1'});
        */
    }
    
    pub fn erase_lockedutxo(&mut self, output: &OutPoint) -> bool {
        
        todo!();
        /*
            return EraseIC(std::make_pair(DBKeys::LOCKED_UTXO, std::make_pair(output.hash, output.n)));
        */
    }

    /**
      | Function to determine if a certain KV/key-type
      | is a key (cryptographical key) type
      |
      */
    pub fn is_key_type(&mut self, str_type: &String) -> bool {
        
        todo!();
        /*
            return (strType == DBKeys::KEY ||
                strType == DBKeys::MASTER_KEY || strType == DBKeys::CRYPTED_KEY);
        */
    }
    
    pub fn load_wallet(&mut self, pwallet: &mut dyn WalletInterface) -> DBErrors {
        
        todo!();
        /*
            CWalletScanState wss;
        bool fNoncriticalErrors = false;
        bool rescan_required = false;
        DBErrors result = DBErrors::LOAD_OK;

        LOCK(pwallet->cs_wallet);
        try {
            int nMinVersion = 0;
            if (m_batch->Read(DBKeys::MINVERSION, nMinVersion)) {
                if (nMinVersion > FEATURE_LATEST)
                    return DBErrors::TOO_NEW;
                pwallet->LoadMinVersion(nMinVersion);
            }

            // Load wallet flags, so they are known when processing other records.
            // The FLAGS key is absent during wallet creation.
            uint64_t flags;
            if (m_batch->Read(DBKeys::FLAGS, flags)) {
                if (!pwallet->LoadWalletFlags(flags)) {
                    pwallet->WalletLogPrintf("Error reading wallet database: Unknown non-tolerable wallet flags found\n");
                    return DBErrors::CORRUPT;
                }
            }

    #ifndef ENABLE_EXTERNAL_SIGNER
            if (pwallet->IsWalletFlagSet(WALLET_FLAG_EXTERNAL_SIGNER)) {
                pwallet->WalletLogPrintf("Error: External signer wallet being loaded without external signer support compiled\n");
                return DBErrors::TOO_NEW;
            }
    #endif

            // Get cursor
            if (!m_batch->StartCursor())
            {
                pwallet->WalletLogPrintf("Error getting wallet database cursor\n");
                return DBErrors::CORRUPT;
            }

            while (true)
            {
                // Read next record
                DataStream ssKey(SER_DISK, CLIENT_VERSION);
                DataStream ssValue(SER_DISK, CLIENT_VERSION);
                bool complete;
                bool ret = m_batch->ReadAtCursor(ssKey, ssValue, complete);
                if (complete) {
                    break;
                }
                else if (!ret)
                {
                    m_batch->CloseCursor();
                    pwallet->WalletLogPrintf("Error reading next record from wallet database\n");
                    return DBErrors::CORRUPT;
                }

                // Try to be tolerant of single corrupt records:
                std::string strType, strErr;
                if (!ReadKeyValue(pwallet, ssKey, ssValue, wss, strType, strErr))
                {
                    // losing keys is considered a catastrophic error, anything else
                    // we assume the user can live with:
                    if (IsKeyType(strType) || strType == DBKeys::DEFAULTKEY) {
                        result = DBErrors::CORRUPT;
                    } else if (strType == DBKeys::FLAGS) {
                        // reading the wallet flags can only fail if unknown flags are present
                        result = DBErrors::TOO_NEW;
                    } else if (wss.tx_corrupt) {
                        pwallet->WalletLogPrintf("Error: Corrupt transaction found. This can be fixed by removing transactions from wallet and rescanning.\n");
                        // Set tx_corrupt back to false so that the error is only printed once (per corrupt tx)
                        wss.tx_corrupt = false;
                        result = DBErrors::CORRUPT;
                    } else {
                        // Leave other errors alone, if we try to fix them we might make things worse.
                        fNoncriticalErrors = true; // ... but do warn the user there is something wrong.
                        if (strType == DBKeys::TX)
                            // Rescan if there is a bad transaction record:
                            rescan_required = true;
                    }
                }
                if (!strErr.empty())
                    pwallet->WalletLogPrintf("%s\n", strErr);
            }
        } catch (...) {
            result = DBErrors::CORRUPT;
        }
        m_batch->CloseCursor();

        // Set the active ScriptPubKeyMans
        for (auto spk_man_pair : wss.m_active_external_spks) {
            pwallet->LoadActiveScriptPubKeyMan(spk_man_pair.second, spk_man_pair.first, /* internal */ false);
        }
        for (auto spk_man_pair : wss.m_active_internal_spks) {
            pwallet->LoadActiveScriptPubKeyMan(spk_man_pair.second, spk_man_pair.first, /* internal */ true);
        }

        // Set the descriptor caches
        for (auto desc_cache_pair : wss.m_descriptor_caches) {
            auto spk_man = pwallet->GetScriptPubKeyMan(desc_cache_pair.first);
            assert(spk_man);
            ((DescriptorScriptPubKeyMan*)spk_man)->SetCache(desc_cache_pair.second);
        }

        // Set the descriptor keys
        for (auto desc_key_pair : wss.m_descriptor_keys) {
            auto spk_man = pwallet->GetScriptPubKeyMan(desc_key_pair.first.first);
            ((DescriptorScriptPubKeyMan*)spk_man)->AddKey(desc_key_pair.first.second, desc_key_pair.second);
        }
        for (auto desc_key_pair : wss.m_descriptor_crypt_keys) {
            auto spk_man = pwallet->GetScriptPubKeyMan(desc_key_pair.first.first);
            ((DescriptorScriptPubKeyMan*)spk_man)->AddCryptedKey(desc_key_pair.first.second, desc_key_pair.second.first, desc_key_pair.second.second);
        }

        if (rescan_required && result == DBErrors::LOAD_OK) {
            result = DBErrors::NEED_RESCAN;
        } else if (fNoncriticalErrors && result == DBErrors::LOAD_OK) {
            result = DBErrors::NONCRITICAL_ERROR;
        }

        // Any wallet corruption at all: skip any rewriting or
        // upgrading, we don't want to make it worse.
        if (result != DBErrors::LOAD_OK)
            return result;

        // Last client version to open this wallet, was previously the file version number
        int last_client = CLIENT_VERSION;
        m_batch->Read(DBKeys::VERSION, last_client);

        int wallet_version = pwallet->GetVersion();
        pwallet->WalletLogPrintf("Wallet File Version = %d\n", wallet_version > 0 ? wallet_version : last_client);

        pwallet->WalletLogPrintf("Keys: %u plaintext, %u encrypted, %u w/ metadata, %u total. Unknown wallet records: %u\n",
               wss.nKeys, wss.nCKeys, wss.nKeyMeta, wss.nKeys + wss.nCKeys, wss.m_unknown_records);

        // nTimeFirstKey is only reliable if all keys have metadata
        if (pwallet->IsLegacy() && (wss.nKeys + wss.nCKeys + wss.nWatchKeys) != wss.nKeyMeta) {
            auto spk_man = pwallet->GetOrCreateLegacyScriptPubKeyMan();
            if (spk_man) {
                LOCK(spk_man->cs_KeyStore);
                spk_man->UpdateTimeFirstKey(1);
            }
        }

        for (const uint256& hash : wss.vWalletUpgrade)
            WriteTx(pwallet->mapWallet.at(hash));

        // Rewrite encrypted wallets of versions 0.4.0 and 0.5.0rc:
        if (wss.fIsEncrypted && (last_client == 40000 || last_client == 50000))
            return DBErrors::NEED_REWRITE;

        if (last_client < CLIENT_VERSION) // Update
            m_batch->Write(DBKeys::VERSION, CLIENT_VERSION);

        if (wss.fAnyUnordered)
            result = pwallet->ReorderTransactions();

        // Upgrade all of the wallet keymetadata to have the hd master key id
        // This operation is not atomic, but if it fails, updated entries are still backwards compatible with older software
        try {
            pwallet->UpgradeKeyMetadata();
        } catch (...) {
            result = DBErrors::CORRUPT;
        }

        // Upgrade all of the descriptor caches to cache the last hardened xpub
        // This operation is not atomic, but if it fails, only new entries are added so it is backwards compatible
        try {
            pwallet->UpgradeDescriptorCache();
        } catch (...) {
            result = DBErrors::CORRUPT;
        }

        // Set the inactive chain
        if (wss.m_hd_chains.size() > 0) {
            LegacyScriptPubKeyMan* legacy_spkm = pwallet->GetLegacyScriptPubKeyMan();
            if (!legacy_spkm) {
                pwallet->WalletLogPrintf("Inactive HD Chains found but no Legacy ScriptPubKeyMan\n");
                return DBErrors::CORRUPT;
            }
            for (const auto& chain_pair : wss.m_hd_chains) {
                if (chain_pair.first != pwallet->GetLegacyScriptPubKeyMan()->GetHDChain().seed_id) {
                    pwallet->GetLegacyScriptPubKeyMan()->AddInactiveHDChain(chain_pair.second);
                }
            }
        }

        return result;
        */
    }
    
    pub fn find_wallet_tx(&mut self, 
        tx_hash: &mut Vec<u256>,
        wtx:     &mut LinkedList<WalletTx>) -> DBErrors {
        
        todo!();
        /*
            DBErrors result = DBErrors::LOAD_OK;

        try {
            int nMinVersion = 0;
            if (m_batch->Read(DBKeys::MINVERSION, nMinVersion)) {
                if (nMinVersion > FEATURE_LATEST)
                    return DBErrors::TOO_NEW;
            }

            // Get cursor
            if (!m_batch->StartCursor())
            {
                LogPrintf("Error getting wallet database cursor\n");
                return DBErrors::CORRUPT;
            }

            while (true)
            {
                // Read next record
                DataStream ssKey(SER_DISK, CLIENT_VERSION);
                DataStream ssValue(SER_DISK, CLIENT_VERSION);
                bool complete;
                bool ret = m_batch->ReadAtCursor(ssKey, ssValue, complete);
                if (complete) {
                    break;
                } else if (!ret) {
                    m_batch->CloseCursor();
                    LogPrintf("Error reading next record from wallet database\n");
                    return DBErrors::CORRUPT;
                }

                std::string strType;
                ssKey >> strType;
                if (strType == DBKeys::TX) {
                    uint256 hash;
                    ssKey >> hash;
                    vTxHash.push_back(hash);
                    vWtx.emplace_back(nullptr /* tx */);
                    ssValue >> vWtx.back();
                }
            }
        } catch (...) {
            result = DBErrors::CORRUPT;
        }
        m_batch->CloseCursor();

        return result;
        */
    }
    
    pub fn zap_select_tx(&mut self, 
        tx_hash_in:  &mut Vec<u256>,
        tx_hash_out: &mut Vec<u256>) -> DBErrors {
        
        todo!();
        /*
            // build list of wallet TXs and hashes
        std::vector<uint256> vTxHash;
        std::list<CWalletTx> vWtx;
        DBErrors err = FindWalletTx(vTxHash, vWtx);
        if (err != DBErrors::LOAD_OK) {
            return err;
        }

        std::sort(vTxHash.begin(), vTxHash.end());
        std::sort(vTxHashIn.begin(), vTxHashIn.end());

        // erase each matching wallet TX
        bool delerror = false;
        std::vector<uint256>::iterator it = vTxHashIn.begin();
        for (const uint256& hash : vTxHash) {
            while (it < vTxHashIn.end() && (*it) < hash) {
                it++;
            }
            if (it == vTxHashIn.end()) {
                break;
            }
            else if ((*it) == hash) {
                if(!EraseTx(hash)) {
                    LogPrint(BCLog::WALLETDB, "Transaction was found for deletion but returned database error: %s\n", hash.GetHex());
                    delerror = true;
                }
                vTxHashOut.push_back(hash);
            }
        }

        if (delerror) {
            return DBErrors::CORRUPT;
        }
        return DBErrors::LOAD_OK;
        */
    }
    
    /**
      | Write destination data key,value tuple
      | to database
      |
      */
    pub fn write_dest_data(&mut self, 
        address: &String,
        key:     &String,
        value:   &String) -> bool {
        
        todo!();
        /*
            return WriteIC(std::make_pair(DBKeys::DESTDATA, std::make_pair(address, key)), value);
        */
    }
    
    /**
      | Erase destination data tuple from wallet
      | database
      |
      */
    pub fn erase_dest_data(&mut self, 
        address: &String,
        key:     &String) -> bool {
        
        todo!();
        /*
            return EraseIC(std::make_pair(DBKeys::DESTDATA, std::make_pair(address, key)));
        */
    }
    
    /**
      | write the hdchain model (external chain
      | child index counter)
      |
      */
    pub fn write_hd_chain(&mut self, chain: &HDChain) -> bool {
        
        todo!();
        /*
            return WriteIC(DBKeys::HDCHAIN, chain);
        */
    }
    
    pub fn write_wallet_flags(&mut self, flags: u64) -> bool {
        
        todo!();
        /*
            return WriteIC(DBKeys::FLAGS, flags);
        */
    }
    
    /**
      | Begin a new transaction
      |
      */
    pub fn txn_begin(&mut self) -> bool {
        
        todo!();
        /*
            return m_batch->TxnBegin();
        */
    }
    
    /**
      | Commit current transaction
      |
      */
    pub fn txn_commit(&mut self) -> bool {
        
        todo!();
        /*
            return m_batch->TxnCommit();
        */
    }
    
    /**
      | Abort current transaction
      |
      */
    pub fn txn_abort(&mut self) -> bool {
        
        todo!();
        /*
            return m_batch->TxnAbort();
        */
    }
}

/**
  | Coin Control Features.
  |
  */
pub struct CoinControl {

    /**
      | Custom change destination, if not set
      | an address is generated
      |
      */
    dest_change:           TxDestination, // default = CNoDestination

    /**
      | Override the default change type if
      | set, ignored if destChange is set
      |
      */
    change_type:           Option<OutputType>,

    /**
      | If false, only selected inputs are used
      |
      */
    add_inputs:            bool, // default = true

    /**
      | If false, only safe inputs will be used
      |
      */
    include_unsafe_inputs: bool, // default = false

    /**
      | If false, allows unselected inputs,
      | but requires all selected inputs be
      | used
      |
      */
    allow_other_inputs:    bool, // default = false

    /**
      | Includes watch only addresses which
      | are solvable
      |
      */
    allow_watch_only:      bool, // default = false

    /**
      | Override automatic min/max checks
      | on fee, m_feerate must be set if true
      |
      */
    override_fee_rate:     bool, // default = false

    /**
      | Override the wallet's m_pay_tx_fee
      | if set
      |
      */
    feerate:               Option<FeeRate>,

    /**
      | Override the default confirmation
      | target if set
      |
      */
    confirm_target:        Option<u32>,

    /**
      Override the wallet's m_signal_rbf if set
      */
    signal_bip125_rbf:     Option<bool>,

    /**
      | Avoid partial use of funds sent to a given
      | address
      |
      */
    avoid_partial_spends:  bool, // default = DEFAULT_AVOIDPARTIALSPENDS

    /**
      | Forbids inclusion of dirty (previously
      | used) addresses
      |
      */
    avoid_address_reuse:   bool, // default = false

    /**
      | Fee estimation mode to control arguments
      | to estimateSmartFee
      |
      */
    fee_mode:              FeeEstimateMode, // default = FeeEstimateMode_UNSET

    /**
      | Minimum chain depth value for coin availability
      |
      */
    min_depth:             i32, // default = DEFAULT_MIN_DEPTH

    /**
      | Maximum chain depth value for coin availability
      |
      */
    max_depth:             i32, // default = DEFAULT_MAX_DEPTH

    /**
      | SigningProvider that has pubkeys and
      | scripts to do spend size estimation
      | for external inputs
      |
      */
    external_provider:     FlatSigningProvider,

    set_selected:          HashSet<OutPoint>,
    external_txouts:       HashMap<OutPoint,TxOut>,
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/coincontrol.cpp]
impl CoinControl {
    
    pub fn has_selected(&self) -> bool {
        
        todo!();
        /*
            return (setSelected.size() > 0);
        */
    }
    
    pub fn is_selected(&self, output: &OutPoint) -> bool {
        
        todo!();
        /*
            return (setSelected.count(output) > 0);
        */
    }
    
    pub fn is_external_selected(&self, output: &OutPoint) -> bool {
        
        todo!();
        /*
            return (m_external_txouts.count(output) > 0);
        */
    }
    
    pub fn get_external_output(&self, 
        outpoint: &OutPoint,
        txout:    &mut TxOut) -> bool {
        
        todo!();
        /*
            const auto ext_it = m_external_txouts.find(outpoint);
            if (ext_it == m_external_txouts.end()) {
                return false;
            }
            txout = ext_it->second;
            return true;
        */
    }
    
    pub fn select(&mut self, output: &OutPoint)  {
        
        todo!();
        /*
            setSelected.insert(output);
        */
    }
    
    pub fn select_external(&mut self, 
        outpoint: &OutPoint,
        txout:    &TxOut)  {
        
        todo!();
        /*
            setSelected.insert(outpoint);
            m_external_txouts.emplace(outpoint, txout);
        */
    }
    
    pub fn un_select(&mut self, output: &OutPoint)  {
        
        todo!();
        /*
            setSelected.erase(output);
        */
    }
    
    pub fn un_select_all(&mut self)  {
        
        todo!();
        /*
            setSelected.clear();
        */
    }
    
    pub fn list_selected(&self, outpoints: &mut Vec<OutPoint>)  {
        
        todo!();
        /*
            vOutpoints.assign(setSelected.begin(), setSelected.end());
        */
    }
    
    pub fn new() -> Self {
    
        todo!();
        /*


            m_avoid_partial_spends = gArgs.GetBoolArg("-avoidpartialspends", DEFAULT_AVOIDPARTIALSPENDS);
        */
    }
}

/**
  | Collection of wallet balances.
  |
  */
#[Q_METATYPE]
pub struct WalletBalances {
    balance:                        Amount, // default = 0
    unconfirmed_balance:            Amount, // default = 0
    immature_balance:               Amount, // default = 0
    have_watch_only:                bool, // default = false
    watch_only_balance:             Amount, // default = 0
    unconfirmed_watch_only_balance: Amount, // default = 0
    immature_watch_only_balance:    Amount, // default = 0
}

impl WalletBalances {
    
    pub fn balance_changed(&self, prev: &WalletBalances) -> bool {
        
        todo!();
        /*
            return balance != prev.balance || unconfirmed_balance != prev.unconfirmed_balance ||
                   immature_balance != prev.immature_balance || watch_only_balance != prev.watch_only_balance ||
                   unconfirmed_watch_only_balance != prev.unconfirmed_watch_only_balance ||
                   immature_watch_only_balance != prev.immature_watch_only_balance;
        */
    }
}

/**
  | RAII class that provides access to a
  | WalletDatabase
  |
  */
pub struct DatabaseBatch {

}

pub trait DatabaseBatchInterface:
    ReadKey
    + WriteKey
    + EraseKey
    + HasKey
    + Flush
    + Close
    + StartCursor
    + ReadAtCursor
    + CloseCursor
    + TxnBegin
    + TxnCommit
    + TxnAbort
    {}

pub trait ReadKey {

    fn read_key(&mut self, 
        key:   DataStream,
        value: &mut DataStream) -> bool;
}

pub trait WriteKey {

    fn write_key(&mut self, 
        key:       DataStream,
        value:     DataStream,
        overwrite: bool) -> bool;
}


pub trait ReadAtCursor {

    fn read_at_cursor(&mut self, 
        ss_key:   &mut DataStream,
        ss_value: &mut DataStream,
        complete: &mut bool) -> bool;
}

pub trait EraseKey { 

    fn erase_key(&mut self, key: DataStream) -> bool; 
}

pub trait HasKey { 

    fn has_key(&mut self, key: DataStream) -> bool; 
}

pub trait StartCursor { 

    fn start_cursor(&mut self) -> bool; 
}

pub trait CloseCursor { 

    fn close_cursor(&mut self); 
}

pub trait TxnBegin { 

    fn txn_begin(&mut self) -> bool; 
}

pub trait TxnCommit { 

    fn txn_commit(&mut self) -> bool; 
}

pub trait TxnAbort { 

    fn txn_abort(&mut self) -> bool; 
}

pub trait Open {

    /**
      | Open the database if it is not already
      | opened.
      |
      */
    fn open(&mut self);

}

pub trait AddRef {

    /**
      | Indicate the a new database user has
      | began using the database. Increments
      | m_refcount
      |
      */
    fn add_ref(&mut self);

}

pub trait RemoveRef {

    /**
      | Indicate that database user has stopped
      | using the database and that it could
      | be flushed or closed. Decrement m_refcount
      |
      */
    fn remove_ref(&mut self);

}

pub trait Rewrite {

    /**
      | Rewrite the entire database on disk,
      | with the exception of key pszSkip if
      | non-zero
      |
      */
    fn rewrite(&mut self, psz_skip: *const u8) -> bool;

}

pub trait Backup {

    /**
      | Back up the entire database to a file.
      |
      */
    fn backup(&self, str_dest: &String) -> bool;

}

pub trait Close {

    /**
      | Flush to the database file and close
      | the database.
      | 
      | Also close the environment if no other
      | databases are open in it.
      |
      */
    fn close(&mut self);
}

pub trait PeriodicFlush {

    /**
      | flush the wallet passively (TRY_LOCK)
      | ideal to be called periodically
      |
      */
    fn periodic_flush(&mut self) -> bool;

}

pub trait IncrementUpdateCounter {

    fn increment_update_counter(&mut self);
}

pub trait ReloadDbEnv {

    fn reload_db_env(&mut self);
}

pub trait Filename {

    /**
      | Return path to main database file for
      | logs and error messages.
      |
      */
    fn filename(&mut self) -> String;

}

pub trait Format {

    fn format(&mut self) -> String;
}

pub trait MakeBatch {

    /**
      | Make a DatabaseBatch connected to this
      | database
      |
      */
    fn make_batch(&mut self, flush_on_close: bool) -> Box<DatabaseBatch>;

}



impl DatabaseBatch {

    pub fn read<K, T>(&mut self, 
        key:   &K,
        value: &mut T) -> bool {
    
        todo!();
        /*
            DataStream ssKey(SER_DISK, CLIENT_VERSION);
            ssKey.reserve(1000);
            ssKey << key;

            DataStream ssValue(SER_DISK, CLIENT_VERSION);
            if (!ReadKey(std::move(ssKey), ssValue)) return false;
            try {
                ssValue >> value;
                return true;
            } catch (const std::exception&) {
                return false;
            }
        */
    }
    
    
    pub fn write<K, T>(&mut self, 
        key:       &K,
        value:     &T,
        overwrite: Option<bool>) -> bool {
        let overwrite: bool = overwrite.unwrap_or(true);
        todo!();
        /*
            DataStream ssKey(SER_DISK, CLIENT_VERSION);
            ssKey.reserve(1000);
            ssKey << key;

            DataStream ssValue(SER_DISK, CLIENT_VERSION);
            ssValue.reserve(10000);
            ssValue << value;

            return WriteKey(std::move(ssKey), std::move(ssValue), fOverwrite);
        */
    }
    
    
    pub fn erase<K>(&mut self, key: &K) -> bool {
    
        todo!();
        /*
            DataStream ssKey(SER_DISK, CLIENT_VERSION);
            ssKey.reserve(1000);
            ssKey << key;

            return EraseKey(std::move(ssKey));
        */
    }
    
    
    pub fn exists<K>(&mut self, key: &K) -> bool {
    
        todo!();
        /*
            DataStream ssKey(SER_DISK, CLIENT_VERSION);
            ssKey.reserve(1000);
            ssKey << key;

            return HasKey(std::move(ssKey));
        */
    }
}

/**
  | An instance of this class represents
  | one database.
  |
  */
pub struct WalletDatabase {

    /**
      | Counts the number of active database
      | users to be sure that the database is
      | not closed while someone is using it
      |
      */
    refcount:             Atomic<i32>, // default = { 0 }

    n_update_counter:     Atomic<u32>,
    n_last_seen:          u32,
    n_last_flushed:       u32,
    n_last_wallet_update: i64,
}

pub trait WalletDatabaseInterface:
    Open
    + AddRef
    + RemoveRef
    + Rewrite
    + Backup
    + Flush
    + Close
    + PeriodicFlush
    + IncrementUpdateCounter
    + ReloadDbEnv
    + Filename
    + Format
    + MakeBatch {}

impl Default for WalletDatabase {
    
    /**
      | Create dummy DB handle
      |
      */
    fn default() -> Self {
        todo!();
        /*
        : n_update_counter(0),
        : n_last_seen(0),
        : n_last_flushed(0),
        : n_last_wallet_update(0),

        
        */
    }
}

/**
  | Wallet storage things that ScriptPubKeyMans
  | need in order to be able to store things to the
  | wallet database.  It provides access to things
  | that are part of the entire wallet and not
  | specific to a ScriptPubKeyMan such as wallet
  | flags, wallet version, encryption keys,
  | encryption status, and the database
  | itself. This allows a ScriptPubKeyMan to have
  | callbacks into CWallet without causing
  | a circular dependency.  WalletStorage should be
  | the same for all ScriptPubKeyMans of a wallet.
  */
pub trait WalletStorage:
GetDisplayName
+ GetDatabase
+ IsWalletFlagSet
+ UnsetBlankWalletFlag
+ CanSupportFeature
+ SetMinVersion
+ GetEncryptionKey
+ HasEncryptionKeys
+ IsLocked { }

pub trait GetDisplayName {

    fn get_display_name(&self) -> String;
}

pub trait GetDatabase {

    fn get_database(&self) -> &mut WalletDatabase;
}

pub trait IsWalletFlagSet {

    fn is_wallet_flag_set(&self, _0: u64) -> bool;
}

pub trait UnsetBlankWalletFlag {

    fn unset_blank_wallet_flag(&mut self, _0: &mut WalletBatch);
}

pub trait CanSupportFeature {

    fn can_support_feature(&self, _0: WalletFeature) -> bool;
}

pub trait SetMinVersion {

    fn set_min_version(&mut self, 
            _0: WalletFeature,
            _1: Option<*mut WalletBatch>);
}

pub trait GetEncryptionKey {

    fn get_encryption_key(&self) -> &KeyingMaterial;
}

pub trait HasEncryptionKeys {

    fn has_encryption_keys(&self) -> bool;
}

///-----------------------------
pub trait GetNewDestination {

    fn get_new_destination(&mut self, 
        ty:    OutputType,
        dest:  &mut TxDestination,
        error: &mut BilingualStr) -> bool;
}

pub trait IsMine {

    fn is_mine(&self, script: &Script) -> IsMineType;
}

pub trait CheckDecryptionKey {

    fn check_decryption_key(&mut self, 
        master_key:     &KeyingMaterial,
        accept_no_keys: Option<bool>) -> bool;
}

pub trait Encrypt {

    fn encrypt(&mut self, 
        master_key: &KeyingMaterial,
        batch:      *mut WalletBatch) -> bool;
}

pub trait GetReservedDestination {

    fn get_reserved_destination(&mut self, 
        ty:       OutputType,
        internal: bool,
        address:  &mut TxDestination,
        index:    &mut i64,
        keypool:  &mut KeyPool,
        error:    &mut BilingualStr) -> bool;
}

pub trait KeepDestination {

    fn keep_destination(&mut self, 
        index: i64,
        ty:    &OutputType);
}

pub trait ReturnDestination {

    fn return_destination(&mut self, 
        index:    i64,
        internal: bool,
        addr:     &TxDestination);
}

pub trait TopUp {

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
    fn top_up(&mut self, size: Option<u32>) -> bool;
}

pub trait MarkUnusedAddresses {

    /**
      | Mark unused addresses as being used
      |
      */
    fn mark_unused_addresses(&mut self, script: &Script);
}

pub trait SetupGeneration {

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
    fn setup_generation(&mut self, force: Option<bool>) -> bool;
}

pub trait IsHDEnabled {

    /**
      | Returns true if HD is enabled
      |
      */
    fn is_hd_enabled(&self) -> bool;
}

pub trait Upgrade {

    /**
      | Upgrades the wallet to the specified
      | version
      |
      */
    fn upgrade(&mut self, 
        prev_version: i32,
        new_version:  i32,
        error:        &mut BilingualStr) -> bool;
}

pub trait HavePrivateKeys {

    fn have_private_keys(&self) -> bool;
}

pub trait RewriteDB {

    /**
      | The action to do when the DB needs rewrite
      |
      */
    fn rewritedb(&mut self);
}

pub trait GetOldestKeyPoolTime {

    fn get_oldest_key_pool_time(&self) -> i64;
}

pub trait GetKeyPoolSize {

    fn get_key_pool_size(&self) -> u32;
}

pub trait GetTimeFirstKey {

    fn get_time_first_key(&self) -> i64;
}

pub trait GetMetadata {

    fn get_metadata(&self, 
        dest: &TxDestination) -> Box<KeyMetadata>;
}

pub trait GetSolvingProvider {

    fn get_solving_provider(&self, 
        script: &Script) -> Box<SigningProvider>;
}

pub trait CanProvide {

    /**
      | Whether this ScriptPubKeyMan can provide
      | a SigningProvider (via GetSolvingProvider)
      | that, combined with sigdata, can produce
      | solving data.
      |
      */
    fn can_provide(&mut self, 
        script:  &Script,
        sigdata: &mut SignatureData) -> bool;
}

pub trait SignTransaction {

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
        input_errors: &mut HashMap<i32,BilingualStr>) -> bool;
}

pub trait FillPSBT {

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
        n_signed:     Option<*mut i32>) -> TransactionError;
}

pub trait GetID {

    fn getid(&self) -> u256;
}


pub trait AddToSpendsWithOutpoint {

    fn add_to_spends_with_outpoint(&mut self, 
        outpoint: &OutPoint,
        wtxid:    &u256,
        batch:    Option<*mut WalletBatch>);
}

pub trait AddToSpends {

    fn add_to_spends(&mut self, 
        wtxid: &u256,
        batch: Option<*mut WalletBatch>);
}

pub trait AddToWalletIfInvolvingMe {

    fn add_to_wallet_if_involving_me(&mut self, 
        tx:                   &TransactionRef,
        confirm:              WalletTxConfirmation,
        update:               bool,
        rescanning_old_block: bool) -> bool;
}

pub trait MarkConflicted {

    fn mark_conflicted(&mut self, 
        hash_block:         &u256,
        conflicting_height: i32,
        hash_tx:            &u256);
}

pub trait MarkInputsDirty {

    fn mark_inputs_dirty(&mut self, tx: &TransactionRef);
}

pub trait SyncMetaData {

    fn sync_meta_data(&mut self, _0: (WalletTxSpendsIterator,WalletTxSpendsIterator));
}

pub trait SyncTransaction {

    fn sync_transaction(&mut self, 
        tx:                   &TransactionRef,
        confirm:              WalletTxConfirmation,
        update_tx:            Option<bool>,
        rescanning_old_block: Option<bool>);
}
    
pub trait SetAddressBookWithDB {

    fn set_address_book_withdb(&mut self, 
        batch:       &mut WalletBatch,
        address:     &TxDestination,
        str_name:    &str,
        str_purpose: &str) -> bool;
}

pub trait UnsetWalletFlagWithDB {

    fn unset_wallet_flag_withdb(&mut self, 
        batch: &mut WalletBatch,
        flag:  u64);
}

pub trait HaveChain {

    fn have_chain(&self) -> bool;
}

pub trait GetTxConflicts {

    fn get_tx_conflicts(&self, wtx: &WalletTx) -> HashSet<u256>;
}

pub trait GetTxDepthInMainChain {

    fn get_tx_depth_in_main_chain(&self, wtx: &WalletTx) -> i32;
}
    
pub trait IsTxInMainChain {
    fn is_tx_in_main_chain(&self, wtx: &WalletTx) -> bool;
}

pub trait GetTxBlocksToMaturity {

    fn get_tx_blocks_to_maturity(&self, wtx: &WalletTx) -> i32;
}

pub trait IsTxImmatureCoinBase {

    fn is_tx_immature_coin_base(&self, wtx: &WalletTx) -> bool;
}

pub trait IsSpent {

    fn is_spent(&self, 
        hash: &u256,
        n:    u32) -> bool;
}

pub trait IsSpentKey {

    fn is_spent_key(&self, 
        hash: &u256,
        n:    u32) -> bool;
}

pub trait SetSpentKeyState {

    fn set_spent_key_state(&mut self, 
        batch:           &mut WalletBatch,
        hash:            &u256,
        n:               u32,
        used:            bool,
        tx_destinations: &mut HashSet<TxDestination>);
}

pub trait UnlockAllCoins {

    fn unlock_all_coins(&mut self) -> bool;
}

/* ------------- Rescan abort properties  ------------- */
    
pub trait IsAbortingRescan {

    fn is_aborting_rescan(&self) -> bool;
}

pub trait IsScanning {

    fn is_scanning(&self) -> bool;
}
    
pub trait ScanningDuration {

    fn scanning_duration(&self) -> i64;
}

pub trait ScanningProgress {

    fn scanning_progress(&self) -> f64;
}

pub trait UpgradeKeyMetadata {

    fn upgrade_key_metadata(&mut self);
}

pub trait UpgradeDescriptorCache {

    fn upgrade_descriptor_cache(&mut self);
}

pub trait LoadMinVersion {

    fn load_min_version(&mut self, n_version: i32) -> bool;
}

pub trait LoadDestData {

    fn load_dest_data(&mut self, 
        dest:  &TxDestination,
        key:   &String,
        value: &String);
}

pub trait GetKeyBirthTimes {

    fn get_key_birth_times(&self, map_key_birth: &mut HashMap<KeyID,i64>);
}
    
pub trait ComputeTimeSmart {

    fn compute_time_smart(&self, 
        wtx:                  &WalletTx,
        rescanning_old_block: bool) -> u32;
}

pub trait IncOrderPosNext {

    fn inc_order_pos_next(&mut self, 
        batch: Option<*mut WalletBatch>) -> i64;
}
    
pub trait ReorderTransactions {

    fn reorder_transactions(&mut self) -> DBErrors;
}
    
pub trait MarkDirty {

    fn mark_dirty(&mut self);
}
    
pub trait AddToWallet {

    fn add_to_wallet(&mut self, 
        tx:                   TransactionRef,
        confirm:              &WalletTxConfirmation,
        update_wtx:           Option<&WalletUpdateWalletTxFn>,
        flush_on_close:       Option<bool>,
        rescanning_old_block: Option<bool>) -> *mut WalletTx;
}

pub trait LoadToWallet {

    fn load_to_wallet(&mut self, 
        hash:     &u256,
        fill_wtx: &WalletUpdateWalletTxFn) -> bool;
}
    
pub trait RescanFromTime {

    fn rescan_from_time(&mut self, 
        start_time: i64,
        reserver:   &WalletRescanReserver,
        update:     bool) -> i64;
}
    
pub trait ScanForWalletTransactions {

    fn scan_for_wallet_transactions(&mut self, 
        start_block:  &u256,
        start_height: i32,
        max_height:   Option<i32>,
        reserver:     &WalletRescanReserver,
        update:       bool) -> WalletScanResult;
}

pub trait ReacceptWalletTransactions {

    fn reaccept_wallet_transactions(&mut self);
}

pub trait ResendWalletTransactions {

    fn resend_wallet_transactions(&mut self);
}
    
pub trait TransactionChangeType {

    fn transaction_change_type(&self, 
        change_type: &Option<OutputType>,
        vec_send:    &Vec<Recipient>) -> OutputType;
}

pub trait WalletSignTransaction {

    fn sign_transaction(&self, tx: &mut MutableTransaction) -> bool;
}

pub trait SignTransactionGivenInputCoinsAndSighash {

    fn sign_transaction_given_input_coins_and_sighash(&self, 
        tx:           &mut MutableTransaction,
        coins:        &HashMap<OutPoint,Coin>,
        sighash:      i32,
        input_errors: &mut HashMap<i32,BilingualStr>) -> bool;
}

pub trait SubmitTxMemoryPoolAndRelay {

    fn submit_tx_memory_pool_and_relay(&self, 
        wtx:        &WalletTx,
        err_string: &mut String,
        relay:      bool) -> bool;
}

pub trait DummySignTx {

    fn dummy_sign_tx(&self, 
        tx_new:       &mut MutableTransaction,
        txouts:       &HashSet<TxOut>,
        coin_control: Option<*const CoinControl>) -> bool;
}
    
pub trait ImportScripts {

    fn import_scripts(&mut self, 
        scripts:   HashSet<Script>,
        timestamp: i64) -> bool;
}


pub trait ImportPrivKeys {

    fn import_priv_keys(&mut self, 
        privkey_map: &HashMap<KeyID,Key>,
        timestamp:   i64) -> bool;
}


pub trait ImportPubKeys {

    fn import_pub_keys(&mut self, 
        ordered_pubkeys: &Vec<KeyID>,
        pubkey_map:      &HashMap<KeyID,PubKey>,
        key_origins:     &HashMap<KeyID,(PubKey,KeyOriginInfo)>,
        add_keypool:     bool,
        internal:        bool,
        timestamp:       i64) -> bool;
}


pub trait ImportScriptPubKeys {

    fn import_script_pub_keys(&mut self, 
        label:             &String,
        script_pub_keys:   &HashSet<Script>,
        have_solving_data: bool,
        apply_label:       bool,
        timestamp:         i64) -> bool;
}


pub trait KeypoolCountExternalKeys {

    fn keypool_count_external_keys(&self) -> usize;
}

    
pub trait TopUpKeyPool {

    fn top_up_key_pool(&mut self, kp_size: Option<u32>) -> bool;
}

    

pub trait GetLabelAddresses {

    fn get_label_addresses(&self, label: &String) -> HashSet<TxDestination>;
}


pub trait MarkDestinationsDirty {

    fn mark_destinations_dirty(&mut self, destinations: &HashSet<TxDestination>);
}


    
pub trait GetNewChangeDestination {
    fn get_new_change_destination(&mut self, 
        ty:    OutputType,
        dest:  &mut TxDestination,
        error: &mut BilingualStr) -> bool;
}


pub trait IsMineWithTxDest {

    fn is_mine_with_tx_dest(&self, dest: &TxDestination) -> IsMineType;
}


pub trait IsMineWithScript {

    fn is_mine_with_script(&self, script: &Script) -> IsMineType;
}


pub trait GetDebitWithTxinAndFilter {

    fn get_debit_with_txin_and_filter(&self, 
        txin:   &TxIn,
        filter: &IsMineFilter) -> Amount;
}


pub trait IsMineWithTxout {

    fn is_mine_with_txout(&self, txout: &TxOut) -> IsMineType;
}


pub trait IsMineWithTx {

    fn is_mine_with_tx(&self, tx: &Transaction) -> bool;
}


pub trait IsFromMe {
    fn is_from_me(&self, tx: &Transaction) -> bool;
}

    
pub trait GetDebitWithTxAndFilter {
    fn get_debit_with_tx_and_filter(&self, 
        tx:     &Transaction,
        filter: &IsMineFilter) -> Amount;
}

    
    
pub trait LoadWallet {

    fn load_wallet(&mut self) -> DBErrors;
}


pub trait ZapSelectTx {
    fn zap_select_tx(&mut self, 
        hash_in:  &mut Vec<u256>,
        hash_out: &mut Vec<u256>) -> DBErrors;
}

pub trait IsAddressUsed {

    fn is_address_used(&self, dest: &TxDestination) -> bool;
}


pub trait SetAddressUsed {

    fn set_address_used(&mut self, 
        batch: &mut WalletBatch,
        dest:  &TxDestination,
        used:  bool) -> bool;
}



pub trait GetVersion {

    fn get_version(&self) -> i32;
}


pub trait GetConflicts {

    fn get_conflicts(&self, txid: &u256) -> HashSet<u256>;
}


pub trait HasWalletSpend {

    fn has_wallet_spend(&self, txid: &u256) -> bool;
}




pub trait GetBroadcastTransactions {

    fn get_broadcast_transactions(&self) -> bool;
}


pub trait SetBroadcastTransactions {

    fn set_broadcast_transactions(&mut self, broadcast: bool);
}




pub trait MarkReplaced {

    fn mark_replaced(&mut self, 
        original_hash: &u256,
        new_hash:      &u256) -> bool;
}


pub trait PostInitProcess {

    fn post_init_process(&mut self);
}

pub trait BlockUntilSyncedToCurrentChain {

    fn block_until_synced_to_current_chain(&self);
}

pub trait SetWalletFlag {

    fn set_wallet_flag(&mut self, flags: u64);
}

pub trait UnsetWalletFlag {

    fn unset_wallet_flag(&mut self, flag: u64);
}

pub trait AddWalletFlags {

    fn add_wallet_flags(&mut self, flags: u64) -> bool;
}

pub trait LoadWalletFlags {

    fn load_wallet_flags(&mut self, flags: u64) -> bool;
}

pub trait WalletLogPrintf {

    fn wallet_log_printf<Params>(&self, 
        fmt:        String,
        parameters: Params);
}

pub trait UpgradeWallet {

    fn upgrade_wallet(&mut self, 
        version: i32,
        error:   &mut BilingualStr) -> bool;
}

    
pub trait GetSolvingProviderWithSigdata {

    fn get_solving_provider_with_sigdata(&self, 
        script:  &Script,
        sigdata: &mut SignatureData) -> Box<SigningProvider>;
}


    

pub trait GetLastBlockHeight {
    fn get_last_block_height(&self) -> i32;
}


pub trait GetLastBlockHash {

    fn get_last_block_hash(&self) -> u256;
}


pub trait SetLastBlockProcessed {

    fn set_last_block_processed(&mut self, 
        block_height: i32,
        block_hash:   u256);
}

//-------------------------------------------[.cpp/bitcoin/src/interfaces/wallet.h]

pub trait SetAddressReceiveRequestWithBatch {

    /**
      | Save or remove receive request.
      |
      */
    fn set_address_receive_request_with_batch(
        &mut self, 
        batch: &mut WalletBatch,
        dest:  &TxDestination,
        id:    &String,
        value: &String) -> bool;
}

pub trait UnlockCoinWithBatch {

    fn unlock_coin_with_batch(&mut self, 
        output: &OutPoint,
        batch:  Option<*mut WalletBatch>) -> bool;
}

pub trait CheckIsLockedCoinWithHash {

    fn check_is_locked_coin_with_hash(&self, 
        hash: u256,
        n:    u32) -> bool;
}

pub trait GetWalletTxPtr {

    fn get_wallet_tx_ptr(&self, hash: &u256) -> *const WalletTx;
}

pub trait WalletCanGetAddresses {

    /**
      | Return whether the wallet is blank.
      |
      */
    fn can_get_addresses(&mut self) -> bool;
}

pub type WalletTxSpends         = MultiMap<OutPoint,u256>;
pub type WalletTxSpendsIterator = (Rc<OutPoint>,Rc<u256>);
pub type WalletMasterKeyMap     = HashMap<u32,MasterKey>;
pub type WalletTxItems          = MultiMap<i64,*mut WalletTx>;

/**
  | Callback for updating transaction
  | metadata in mapWallet.
  | 
  | -----------
  | @param wtx
  | 
  | - reference to mapWallet transaction
  | to update
  | ----------
  | @param new_tx
  | 
  | - true if wtx is newly inserted, false
  | if it previously existed
  | 
  | -----------
  | @return
  | 
  | true if wtx is changed and needs to be
  | saved to disk, otherwise false
  |
  */
pub type WalletUpdateWalletTxFn = fn(wtx: &mut WalletTx, new_tx: bool) -> bool;

pub enum WalletScanResultStatus { SUCCESS, FAILURE, USER_ABORT }

/**
  | RAII object to check and reserve a wallet
  | rescan
  |
  */
pub struct WalletRescanReserver {
    wallet:        Rc<RefCell<dyn WalletInterface>>,
    could_reserve: bool,
}
    
impl Drop for WalletRescanReserver {

    fn drop(&mut self) {
        todo!();
        /*
            if (m_could_reserve) {
                m_wallet.fScanningWallet = false;
            }
        */
    }
}

impl From<&mut dyn WalletInterface> for WalletRescanReserver {
    
    fn from(w: &mut dyn WalletInterface) -> Self {
    
        todo!();
        /*
        : wallet(w),
        : could_reserve(false),
        */
    }
}

impl WalletRescanReserver {

    pub fn reserve(&mut self) -> bool {
        
        todo!();
        /*
            assert(!m_could_reserve);
            if (m_wallet.fScanningWallet.exchange(true)) {
                return false;
            }
            m_wallet.m_scanning_start = GetTimeMillis();
            m_wallet.m_scanning_progress = 0;
            m_could_reserve = true;
            return true;
        */
    }
    
    pub fn is_reserved(&self) -> bool {
        
        todo!();
        /*
            return (m_could_reserve && m_wallet.fScanningWallet);
        */
    }
}

pub struct WalletScanResult {

    status:              WalletScanResultStatus, // default = SUCCESS

        /**
          | Hash and height of most recent block
          | that was successfully scanned.
          | 
          | Unset if no blocks were scanned due to
          | read errors or the chain being empty.
          |
          */
    last_scanned_block:  u256,

    last_scanned_height: Option<i32>,

    /**
      | Height of the most recent block that
      | could not be scanned due to read errors
      | or pruning. Will be set if status is FAILURE,
      | unset if status is SUCCESS, and may or
      | may not be set if status is
      | 
      | USER_ABORT.
      |
      */
    last_failed_block:   u256,
}

pub type LoadWalletFn = fn(wallet: Box<dyn WalletInterface>) -> ();
