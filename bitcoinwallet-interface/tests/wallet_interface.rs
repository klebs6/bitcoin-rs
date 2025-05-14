// ---------------- [ File: bitcoinwallet-interface/tests/wallet_interface.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/test/util.h]
//-------------------------------------------[.cpp/bitcoin/src/wallet/test/util.cpp]

pub fn create_synced_wallet(
    chain:  &mut dyn ChainInterface,
    cchain: &mut Chain,
    key:    &Key) -> Box<dyn WalletInterface> 
{
    todo!();
        /*
            auto wallet = std::make_unique<CWallet>(&chain, "", CreateMockWalletDatabase());
        {
            LOCK2(wallet->cs_wallet, ::cs_main);
            wallet->SetLastBlockProcessed(cchain.Height(), cchain.Tip()->GetBlockHash());
        }
        wallet->LoadWallet();
        {
            LOCK(wallet->cs_wallet);
            wallet->SetWalletFlag(WALLET_FLAG_DESCRIPTORS);
            wallet->SetupDescriptorScriptPubKeyMans();

            FlatSigningProvider provider;
            std::string error;
            std::unique_ptr<Descriptor> desc = Parse("combo(" + EncodeSecret(key) + ")", provider, error, /* require_checksum=*/ false);
            assert(desc);
            WalletDescriptor w_desc(std::move(desc), 0, 0, 1, 1);
            if (!wallet->AddWalletDescriptor(w_desc, provider, "", false)) assert(false);
        }
        WalletRescanReserver reserver(*wallet);
        reserver.reserve();
        CWallet::ScanResult result = wallet->ScanForWalletTransactions(cchain.Genesis()->GetBlockHash(), 0 /* start_height */, {} /* max_height */, reserver, false /* update */);
        BOOST_CHECK_EQUAL(result.status, CWallet::ScanResult::SUCCESS);
        BOOST_CHECK_EQUAL(result.last_scanned_block, cchain.Tip()->GetBlockHash());
        BOOST_CHECK_EQUAL(*result.last_scanned_height, cchain.Height());
        BOOST_CHECK(result.last_failed_block.IsNull());
        return wallet;
        */
}
