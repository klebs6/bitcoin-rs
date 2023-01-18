crate::ix!();

/**
  | Address book data
  |
  */
pub struct AddressBookData {
    change:   bool, // default = { true }
    label:    String,
    purpose:  String,
    destdata: AddressBookDataStringMap,
}

pub type AddressBookDataStringMap = HashMap<String,String>;

pub trait FindAddressBookEntry {

    fn find_address_book_entry(&self, 
        _0:           &TxDestination,
        allow_change: Option<bool>) -> *const AddressBookData;
}

impl Default for AddressBookData {
    
    fn default() -> Self {
        todo!();
        /*
        : purpose("unknown"),

        
        */
    }
}

impl AddressBookData {
    
    pub fn is_change(&self) -> bool {
        
        todo!();
        /*
            return m_change;
        */
    }
    
    pub fn get_label(&self) -> &String {
        
        todo!();
        /*
            return m_label;
        */
    }
    
    pub fn set_label(&mut self, label: &String)  {
        
        todo!();
        /*
            m_change = false;
            m_label = label;
        */
    }
}


/**
  | -paytxfee default
  |
  */
pub const DEFAULT_PAY_TX_FEE: Amount = 0;

/**
  | -fallbackfee default
  |
  */
pub const DEFAULT_FALLBACK_FEE: Amount = 0;

/**
  | -discardfee default
  |
  */
pub const DEFAULT_DISCARD_FEE: Amount = 10000;

/**
  | -mintxfee default
  |
  */
pub const DEFAULT_TRANSACTION_MINFEE: Amount = 1000;

/**
  | -consolidatefeerate default
  |
  */
pub const DEFAULT_CONSOLIDATE_FEERATE: Amount = 10000; // 10 sat/vbyte

/**
  | maximum fee increase allowed to do partial
  | spend avoidance, even for nodes with
  | this feature disabled by default
  | 
  | A value of -1 disables this feature completely.
  | 
  | A value of 0 (current default) means
  | to attempt to do partial spend avoidance,
  | and use its results if the fees remain
  | *unchanged*
  | 
  | A value > 0 means to do partial spend avoidance
  | if the fee difference against a regular
  | coin selection instance is in the range
  | [0..value].
  |
  */
pub const DEFAULT_MAX_AVOIDPARTIALSPEND_FEE: Amount = 0;

/**
  | discourage APS fee higher than this
  | amount
  |
  */
pub const HIGH_APS_FEE: Amount = {COIN / 10000};

/**
  | minimum recommended increment for
  | BIP 125 replacement txs
  |
  */
pub const WALLET_INCREMENTAL_RELAY_FEE: Amount = 5000;

/**
  | Default for -spendzeroconfchange
  |
  */
pub const DEFAULT_SPEND_ZEROCONF_CHANGE: bool = true;

/**
  | Default for -walletrejectlongchains
  |
  */
pub const DEFAULT_WALLET_REJECT_LONG_CHAINS: bool = false;

/**
  | -txconfirmtarget default
  |
  */
pub const DEFAULT_TX_CONFIRM_TARGET: u32 = 6;

/**
  | -walletrbf default
  |
  */
pub const DEFAULT_WALLET_RBF:      bool = false;
pub const DEFAULT_WALLETBROADCAST: bool = true;
pub const DEFAULT_DISABLE_WALLET:  bool = false;

/**
   -maxtxfee default
  */
pub const DEFAULT_TRANSACTION_MAXFEE: Amount = COIN / 10;

/**
  | Discourage users to set fees higher
  | than this amount (in satoshis) per kB
  |
  */
pub const HIGH_TX_FEE_PER_KB: Amount = COIN / 100;

/**
  | -maxtxfee will warn if called with a
  | higher fee than this amount (in satoshis)
  |
  */
pub const HIGH_MAX_TX_FEE: Amount = 100 * HIGH_TX_FEE_PER_KB;

/**
  | Pre-calculated constants for input
  | size estimation in *virtual size*
  |
  */
pub const DUMMY_NESTED_P2WPKH_INPUT_SIZE: usize = 91;

/**
  | Default for -addresstype
  |
  */
pub const DEFAULT_ADDRESS_TYPE: OutputType = OutputType::BECH32;

/*
pub const KNOWN_WALLET_FLAGS: u64 = WalletFlags::WALLET_FLAG_AVOID_REUSE.bits
    |   WalletFlags::WALLET_FLAG_BLANK_WALLET.bits
    |   WalletFlags::WALLET_FLAG_KEY_ORIGIN_METADATA.bits
    |   WalletFlags::WALLET_FLAG_LAST_HARDENED_XPUB_CACHED.bits
    |   WalletFlags::WALLET_FLAG_DISABLE_PRIVATE_KEYS.bits
    |   WalletFlags::WALLET_FLAG_DESCRIPTORS.bits
    |   WalletFlags::WALLET_FLAG_EXTERNAL_SIGNER.bits;

pub const MUTABLE_WALLET_FLAGS: u64 = WalletFlags::WALLET_FLAG_AVOID_REUSE;

pub const WALLET_FLAG_MAP: HashMap<&'static str,u64> = hashmap!{
    "avoid_reuse"               => WalletFlags::WALLET_FLAG_AVOID_REUSE.bits,
    "blank"                     => WalletFlags::WALLET_FLAG_BLANK_WALLET.bits,
    "key_origin_metadata"       => WalletFlags::WALLET_FLAG_KEY_ORIGIN_METADATA.bits,
    "last_hardened_xpub_cached" => WalletFlags::WALLET_FLAG_LAST_HARDENED_XPUB_CACHED.bits,
    "disable_private_keys"      => WalletFlags::WALLET_FLAG_DISABLE_PRIVATE_KEYS.bits,
    "descriptor_wallet"         => WalletFlags::WALLET_FLAG_DESCRIPTORS.bits,
    "external_signer"           => WalletFlags::WALLET_FLAG_EXTERNAL_SIGNER.bits
};
*/

lazy_static!{
    /*
    extern const std::map<uint64_t,std::string> WALLET_FLAG_CAVEATS;
    */
}

/**
  | Return implementation of Wallet
  | interface. This function is defined in
  | dummywallet.cpp and throws if the wallet
  | component is not compiled.
  */
pub fn make_wallet_with_context(
    context: &mut WalletContext,
    wallet:  &Arc<Wallet>) -> Box<dyn WalletInterface> 
{
    todo!();
        /*
            return wallet ? std::make_unique<WalletImpl>(context, wallet) : nullptr;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/wallet.h]

/**
  | Called periodically by the schedule
  | thread. Prompts individual wallets
  | to resend their transactions. Actual
  | rebroadcast schedule is managed by
  | the wallets themselves.
  |
  */
pub fn maybe_resend_wallet_txs(context: &mut WalletContext)  {
    
    todo!();
        /*
        
        */
}

pub fn dummy_sign_input(
        provider:    &SigningProvider,
        tx_in:       &mut TxIn,
        txout:       &TxOut,
        use_max_sig: bool) -> bool {
    
    todo!();
        /*
        
        */
}

pub struct WalletImpl {
    context: Rc<RefCell<WalletContext>>,
    wallet:  Arc<dyn WalletInterface>,
}

impl WalletInterface for WalletImpl {

}

impl EncryptWallet for WalletImpl {

    fn encrypt_wallet(&mut self, wallet_passphrase: &SecureString) -> bool {
        
        todo!();
        /*
            return m_wallet->EncryptWallet(wallet_passphrase);
        */
    }
}
    
impl IsCrypted for WalletImpl {

    fn is_crypted(&self) -> bool {
        
        todo!();
        /*
            return m_wallet->IsCrypted();
        */
    }
}
    
impl Lock for WalletImpl {

    fn lock(&mut self) -> bool {
        
        todo!();
        /*
            return m_wallet->Lock();
        */
    }
}
    
impl Unlock for WalletImpl {

    fn unlock(
        &mut self, 
        wallet_passphrase: &SecureString, 
        _accept_no_keys:   Option<bool>) -> bool 
    {
        todo!();
        /*
            return m_wallet->Unlock(wallet_passphrase);
        */
    }
}
    
impl IsLocked for WalletImpl {

    fn is_locked(&self) -> bool {
        
        todo!();
        /*
            return m_wallet->IsLocked();
        */
    }
}
    
impl ChangeWalletPassphrase for WalletImpl {

    fn change_wallet_passphrase(&mut self, 
        old_wallet_passphrase: &SecureString,
        new_wallet_passphrase: &SecureString) -> bool {
        
        todo!();
        /*
            return m_wallet->ChangeWalletPassphrase(old_wallet_passphrase, new_wallet_passphrase);
        */
    }
}
    
impl AbortRescan for WalletImpl {

    fn abort_rescan(&mut self)  {
        
        todo!();
        /*
            m_wallet->AbortRescan();
        */
    }
}
    
impl BackupWallet for WalletImpl {

    fn backup_wallet(&self, filename: &String) -> bool {
        
        todo!();
        /*
            return m_wallet->BackupWallet(filename);
        */
    }
}
    
impl GetWalletName for WalletImpl {

    fn get_wallet_name(&mut self) -> String {
        
        todo!();
        /*
            return m_wallet->GetName();
        */
    }
}
    
impl WalletGetNewDestination for WalletImpl {

    fn get_new_destination(
        &mut self, 
        ty:     OutputType,
        label:  String,
        dest:   &mut TxDestination, 
        _error: &mut BilingualStr) -> bool 
    {
        todo!();

        /*
            LOCK(m_wallet->cs_wallet);
            bilingual_str error;
            return m_wallet->WalletGetNewDestination(type, label, dest, error);
        */
    }
}
    
impl GetPubKeyWithScriptAndAddress for WalletImpl {

    fn get_pub_key_with_script_and_address(&mut self, 
        script:  &Script,
        address: &KeyID,
        pub_key: &mut PubKey) -> bool {
        
        todo!();
        /*
            std::unique_ptr<SigningProvider> provider = m_wallet->GetSolvingProvider(script);
            if (provider) {
                return provider->GetPubKey(address, pub_key);
            }
            return false;
        */
    }
}
    
impl SignMessage for WalletImpl {

    fn sign_message(&self, 
        message: &String,
        pkhash:  &PKHash,
        str_sig: &mut String) -> SigningResult {
        
        todo!();
        /*
            return m_wallet->SignMessage(message, pkhash, str_sig);
        */
    }
}
    
impl IsSpendable for WalletImpl {

    fn is_spendable(&mut self, dest: &TxDestination) -> bool {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return m_wallet->IsMine(dest) & ISMINE_SPENDABLE;
        */
    }
}
    
impl HaveWatchOnly for WalletImpl {

    fn have_watch_only(&mut self) -> bool {
        
        todo!();
        /*
            auto spk_man = m_wallet->GetLegacyScriptPubKeyMan();
            if (spk_man) {
                return spk_man->HaveWatchOnly();
            }
            return false;
        }{
        */
    }
}
    
impl SetAddressBook for WalletImpl {

    fn set_address_book(&mut self, 
        dest:    &TxDestination,
        name:    &String,
        purpose: &String) -> bool {
        
        todo!();
        /*
            return m_wallet->SetAddressBook(dest, name, purpose);
        */
    }
}
    
impl DelAddressBook for WalletImpl {

    fn del_address_book(&mut self, dest: &TxDestination) -> bool {
        
        todo!();
        /*
            return m_wallet->DelAddressBook(dest);
        */
    }
}
    
impl GetAddress for WalletImpl {

    fn get_address(&mut self, 
        dest:    &TxDestination,
        name:    *mut String,
        is_mine: *mut IsMineType,
        purpose: *mut String) -> bool {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            auto it = m_wallet->m_address_book.find(dest);
            if (it == m_wallet->m_address_book.end() || it->second.IsChange()) {
                return false;
            }
            if (name) {
                *name = it->second.GetLabel();
            }
            if (is_mine) {
                *is_mine = m_wallet->IsMine(dest);
            }
            if (purpose) {
                *purpose = it->second.purpose;
            }
            return true;
        */
    }
}
    
impl GetAddresses for WalletImpl {

    fn get_addresses(&mut self) -> Vec<WalletAddress> {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            std::vector<WalletAddress> result;
            for (const auto& item : m_wallet->m_address_book) {
                if (item.second.IsChange()) continue;
                result.emplace_back(item.first, m_wallet->IsMine(item.first), item.second.GetLabel(), item.second.purpose);
            }
            return result;
        */
    }
}
    
impl GetAddressReceiveRequests for WalletImpl {

    fn get_address_receive_requests(&self) -> Vec<String> {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return m_wallet->GetAddressReceiveRequests();
        */
    }
}
    
impl SetAddressReceiveRequest for WalletImpl {

    fn set_address_receive_request(
        &mut self, 
        dest:  &TxDestination,
        id:    &String,
        value: &String) -> bool {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            WalletBatch batch{m_wallet->GetDatabase()};
            return m_wallet->SetAddressReceiveRequest(batch, dest, id, value);
        */
    }
}

impl DisplayAddress for WalletImpl {

    fn display_address(&mut self, dest: &TxDestination) -> bool {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return m_wallet->DisplayAddress(dest);
        */
    }
}
    
impl LockCoin for WalletImpl {

    fn lock_coin(&mut self, 
        output:      &OutPoint,
        write_to_db: bool) -> bool {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            std::unique_ptr<WalletBatch> batch = write_to_db ? std::make_unique<WalletBatch>(m_wallet->GetDatabase()) : nullptr;
            return m_wallet->LockCoin(output, batch.get());
        */
    }
}
    
impl UnlockCoin for WalletImpl {

    fn unlock_coin(&mut self, output: &OutPoint) -> bool {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            std::unique_ptr<WalletBatch> batch = std::make_unique<WalletBatch>(m_wallet->GetDatabase());
            return m_wallet->UnlockCoin(output, batch.get());
        */
    }
}
    
impl IsLockedCoin for WalletImpl {

    fn is_locked_coin(&mut self, output: &OutPoint) -> bool {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return m_wallet->IsLockedCoin(output.hash, output.n);
        */
    }
}
    
impl ListLockedCoins for WalletImpl {

    fn list_locked_coins(&self, outputs: &mut Vec<OutPoint>)  {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return m_wallet->ListLockedCoins(outputs);
        */
    }
}
    
impl CreateTransaction for WalletImpl {

    fn create_transaction(&mut self, 
        recipients:   &Vec<Recipient>,
        coin_control: &CoinControl,
        sign:         bool,
        change_pos:   &mut i32,
        fee:          &mut Amount,
        fail_reason:  &mut BilingualStr) -> TransactionRef {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            CTransactionRef tx;
            FeeCalculation fee_calc_out;
            if (!CreateTransaction(*m_wallet, recipients, tx, fee, change_pos,
                    fail_reason, coin_control, fee_calc_out, sign)) {
                return {};
            }
            return tx;
        */
    }
}
    
impl CommitTransaction for WalletImpl {

    fn commit_transaction(&mut self, 
        tx:         TransactionRef,
        value_map:  WalletValueMap,
        order_form: WalletOrderForm)  {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            m_wallet->CommitTransaction(std::move(tx), std::move(value_map), std::move(order_form));
        */
    }
}
    
impl TransactionCanBeAbandoned for WalletImpl {

    fn transaction_can_be_abandoned(&self, txid: &u256) -> bool {
        
        todo!();
        /*
            return m_wallet->TransactionCanBeAbandoned(txid);
        */
    }
}
    
impl AbandonTransaction for WalletImpl {

    fn abandon_transaction(&mut self, txid: &u256) -> bool {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return m_wallet->AbandonTransaction(txid);
        */
    }
}
    
impl TransactionCanBeBumped for WalletImpl {

    fn transaction_can_be_bumped(&self, txid: &u256) -> bool {
        
        todo!();
        /*
            return feebumper::TransactionCanBeBumped(*m_wallet.get(), txid);
        */
    }
}
    
impl CreateBumpTransaction for WalletImpl {

    fn create_bump_transaction(&mut self, 
        txid:         &u256,
        coin_control: &CoinControl,
        errors:       &mut Vec<BilingualStr>,
        old_fee:      &mut Amount,
        new_fee:      &mut Amount,
        mtx:          &mut MutableTransaction) -> bool {
        
        todo!();
        /*
            return feebumper::CreateRateBumpTransaction(*m_wallet.get(), txid, coin_control, errors, old_fee, new_fee, mtx) == feebumper::Result::OK;
        */
    }
}
    
impl SignBumpTransaction for WalletImpl {

    fn sign_bump_transaction(&mut self, mtx: &mut MutableTransaction) -> bool {
        
        todo!();
        /*
            return feebumper::SignTransaction(*m_wallet.get(), mtx);
        */
    }
}
    
impl CommitBumpTransaction for WalletImpl {

    fn commit_bump_transaction(&mut self, 
        txid:        &u256,
        mtx:         MutableTransaction,
        errors:      &mut Vec<BilingualStr>,
        bumped_txid: &mut u256) -> bool {
        
        todo!();
        /*
            return feebumper::CommitTransaction(*m_wallet.get(), txid, std::move(mtx), errors, bumped_txid) ==
                   feebumper::Result::OK;
        */
    }
}
    
impl GetTx for WalletImpl {

    fn get_tx(&mut self, txid: &u256) -> TransactionRef {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            auto mi = m_wallet->mapWallet.find(txid);
            if (mi != m_wallet->mapWallet.end()) {
                return mi->second.tx;
            }
            return {};
        */
    }
}
    
impl GetWalletTx for WalletImpl {

    fn get_wallet_tx(&self, txid: &u256) -> WalletTx {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            auto mi = m_wallet->mapWallet.find(txid);
            if (mi != m_wallet->mapWallet.end()) {
                return MakeWalletTx(*m_wallet, mi->second);
            }
            return {};
        */
    }
}
    
impl GetWalletTxs for WalletImpl {

    fn get_wallet_txs(&mut self) -> Vec<WalletTx> {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            std::vector<WalletTx> result;
            result.reserve(m_wallet->mapWallet.size());
            for (const auto& entry : m_wallet->mapWallet) {
                result.emplace_back(MakeWalletTx(*m_wallet, entry.second));
            }
            return result;
        */
    }
}
    
impl TryGetTxStatus for WalletImpl {

    fn try_get_tx_status(&mut self, 
        txid:       &u256,
        tx_status:  &mut WalletTxStatus,
        num_blocks: &mut i32,
        block_time: &mut i64) -> bool {
        
        todo!();
        /*
            TRY_LOCK(m_wallet->cs_wallet, locked_wallet);
            if (!locked_wallet) {
                return false;
            }
            auto mi = m_wallet->mapWallet.find(txid);
            if (mi == m_wallet->mapWallet.end()) {
                return false;
            }
            num_blocks = m_wallet->GetLastBlockHeight();
            block_time = -1;
            CHECK_NONFATAL(m_wallet->chain().findBlock(m_wallet->GetLastBlockHash(), FoundBlock().time(block_time)));
            tx_status = MakeWalletTxStatus(*m_wallet, mi->second);
            return true;
        */
    }
}
    
impl GetWalletTxDetails for WalletImpl {

    fn get_wallet_tx_details(&mut self, 
        txid:       &u256,
        tx_status:  &mut WalletTxStatus,
        order_form: &mut WalletOrderForm,
        in_mempool: &mut bool,
        num_blocks: &mut i32) -> WalletTx {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            auto mi = m_wallet->mapWallet.find(txid);
            if (mi != m_wallet->mapWallet.end()) {
                num_blocks = m_wallet->GetLastBlockHeight();
                in_mempool = mi->second.InMempool();
                order_form = mi->second.vOrderForm;
                tx_status = MakeWalletTxStatus(*m_wallet, mi->second);
                return MakeWalletTx(*m_wallet, mi->second);
            }
            return {};
        */
    }
}
    
impl WalletFillPSBT for WalletImpl {

    fn fill_psbt(&mut self, 
        sighash_type: i32,
        sign:         bool,
        bip_32derivs: bool,
        n_signed:     *mut usize,
        psbtx:        &mut PartiallySignedTransaction,
        complete:     &mut bool) -> TransactionError {
        
        todo!();
        /*
            return m_wallet->FillPSBT(psbtx, complete, sighash_type, sign, bip32derivs, n_signed);
        */
    }
}
    
impl GetBalances for WalletImpl {

    fn get_balances(&mut self) -> WalletBalances {
        
        todo!();
        /*
            const auto bal = GetBalance(*m_wallet);
            WalletBalances result;
            result.balance = bal.m_mine_trusted;
            result.unconfirmed_balance = bal.m_mine_untrusted_pending;
            result.immature_balance = bal.m_mine_immature;
            result.have_watch_only = haveWatchOnly();
            if (result.have_watch_only) {
                result.watch_only_balance = bal.m_watchonly_trusted;
                result.unconfirmed_watch_only_balance = bal.m_watchonly_untrusted_pending;
                result.immature_watch_only_balance = bal.m_watchonly_immature;
            }
            return result;
        */
    }
}
    
impl TryGetBalances for WalletImpl {

    fn try_get_balances(&mut self, 
        balances:   &mut WalletBalances,
        block_hash: &mut u256) -> bool {
        
        todo!();
        /*
            TRY_LOCK(m_wallet->cs_wallet, locked_wallet);
            if (!locked_wallet) {
                return false;
            }
            block_hash = m_wallet->GetLastBlockHash();
            balances = getBalances();
            return true;
        */
    }
}
    
impl GetBalance for WalletImpl {

    fn get_balance(&mut self) -> Amount {
        
        todo!();
        /*
            return GetBalance(*m_wallet).m_mine_trusted;
        */
    }
}
    
impl GetAvailableBalance for WalletImpl {

    fn get_available_balance(&mut self, coin_control: &CoinControl) -> Amount {
        
        todo!();
        /*
            return GetAvailableBalance(*m_wallet, &coin_control);
        */
    }
}
    
impl TxinIsMine for WalletImpl {

    fn txin_is_mine(&mut self, txin: &TxIn) -> IsMineType {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return InputIsMine(*m_wallet, txin);
        */
    }
}
    
impl TxoutIsMine for WalletImpl {

    fn txout_is_mine(&mut self, txout: &TxOut) -> IsMineType {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return m_wallet->IsMine(txout);
        */
    }
}
    
impl GetDebit for WalletImpl {

    fn get_debit(&mut self, 
        txin:   &TxIn,
        filter: IsMineFilter) -> Amount {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return m_wallet->GetDebit(txin, filter);
        */
    }
}
    
impl GetCredit for WalletImpl {

    fn get_credit(&mut self, 
        txout:  &TxOut,
        filter: IsMineFilter) -> Amount {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            return OutputGetCredit(*m_wallet, txout, filter);
        */
    }
}
    
impl ListCoins for WalletImpl {

    fn list_coins(&mut self) -> CoinsList {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            CoinsList result;
            for (const auto& entry : ListCoins(*m_wallet)) {
                auto& group = result[entry.first];
                for (const auto& coin : entry.second) {
                    group.emplace_back(OutPoint(coin.tx->GetHash(), coin.i),
                        MakeWalletTxOut(*m_wallet, *coin.tx, coin.i, coin.nDepth));
                }
            }
            return result;
        */
    }
}
    
impl GetCoins for WalletImpl {

    fn get_coins(&mut self, outputs: &Vec<OutPoint>) -> Vec<WalletTxOut> {
        
        todo!();
        /*
            LOCK(m_wallet->cs_wallet);
            std::vector<WalletTxOut> result;
            result.reserve(outputs.size());
            for (const auto& output : outputs) {
                result.emplace_back();
                auto it = m_wallet->mapWallet.find(output.hash);
                if (it != m_wallet->mapWallet.end()) {
                    int depth = m_wallet->GetTxDepthInMainChain(it->second);
                    if (depth >= 0) {
                        result.back() = MakeWalletTxOut(*m_wallet, it->second, output.n, depth);
                    }
                }
            }
            return result;
        */
    }
}
    
impl GetRequiredFee for WalletImpl {

    fn get_required_fee(&mut self, tx_bytes: u32) -> Amount {
        
        todo!();
        /*
            return GetRequiredFee(*m_wallet, tx_bytes);
        */
    }
}
    
impl GetMinimumFee for WalletImpl {

    fn get_minimum_fee(&mut self, 
        tx_bytes:        u32,
        coin_control:    &CoinControl,
        returned_target: *mut i32,
        reason:          *mut FeeReason) -> Amount {
        
        todo!();
        /*
            FeeCalculation fee_calc;
            CAmount result;
            result = GetMinimumFee(*m_wallet, tx_bytes, coin_control, &fee_calc);
            if (returned_target) *returned_target = fee_calc.returnedTarget;
            if (reason) *reason = fee_calc.reason;
            return result;
        */
    }
}
    
impl GetConfirmTarget for WalletImpl {

    fn get_confirm_target(&mut self) -> u32 {
        
        todo!();
        /*
            return m_wallet->m_confirm_target;
        */
    }
}
    
impl HdEnabled for WalletImpl {

    fn hd_enabled(&mut self) -> bool {
        
        todo!();
        /*
            return m_wallet->IsHDEnabled();
        */
    }
}
    
impl CanGetAddresses for WalletImpl {

    fn can_get_addresses(&self, internal: Option<bool>) -> bool {
        
        todo!();
        /*
            return m_wallet->CanGetAddresses();
        */
    }
}
    
impl HasExternalSigner for WalletImpl {

    fn has_external_signer(&mut self) -> bool {
        
        todo!();
        /*
            return m_wallet->IsWalletFlagSet(WALLET_FLAG_EXTERNAL_SIGNER);
        */
    }
}
    
impl PrivateKeysDisabled for WalletImpl {

    fn private_keys_disabled(&mut self) -> bool {
        
        todo!();
        /*
            return m_wallet->IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS);
        */
    }
}
    
impl GetDefaultAddressType for WalletImpl {

    fn get_default_address_type(&mut self) -> OutputType {
        
        todo!();
        /*
            return m_wallet->m_default_address_type;
        */
    }
}
    
impl GetDefaultMaxTxFee for WalletImpl {

    fn get_default_max_tx_fee(&mut self) -> Amount {
        
        todo!();
        /*
            return m_wallet->m_default_max_tx_fee;
        */
    }
}
    
impl Remove for WalletImpl {

    fn remove(&mut self)  {
        
        todo!();
        /*
            RemoveWallet(m_context, m_wallet, false /* load_on_start */);
        */
    }
}
    
impl IsLegacy for WalletImpl {

    fn is_legacy(&self) -> bool {
        
        todo!();
        /*
            return m_wallet->IsLegacy();
        */
    }
}
    
impl HandleUnload for WalletImpl {

    fn handle_unload(&mut self, fn_: WalletUnloadFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(m_wallet->NotifyUnload.connect(fn));
        */
    }
}
    
impl HandleShowProgress for WalletImpl {

    type Callback = WalletShowProgressFn;

    fn handle_show_progress(&mut self, fn_: Self::Callback) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(m_wallet->ShowProgress.connect(fn));
        */
    }
}
    
impl HandleStatusChanged for WalletImpl {

    fn handle_status_changed(&mut self, fn_: WalletStatusChangedFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(m_wallet->NotifyStatusChanged.connect([fn](CWallet*) { fn(); }));
        */
    }
}
    
impl HandleAddressBookChanged for WalletImpl {

    fn handle_address_book_changed(&mut self, fn_: WalletAddressBookChangedFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(m_wallet->NotifyAddressBookChanged.connect(
                [fn](const TxDestination& address, const std::string& label, bool is_mine,
                     const std::string& purpose, ChangeType status) { fn(address, label, is_mine, purpose, status); }));
        */
    }
}
    
impl HandleTransactionChanged for WalletImpl {

    fn handle_transaction_changed(&mut self, fn_: WalletTransactionChangedFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(m_wallet->NotifyTransactionChanged.connect(
                [fn](const uint256& txid, ChangeType status) { fn(txid, status); }));
        */
    }
}
    
impl HandleWatchOnlyChanged for WalletImpl {

    fn handle_watch_only_changed(&mut self, fn_: WalletWatchOnlyChangedFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(m_wallet->NotifyWatchonlyChanged.connect(fn));
        */
    }
}
    
impl HandleCanGetAddressesChanged for WalletImpl {

    fn handle_can_get_addresses_changed(&mut self, fn_: WalletCanGetAddressesChangedFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(m_wallet->NotifyCanGetAddressesChanged.connect(fn));
        */
    }
}

impl WalletImpl {
    
    pub fn new(
        context: &mut WalletContext,
        wallet:  &Arc<Wallet>) -> Self {
    
        todo!();
        /*
        : context(context),
        : wallet(wallet),

        
        */
    }
}

impl GetWallet for WalletImpl {

    fn wallet(&mut self) -> Rc<RefCell<dyn WalletInterface>> {
        
        todo!();
        /*
            return m_wallet.get();
        */
    }
}

/**
  | Explicitly unload and delete the wallet.
  |
  | Blocks the current thread after signaling the
  | unload intent so that all wallet clients
  | release the wallet.
  |
  | Note that, when blocking is not required, the
  | wallet is implicitly unloaded by the shared
  | pointer deleter.
  */
pub fn unload_wallet(wallet: Arc<Wallet>)  {
    
    todo!();
        /*
        
        */
}

pub fn add_wallet(
    context: &mut WalletContext,
    wallet:  &Arc<Wallet>) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn get_wallets(context: &mut WalletContext) -> Vec<Arc<Wallet>> {
    
    todo!();
        /*
        
        */
}

pub fn get_wallet(
    context: &mut WalletContext,
    name:    &String) -> Arc<Wallet> {

    todo!();
        /*
        
        */
}

pub fn load_wallet(
    context:       &mut WalletContext,
    name:          &String,
    load_on_start: Option<bool>,
    options:       &DatabaseOptions,
    status:        &mut DatabaseStatus,
    error:         &mut BilingualStr,
    warnings:      &mut Vec<BilingualStr>) -> Arc<Wallet> {

    todo!();
        /*
        
        */
}

pub fn create_wallet(
    context:       &mut WalletContext,
    name:          &String,
    load_on_start: Option<bool>,
    options:       &mut DatabaseOptions,
    status:        &mut DatabaseStatus,
    error:         &mut BilingualStr,
    warnings:      &mut Vec<BilingualStr>) -> Arc<Wallet> {

    todo!();
    /*
        
        */
}

pub fn handle_load_wallet(
    context:     &mut WalletContext,
    load_wallet: LoadWalletFn) -> Box<dyn Handler> {
    
    todo!();
        /*
        
        */
}

pub fn make_wallet_database(
    name:    &String,
    options: &DatabaseOptions,
    status:  &mut DatabaseStatus,
    error:   &mut BilingualStr) -> Box<WalletDatabase> {
    
    todo!();
        /*
        
        */
}


/**
  | A wrapper to reserve an address from
  | a wallet
  | 
  | ReserveDestination is used to reserve
  | an address.
  | 
  | It is currently only used inside of CreateTransaction.
  | 
  | Instantiating a ReserveDestination
  | does not reserve an address. To do so,
  | 
  | GetReservedDestination() needs to
  | be called on the object. Once an address
  | has been reserved, call KeepDestination()
  | on the ReserveDestination object to
  | make sure it is not returned. Call ReturnDestination()
  | to return the address so it can be re-used
  | (for example, if the address was used
  | in a new transaction and that transaction
  | was not completed and needed to be aborted).
  | 
  | If an address is reserved and KeepDestination()
  | is not called, then the address will
  | be returned when the ReserveDestination
  | goes out of scope.
  |
  */
pub struct ReserveDestination {

    /**
      | The wallet to reserve from
      |
      */
    pwallet:  *const Wallet,

    /**
      | The ScriptPubKeyMan to reserve from.
      | Based on type when GetReservedDestination
      | is called
      |
      */
    spk_man:  *mut ScriptPubKeyMan, // default = { nullptr }

    ty:       OutputType,

    /**
      | The index of the address's key in the
      | keypool
      |
      */
    n_index:  i64, // default = { -1 }

    /**
      | The destination
      |
      */
    address:  TxDestination,

    /**
      | Whether this is from the internal (change
      | output) keypool
      |
      */
    internal: bool, // default = { false }

}

impl Drop for ReserveDestination {

    /**
      | Destructor. If a key has been reserved
      | and not KeepKey'ed, it will be returned
      | to the keypool
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            ReturnDestination();
        */
    }
}

impl ReserveDestination {

    /**
      | Construct a ReserveDestination object.
      | This does NOT reserve an address yet
      |
      */
    pub fn new(
        pwallet: *mut Wallet,
        ty:      OutputType) -> Self {
    
        todo!();
        /*
        : pwallet(pwallet),
        : ty(type),

        
        */
    }

    /**
      | Reserve an address
      |
      */
    pub fn get_reserved_destination(&mut self, 
        pubkey:   &mut TxDestination,
        internal: bool,
        error:    &mut BilingualStr) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Return reserved address
      |
      */
    pub fn return_destination(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Keep the address. Do not return it's
      | key to the keypool when this object goes
      | out of scope
      |
      */
    pub fn keep_destination(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}

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

//-------------------------------------------[.cpp/bitcoin/src/wallet/context.h]
//-------------------------------------------[.cpp/bitcoin/src/wallet/context.cpp]

//-------------------------------------------[.cpp/bitcoin/src/wallet/walletdb.h]

/**
  | Overview of wallet database classes:
  | 
  | - WalletBatch is an abstract modifier
  | object for the wallet database, and
  | encapsulates a database batch update
  | as well as methods to act on the database.
  | It should be agnostic to the database
  | implementation.
  | 
  | The following classes are implementation
  | specific:
  | 
  | - BerkeleyEnvironment is an environment
  | in which the database exists.
  | 
  | - BerkeleyDatabase represents a wallet
  | database.
  | 
  | - BerkeleyBatch is a low-level database
  | batch update.
  |
  */
pub const DEFAULT_FLUSHWALLET: bool = true;

/**
  | Callback for filtering key types to
  | deserialize in ReadKeyValue
  |
  */
pub type KeyFilterFn = fn(_0: &String) -> bool;

//-------------------------------------------[.cpp/bitcoin/src/wallet/walletdb.cpp]

pub mod db_keys {
    pub const ACENTRY:                 &'static str = "acentry";
    pub const ACTIVEEXTERNALSPK:       &'static str = "activeexternalspk";
    pub const ACTIVEINTERNALSPK:       &'static str = "activeinternalspk";
    pub const BESTBLOCK_NOMERKLE:      &'static str = "bestblock_nomerkle";
    pub const BESTBLOCK:               &'static str = "bestblock";
    pub const CRYPTED_KEY:             &'static str = "ckey";
    pub const CSCRIPT:                 &'static str = "cscript";
    pub const DEFAULTKEY:              &'static str = "defaultkey";
    pub const DESTDATA:                &'static str = "destdata";
    pub const FLAGS:                   &'static str = "flags";
    pub const HDCHAIN:                 &'static str = "hdchain";
    pub const KEYMETA:                 &'static str = "keymeta";
    pub const KEY:                     &'static str = "key";
    pub const LOCKED_UTXO:             &'static str = "lockedutxo";
    pub const MASTER_KEY:              &'static str = "mkey";
    pub const MINVERSION:              &'static str = "minversion";
    pub const NAME:                    &'static str = "name";
    pub const OLD_KEY:                 &'static str = "wkey";
    pub const ORDERPOSNEXT:            &'static str = "orderposnext";
    pub const POOL:                    &'static str = "pool";
    pub const PURPOSE:                 &'static str = "purpose";
    pub const SETTINGS:                &'static str = "settings";
    pub const TX:                      &'static str = "tx";
    pub const VERSION:                 &'static str = "version";
    pub const WALLETDESCRIPTOR:        &'static str = "walletdescriptor";
    pub const WALLETDESCRIPTORCACHE:   &'static str = "walletdescriptorcache";
    pub const WALLETDESCRIPTORLHCACHE: &'static str = "walletdescriptorlhcache";
    pub const WALLETDESCRIPTORCKEY:    &'static str = "walletdescriptorckey";
    pub const WALLETDESCRIPTORKEY:     &'static str = "walletdescriptorkey";
    pub const WATCHMETA:               &'static str = "watchmeta";
    pub const WATCHS:                  &'static str = "watchs";
}

#[derive(Default)]
pub struct WalletScanState {
    n_keys:                u32, // default = { 0 }
    n_ckeys:               u32, // default = { 0 }
    n_watch_keys:          u32, // default = { 0 }
    n_key_meta:            u32, // default = { 0 }
    unknown_records:       u32, // default = { 0 }
    is_encrypted:          bool, // default = { false }
    any_unordered:         bool, // default = { false }
    wallet_upgrade:        Vec<u256>,
    active_external_spks:  HashMap<OutputType,u256>,
    active_internal_spks:  HashMap<OutputType,u256>,
    descriptor_caches:     HashMap<u256,DescriptorCache>,
    descriptor_keys:       HashMap<(u256,KeyID),Key>,
    descriptor_crypt_keys: HashMap<(u256,KeyID),(PubKey,Vec<u8>)>,
    hd_chains:             HashMap<u160,HDChain>,
    tx_corrupt:            bool, // default = { false }
}

#[EXCLUSIVE_LOCKS_REQUIRED(pwallet->cs_wallet)]
pub fn read_key_value_with_wallet_scan_state(
        pwallet:   *mut Wallet,
        ss_key:    &mut DataStream,
        ss_value:  &mut DataStream,
        wss:       &mut WalletScanState,
        str_type:  &mut String,
        str_err:   &mut String,
        filter_fn: Option<&KeyFilterFn>) -> bool {

    todo!();
        /*
            try {
            // Unserialize
            // Taking advantage of the fact that pair serialization
            // is just the two items serialized one after the other
            ssKey >> strType;
            // If we have a filter, check if this matches the filter
            if (filter_fn && !filter_fn(strType)) {
                return true;
            }
            if (strType == DBKeys::NAME) {
                std::string strAddress;
                ssKey >> strAddress;
                std::string label;
                ssValue >> label;
                pwallet->m_address_book[DecodeDestination(strAddress)].SetLabel(label);
            } else if (strType == DBKeys::PURPOSE) {
                std::string strAddress;
                ssKey >> strAddress;
                ssValue >> pwallet->m_address_book[DecodeDestination(strAddress)].purpose;
            } else if (strType == DBKeys::TX) {
                uint256 hash;
                ssKey >> hash;
                // LoadToWallet call below creates a new CWalletTx that fill_wtx
                // callback fills with transaction metadata.
                auto fill_wtx = [&](CWalletTx& wtx, bool new_tx) {
                    if(!new_tx) {
                        // There's some corruption here since the tx we just tried to load was already in the wallet.
                        // We don't consider this type of corruption critical, and can fix it by removing tx data and
                        // rescanning.
                        wss.tx_corrupt = true;
                        return false;
                    }
                    ssValue >> wtx;
                    if (wtx.GetHash() != hash)
                        return false;

                    // Undo serialize changes in 31600
                    if (31404 <= wtx.fTimeReceivedIsTxTime && wtx.fTimeReceivedIsTxTime <= 31703)
                    {
                        if (!ssValue.empty())
                        {
                            uint8_t fTmp;
                            uint8_t fUnused;
                            std::string unused_string;
                            ssValue >> fTmp >> fUnused >> unused_string;
                            strErr = strprintf("LoadWallet() upgrading tx ver=%d %d %s",
                                               wtx.fTimeReceivedIsTxTime, fTmp, hash.ToString());
                            wtx.fTimeReceivedIsTxTime = fTmp;
                        }
                        else
                        {
                            strErr = strprintf("LoadWallet() repairing tx ver=%d %s", wtx.fTimeReceivedIsTxTime, hash.ToString());
                            wtx.fTimeReceivedIsTxTime = 0;
                        }
                        wss.vWalletUpgrade.push_back(hash);
                    }

                    if (wtx.nOrderPos == -1)
                        wss.fAnyUnordered = true;

                    return true;
                };
                if (!pwallet->LoadToWallet(hash, fill_wtx)) {
                    return false;
                }
            } else if (strType == DBKeys::WATCHS) {
                wss.nWatchKeys++;
                CScript script;
                ssKey >> script;
                uint8_t fYes;
                ssValue >> fYes;
                if (fYes == '1') {
                    pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadWatchOnly(script);
                }
            } else if (strType == DBKeys::KEY) {
                CPubKey vchPubKey;
                ssKey >> vchPubKey;
                if (!vchPubKey.IsValid())
                {
                    strErr = "Error reading wallet database: CPubKey corrupt";
                    return false;
                }
                CKey key;
                CPrivKey pkey;
                uint256 hash;

                wss.nKeys++;
                ssValue >> pkey;

                // Old wallets store keys as DBKeys::KEY [pubkey] => [privkey]
                // ... which was slow for wallets with lots of keys, because the public key is re-derived from the private key
                // using EC operations as a checksum.
                // Newer wallets store keys as DBKeys::KEY [pubkey] => [privkey][hash(pubkey,privkey)], which is much faster while
                // remaining backwards-compatible.
                try
                {
                    ssValue >> hash;
                }
                catch (const std::ios_base::failure&) {}

                bool fSkipCheck = false;

                if (!hash.IsNull())
                {
                    // hash pubkey/privkey to accelerate wallet load
                    std::vector<unsigned char> vchKey;
                    vchKey.reserve(vchPubKey.size() + pkey.size());
                    vchKey.insert(vchKey.end(), vchPubKey.begin(), vchPubKey.end());
                    vchKey.insert(vchKey.end(), pkey.begin(), pkey.end());

                    if (Hash(vchKey) != hash)
                    {
                        strErr = "Error reading wallet database: CPubKey/CPrivKey corrupt";
                        return false;
                    }

                    fSkipCheck = true;
                }

                if (!key.Load(pkey, vchPubKey, fSkipCheck))
                {
                    strErr = "Error reading wallet database: CPrivKey corrupt";
                    return false;
                }
                if (!pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadKey(key, vchPubKey))
                {
                    strErr = "Error reading wallet database: LegacyScriptPubKeyMan::LoadKey failed";
                    return false;
                }
            } else if (strType == DBKeys::MASTER_KEY) {
                // Master encryption key is loaded into only the wallet and not any of the ScriptPubKeyMans.
                unsigned int nID;
                ssKey >> nID;
                CMasterKey kMasterKey;
                ssValue >> kMasterKey;
                if(pwallet->mapMasterKeys.count(nID) != 0)
                {
                    strErr = strprintf("Error reading wallet database: duplicate CMasterKey id %u", nID);
                    return false;
                }
                pwallet->mapMasterKeys[nID] = kMasterKey;
                if (pwallet->nMasterKeyMaxID < nID)
                    pwallet->nMasterKeyMaxID = nID;
            } else if (strType == DBKeys::CRYPTED_KEY) {
                CPubKey vchPubKey;
                ssKey >> vchPubKey;
                if (!vchPubKey.IsValid())
                {
                    strErr = "Error reading wallet database: CPubKey corrupt";
                    return false;
                }
                std::vector<unsigned char> vchPrivKey;
                ssValue >> vchPrivKey;

                // Get the checksum and check it
                bool checksum_valid = false;
                if (!ssValue.eof()) {
                    uint256 checksum;
                    ssValue >> checksum;
                    if ((checksum_valid = Hash(vchPrivKey) != checksum)) {
                        strErr = "Error reading wallet database: Encrypted key corrupt";
                        return false;
                    }
                }

                wss.nCKeys++;

                if (!pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadCryptedKey(vchPubKey, vchPrivKey, checksum_valid))
                {
                    strErr = "Error reading wallet database: LegacyScriptPubKeyMan::LoadCryptedKey failed";
                    return false;
                }
                wss.fIsEncrypted = true;
            } else if (strType == DBKeys::KEYMETA) {
                CPubKey vchPubKey;
                ssKey >> vchPubKey;
                CKeyMetadata keyMeta;
                ssValue >> keyMeta;
                wss.nKeyMeta++;
                pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadKeyMetadata(vchPubKey.GetID(), keyMeta);

                // Extract some CHDChain info from this metadata if it has any
                if (keyMeta.nVersion >= CKeyMetadata::VERSION_WITH_HDDATA && !keyMeta.hd_seed_id.IsNull() && keyMeta.hdKeypath.size() > 0) {
                    // Get the path from the key origin or from the path string
                    // Not applicable when path is "s" or "m" as those indicate a seed
                    // See https://github.com/bitcoin/bitcoin/pull/12924
                    bool internal = false;
                    uint32_t index = 0;
                    if (keyMeta.hdKeypath != "s" && keyMeta.hdKeypath != "m") {
                        std::vector<uint32_t> path;
                        if (keyMeta.has_key_origin) {
                            // We have a key origin, so pull it from its path vector
                            path = keyMeta.key_origin.path;
                        } else {
                            // No key origin, have to parse the string
                            if (!ParseHDKeypath(keyMeta.hdKeypath, path)) {
                                strErr = "Error reading wallet database: keymeta with invalid HD keypath";
                                return false;
                            }
                        }

                        // Extract the index and internal from the path
                        // Path string is m/0'/k'/i'
                        // Path vector is [0', k', i'] (but as ints OR'd with the hardened bit
                        // k == 0 for external, 1 for internal. i is the index
                        if (path.size() != 3) {
                            strErr = "Error reading wallet database: keymeta found with unexpected path";
                            return false;
                        }
                        if (path[0] != 0x80000000) {
                            strErr = strprintf("Unexpected path index of 0x%08x (expected 0x80000000) for the element at index 0", path[0]);
                            return false;
                        }
                        if (path[1] != 0x80000000 && path[1] != (1 | 0x80000000)) {
                            strErr = strprintf("Unexpected path index of 0x%08x (expected 0x80000000 or 0x80000001) for the element at index 1", path[1]);
                            return false;
                        }
                        if ((path[2] & 0x80000000) == 0) {
                            strErr = strprintf("Unexpected path index of 0x%08x (expected to be greater than or equal to 0x80000000)", path[2]);
                            return false;
                        }
                        internal = path[1] == (1 | 0x80000000);
                        index = path[2] & ~0x80000000;
                    }

                    // Insert a new CHDChain, or get the one that already exists
                    auto ins = wss.m_hd_chains.emplace(keyMeta.hd_seed_id, CHDChain());
                    CHDChain& chain = ins.first->second;
                    if (ins.second) {
                        // For new chains, we want to default to VERSION_HD_BASE until we see an internal
                        chain.nVersion = CHDChain::VERSION_HD_BASE;
                        chain.seed_id = keyMeta.hd_seed_id;
                    }
                    if (internal) {
                        chain.nVersion = CHDChain::VERSION_HD_CHAIN_SPLIT;
                        chain.nInternalChainCounter = std::max(chain.nInternalChainCounter, index);
                    } else {
                        chain.nExternalChainCounter = std::max(chain.nExternalChainCounter, index);
                    }
                }
            } else if (strType == DBKeys::WATCHMETA) {
                CScript script;
                ssKey >> script;
                CKeyMetadata keyMeta;
                ssValue >> keyMeta;
                wss.nKeyMeta++;
                pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadScriptMetadata(CScriptID(script), keyMeta);
            } else if (strType == DBKeys::DEFAULTKEY) {
                // We don't want or need the default key, but if there is one set,
                // we want to make sure that it is valid so that we can detect corruption
                CPubKey vchPubKey;
                ssValue >> vchPubKey;
                if (!vchPubKey.IsValid()) {
                    strErr = "Error reading wallet database: Default Key corrupt";
                    return false;
                }
            } else if (strType == DBKeys::POOL) {
                int64_t nIndex;
                ssKey >> nIndex;
                CKeyPool keypool;
                ssValue >> keypool;

                pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadKeyPool(nIndex, keypool);
            } else if (strType == DBKeys::CSCRIPT) {
                u160 hash;
                ssKey >> hash;
                CScript script;
                ssValue >> script;
                if (!pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadCScript(script))
                {
                    strErr = "Error reading wallet database: LegacyScriptPubKeyMan::LoadCScript failed";
                    return false;
                }
            } else if (strType == DBKeys::ORDERPOSNEXT) {
                ssValue >> pwallet->nOrderPosNext;
            } else if (strType == DBKeys::DESTDATA) {
                std::string strAddress, strKey, strValue;
                ssKey >> strAddress;
                ssKey >> strKey;
                ssValue >> strValue;
                pwallet->LoadDestData(DecodeDestination(strAddress), strKey, strValue);
            } else if (strType == DBKeys::HDCHAIN) {
                CHDChain chain;
                ssValue >> chain;
                pwallet->GetOrCreateLegacyScriptPubKeyMan()->LoadHDChain(chain);
            } else if (strType == DBKeys::OLD_KEY) {
                strErr = "Found unsupported 'wkey' record, try loading with version 0.18";
                return false;
            } else if (strType == DBKeys::ACTIVEEXTERNALSPK || strType == DBKeys::ACTIVEINTERNALSPK) {
                uint8_t type;
                ssKey >> type;
                uint256 id;
                ssValue >> id;

                bool internal = strType == DBKeys::ACTIVEINTERNALSPK;
                auto& spk_mans = internal ? wss.m_active_internal_spks : wss.m_active_external_spks;
                if (spk_mans.count(static_cast<OutputType>(type)) > 0) {
                    strErr = "Multiple ScriptPubKeyMans specified for a single type";
                    return false;
                }
                spk_mans[static_cast<OutputType>(type)] = id;
            } else if (strType == DBKeys::WALLETDESCRIPTOR) {
                uint256 id;
                ssKey >> id;
                WalletDescriptor desc;
                ssValue >> desc;
                if (wss.m_descriptor_caches.count(id) == 0) {
                    wss.m_descriptor_caches[id] = DescriptorCache();
                }
                pwallet->LoadDescriptorScriptPubKeyMan(id, desc);
            } else if (strType == DBKeys::WALLETDESCRIPTORCACHE) {
                bool parent = true;
                uint256 desc_id;
                uint32_t key_exp_index;
                uint32_t der_index;
                ssKey >> desc_id;
                ssKey >> key_exp_index;

                // if the der_index exists, it's a derived xpub
                try
                {
                    ssKey >> der_index;
                    parent = false;
                }
                catch (...) {}

                std::vector<unsigned char> ser_xpub(BIP32_EXTKEY_SIZE);
                ssValue >> ser_xpub;
                CExtPubKey xpub;
                xpub.Decode(ser_xpub.data());
                if (parent) {
                    wss.m_descriptor_caches[desc_id].CacheParentExtPubKey(key_exp_index, xpub);
                } else {
                    wss.m_descriptor_caches[desc_id].CacheDerivedExtPubKey(key_exp_index, der_index, xpub);
                }
            } else if (strType == DBKeys::WALLETDESCRIPTORLHCACHE) {
                uint256 desc_id;
                uint32_t key_exp_index;
                ssKey >> desc_id;
                ssKey >> key_exp_index;

                std::vector<unsigned char> ser_xpub(BIP32_EXTKEY_SIZE);
                ssValue >> ser_xpub;
                CExtPubKey xpub;
                xpub.Decode(ser_xpub.data());
                wss.m_descriptor_caches[desc_id].CacheLastHardenedExtPubKey(key_exp_index, xpub);
            } else if (strType == DBKeys::WALLETDESCRIPTORKEY) {
                uint256 desc_id;
                CPubKey pubkey;
                ssKey >> desc_id;
                ssKey >> pubkey;
                if (!pubkey.IsValid())
                {
                    strErr = "Error reading wallet database: CPubKey corrupt";
                    return false;
                }
                CKey key;
                CPrivKey pkey;
                uint256 hash;

                wss.nKeys++;
                ssValue >> pkey;
                ssValue >> hash;

                // hash pubkey/privkey to accelerate wallet load
                std::vector<unsigned char> to_hash;
                to_hash.reserve(pubkey.size() + pkey.size());
                to_hash.insert(to_hash.end(), pubkey.begin(), pubkey.end());
                to_hash.insert(to_hash.end(), pkey.begin(), pkey.end());

                if (Hash(to_hash) != hash)
                {
                    strErr = "Error reading wallet database: CPubKey/CPrivKey corrupt";
                    return false;
                }

                if (!key.Load(pkey, pubkey, true))
                {
                    strErr = "Error reading wallet database: CPrivKey corrupt";
                    return false;
                }
                wss.m_descriptor_keys.insert(std::make_pair(std::make_pair(desc_id, pubkey.GetID()), key));
            } else if (strType == DBKeys::WALLETDESCRIPTORCKEY) {
                uint256 desc_id;
                CPubKey pubkey;
                ssKey >> desc_id;
                ssKey >> pubkey;
                if (!pubkey.IsValid())
                {
                    strErr = "Error reading wallet database: CPubKey corrupt";
                    return false;
                }
                std::vector<unsigned char> privkey;
                ssValue >> privkey;
                wss.nCKeys++;

                wss.m_descriptor_crypt_keys.insert(std::make_pair(std::make_pair(desc_id, pubkey.GetID()), std::make_pair(pubkey, privkey)));
                wss.fIsEncrypted = true;
            } else if (strType == DBKeys::LOCKED_UTXO) {
                uint256 hash;
                uint32_t n;
                ssKey >> hash;
                ssKey >> n;
                pwallet->LockCoin(OutPoint(hash, n));
            } else if (strType != DBKeys::BESTBLOCK && strType != DBKeys::BESTBLOCK_NOMERKLE &&
                       strType != DBKeys::MINVERSION && strType != DBKeys::ACENTRY &&
                       strType != DBKeys::VERSION && strType != DBKeys::SETTINGS &&
                       strType != DBKeys::FLAGS) {
                wss.m_unknown_records++;
            }
        } catch (const std::exception& e) {
            if (strErr.empty()) {
                strErr = e.what();
            }
            return false;
        } catch (...) {
            if (strErr.empty()) {
                strErr = "Caught unknown exception in ReadKeyValue";
            }
            return false;
        }
        return true;
        */
}

/**
  | Unserialize a given Key-Value pair
  | and load it into the wallet
  |
  */
pub fn read_key_value(
        pwallet:   *mut Wallet,
        ss_key:    &mut DataStream,
        ss_value:  &mut DataStream,
        str_type:  &mut String,
        str_err:   &mut String,
        filter_fn: Option<&KeyFilterFn>) -> bool {
    
    todo!();
        /*
        CWalletScanState dummy_wss;
        LOCK(pwallet->cs_wallet);
        return ReadKeyValue(pwallet, ssKey, ssValue, dummy_wss, strType, strErr, filter_fn);
        */
}

pub fn make_database(
        path:    &Path,
        options: &DatabaseOptions,
        status:  &mut DatabaseStatus,
        error:   &mut BilingualStr) -> Box<WalletDatabase> {
    
    todo!();
        /*
            bool exists;
        try {
            exists = fs::symlink_status(path).type() != fs::file_not_found;
        } catch (const fs::filesystem_error& e) {
            error = Untranslated(strprintf("Failed to access database path '%s': %s", fs::PathToString(path), fsbridge::get_filesystem_error_message(e)));
            status = DatabaseStatus::FAILED_BAD_PATH;
            return nullptr;
        }

        std::optional<DatabaseFormat> format;
        if (exists) {
            if (IsBDBFile(BDBDataFile(path))) {
                format = DatabaseFormat::BERKELEY;
            }
            if (IsSQLiteFile(SQLiteDataFile(path))) {
                if (format) {
                    error = Untranslated(strprintf("Failed to load database path '%s'. Data is in ambiguous format.", fs::PathToString(path)));
                    status = DatabaseStatus::FAILED_BAD_FORMAT;
                    return nullptr;
                }
                format = DatabaseFormat::SQLITE;
            }
        } else if (options.require_existing) {
            error = Untranslated(strprintf("Failed to load database path '%s'. Path does not exist.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_NOT_FOUND;
            return nullptr;
        }

        if (!format && options.require_existing) {
            error = Untranslated(strprintf("Failed to load database path '%s'. Data is not in recognized format.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_BAD_FORMAT;
            return nullptr;
        }

        if (format && options.require_create) {
            error = Untranslated(strprintf("Failed to create database path '%s'. Database already exists.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_ALREADY_EXISTS;
            return nullptr;
        }

        // A db already exists so format is set, but options also specifies the format, so make sure they agree
        if (format && options.require_format && format != options.require_format) {
            error = Untranslated(strprintf("Failed to load database path '%s'. Data is not in required format.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_BAD_FORMAT;
            return nullptr;
        }

        // Format is not set when a db doesn't already exist, so use the format specified by the options if it is set.
        if (!format && options.require_format) format = options.require_format;

        // If the format is not specified or detected, choose the default format based on what is available. We prefer BDB over SQLite for now.
        if (!format) {
    #ifdef USE_SQLITE
            format = DatabaseFormat::SQLITE;
    #endif
    #ifdef USE_BDB
            format = DatabaseFormat::BERKELEY;
    #endif
        }

        if (format == DatabaseFormat::SQLITE) {
    #ifdef USE_SQLITE
            return MakeSQLiteDatabase(path, options, status, error);
    #endif
            error = Untranslated(strprintf("Failed to open database path '%s'. Build does not support SQLite database format.", fs::PathToString(path)));
            status = DatabaseStatus::FAILED_BAD_FORMAT;
            return nullptr;
        }

    #ifdef USE_BDB
        return MakeBerkeleyDatabase(path, options, status, error);
    #endif
        error = Untranslated(strprintf("Failed to open database path '%s'. Build does not support Berkeley DB database format.", fs::PathToString(path)));
        status = DatabaseStatus::FAILED_BAD_FORMAT;
        return nullptr;
        */
}

/**
  | Return object for accessing dummy database
  | with no read/write capabilities.
  |
  */
pub fn create_dummy_wallet_database() -> Box<WalletDatabase> {
    
    todo!();
        /*
            return std::make_unique<DummyDatabase>();
        */
}

/**
  | Return object for accessing temporary
  | in-memory database.
  |
  */
pub fn create_mock_wallet_database() -> Box<WalletDatabase> {
    
    todo!();
        /*
            #ifdef USE_SQLITE
        return std::make_unique<SQLiteDatabase>("", "", true);
    #elif USE_BDB
        return std::make_unique<BerkeleyDatabase>(std::make_shared<BerkeleyEnvironment>(), "");
    #endif
        */
}

/**
  | Compacts BDB state so that wallet.dat
  | is self-contained (if there are changes)
  |
  */
pub fn maybe_compact_walletdb(context: &mut WalletContext)  {
    
    todo!();
        /*
            static std::atomic<bool> fOneThread(false);
        if (fOneThread.exchange(true)) {
            return;
        }

        for (const std::shared_ptr<CWallet>& pwallet : GetWallets(context)) {
            WalletDatabase& dbh = pwallet->GetDatabase();

            unsigned int nUpdateCounter = dbh.nUpdateCounter;

            if (dbh.nLastSeen != nUpdateCounter) {
                dbh.nLastSeen = nUpdateCounter;
                dbh.nLastWalletUpdate = GetTime();
            }

            if (dbh.nLastFlushed != nUpdateCounter && GetTime() - dbh.nLastWalletUpdate >= 2) {
                if (dbh.PeriodicFlush()) {
                    dbh.nLastFlushed = nUpdateCounter;
                }
            }
        }

        fOneThread = false;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/walletinitinterface.h]

pub trait WalletInitInterface:
HasWalletSupport
+ AddWalletOptions
+ ParameterInteraction
+ Construct
{ }

pub trait HasWalletSupport {

    /**
      | Is the wallet component enabled
      |
      */
    fn has_wallet_support(&self) -> bool;
}

pub trait AddWalletOptions {

    /**
      | Get wallet help string
      |
      */
    fn add_wallet_options(&self, argsman: &mut ArgsManager);
}

pub trait ParameterInteraction {

    /**
      | Check wallet parameter interaction
      |
      */
    fn parameter_interaction(&self) -> bool;
}

pub trait Construct {

    /**
      | Add wallets that should be opened to
      | list of chain clients.
      |
      */
    fn construct(&self, node: &mut NodeContext);
}

lazy_static!{
    /*
    extern const WalletInitInterface& g_wallet_init_interface;
    */
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/wallettool.h]
//-------------------------------------------[.cpp/bitcoin/src/wallet/wallettool.cpp]

/**
  | The standard wallet deleter function blocks on
  | the validation interface queue, which doesn't
  | exist for the bitcoin-wallet. Define our own
  | deleter here.
  */
pub fn wallet_tool_release_wallet(wallet: *mut Wallet)  {
    
    todo!();
        /*
            wallet->WalletLogPrintf("Releasing wallet\n");
        wallet->Close();
        delete wallet;
        */
}

pub fn wallet_create(
        wallet_instance:       *mut Wallet,
        wallet_creation_flags: u64)  {
    
    todo!();
        /*
            LOCK(wallet_instance->cs_wallet);

        wallet_instance->SetMinVersion(FEATURE_HD_SPLIT);
        wallet_instance->AddWalletFlags(wallet_creation_flags);

        if (!wallet_instance->IsWalletFlagSet(WALLET_FLAG_DESCRIPTORS)) {
            auto spk_man = wallet_instance->GetOrCreateLegacyScriptPubKeyMan();
            spk_man->SetupGeneration(false);
        } else {
            wallet_instance->SetupDescriptorScriptPubKeyMans();
        }

        tfm::format(std::cout, "Topping up keypool...\n");
        wallet_instance->TopUpKeyPool();
        */
}

pub fn make_wallet(
        name:    &String,
        path:    &Path,
        options: DatabaseOptions) -> Arc<Wallet> {
    
    todo!();
        /*
            DatabaseStatus status;
        bilingual_str error;
        std::unique_ptr<WalletDatabase> database = MakeDatabase(path, options, status, error);
        if (!database) {
            tfm::format(std::cerr, "%s\n", error.original);
            return nullptr;
        }

        // dummy chain interface
        std::shared_ptr<CWallet> wallet_instance{new CWallet(nullptr /* chain */, name, std::move(database)), WalletToolReleaseWallet};
        DBErrors load_wallet_ret;
        try {
            load_wallet_ret = wallet_instance->LoadWallet();
        } catch (const std::runtime_error&) {
            tfm::format(std::cerr, "Error loading %s. Is wallet being used by another process?\n", name);
            return nullptr;
        }

        if (load_wallet_ret != DBErrors::LOAD_OK) {
            wallet_instance = nullptr;
            if (load_wallet_ret == DBErrors::CORRUPT) {
                tfm::format(std::cerr, "Error loading %s: Wallet corrupted", name);
                return nullptr;
            } else if (load_wallet_ret == DBErrors::NONCRITICAL_ERROR) {
                tfm::format(std::cerr, "Error reading %s! All keys read correctly, but transaction data"
                                " or address book entries might be missing or incorrect.",
                    name);
            } else if (load_wallet_ret == DBErrors::TOO_NEW) {
                tfm::format(std::cerr, "Error loading %s: Wallet requires newer version of %s",
                    name, PACKAGE_NAME);
                return nullptr;
            } else if (load_wallet_ret == DBErrors::NEED_REWRITE) {
                tfm::format(std::cerr, "Wallet needed to be rewritten: restart %s to complete", PACKAGE_NAME);
                return nullptr;
            } else if (load_wallet_ret == DBErrors::NEED_RESCAN) {
                tfm::format(std::cerr, "Error reading %s! Some transaction data might be missing or"
                               " incorrect. Wallet requires a rescan.",
                    name);
            } else {
                tfm::format(std::cerr, "Error loading %s", name);
                return nullptr;
            }
        }

        if (options.require_create) WalletCreate(wallet_instance.get(), options.create_flags);

        return wallet_instance;
        */
}

pub fn wallet_show_info(wallet_instance: *mut Wallet)  {
    
    todo!();
        /*
            LOCK(wallet_instance->cs_wallet);

        tfm::format(std::cout, "Wallet info\n===========\n");
        tfm::format(std::cout, "Name: %s\n", wallet_instance->GetName());
        tfm::format(std::cout, "Format: %s\n", wallet_instance->GetDatabase().Format());
        tfm::format(std::cout, "Descriptors: %s\n", wallet_instance->IsWalletFlagSet(WALLET_FLAG_DESCRIPTORS) ? "yes" : "no");
        tfm::format(std::cout, "Encrypted: %s\n", wallet_instance->IsCrypted() ? "yes" : "no");
        tfm::format(std::cout, "HD (hd seed available): %s\n", wallet_instance->IsHDEnabled() ? "yes" : "no");
        tfm::format(std::cout, "Keypool Size: %u\n", wallet_instance->GetKeyPoolSize());
        tfm::format(std::cout, "Transactions: %zu\n", wallet_instance->mapWallet.size());
        tfm::format(std::cout, "Address Book: %zu\n", wallet_instance->m_address_book.size());
        */
}

pub fn execute_wallet_tool_func(
        args:    &ArgsManager,
        command: &String) -> bool {
    
    todo!();
        /*
            if (args.IsArgSet("-format") && command != "createfromdump") {
            tfm::format(std::cerr, "The -format option can only be used with the \"createfromdump\" command.\n");
            return false;
        }
        if (args.IsArgSet("-dumpfile") && command != "dump" && command != "createfromdump") {
            tfm::format(std::cerr, "The -dumpfile option can only be used with the \"dump\" and \"createfromdump\" commands.\n");
            return false;
        }
        if (args.IsArgSet("-descriptors") && command != "create") {
            tfm::format(std::cerr, "The -descriptors option can only be used with the 'create' command.\n");
            return false;
        }
        if (args.IsArgSet("-legacy") && command != "create") {
            tfm::format(std::cerr, "The -legacy option can only be used with the 'create' command.\n");
            return false;
        }
        if (command == "create" && !args.IsArgSet("-wallet")) {
            tfm::format(std::cerr, "Wallet name must be provided when creating a new wallet.\n");
            return false;
        }
        const std::string name = args.GetArg("-wallet", "");
        const fs::path path = fsbridge::AbsPathJoin(GetWalletDir(), fs::PathFromString(name));

        if (command == "create") {
            DatabaseOptions options;
            options.require_create = true;
            // If -legacy is set, use it. Otherwise default to false.
            bool make_legacy = args.GetBoolArg("-legacy", false);
            // If neither -legacy nor -descriptors is set, default to true. If -descriptors is set, use its value.
            bool make_descriptors = (!args.IsArgSet("-descriptors") && !args.IsArgSet("-legacy")) || (args.IsArgSet("-descriptors") && args.GetBoolArg("-descriptors", true));
            if (make_legacy && make_descriptors) {
                tfm::format(std::cerr, "Only one of -legacy or -descriptors can be set to true, not both\n");
                return false;
            }
            if (!make_legacy && !make_descriptors) {
                tfm::format(std::cerr, "One of -legacy or -descriptors must be set to true (or omitted)\n");
                return false;
            }
            if (make_descriptors) {
                options.create_flags |= WALLET_FLAG_DESCRIPTORS;
                options.require_format = DatabaseFormat::SQLITE;
            }

            std::shared_ptr<CWallet> wallet_instance = MakeWallet(name, path, options);
            if (wallet_instance) {
                WalletShowInfo(wallet_instance.get());
                wallet_instance->Close();
            }
        } else if (command == "info") {
            DatabaseOptions options;
            options.require_existing = true;
            std::shared_ptr<CWallet> wallet_instance = MakeWallet(name, path, options);
            if (!wallet_instance) return false;
            WalletShowInfo(wallet_instance.get());
            wallet_instance->Close();
        } else if (command == "salvage") {
    #ifdef USE_BDB
            bilingual_str error;
            std::vector<bilingual_str> warnings;
            bool ret = RecoverDatabaseFile(path, error, warnings);
            if (!ret) {
                for (const auto& warning : warnings) {
                    tfm::format(std::cerr, "%s\n", warning.original);
                }
                if (!error.empty()) {
                    tfm::format(std::cerr, "%s\n", error.original);
                }
            }
            return ret;
    #else
            tfm::format(std::cerr, "Salvage command is not available as BDB support is not compiled");
            return false;
    #endif
        } else if (command == "dump") {
            DatabaseOptions options;
            options.require_existing = true;
            std::shared_ptr<CWallet> wallet_instance = MakeWallet(name, path, options);
            if (!wallet_instance) return false;
            bilingual_str error;
            bool ret = DumpWallet(*wallet_instance, error);
            if (!ret && !error.empty()) {
                tfm::format(std::cerr, "%s\n", error.original);
                return ret;
            }
            tfm::format(std::cout, "The dumpfile may contain private keys. To ensure the safety of your Bitcoin, do not share the dumpfile.\n");
            return ret;
        } else if (command == "createfromdump") {
            bilingual_str error;
            std::vector<bilingual_str> warnings;
            bool ret = CreateFromDump(name, path, error, warnings);
            for (const auto& warning : warnings) {
                tfm::format(std::cout, "%s\n", warning.original);
            }
            if (!ret && !error.empty()) {
                tfm::format(std::cerr, "%s\n", error.original);
            }
            return ret;
        } else {
            tfm::format(std::cerr, "Invalid command: %s\n", command);
            return false;
        }

        return true;
        */
}


//-------------------------------------------[.cpp/bitcoin/src/wallet/walletutil.h]
//-------------------------------------------[.cpp/bitcoin/src/wallet/walletutil.cpp]

/**
  | Get the path of the wallet directory.
  |
  */
pub fn get_wallet_dir() -> Box<Path> {
    
    todo!();
        /*
            fs::path path;

        if (gArgs.IsArgSet("-walletdir")) {
            path = fs::PathFromString(gArgs.GetArg("-walletdir", ""));
            if (!fs::is_directory(path)) {
                // If the path specified doesn't exist, we return the deliberately
                // invalid empty string.
                path = "";
            }
        } else {
            path = gArgs.GetDataDirNet();
            // If a wallets directory exists, use that, otherwise default to GetDataDir
            if (fs::is_directory(path / "wallets")) {
                path /= "wallets";
            }
        }

        return path;
        */
}

pub trait AttachChain {

    fn attach_chain<'a>(
        wallet:          &Arc<Wallet>,
        chain:           &'a mut dyn ChainInterface,
        rescan_required: bool,
        error:           &mut BilingualStr,
        warnings:        &mut Vec<BilingualStr>) -> bool;
}

pub trait Create {

    fn create(
        context:               &mut WalletContext,
        name:                  &String,
        database:              Box<WalletDatabase>,
        wallet_creation_flags: u64,
        error:                 &mut BilingualStr,
        warnings:              &mut Vec<BilingualStr>) -> Arc<Wallet>;
}

/**
  | Construct wallet tx struct.
  |
  */
pub fn make_wallet_tx(
        wallet: &mut Wallet,
        wtx:    &WalletTx) -> WalletTx {
    
    todo!();
        /*
            LOCK(wallet.cs_wallet);
        WalletTx result;
        result.tx = wtx.tx;
        result.txin_is_mine.reserve(wtx.tx->vin.size());
        for (const auto& txin : wtx.tx->vin) {
            result.txin_is_mine.emplace_back(InputIsMine(wallet, txin));
        }
        result.txout_is_mine.reserve(wtx.tx->vout.size());
        result.txout_address.reserve(wtx.tx->vout.size());
        result.txout_address_is_mine.reserve(wtx.tx->vout.size());
        for (const auto& txout : wtx.tx->vout) {
            result.txout_is_mine.emplace_back(wallet.IsMine(txout));
            result.txout_address.emplace_back();
            result.txout_address_is_mine.emplace_back(ExtractDestination(txout.scriptPubKey, result.txout_address.back()) ?
                                                          wallet.IsMine(result.txout_address.back()) :
                                                          ISMINE_NO);
        }
        result.credit = CachedTxGetCredit(wallet, wtx, ISMINE_ALL);
        result.debit = CachedTxGetDebit(wallet, wtx, ISMINE_ALL);
        result.change = CachedTxGetChange(wallet, wtx);
        result.time = wtx.GetTxTime();
        result.value_map = wtx.mapValue;
        result.is_coinbase = wtx.IsCoinBase();
        return result;
        */
}

/**
  | Construct wallet tx status struct.
  |
  */
pub fn make_wallet_tx_status(
        wallet: &Wallet,
        wtx:    &WalletTx) -> WalletTxStatus {
    
    todo!();
        /*
            WalletTxStatus result;
        result.block_height = wtx.m_confirm.block_height > 0 ? wtx.m_confirm.block_height : std::numeric_limits<int>::max();
        result.blocks_to_maturity = wallet.GetTxBlocksToMaturity(wtx);
        result.depth_in_main_chain = wallet.GetTxDepthInMainChain(wtx);
        result.time_received = wtx.nTimeReceived;
        result.lock_time = wtx.tx->nLockTime;
        result.is_final = wallet.chain().checkFinalTx(*wtx.tx);
        result.is_trusted = CachedTxIsTrusted(wallet, wtx);
        result.is_abandoned = wtx.isAbandoned();
        result.is_coinbase = wtx.IsCoinBase();
        result.is_in_main_chain = wallet.IsTxInMainChain(wtx);
        return result;
        */
}

/**
  | Construct wallet TxOut struct.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn make_wallet_tx_out(
        wallet: &Wallet,
        wtx:    &WalletTx,
        n:      i32,
        depth:  i32) -> WalletTxOut {
    
    todo!();
        /*
            WalletTxOut result;
        result.txout = wtx.tx->vout[n];
        result.time = wtx.GetTxTime();
        result.depth_in_main_chain = depth;
        result.is_spent = wallet.IsSpent(wtx.GetHash(), n);
        return result;
        */
}
