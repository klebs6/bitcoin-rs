// ---------------- [ File: bitcoin-init/src/common.rs ]
/*!
  | @file
  |
  | @brief Common init functions shared by
  | bitcoin-node, bitcoin-wallet, etc.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/init/common.h]
//-------------------------------------------[.cpp/bitcoin/src/init/common.cpp]

lazy_static!{
    /*
    static std::unique_ptr<ECCVerifyHandle> globalVerifyHandle;
    */
}

pub fn set_globals()  {
    
    todo!();
        /*
            std::string sha256_algo = SHA256AutoDetect();
        LogPrintf("Using the '%s' SHA256 implementation\n", sha256_algo);
        RandomInit();
        ECC_Start();
        globalVerifyHandle.reset(new ECCVerifyHandle());
        */
}

pub fn unset_globals()  {
    
    todo!();
        /*
            globalVerifyHandle.reset();
        ECC_Stop();
        */
}

/**
  | Ensure a usable environment with all
  | necessary library support.
  |
  */
pub fn sanity_checks() -> bool {
    
    todo!();
        /*
            if (!ECC_InitSanityCheck()) {
            return InitError(Untranslated("Elliptic curve cryptography sanity check failure. Aborting."));
        }

        if (!glibcxx_sanity_test())
            return false;

        if (!Random_SanityCheck()) {
            return InitError(Untranslated("OS cryptographic RNG sanity check failure. Aborting."));
        }

        if (!ChronoSanityCheck()) {
            return InitError(Untranslated("Clock epoch mismatch. Aborting."));
        }

        return true;
        */
}

pub fn add_logging_args(argsman: &mut ArgsManager)  {
    
    todo!();
        /*
            argsman.AddArg("-debuglogfile=<file>", strprintf("Specify location of debug log file. Relative paths will be prefixed by a net-specific datadir location. (-nodebuglogfile to disable; default: %s)", DEFAULT_DEBUGLOGFILE), ArgsManager::ALLOW_ANY, OptionsCategory::OPTIONS);
        argsman.AddArg("-debug=<category>", "Output debugging information (default: -nodebug, supplying <category> is optional). "
            "If <category> is not supplied or if <category> = 1, output all debugging information. <category> can be: " + LogInstance().LogCategoriesString() + ". This option can be specified multiple times to output multiple categories.",
            ArgsManager::ALLOW_ANY, OptionsCategory::DEBUG_TEST);
        argsman.AddArg("-debugexclude=<category>", strprintf("Exclude debugging information for a category. Can be used in conjunction with -debug=1 to output debug logs for all categories except the specified category. This option can be specified multiple times to exclude multiple categories."), ArgsManager::ALLOW_ANY, OptionsCategory::DEBUG_TEST);
        argsman.AddArg("-logips", strprintf("Include IP addresses in debug output (default: %u)", DEFAULT_LOGIPS), ArgsManager::ALLOW_ANY, OptionsCategory::DEBUG_TEST);
        argsman.AddArg("-logtimestamps", strprintf("Prepend debug output with timestamp (default: %u)", DEFAULT_LOGTIMESTAMPS), ArgsManager::ALLOW_ANY, OptionsCategory::DEBUG_TEST);
    #ifdef HAVE_THREAD_LOCAL
        argsman.AddArg("-logthreadnames", strprintf("Prepend debug output with name of the originating thread (only available on platforms supporting thread_local) (default: %u)", DEFAULT_LOGTHREADNAMES), ArgsManager::ALLOW_ANY, OptionsCategory::DEBUG_TEST);
    #else
        argsman.AddHiddenArgs({"-logthreadnames"});
    #endif
        argsman.AddArg("-logsourcelocations", strprintf("Prepend debug output with name of the originating source location (source file, line number and function name) (default: %u)", DEFAULT_LOGSOURCELOCATIONS), ArgsManager::ALLOW_ANY, OptionsCategory::DEBUG_TEST);
        argsman.AddArg("-logtimemicros", strprintf("Add microsecond precision to debug timestamps (default: %u)", DEFAULT_LOGTIMEMICROS), ArgsManager::ALLOW_ANY | ArgsManager::DEBUG_ONLY, OptionsCategory::DEBUG_TEST);
        argsman.AddArg("-printtoconsole", "Send trace/debug info to console (default: 1 when no -daemon. To disable logging to file, set -nodebuglogfile)", ArgsManager::ALLOW_ANY, OptionsCategory::DEBUG_TEST);
        argsman.AddArg("-shrinkdebugfile", "Shrink debug.log file on client startup (default: 1 when no -debug)", ArgsManager::ALLOW_ANY, OptionsCategory::DEBUG_TEST);
        */
}

pub fn set_logging_options(args: &ArgsManager)  {
    
    todo!();
        /*
            LogInstance().m_print_to_file = !args.IsArgNegated("-debuglogfile");
        LogInstance().m_file_path = AbsPathForConfigVal(fs::PathFromString(args.GetArg("-debuglogfile", DEFAULT_DEBUGLOGFILE)));
        LogInstance().m_print_to_console = args.GetBoolArg("-printtoconsole", !args.GetBoolArg("-daemon", false));
        LogInstance().m_log_timestamps = args.GetBoolArg("-logtimestamps", DEFAULT_LOGTIMESTAMPS);
        LogInstance().m_log_time_micros = args.GetBoolArg("-logtimemicros", DEFAULT_LOGTIMEMICROS);
    #ifdef HAVE_THREAD_LOCAL
        LogInstance().m_log_threadnames = args.GetBoolArg("-logthreadnames", DEFAULT_LOGTHREADNAMES);
    #endif
        LogInstance().m_log_sourcelocations = args.GetBoolArg("-logsourcelocations", DEFAULT_LOGSOURCELOCATIONS);

        fLogIPs = args.GetBoolArg("-logips", DEFAULT_LOGIPS);
        */
}

pub fn set_logging_categories(args: &ArgsManager)  {
    
    todo!();
        /*
            if (args.IsArgSet("-debug")) {
            // Special-case: if -debug=0/-nodebug is set, turn off debugging messages
            const std::vector<std::string> categories = args.GetArgs("-debug");

            if (std::none_of(categories.begin(), categories.end(),
                [](std::string cat){return cat == "0" || cat == "none";})) {
                for (const auto& cat : categories) {
                    if (!LogInstance().EnableCategory(cat)) {
                        InitWarning(strprintf(_("Unsupported logging category %s=%s."), "-debug", cat));
                    }
                }
            }
        }

        // Now remove the logging categories which were explicitly excluded
        for (const std::string& cat : args.GetArgs("-debugexclude")) {
            if (!LogInstance().DisableCategory(cat)) {
                InitWarning(strprintf(_("Unsupported logging category %s=%s."), "-debugexclude", cat));
            }
        }
        */
}

pub fn start_logging(args: &ArgsManager) -> bool {
    
    todo!();
        /*
            if (LogInstance().m_print_to_file) {
            if (args.GetBoolArg("-shrinkdebugfile", LogInstance().DefaultShrinkDebugFile())) {
                // Do this first since it both loads a bunch of debug.log into memory,
                // and because this needs to happen before any other debug.log printing
                LogInstance().ShrinkDebugFile();
            }
        }
        if (!LogInstance().StartLogging()) {
                return InitError(strprintf(Untranslated("Could not open debug log file %s"),
                    fs::PathToString(LogInstance().m_file_path)));
        }

        if (!LogInstance().m_log_timestamps)
            LogPrintf("Startup time: %s\n", FormatISO8601DateTime(GetTime()));
        LogPrintf("Default data directory %s\n", fs::PathToString(GetDefaultDataDir()));
        LogPrintf("Using data directory %s\n", fs::PathToString(gArgs.GetDataDirNet()));

        // Only log conf file usage message if conf file actually exists.
        fs::path config_file_path = GetConfigFile(args.GetArg("-conf", BITCOIN_CONF_FILENAME));
        if (fs::exists(config_file_path)) {
            LogPrintf("Config file: %s\n", fs::PathToString(config_file_path));
        } else if (args.IsArgSet("-conf")) {
            // Warn if no conf file exists at path provided by user
            InitWarning(strprintf(_("The specified config file %s does not exist"), fs::PathToString(config_file_path)));
        } else {
            // Not categorizing as "Warning" because it's the default behavior
            LogPrintf("Config file: %s (not found, skipping)\n", fs::PathToString(config_file_path));
        }

        // Log the config arguments to debug.log
        args.LogArgs();

        return true;
        */
}

pub fn log_package_version()  {
    
    todo!();
        /*
            std::string version_string = FormatFullVersion();
    #ifdef DEBUG
        version_string += " (debug build)";
    #else
        version_string += " (release build)";
    #endif
        LogPrintf(PACKAGE_NAME " version %s\n", version_string);
        */
}
