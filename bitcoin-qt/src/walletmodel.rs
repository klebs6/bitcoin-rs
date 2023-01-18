crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/walletmodel.h]

/**
  | Interface to Bitcoin wallet from Qt
  | view code.
  |
  */
#[Q_OBJECT]
pub struct WalletModel {
    base:                          QObject,

    wallet:                        Box<dyn WalletInterface>,
    handler_unload:                Box<dyn Handler>,
    handler_status_changed:        Box<dyn Handler>,
    handler_address_book_changed:  Box<dyn Handler>,
    handler_transaction_changed:   Box<dyn Handler>,
    handler_show_progress:         Box<dyn Handler>,
    handler_watch_only_changed:    Box<dyn Handler>,
    handler_can_get_addrs_changed: Box<dyn Handler>,
    client_model:                  *mut ClientModel,
    node:                          Rc<RefCell<dyn NodeInterface>>,
    have_watch_only:               bool,
    force_check_balance_changed:   bool, // default = { false }

    /**
      | Wallet has an options model for wallet-specific
      | options (transaction fee, for example)
      |
      */
    options_model:                 *mut OptionsModel,

    address_table_model:           *mut AddressTableModel,
    transaction_table_model:       *mut TransactionTableModel,
    recent_requests_table_model:   *mut RecentRequestsTableModel,

    /**
      | Cache some values to be able to detect
      | changes
      |
      */
    cached_balances:               WalletBalances,

    cached_encryption_status:      wallet_model::EncryptionStatus,
    timer:                         *mut QTimer,

    /**
      | Block hash denoting when the last balance
      | update was done.
      |
      */
    cached_last_update_tip:        u256,
}

pub mod wallet_model {

    use super::*;

    /**
      | Returned by sendCoins
      |
      */
    pub enum StatusCode 
    {
        OK,
        InvalidAmount,
        InvalidAddress,
        AmountExceedsBalance,
        AmountWithFeeExceedsBalance,
        DuplicateAddress,

        /**
          | Error returned when wallet is still
          | locked
          |
          */
        TransactionCreationFailed, 
        AbsurdFee,
        PaymentRequestExpired
    }

    pub enum EncryptionStatus
    {
        /**
          | !wallet->IsCrypted()
          |
          */
        Unencrypted,  

        /**
          | wallet->IsCrypted() && wallet->IsLocked()
          |
          */
        Locked,       

        /**
          | wallet->IsCrypted() && !wallet->IsLocked()
          |
          */
        Unlocked      
    }

    /**
      | Return status record for SendCoins,
      | contains error id + information
      |
      */
    pub struct SendCoinsReturn {
        status:               StatusCode,
        reason_commit_failed: String,
    }

    impl SendCoinsReturn {

        pub fn new(
            status:               Option<StatusCode>,
            reason_commit_failed: Option<&str>) -> Self {

            let status: StatusCode = status.unwrap_or(StatusCode::OK);

            let reason_commit_failed: &str =
                     reason_commit_failed.unwrap_or("");

            todo!();
            /*


                : status(_status),
                      reasonCommitFailed(_reasonCommitFailed)
            */
        }
    }

    /**
      | RAI object for unlocking wallet, returned
      | by requestUnlock()
      |
      */
    pub struct UnlockContext {
        wallet: *mut WalletModel,
        valid:  bool,

        /**
          | mutable, as it can be set to false by copying
          |
          */
        relock: RefCell<bool>,
    }

    impl Drop for UnlockContext {
        fn drop(&mut self) {
            todo!();
            /*
                if(valid && relock)
            {
                wallet->setWalletLocked(true);
            }
            */
        }
    }

    impl UnlockContext {
        
        pub fn is_valid(&self) -> bool {
            
            todo!();
            /*
                return valid;
            */
        }

        /**
          | Move operator and constructor transfer
          | the context
          |
          */
        pub fn new_from_other(obj: UnlockContext) -> Self {
        
            todo!();
            /*
                CopyFrom(std::move(obj));
            */
        }
        
        pub fn assign_from(&mut self, rhs: UnlockContext) -> &mut UnlockContext {
            
            todo!();
            /*
                CopyFrom(std::move(rhs)); return *this;
            */
        }
        
        pub fn new(
            wallet: *mut WalletModel,
            valid:  bool,
            relock: bool) -> Self {
        
            todo!();
            /*
                :
                wallet(_wallet),
                valid(_valid),
                relock(_relock)
            */
        }
        
        pub fn copy_from(&mut self, rhs: UnlockContext)  {
            
            todo!();
            /*
                // Transfer context; old object no longer relocks wallet
            *this = rhs;
            rhs.relock = false;
            */
        }
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/walletmodel.cpp]
impl Drop for WalletModel {
    fn drop(&mut self) {
        todo!();
        /*
            unsubscribeFromCoreSignals();
        */
    }
}

impl WalletModel {

    pub fn node(&self) -> Rc<RefCell<dyn NodeInterface>> {
        
        todo!();
        /*
            return m_node;
        */
    }
    
    pub fn wallet(&self) -> Rc<RefCell<dyn WalletInterface>> {
        
        todo!();
        /*
            return *m_wallet;
        */
    }
    
    pub fn client_model(&self) -> &mut ClientModel {
        
        todo!();
        /*
            return *m_client_model;
        */
    }
    
    /**
      | Signal that balance in wallet changed
      |
      */
    #[Q_SIGNAL]
    pub fn balance_changed(&mut self, balances: &WalletBalances)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Encryption status of wallet changed
      |
      */
    #[Q_SIGNAL]
    pub fn encryption_status_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Signal emitted when wallet needs to be
      | unlocked
      |
      | It is valid behaviour for listeners to keep
      | the wallet locked after this signal; this
      | means that the unlocking failed or was
      | cancelled.
      */
    #[Q_SIGNAL]
    pub fn require_unlock(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Fired when a message should be reported
      | to the user
      |
      */
    #[Q_SIGNAL]
    pub fn message(&mut self, 
        title:   &String,
        message: &String,
        style:   u32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Coins sent: from wallet, to recipient,
      | in (serialized) transaction:
      |
      */
    #[Q_SIGNAL]
    pub fn coins_sent(&mut self, 
        wallet:      *mut WalletModel,
        recipient:   SendCoinsRecipient,
        transaction: QByteArray)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Show progress dialog e.g. for rescan
      |
      */
    #[Q_SIGNAL]
    pub fn show_progress(&mut self, 
        title:      &String,
        n_progress: i32)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Watch-only address added
      |
      */
    #[Q_SIGNAL]
    pub fn notify_watchonly_changed(&mut self, have_watchonly: bool)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Signal that wallet is about to be removed
      |
      */
    #[Q_SIGNAL]
    pub fn unload(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Notify that there are now keys in the
      | keypool
      |
      */
    #[Q_SIGNAL]
    pub fn can_get_addresses_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn timer_timeout(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    pub fn new(
        wallet:         Box<dyn WalletInterface>,
        client_model:   &mut ClientModel,
        platform_style: *const PlatformStyle,
        parent:         *mut QObject) -> Self {
    
        todo!();
        /*


            :
        QObject(parent),
        m_wallet(std::move(wallet)),
        m_client_model(&client_model),
        m_node(client_model.node()),
        optionsModel(client_model.getOptionsModel()),
        addressTableModel(nullptr),
        transactionTableModel(nullptr),
        recentRequestsTableModel(nullptr),
        cachedEncryptionStatus(Unencrypted),
        timer(new QTimer(this))
        fHaveWatchOnly = m_wallet->haveWatchOnly();
        addressTableModel = new AddressTableModel(this);
        transactionTableModel = new TransactionTableModel(platformStyle, this);
        recentRequestsTableModel = new RecentRequestsTableModel(this);

        subscribeToCoreSignals();
        */
    }
    
    /**
      | Starts a timer to periodically update
      | the balance
      |
      */
    #[Q_SLOT]
    pub fn start_poll_balance(&mut self)  {
        
        todo!();
        /*
            // This timer will be fired repeatedly to update the balance
        // Since the QTimer::timeout is a private signal, it cannot be used
        // in the typename gui_util::ExceptionSafeConnect directly.
        connect(timer, &QTimer::timeout, this, &WalletModel::timerTimeout);
        typename gui_util::ExceptionSafeConnect(this, &WalletModel::timerTimeout, this, &WalletModel::pollBalanceChanged);
        timer->start(MODEL_UPDATE_DELAY);
        */
    }
    
    pub fn set_client_model(&mut self, client_model: *mut ClientModel)  {
        
        todo!();
        /*
            m_client_model = client_model;
        if (!m_client_model) timer->stop();
        */
    }
    
    /**
      | Wallet status might have changed
      |
      */
    #[Q_SLOT]
    pub fn update_status(&mut self)  {
        
        todo!();
        /*
            EncryptionStatus newEncryptionStatus = getEncryptionStatus();

        if(cachedEncryptionStatus != newEncryptionStatus) {
            Q_EMIT encryptionStatusChanged();
        }
        */
    }
    
    /**
      | Current, immature or unconfirmed balance
      | might have changed - emit 'balanceChanged'
      | if so
      |
      */
    #[Q_SLOT]
    pub fn poll_balance_changed(&mut self)  {
        
        todo!();
        /*
            // Avoid recomputing wallet balances unless a TransactionChanged or
        // BlockTip notification was received.
        if (!fForceCheckBalanceChanged && m_cached_last_update_tip == getLastBlockProcessed()) return;

        // Try to get balances and return early if locks can't be acquired. This
        // avoids the GUI from getting stuck on periodical polls if the core is
        // holding the locks for a longer time - for example, during a wallet
        // rescan.
        typename interfaces::WalletBalances new_balances;
        uint256 block_hash;
        if (!m_wallet->tryGetBalances(new_balances, block_hash)) {
            return;
        }

        if (fForceCheckBalanceChanged || block_hash != m_cached_last_update_tip) {
            fForceCheckBalanceChanged = false;

            // Balance and number of transactions might have changed
            m_cached_last_update_tip = block_hash;

            checkBalanceChanged(new_balances);
            if(transactionTableModel)
                transactionTableModel->updateConfirmations();
        }
        */
    }
    
    pub fn check_balance_changed(&mut self, new_balances: &WalletBalances)  {
        
        todo!();
        /*
            if(new_balances.balanceChanged(m_cached_balances)) {
            m_cached_balances = new_balances;
            Q_EMIT balanceChanged(new_balances);
        }
        */
    }
    
    /**
      | New transaction, or transaction changed
      | status
      |
      */
    #[Q_SLOT]
    pub fn update_transaction(&mut self)  {
        
        todo!();
        /*
            // Balance and number of transactions might have changed
        fForceCheckBalanceChanged = true;
        */
    }
    
    /**
      | New, updated or removed address book
      | entry
      |
      */
    #[Q_SLOT]
    pub fn update_address_book(&mut self, 
        address: &String,
        label:   &String,
        is_mine: bool,
        purpose: &String,
        status:  i32)  {
        
        todo!();
        /*
            if(addressTableModel)
            addressTableModel->updateEntry(address, label, isMine, purpose, status);
        */
    }
    
    /**
      | Watch-only added
      |
      */
    #[Q_SLOT]
    pub fn update_watch_only_flag(&mut self, have_watchonly: bool)  {
        
        todo!();
        /*
            fHaveWatchOnly = fHaveWatchonly;
        Q_EMIT notifyWatchonlyChanged(fHaveWatchonly);
        */
    }
    
    /**
      | Check address for validity
      |
      */
    pub fn validate_address(&mut self, address: &String) -> bool {
        
        todo!();
        /*
            return IsValidDestinationString(address.toStdString());
        */
    }
    
    /**
      | prepare transaction for getting txfee
      | before sending coins
      |
      */
    pub fn prepare_transaction(&mut self, 
        transaction:  &mut WalletModelTransaction,
        coin_control: &CoinControl) -> wallet_model::SendCoinsReturn {
        
        todo!();
        /*
            CAmount total = 0;
        bool fSubtractFeeFromAmount = false;
        QList<SendCoinsRecipient> recipients = transaction.getRecipients();
        std::vector<CRecipient> vecSend;

        if(recipients.empty())
        {
            return OK;
        }

        QSet<QString> setAddress; // Used to detect duplicates
        int nAddresses = 0;

        // Pre-check input data for validity
        for (const SendCoinsRecipient &rcp : recipients)
        {
            if (rcp.fSubtractFeeFromAmount)
                fSubtractFeeFromAmount = true;
            {   // User-entered bitcoin address / amount:
                if(!validateAddress(rcp.address))
                {
                    return InvalidAddress;
                }
                if(rcp.amount <= 0)
                {
                    return InvalidAmount;
                }
                setAddress.insert(rcp.address);
                ++nAddresses;

                CScript scriptPubKey = GetScriptForDestination(DecodeDestination(rcp.address.toStdString()));
                CRecipient recipient = {scriptPubKey, rcp.amount, rcp.fSubtractFeeFromAmount};
                vecSend.push_back(recipient);

                total += rcp.amount;
            }
        }
        if(setAddress.size() != nAddresses)
        {
            return DuplicateAddress;
        }

        CAmount nBalance = m_wallet->getAvailableBalance(coinControl);

        if(total > nBalance)
        {
            return AmountExceedsBalance;
        }

        {
            CAmount nFeeRequired = 0;
            int nChangePosRet = -1;
            bilingual_str error;

            auto& newTx = transaction.getWtx();
            newTx = m_wallet->createTransaction(vecSend, coinControl, !wallet().privateKeysDisabled() /* sign */, nChangePosRet, nFeeRequired, error);
            transaction.setTransactionFee(nFeeRequired);
            if (fSubtractFeeFromAmount && newTx)
                transaction.reassignAmounts(nChangePosRet);

            if(!newTx)
            {
                if(!fSubtractFeeFromAmount && (total + nFeeRequired) > nBalance)
                {
                    return SendCoinsReturn(AmountWithFeeExceedsBalance);
                }
                Q_EMIT message(tr("Send Coins"), QString::fromStdString(error.translated),
                    CClientUIInterface::MSG_ERROR);
                return TransactionCreationFailed;
            }

            // Reject absurdly high fee. (This can never happen because the
            // wallet never creates transactions with fee greater than
            // m_default_max_tx_fee. This merely a belt-and-suspenders check).
            if (nFeeRequired > m_wallet->getDefaultMaxTxFee()) {
                return AbsurdFee;
            }
        }

        return SendCoinsReturn(OK);
        */
    }
    
    /**
      | Send coins to a list of recipients
      |
      */
    pub fn send_coins(&mut self, transaction: &mut WalletModelTransaction) -> wallet_model::SendCoinsReturn {
        
        todo!();
        /*
            QByteArray transaction_array; /* store serialized transaction */

        {
            std::vector<std::pair<std::string, std::string>> vOrderForm;
            for (const SendCoinsRecipient &rcp : transaction.getRecipients())
            {
                if (!rcp.message.isEmpty()) // Message from normal bitcoin:URI (bitcoin:123...?message=example)
                    vOrderForm.emplace_back("Message", rcp.message.toStdString());
            }

            auto& newTx = transaction.getWtx();
            wallet().commitTransaction(newTx, {} /* mapValue */, std::move(vOrderForm));

            DataStream ssTx(SER_NETWORK, PROTOCOL_VERSION);
            ssTx << *newTx;
            transaction_array.append((const char*)ssTx.data(), ssTx.size());
        }

        // Add addresses / update labels that we've sent to the address book,
        // and emit coinsSent signal for each recipient
        for (const SendCoinsRecipient &rcp : transaction.getRecipients())
        {
            {
                std::string strAddress = rcp.address.toStdString();
                TxDestination dest = DecodeDestination(strAddress);
                std::string strLabel = rcp.label.toStdString();
                {
                    // Check if we have a new address or an updated label
                    std::string name;
                    if (!m_wallet->getAddress(
                         dest, &name, /* is_mine= */ nullptr, /* purpose= */ nullptr))
                    {
                        m_wallet->setAddressBook(dest, strLabel, "send");
                    }
                    else if (name != strLabel)
                    {
                        m_wallet->setAddressBook(dest, strLabel, ""); // "" means don't change purpose
                    }
                }
            }
            Q_EMIT coinsSent(this, rcp, transaction_array);
        }

        checkBalanceChanged(m_wallet->getBalances()); // update balance immediately, otherwise there could be a short noticeable delay until pollBalanceChanged hits

        return SendCoinsReturn(OK);
        */
    }
    
    pub fn get_options_model(&mut self) -> *mut OptionsModel {
        
        todo!();
        /*
            return optionsModel;
        */
    }
    
    pub fn get_address_table_model(&mut self) -> *mut AddressTableModel {
        
        todo!();
        /*
            return addressTableModel;
        */
    }
    
    pub fn get_transaction_table_model(&mut self) -> *mut TransactionTableModel {
        
        todo!();
        /*
            return transactionTableModel;
        */
    }
    
    pub fn get_recent_requests_table_model(&mut self) -> *mut RecentRequestsTableModel {
        
        todo!();
        /*
            return recentRequestsTableModel;
        */
    }
    
    pub fn get_encryption_status(&self) -> wallet_model::EncryptionStatus {
        
        todo!();
        /*
            if(!m_wallet->isCrypted())
        {
            return Unencrypted;
        }
        else if(m_wallet->isLocked())
        {
            return Locked;
        }
        else
        {
            return Unlocked;
        }
        */
    }
    
    /**
      | Wallet encryption
      |
      */
    pub fn set_wallet_encrypted(&mut self, passphrase: &SecureString) -> bool {
        
        todo!();
        /*
            return m_wallet->encryptWallet(passphrase);
        */
    }
    
    /**
      | Passphrase only needed when unlocking
      |
      */
    pub fn set_wallet_locked(&mut self, 
        locked:      bool,
        pass_phrase: Option<&SecureString>) -> bool {

        let pass_phrase: &SecureString = 
            pass_phrase.unwrap_or(&SecureString::default());
        
        todo!();
        /*
            if(locked)
        {
            // Lock
            return m_wallet->lock();
        }
        else
        {
            // Unlock
            return m_wallet->unlock(passPhrase);
        }
        */
    }
    
    pub fn change_passphrase(&mut self, 
        old_pass: &SecureString,
        new_pass: &SecureString) -> bool {
        
        todo!();
        /*
            m_wallet->lock(); // Make sure wallet is locked before attempting pass change
        return m_wallet->changeWalletPassphrase(oldPass, newPass);
        */
    }

    pub fn subscribe_to_core_signals(&mut self)  {
        
        todo!();
        /*
            // Connect signals to wallet
        m_handler_unload = m_wallet->handleUnload(std::bind(&NotifyUnload, this));
        m_handler_status_changed = m_wallet->handleStatusChanged(std::bind(&NotifyKeyStoreStatusChanged, this));
        m_handler_address_book_changed = m_wallet->handleAddressBookChanged(std::bind(NotifyAddressBookChanged, this, std::placeholders::_1, std::placeholders::_2, std::placeholders::_3, std::placeholders::_4, std::placeholders::_5));
        m_handler_transaction_changed = m_wallet->handleTransactionChanged(std::bind(NotifyTransactionChanged, this, std::placeholders::_1, std::placeholders::_2));
        m_handler_show_progress = m_wallet->handleShowProgress(std::bind(ShowProgress, this, std::placeholders::_1, std::placeholders::_2));
        m_handler_watch_only_changed = m_wallet->handleWatchOnlyChanged(std::bind(NotifyWatchonlyChanged, this, std::placeholders::_1));
        m_handler_can_get_addrs_changed = m_wallet->handleCanGetAddressesChanged(std::bind(NotifyCanGetAddressesChanged, this));
        */
    }
    
    pub fn unsubscribe_from_core_signals(&mut self)  {
        
        todo!();
        /*
            // Disconnect signals from wallet
        m_handler_unload->disconnect();
        m_handler_status_changed->disconnect();
        m_handler_address_book_changed->disconnect();
        m_handler_transaction_changed->disconnect();
        m_handler_show_progress->disconnect();
        m_handler_watch_only_changed->disconnect();
        m_handler_can_get_addrs_changed->disconnect();
        */
    }

    /**
      | WalletModel::UnlockContext implementation
      |
      */
    pub fn request_unlock(&mut self) -> wallet_model::UnlockContext {
        
        todo!();
        /*
            bool was_locked = getEncryptionStatus() == Locked;
        if(was_locked)
        {
            // Request UI to unlock wallet
            Q_EMIT requireUnlock();
        }
        // If wallet is still locked, unlock was failed or cancelled, mark context as invalid
        bool valid = getEncryptionStatus() != Locked;

        return UnlockContext(this, valid, was_locked);
        */
    }
    
    pub fn bump_fee(&mut self, 
        hash:     u256,
        new_hash: &mut u256) -> bool {
        
        todo!();
        /*
            CCoinControl coin_control;
        coin_control.m_signal_bip125_rbf = true;
        std::vector<bilingual_str> errors;
        CAmount old_fee;
        CAmount new_fee;
        CMutableTransaction mtx;
        if (!m_wallet->createBumpTransaction(hash, coin_control, errors, old_fee, new_fee, mtx)) {
            QMessageBox::critical(nullptr, tr("Fee bump error"), tr("Increasing transaction fee failed") + "<br />(" +
                (errors.size() ? QString::fromStdString(errors[0].translated) : "") +")");
            return false;
        }

        const bool create_psbt = m_wallet->privateKeysDisabled();

        // allow a user based fee verification
        QString questionString = create_psbt ? tr("Do you want to draft a transaction with fee increase?") : tr("Do you want to increase the fee?");
        questionString.append("<br />");
        questionString.append("<table style=\"text-align: left;\">");
        questionString.append("<tr><td>");
        questionString.append(tr("Current fee:"));
        questionString.append("</td><td>");
        questionString.append(BitcoinUnits::formatHtmlWithUnit(getOptionsModel()->getDisplayUnit(), old_fee));
        questionString.append("</td></tr><tr><td>");
        questionString.append(tr("Increase:"));
        questionString.append("</td><td>");
        questionString.append(BitcoinUnits::formatHtmlWithUnit(getOptionsModel()->getDisplayUnit(), new_fee - old_fee));
        questionString.append("</td></tr><tr><td>");
        questionString.append(tr("New fee:"));
        questionString.append("</td><td>");
        questionString.append(BitcoinUnits::formatHtmlWithUnit(getOptionsModel()->getDisplayUnit(), new_fee));
        questionString.append("</td></tr></table>");

        // Display warning in the "Confirm fee bump" window if the "Coin Control Features" option is enabled
        if (getOptionsModel()->getCoinControlFeatures()) {
            questionString.append("<br><br>");
            questionString.append(tr("Warning: This may pay the additional fee by reducing change outputs or adding inputs, when necessary. It may add a new change output if one does not already exist. These changes may potentially leak privacy."));
        }

        auto confirmationDialog = new SendConfirmationDialog(tr("Confirm fee bump"), questionString);
        confirmationDialog->setAttribute(QtWA_DeleteOnClose);
        // TODO: Replace QDialog::exec() with safer QDialog::show().
        const auto retval = static_cast<QMessageBox::StandardButton>(confirmationDialog->exec());

        // cancel sign&broadcast if user doesn't want to bump the fee
        if (retval != QMessageBox::Yes) {
            return false;
        }

        WalletModel::UnlockContext ctx(requestUnlock());
        if(!ctx.isValid())
        {
            return false;
        }

        // Short-circuit if we are returning a bumped transaction PSBT to clipboard
        if (create_psbt) {
            PartiallySignedTransaction psbtx(mtx);
            bool complete = false;
            const TransactionError err = wallet().fillPSBT(SIGHASH_ALL, false /* sign */, true /* bip32derivs */, nullptr, psbtx, complete);
            if (err != TransactionError::OK || complete) {
                QMessageBox::critical(nullptr, tr("Fee bump error"), tr("Can't draft transaction."));
                return false;
            }
            // Serialize the PSBT
            DataStream ssTx(SER_NETWORK, PROTOCOL_VERSION);
            ssTx << psbtx;
            typename gui_util::setClipboard(EncodeBase64(ssTx.str()).c_str());
            Q_EMIT message(tr("PSBT copied"), "Copied to clipboard", CClientUIInterface::MSG_INFORMATION);
            return true;
        }

        // sign bumped transaction
        if (!m_wallet->signBumpTransaction(mtx)) {
            QMessageBox::critical(nullptr, tr("Fee bump error"), tr("Can't sign transaction."));
            return false;
        }
        // commit the bumped transaction
        if(!m_wallet->commitBumpTransaction(hash, std::move(mtx), errors, new_hash)) {
            QMessageBox::critical(nullptr, tr("Fee bump error"), tr("Could not commit transaction") + "<br />(" +
                QString::fromStdString(errors[0].translated)+")");
            return false;
        }
        return true;
        */
    }
    
    pub fn display_address(&mut self, address: String) -> bool {
        
        todo!();
        /*
            TxDestination dest = DecodeDestination(sAddress);
        bool res = false;
        try {
            res = m_wallet->displayAddress(dest);
        } catch (const std::runtime_error& e) {
            QMessageBox::critical(nullptr, tr("Can't display address"), e.what());
        }
        return res;
        */
    }
    
    pub fn is_wallet_enabled(&mut self) -> bool {
        
        todo!();
        /*
            return !gArgs.GetBoolArg("-disablewallet", DEFAULT_DISABLE_WALLET);
        */
    }
    
    pub fn get_wallet_name(&self) -> String {
        
        todo!();
        /*
            return QString::fromStdString(m_wallet->getWalletName());
        */
    }
    
    pub fn get_display_name(&self) -> String {
        
        todo!();
        /*
            const QString name = getWalletName();
        return name.isEmpty() ? "["+tr("default wallet")+"]" : name;
        */
    }
    
    pub fn is_multiwallet(&mut self) -> bool {
        
        todo!();
        /*
            return m_node.walletClient().getWallets().size() > 1;
        */
    }
    
    pub fn refresh(&mut self, pk_hash_only: Option<bool>)  {
        
        let pk_hash_only: bool = pk_hash_only.unwrap_or(false);

        todo!();
        /*
            addressTableModel = new AddressTableModel(this, pk_hash_only);
        */
    }
    
    pub fn get_last_block_processed(&self) -> u256 {
        
        todo!();
        /*
            return m_client_model ? m_client_model->getBestBlockHash() : uint256{};
        */
    }
}

/**
  | Handlers for core signals
  |
  */
pub fn notify_unload(wallet_model: *mut WalletModel)  {
    
    todo!();
        /*
            qDebug() << "NotifyUnload";
        bool invoked = QMetaObject::invokeMethod(walletModel, "unload");
        assert(invoked);
        */
}

pub fn notify_key_store_status_changed(walletmodel: *mut WalletModel)  {
    
    todo!();
        /*
            qDebug() << "NotifyKeyStoreStatusChanged";
        bool invoked = QMetaObject::invokeMethod(walletmodel, "updateStatus", QtQueuedConnection);
        assert(invoked);
        */
}

pub fn notify_address_book_changed(
        walletmodel: *mut WalletModel,
        address:     &TxDestination,
        label:       &String,
        is_mine:     bool,
        purpose:     &String,
        status:      ChangeType)  {
    
    todo!();
        /*
            QString strAddress = QString::fromStdString(EncodeDestination(address));
        QString strLabel = QString::fromStdString(label);
        QString strPurpose = QString::fromStdString(purpose);

        qDebug() << "NotifyAddressBookChanged: " + strAddress + " " + strLabel + " isMine=" + QString::number(isMine) + " purpose=" + strPurpose + " status=" + QString::number(status);
        bool invoked = QMetaObject::invokeMethod(walletmodel, "updateAddressBook", QtQueuedConnection,
                                  Q_ARG(QString, strAddress),
                                  Q_ARG(QString, strLabel),
                                  Q_ARG(bool, isMine),
                                  Q_ARG(QString, strPurpose),
                                  Q_ARG(int, status));
        assert(invoked);
        */
}

pub fn notify_transaction_changed(
        walletmodel: *mut WalletModel,
        hash:        &u256,
        status:      ChangeType)  {
    
    todo!();
        /*
            Q_UNUSED(hash);
        Q_UNUSED(status);
        bool invoked = QMetaObject::invokeMethod(walletmodel, "updateTransaction", QtQueuedConnection);
        assert(invoked);
        */
}

pub fn show_progress(
        walletmodel: *mut WalletModel,
        title:       &String,
        n_progress:  i32)  {
    
    todo!();
        /*
            // emits signal "showProgress"
        bool invoked = QMetaObject::invokeMethod(walletmodel, "showProgress", QtQueuedConnection,
                                  Q_ARG(QString, QString::fromStdString(title)),
                                  Q_ARG(int, nProgress));
        assert(invoked);
        */
}

pub fn notify_watchonly_changed(
        walletmodel:    *mut WalletModel,
        have_watchonly: bool)  {
    
    todo!();
        /*
            bool invoked = QMetaObject::invokeMethod(walletmodel, "updateWatchOnlyFlag", QtQueuedConnection,
                                  Q_ARG(bool, fHaveWatchonly));
        assert(invoked);
        */
}

pub fn notify_can_get_addresses_changed(walletmodel: *mut WalletModel)  {
    
    todo!();
        /*
            bool invoked = QMetaObject::invokeMethod(walletmodel, "canGetAddressesChanged");
        assert(invoked);
        */
}
