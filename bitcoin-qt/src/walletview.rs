crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/walletview.h]

/**
  | WalletView class. This class represents
  | the view to a single wallet.
  | 
  | It was added to support multiple wallet
  | functionality. Each wallet gets its
  | own
  | 
  | WalletView instance.
  | 
  | It communicates with both the client
  | and the wallet models to give the user
  | an up-to-date view of the current core
  | state.
  |
  */
#[Q_OBJECT]
pub struct WalletView {
    base:                          QStackedWidget,

    client_model:                  *mut ClientModel,

    /**
      | The wallet model represents a bitcoin
      | wallet, and offers access to the list of
      | transactions, address book and sending
      | functionality.
      */
    wallet_model:                  *const WalletModel,

    overview_page:                 *mut OverviewPage,
    transactions_page:             *mut QWidget,
    receive_coins_page:            *mut ReceiveCoinsDialog,
    send_coins_page:               *mut SendCoinsDialog,
    used_sending_addresses_page:   *mut AddressBookPage,
    used_receiving_addresses_page: *mut AddressBookPage,
    transaction_view:              *mut TransactionView,
    progress_dialog:               *mut QProgressDialog, // default = { nullptr }
    platform_style:                *const PlatformStyle,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/walletview.cpp]
impl WalletView {
    
    pub fn get_wallet_model(&self) -> *mut WalletModel {
        
        todo!();
        /*
            return walletModel;
        */
    }
    
    #[Q_SIGNAL]
    pub fn set_privacy(&mut self, privacy: bool)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn transaction_clicked(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn coins_sent(&mut self)  {
        
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
      | Notify that a new transaction appeared
      |
      */
    #[Q_SIGNAL]
    pub fn incoming_transaction(&mut self, 
        date:        &String,
        unit:        i32,
        amount:      &Amount,
        ty:          &String,
        address:     &String,
        label:       &String,
        wallet_name: &String)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Notify that the out of sync warning icon
      | has been pressed
      |
      */
    #[Q_SIGNAL]
    pub fn out_of_sync_warning_clicked(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(
        wallet_model:   *mut WalletModel,
        platform_style: *const PlatformStyle,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*


            : QStackedWidget(parent),
          clientModel(nullptr),
          walletModel(wallet_model),
          platformStyle(_platformStyle)

        assert(walletModel);

        // Create tabs
        overviewPage = new OverviewPage(platformStyle);
        overviewPage->setWalletModel(walletModel);

        transactionsPage = new QWidget(this);
        QVBoxLayout *vbox = new QVBoxLayout();
        QHBoxLayout *hbox_buttons = new QHBoxLayout();
        transactionView = new TransactionView(platformStyle, this);
        transactionView->setModel(walletModel);

        vbox->addWidget(transactionView);
        QPushButton *exportButton = new QPushButton(tr("&Export"), this);
        exportButton->setToolTip(tr("Export the data in the current tab to a file"));
        if (platformStyle->getImagesOnButtons()) {
            exportButton->setIcon(platformStyle->SingleColorIcon(":/icons/export"));
        }
        hbox_buttons->addStretch();
        hbox_buttons->addWidget(exportButton);
        vbox->addLayout(hbox_buttons);
        transactionsPage->setLayout(vbox);

        receiveCoinsPage = new ReceiveCoinsDialog(platformStyle);
        receiveCoinsPage->setModel(walletModel);

        sendCoinsPage = new SendCoinsDialog(platformStyle);
        sendCoinsPage->setModel(walletModel);

        usedSendingAddressesPage = new AddressBookPage(platformStyle, AddressBookPage::ForEditing, AddressBookPage::SendingTab, this);
        usedSendingAddressesPage->setModel(walletModel->getAddressTableModel());

        usedReceivingAddressesPage = new AddressBookPage(platformStyle, AddressBookPage::ForEditing, AddressBookPage::ReceivingTab, this);
        usedReceivingAddressesPage->setModel(walletModel->getAddressTableModel());

        addWidget(overviewPage);
        addWidget(transactionsPage);
        addWidget(receiveCoinsPage);
        addWidget(sendCoinsPage);

        connect(overviewPage, &OverviewPage::transactionClicked, this, &WalletView::transactionClicked);
        // Clicking on a transaction on the overview pre-selects the transaction on the transaction history page
        connect(overviewPage, &OverviewPage::transactionClicked, transactionView, qOverload<const QModelIndex&>(&TransactionView::focusTransaction));

        connect(overviewPage, &OverviewPage::outOfSyncWarningClicked, this, &WalletView::outOfSyncWarningClicked);

        connect(sendCoinsPage, &SendCoinsDialog::coinsSent, this, &WalletView::coinsSent);
        // Highlight transaction after send
        connect(sendCoinsPage, &SendCoinsDialog::coinsSent, transactionView, qOverload<const uint256&>(&TransactionView::focusTransaction));

        // Clicking on "Export" allows to export the transaction list
        connect(exportButton, &QPushButton::clicked, transactionView, &TransactionView::exportClicked);

        // Pass through messages from sendCoinsPage
        connect(sendCoinsPage, &SendCoinsDialog::message, this, &WalletView::message);
        // Pass through messages from transactionView
        connect(transactionView, &TransactionView::message, this, &WalletView::message);

        connect(this, &WalletView::setPrivacy, overviewPage, &OverviewPage::setPrivacy);

        // Receive and pass through messages from wallet model
        connect(walletModel, &WalletModel::message, this, &WalletView::message);

        // Handle changes in encryption status
        connect(walletModel, &WalletModel::encryptionStatusChanged, this, &WalletView::encryptionStatusChanged);

        // Balloon pop-up for new transaction
        connect(walletModel->getTransactionTableModel(), &TransactionTableModel::rowsInserted, this, &WalletView::processNewTransaction);

        // Ask for passphrase if needed
        connect(walletModel, &WalletModel::requireUnlock, this, &WalletView::unlockWallet);

        // Show progress dialog
        connect(walletModel, &WalletModel::showProgress, this, &WalletView::showProgress);
        */
    }
    
    /**
      | Set the client model.
      | 
      | The client model represents the part
      | of the core that communicates with the
      | P2P network, and is wallet-agnostic.
      |
      */
    pub fn set_client_model(&mut self, client_model: *mut ClientModel)  {
        
        todo!();
        /*
            this->clientModel = _clientModel;

        overviewPage->setClientModel(_clientModel);
        sendCoinsPage->setClientModel(_clientModel);
        walletModel->setClientModel(_clientModel);
        */
    }
    
    /**
      | Show incoming transaction notification
      | for new transactions.
      | 
      | The new items are those between start
      | and end inclusive, under the given parent
      | item.
      |
      */
    #[Q_SLOT]
    pub fn process_new_transaction(&mut self, 
        parent: &QModelIndex,
        start:  i32,
        end:    i32)  {
        
        todo!();
        /*
            // Prevent balloon-spam when initial block download is in progress
        if (!clientModel || clientModel->node().isInitialBlockDownload()) {
            return;
        }

        TransactionTableModel *ttm = walletModel->getTransactionTableModel();
        if (!ttm || ttm->processingQueuedTransactions())
            return;

        QString date = ttm->index(start, TransactionTableModel::Date, parent).data().toString();
        i64 amount = ttm->index(start, TransactionTableModel::Amount, parent).data(QtEditRole).toULongLong();
        QString type = ttm->index(start, TransactionTableModel::Type, parent).data().toString();
        QModelIndex index = ttm->index(start, 0, parent);
        QString address = ttm->data(index, TransactionTableModel::AddressRole).toString();
        QString label = typename gui_util::HtmlEscape(ttm->data(index, TransactionTableModel::LabelRole).toString());

        Q_EMIT incomingTransaction(date, walletModel->getOptionsModel()->getDisplayUnit(), amount, type, address, label, typename gui_util::HtmlEscape(walletModel->getWalletName()));
        */
    }
    
    /**
      | Switch to overview (home) page
      |
      */
    #[Q_SLOT]
    pub fn goto_overview_page(&mut self)  {
        
        todo!();
        /*
            setCurrentWidget(overviewPage);
        */
    }
    
    /**
      | Switch to history (transactions) page
      |
      */
    #[Q_SLOT]
    pub fn goto_history_page(&mut self)  {
        
        todo!();
        /*
            setCurrentWidget(transactionsPage);
        */
    }
    
    /**
      | Switch to receive coins page
      |
      */
    #[Q_SLOT]
    pub fn goto_receive_coins_page(&mut self)  {
        
        todo!();
        /*
            setCurrentWidget(receiveCoinsPage);
        */
    }
    
    /**
      | Switch to send coins page
      |
      */
    #[Q_SLOT]
    pub fn goto_send_coins_page(&mut self, addr: Option<&str>)  {

        let addr: &str = addr.unwrap_or("");
        
        todo!();
        /*
            setCurrentWidget(sendCoinsPage);

        if (!addr.isEmpty())
            sendCoinsPage->setAddress(addr);
        */
    }
    
    /**
      | Show Sign/Verify Message dialog and
      | switch to sign message tab
      |
      */
    #[Q_SLOT]
    pub fn goto_sign_message_tab(&mut self, addr: Option<&str>)  {
        let addr: &str = addr.unwrap_or("");
        
        todo!();
        /*
            // calls show() in showTab_SM()
        SignVerifyMessageDialog *signVerifyMessageDialog = new SignVerifyMessageDialog(platformStyle, this);
        signVerifyMessageDialog->setAttribute(QtWA_DeleteOnClose);
        signVerifyMessageDialog->setModel(walletModel);
        signVerifyMessageDialog->showTab_SM(true);

        if (!addr.isEmpty())
            signVerifyMessageDialog->setAddress_SM(addr);
        */
    }
    
    /**
      | Show Sign/Verify Message dialog and
      | switch to verify message tab
      |
      */
    #[Q_SLOT]
    pub fn goto_verify_message_tab(&mut self, addr: Option<&str>)  {
        let addr: &str = addr.unwrap_or("");
        
        todo!();
        /*
            // calls show() in showTab_VM()
        SignVerifyMessageDialog *signVerifyMessageDialog = new SignVerifyMessageDialog(platformStyle, this);
        signVerifyMessageDialog->setAttribute(QtWA_DeleteOnClose);
        signVerifyMessageDialog->setModel(walletModel);
        signVerifyMessageDialog->showTab_VM(true);

        if (!addr.isEmpty())
            signVerifyMessageDialog->setAddress_VM(addr);
        */
    }
    
    pub fn handle_payment_request(&mut self, recipient: &SendCoinsRecipient) -> bool {
        
        todo!();
        /*
            return sendCoinsPage->handlePaymentRequest(recipient);
        */
    }
    
    pub fn show_out_of_sync_warning(&mut self, show: bool)  {
        
        todo!();
        /*
            overviewPage->showOutOfSyncWarning(fShow);
        */
    }
    
    /**
      | Encrypt the wallet
      |
      */
    #[Q_SLOT]
    pub fn encrypt_wallet(&mut self)  {
        
        todo!();
        /*
            auto dlg = new AskPassphraseDialog(AskPassphraseDialog::Encrypt, this);
        dlg->setModel(walletModel);
        connect(dlg, &QDialog::finished, this, &WalletView::encryptionStatusChanged);
        typename gui_util::ShowModalDialogAndDeleteOnClose(dlg);
        */
    }
    
    /**
      | Backup the wallet
      |
      */
    #[Q_SLOT]
    pub fn backup_wallet(&mut self)  {
        
        todo!();
        /*
            QString filename = typename gui_util::getSaveFileName(this,
            tr("Backup Wallet"), QString(),
            //: Name of the wallet data file format.
            tr("Wallet Data") + QLatin1String(" (*.dat)"), nullptr);

        if (filename.isEmpty())
            return;

        if (!walletModel->wallet().backupWallet(filename.toLocal8Bit().data())) {
            Q_EMIT message(tr("Backup Failed"), tr("There was an error trying to save the wallet data to %1.").arg(filename),
                CClientUIInterface::MSG_ERROR);
            }
        else {
            Q_EMIT message(tr("Backup Successful"), tr("The wallet data was successfully saved to %1.").arg(filename),
                CClientUIInterface::MSG_INFORMATION);
        }
        */
    }
    
    /**
      | Change encrypted wallet passphrase
      |
      */
    #[Q_SLOT]
    pub fn change_passphrase(&mut self)  {
        
        todo!();
        /*
            auto dlg = new AskPassphraseDialog(AskPassphraseDialog::ChangePass, this);
        dlg->setModel(walletModel);
        typename gui_util::ShowModalDialogAndDeleteOnClose(dlg);
        */
    }
    
    /**
      | Ask for passphrase to unlock wallet
      | temporarily
      |
      */
    #[Q_SLOT]
    pub fn unlock_wallet(&mut self)  {
        
        todo!();
        /*
            // Unlock wallet when requested by wallet model
        if (walletModel->getEncryptionStatus() == WalletModel::Locked) {
            auto dlg = new AskPassphraseDialog(AskPassphraseDialog::Unlock, this);
            dlg->setModel(walletModel);
            typename gui_util::ShowModalDialogAndDeleteOnClose(dlg);
        }
        */
    }
    
    /**
      | Show used sending addresses
      |
      */
    #[Q_SLOT]
    pub fn used_sending_addresses(&mut self)  {
        
        todo!();
        /*
            typename gui_util::bringToFront(usedSendingAddressesPage);
        */
    }
    
    /**
      | Show used receiving addresses
      |
      */
    #[Q_SLOT]
    pub fn used_receiving_addresses(&mut self)  {
        
        todo!();
        /*
            typename gui_util::bringToFront(usedReceivingAddressesPage);
        */
    }
    
    /**
      | Show progress dialog e.g. for rescan
      |
      */
    #[Q_SLOT]
    pub fn show_progress(&mut self, 
        title:      &String,
        n_progress: i32)  {
        
        todo!();
        /*
            if (nProgress == 0) {
            progressDialog = new QProgressDialog(title, tr("Cancel"), 0, 100);
            typename gui_util::PolishProgressDialog(progressDialog);
            progressDialog->setWindowModality(QtApplicationModal);
            progressDialog->setAutoClose(false);
            progressDialog->setValue(0);
        } else if (nProgress == 100) {
            if (progressDialog) {
                progressDialog->close();
                progressDialog->deleteLater();
                progressDialog = nullptr;
            }
        } else if (progressDialog) {
            if (progressDialog->wasCanceled()) {
                getWalletModel()->wallet().abortRescan();
            } else {
                progressDialog->setValue(nProgress);
            }
        }
        */
    }
}
