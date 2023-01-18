crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoingui.h]

/**
  | Bitcoin GUI main class.
  | 
  | This class represents the main window
  | of the Bitcoin UI.
  | 
  | It communicates with both the client
  | and wallet models to give the user an
  | up-to-date view of the current core
  | state.
  |
  */
#[Q_OBJECT]
pub struct BitcoinGUI {
    base:                            QMainWindow,
    enable_wallet:                   bool, // default = false
    node:                            Rc<RefCell<dyn NodeInterface>>,
    wallet_controller:               Rc<RefCell<WalletController>>, // default = { nullptr }
    handler_message_box:             Box<dyn Handler>,
    handler_question:                Box<dyn Handler>,
    client_model:                    *mut ClientModel, // default = nullptr
    wallet_frame:                    *mut WalletFrame, // default = nullptr
    unit_display_control:            *mut UnitDisplayStatusBarControl, // default = nullptr
    label_wallet_encryption_icon:    *mut gui_util::ThemedLabel, // default = nullptr
    label_wallet_hd_status_icon:     *mut gui_util::ThemedLabel, // default = nullptr
    label_proxy_icon:                *mut gui_util::ClickableLabel, // default = nullptr
    connections_control:             *mut gui_util::ClickableLabel, // default = nullptr
    label_blocks_icon:               *mut gui_util::ClickableLabel, // default = nullptr
    progress_bar_label:              *mut QLabel, // default = nullptr
    progress_bar:                    *mut gui_util::ClickableProgressBar, // default = nullptr
    progress_dialog:                 *mut QProgressDialog, // default = nullptr
    app_menu_bar:                    *mut QMenuBar, // default = nullptr
    app_tool_bar:                    *mut QToolBar, // default = nullptr
    overview_action:                 *mut QAction, // default = nullptr
    history_action:                  *mut QAction, // default = nullptr
    quit_action:                     *mut QAction, // default = nullptr
    send_coins_action:               *mut QAction, // default = nullptr
    send_coins_menu_action:          *mut QAction, // default = nullptr
    used_sending_addresses_action:   *mut QAction, // default = nullptr
    used_receiving_addresses_action: *mut QAction, // default = nullptr
    sign_message_action:             *mut QAction, // default = nullptr
    verify_message_action:           *mut QAction, // default = nullptr
    load_psbt_action:                *mut QAction, // default = nullptr
    load_psbt_clipboard_action:      *mut QAction, // default = nullptr
    about_action:                    *mut QAction, // default = nullptr
    receive_coins_action:            *mut QAction, // default = nullptr
    receive_coins_menu_action:       *mut QAction, // default = nullptr
    options_action:                  *mut QAction, // default = nullptr
    toggle_hide_action:              *mut QAction, // default = nullptr
    encrypt_wallet_action:           *mut QAction, // default = nullptr
    backup_wallet_action:            *mut QAction, // default = nullptr
    change_passphrase_action:        *mut QAction, // default = nullptr
    about_qt_action:                 *mut QAction, // default = nullptr
    open_rpc_console_action:         *mut QAction, // default = nullptr
    open_action:                     *mut QAction, // default = nullptr
    show_help_message_action:        *mut QAction, // default = nullptr
    create_wallet_action:            *mut QAction, // default = { nullptr }
    open_wallet_action:              *mut QAction, // default = { nullptr }
    open_wallet_menu:                *mut QMenu, // default = { nullptr }
    close_wallet_action:             *mut QAction, // default = { nullptr }
    close_all_wallets_action:        *mut QAction, // default = { nullptr }
    wallet_selector_label_action:    *mut QAction, // default = nullptr
    wallet_selector_action:          *mut QAction, // default = nullptr
    mask_values_action:              *mut QAction, // default = { nullptr }
    wallet_selector_label:           *mut QLabel, // default = nullptr
    wallet_selector:                 *mut QComboBox, // default = nullptr
    tray_icon:                       *mut QSystemTrayIcon, // default = nullptr
    tray_icon_menu:                  Box<QMenu>,
    notificator:                     *mut Notificator, // default = nullptr
    rpc_console:                     *mut RPCConsole, // default = nullptr
    help_message_dialog:             *mut HelpMessageDialog, // default = nullptr
    modal_overlay:                   *mut ModalOverlay, // default = nullptr
    network_context_menu:            *mut QMenu, //= new QMenu(this);

    #[cfg(Q_OS_MAC)]
    app_nap_inhibitor:               *mut AppNapInhibitor, // default = nullptr

    /**
      | Keep track of previous number of blocks,
      | to detect progress
      |
      */
    prev_blocks:                     i32, // default = 0
    spinner_frame:                   i32, // default = 0
    platform_style:                  *const PlatformStyle,
    network_style:                   *const NetworkStyle,
}

pub mod bitcoin_gui {

    use super::*;

    lazy_static!{
        /*
        static const std::string DEFAULT_UIPLATFORM;
        */
    }
}

impl BitcoinGUI {

    /**
      | Get the tray icon status.
      | 
      | Some systems have not "system tray"
      | or "notification area" available.
      |
      */
    pub fn has_tray_icon(&self) -> bool {
        
        todo!();
        /*
            return trayIcon;
        */
    }

    #[Q_SIGNAL]
    pub fn quit_requested(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Signal raised when a URI was entered
      | or dragged to the GUI
      |
      */
    #[Q_SIGNAL]
    pub fn receiveduri(&mut self, uri: &String)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Signal raised when RPC console shown
      |
      */
    #[Q_SIGNAL]
    pub fn console_shown(&mut self, console: *mut RPCConsole)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn set_privacy(&mut self, privacy: bool)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Show window if hidden, unminimize when
      | minimized, rise when obscured or show
      | if hidden and fToggleHidden is true
      |
      */
    #[Q_SLOT]
    pub fn show_normal_if_minimized_default(&mut self)  {
        
        todo!();
        /*
            showNormalIfMinimized(false);
        */
    }
}

///------------------------
#[Q_OBJECT]
pub struct UnitDisplayStatusBarControl {
    base:           QLabel,
    options_model:  *mut OptionsModel,
    menu:           *mut QMenu,
    platform_style: *const PlatformStyle,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoingui.cpp]

lazy_static!{
    /*
    const std::string BitcoinGUI::DEFAULT_UIPLATFORM =
    #if defined(Q_OS_MAC)
            "macosx"
    #elif defined(Q_OS_WIN)
            "windows"
    #else
            "other"
    #endif
            ;
    */
}

///---------------------------
impl Drop for BitcoinGUI {
    fn drop(&mut self) {
        todo!();
        /*
            // Unsubscribe from notifications from core
        unsubscribeFromCoreSignals();

        QSettings settings;
        settings.setValue("MainWindowGeometry", saveGeometry());
        if(trayIcon) // Hide tray icon, as deleting will let it linger until quit (on Ubuntu)
            trayIcon->hide();
    #ifdef Q_OS_MAC
        delete m_app_nap_inhibitor;
        delete appMenuBar;
        MacDockIconHandler::cleanup();
    #endif

        delete rpcConsole;
        */
    }
}

impl BitcoinGUI {

    pub fn new(
        node:           Rc<RefCell<dyn NodeInterface>>,
        platform_style: *const PlatformStyle,
        network_style:  *const NetworkStyle,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*


            :
        QMainWindow(parent),
        m_node(node),
        trayIconMenu{new QMenu()},
        platformStyle(_platformStyle),
        m_network_style(networkStyle)
        QSettings settings;
        if (!restoreGeometry(settings.value("MainWindowGeometry").toByteArray())) {
            // Restore failed (perhaps missing setting), center the window
            move(QGuiApplication::primaryScreen()->availableGeometry().center() - frameGeometry().center());
        }

        setContextMenuPolicy(QtPreventContextMenu);

    #ifdef ENABLE_WALLET
        enableWallet = WalletModel::isWalletEnabled();
    #endif // ENABLE_WALLET
        QApplication::setWindowIcon(m_network_style->getTrayAndWindowIcon());
        setWindowIcon(m_network_style->getTrayAndWindowIcon());
        updateWindowTitle();

        rpcConsole = new RPCConsole(node, _platformStyle, nullptr);
        helpMessageDialog = new HelpMessageDialog(this, false);
    #ifdef ENABLE_WALLET
        if(enableWallet)
        {
            /** Create wallet frame and make it the central widget */
            walletFrame = new WalletFrame(_platformStyle, this);
            connect(walletFrame, &WalletFrame::createWalletButtonClicked, [this] {
                auto activity = new CreateWalletActivity(getWalletController(), this);
                activity->create();
            });
            connect(walletFrame, &WalletFrame::message, [this](const QString& title, const QString& message, unsigned int style) {
                this->message(title, message, style);
            });
            connect(walletFrame, &WalletFrame::currentWalletSet, [this] { updateWalletStatus(); });
            setCentralWidget(walletFrame);
        } else
    #endif // ENABLE_WALLET
        {
            /* When compiled without wallet or -disablewallet is provided,
             * the central widget is the rpc console.
             */
            setCentralWidget(rpcConsole);
            Q_EMIT consoleShown(rpcConsole);
        }

        modalOverlay = new ModalOverlay(enableWallet, this->centralWidget());

        // Accept D&D of URIs
        setAcceptDrops(true);

        // Create actions for the toolbar, menu bar and tray/dock icon
        // Needs walletFrame to be initialized
        createActions();

        // Create application menu bar
        createMenuBar();

        // Create the toolbars
        createToolBars();

        // Create system tray icon and notification
        if (QSystemTrayIcon::isSystemTrayAvailable()) {
            createTrayIcon();
        }
        notificator = new Notificator(QApplication::applicationName(), trayIcon, this);

        // Create status bar
        statusBar();

        // Disable size grip because it looks ugly and nobody needs it
        statusBar()->setSizeGripEnabled(false);

        // Status bar notification icons
        QFrame *frameBlocks = new QFrame();
        frameBlocks->setContentsMargins(0,0,0,0);
        frameBlocks->setSizePolicy(QSizePolicy::Fixed, QSizePolicy::Preferred);
        QHBoxLayout *frameBlocksLayout = new QHBoxLayout(frameBlocks);
        frameBlocksLayout->setContentsMargins(3,0,3,0);
        frameBlocksLayout->setSpacing(3);
        unitDisplayControl = new UnitDisplayStatusBarControl(platformStyle);
        labelWalletEncryptionIcon = new gui_util::ThemedLabel(platformStyle);
        labelWalletHDStatusIcon = new gui_util::ThemedLabel(platformStyle);
        labelProxyIcon = new gui_util::ClickableLabel(platformStyle);
        connectionsControl = new gui_util::ClickableLabel(platformStyle);
        labelBlocksIcon = new gui_util::ClickableLabel(platformStyle);
        if(enableWallet)
        {
            frameBlocksLayout->addStretch();
            frameBlocksLayout->addWidget(unitDisplayControl);
            frameBlocksLayout->addStretch();
            frameBlocksLayout->addWidget(labelWalletEncryptionIcon);
            labelWalletEncryptionIcon->hide();
            frameBlocksLayout->addWidget(labelWalletHDStatusIcon);
            labelWalletHDStatusIcon->hide();
        }
        frameBlocksLayout->addWidget(labelProxyIcon);
        frameBlocksLayout->addStretch();
        frameBlocksLayout->addWidget(connectionsControl);
        frameBlocksLayout->addStretch();
        frameBlocksLayout->addWidget(labelBlocksIcon);
        frameBlocksLayout->addStretch();

        // Progress bar and label for blocks download
        progressBarLabel = new QLabel();
        progressBarLabel->setVisible(false);
        progressBar = new gui_util::ProgressBar();
        progressBar->setAlignment(QtAlignCenter);
        progressBar->setVisible(false);

        // Override style sheet for progress bar for styles that have a segmented progress bar,
        // as they make the text unreadable (workaround for issue #1071)
        // See https://doc.qt.io/qt-5/gallery.html
        QString curStyle = QApplication::style()->metaObject()->className();
        if(curStyle == "QWindowsStyle" || curStyle == "QWindowsXPStyle")
        {
            progressBar->setStyleSheet("QProgressBar { background-color: #e8e8e8; border: 1px solid grey; border-radius: 7px; padding: 1px; text-align: center; } QProgressBar::chunk { background: QLinearGradient(x1: 0, y1: 0, x2: 1, y2: 0, stop: 0 #FF8000, stop: 1 orange); border-radius: 7px; margin: 0px; }");
        }

        statusBar()->addWidget(progressBarLabel);
        statusBar()->addWidget(progressBar);
        statusBar()->addPermanentWidget(frameBlocks);

        // Install event filter to be able to catch status tip events (QEvent::StatusTip)
        this->installEventFilter(this);

        // Initially wallet actions should be disabled
        setWalletActionsEnabled(false);

        // Subscribe to notifications from core
        subscribeToCoreSignals();

        connect(labelProxyIcon, &gui_util::ClickableLabel::clicked, [this] {
            openOptionsDialogWithTab(OptionsDialog::TAB_NETWORK);
        });

        connect(labelBlocksIcon, &gui_util::ClickableLabel::clicked, this, &BitcoinGUI::showModalOverlay);
        connect(progressBar, &gui_util::ClickableProgressBar::clicked, this, &BitcoinGUI::showModalOverlay);

    #ifdef Q_OS_MAC
        m_app_nap_inhibitor = new CAppNapInhibitor;
    #endif

        gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    /**
      | Create the main UI actions.
      |
      */
    pub fn create_actions(&mut self)  {
        
        todo!();
        /*
            QActionGroup *tabGroup = new QActionGroup(this);
        connect(modalOverlay, &ModalOverlay::triggered, tabGroup, &QActionGroup::setEnabled);

        overviewAction = new QAction(platformStyle->SingleColorIcon(":/icons/overview"), tr("&Overview"), this);
        overviewAction->setStatusTip(tr("Show general overview of wallet"));
        overviewAction->setToolTip(overviewAction->statusTip());
        overviewAction->setCheckable(true);
        overviewAction->setShortcut(QKeySequence(QtALT + QtKey_1));
        tabGroup->addAction(overviewAction);

        sendCoinsAction = new QAction(platformStyle->SingleColorIcon(":/icons/send"), tr("&Send"), this);
        sendCoinsAction->setStatusTip(tr("Send coins to a Bitcoin address"));
        sendCoinsAction->setToolTip(sendCoinsAction->statusTip());
        sendCoinsAction->setCheckable(true);
        sendCoinsAction->setShortcut(QKeySequence(QtALT + QtKey_2));
        tabGroup->addAction(sendCoinsAction);

        sendCoinsMenuAction = new QAction(sendCoinsAction->text(), this);
        sendCoinsMenuAction->setStatusTip(sendCoinsAction->statusTip());
        sendCoinsMenuAction->setToolTip(sendCoinsMenuAction->statusTip());

        receiveCoinsAction = new QAction(platformStyle->SingleColorIcon(":/icons/receiving_addresses"), tr("&Receive"), this);
        receiveCoinsAction->setStatusTip(tr("Request payments (generates QR codes and bitcoin: URIs)"));
        receiveCoinsAction->setToolTip(receiveCoinsAction->statusTip());
        receiveCoinsAction->setCheckable(true);
        receiveCoinsAction->setShortcut(QKeySequence(QtALT + QtKey_3));
        tabGroup->addAction(receiveCoinsAction);

        receiveCoinsMenuAction = new QAction(receiveCoinsAction->text(), this);
        receiveCoinsMenuAction->setStatusTip(receiveCoinsAction->statusTip());
        receiveCoinsMenuAction->setToolTip(receiveCoinsMenuAction->statusTip());

        historyAction = new QAction(platformStyle->SingleColorIcon(":/icons/history"), tr("&Transactions"), this);
        historyAction->setStatusTip(tr("Browse transaction history"));
        historyAction->setToolTip(historyAction->statusTip());
        historyAction->setCheckable(true);
        historyAction->setShortcut(QKeySequence(QtALT + QtKey_4));
        tabGroup->addAction(historyAction);

    #ifdef ENABLE_WALLET
        // These showNormalIfMinimized are needed because Send Coins and Receive Coins
        // can be triggered from the tray menu, and need to show the GUI to be useful.
        connect(overviewAction, &QAction::triggered, [this]{ showNormalIfMinimized(); });
        connect(overviewAction, &QAction::triggered, this, &BitcoinGUI::gotoOverviewPage);
        connect(sendCoinsAction, &QAction::triggered, [this]{ showNormalIfMinimized(); });
        connect(sendCoinsAction, &QAction::triggered, [this]{ gotoSendCoinsPage(); });
        connect(sendCoinsMenuAction, &QAction::triggered, [this]{ showNormalIfMinimized(); });
        connect(sendCoinsMenuAction, &QAction::triggered, [this]{ gotoSendCoinsPage(); });
        connect(receiveCoinsAction, &QAction::triggered, [this]{ showNormalIfMinimized(); });
        connect(receiveCoinsAction, &QAction::triggered, this, &BitcoinGUI::gotoReceiveCoinsPage);
        connect(receiveCoinsMenuAction, &QAction::triggered, [this]{ showNormalIfMinimized(); });
        connect(receiveCoinsMenuAction, &QAction::triggered, this, &BitcoinGUI::gotoReceiveCoinsPage);
        connect(historyAction, &QAction::triggered, [this]{ showNormalIfMinimized(); });
        connect(historyAction, &QAction::triggered, this, &BitcoinGUI::gotoHistoryPage);
    #endif // ENABLE_WALLET

        quitAction = new QAction(tr("E&xit"), this);
        quitAction->setStatusTip(tr("Quit application"));
        quitAction->setShortcut(QKeySequence(QtCTRL + QtKey_Q));
        quitAction->setMenuRole(QAction::QuitRole);
        aboutAction = new QAction(tr("&About %1").arg(PACKAGE_NAME), this);
        aboutAction->setStatusTip(tr("Show information about %1").arg(PACKAGE_NAME));
        aboutAction->setMenuRole(QAction::AboutRole);
        aboutAction->setEnabled(false);
        aboutQtAction = new QAction(tr("About &Qt"), this);
        aboutQtAction->setStatusTip(tr("Show information about Qt"));
        aboutQtAction->setMenuRole(QAction::AboutQtRole);
        optionsAction = new QAction(tr("&Options…"), this);
        optionsAction->setStatusTip(tr("Modify configuration options for %1").arg(PACKAGE_NAME));
        optionsAction->setMenuRole(QAction::PreferencesRole);
        optionsAction->setEnabled(false);
        toggleHideAction = new QAction(tr("&Show / Hide"), this);
        toggleHideAction->setStatusTip(tr("Show or hide the main Window"));

        encryptWalletAction = new QAction(tr("&Encrypt Wallet…"), this);
        encryptWalletAction->setStatusTip(tr("Encrypt the private keys that belong to your wallet"));
        encryptWalletAction->setCheckable(true);
        backupWalletAction = new QAction(tr("&Backup Wallet…"), this);
        backupWalletAction->setStatusTip(tr("Backup wallet to another location"));
        changePassphraseAction = new QAction(tr("&Change Passphrase…"), this);
        changePassphraseAction->setStatusTip(tr("Change the passphrase used for wallet encryption"));
        signMessageAction = new QAction(tr("Sign &message…"), this);
        signMessageAction->setStatusTip(tr("Sign messages with your Bitcoin addresses to prove you own them"));
        verifyMessageAction = new QAction(tr("&Verify message…"), this);
        verifyMessageAction->setStatusTip(tr("Verify messages to ensure they were signed with specified Bitcoin addresses"));
        m_load_psbt_action = new QAction(tr("&Load PSBT from file…"), this);
        m_load_psbt_action->setStatusTip(tr("Load Partially Signed Bitcoin Transaction"));
        m_load_psbt_clipboard_action = new QAction(tr("Load PSBT from &clipboard…"), this);
        m_load_psbt_clipboard_action->setStatusTip(tr("Load Partially Signed Bitcoin Transaction from clipboard"));

        openRPCConsoleAction = new QAction(tr("Node window"), this);
        openRPCConsoleAction->setStatusTip(tr("Open node debugging and diagnostic console"));
        // initially disable the debug window menu item
        openRPCConsoleAction->setEnabled(false);
        openRPCConsoleAction->setObjectName("openRPCConsoleAction");

        usedSendingAddressesAction = new QAction(tr("&Sending addresses"), this);
        usedSendingAddressesAction->setStatusTip(tr("Show the list of used sending addresses and labels"));
        usedReceivingAddressesAction = new QAction(tr("&Receiving addresses"), this);
        usedReceivingAddressesAction->setStatusTip(tr("Show the list of used receiving addresses and labels"));

        openAction = new QAction(tr("Open &URI…"), this);
        openAction->setStatusTip(tr("Open a bitcoin: URI"));

        m_open_wallet_action = new QAction(tr("Open Wallet"), this);
        m_open_wallet_action->setEnabled(false);
        m_open_wallet_action->setStatusTip(tr("Open a wallet"));
        m_open_wallet_menu = new QMenu(this);

        m_close_wallet_action = new QAction(tr("Close Wallet…"), this);
        m_close_wallet_action->setStatusTip(tr("Close wallet"));

        m_create_wallet_action = new QAction(tr("Create Wallet…"), this);
        m_create_wallet_action->setEnabled(false);
        m_create_wallet_action->setStatusTip(tr("Create a new wallet"));

        m_close_all_wallets_action = new QAction(tr("Close All Wallets…"), this);
        m_close_all_wallets_action->setStatusTip(tr("Close all wallets"));

        showHelpMessageAction = new QAction(tr("&Command-line options"), this);
        showHelpMessageAction->setMenuRole(QAction::NoRole);
        showHelpMessageAction->setStatusTip(tr("Show the %1 help message to get a list with possible Bitcoin command-line options").arg(PACKAGE_NAME));

        m_mask_values_action = new QAction(tr("&Mask values"), this);
        m_mask_values_action->setShortcut(QKeySequence(QtCTRL + QtSHIFT + QtKey_M));
        m_mask_values_action->setStatusTip(tr("Mask the values in the Overview tab"));
        m_mask_values_action->setCheckable(true);

        connect(quitAction, &QAction::triggered, this, &BitcoinGUI::quitRequested);
        connect(aboutAction, &QAction::triggered, this, &BitcoinGUI::aboutClicked);
        connect(aboutQtAction, &QAction::triggered, qApp, QApplication::aboutQt);
        connect(optionsAction, &QAction::triggered, this, &BitcoinGUI::optionsClicked);
        connect(toggleHideAction, &QAction::triggered, this, &BitcoinGUI::toggleHidden);
        connect(showHelpMessageAction, &QAction::triggered, this, &BitcoinGUI::showHelpMessageClicked);
        connect(openRPCConsoleAction, &QAction::triggered, this, &BitcoinGUI::showDebugWindow);
        // prevents an open debug window from becoming stuck/unusable on client shutdown
        connect(quitAction, &QAction::triggered, rpcConsole, &QWidget::hide);

    #ifdef ENABLE_WALLET
        if(walletFrame)
        {
            connect(encryptWalletAction, &QAction::triggered, walletFrame, &WalletFrame::encryptWallet);
            connect(backupWalletAction, &QAction::triggered, walletFrame, &WalletFrame::backupWallet);
            connect(changePassphraseAction, &QAction::triggered, walletFrame, &WalletFrame::changePassphrase);
            connect(signMessageAction, &QAction::triggered, [this]{ showNormalIfMinimized(); });
            connect(signMessageAction, &QAction::triggered, [this]{ gotoSignMessageTab(); });
            connect(m_load_psbt_action, &QAction::triggered, [this]{ gotoLoadPSBT(); });
            connect(m_load_psbt_clipboard_action, &QAction::triggered, [this]{ gotoLoadPSBT(true); });
            connect(verifyMessageAction, &QAction::triggered, [this]{ showNormalIfMinimized(); });
            connect(verifyMessageAction, &QAction::triggered, [this]{ gotoVerifyMessageTab(); });
            connect(usedSendingAddressesAction, &QAction::triggered, walletFrame, &WalletFrame::usedSendingAddresses);
            connect(usedReceivingAddressesAction, &QAction::triggered, walletFrame, &WalletFrame::usedReceivingAddresses);
            connect(openAction, &QAction::triggered, this, &BitcoinGUI::openClicked);
            connect(m_open_wallet_menu, &QMenu::aboutToShow, [this] {
                m_open_wallet_menu->clear();
                for (const std::pair<const std::string, bool>& i : m_wallet_controller->listWalletDir()) {
                    const std::string& path = i.first;
                    QString name = path.empty() ? QString("["+tr("default wallet")+"]") : QString::fromStdString(path);
                    // Menu items remove single &. Single & are shown when && is in
                    // the string, but only the first occurrence. So replace only
                    // the first & with &&.
                    name.replace(name.indexOf(QChar('&')), 1, QString("&&"));
                    QAction* action = m_open_wallet_menu->addAction(name);

                    if (i.second) {
                        // This wallet is already loaded
                        action->setEnabled(false);
                        continue;
                    }

                    connect(action, &QAction::triggered, [this, path] {
                        auto activity = new OpenWalletActivity(m_wallet_controller, this);
                        connect(activity, &OpenWalletActivity::opened, this, &BitcoinGUI::setCurrentWallet);
                        activity->open(path);
                    });
                }
                if (m_open_wallet_menu->isEmpty()) {
                    QAction* action = m_open_wallet_menu->addAction(tr("No wallets available"));
                    action->setEnabled(false);
                }
            });
            connect(m_close_wallet_action, &QAction::triggered, [this] {
                m_wallet_controller->closeWallet(walletFrame->currentWalletModel(), this);
            });
            connect(m_create_wallet_action, &QAction::triggered, [this] {
                auto activity = new CreateWalletActivity(m_wallet_controller, this);
                connect(activity, &CreateWalletActivity::created, this, &BitcoinGUI::setCurrentWallet);
                activity->create();
            });
            connect(m_close_all_wallets_action, &QAction::triggered, [this] {
                m_wallet_controller->closeAllWallets(this);
            });
            connect(m_mask_values_action, &QAction::toggled, this, &BitcoinGUI::setPrivacy);
        }
    #endif // ENABLE_WALLET

        connect(new QShortcut(QKeySequence(QtCTRL + QtSHIFT + QtKey_C), this), &QShortcut::activated, this, &BitcoinGUI::showDebugWindowActivateConsole);
        connect(new QShortcut(QKeySequence(QtCTRL + QtSHIFT + QtKey_D), this), &QShortcut::activated, this, &BitcoinGUI::showDebugWindow);
        */
    }
    
    /**
      | Create the menu bar and sub-menus.
      |
      */
    pub fn create_menu_bar(&mut self)  {
        
        todo!();
        /*
            #ifdef Q_OS_MAC
        // Create a decoupled menu bar on Mac which stays even if the window is closed
        appMenuBar = new QMenuBar();
    #else
        // Get the main window's menu bar on other platforms
        appMenuBar = menuBar();
    #endif

        // Configure the menus
        QMenu *file = appMenuBar->addMenu(tr("&File"));
        if(walletFrame)
        {
            file->addAction(m_create_wallet_action);
            file->addAction(m_open_wallet_action);
            file->addAction(m_close_wallet_action);
            file->addAction(m_close_all_wallets_action);
            file->addSeparator();
            file->addAction(openAction);
            file->addAction(backupWalletAction);
            file->addAction(signMessageAction);
            file->addAction(verifyMessageAction);
            file->addAction(m_load_psbt_action);
            file->addAction(m_load_psbt_clipboard_action);
            file->addSeparator();
        }
        file->addAction(quitAction);

        QMenu *settings = appMenuBar->addMenu(tr("&Settings"));
        if(walletFrame)
        {
            settings->addAction(encryptWalletAction);
            settings->addAction(changePassphraseAction);
            settings->addSeparator();
            settings->addAction(m_mask_values_action);
            settings->addSeparator();
        }
        settings->addAction(optionsAction);

        QMenu* window_menu = appMenuBar->addMenu(tr("&Window"));

        QAction* minimize_action = window_menu->addAction(tr("&Minimize"));
        minimize_action->setShortcut(QKeySequence(QtCTRL + QtKey_M));
        connect(minimize_action, &QAction::triggered, [] {
            QApplication::activeWindow()->showMinimized();
        });
        connect(qApp, &QApplication::focusWindowChanged, [minimize_action] (QWindow* window) {
            minimize_action->setEnabled(window != nullptr && (window->flags() & QtDialog) != QtDialog && window->windowState() != QtWindowMinimized);
        });

    #ifdef Q_OS_MAC
        QAction* zoom_action = window_menu->addAction(tr("Zoom"));
        connect(zoom_action, &QAction::triggered, [] {
            QWindow* window = qApp->focusWindow();
            if (window->windowState() != QtWindowMaximized) {
                window->showMaximized();
            } else {
                window->showNormal();
            }
        });

        connect(qApp, &QApplication::focusWindowChanged, [zoom_action] (QWindow* window) {
            zoom_action->setEnabled(window != nullptr);
        });
    #endif

        if (walletFrame) {
    #ifdef Q_OS_MAC
            window_menu->addSeparator();
            QAction* main_window_action = window_menu->addAction(tr("Main Window"));
            connect(main_window_action, &QAction::triggered, [this] {
                gui_util::bringToFront(this);
            });
    #endif
            window_menu->addSeparator();
            window_menu->addAction(usedSendingAddressesAction);
            window_menu->addAction(usedReceivingAddressesAction);
        }

        window_menu->addSeparator();
        for (RPCConsole::TabTypes tab_type : rpcConsole->tabs()) {
            QAction* tab_action = window_menu->addAction(rpcConsole->tabTitle(tab_type));
            tab_action->setShortcut(rpcConsole->tabShortcut(tab_type));
            connect(tab_action, &QAction::triggered, [this, tab_type] {
                rpcConsole->setTabFocus(tab_type);
                showDebugWindow();
            });
        }

        QMenu *help = appMenuBar->addMenu(tr("&Help"));
        help->addAction(showHelpMessageAction);
        help->addSeparator();
        help->addAction(aboutAction);
        help->addAction(aboutQtAction);
        */
    }
    
    /**
      | Create the toolbars
      |
      */
    pub fn create_tool_bars(&mut self)  {
        
        todo!();
        /*
            if(walletFrame)
        {
            QToolBar *toolbar = addToolBar(tr("Tabs toolbar"));
            appToolBar = toolbar;
            toolbar->setMovable(false);
            toolbar->setToolButtonStyle(QtToolButtonTextBesideIcon);
            toolbar->addAction(overviewAction);
            toolbar->addAction(sendCoinsAction);
            toolbar->addAction(receiveCoinsAction);
            toolbar->addAction(historyAction);
            overviewAction->setChecked(true);

    #ifdef ENABLE_WALLET
            QWidget *spacer = new QWidget();
            spacer->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Expanding);
            toolbar->addWidget(spacer);

            m_wallet_selector = new QComboBox();
            m_wallet_selector->setSizeAdjustPolicy(QComboBox::AdjustToContents);
            connect(m_wallet_selector, qOverload<int>(&QComboBox::currentIndexChanged), this, &BitcoinGUI::setCurrentWalletBySelectorIndex);

            m_wallet_selector_label = new QLabel();
            m_wallet_selector_label->setText(tr("Wallet:") + " ");
            m_wallet_selector_label->setBuddy(m_wallet_selector);

            m_wallet_selector_label_action = appToolBar->addWidget(m_wallet_selector_label);
            m_wallet_selector_action = appToolBar->addWidget(m_wallet_selector);

            m_wallet_selector_label_action->setVisible(false);
            m_wallet_selector_action->setVisible(false);
    #endif
        }
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
    pub fn set_client_model(&mut self, 
        client_model: *mut ClientModel,
        tip_info:     *mut BlockAndHeaderTipInfo)  {
        
        todo!();
        /*
            this->clientModel = _clientModel;
        if(_clientModel)
        {
            // Create system tray menu (or setup the dock menu) that late to prevent users from calling actions,
            // while the client has not yet fully loaded
            createTrayIconMenu();

            // Keep up to date with client
            setNetworkActive(m_node.getNetworkActive());
            connect(connectionsControl, &gui_util::ClickableLabel::clicked, [this] {
                gui_util::PopupMenu(m_network_context_menu, QCursor::pos());
            });
            connect(_clientModel, &ClientModel::numConnectionsChanged, this, &BitcoinGUI::setNumConnections);
            connect(_clientModel, &ClientModel::networkActiveChanged, this, &BitcoinGUI::setNetworkActive);

            modalOverlay->setKnownBestHeight(tip_info->header_height, QDateTime::fromSecsSinceEpoch(tip_info->header_time));
            setNumBlocks(tip_info->block_height, QDateTime::fromSecsSinceEpoch(tip_info->block_time), tip_info->verification_progress, false, SynchronizationState::INIT_DOWNLOAD);
            connect(_clientModel, &ClientModel::numBlocksChanged, this, &BitcoinGUI::setNumBlocks);

            // Receive and report messages from client model
            connect(_clientModel, &ClientModel::message, [this](const QString &title, const QString &message, unsigned int style){
                this->message(title, message, style);
            });

            // Show progress dialog
            connect(_clientModel, &ClientModel::showProgress, this, &BitcoinGUI::showProgress);

            rpcConsole->setClientModel(_clientModel, tip_info->block_height, tip_info->block_time, tip_info->verification_progress);

            updateProxyIcon();

    #ifdef ENABLE_WALLET
            if(walletFrame)
            {
                walletFrame->setClientModel(_clientModel);
            }
    #endif // ENABLE_WALLET
            unitDisplayControl->setOptionsModel(_clientModel->getOptionsModel());

            OptionsModel* optionsModel = _clientModel->getOptionsModel();
            if (optionsModel && trayIcon) {
                // be aware of the tray icon disable state change reported by the OptionsModel object.
                connect(optionsModel, &OptionsModel::showTrayIconChanged, trayIcon, &QSystemTrayIcon::setVisible);

                // initialize the disable state of the tray icon with the current value in the model.
                trayIcon->setVisible(optionsModel->getShowTrayIcon());
            }
        } else {
            // Disable possibility to show main window via action
            toggleHideAction->setEnabled(false);
            if(trayIconMenu)
            {
                // Disable context menu on tray icon
                trayIconMenu->clear();
            }
            // Propagate cleared model to child objects
            rpcConsole->setClientModel(nullptr);
    #ifdef ENABLE_WALLET
            if (walletFrame)
            {
                walletFrame->setClientModel(nullptr);
            }
    #endif // ENABLE_WALLET
            unitDisplayControl->setOptionsModel(nullptr);
        }
        */
    }

    #[cfg(ENABLE_WALLET)]
    pub fn set_wallet_controller(&mut self, wallet_controller: *mut WalletController)  {
        
        todo!();
        /*
            assert(!m_wallet_controller);
        assert(wallet_controller);

        m_wallet_controller = wallet_controller;

        m_create_wallet_action->setEnabled(true);
        m_open_wallet_action->setEnabled(true);
        m_open_wallet_action->setMenu(m_open_wallet_menu);

        gui_util::ExceptionSafeConnect(wallet_controller, &WalletController::walletAdded, this, &BitcoinGUI::addWallet);
        connect(wallet_controller, &WalletController::walletRemoved, this, &BitcoinGUI::removeWallet);

        auto activity = new LoadWalletsActivity(m_wallet_controller, this);
        activity->load();
        */
    }
    
    #[cfg(ENABLE_WALLET)]
    pub fn get_wallet_controller(&mut self) -> *mut WalletController {
        
        todo!();
        /*
            return m_wallet_controller;
        */
    }
    
    /**
      | Set the wallet model.
      | 
      | The wallet model represents a bitcoin
      | wallet, and offers access to the list
      | of transactions, address book and sending
      | functionality.
      |
      */
    #[cfg(ENABLE_WALLET)]
    pub fn add_wallet(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
            if (!walletFrame) return;

        WalletView* wallet_view = new WalletView(walletModel, platformStyle, walletFrame);
        if (!walletFrame->addView(wallet_view)) return;

        rpcConsole->addWallet(walletModel);
        if (m_wallet_selector->count() == 0) {
            setWalletActionsEnabled(true);
        } else if (m_wallet_selector->count() == 1) {
            m_wallet_selector_label_action->setVisible(true);
            m_wallet_selector_action->setVisible(true);
        }

        connect(wallet_view, &WalletView::outOfSyncWarningClicked, this, &BitcoinGUI::showModalOverlay);
        connect(wallet_view, &WalletView::transactionClicked, this, &BitcoinGUI::gotoHistoryPage);
        connect(wallet_view, &WalletView::coinsSent, this, &BitcoinGUI::gotoHistoryPage);
        connect(wallet_view, &WalletView::message, [this](const QString& title, const QString& message, unsigned int style) {
            this->message(title, message, style);
        });
        connect(wallet_view, &WalletView::encryptionStatusChanged, this, &BitcoinGUI::updateWalletStatus);
        connect(wallet_view, &WalletView::incomingTransaction, this, &BitcoinGUI::incomingTransaction);
        connect(this, &BitcoinGUI::setPrivacy, wallet_view, &WalletView::setPrivacy);
        wallet_view->setPrivacy(isPrivacyModeActivated());
        const QString display_name = walletModel->getDisplayName();
        m_wallet_selector->addItem(display_name, QVariant::fromValue(walletModel));
        */
    }
    
    #[cfg(ENABLE_WALLET)]
    pub fn remove_wallet(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
            if (!walletFrame) return;

        labelWalletHDStatusIcon->hide();
        labelWalletEncryptionIcon->hide();

        int index = m_wallet_selector->findData(QVariant::fromValue(walletModel));
        m_wallet_selector->removeItem(index);
        if (m_wallet_selector->count() == 0) {
            setWalletActionsEnabled(false);
            overviewAction->setChecked(true);
        } else if (m_wallet_selector->count() == 1) {
            m_wallet_selector_label_action->setVisible(false);
            m_wallet_selector_action->setVisible(false);
        }
        rpcConsole->removeWallet(walletModel);
        walletFrame->removeWallet(walletModel);
        updateWindowTitle();
        */
    }
    
    #[cfg(ENABLE_WALLET)]
    pub fn set_current_wallet(&mut self, wallet_model: *mut WalletModel)  {
        
        todo!();
        /*
            if (!walletFrame) return;
        walletFrame->setCurrentWallet(wallet_model);
        for (int index = 0; index < m_wallet_selector->count(); ++index) {
            if (m_wallet_selector->itemData(index).value<WalletModel*>() == wallet_model) {
                m_wallet_selector->setCurrentIndex(index);
                break;
            }
        }
        updateWindowTitle();
        */
    }
    
    #[cfg(ENABLE_WALLET)]
    pub fn set_current_wallet_by_selector_index(&mut self, index: i32)  {
        
        todo!();
        /*
            WalletModel* wallet_model = m_wallet_selector->itemData(index).value<WalletModel*>();
        if (wallet_model) setCurrentWallet(wallet_model);
        */
    }
    
    #[cfg(ENABLE_WALLET)]
    pub fn remove_all_wallets(&mut self)  {
        
        todo!();
        /*
            if(!walletFrame)
            return;
        setWalletActionsEnabled(false);
        walletFrame->removeAllWallets();
        */
    }
    
    /**
      | Enable or disable all wallet-related
      | actions
      |
      */
    pub fn set_wallet_actions_enabled(&mut self, enabled: bool)  {
        
        todo!();
        /*
            overviewAction->setEnabled(enabled);
        sendCoinsAction->setEnabled(enabled);
        sendCoinsMenuAction->setEnabled(enabled);
        receiveCoinsAction->setEnabled(enabled);
        receiveCoinsMenuAction->setEnabled(enabled);
        historyAction->setEnabled(enabled);
        encryptWalletAction->setEnabled(enabled);
        backupWalletAction->setEnabled(enabled);
        changePassphraseAction->setEnabled(enabled);
        signMessageAction->setEnabled(enabled);
        verifyMessageAction->setEnabled(enabled);
        usedSendingAddressesAction->setEnabled(enabled);
        usedReceivingAddressesAction->setEnabled(enabled);
        openAction->setEnabled(enabled);
        m_close_wallet_action->setEnabled(enabled);
        m_close_all_wallets_action->setEnabled(enabled);
        */
    }
    
    /**
      | Create system tray icon and notification
      |
      */
    pub fn create_tray_icon(&mut self)  {
        
        todo!();
        /*
            assert(QSystemTrayIcon::isSystemTrayAvailable());

    #ifndef Q_OS_MAC
        if (QSystemTrayIcon::isSystemTrayAvailable()) {
            trayIcon = new QSystemTrayIcon(m_network_style->getTrayAndWindowIcon(), this);
            QString toolTip = tr("%1 client").arg(PACKAGE_NAME) + " " + m_network_style->getTitleAddText();
            trayIcon->setToolTip(toolTip);
        }
    #endif
        */
    }
    
    /**
      | Create system tray menu (or setup the
      | dock menu)
      |
      */
    pub fn create_tray_icon_menu(&mut self)  {
        
        todo!();
        /*
            #ifndef Q_OS_MAC
        // return if trayIcon is unset (only on non-macOSes)
        if (!trayIcon)
            return;

        trayIcon->setContextMenu(trayIconMenu.get());
        connect(trayIcon, &QSystemTrayIcon::activated, this, &BitcoinGUI::trayIconActivated);
    #else
        // Note: On macOS, the Dock icon is used to provide the tray's functionality.
        MacDockIconHandler *dockIconHandler = MacDockIconHandler::instance();
        connect(dockIconHandler, &MacDockIconHandler::dockIconClicked, this, &BitcoinGUI::macosDockIconActivated);
        trayIconMenu->setAsDockMenu();
    #endif

        // Configuration of the tray icon (or Dock icon) menu
    #ifndef Q_OS_MAC
        // Note: On macOS, the Dock icon's menu already has Show / Hide action.
        trayIconMenu->addAction(toggleHideAction);
        trayIconMenu->addSeparator();
    #endif
        if (enableWallet) {
            trayIconMenu->addAction(sendCoinsMenuAction);
            trayIconMenu->addAction(receiveCoinsMenuAction);
            trayIconMenu->addSeparator();
            trayIconMenu->addAction(signMessageAction);
            trayIconMenu->addAction(verifyMessageAction);
            trayIconMenu->addSeparator();
        }
        trayIconMenu->addAction(optionsAction);
        trayIconMenu->addAction(openRPCConsoleAction);
    #ifndef Q_OS_MAC // This is built-in on macOS
        trayIconMenu->addSeparator();
        trayIconMenu->addAction(quitAction);
    #endif
        */
    }

    /**
      | Handle tray icon clicked
      |
      */
    #[Q_SLOT]
    #[cfg(not(Q_OS_MAC))]
    pub fn tray_icon_activated(&mut self, reason: QSystemTrayIconActivationReason)  {
        
        todo!();
        /*
            if(reason == QSystemTrayIcon::Trigger)
        {
            // Click on system tray icon triggers show/hide of the main window
            toggleHidden();
        }
        */
    }

    /**
      | Handle macOS Dock icon clicked
      |
      */
    #[Q_SLOT]
    #[cfg(Q_OS_MAC)]
    pub fn macos_dock_icon_activated(&mut self)  {
        
        todo!();
        /*
            show();
        activateWindow();
        */
    }
    
    /**
      | Show configuration dialog
      |
      */
    #[Q_SLOT]
    pub fn options_clicked(&mut self)  {
        
        todo!();
        /*
            openOptionsDialogWithTab(OptionsDialog::TAB_MAIN);
        */
    }
    
    /**
      | Show about dialog
      |
      */
    #[Q_SLOT]
    pub fn about_clicked(&mut self)  {
        
        todo!();
        /*
            if(!clientModel)
            return;

        auto dlg = new HelpMessageDialog(this, /* about */ true);
        gui_util::ShowModalDialogAndDeleteOnClose(dlg);
        */
    }
    
    /**
      | Show debug window
      |
      */
    #[Q_SLOT]
    pub fn show_debug_window(&mut self)  {
        
        todo!();
        /*
            gui_util::bringToFront(rpcConsole);
        Q_EMIT consoleShown(rpcConsole);
        */
    }
    
    /**
      | Show debug window and set focus to the
      | console
      |
      */
    #[Q_SLOT]
    pub fn show_debug_window_activate_console(&mut self)  {
        
        todo!();
        /*
            rpcConsole->setTabFocus(RPCConsole::TabTypes::CONSOLE);
        showDebugWindow();
        */
    }
    
    /**
      | Show help message dialog
      |
      */
    #[Q_SLOT]
    pub fn show_help_message_clicked(&mut self)  {
        
        todo!();
        /*
            gui_util::bringToFront(helpMessageDialog);
        */
    }

    /**
      | Show open dialog
      |
      */
    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn open_clicked(&mut self)  {
        
        todo!();
        /*
            OpenURIDialog dlg(this);
        if(dlg.exec())
        {
            Q_EMIT receivedURI(dlg.getURI());
        }
        */
    }
    
    /**
      | Switch to overview (home) page
      |
      */
    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn goto_overview_page(&mut self)  {
        
        todo!();
        /*
            overviewAction->setChecked(true);
        if (walletFrame) walletFrame->gotoOverviewPage();
        */
    }
    
    /**
      | Switch to history (transactions) page
      |
      */
    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn goto_history_page(&mut self)  {
        
        todo!();
        /*
            historyAction->setChecked(true);
        if (walletFrame) walletFrame->gotoHistoryPage();
        */
    }
    
    /**
      | Switch to receive coins page
      |
      */
    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn goto_receive_coins_page(&mut self)  {
        
        todo!();
        /*
            receiveCoinsAction->setChecked(true);
        if (walletFrame) walletFrame->gotoReceiveCoinsPage();
        */
    }
    
    /**
      | Switch to send coins page
      |
      */
    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn goto_send_coins_page(&mut self, addr: Option<&str>)  {

        let addr: &str = addr.unwrap_or("");
        
        todo!();
        /*
            sendCoinsAction->setChecked(true);
        if (walletFrame) walletFrame->gotoSendCoinsPage(addr);
        */
    }
    
    /**
      | Show Sign/Verify Message dialog and
      | switch to sign message tab
      |
      */
    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn goto_sign_message_tab(&mut self, addr: Option<&str>)  {

        let addr: &str = addr.unwrap_or("");
        
        todo!();
        /*
            if (walletFrame) walletFrame->gotoSignMessageTab(addr);
        */
    }
    
    /**
      | Show Sign/Verify Message dialog and
      | switch to verify message tab
      |
      */
    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn goto_verify_message_tab(&mut self, addr: Option<&str>)  {

        let addr: &str = addr.unwrap_or("");
        
        todo!();
        /*
            if (walletFrame) walletFrame->gotoVerifyMessageTab(addr);
        */
    }
    
    /**
      | Load Partially Signed Bitcoin Transaction
      | from file or clipboard
      |
      */
    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn goto_loadpsbt(&mut self, from_clipboard: Option<bool>)  {

        let from_clipboard: bool = from_clipboard.unwrap_or(false);
        
        todo!();
        /*
            if (walletFrame) walletFrame->gotoLoadPSBT(from_clipboard);
        */
    }
    
    /**
      | Update UI with latest network info from
      | model.
      |
      */
    pub fn update_network_state(&mut self)  {
        
        todo!();
        /*
            int count = clientModel->getNumConnections();
        QString icon;
        switch(count)
        {
        case 0: icon = ":/icons/connect_0"; break;
        case 1: case 2: case 3: icon = ":/icons/connect_1"; break;
        case 4: case 5: case 6: icon = ":/icons/connect_2"; break;
        case 7: case 8: case 9: icon = ":/icons/connect_3"; break;
        default: icon = ":/icons/connect_4"; break;
        }

        QString tooltip;

        if (m_node.getNetworkActive()) {
            //: A substring of the tooltip.
            tooltip = tr("%n active connection(s) to Bitcoin network.", "", count);
        } else {
            //: A substring of the tooltip.
            tooltip = tr("Network activity disabled.");
            icon = ":/icons/network_disabled";
        }

        // Don't word-wrap this (fixed-width) tooltip
        tooltip = QLatin1String("<nobr>") + tooltip + QLatin1String("<br>") +
                  //: A substring of the tooltip. "More actions" are available via the context menu.
                  tr("Click for more actions.") + QLatin1String("</nobr>");
        connectionsControl->setToolTip(tooltip);

        connectionsControl->setThemedPixmap(icon, STATUSBAR_ICONSIZE, STATUSBAR_ICONSIZE);
        */
    }
    
    /**
      | Set number of connections shown in the
      | UI
      |
      */
    #[Q_SLOT]
    pub fn set_num_connections(&mut self, count: i32)  {
        
        todo!();
        /*
            updateNetworkState();
        */
    }
    
    /**
      | Set network state shown in the UI
      |
      */
    pub fn set_network_active(&mut self, network_active: bool)  {
        
        todo!();
        /*
            updateNetworkState();
        m_network_context_menu->clear();
        m_network_context_menu->addAction(
            //: A context menu item. The "Peers tab" is an element of the "Node window".
            tr("Show Peers tab"),
            [this] {
                rpcConsole->setTabFocus(RPCConsole::TabTypes::PEERS);
                showDebugWindow();
            });
        m_network_context_menu->addAction(
            network_active ?
                //: A context menu item.
                tr("Disable network activity") :
                //: A context menu item. The network activity was disabled previously.
                tr("Enable network activity"),
            [this, new_state = !network_active] { m_node.setNetworkActive(new_state); });
        */
    }
    
    pub fn update_headers_sync_progress_label(&mut self)  {
        
        todo!();
        /*
            int64_t headersTipTime = clientModel->getHeaderTipTime();
        int headersTipHeight = clientModel->getHeaderTipHeight();
        int estHeadersLeft = (GetTime() - headersTipTime) / Params().GetConsensus().nPowTargetSpacing;
        if (estHeadersLeft > HEADER_HEIGHT_DELTA_SYNC)
            progressBarLabel->setText(tr("Syncing Headers (%1%)…").arg(QString::number(100.0 / (headersTipHeight+estHeadersLeft)*headersTipHeight, 'f', 1)));
        */
    }
    
    /**
      | Open the OptionsDialog on the specified
      | tab index
      |
      */
    pub fn open_options_dialog_with_tab(&mut self, tab: OptionsDialogTab)  {
        
        todo!();
        /*
            if (!clientModel || !clientModel->getOptionsModel())
            return;

        auto dlg = new OptionsDialog(this, enableWallet);
        connect(dlg, &OptionsDialog::quitOnReset, this, &BitcoinGUI::quitRequested);
        dlg->setCurrentTab(tab);
        dlg->setModel(clientModel->getOptionsModel());
        gui_util::ShowModalDialogAndDeleteOnClose(dlg);
        */
    }
    
    /**
      | Set number of blocks and last block date
      | shown in the UI
      |
      */
    pub fn set_num_blocks(&mut self, 
        count:                   i32,
        block_date:              &QDateTime,
        n_verification_progress: f64,
        header:                  bool,
        sync_state:              SynchronizationState)  {
        
        todo!();
        /*
            // Disabling macOS App Nap on initial sync, disk and reindex operations.
    #ifdef Q_OS_MAC
        if (sync_state == SynchronizationState::POST_INIT) {
            m_app_nap_inhibitor->enableAppNap();
        } else {
            m_app_nap_inhibitor->disableAppNap();
        }
    #endif

        if (modalOverlay)
        {
            if (header)
                modalOverlay->setKnownBestHeight(count, blockDate);
            else
                modalOverlay->tipUpdate(count, blockDate, nVerificationProgress);
        }
        if (!clientModel)
            return;

        // Prevent orphan statusbar messages (e.g. hover Quit in main menu, wait until chain-sync starts -> garbled text)
        statusBar()->clearMessage();

        // Acquire current block source
        enum BlockSource blockSource = clientModel->getBlockSource();
        switch (blockSource) {
            case BlockSource::NETWORK:
                if (header) {
                    updateHeadersSyncProgressLabel();
                    return;
                }
                progressBarLabel->setText(tr("Synchronizing with network…"));
                updateHeadersSyncProgressLabel();
                break;
            case BlockSource::DISK:
                if (header) {
                    progressBarLabel->setText(tr("Indexing blocks on disk…"));
                } else {
                    progressBarLabel->setText(tr("Processing blocks on disk…"));
                }
                break;
            case BlockSource::REINDEX:
                progressBarLabel->setText(tr("Reindexing blocks on disk…"));
                break;
            case BlockSource::NONE:
                if (header) {
                    return;
                }
                progressBarLabel->setText(tr("Connecting to peers…"));
                break;
        }

        QString tooltip;

        QDateTime currentDate = QDateTime::currentDateTime();
        i64 secs = blockDate.secsTo(currentDate);

        tooltip = tr("Processed %n block(s) of transaction history.", "", count);

        // Set icon state: spinning if catching up, tick otherwise
        if (secs < MAX_BLOCK_TIME_GAP) {
            tooltip = tr("Up to date") + QString(".<br>") + tooltip;
            labelBlocksIcon->setThemedPixmap(QStringLiteral(":/icons/synced"), STATUSBAR_ICONSIZE, STATUSBAR_ICONSIZE);

    #ifdef ENABLE_WALLET
            if(walletFrame)
            {
                walletFrame->showOutOfSyncWarning(false);
                modalOverlay->showHide(true, true);
            }
    #endif // ENABLE_WALLET

            progressBarLabel->setVisible(false);
            progressBar->setVisible(false);
        }
        else
        {
            QString timeBehindText = gui_util::formatNiceTimeOffset(secs);

            progressBarLabel->setVisible(true);
            progressBar->setFormat(tr("%1 behind").arg(timeBehindText));
            progressBar->setMaximum(1000000000);
            progressBar->setValue(nVerificationProgress * 1000000000.0 + 0.5);
            progressBar->setVisible(true);

            tooltip = tr("Catching up…") + QString("<br>") + tooltip;
            if(count != prevBlocks)
            {
                labelBlocksIcon->setThemedPixmap(
                    QString(":/animation/spinner-%1").arg(spinnerFrame, 3, 10, QChar('0')),
                    STATUSBAR_ICONSIZE, STATUSBAR_ICONSIZE);
                spinnerFrame = (spinnerFrame + 1) % SPINNER_FRAMES;
            }
            prevBlocks = count;

    #ifdef ENABLE_WALLET
            if(walletFrame)
            {
                walletFrame->showOutOfSyncWarning(true);
                modalOverlay->showHide();
            }
    #endif // ENABLE_WALLET

            tooltip += QString("<br>");
            tooltip += tr("Last received block was generated %1 ago.").arg(timeBehindText);
            tooltip += QString("<br>");
            tooltip += tr("Transactions after this will not yet be visible.");
        }

        // Don't word-wrap this (fixed-width) tooltip
        tooltip = QString("<nobr>") + tooltip + QString("</nobr>");

        labelBlocksIcon->setToolTip(tooltip);
        progressBarLabel->setToolTip(tooltip);
        progressBar->setToolTip(tooltip);
        */
    }
    
    /**
      | Notify the user of an event from the core
      | network or transaction handling code.
      | 
      | -----------
      | @param[in] title
      | 
      | the message box / notification title
      | ----------
      | @param[in] message
      | 
      | the displayed text
      | ----------
      | @param[in] style
      | 
      | modality and style definitions (icon
      | and used buttons - buttons only for message
      | boxes) @see CClientUIInterface::MessageBoxFlags
      | ----------
      | @param[in] ret
      | 
      | pointer to a bool that will be modified
      | to whether Ok was clicked (modal only)
      | ----------
      | @param[in] detailed_message
      | 
      | the text to be displayed in the details
      | area
      |
      */
    pub fn message(&mut self, 
        title:            &str,
        message:          &str,
        style:            u32,
        ret:              *mut bool,
        detailed_message: &str)  {
        
        todo!();
        /*
            // Default title. On macOS, the window title is ignored (as required by the macOS Guidelines).
        QString strTitle{PACKAGE_NAME};
        // Default to information icon
        int nMBoxIcon = QMessageBox::Information;
        int nNotifyIcon = Notificator::Information;

        QString msgType;
        if (!title.isEmpty()) {
            msgType = title;
        } else {
            switch (style) {
            case CClientUIInterface::MSG_ERROR:
                msgType = tr("Error");
                message = tr("Error: %1").arg(message);
                break;
            case CClientUIInterface::MSG_WARNING:
                msgType = tr("Warning");
                message = tr("Warning: %1").arg(message);
                break;
            case CClientUIInterface::MSG_INFORMATION:
                msgType = tr("Information");
                // No need to prepend the prefix here.
                break;
            default:
                break;
            }
        }

        if (!msgType.isEmpty()) {
            strTitle += " - " + msgType;
        }

        if (style & CClientUIInterface::ICON_ERROR) {
            nMBoxIcon = QMessageBox::Critical;
            nNotifyIcon = Notificator::Critical;
        } else if (style & CClientUIInterface::ICON_WARNING) {
            nMBoxIcon = QMessageBox::Warning;
            nNotifyIcon = Notificator::Warning;
        }

        if (style & CClientUIInterface::MODAL) {
            // Check for buttons, use OK as default, if none was supplied
            QMessageBox::StandardButton buttons;
            if (!(buttons = (QMessageBox::StandardButton)(style & CClientUIInterface::BTN_MASK)))
                buttons = QMessageBox::Ok;

            showNormalIfMinimized();
            QMessageBox mBox(static_cast<QMessageBox::Icon>(nMBoxIcon), strTitle, message, buttons, this);
            mBox.setTextFormat(QtPlainText);
            mBox.setDetailedText(detailed_message);
            int r = mBox.exec();
            if (ret != nullptr)
                *ret = r == QMessageBox::Ok;
        } else {
            notificator->notify(static_cast<Notificator::Class>(nNotifyIcon), strTitle, message);
        }
        */
    }
    
    pub fn change_event(&mut self, e: *mut QEvent)  {
        
        todo!();
        /*
            if (e->type() == QEvent::PaletteChange) {
            overviewAction->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/overview")));
            sendCoinsAction->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/send")));
            receiveCoinsAction->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/receiving_addresses")));
            historyAction->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/history")));
        }

        QMainWindow::changeEvent(e);

    #ifndef Q_OS_MAC // Ignored on Mac
        if(e->type() == QEvent::WindowStateChange)
        {
            if(clientModel && clientModel->getOptionsModel() && clientModel->getOptionsModel()->getMinimizeToTray())
            {
                QWindowStateChangeEvent *wsevt = static_cast<QWindowStateChangeEvent*>(e);
                if(!(wsevt->oldState() & QtWindowMinimized) && isMinimized())
                {
                    QTimer::singleShot(0, this, &BitcoinGUI::hide);
                    e->ignore();
                }
                else if((wsevt->oldState() & QtWindowMinimized) && !isMinimized())
                {
                    QTimer::singleShot(0, this, &BitcoinGUI::show);
                    e->ignore();
                }
            }
        }
    #endif
        */
    }
    
    pub fn close_event(&mut self, event: *mut QCloseEvent)  {
        
        todo!();
        /*
            #ifndef Q_OS_MAC // Ignored on Mac
        if(clientModel && clientModel->getOptionsModel())
        {
            if(!clientModel->getOptionsModel()->getMinimizeOnClose())
            {
                // close rpcConsole in case it was open to make some space for the shutdown window
                rpcConsole->close();

                Q_EMIT quitRequested();
            }
            else
            {
                QMainWindow::showMinimized();
                event->ignore();
            }
        }
    #else
        QMainWindow::closeEvent(event);
    #endif
        */
    }
    
    pub fn show_event(&mut self, event: *mut QShowEvent)  {
        
        todo!();
        /*
            // enable the debug window when the main window shows up
        openRPCConsoleAction->setEnabled(true);
        aboutAction->setEnabled(true);
        optionsAction->setEnabled(true);
        */
    }

    /**
      | Show incoming transaction notification
      | for new transactions.
      |
      */
    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn incoming_transaction(&mut self, 
        date:        &str,
        unit:        i32,
        amount:      &Amount,
        ty:          &str,
        address:     &str,
        label:       &str,
        wallet_name: &str)  {
        
        todo!();
        /*
            // On new transaction, make an info balloon
        QString msg = tr("Date: %1\n").arg(date) +
                      tr("Amount: %1\n").arg(BitcoinUnits::formatWithUnit(unit, amount, true));
        if (m_node.walletClient().getWallets().size() > 1 && !walletName.isEmpty()) {
            msg += tr("Wallet: %1\n").arg(walletName);
        }
        msg += tr("Type: %1\n").arg(type);
        if (!label.isEmpty())
            msg += tr("Label: %1\n").arg(label);
        else if (!address.isEmpty())
            msg += tr("Address: %1\n").arg(address);
        message((amount)<0 ? tr("Sent transaction") : tr("Incoming transaction"),
                 msg, CClientUIInterface::MSG_INFORMATION);
        */
    }
    
    pub fn drag_enter_event(&mut self, event: *mut QDragEnterEvent)  {
        
        todo!();
        /*
            // Accept only URIs
        if(event->mimeData()->hasUrls())
            event->acceptProposedAction();
        */
    }
    
    pub fn drop_event(&mut self, event: *mut QDropEvent)  {
        
        todo!();
        /*
            if(event->mimeData()->hasUrls())
        {
            for (const QUrl &uri : event->mimeData()->urls())
            {
                Q_EMIT receivedURI(uri.toString());
            }
        }
        event->acceptProposedAction();
        */
    }
    
    pub fn event_filter(&mut self, 
        object: *mut QObject,
        event:  *mut QEvent) -> bool {
        
        todo!();
        /*
            // Catch status tip events
        if (event->type() == QEvent::StatusTip)
        {
            // Prevent adding text from setStatusTip(), if we currently use the status bar for displaying other stuff
            if (progressBarLabel->isVisible() || progressBar->isVisible())
                return true;
        }
        return QMainWindow::eventFilter(object, event);
        */
    }

    #[Q_SLOT]
    #[cfg(ENABLE_WALLET)]
    pub fn handle_payment_request(&mut self, recipient: &SendCoinsRecipient) -> bool {
        
        todo!();
        /*
            // URI has to be valid
        if (walletFrame && walletFrame->handlePaymentRequest(recipient))
        {
            showNormalIfMinimized();
            gotoSendCoinsPage();
            return true;
        }
        return false;
        */
    }
    
    /**
      | Set the hd-enabled status as shown in
      | the UI.
      | 
      | -----------
      | @param[in] hdEnabled
      | 
      | current hd enabled status @see WalletModel::EncryptionStatus
      |
      */
    #[cfg(ENABLE_WALLET)]
    pub fn set_hd_status(&mut self, 
        privkey_disabled: bool,
        hd_enabled:       i32)  {
        
        todo!();
        /*
            labelWalletHDStatusIcon->setThemedPixmap(privkeyDisabled ? QStringLiteral(":/icons/eye") : hdEnabled ? QStringLiteral(":/icons/hd_enabled") : QStringLiteral(":/icons/hd_disabled"), STATUSBAR_ICONSIZE, STATUSBAR_ICONSIZE);
        labelWalletHDStatusIcon->setToolTip(privkeyDisabled ? tr("Private key <b>disabled</b>") : hdEnabled ? tr("HD key generation is <b>enabled</b>") : tr("HD key generation is <b>disabled</b>"));
        labelWalletHDStatusIcon->show();
        */
    }
    
    /**
      | Set the encryption status as shown in
      | the UI.
      | 
      | -----------
      | @param[in] status
      | 
      | current encryption status @see WalletModel::EncryptionStatus
      |
      */
    #[cfg(ENABLE_WALLET)]
    pub fn set_encryption_status(&mut self, status: i32)  {
        
        todo!();
        /*
            switch(status)
        {
        case WalletModel::Unencrypted:
            labelWalletEncryptionIcon->hide();
            encryptWalletAction->setChecked(false);
            changePassphraseAction->setEnabled(false);
            encryptWalletAction->setEnabled(true);
            break;
        case WalletModel::Unlocked:
            labelWalletEncryptionIcon->show();
            labelWalletEncryptionIcon->setThemedPixmap(QStringLiteral(":/icons/lock_open"), STATUSBAR_ICONSIZE, STATUSBAR_ICONSIZE);
            labelWalletEncryptionIcon->setToolTip(tr("Wallet is <b>encrypted</b> and currently <b>unlocked</b>"));
            encryptWalletAction->setChecked(true);
            changePassphraseAction->setEnabled(true);
            encryptWalletAction->setEnabled(false);
            break;
        case WalletModel::Locked:
            labelWalletEncryptionIcon->show();
            labelWalletEncryptionIcon->setThemedPixmap(QStringLiteral(":/icons/lock_closed"), STATUSBAR_ICONSIZE, STATUSBAR_ICONSIZE);
            labelWalletEncryptionIcon->setToolTip(tr("Wallet is <b>encrypted</b> and currently <b>locked</b>"));
            encryptWalletAction->setChecked(true);
            changePassphraseAction->setEnabled(true);
            encryptWalletAction->setEnabled(false);
            break;
        }
        */
    }
    
    /**
      | Set the UI status indicators based on
      | the currently selected wallet.
      |
      */
    #[cfg(ENABLE_WALLET)]
    pub fn update_wallet_status(&mut self)  {
        
        todo!();
        /*
            assert(walletFrame);

        WalletView * const walletView = walletFrame->currentWalletView();
        if (!walletView) {
            return;
        }
        WalletModel * const walletModel = walletView->getWalletModel();
        setEncryptionStatus(walletModel->getEncryptionStatus());
        setHDStatus(walletModel->wallet().privateKeysDisabled(), walletModel->wallet().hdEnabled());
        */
    }
    
    /**
      | Set the proxy-enabled icon as shown
      | in the UI.
      |
      */
    pub fn update_proxy_icon(&mut self)  {
        
        todo!();
        /*
            std::string ip_port;
        bool proxy_enabled = clientModel->getProxyInfo(ip_port);

        if (proxy_enabled) {
            if (!gui_util::HasPixmap(labelProxyIcon)) {
                QString ip_port_q = QString::fromStdString(ip_port);
                labelProxyIcon->setThemedPixmap((":/icons/proxy"), STATUSBAR_ICONSIZE, STATUSBAR_ICONSIZE);
                labelProxyIcon->setToolTip(tr("Proxy is <b>enabled</b>: %1").arg(ip_port_q));
            } else {
                labelProxyIcon->show();
            }
        } else {
            labelProxyIcon->hide();
        }
        */
    }
    
    pub fn update_window_title(&mut self)  {
        
        todo!();
        /*
            QString window_title = PACKAGE_NAME;
    #ifdef ENABLE_WALLET
        if (walletFrame) {
            WalletModel* const wallet_model = walletFrame->currentWalletModel();
            if (wallet_model && !wallet_model->getWalletName().isEmpty()) {
                window_title += " - " + wallet_model->getDisplayName();
            }
        }
    #endif
        if (!m_network_style->getTitleAddText().isEmpty()) {
            window_title += " - " + m_network_style->getTitleAddText();
        }
        setWindowTitle(window_title);
        */
    }
    
    #[Q_SLOT]
    pub fn show_normal_if_minimized(&mut self, toggle_hidden: bool)  {
        
        todo!();
        /*
            if(!clientModel)
            return;

        if (!isHidden() && !isMinimized() && !gui_util::isObscured(this) && fToggleHidden) {
            hide();
        } else {
            gui_util::bringToFront(this);
        }
        */
    }
    
    /**
      | Simply calls showNormalIfMinimized(true)
      | for use in SLOT() macro
      |
      */
    #[Q_SLOT]
    pub fn toggle_hidden(&mut self)  {
        
        todo!();
        /*
            showNormalIfMinimized(true);
        */
    }
    
    /**
      | called by a timer to check if ShutdownRequested()
      | has been set *
      |
      */
    #[Q_SLOT]
    pub fn detect_shutdown(&mut self)  {
        
        todo!();
        /*
            if (m_node.shutdownRequested())
        {
            if(rpcConsole)
                rpcConsole->hide();
            Q_EMIT quitRequested();
        }
        */
    }
    
    /**
      | Show progress dialog e.g. for verifychain
      |
      */
    #[Q_SLOT]
    pub fn show_progress(&mut self, 
        title:      &str,
        n_progress: i32)  {
        
        todo!();
        /*
            if (nProgress == 0) {
            progressDialog = new QProgressDialog(title, QString(), 0, 100);
            gui_util::PolishProgressDialog(progressDialog);
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
            progressDialog->setValue(nProgress);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn show_modal_overlay(&mut self)  {
        
        todo!();
        /*
            if (modalOverlay && (progressBar->isVisible() || modalOverlay->isLayerVisible()))
            modalOverlay->toggleVisibility();
        */
    }
    
    /**
      | Connect core signals to GUI client
      |
      */
    pub fn subscribe_to_core_signals(&mut self)  {
        
        todo!();
        /*
            // Connect signals to client
        m_handler_message_box = m_node.handleMessageBox(std::bind(ThreadSafeMessageBox, this, std::placeholders::_1, std::placeholders::_2, std::placeholders::_3));
        m_handler_question = m_node.handleQuestion(std::bind(ThreadSafeMessageBox, this, std::placeholders::_1, std::placeholders::_3, std::placeholders::_4));
        */
    }
    
    /**
      | Disconnect core signals from GUI client
      |
      */
    pub fn unsubscribe_from_core_signals(&mut self)  {
        
        todo!();
        /*
            // Disconnect signals from client
        m_handler_message_box->disconnect();
        m_handler_question->disconnect();
        */
    }
    
    pub fn is_privacy_mode_activated(&self) -> bool {
        
        todo!();
        /*
            assert(m_mask_values_action);
        return m_mask_values_action->isChecked();
        */
    }
}

impl UnitDisplayStatusBarControl {

    pub fn new(platform_style: *const PlatformStyle) -> Self {
    
        todo!();
        /*


            : optionsModel(nullptr),
          menu(nullptr),
          m_platform_style{platformStyle}
        createContextMenu();
        setToolTip(tr("Unit to show amounts in. Click to select another unit."));
        QList<BitcoinUnits::Unit> units = BitcoinUnits::availableUnits();
        int max_width = 0;
        const QFontMetrics fm(font());
        for (const BitcoinUnits::Unit unit : units)
        {
            max_width = qMax(max_width, gui_util::TextWidth(fm, BitcoinUnits::longName(unit)));
        }
        setMinimumSize(max_width, 0);
        setAlignment(QtAlignRight | QtAlignVCenter);
        setStyleSheet(QString("QLabel { color : %1 }").arg(m_platform_style->SingleColor().name()));
        */
    }

    /**
      | So that it responds to button clicks
      |
      */
    pub fn mouse_press_event(&mut self, event: *mut QMouseEvent)  {
        
        todo!();
        /*
            onDisplayUnitsClicked(event->pos());
        */
    }
    
    pub fn change_event(&mut self, e: *mut QEvent)  {
        
        todo!();
        /*
            if (e->type() == QEvent::PaletteChange) {
            QString style = QString("QLabel { color : %1 }").arg(m_platform_style->SingleColor().name());
            if (style != styleSheet()) {
                setStyleSheet(style);
            }
        }

        QLabel::changeEvent(e);
        */
    }

    /**
      | Creates context menu, its actions,
      | and wires up all the relevant signals
      | for mouse events.
      |
      */
    pub fn create_context_menu(&mut self)  {
        
        todo!();
        /*
            menu = new QMenu(this);
        for (const BitcoinUnits::Unit u : BitcoinUnits::availableUnits()) {
            menu->addAction(BitcoinUnits::longName(u))->setData(QVariant(u));
        }
        connect(menu, &QMenu::triggered, this, &UnitDisplayStatusBarControl::onMenuSelection);
        */
    }

    /**
      | Lets the control know about the Options
      | Model (and its signals)
      |
      */
    pub fn set_options_model(&mut self, options_model: *mut OptionsModel)  {
        
        todo!();
        /*
            if (_optionsModel)
        {
            this->optionsModel = _optionsModel;

            // be aware of a display unit change reported by the OptionsModel object.
            connect(_optionsModel, &OptionsModel::displayUnitChanged, this, &UnitDisplayStatusBarControl::updateDisplayUnit);

            // initialize the display units label with the current value in the model.
            updateDisplayUnit(_optionsModel->getDisplayUnit());
        }
        */
    }

    /**
      | When Display Units are changed on OptionsModel
      | it will refresh the display text of the
      | control on the status bar
      |
      */
    #[Q_SLOT]
    pub fn update_display_unit(&mut self, new_units: i32)  {
        
        todo!();
        /*
            setText(BitcoinUnits::longName(newUnits));
        */
    }

    /**
      | Shows context menu with Display Unit
      | options by the mouse coordinates
      |
      */
    pub fn on_display_units_clicked(&mut self, point: &QPoint)  {
        
        todo!();
        /*
            QPoint globalPos = mapToGlobal(point);
        menu->exec(globalPos);
        */
    }

    /**
      | Tells underlying optionsModel to update
      | its current display unit.
      |
      */
    #[Q_SLOT]
    pub fn on_menu_selection(&mut self, action: *mut QAction)  {
        
        todo!();
        /*
            if (action)
        {
            optionsModel->setDisplayUnit(action->data());
        }
        */
    }
}

pub fn thread_safe_message_box(
        gui:     *mut BitcoinGUI,
        message: &BilingualStr,
        caption: &str,
        style:   u32) -> bool {
    
    todo!();
        /*
            bool modal = (style & CClientUIInterface::MODAL);
        // The SECURE flag has no effect in the Qt GUI.
        // bool secure = (style & CClientUIInterface::SECURE);
        style &= ~CClientUIInterface::SECURE;
        bool ret = false;

        QString detailed_message; // This is original message, in English, for googling and referencing.
        if (message.original != message.translated) {
            detailed_message = BitcoinGUI::tr("Original message:") + "\n" + QString::fromStdString(message.original);
        }

        // In case of modal message, use blocking connection to wait for user to click a button
        bool invoked = QMetaObject::invokeMethod(gui, "message",
                                   modal ? gui_util::blockingGUIThreadConnection() : QtQueuedConnection,
                                   Q_ARG(QString, QString::fromStdString(caption)),
                                   Q_ARG(QString, QString::fromStdString(message.translated)),
                                   Q_ARG(unsigned int, style),
                                   Q_ARG(bool*, &ret),
                                   Q_ARG(QString, detailed_message));
        assert(invoked);
        return ret;
        */
}
