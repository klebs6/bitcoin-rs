crate::ix!();

pub fn wallet_show_info(wallet_instance: *mut Wallet)  {
    
    todo!();
        /*
            LOCK(wallet_instance->cs_wallet);

        tfm::format(std::cout, "Wallet info\n===========\n");
        tfm::format(std::cout, "Name: %s\n", wallet_instance->GetName());
        tfm::format(std::cout, "Format: %s\n", wallet_instance->GetDatabase().Format());
        tfm::format(std::cout, "Descriptors: %s\n", wallet_instance->IsWalletFlagSet(WALLET_FLAG_DESCRIPTORS) ? "yes" : "no");
        tfm::format(std::cout, "Encrypted: %s\n", wallet_instance->IsCrypted() ? "yes" : "no");
        tfm::format(std::cout, "HD (hd seed available): %s\n", wallet_instance->IsHDEnabled() ? "yes" : "no");
        tfm::format(std::cout, "Keypool Size: %u\n", wallet_instance->GetKeyPoolSize());
        tfm::format(std::cout, "Transactions: %zu\n", wallet_instance->mapWallet.size());
        tfm::format(std::cout, "Address Book: %zu\n", wallet_instance->m_address_book.size());
        */
}
