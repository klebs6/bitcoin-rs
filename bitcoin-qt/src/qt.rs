// ---------------- [ File: bitcoin-qt/src/qt.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoin.h]

/**
  | Main Bitcoin application object
  |
  */
#[Q_OBJECT]
pub struct BitcoinApplication {
    base: QApplication,
    executor:            Option<InitExecutor>,
    options_model:       *mut OptionsModel,
    client_model:        *mut ClientModel,
    window:              *mut BitcoinGUI,
    poll_shutdown_timer: *mut QTimer,

    #[cfg(ENABLE_WALLET)]
    payment_server:      *mut PaymentServer, // default = { nullptr }

    #[cfg(ENABLE_WALLET)]
    wallet_controller:   *mut WalletController, // default = { nullptr }

    return_value:        i32,
    platform_style:      *const PlatformStyle,
    shutdown_window:     Box<QWidget>,
    splash:              *mut SplashScreen, // default = nullptr
    node:                Box<dyn NodeInterface>,
}

impl BitcoinApplication {

    /**
      | Get process return value
      |
      */
    pub fn get_return_value(&self) -> i32 {
        
        todo!();
        /*
            return returnValue;
        */
    }

    pub fn node(&self) -> Rc<RefCell<dyn NodeInterface>> {
        
        todo!();
        /*
            assert(m_node); return *m_node;
        */
    }

    #[Q_SIGNAL]
    pub fn requested_initialize(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn requested_shutdown(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn splash_finished(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn window_shown(&mut self, window: *mut BitcoinGUI)  {
        
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoin.cpp]

#[cfg(QT_STATICPLUGIN)]
#[cfg(QT_QPA_PLATFORM_XCB)]     q_import_plugin!{ QXcbIntegrationPlugin }

#[cfg(QT_STATICPLUGIN)]
#[cfg(QT_QPA_PLATFORM_WINDOWS)] q_import_plugin!{ QWindowsIntegrationPlugin }

#[cfg(QT_STATICPLUGIN)]
#[cfg(QT_QPA_PLATFORM_WINDOWS)] q_import_plugin!{ QWindowsVistaStylePlugin }

#[cfg(QT_STATICPLUGIN)]
#[cfg(QT_QPA_PLATFORM_COCOA)] q_import_plugin!{ QCocoaIntegrationPlugin }

#[cfg(QT_STATICPLUGIN)]
#[cfg(QT_QPA_PLATFORM_COCOA)] q_import_plugin!{ QMacStylePlugin }

/**
  | Declare meta types used for
  | QMetaObject::invokeMethod
  |
  */
q_declare_metatype!{ SynchronizationState }
q_declare_metatype!{ *mut bool }

pub fn register_meta_types()  {
    
    todo!();
        /*
            // Register meta types used for QMetaObject::invokeMethod and QtQueuedConnection
        qRegisterMetaType<bool*>();
        qRegisterMetaType<SynchronizationState>();
      #ifdef ENABLE_WALLET
        qRegisterMetaType<WalletModel*>();
      #endif
        // Register typedefs (see https://doc.qt.io/qt-5/qmetatype.html#qRegisterMetaType)
        // IMPORTANT: if CAmount is no longer a typedef use the normal variant above (see https://doc.qt.io/qt-5/qmetatype.html#qRegisterMetaType-1)
        qRegisterMetaType<CAmount>("CAmount");
        qRegisterMetaType<size_t>("size_t");

        qRegisterMetaType<std::function<c_void()>>("std::function<c_void()>");
        qRegisterMetaType<QMessageBox::Icon>("QMessageBox::Icon");
        qRegisterMetaType<typename interfaces::BlockAndHeaderTipInfo>("typename interfaces::BlockAndHeaderTipInfo");
        */
}

pub fn get_lang_territory() -> String {
    
    todo!();
        /*
            QSettings settings;
        // Get desired locale (e.g. "de_DE")
        // 1) System default language
        QString lang_territory = QLocale::system().name();
        // 2) Language from QSettings
        QString lang_territory_qsettings = settings.value("language", "").toString();
        if(!lang_territory_qsettings.isEmpty())
            lang_territory = lang_territory_qsettings;
        // 3) -lang command line argument
        lang_territory = QString::fromStdString(gArgs.GetArg("-lang", lang_territory.toStdString()));
        return lang_territory;
        */
}

/**
  | Set up translations
  |
  */
pub fn init_translations(
        qt_translator_base: &mut QTranslator,
        qt_translator:      &mut QTranslator,
        translator_base:    &mut QTranslator,
        translator:         &mut QTranslator)  {
    
    todo!();
        /*
            // Remove old translators
        QApplication::removeTranslator(&qtTranslatorBase);
        QApplication::removeTranslator(&qtTranslator);
        QApplication::removeTranslator(&translatorBase);
        QApplication::removeTranslator(&translator);

        // Get desired locale (e.g. "de_DE")
        // 1) System default language
        QString lang_territory = GetLangTerritory();

        // Convert to "de" only by truncating "_DE"
        QString lang = lang_territory;
        lang.truncate(lang_territory.lastIndexOf('_'));

        // Load language files for configured locale:
        // - First load the translator for the base language, without territory
        // - Then load the more specific locale translator

        // Load e.g. qt_de.qm
        if (qtTranslatorBase.load("qt_" + lang, QLibraryInfo::location(QLibraryInfo::TranslationsPath)))
            QApplication::installTranslator(&qtTranslatorBase);

        // Load e.g. qt_de_DE.qm
        if (qtTranslator.load("qt_" + lang_territory, QLibraryInfo::location(QLibraryInfo::TranslationsPath)))
            QApplication::installTranslator(&qtTranslator);

        // Load e.g. bitcoin_de.qm (shortcut "de" needs to be defined in bitcoin.qrc)
        if (translatorBase.load(lang, ":/translations/"))
            QApplication::installTranslator(&translatorBase);

        // Load e.g. bitcoin_de_DE.qm (shortcut "de_DE" needs to be defined in bitcoin.qrc)
        if (translator.load(lang_territory, ":/translations/"))
            QApplication::installTranslator(&translator);
        */
}

pub fn init_settings() -> bool {
    
    todo!();
        /*
            if (!gArgs.GetSettingsPath()) {
            return true; // Do nothing if settings file disabled.
        }

        std::vector<std::string> errors;
        if (!gArgs.ReadSettingsFile(&errors)) {
            std::string error = QT_TRANSLATE_NOOP("bitcoin-core", "Settings file could not be read");
            std::string error_translated = QCoreApplication::translate("bitcoin-core", error.c_str()).toStdString();
            InitError(Untranslated(strprintf("%s:\n%s\n", error, MakeUnorderedList(errors))));

            QMessageBox messagebox(QMessageBox::Critical, PACKAGE_NAME, QString::fromStdString(strprintf("%s.", error_translated)), QMessageBox::Reset | QMessageBox::Abort);
            /*: Explanatory text shown on startup when the settings file cannot be read.
                Prompts user to make a choice between resetting or aborting. */
            messagebox.setInformativeText(QObject::tr("Do you want to reset settings to default values, or to abort without making changes?"));
            messagebox.setDetailedText(QString::fromStdString(MakeUnorderedList(errors)));
            messagebox.setTextFormat(QtPlainText);
            messagebox.setDefaultButton(QMessageBox::Reset);
            switch (messagebox.exec()) {
            case QMessageBox::Reset:
                break;
            case QMessageBox::Abort:
                return false;
            default:
                assert(false);
            }
        }

        errors.clear();
        if (!gArgs.WriteSettingsFile(&errors)) {
            std::string error = QT_TRANSLATE_NOOP("bitcoin-core", "Settings file could not be written");
            std::string error_translated = QCoreApplication::translate("bitcoin-core", error.c_str()).toStdString();
            InitError(Untranslated(strprintf("%s:\n%s\n", error, MakeUnorderedList(errors))));

            QMessageBox messagebox(QMessageBox::Critical, PACKAGE_NAME, QString::fromStdString(strprintf("%s.", error_translated)), QMessageBox::Ok);
            /*: Explanatory text shown on startup when the settings file could not be written.
                Prompts user to check that we have the ability to write to the file.
                Explains that the user has the option of running without a settings file.*/
            messagebox.setInformativeText(QObject::tr("A fatal error occurred. Check that settings file is writable, or try running with -nosettings."));
            messagebox.setDetailedText(QString::fromStdString(MakeUnorderedList(errors)));
            messagebox.setTextFormat(QtPlainText);
            messagebox.setDefaultButton(QMessageBox::Ok);
            messagebox.exec();
            return false;
        }
        return true;
        */
}

/**
  | qDebug() message handler --> debug.log
  |
  */
pub fn debug_message_handler(
        ty:      QtMsgType,
        context: &QMessageLogContext,
        msg:     &String)  {
    
    todo!();
        /*
            Q_UNUSED(context);
        if (type == QtDebugMsg) {
            LogPrint(LogFlags::QT, "GUI: %s\n", msg.toStdString());
        } else {
            LogPrintf("GUI: %s\n", msg.toStdString());
        }
        */
}

lazy_static!{
    /*
    static int qt_argc = 1;
    */
}

pub const QT_ARGV: &'static str = "bitcoin-qt";

impl Drop for BitcoinApplication {
    fn drop(&mut self) {
        todo!();
        /*
            m_executor.reset();

        delete window;
        window = nullptr;
        delete platformStyle;
        platformStyle = nullptr;
        */
    }
}

impl BitcoinApplication {

    pub fn new() -> Self {
    
        todo!();
        /*


            :
        QApplication(qt_argc, const_cast<char **>(&qt_argv)),
        optionsModel(nullptr),
        clientModel(nullptr),
        window(nullptr),
        pollShutdownTimer(nullptr),
        returnValue(0),
        platformStyle(nullptr)
        // Qt runs setlocale(LC_ALL, "") on initialization.
        RegisterMetaTypes();
        setQuitOnLastWindowClosed(false);
        */
    }
    
    /**
      | Setup platform style
      |
      */
    pub fn setup_platform_style(&mut self)  {
        
        todo!();
        /*
            // UI per-platform customization
        // This must be done inside the BitcoinApplication constructor, or after it, because
        // PlatformStyle::instantiate requires a QApplication
        std::string platformName;
        platformName = gArgs.GetArg("-uiplatform", BitcoinGUI::DEFAULT_UIPLATFORM);
        platformStyle = PlatformStyle::instantiate(QString::fromStdString(platformName));
        if (!platformStyle) // Fall back to "other" if specified name not found
            platformStyle = PlatformStyle::instantiate("other");
        assert(platformStyle);
        */
    }

    /**
      | Create payment server
      |
      */
    #[cfg(ENABLE_WALLET)]
    pub fn create_payment_server(&mut self)  {
        
        todo!();
        /*
            paymentServer = new PaymentServer(this);
        */
    }
    
    /**
      | Create options model
      |
      */
    pub fn create_options_model(&mut self, reset_settings: bool)  {
        
        todo!();
        /*
            optionsModel = new OptionsModel(this, resetSettings);
        */
    }
    
    /**
      | Create main window
      |
      */
    pub fn create_window(&mut self, network_style: *const NetworkStyle)  {
        
        todo!();
        /*
            window = new BitcoinGUI(node(), platformStyle, networkStyle, nullptr);
        connect(window, &BitcoinGUI::quitRequested, this, &BitcoinApplication::requestShutdown);

        pollShutdownTimer = new QTimer(window);
        connect(pollShutdownTimer, &QTimer::timeout, window, &BitcoinGUI::detectShutdown);
        */
    }
    
    /**
      | Create splash screen
      |
      */
    pub fn create_splash_screen(&mut self, network_style: *const NetworkStyle)  {
        
        todo!();
        /*
            assert(!m_splash);
        m_splash = new SplashScreen(networkStyle);
        // We don't hold a direct pointer to the splash screen after creation, but the splash
        // screen will take care of deleting itself when finish() happens.
        m_splash->show();
        connect(this, &BitcoinApplication::requestedInitialize, m_splash, &SplashScreen::handleLoadWallet);
        connect(this, &BitcoinApplication::splashFinished, m_splash, &SplashScreen::finish);
        connect(this, &BitcoinApplication::requestedShutdown, m_splash, &QWidget::close);
        */
    }
    
    /**
      | Create or spawn node
      |
      */
    pub fn create_node(&mut self, init: Box<dyn Init>)  {
        
        todo!();
        /*
            assert(!m_node);
        m_node = init.makeNode();
        if (optionsModel) optionsModel->setNode(*m_node);
        if (m_splash) m_splash->setNode(*m_node);
        */
    }
    
    /**
      | Basic initialization, before starting
      | initialization/shutdown thread.
      | Return true on success.
      |
      */
    pub fn base_initialize(&mut self) -> bool {
        
        todo!();
        /*
            return node().baseInitialize();
        */
    }
    
    pub fn start_thread(&mut self)  {
        
        todo!();
        /*
            assert(!m_executor);
        m_executor.emplace(node());

        /*  communication to and from thread */
        connect(&m_executor.value(), &InitExecutor::initializeResult, this, &BitcoinApplication::initializeResult);
        connect(&m_executor.value(), &InitExecutor::shutdownResult, this, &QCoreApplication::quit);
        connect(&m_executor.value(), &InitExecutor::runawayException, this, &BitcoinApplication::handleRunawayException);
        connect(this, &BitcoinApplication::requestedInitialize, &m_executor.value(), &InitExecutor::initialize);
        connect(this, &BitcoinApplication::requestedShutdown, &m_executor.value(), &InitExecutor::shutdown);
        */
    }
    
    /**
      | parameter interaction/setup based
      | on rules
      |
      */
    pub fn parameter_setup(&mut self)  {
        
        todo!();
        /*
            // Default printtoconsole to false for the GUI. GUI programs should not
        // print to the console unnecessarily.
        gArgs.SoftSetBoolArg("-printtoconsole", false);

        InitLogging(gArgs);
        InitParameterInteraction(gArgs);
        */
    }
    
    /**
      | Initialize prune setting
      |
      */
    pub fn init_prune_setting(&mut self, prune_mib: i64)  {
        
        todo!();
        /*
            optionsModel->SetPruneTargetGB(PruneMiBtoGB(prune_MiB), true);
        */
    }
    
    /**
      | Request core initialization
      |
      */
    pub fn request_initialize(&mut self)  {
        
        todo!();
        /*
            qDebug() << __func__ << ": Requesting initialize";
        startThread();
        Q_EMIT requestedInitialize();
        */
    }
    
    /**
      | Request core shutdown
      |
      */
    #[Q_SLOT]
    pub fn request_shutdown(&mut self)  {
        
        todo!();
        /*
            for (const auto w : QGuiApplication::topLevelWindows()) {
            w->hide();
        }

        // Show a simple window indicating shutdown status
        // Do this first as some of the steps may take some time below,
        // for example the RPC console may still be executing a command.
        shutdownWindow.reset(ShutdownWindow::showShutdownWindow(window));

        qDebug() << __func__ << ": Requesting shutdown";

        // Must disconnect node signals otherwise current thread can deadlock since
        // no event loop is running.
        window->unsubscribeFromCoreSignals();
        // Request node shutdown, which can interrupt long operations, like
        // rescanning a wallet.
        node().startShutdown();
        // Unsetting the client model can cause the current thread to wait for node
        // to complete an operation, like wait for a RPC execution to complete.
        window->setClientModel(nullptr);
        pollShutdownTimer->stop();

    #ifdef ENABLE_WALLET
        // Delete wallet controller here manually, instead of relying on Qt object
        // tracking (https://doc.qt.io/qt-5/objecttrees.html). This makes sure
        // walletmodel m_handle_* notification handlers are deleted before wallets
        // are unloaded, which can simplify wallet implementations. It also avoids
        // these notifications having to be handled while GUI objects are being
        // destroyed, making GUI code less fragile as well.
        delete m_wallet_controller;
        m_wallet_controller = nullptr;
    #endif // ENABLE_WALLET

        delete clientModel;
        clientModel = nullptr;

        // Request shutdown from core thread
        Q_EMIT requestedShutdown();
        */
    }
    
    #[Q_SLOT]
    pub fn initialize_result(&mut self, 
        success:  bool,
        tip_info: BlockAndHeaderTipInfo)  {
        
        todo!();
        /*
            qDebug() << __func__ << ": Initialization result: " << success;
        // Set exit result.
        returnValue = success ? EXIT_SUCCESS : EXIT_FAILURE;
        if(success)
        {
            // Log this only after AppInitMain finishes, as then logging setup is guaranteed complete
            qInfo() << "Platform customization:" << platformStyle->getName();
            clientModel = new ClientModel(node(), optionsModel);
            window->setClientModel(clientModel, &tip_info);
    #ifdef ENABLE_WALLET
            if (WalletModel::isWalletEnabled()) {
                m_wallet_controller = new WalletController(*clientModel, platformStyle, this);
                window->setWalletController(m_wallet_controller);
                if (paymentServer) {
                    paymentServer->setOptionsModel(optionsModel);
                }
            }
    #endif // ENABLE_WALLET

            // If -min option passed, start window minimized (iconified) or minimized to tray
            if (!gArgs.GetBoolArg("-min", false)) {
                window->show();
            } else if (clientModel->getOptionsModel()->getMinimizeToTray() && window->hasTrayIcon()) {
                // do nothing as the window is managed by the tray icon
            } else {
                window->showMinimized();
            }
            Q_EMIT splashFinished();
            Q_EMIT windowShown(window);

    #ifdef ENABLE_WALLET
            // Now that initialization/startup is done, process any command-line
            // bitcoin: URIs or payment requests:
            if (paymentServer) {
                connect(paymentServer, &PaymentServer::receivedPaymentRequest, window, &BitcoinGUI::handlePaymentRequest);
                connect(window, &BitcoinGUI::receivedURI, paymentServer, &PaymentServer::handleURIOrFile);
                connect(paymentServer, &PaymentServer::message, [this](const QString& title, const QString& message, unsigned int style) {
                    window->message(title, message, style);
                });
                QTimer::singleShot(100, paymentServer, &PaymentServer::uiReady);
            }
    #endif
            pollShutdownTimer->start(200);
        } else {
            Q_EMIT splashFinished(); // Make sure splash screen doesn't stick around during shutdown
            requestShutdown();
        }
        */
    }
    
    /**
      | Handle runaway exceptions. Shows a
      | message box with the problem and quits
      | the program.
      |
      */
    #[Q_SLOT]
    pub fn handle_runaway_exception(&mut self, message: &String)  {
        
        todo!();
        /*
            QMessageBox::critical(
            nullptr, tr("Runaway exception"),
            tr("A fatal error occurred. %1 can no longer continue safely and will quit.").arg(PACKAGE_NAME) +
            QLatin1String("<br><br>") + typename gui_util::MakeHtmlLink(message, PACKAGE_BUGREPORT));
        ::exit(EXIT_FAILURE);
        */
    }
    
    /**
      | A helper function that shows a message
      | box with details about a non-fatal exception.
      |
      */
    #[Q_SLOT]
    pub fn handle_non_fatal_exception(&mut self, message: &String)  {
        
        todo!();
        /*
            assert(QThread::currentThread() == thread());
        QMessageBox::warning(
            nullptr, tr("Internal error"),
            tr("An internal error occurred. %1 will attempt to continue safely. This is "
               "an unexpected bug which can be reported as described below.").arg(PACKAGE_NAME) +
            QLatin1String("<br><br>") + typename gui_util::MakeHtmlLink(message, PACKAGE_BUGREPORT));
        */
    }
    
    /**
      | Get window identifier of QMainWindow
      | (BitcoinGUI)
      |
      */
    pub fn get_main_win_id(&self) -> WId {
        
        todo!();
        /*
            if (!window)
            return 0;

        return window->winId();
        */
    }
}

pub fn setup_ui_args(argsman: &mut ArgsManager)  {
    
    todo!();
        /*
            argsman.AddArg("-choosedatadir", strprintf("Choose data directory on startup (default: %u)", DEFAULT_CHOOSE_DATADIR), ArgsManager::ALLOW_ANY, OptionsCategory::GUI);
        argsman.AddArg("-lang=<lang>", "Set language, for example \"de_DE\" (default: system locale)", ArgsManager::ALLOW_ANY, OptionsCategory::GUI);
        argsman.AddArg("-min", "Start minimized", ArgsManager::ALLOW_ANY, OptionsCategory::GUI);
        argsman.AddArg("-resetguisettings", "Reset all settings changed in the GUI", ArgsManager::ALLOW_ANY, OptionsCategory::GUI);
        argsman.AddArg("-splash", strprintf("Show splash screen on startup (default: %u)", DEFAULT_SPLASHSCREEN), ArgsManager::ALLOW_ANY, OptionsCategory::GUI);
        argsman.AddArg("-uiplatform", strprintf("Select platform to customize UI for (one of windows, macosx, other; default: %s)", BitcoinGUI::DEFAULT_UIPLATFORM), ArgsManager::ALLOW_ANY | ArgsManager::DEBUG_ONLY, OptionsCategory::GUI);
        */
}

pub fn gui_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
            #ifdef WIN32
        util::WinCmdLineArgs winArgs;
        std::tie(argc, argv) = winArgs.get();
    #endif

        std::unique_ptr<typename interfaces::Init> init = typename interfaces::MakeGuiInit(argc, argv);

        SetupEnvironment();
        util::ThreadSetInternalName("main");

        // Subscribe to global signals from core
        boost::signals2::scoped_connection handler_message_box = ::uiInterface.ThreadSafeMessageBox_connect(noui_ThreadSafeMessageBox);
        boost::signals2::scoped_connection handler_question = ::uiInterface.ThreadSafeQuestion_connect(noui_ThreadSafeQuestion);
        boost::signals2::scoped_connection handler_init_message = ::uiInterface.InitMessage_connect(noui_InitMessage);

        // Do not refer to data directory yet, this can be overridden by Intro::pickDataDirectory

        /// 1. Basic Qt initialization (not dependent on parameters or configuration)
        Q_INIT_RESOURCE(bitcoin);
        Q_INIT_RESOURCE(bitcoin_locale);

        // Generate high-dpi pixmaps
        QApplication::setAttribute(QtAA_UseHighDpiPixmaps);
        QCoreApplication::setAttribute(QtAA_EnableHighDpiScaling);

    #if defined(QT_QPA_PLATFORM_ANDROID)
        QApplication::setAttribute(QtAA_DontUseNativeMenuBar);
        QApplication::setAttribute(QtAA_DontCreateNativeWidgetSiblings);
        QApplication::setAttribute(QtAA_DontUseNativeDialogs);
    #endif

        BitcoinApplication app;
        typename gui_util::LoadFont(QStringLiteral(":/fonts/monospace"));

        /// 2. Parse command-line options. We do this after qt in order to show an error if there are problems parsing these
        // Command-line options take precedence:
        SetupServerArgs(gArgs);
        SetupUIArgs(gArgs);
        std::string error;
        if (!gArgs.ParseParameters(argc, argv, error)) {
            InitError(strprintf(Untranslated("Error parsing command line arguments: %s\n"), error));
            // Create a message box, because the gui has neither been created nor has subscribed to core signals
            QMessageBox::critical(nullptr, PACKAGE_NAME,
                // message can not be translated because translations have not been initialized
                QString::fromStdString("Error parsing command line arguments: %1.").arg(QString::fromStdString(error)));
            return EXIT_FAILURE;
        }

        // Now that the QApplication is setup and we have parsed our parameters, we can set the platform style
        app.setupPlatformStyle();

        /// 3. Application identification
        // must be set before OptionsModel is initialized or translations are loaded,
        // as it is used to locate QSettings
        QApplication::setOrganizationName(QAPP_ORG_NAME);
        QApplication::setOrganizationDomain(QAPP_ORG_DOMAIN);
        QApplication::setApplicationName(QAPP_APP_NAME_DEFAULT);

        /// 4. Initialization of translations, so that intro dialog is in user's language
        // Now that QSettings are accessible, initialize translations
        QTranslator qtTranslatorBase, qtTranslator, translatorBase, translator;
        initTranslations(qtTranslatorBase, qtTranslator, translatorBase, translator);

        // Show help message immediately after parsing command-line options (for "-lang") and setting locale,
        // but before showing splash screen.
        if (HelpRequested(gArgs) || gArgs.IsArgSet("-version")) {
            HelpMessageDialog help(nullptr, gArgs.IsArgSet("-version"));
            help.showOrPrint();
            return EXIT_SUCCESS;
        }

        // Install global event filter that makes sure that long tooltips can be word-wrapped
        app.installEventFilter(new typename gui_util::ToolTipToRichTextFilter(TOOLTIP_WRAP_THRESHOLD, &app));

        /// 5. Now that settings and translations are available, ask user for data directory
        // User language is set up: pick a data directory
        bool did_show_intro = false;
        int64_t prune_MiB = 0;  // Intro dialog prune configuration
        // Gracefully exit if the user cancels
        if (!Intro::showIfNeeded(did_show_intro, prune_MiB)) return EXIT_SUCCESS;

        /// 6. Determine availability of data directory and parse bitcoin.conf
        /// - Do not call gArgs.GetDataDirNet() before this step finishes
        if (!CheckDataDirOption()) {
            InitError(strprintf(Untranslated("Specified data directory \"%s\" does not exist.\n"), gArgs.GetArg("-datadir", "")));
            QMessageBox::critical(nullptr, PACKAGE_NAME,
                QObject::tr("Error: Specified data directory \"%1\" does not exist.").arg(QString::fromStdString(gArgs.GetArg("-datadir", ""))));
            return EXIT_FAILURE;
        }
        if (!gArgs.ReadConfigFiles(error, true)) {
            InitError(strprintf(Untranslated("Error reading configuration file: %s\n"), error));
            QMessageBox::critical(nullptr, PACKAGE_NAME,
                QObject::tr("Error: Cannot parse configuration file: %1.").arg(QString::fromStdString(error)));
            return EXIT_FAILURE;
        }

        /// 7. Determine network (and switch to network specific options)
        // - Do not call Params() before this step
        // - Do this after parsing the configuration file, as the network can be switched there
        // - QSettings() will use the new application name after this, resulting in network-specific settings
        // - Needs to be done before createOptionsModel

        // Check for chain settings (Params() calls are only valid after this clause)
        try {
            SelectParams(gArgs.GetChainName());
        } catch(std::exception &e) {
            InitError(Untranslated(strprintf("%s\n", e.what())));
            QMessageBox::critical(nullptr, PACKAGE_NAME, QObject::tr("Error: %1").arg(e.what()));
            return EXIT_FAILURE;
        }
    #ifdef ENABLE_WALLET
        // Parse URIs on command line -- this can affect Params()
        PaymentServer::ipcParseCommandLine(argc, argv);
    #endif

        if (!InitSettings()) {
            return EXIT_FAILURE;
        }

        QScopedPointer<const NetworkStyle> networkStyle(NetworkStyle::instantiate(Params().NetworkIDString()));
        assert(!networkStyle.isNull());
        // Allow for separate UI settings for testnets
        QApplication::setApplicationName(networkStyle->getAppName());
        // Re-initialize translations after changing application name (language in network-specific settings can be different)
        initTranslations(qtTranslatorBase, qtTranslator, translatorBase, translator);

    #ifdef ENABLE_WALLET
        /// 8. URI IPC sending
        // - Do this early as we don't want to bother initializing if we are just calling IPC
        // - Do this *after* setting up the data directory, as the data directory hash is used in the name
        // of the server.
        // - Do this after creating app and setting up translations, so errors are
        // translated properly.
        if (PaymentServer::ipcSendCommandLine())
            exit(EXIT_SUCCESS);

        // Start up the payment server early, too, so impatient users that click on
        // bitcoin: links repeatedly have their payment requests routed to this process:
        if (WalletModel::isWalletEnabled()) {
            app.createPaymentServer();
        }
    #endif // ENABLE_WALLET

        /// 9. Main GUI initialization
        // Install global event filter that makes sure that out-of-focus labels do not contain text cursor.
        app.installEventFilter(new typename gui_util::LabelOutOfFocusEventFilter(&app));
    #if defined(Q_OS_WIN)
        // Install global event filter for processing Windows session related Windows messages (WM_QUERYENDSESSION and WM_ENDSESSION)
        qApp->installNativeEventFilter(new WinShutdownMonitor());
    #endif
        // Install qDebug() message handler to route to debug.log
        qInstallMessageHandler(DebugMessageHandler);
        // Allow parameter interaction before we create the options model
        app.parameterSetup();
        typename gui_util::LogQtInfo();
        // Load GUI settings from QSettings
        app.createOptionsModel(gArgs.GetBoolArg("-resetguisettings", false));

        if (did_show_intro) {
            // Store intro dialog settings other than datadir (network specific)
            app.InitPruneSetting(prune_MiB);
        }

        if (gArgs.GetBoolArg("-splash", DEFAULT_SPLASHSCREEN) && !gArgs.GetBoolArg("-min", false))
            app.createSplashScreen(networkStyle.data());

        app.createNode(*init);

        int rv = EXIT_SUCCESS;
        try
        {
            app.createWindow(networkStyle.data());
            // Perform base initialization before spinning up initialization/shutdown thread
            // This is acceptable because this function only contains steps that are quick to execute,
            // so the GUI thread won't be held up.
            if (app.baseInitialize()) {
                app.requestInitialize();
    #if defined(Q_OS_WIN)
                WinShutdownMonitor::registerShutdownBlockReason(QObject::tr("%1 didn't yet exit safely…").arg(PACKAGE_NAME), (HWND)app.getMainWinId());
    #endif
                app.exec();
                rv = app.getReturnValue();
            } else {
                // A dialog with detailed error will have been shown by InitError()
                rv = EXIT_FAILURE;
            }
        } catch (const std::exception& e) {
            PrintExceptionContinue(&e, "Runaway exception");
            app.handleRunawayException(QString::fromStdString(app.node().getWarnings().translated));
        } catch (...) {
            PrintExceptionContinue(nullptr, "Runaway exception");
            app.handleRunawayException(QString::fromStdString(app.node().getWarnings().translated));
        }
        return rv;
        */
}
