// ---------------- [ File: bitcoin-qt/src/test_apptests.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/test/apptests.h]

#[Q_OBJECT]
pub struct AppTests {

    base:      QObject,

    /**
      | Bitcoin application.
      |
      */
    app:       Rc<RefCell<BitcoinApplication>>,

    /**
      | Set of pending callback names. 
      |
      | Used to track expected callbacks and shut
      | down the app after the last callback has
      | been handled and all tests have either run
      | or thrown exceptions. 
      |
      | This could be a simple int counter instead
      | of a set of names, but the names might be
      | useful for debugging.
      */
    callbacks: MultiSet<String>,
}

pub mod app_tests {

    use super::*;

    /**
      | RAII helper to remove no-longer-pending
      | callback.
      |
      */
    pub struct HandleCallback
    {
        callback:  String,
        app_tests: Rc<RefCell<AppTests>>,
    }

    impl Drop for HandleCallback {

        /**
          | Destructor to shut down after the last
          | expected callback completes.
          |
          */
        fn drop(&mut self) {
            todo!();
            /*
                auto& callbacks = m_app_tests.m_callbacks;
            auto it = callbacks.find(m_callback);
            assert(it != callbacks.end());
            callbacks.erase(it);
            if (callbacks.empty()) {
                m_app_tests.m_app.quit();
            }
            */
        }
    }
}

impl AppTests {

    pub fn new(app: &mut BitcoinApplication) -> Self {
    
        todo!();
        /*
        : app(app),
        */
    }

    /**
      | Add expected callback name to list of
      | pending callbacks.
      |
      */
    #[Q_SLOT]
    pub fn expect_callback(&mut self, callback: String)  {
        
        todo!();
        /*
            m_callbacks.emplace(std::move(callback));
        */
    }

    /**
      | Entry point for BitcoinApplication
      | tests.
      |
      */
    #[Q_SLOT]
    pub fn app_tests(&mut self)  {
        
        todo!();
        /*
            #ifdef Q_OS_MAC
        if (QApplication::platformName() == "minimal") {
            // Disable for mac on "minimal" platform to avoid crashes inside the Qt
            // framework when it tries to look up unimplemented cocoa functions,
            // and fails to handle returned nulls
            // (https://bugreports.qt.io/browse/QTBUG-49686).
            QWARN("Skipping AppTests on mac build with 'minimal' platform set due to Qt bugs. To run AppTests, invoke "
                  "with 'QT_QPA_PLATFORM=cocoa test_bitcoin-qt' on mac, or else use a linux or windows build.");
            return;
        }
    #endif

        fs::create_directories([] {
            BasicTestingSetup test{CBaseChainParams::REGTEST}; // Create a temp data directory to backup the gui settings to
            return gArgs.GetDataDirNet() / "blocks";
        }());

        qRegisterMetaType<typename interfaces::BlockAndHeaderTipInfo>("typename interfaces::BlockAndHeaderTipInfo");
        m_app.parameterSetup();
        m_app.createOptionsModel(true /* reset settings */);
        QScopedPointer<const NetworkStyle> style(NetworkStyle::instantiate(Params().NetworkIDString()));
        m_app.setupPlatformStyle();
        m_app.createWindow(style.data());
        connect(&m_app, &BitcoinApplication::windowShown, this, &AppTests::guiTests);
        expectCallback("guiTests");
        m_app.baseInitialize();
        m_app.requestInitialize();
        m_app.exec();
        m_app.requestShutdown();
        m_app.exec();

        // Reset global state to avoid interfering with later tests.
        LogInstance().DisconnectTestLogger();
        AbortShutdown();
        */
    }

    /**
      | Entry point for BitcoinGUI tests.
      |
      */
    #[Q_SLOT]
    pub fn gui_tests(&mut self, window: *mut BitcoinGUI)  {
        
        todo!();
        /*
            HandleCallback callback{"guiTests", *this};
        connect(window, &BitcoinGUI::consoleShown, this, &AppTests::consoleTests);
        expectCallback("consoleTests");
        QAction* action = window->findChild<QAction*>("openRPCConsoleAction");
        action->activate(QAction::Trigger);
        */
    }

    /**
      | Entry point for RPCConsole tests.
      |
      */
    #[Q_SLOT]
    pub fn console_tests(&mut self, console: *mut RPCConsole)  {
        
        todo!();
        /*
            HandleCallback callback{"consoleTests", *this};
        TestRpcCommand(console);
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/test/apptests.cpp]

/**
  | Regex find a string group inside of the
  | console output
  |
  */
pub fn find_in_console(
        output:  &String,
        pattern: &String) -> String {
    
    todo!();
        /*
            const QRegularExpression re(pattern);
        return re.match(output).captured(1);
        */
}

/**
  | Call getblockchaininfo RPC and check
  | first field of JSON output.
  |
  */
pub fn test_rpc_command(console: *mut RPCConsole)  {
    
    todo!();
        /*
            QTextEdit* messagesWidget = console->findChild<QTextEdit*>("messagesWidget");
        QLineEdit* lineEdit = console->findChild<QLineEdit*>("lineEdit");
        QSignalSpy mw_spy(messagesWidget, &QTextEdit::textChanged);
        QVERIFY(mw_spy.isValid());
        QTest::keyClicks(lineEdit, "getblockchaininfo");
        QTest::keyClick(lineEdit, QtKey_Return);
        QVERIFY(mw_spy.wait(1000));
        QCOMPARE(mw_spy.count(), 4);
        const QString output = messagesWidget->toPlainText();
        const QString pattern = QStringLiteral("\"chain\": \"(\\w+)\"");
        QCOMPARE(FindInConsole(output, pattern), QString("regtest"));
        */
}
