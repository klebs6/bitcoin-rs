// ---------------- [ File: bitcoinwallet-library/src/wallet_impl.rs ]
crate::ix!();

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
