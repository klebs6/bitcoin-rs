crate::ix!();

pub fn make_wallet(
        name:    &String,
        path:    &Path,
        options: DatabaseOptions) -> Arc<Wallet> {
    
    todo!();
        /*
            DatabaseStatus status;
        bilingual_str error;
        std::unique_ptr<WalletDatabase> database = MakeDatabase(path, options, status, error);
        if (!database) {
            tfm::format(std::cerr, "%s\n", error.original);
            return nullptr;
        }

        // dummy chain interface
        std::shared_ptr<CWallet> wallet_instance{new CWallet(nullptr /* chain */, name, std::move(database)), WalletToolReleaseWallet};
        DBErrors load_wallet_ret;
        try {
            load_wallet_ret = wallet_instance->LoadWallet();
        } catch (const std::runtime_error&) {
            tfm::format(std::cerr, "Error loading %s. Is wallet being used by another process?\n", name);
            return nullptr;
        }

        if (load_wallet_ret != DBErrors::LOAD_OK) {
            wallet_instance = nullptr;
            if (load_wallet_ret == DBErrors::CORRUPT) {
                tfm::format(std::cerr, "Error loading %s: Wallet corrupted", name);
                return nullptr;
            } else if (load_wallet_ret == DBErrors::NONCRITICAL_ERROR) {
                tfm::format(std::cerr, "Error reading %s! All keys read correctly, but transaction data"
                                " or address book entries might be missing or incorrect.",
                    name);
            } else if (load_wallet_ret == DBErrors::TOO_NEW) {
                tfm::format(std::cerr, "Error loading %s: Wallet requires newer version of %s",
                    name, PACKAGE_NAME);
                return nullptr;
            } else if (load_wallet_ret == DBErrors::NEED_REWRITE) {
                tfm::format(std::cerr, "Wallet needed to be rewritten: restart %s to complete", PACKAGE_NAME);
                return nullptr;
            } else if (load_wallet_ret == DBErrors::NEED_RESCAN) {
                tfm::format(std::cerr, "Error reading %s! Some transaction data might be missing or"
                               " incorrect. Wallet requires a rescan.",
                    name);
            } else {
                tfm::format(std::cerr, "Error loading %s", name);
                return nullptr;
            }
        }

        if (options.require_create) WalletCreate(wallet_instance.get(), options.create_flags);

        return wallet_instance;
        */
}
