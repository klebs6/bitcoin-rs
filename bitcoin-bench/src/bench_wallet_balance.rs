crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/wallet_balance.cpp]

pub fn wallet_balance(
        bench:     &mut Bencher,
        set_dirty: bool,
        add_mine:  bool)  {
    
    todo!();
        /*
            const auto test_setup = MakeNoLogFileContext<const TestingSetup>();

        const auto& ADDRESS_WATCHONLY = ADDRESS_BCRT1_UNSPENDABLE;

        CWallet wallet{test_setup->m_node.chain.get(), "", CreateMockWalletDatabase()};
        {
            LOCK(wallet.cs_wallet);
            wallet.SetWalletFlag(WALLET_FLAG_DESCRIPTORS);
            wallet.SetupDescriptorScriptPubKeyMans();
            if (wallet.LoadWallet() != DBErrors::LOAD_OK) assert(false);
        }
        auto handler = test_setup->m_node.chain->handleNotifications({&wallet, [](CWallet*) {}});

        const std::optional<std::string> address_mine{add_mine ? std::optional<std::string>{getnewaddress(wallet)} : std::nullopt};

        for (int i = 0; i < 100; ++i) {
            generatetoaddress(test_setup->m_node, address_mine.value_or(ADDRESS_WATCHONLY));
            generatetoaddress(test_setup->m_node, ADDRESS_WATCHONLY);
        }
        SyncWithValidationInterfaceQueue();

        auto bal = GetBalance(wallet); // Cache

        bench.run([&] {
            if (set_dirty) wallet.MarkDirty();
            bal = GetBalance(wallet);
            if (add_mine) assert(bal.m_mine_trusted > 0);
        });
        */
}


#[bench] fn wallet_balance_dirty(b: &mut Bencher)  {
    
    todo!();
        /*
            WalletBalance(bench, /* set_dirty */ true, /* add_mine */ true);
        */
}

#[bench] fn wallet_balance_clean(b: &mut Bencher)  {
    
    todo!();
        /*
            WalletBalance(bench, /* set_dirty */ false, /* add_mine */ true);
        */
}

#[bench] fn wallet_balance_mine(b: &mut Bencher)  {
    
    todo!();
        /*
            WalletBalance(bench, /* set_dirty */ false, /* add_mine */ true);
        */
}

#[bench] fn wallet_balance_watch(b: &mut Bencher)  {
    
    todo!();
        /*
            WalletBalance(bench, /* set_dirty */ false, /* add_mine */ false);
        */
}
