crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/walletframe.h]

/**
  | A container for embedding all wallet-related
  | controls into BitcoinGUI.
  | 
  | The purpose of this class is to allow
  | future refinements of the wallet controls
  | with minimal need for further modifications
  | to BitcoinGUI, thus greatly simplifying
  | merges while reducing the risk of breaking
  | top-level stuff.
  |
  */
#[Q_OBJECT]
pub struct WalletFrame {
    base:             QFrame,
    wallet_stack:     *mut QStackedWidget,
    client_model:     *mut ClientModel,
    map_wallet_views: QMap<*mut WalletModel,*mut WalletView>,
    out_of_sync:      bool,
    platform_style:   *const PlatformStyle,
    size_hint:        QSize,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/walletframe.cpp]
impl WalletFrame {
    
    pub fn size_hint(&self) -> QSize {
        
        todo!();
        /*
            return m_size_hint;
        */
    }

    #[Q_SIGNAL]
    pub fn create_wallet_button_clicked(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn message(&mut self, 
        title:   &String,
        message: &String,
        style:   u32)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn current_wallet_set(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(
        platform_style: *const PlatformStyle,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*


            : QFrame(parent),
          platformStyle(_platformStyle),
          m_size_hint(OverviewPage{platformStyle, nullptr}.sizeHint())

        // Leave HBox hook for adding a list view later
        QHBoxLayout *walletFrameLayout = new QHBoxLayout(this);
        setContentsMargins(0,0,0,0);
        walletStack = new QStackedWidget(this);
        walletFrameLayout->setContentsMargins(0,0,0,0);
        walletFrameLayout->addWidget(walletStack);

        // hbox for no wallet
        QGroupBox* no_wallet_group = new QGroupBox(walletStack);
        QVBoxLayout* no_wallet_layout = new QVBoxLayout(no_wallet_group);

        QLabel *noWallet = new QLabel(tr("No wallet has been loaded.\nGo to File > Open Wallet to load a wallet.\n- OR -"));
        noWallet->setAlignment(QtAlignCenter);
        no_wallet_layout->addWidget(noWallet, 0, QtAlignHCenter | QtAlignBottom);

        // A button for create wallet dialog
        QPushButton* create_wallet_button = new QPushButton(tr("Create a new wallet"), walletStack);
        connect(create_wallet_button, &QPushButton::clicked, this, &WalletFrame::createWalletButtonClicked);
        no_wallet_layout->addWidget(create_wallet_button, 0, QtAlignHCenter | QtAlignTop);
        no_wallet_group->setLayout(no_wallet_layout);

        walletStack->addWidget(no_wallet_group);
        */
    }
    
    pub fn set_client_model(&mut self, client_model: *mut ClientModel)  {
        
        todo!();
        /*
            this->clientModel = _clientModel;

        for (auto i = mapWalletViews.constBegin(); i != mapWalletViews.constEnd(); ++i) {
            i.value()->setClientModel(_clientModel);
        }
        */
    }
    
    pub fn add_view(&mut self, wallet_view: *mut WalletView) -> bool {
        
        todo!();
        /*
            if (!clientModel) return false;

        if (mapWalletViews.count(walletView->getWalletModel()) > 0) return false;

        walletView->setClientModel(clientModel);
        walletView->showOutOfSyncWarning(bOutOfSync);

        WalletView* current_wallet_view = currentWalletView();
        if (current_wallet_view) {
            walletView->setCurrentIndex(current_wallet_view->currentIndex());
        } else {
            walletView->gotoOverviewPage();
        }

        walletStack->addWidget(walletView);
        mapWalletViews[walletView->getWalletModel()] = walletView;

        return true;
        */
    }
    
    pub fn set_current_wallet(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
            if (mapWalletViews.count(wallet_model) == 0) return;

        // Stop the effect of hidden widgets on the size hint of the shown one in QStackedWidget.
        WalletView* view_about_to_hide = currentWalletView();
        if (view_about_to_hide) {
            QSizePolicy sp = view_about_to_hide->sizePolicy();
            sp.setHorizontalPolicy(QSizePolicy::Ignored);
            view_about_to_hide->setSizePolicy(sp);
        }

        WalletView *walletView = mapWalletViews.value(wallet_model);
        assert(walletView);

        // Set or restore the default QSizePolicy which could be set to QSizePolicy::Ignored previously.
        QSizePolicy sp = walletView->sizePolicy();
        sp.setHorizontalPolicy(QSizePolicy::Preferred);
        walletView->setSizePolicy(sp);
        walletView->updateGeometry();

        walletStack->setCurrentWidget(walletView);

        Q_EMIT currentWalletSet();
        */
    }
    
    pub fn remove_wallet(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
            if (mapWalletViews.count(wallet_model) == 0) return;

        WalletView *walletView = mapWalletViews.take(wallet_model);
        walletStack->removeWidget(walletView);
        delete walletView;
        */
    }
    
    pub fn remove_all_wallets(&mut self)  {
        
        todo!();
        /*
            QMap<WalletModel*, WalletView*>::const_iterator i;
        for (i = mapWalletViews.constBegin(); i != mapWalletViews.constEnd(); ++i)
            walletStack->removeWidget(i.value());
        mapWalletViews.clear();
        */
    }
    
    pub fn handle_payment_request(&mut self, recipient: &SendCoinsRecipient) -> bool {
        
        todo!();
        /*
            WalletView *walletView = currentWalletView();
        if (!walletView)
            return false;

        return walletView->handlePaymentRequest(recipient);
        */
    }
    
    pub fn show_out_of_sync_warning(&mut self, show: bool)  {
        
        todo!();
        /*
            bOutOfSync = fShow;
        QMap<WalletModel*, WalletView*>::const_iterator i;
        for (i = mapWalletViews.constBegin(); i != mapWalletViews.constEnd(); ++i)
            i.value()->showOutOfSyncWarning(fShow);
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
            QMap<WalletModel*, WalletView*>::const_iterator i;
        for (i = mapWalletViews.constBegin(); i != mapWalletViews.constEnd(); ++i)
            i.value()->gotoOverviewPage();
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
            QMap<WalletModel*, WalletView*>::const_iterator i;
        for (i = mapWalletViews.constBegin(); i != mapWalletViews.constEnd(); ++i)
            i.value()->gotoHistoryPage();
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
            QMap<WalletModel*, WalletView*>::const_iterator i;
        for (i = mapWalletViews.constBegin(); i != mapWalletViews.constEnd(); ++i)
            i.value()->gotoReceiveCoinsPage();
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
            QMap<WalletModel*, WalletView*>::const_iterator i;
        for (i = mapWalletViews.constBegin(); i != mapWalletViews.constEnd(); ++i)
            i.value()->gotoSendCoinsPage(addr);
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
            WalletView *walletView = currentWalletView();
        if (walletView)
            walletView->gotoSignMessageTab(addr);
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
            WalletView *walletView = currentWalletView();
        if (walletView)
            walletView->gotoVerifyMessageTab(addr);
        */
    }
    
    /**
      | Load Partially Signed Bitcoin Transaction
      |
      */
    #[Q_SLOT]
    pub fn goto_loadpsbt(&mut self, from_clipboard: Option<bool>)  {

        let from_clipboard: bool = from_clipboard.unwrap_or(false);
        
        todo!();
        /*
            std::string data;

        if (from_clipboard) {
            std::string raw = QApplication::clipboard()->text().toStdString();
            bool invalid;
            data = DecodeBase64(raw, &invalid);
            if (invalid) {
                Q_EMIT message(tr("Error"), tr("Unable to decode PSBT from clipboard (invalid base64)"), CClientUIInterface::MSG_ERROR);
                return;
            }
        } else {
            QString filename = typename gui_util::getOpenFileName(this,
                tr("Load Transaction Data"), QString(),
                tr("Partially Signed Transaction (*.psbt)"), nullptr);
            if (filename.isEmpty()) return;
            if (GetFileSize(filename.toLocal8Bit().data(), MAX_FILE_SIZE_PSBT) == MAX_FILE_SIZE_PSBT) {
                Q_EMIT message(tr("Error"), tr("PSBT file must be smaller than 100 MiB"), CClientUIInterface::MSG_ERROR);
                return;
            }
            std::ifstream in(filename.toLocal8Bit().data(), std::ios::binary);
            data = std::string(std::istreambuf_iterator<char>{in}, {});
        }

        std::string error;
        PartiallySignedTransaction psbtx;
        if (!DecodeRawPSBT(psbtx, data, error)) {
            Q_EMIT message(tr("Error"), tr("Unable to decode PSBT") + "\n" + QString::fromStdString(error), CClientUIInterface::MSG_ERROR);
            return;
        }

        auto dlg = new PSBTOperationsDialog(this, currentWalletModel(), clientModel);
        dlg->openWithPSBT(psbtx);
        typename gui_util::ShowModalDialogAndDeleteOnClose(dlg);
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
            WalletView *walletView = currentWalletView();
        if (walletView)
            walletView->encryptWallet();
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
            WalletView *walletView = currentWalletView();
        if (walletView)
            walletView->backupWallet();
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
            WalletView *walletView = currentWalletView();
        if (walletView)
            walletView->changePassphrase();
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
            WalletView *walletView = currentWalletView();
        if (walletView)
            walletView->unlockWallet();
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
            WalletView *walletView = currentWalletView();
        if (walletView)
            walletView->usedSendingAddresses();
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
            WalletView *walletView = currentWalletView();
        if (walletView)
            walletView->usedReceivingAddresses();
        */
    }
    
    pub fn current_wallet_view(&self) -> *mut WalletView {
        
        todo!();
        /*
            return qobject_cast<WalletView*>(walletStack->currentWidget());
        */
    }
    
    pub fn current_wallet_model(&self) -> *mut WalletModel {
        
        todo!();
        /*
            WalletView* wallet_view = currentWalletView();
        return wallet_view ? wallet_view->getWalletModel() : nullptr;
        */
    }
}
