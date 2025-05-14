// ---------------- [ File: bitcoin-qt/src/test_test_main.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/test/test_main.cpp]

#[cfg(QT_STATICPLUGIN)]
#[cfg(QT_QPA_PLATFORM_MINIMAL)] q_import_plugin!{ QMinimalIntegrationPlugin }

#[cfg(QT_STATICPLUGIN)]
#[cfg(QT_QPA_PLATFORM_XCB)]     q_import_plugin!{ QXcbIntegrationPlugin }

#[cfg(QT_STATICPLUGIN)]
#[cfg(QT_QPA_PLATFORM_WINDOWS)] q_import_plugin!{ QWindowsIntegrationPlugin }

#[cfg(QT_STATICPLUGIN)]
#[cfg(QT_QPA_PLATFORM_COCOA)]   q_import_plugin!{ QCocoaIntegrationPlugin }

lazy_static!{
    /*
    const std::function<c_void(const std::string&)> G_TEST_LOG_FUN{};
    */
}

/**
  | This is all you need to run all the tests
  |
  */
pub fn qt_test_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    todo!();
        /*
            // Initialize persistent globals with the testing setup state for sanity.
        // E.g. -datadir in gArgs is set to a temp directory dummy value (instead
        // of defaulting to the default datadir), or globalChainParams is set to
        // regtest params.
        //
        // All tests must use their own testing setup (if needed).
        {
            BasicTestingSetup dummy{CBaseChainParams::REGTEST};
        }

        std::unique_ptr<interfaces::Init> init = interfaces::MakeGuiInit(argc, argv);
        gArgs.ForceSetArg("-listen", "0");
        gArgs.ForceSetArg("-listenonion", "0");
        gArgs.ForceSetArg("-discover", "0");
        gArgs.ForceSetArg("-dnsseed", "0");
        gArgs.ForceSetArg("-fixedseeds", "0");
        gArgs.ForceSetArg("-upnp", "0");
        gArgs.ForceSetArg("-natpmp", "0");

        bool fInvalid = false;

        // Prefer the "minimal" platform for the test instead of the normal default
        // platform ("xcb", "windows", or "cocoa") so tests can't unintentionally
        // interfere with any background GUIs and don't require extra resources.
        #if defined(WIN32)
            if (getenv("QT_QPA_PLATFORM") == nullptr) _putenv_s("QT_QPA_PLATFORM", "minimal");
        #else
            setenv("QT_QPA_PLATFORM", "minimal", /* overwrite */ 0);
        #endif

        // Don't remove this, it's needed to access
        // QApplication:: and QCoreApplication:: in the tests
        BitcoinApplication app;
        app.setApplicationName("Bitcoin-Qt-test");
        app.createNode(*init);

        AppTests app_tests(app);
        if (QTest::qExec(&app_tests) != 0) {
            fInvalid = true;
        }
        URITests test1;
        if (QTest::qExec(&test1) != 0) {
            fInvalid = true;
        }
        RPCNestedTests test3(app.node());
        if (QTest::qExec(&test3) != 0) {
            fInvalid = true;
        }
    #ifdef ENABLE_WALLET
        WalletTests test5(app.node());
        if (QTest::qExec(&test5) != 0) {
            fInvalid = true;
        }
        AddressBookTests test6(app.node());
        if (QTest::qExec(&test6) != 0) {
            fInvalid = true;
        }
    #endif

        return fInvalid;
        */
}
