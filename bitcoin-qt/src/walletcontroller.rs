// ---------------- [ File: bitcoin-qt/src/walletcontroller.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/walletcontroller.h]

/**
  | Controller between typename NodeInterface,
  | WalletModel instances and the GUI.
  |
  */
#[Q_OBJECT]
pub struct WalletController {
    base:                QObject,
    activity_thread:     *const QThread,
    activity_worker:     *const QObject,
    client_model:        Rc<RefCell<ClientModel>>,
    node:                Rc<RefCell<dyn NodeInterface>>,
    platform_style:      *const PlatformStyle,
    options_model:       *const OptionsModel,
    mutex:               RefCell<QMutex>,
    wallets:             Vec<*mut WalletModel>,
    handler_load_wallet: Box<dyn Handler>,
}

impl WalletController {

    #[Q_SIGNAL]
    pub fn wallet_added(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn wallet_removed(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn coins_sent(&mut self, 
        wallet_model: *mut WalletModel,
        recipient:    SendCoinsRecipient,
        transaction:  QByteArray)  {
        
        todo!();
        /*
        
        */
    }
}

#[Q_OBJECT]
pub struct WalletControllerActivity {
    base:              QObject,
    wallet_controller: *const WalletController,
    parent_widget:     *const QWidget,
    wallet_model:      *mut WalletModel, // default = { nullptr }
    error_message:     BilingualStr,
    warning_message:   Vec<BilingualStr>,
}

impl WalletControllerActivity {

    #[Q_SIGNAL]
    pub fn finished(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn node(&self) -> Rc<RefCell<dyn NodeInterface>> {
        
        todo!();
        /*
            return m_wallet_controller->m_node;
        */
    }
    
    #[Q_SIGNAL]
    pub fn worker(&self) -> *mut QObject {
        
        todo!();
        /*
            return m_wallet_controller->m_activity_worker;
        */
    }
}

#[Q_OBJECT]
pub struct CreateWalletActivity {
    base:                 WalletControllerActivity,
    passphrase:           SecureString,
    create_wallet_dialog: *mut CreateWalletDialog, // default = { nullptr }
    passphrase_dialog:    *mut AskPassphraseDialog, // default = { nullptr }
}

impl CreateWalletActivity {

    #[Q_SIGNAL]
    pub fn created(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
        
        */
    }
}

///--------------------
#[Q_OBJECT]
pub struct OpenWalletActivity {
    base: WalletControllerActivity,
}

impl OpenWalletActivity {
    
    #[Q_SIGNAL]
    pub fn opened(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
        
        */
    }
}

///-------------------
#[Q_OBJECT]
pub struct LoadWalletsActivity {
    base: WalletControllerActivity,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/walletcontroller.cpp]
impl Drop for WalletController {

    /**
      | Not using the default destructor because
      | not all member types definitions are
      | available in the header, just forward
      | declared.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            m_activity_thread->quit();
        m_activity_thread->wait();
        delete m_activity_worker;
        */
    }
}

impl WalletController {
    
    pub fn new(
        client_model:   &mut ClientModel,
        platform_style: *const PlatformStyle,
        parent:         *mut QObject) -> Self {
    
        todo!();
        /*


            : QObject(parent)
        , m_activity_thread(new QThread(this))
        , m_activity_worker(new QObject)
        , m_client_model(client_model)
        , m_node(client_model.node())
        , m_platform_style(platform_style)
        , m_options_model(client_model.getOptionsModel())
        m_handler_load_wallet = m_node.walletClient().handleLoadWallet([this](std::unique_ptr<dyn WalletInterface> wallet) {
            getOrCreateWallet(std::move(wallet));
        });

        m_activity_worker->moveToThread(m_activity_thread);
        m_activity_thread->start();
        QTimer::singleShot(0, m_activity_worker, []() {
            util::ThreadRename("qt-walletctrl");
        });
        */
    }
    
    /**
      | Returns all wallet names in the wallet
      | dir mapped to whether the wallet is loaded.
      |
      */
    pub fn list_wallet_dir(&self) -> HashMap<String,bool> {
        
        todo!();
        /*
            QMutexLocker locker(&m_mutex);
        std::map<std::string, bool> wallets;
        for (const std::string& name : m_node.walletClient().listWalletDir()) {
            wallets[name] = false;
        }
        for (WalletModel* wallet_model : m_wallets) {
            auto it = wallets.find(wallet_model->wallet().getWalletName());
            if (it != wallets.end()) it->second = true;
        }
        return wallets;
        */
    }
    
    pub fn close_wallet(&mut self, 
        wallet_model: *mut WalletModel,
        parent:       *mut QWidget)  {
        
        todo!();
        /*
            QMessageBox box(parent);
        box.setWindowTitle(tr("Close wallet"));
        box.setText(tr("Are you sure you wish to close the wallet <i>%1</i>?").arg(typename gui_util::HtmlEscape(wallet_model->getDisplayName())));
        box.setInformativeText(tr("Closing the wallet for too long can result in having to resync the entire chain if pruning is enabled."));
        box.setStandardButtons(QMessageBox::Yes|QMessageBox::Cancel);
        box.setDefaultButton(QMessageBox::Yes);
        if (box.exec() != QMessageBox::Yes) return;

        // First remove wallet from node.
        wallet_model->wallet().remove();
        // Now release the model.
        removeAndDeleteWallet(wallet_model);
        */
    }
    
    pub fn close_all_wallets(&mut self, parent: *mut QWidget)  {
        
        todo!();
        /*
            QMessageBox::StandardButton button = QMessageBox::question(parent, tr("Close all wallets"),
            tr("Are you sure you wish to close all wallets?"),
            QMessageBox::Yes|QMessageBox::Cancel,
            QMessageBox::Yes);
        if (button != QMessageBox::Yes) return;

        QMutexLocker locker(&m_mutex);
        for (WalletModel* wallet_model : m_wallets) {
            wallet_model->wallet().remove();
            Q_EMIT walletRemoved(wallet_model);
            delete wallet_model;
        }
        m_wallets.clear();
        */
    }
    
    pub fn get_or_create_wallet(&mut self, wallet: Box<dyn WalletInterface>) -> *mut WalletModel {
        
        todo!();
        /*
            QMutexLocker locker(&m_mutex);

        // Return model instance if exists.
        if (!m_wallets.empty()) {
            std::string name = wallet->getWalletName();
            for (WalletModel* wallet_model : m_wallets) {
                if (wallet_model->wallet().getWalletName() == name) {
                    return wallet_model;
                }
            }
        }

        // Instantiate model and register it.
        WalletModel* wallet_model = new WalletModel(std::move(wallet), m_client_model, m_platform_style,
                                                    nullptr /* required for the following moveToThread() call */);

        // Move WalletModel object to the thread that created the WalletController
        // object (GUI main thread), instead of the current thread, which could be
        // an outside wallet thread or RPC thread sending a LoadWallet notification.
        // This ensures queued signals sent to the WalletModel object will be
        // handled on the GUI event loop.
        wallet_model->moveToThread(thread());
        // setParent(parent) must be called in the thread which created the parent object. More details in #18948.
        typename gui_util::ObjectInvoke(this, [wallet_model, this] {
            wallet_model->setParent(this);
        }, typename gui_util::blockingGUIThreadConnection());

        m_wallets.push_back(wallet_model);

        // WalletModel::startPollBalance needs to be called in a thread managed by
        // Qt because of startTimer. Considering the current thread can be a RPC
        // thread, better delegate the calling to Qt with QtAutoConnection.
        const bool called = QMetaObject::invokeMethod(wallet_model, "startPollBalance");
        assert(called);

        connect(wallet_model, &WalletModel::unload, this, [this, wallet_model] {
            // Defer removeAndDeleteWallet when no modal widget is active.
            // TODO: remove this workaround by removing usage of QDialog::exec.
            if (QApplication::activeModalWidget()) {
                connect(qApp, &QApplication::focusWindowChanged, wallet_model, [this, wallet_model]() {
                    if (!QApplication::activeModalWidget()) {
                        removeAndDeleteWallet(wallet_model);
                    }
                }, QtQueuedConnection);
            } else {
                removeAndDeleteWallet(wallet_model);
            }
        }, QtQueuedConnection);

        // Re-emit coinsSent signal from wallet model.
        connect(wallet_model, &WalletModel::coinsSent, this, &WalletController::coinsSent);

        Q_EMIT walletAdded(wallet_model);

        return wallet_model;
        */
    }
    
    pub fn remove_and_delete_wallet(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
            // Unregister wallet model.
        {
            QMutexLocker locker(&m_mutex);
            m_wallets.erase(std::remove(m_wallets.begin(), m_wallets.end(), wallet_model));
        }
        Q_EMIT walletRemoved(wallet_model);
        // Currently this can trigger the unload since the model can hold the last
        // CWallet shared pointer.
        delete wallet_model;
        */
    }
}

impl WalletControllerActivity {

    pub fn new(
        wallet_controller: *mut WalletController,
        parent_widget:     *mut QWidget) -> Self {
    
        todo!();
        /*


            : QObject(wallet_controller)
        , m_wallet_controller(wallet_controller)
        , m_parent_widget(parent_widget)
        connect(this, &WalletControllerActivity::finished, this, &QObject::deleteLater);
        */
    }
    
    #[Q_SIGNAL]
    pub fn show_progress_dialog(&mut self, 
        title_text: &String,
        label_text: &String)  {
        
        todo!();
        /*
            auto progress_dialog = new QProgressDialog(m_parent_widget);
        progress_dialog->setAttribute(QtWA_DeleteOnClose);
        connect(this, &WalletControllerActivity::finished, progress_dialog, &QWidget::close);

        progress_dialog->setWindowTitle(title_text);
        progress_dialog->setLabelText(label_text);
        progress_dialog->setRange(0, 0);
        progress_dialog->setCancelButton(nullptr);
        progress_dialog->setWindowModality(QtApplicationModal);
        typename gui_util::PolishProgressDialog(progress_dialog);
        // The setValue call forces QProgressDialog to start the internal duration estimation.
        // See details in https://bugreports.qt.io/browse/QTBUG-47042.
        progress_dialog->setValue(0);
        */
    }
}

///----------------------
impl Drop for CreateWalletActivity {
    fn drop(&mut self) {
        todo!();
        /*
            delete m_create_wallet_dialog;
        delete m_passphrase_dialog;
        */
    }
}

impl CreateWalletActivity {

    pub fn new(
        wallet_controller: *mut WalletController,
        parent_widget:     *mut QWidget) -> Self {
    
        todo!();
        /*


            : WalletControllerActivity(wallet_controller, parent_widget)

        m_passphrase.reserve(MAX_PASSPHRASE_SIZE);
        */
    }
    
    #[Q_SIGNAL]
    pub fn ask_passphrase(&mut self)  {
        
        todo!();
        /*
            m_passphrase_dialog = new AskPassphraseDialog(AskPassphraseDialog::Encrypt, m_parent_widget, &m_passphrase);
        m_passphrase_dialog->setWindowModality(QtApplicationModal);
        m_passphrase_dialog->show();

        connect(m_passphrase_dialog, &QObject::destroyed, [this] {
            m_passphrase_dialog = nullptr;
        });
        connect(m_passphrase_dialog, &QDialog::accepted, [this] {
            createWallet();
        });
        connect(m_passphrase_dialog, &QDialog::rejected, [this] {
            Q_EMIT finished();
        });
        */
    }
    
    #[Q_SIGNAL]
    pub fn create_wallet(&mut self)  {
        
        todo!();
        /*
            showProgressDialog(
            //: Title of window indicating the progress of creation of a new wallet.
            tr("Create Wallet"),
            /*: Descriptive text of the create wallet progress window which indicates
                to the user which wallet is currently being created. */
            tr("Creating Wallet <b>%1</b>…").arg(m_create_wallet_dialog->walletName().toHtmlEscaped()));

        std::string name = m_create_wallet_dialog->walletName().toStdString();
        uint64_t flags = 0;
        if (m_create_wallet_dialog->isDisablePrivateKeysChecked()) {
            flags |= WALLET_FLAG_DISABLE_PRIVATE_KEYS;
        }
        if (m_create_wallet_dialog->isMakeBlankWalletChecked()) {
            flags |= WALLET_FLAG_BLANK_WALLET;
        }
        if (m_create_wallet_dialog->isDescriptorWalletChecked()) {
            flags |= WALLET_FLAG_DESCRIPTORS;
        }
        if (m_create_wallet_dialog->isExternalSignerChecked()) {
            flags |= WALLET_FLAG_EXTERNAL_SIGNER;
        }

        QTimer::singleShot(500, worker(), [this, name, flags] {
            std::unique_ptr<dyn WalletInterface> wallet = node().walletClient().createWallet(name, m_passphrase, flags, m_error_message, m_warning_message);

            if (wallet) m_wallet_model = m_wallet_controller->getOrCreateWallet(std::move(wallet));

            QTimer::singleShot(500, this, &CreateWalletActivity::finish);
        });
        */
    }
    
    #[Q_SIGNAL]
    pub fn finish(&mut self)  {
        
        todo!();
        /*
            if (!m_error_message.empty()) {
            QMessageBox::critical(m_parent_widget, tr("Create wallet failed"), QString::fromStdString(m_error_message.translated));
        } else if (!m_warning_message.empty()) {
            QMessageBox::warning(m_parent_widget, tr("Create wallet warning"), QString::fromStdString(Join(m_warning_message, Untranslated("\n")).translated));
        }

        if (m_wallet_model) Q_EMIT created(m_wallet_model);

        Q_EMIT finished();
        */
    }
    
    pub fn create(&mut self)  {
        
        todo!();
        /*
            m_create_wallet_dialog = new CreateWalletDialog(m_parent_widget);

        std::vector<ExternalSigner> signers;
        try {
            signers = node().externalSigners();
        } catch (const std::runtime_error& e) {
            QMessageBox::critical(nullptr, tr("Can't list signers"), e.what());
        }
        m_create_wallet_dialog->setSigners(signers);

        m_create_wallet_dialog->setWindowModality(QtApplicationModal);
        m_create_wallet_dialog->show();

        connect(m_create_wallet_dialog, &QObject::destroyed, [this] {
            m_create_wallet_dialog = nullptr;
        });
        connect(m_create_wallet_dialog, &QDialog::rejected, [this] {
            Q_EMIT finished();
        });
        connect(m_create_wallet_dialog, &QDialog::accepted, [this] {
            if (m_create_wallet_dialog->isEncryptWalletChecked()) {
                askPassphrase();
            } else {
                createWallet();
            }
        });
        */
    }
}

impl OpenWalletActivity {
    
    pub fn new(
        wallet_controller: *mut WalletController,
        parent_widget:     *mut QWidget) -> Self {
    
        todo!();
        /*


            : WalletControllerActivity(wallet_controller, parent_widget)
        */
    }
    
    #[Q_SIGNAL]
    pub fn finish(&mut self)  {
        
        todo!();
        /*
            if (!m_error_message.empty()) {
            QMessageBox::critical(m_parent_widget, tr("Open wallet failed"), QString::fromStdString(m_error_message.translated));
        } else if (!m_warning_message.empty()) {
            QMessageBox::warning(m_parent_widget, tr("Open wallet warning"), QString::fromStdString(Join(m_warning_message, Untranslated("\n")).translated));
        }

        if (m_wallet_model) Q_EMIT opened(m_wallet_model);

        Q_EMIT finished();
        */
    }
    
    pub fn open(&mut self, path: &String)  {
        
        todo!();
        /*
            QString name = path.empty() ? QString("["+tr("default wallet")+"]") : QString::fromStdString(path);

        showProgressDialog(
            //: Title of window indicating the progress of opening of a wallet.
            tr("Open Wallet"),
            /*: Descriptive text of the open wallet progress window which indicates
                to the user which wallet is currently being opened. */
            tr("Opening Wallet <b>%1</b>…").arg(name.toHtmlEscaped()));

        QTimer::singleShot(0, worker(), [this, path] {
            std::unique_ptr<dyn WalletInterface> wallet = node().walletClient().loadWallet(path, m_error_message, m_warning_message);

            if (wallet) m_wallet_model = m_wallet_controller->getOrCreateWallet(std::move(wallet));

            QTimer::singleShot(0, this, &OpenWalletActivity::finish);
        });
        */
    }
}

impl LoadWalletsActivity {
    
    pub fn new(
        wallet_controller: *mut WalletController,
        parent_widget:     *mut QWidget) -> Self {
    
        todo!();
        /*
            : WalletControllerActivity(wallet_controller, parent_widget)
        */
    }
    
    pub fn load(&mut self)  {
        
        todo!();
        /*
            showProgressDialog(
            //: Title of progress window which is displayed when wallets are being loaded.
            tr("Load Wallets"),
            /*: Descriptive text of the load wallets progress window which indicates to
                the user that wallets are currently being loaded.*/
            tr("Loading wallets…"));

        QTimer::singleShot(0, worker(), [this] {
            for (auto& wallet : node().walletClient().getWallets()) {
                m_wallet_controller->getOrCreateWallet(std::move(wallet));
            }

            QTimer::singleShot(0, this, [this] { Q_EMIT finished(); });
        });
        */
    }
}
