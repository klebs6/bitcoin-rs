// ---------------- [ File: bitcoinwallet-init/src/load.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/load.h]
//-------------------------------------------[.cpp/bitcoin/src/wallet/load.cpp]

/**
  | Responsible for reading and validating
  | the -wallet arguments and verifying the
  | wallet database.
  |
  */
pub fn verify_wallets<'a>(context: &'a mut WalletContext) -> bool {
    
    todo!();
        /*
            interfaces::Chain& chain = *context.chain;
        ArgsManager& args = *Assert(context.args);

        if (args.IsArgSet("-walletdir")) {
            fs::path wallet_dir = fs::PathFromString(args.GetArg("-walletdir", ""));
            boost::system::error_code error;
            // The canonical path cleans the path, preventing >1 Berkeley environment instances for the same directory
            fs::path canonical_wallet_dir = fs::canonical(wallet_dir, error);
            if (error || !fs::exists(wallet_dir)) {
                chain.initError(strprintf(_("Specified -walletdir \"%s\" does not exist"), fs::PathToString(wallet_dir)));
                return false;
            } else if (!fs::is_directory(wallet_dir)) {
                chain.initError(strprintf(_("Specified -walletdir \"%s\" is not a directory"), fs::PathToString(wallet_dir)));
                return false;
            // The canonical path transforms relative paths into absolute ones, so we check the non-canonical version
            } else if (!wallet_dir.is_absolute()) {
                chain.initError(strprintf(_("Specified -walletdir \"%s\" is a relative path"), fs::PathToString(wallet_dir)));
                return false;
            }
            args.ForceSetArg("-walletdir", fs::PathToString(canonical_wallet_dir));
        }

        LogPrintf("Using wallet directory %s\n", fs::PathToString(GetWalletDir()));

        chain.initMessage(_("Verifying wallet(s)…").translated);

        // For backwards compatibility if an unnamed top level wallet exists in the
        // wallets directory, include it in the default list of wallets to load.
        if (!args.IsArgSet("wallet")) {
            DatabaseOptions options;
            DatabaseStatus status;
            bilingual_str error_string;
            options.require_existing = true;
            options.verify = false;
            if (MakeWalletDatabase("", options, status, error_string)) {
                util::SettingsValue wallets(util::SettingsValue::VARR);
                wallets.push_back(""); // Default wallet name is ""
                // Pass write=false because no need to write file and probably
                // better not to. If unnamed wallet needs to be added next startup
                // and the setting is empty, this code will just run again.
                chain.updateRwSetting("wallet", wallets, /* write= */ false);
            }
        }

        // Keep track of each wallet absolute path to detect duplicates.
        std::set<fs::path> wallet_paths;

        for (const auto& wallet : chain.getSettingsList("wallet")) {
            const auto& wallet_file = wallet.get_str();
            const fs::path path = fsbridge::AbsPathJoin(GetWalletDir(), fs::PathFromString(wallet_file));

            if (!wallet_paths.insert(path).second) {
                chain.initWarning(strprintf(_("Ignoring duplicate -wallet %s."), wallet_file));
                continue;
            }

            DatabaseOptions options;
            DatabaseStatus status;
            options.require_existing = true;
            options.verify = true;
            bilingual_str error_string;
            if (!MakeWalletDatabase(wallet_file, options, status, error_string)) {
                if (status == DatabaseStatus::FAILED_NOT_FOUND) {
                    chain.initWarning(Untranslated(strprintf("Skipping -wallet path that doesn't exist. %s", error_string.original)));
                } else {
                    chain.initError(error_string);
                    return false;
                }
            }
        }

        return true;
        */
}

/**
  | Load wallet databases.
  |
  */
pub fn load_wallets(context: &mut WalletContext) -> bool {
    
    todo!();
        /*
            interfaces::Chain& chain = *context.chain;
        try {
            std::set<fs::path> wallet_paths;
            for (const auto& wallet : chain.getSettingsList("wallet")) {
                const auto& name = wallet.get_str();
                if (!wallet_paths.insert(fs::PathFromString(name)).second) {
                    continue;
                }
                DatabaseOptions options;
                DatabaseStatus status;
                options.require_existing = true;
                options.verify = false; // No need to verify, assuming verified earlier in VerifyWallets()
                bilingual_str error;
                std::vector<bilingual_str> warnings;
                std::unique_ptr<WalletDatabase> database = MakeWalletDatabase(name, options, status, error);
                if (!database && status == DatabaseStatus::FAILED_NOT_FOUND) {
                    continue;
                }
                chain.initMessage(_("Loading wallet…").translated);
                std::shared_ptr<CWallet> pwallet = database ? CWallet::Create(context, name, std::move(database), options.create_flags, error, warnings) : nullptr;
                if (!warnings.empty()) chain.initWarning(Join(warnings, Untranslated("\n")));
                if (!pwallet) {
                    chain.initError(error);
                    return false;
                }
                AddWallet(context, pwallet);
            }
            return true;
        } catch (const std::runtime_error& e) {
            chain.initError(Untranslated(e.what()));
            return false;
        }
        */
}

/**
  | Complete startup of wallets.
  |
  */
pub fn start_wallets(
        context:   &mut WalletContext,
        scheduler: &mut Scheduler)  {
    
    todo!();
        /*
            for (const std::shared_ptr<CWallet>& pwallet : GetWallets(context)) {
            pwallet->postInitProcess();
        }

        // Schedule periodic wallet flushes and tx rebroadcasts
        if (context.args->GetBoolArg("-flushwallet", DEFAULT_FLUSHWALLET)) {
            scheduler.scheduleEvery([&context] { MaybeCompactWalletDB(context); }, std::chrono::milliseconds{500});
        }
        scheduler.scheduleEvery([&context] { MaybeResendWalletTxs(context); }, std::chrono::milliseconds{1000});
        */
}

/**
  | Flush all wallets in preparation for
  | shutdown.
  |
  */
pub fn flush_wallets(context: &mut WalletContext)  {
    
    todo!();
        /*
            for (const std::shared_ptr<CWallet>& pwallet : GetWallets(context)) {
            pwallet->Flush();
        }
        */
}

/**
  | Stop all wallets. Wallets will be flushed
  | first.
  |
  */
pub fn stop_wallets(context: &mut WalletContext)  {
    
    todo!();
        /*
            for (const std::shared_ptr<CWallet>& pwallet : GetWallets(context)) {
            pwallet->Close();
        }
        */
}

/**
  | Close all wallets.
  |
  */
pub fn unload_wallets(context: &mut WalletContext)  {
    
    todo!();
        /*
            auto wallets = GetWallets(context);
        while (!wallets.empty()) {
            auto wallet = wallets.back();
            wallets.pop_back();
            std::vector<bilingual_str> warnings;
            RemoveWallet(context, wallet, /* load_on_start= */ std::nullopt, warnings);
            UnloadWallet(std::move(wallet));
        }
        */
}
