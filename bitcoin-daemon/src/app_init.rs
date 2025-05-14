// ---------------- [ File: bitcoin-daemon/src/app_init.rs ]
crate::ix!();

pub fn app_init(
        node: &mut NodeContext,
        argc: i32,
        argv: &[*mut u8]) -> bool {
    
    todo!();
        /*
            bool fRet = false;

        util::ThreadSetInternalName("init");

        // If Qt is used, parameters/bitcoin.conf are parsed in qt/bitcoin.cpp's main()
        ArgsManager& args = *Assert(node.args);
        SetupServerArgs(args);
        std::string error;
        if (!args.ParseParameters(argc, argv, error)) {
            return InitError(Untranslated(strprintf("Error parsing command line arguments: %s\n", error)));
        }

        // Process help and version before taking care about datadir
        if (HelpRequested(args) || args.IsArgSet("-version")) {
            std::string strUsage = PACKAGE_NAME " version " + FormatFullVersion() + "\n";

            if (!args.IsArgSet("-version")) {
                strUsage += FormatParagraph(LicenseInfo()) + "\n"
                    "\nUsage:  bitcoind [options]                     Start " PACKAGE_NAME "\n"
                    "\n";
                strUsage += args.GetHelpMessage();
            }

            tfm::format(std::cout, "%s", strUsage);
            return true;
        }

    #if HAVE_DECL_FORK
        // Communication with parent after daemonizing. This is used for signalling in the following ways:
        // - a boolean token is sent when the initialization process (all the Init* functions) have finished to indicate
        // that the parent process can quit, and whether it was successful/unsuccessful.
        // - an unexpected shutdown of the child process creates an unexpected end of stream at the parent
        // end, which is interpreted as failure to start.
        TokenPipeEnd daemon_ep;
    #endif
        std::any context{&node};
        try
        {
            if (!CheckDataDirOption()) {
                return InitError(Untranslated(strprintf("Specified data directory \"%s\" does not exist.\n", args.GetArg("-datadir", ""))));
            }
            if (!args.ReadConfigFiles(error, true)) {
                return InitError(Untranslated(strprintf("Error reading configuration file: %s\n", error)));
            }
            // Check for chain settings (Params() calls are only valid after this clause)
            try {
                SelectParams(args.GetChainName());
            } catch (const std::exception& e) {
                return InitError(Untranslated(strprintf("%s\n", e.what())));
            }

            // Error out when loose non-argument tokens are encountered on command line
            for (int i = 1; i < argc; i++) {
                if (!IsSwitchChar(argv[i][0])) {
                    return InitError(Untranslated(strprintf("Command line contains unexpected token '%s', see bitcoind -h for a list of options.\n", argv[i])));
                }
            }

            if (!args.InitSettings(error)) {
                InitError(Untranslated(error));
                return false;
            }

            // -server defaults to true for bitcoind but not for the GUI so do this here
            args.SoftSetBoolArg("-server", true);
            // Set this early so that parameter interactions go to console
            InitLogging(args);
            InitParameterInteraction(args);
            if (!AppInitBasicSetup(args)) {
                // InitError will have been called with detailed error, which ends up on console
                return false;
            }
            if (!AppInitParameterInteraction(args)) {
                // InitError will have been called with detailed error, which ends up on console
                return false;
            }
            if (!AppInitSanityChecks())
            {
                // InitError will have been called with detailed error, which ends up on console
                return false;
            }
            if (args.GetBoolArg("-daemon", DEFAULT_DAEMON) || args.GetBoolArg("-daemonwait", DEFAULT_DAEMONWAIT)) {
    #if HAVE_DECL_FORK
                tfm::format(std::cout, PACKAGE_NAME " starting\n");

                // Daemonize
                switch (fork_daemon(1, 0, daemon_ep)) { // don't chdir (1), do close FDs (0)
                case 0: // Child: continue.
                    // If -daemonwait is not enabled, immediately send a success token the parent.
                    if (!args.GetBoolArg("-daemonwait", DEFAULT_DAEMONWAIT)) {
                        daemon_ep.TokenWrite(1);
                        daemon_ep.Close();
                    }
                    break;
                case -1: // Error happened.
                    return InitError(Untranslated(strprintf("fork_daemon() failed: %s\n", strerror(errno))));
                default: { // Parent: wait and exit.
                    int token = daemon_ep.TokenRead();
                    if (token) { // Success
                        exit(EXIT_SUCCESS);
                    } else { // fRet = false or token read error (premature exit).
                        tfm::format(std::cerr, "Error during initializaton - check debug.log for details\n");
                        exit(EXIT_FAILURE);
                    }
                }
                }
    #else
                return InitError(Untranslated("-daemon is not supported on this operating system\n"));
    #endif // HAVE_DECL_FORK
            }
            // Lock data directory after daemonization
            if (!AppInitLockDataDirectory())
            {
                // If locking the data directory failed, exit immediately
                return false;
            }
            fRet = AppInitInterfaces(node) && AppInitMain(node);
        }
        catch (const std::exception& e) {
            PrintExceptionContinue(&e, "AppInit()");
        } catch (...) {
            PrintExceptionContinue(nullptr, "AppInit()");
        }

    #if HAVE_DECL_FORK
        if (daemon_ep.IsOpen()) {
            // Signal initialization status to parent, then close pipe.
            daemon_ep.TokenWrite(fRet);
            daemon_ep.Close();
        }
    #endif
        SetSyscallSandboxPolicy(SyscallSandboxPolicy::SHUTOFF);
        if (fRet) {
            WaitForShutdown();
        }
        Interrupt(node);
        Shutdown(node);

        return fRet;
        */
}
