// ---------------- [ File: bitcoin-validation/tests/validation_chainstate_manager.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/validation_chainstatemanager_tests.cpp]

#[cfg(test)]
#[fixture(ChainTestingSetup)]
pub mod validation_chainstatemanager_tests {

    /**
      | Basic tests for ChainstateManager.
      |
      | First create a legacy (IBD) chainstate, then
      | create a snapshot chainstate.
      */
    #[test] fn chainstatemanager() {
        todo!();
        /*
        
            ChainstateManager& manager = *m_node.chainman;
            CTxMemPool& mempool = *m_node.mempool;

            std::vector<CChainState*> chainstates;

            BOOST_CHECK(!manager.SnapshotBlockhash().has_value());

            // Create a legacy (IBD) chainstate.
            //
            CChainState& c1 = 
        [&]() { LOCK(::cs_main);  return manager.InitializeChainstate(&mempool) }()
        ;
            chainstates.push_back(&c1);
            c1.InitCoinsDB(
                /* cache_size_bytes */ 1 << 23, /* in_memory */ true, /* should_wipe */ false);
            
        [&]() { LOCK(::cs_main);  c1.InitCoinsCache(1 << 23) }()
        ;

            BOOST_CHECK(!manager.IsSnapshotActive());
            BOOST_CHECK(!manager.IsSnapshotValidated());
            auto all = manager.GetAll();
            BOOST_CHECK_EQUAL_COLLECTIONS(all.begin(), all.end(), chainstates.begin(), chainstates.end());

            auto& active_chain = manager.ActiveChain();
            BOOST_CHECK_EQUAL(&active_chain, &c1.m_chain);

            BOOST_CHECK_EQUAL(manager.ActiveHeight(), -1);

            auto active_tip = manager.ActiveTip();
            auto exp_tip = c1.m_chain.Tip();
            BOOST_CHECK_EQUAL(active_tip, exp_tip);

            BOOST_CHECK(!manager.SnapshotBlockhash().has_value());

            // Create a snapshot-based chainstate.
            //
            const uint256 snapshot_blockhash = GetRandHash();
            CChainState& c2 = 
        [&]() { LOCK(::cs_main);  return manager.InitializeChainstate( &mempool, snapshot_blockhash) }()
        ;
            chainstates.push_back(&c2);

            BOOST_CHECK_EQUAL(manager.SnapshotBlockhash().value(), snapshot_blockhash);

            c2.InitCoinsDB(
                /* cache_size_bytes */ 1 << 23, /* in_memory */ true, /* should_wipe */ false);
            
        [&]() { LOCK(::cs_main);  c2.InitCoinsCache(1 << 23) }()
        ;
            // Unlike c1, which doesn't have any blocks. Gets us different tip, height.
            c2.LoadGenesisBlock();
            BlockValidationState _;
            BOOST_CHECK(c2.ActivateBestChain(_, nullptr));

            BOOST_CHECK(manager.IsSnapshotActive());
            BOOST_CHECK(!manager.IsSnapshotValidated());
            BOOST_CHECK_EQUAL(&c2, &manager.ActiveChainstate());
            BOOST_CHECK(&c1 != &manager.ActiveChainstate());
            auto all2 = manager.GetAll();
            BOOST_CHECK_EQUAL_COLLECTIONS(all2.begin(), all2.end(), chainstates.begin(), chainstates.end());

            auto& active_chain2 = manager.ActiveChain();
            BOOST_CHECK_EQUAL(&active_chain2, &c2.m_chain);

            BOOST_CHECK_EQUAL(manager.ActiveHeight(), 0);

            auto active_tip2 = manager.ActiveTip();
            auto exp_tip2 = c2.m_chain.Tip();
            BOOST_CHECK_EQUAL(active_tip2, exp_tip2);

            // Ensure that these pointers actually correspond to different
            // CCoinsViewCache instances.
            BOOST_CHECK(exp_tip != exp_tip2);

            // Let scheduler events finish running to avoid accessing memory that is going to be unloaded
            SyncWithValidationInterfaceQueue();

            
        [&]() { LOCK(::cs_main);  manager.Unload() }()
        ;

        */
    }

    /**
      | Test rebalancing the caches associated
      | with each chainstate.
      |
      */
    #[test] fn chainstatemanager_rebalance_caches() {
        todo!();
        /*
        
            ChainstateManager& manager = *m_node.chainman;
            CTxMemPool& mempool = *m_node.mempool;

            size_t max_cache = 10000;
            manager.m_total_coinsdb_cache = max_cache;
            manager.m_total_coinstip_cache = max_cache;

            std::vector<CChainState*> chainstates;

            // Create a legacy (IBD) chainstate.
            //
            CChainState& c1 = 
        [&]() { LOCK(cs_main);  return manager.InitializeChainstate(&mempool) }()
        ;
            chainstates.push_back(&c1);
            c1.InitCoinsDB(
                /* cache_size_bytes */ 1 << 23, /* in_memory */ true, /* should_wipe */ false);

            {
                LOCK(::cs_main);
                c1.InitCoinsCache(1 << 23);
                BOOST_REQUIRE(c1.LoadGenesisBlock());
                c1.CoinsTip().SetBestBlock(InsecureRand256());
                manager.MaybeRebalanceCaches();
            }

            BOOST_CHECK_EQUAL(c1.m_coinstip_cache_size_bytes, max_cache);
            BOOST_CHECK_EQUAL(c1.m_coinsdb_cache_size_bytes, max_cache);

            // Create a snapshot-based chainstate.
            //
            CChainState& c2 = 
        [&]() { LOCK(cs_main);  return manager.InitializeChainstate(&mempool, GetRandHash()) }()
        ;
            chainstates.push_back(&c2);
            c2.InitCoinsDB(
                /* cache_size_bytes */ 1 << 23, /* in_memory */ true, /* should_wipe */ false);

            {
                LOCK(::cs_main);
                c2.InitCoinsCache(1 << 23);
                BOOST_REQUIRE(c2.LoadGenesisBlock());
                c2.CoinsTip().SetBestBlock(InsecureRand256());
                manager.MaybeRebalanceCaches();
            }

            // Since both chainstates are considered to be in initial block download,
            // the snapshot chainstate should take priority.
            BOOST_CHECK_CLOSE(c1.m_coinstip_cache_size_bytes, max_cache * 0.05, 1);
            BOOST_CHECK_CLOSE(c1.m_coinsdb_cache_size_bytes, max_cache * 0.05, 1);
            BOOST_CHECK_CLOSE(c2.m_coinstip_cache_size_bytes, max_cache * 0.95, 1);
            BOOST_CHECK_CLOSE(c2.m_coinsdb_cache_size_bytes, max_cache * 0.95, 1);

        */
    }

    /**
      | Test basic snapshot activation.
      |
      */
    #[test] fn chainstatemanager_activate_snapshot_test_chain_100setup() {
        todo!();
        /*
        
            ChainstateManager& chainman = *Assert(m_node.chainman);

            size_t initial_size;
            size_t initial_total_coins{100};

            // Make some initial assertions about the contents of the chainstate.
            {
                LOCK(::cs_main);
                CCoinsViewCache& ibd_coinscache = chainman.ActiveChainstate().CoinsTip();
                initial_size = ibd_coinscache.GetCacheSize();
                size_t total_coins{0};

                for (CTransactionRef& txn : m_coinbase_txns) {
                    OutPoint op{txn->GetHash(), 0};
                    BOOST_CHECK(ibd_coinscache.HaveCoin(op));
                    total_coins++;
                }

                BOOST_CHECK_EQUAL(total_coins, initial_total_coins);
                BOOST_CHECK_EQUAL(initial_size, initial_total_coins);
            }

            // Snapshot should refuse to load at this height.
            BOOST_REQUIRE(!CreateAndActivateUTXOSnapshot(m_node, m_path_root));
            BOOST_CHECK(!chainman.ActiveChainstate().m_from_snapshot_blockhash);
            BOOST_CHECK(!chainman.SnapshotBlockhash());

            // Mine 10 more blocks, putting at us height 110 where a valid assumeutxo value can
            // be found.
            constexpr int snapshot_height = 110;
            mineBlocks(10);
            initial_size += 10;
            initial_total_coins += 10;

            // Should not load malleated snapshots
            BOOST_REQUIRE(!CreateAndActivateUTXOSnapshot(
                m_node, m_path_root, [](CAutoFile& auto_infile, SnapshotMetadata& metadata) {
                    // A UTXO is missing but count is correct
                    metadata.m_coins_count -= 1;

                    OutPoint outpoint;
                    Coin coin;

                    auto_infile >> outpoint;
                    auto_infile >> coin;
            }));
            BOOST_REQUIRE(!CreateAndActivateUTXOSnapshot(
                m_node, m_path_root, [](CAutoFile& auto_infile, SnapshotMetadata& metadata) {
                    // Coins count is larger than coins in file
                    metadata.m_coins_count += 1;
            }));
            BOOST_REQUIRE(!CreateAndActivateUTXOSnapshot(
                m_node, m_path_root, [](CAutoFile& auto_infile, SnapshotMetadata& metadata) {
                    // Coins count is smaller than coins in file
                    metadata.m_coins_count -= 1;
            }));
            BOOST_REQUIRE(!CreateAndActivateUTXOSnapshot(
                m_node, m_path_root, [](CAutoFile& auto_infile, SnapshotMetadata& metadata) {
                    // Wrong hash
                    metadata.m_base_blockhash = uint256::ZERO;
            }));
            BOOST_REQUIRE(!CreateAndActivateUTXOSnapshot(
                m_node, m_path_root, [](CAutoFile& auto_infile, SnapshotMetadata& metadata) {
                    // Wrong hash
                    metadata.m_base_blockhash = uint256::ONE;
            }));

            BOOST_REQUIRE(CreateAndActivateUTXOSnapshot(m_node, m_path_root));

            // Ensure our active chain is the snapshot chainstate.
            BOOST_CHECK(!chainman.ActiveChainstate().m_from_snapshot_blockhash->IsNull());
            BOOST_CHECK_EQUAL(
                *chainman.ActiveChainstate().m_from_snapshot_blockhash,
                *chainman.SnapshotBlockhash());

            const AssumeutxoData& au_data = *ExpectedAssumeutxo(snapshot_height, ::Params());
            const CBlockIndex* tip = chainman.ActiveTip();

            BOOST_CHECK_EQUAL(tip->nChainTx, au_data.nChainTx);

            // To be checked against later when we try loading a subsequent snapshot.
            uint256 loaded_snapshot_blockhash{*chainman.SnapshotBlockhash()};

            // Make some assertions about the both chainstates. These checks ensure the
            // legacy chainstate hasn't changed and that the newly created chainstate
            // reflects the expected content.
            {
                LOCK(::cs_main);
                int chains_tested{0};

                for (CChainState* chainstate : chainman.GetAll()) {
                    BOOST_TEST_MESSAGE("Checking coins in " << chainstate->ToString());
                    CCoinsViewCache& coinscache = chainstate->CoinsTip();

                    // Both caches will be empty initially.
                    BOOST_CHECK_EQUAL((unsigned int)0, coinscache.GetCacheSize());

                    size_t total_coins{0};

                    for (CTransactionRef& txn : m_coinbase_txns) {
                        OutPoint op{txn->GetHash(), 0};
                        BOOST_CHECK(coinscache.HaveCoin(op));
                        total_coins++;
                    }

                    BOOST_CHECK_EQUAL(initial_size , coinscache.GetCacheSize());
                    BOOST_CHECK_EQUAL(total_coins, initial_total_coins);
                    chains_tested++;
                }

                BOOST_CHECK_EQUAL(chains_tested, 2);
            }

            // Mine some new blocks on top of the activated snapshot chainstate.
            constexpr size_t new_coins{100};
            mineBlocks(new_coins);  // Defined in TestChain100Setup.

            {
                LOCK(::cs_main);
                size_t coins_in_active{0};
                size_t coins_in_background{0};
                size_t coins_missing_from_background{0};

                for (CChainState* chainstate : chainman.GetAll()) {
                    BOOST_TEST_MESSAGE("Checking coins in " << chainstate->ToString());
                    CCoinsViewCache& coinscache = chainstate->CoinsTip();
                    bool is_background = chainstate != &chainman.ActiveChainstate();

                    for (CTransactionRef& txn : m_coinbase_txns) {
                        OutPoint op{txn->GetHash(), 0};
                        if (coinscache.HaveCoin(op)) {
                            (is_background ? coins_in_background : coins_in_active)++;
                        } else if (is_background) {
                            coins_missing_from_background++;
                        }
                    }
                }

                BOOST_CHECK_EQUAL(coins_in_active, initial_total_coins + new_coins);
                BOOST_CHECK_EQUAL(coins_in_background, initial_total_coins);
                BOOST_CHECK_EQUAL(coins_missing_from_background, new_coins);
            }

            // Snapshot should refuse to load after one has already loaded.
            BOOST_REQUIRE(!CreateAndActivateUTXOSnapshot(m_node, m_path_root));

            // Snapshot blockhash should be unchanged.
            BOOST_CHECK_EQUAL(
                *chainman.ActiveChainstate().m_from_snapshot_blockhash,
                loaded_snapshot_blockhash);

        */
    }
}
