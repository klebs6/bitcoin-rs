crate::ix!();

pub fn wallet_create(
    wallet_instance:       *mut Wallet,
    wallet_creation_flags: u64)  {
    
    todo!();
        /*
            LOCK(wallet_instance->cs_wallet);

        wallet_instance->SetMinVersion(FEATURE_HD_SPLIT);
        wallet_instance->AddWalletFlags(wallet_creation_flags);

        if (!wallet_instance->IsWalletFlagSet(WALLET_FLAG_DESCRIPTORS)) {
            auto spk_man = wallet_instance->GetOrCreateLegacyScriptPubKeyMan();
            spk_man->SetupGeneration(false);
        } else {
            wallet_instance->SetupDescriptorScriptPubKeyMans();
        }

        tfm::format(std::cout, "Topping up keypool...\n");
        wallet_instance->TopUpKeyPool();
        */
}
