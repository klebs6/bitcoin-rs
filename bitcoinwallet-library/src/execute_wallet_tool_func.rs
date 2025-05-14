// ---------------- [ File: bitcoinwallet-library/src/execute_wallet_tool_func.rs ]
crate::ix!();

pub fn execute_wallet_tool_func(
        args:    &ArgsManager,
        command: &String) -> bool {
    
    todo!();
        /*
            if (args.IsArgSet("-format") && command != "createfromdump") {
            tfm::format(std::cerr, "The -format option can only be used with the \"createfromdump\" command.\n");
            return false;
        }
        if (args.IsArgSet("-dumpfile") && command != "dump" && command != "createfromdump") {
            tfm::format(std::cerr, "The -dumpfile option can only be used with the \"dump\" and \"createfromdump\" commands.\n");
            return false;
        }
        if (args.IsArgSet("-descriptors") && command != "create") {
            tfm::format(std::cerr, "The -descriptors option can only be used with the 'create' command.\n");
            return false;
        }
        if (args.IsArgSet("-legacy") && command != "create") {
            tfm::format(std::cerr, "The -legacy option can only be used with the 'create' command.\n");
            return false;
        }
        if (command == "create" && !args.IsArgSet("-wallet")) {
            tfm::format(std::cerr, "Wallet name must be provided when creating a new wallet.\n");
            return false;
        }
        const std::string name = args.GetArg("-wallet", "");
        const fs::path path = fsbridge::AbsPathJoin(GetWalletDir(), fs::PathFromString(name));

        if (command == "create") {
            DatabaseOptions options;
            options.require_create = true;
            // If -legacy is set, use it. Otherwise default to false.
            bool make_legacy = args.GetBoolArg("-legacy", false);
            // If neither -legacy nor -descriptors is set, default to true. If -descriptors is set, use its value.
            bool make_descriptors = (!args.IsArgSet("-descriptors") && !args.IsArgSet("-legacy")) || (args.IsArgSet("-descriptors") && args.GetBoolArg("-descriptors", true));
            if (make_legacy && make_descriptors) {
                tfm::format(std::cerr, "Only one of -legacy or -descriptors can be set to true, not both\n");
                return false;
            }
            if (!make_legacy && !make_descriptors) {
                tfm::format(std::cerr, "One of -legacy or -descriptors must be set to true (or omitted)\n");
                return false;
            }
            if (make_descriptors) {
                options.create_flags |= WALLET_FLAG_DESCRIPTORS;
                options.require_format = DatabaseFormat::SQLITE;
            }

            std::shared_ptr<CWallet> wallet_instance = MakeWallet(name, path, options);
            if (wallet_instance) {
                WalletShowInfo(wallet_instance.get());
                wallet_instance->Close();
            }
        } else if (command == "info") {
            DatabaseOptions options;
            options.require_existing = true;
            std::shared_ptr<CWallet> wallet_instance = MakeWallet(name, path, options);
            if (!wallet_instance) return false;
            WalletShowInfo(wallet_instance.get());
            wallet_instance->Close();
        } else if (command == "salvage") {
    #ifdef USE_BDB
            bilingual_str error;
            std::vector<bilingual_str> warnings;
            bool ret = RecoverDatabaseFile(path, error, warnings);
            if (!ret) {
                for (const auto& warning : warnings) {
                    tfm::format(std::cerr, "%s\n", warning.original);
                }
                if (!error.empty()) {
                    tfm::format(std::cerr, "%s\n", error.original);
                }
            }
            return ret;
    #else
            tfm::format(std::cerr, "Salvage command is not available as BDB support is not compiled");
            return false;
    #endif
        } else if (command == "dump") {
            DatabaseOptions options;
            options.require_existing = true;
            std::shared_ptr<CWallet> wallet_instance = MakeWallet(name, path, options);
            if (!wallet_instance) return false;
            bilingual_str error;
            bool ret = DumpWallet(*wallet_instance, error);
            if (!ret && !error.empty()) {
                tfm::format(std::cerr, "%s\n", error.original);
                return ret;
            }
            tfm::format(std::cout, "The dumpfile may contain private keys. To ensure the safety of your Bitcoin, do not share the dumpfile.\n");
            return ret;
        } else if (command == "createfromdump") {
            bilingual_str error;
            std::vector<bilingual_str> warnings;
            bool ret = CreateFromDump(name, path, error, warnings);
            for (const auto& warning : warnings) {
                tfm::format(std::cout, "%s\n", warning.original);
            }
            if (!ret && !error.empty()) {
                tfm::format(std::cerr, "%s\n", error.original);
            }
            return ret;
        } else {
            tfm::format(std::cerr, "Invalid command: %s\n", command);
            return false;
        }

        return true;
        */
}
