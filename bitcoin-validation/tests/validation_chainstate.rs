// ---------------- [ File: bitcoin-validation/tests/validation_chainstate.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/validation_chainstate_tests.cpp]

#[cfg(test)]
#[fixture(TestingSetup)]
pub mod validation_chainstate_tests {

    /**
      | Test resizing coins-related CChainState
      | caches during runtime.
      |
      */
    #[test] fn validation_chainstate_resize_caches() {
        todo!();
        /*
        
            ChainstateManager manager;
            
        [&]() { LOCK(::cs_main);  manager.m_blockman.m_block_tree_db = std::make_unique<CBlockTreeDB>(1 << 20, true) }()
        ;
            CTxMemPool mempool;

            /// Create and add a Coin with DynamicMemoryUsage of 80 bytes to the given view.
            auto add_coin = [](CCoinsViewCache& coins_view) -> OutPoint {
                Coin newcoin;
                uint256 txid = InsecureRand256();
                OutPoint outp{txid, 0};
                newcoin.nHeight = 1;
                newcoin.out.nValue = InsecureRand32();
                newcoin.out.scriptPubKey.assign((uint32_t)56, 1);
                coins_view.AddCoin(outp, std::move(newcoin), false);

                return outp;
            };

            CChainState& c1 = 
        [&]() { LOCK(cs_main);  return manager.InitializeChainstate(&mempool) }()
        ;
            c1.InitCoinsDB(
                /* cache_size_bytes */ 1 << 23, /* in_memory */ true, /* should_wipe */ false);
            
        [&]() { LOCK(::cs_main);  c1.InitCoinsCache(1 << 23) }()
        ;

            // Add a coin to the in-memory cache, upsize once, then downsize.
            {
                LOCK(::cs_main);
                auto outpoint = add_coin(c1.CoinsTip());

                // Set a meaningless bestblock value in the coinsview cache - otherwise we won't
                // flush during ResizecoinsCaches() and will subsequently hit an assertion.
                c1.CoinsTip().SetBestBlock(InsecureRand256());

                BOOST_CHECK(c1.CoinsTip().HaveCoinInCache(outpoint));

                c1.ResizeCoinsCaches(
                    1 << 24,  // upsizing the coinsview cache
                    1 << 22  // downsizing the coinsdb cache
                );

                // View should still have the coin cached, since we haven't destructed the cache on upsize.
                BOOST_CHECK(c1.CoinsTip().HaveCoinInCache(outpoint));

                c1.ResizeCoinsCaches(
                    1 << 22,  // downsizing the coinsview cache
                    1 << 23  // upsizing the coinsdb cache
                );

                // The view cache should be empty since we had to destruct to downsize.
                BOOST_CHECK(!c1.CoinsTip().HaveCoinInCache(outpoint));
            }

            // Avoid triggering the address sanitizer.
            
        [&]() { LOCK(::cs_main);  manager.Unload() }()
        ;

        */
    }

    /**
      | Test UpdateTip behavior for both active and
      | background chainstates.
      |
      | When run on the background chainstate,
      | UpdateTip should do a subset of what it does
      | for the active chainstate.
      */
    #[test] fn chainstate_update_tip_test_chain_100setup() {
        todo!();
        /*
        
            ChainstateManager& chainman = *Assert(m_node.chainman);
            uint256 curr_tip = ::g_best_block;

            // Mine 10 more blocks, putting at us height 110 where a valid assumeutxo value can
            // be found.
            mineBlocks(10);

            // After adding some blocks to the tip, best block should have changed.
            BOOST_CHECK(::g_best_block != curr_tip);

            BOOST_REQUIRE(CreateAndActivateUTXOSnapshot(m_node, m_path_root));

            // Ensure our active chain is the snapshot chainstate.
            BOOST_CHECK(chainman.IsSnapshotActive());

            curr_tip = ::g_best_block;

            // Mine a new block on top of the activated snapshot chainstate.
            mineBlocks(1);  // Defined in TestChain100Setup.

            // After adding some blocks to the snapshot tip, best block should have changed.
            BOOST_CHECK(::g_best_block != curr_tip);

            curr_tip = ::g_best_block;

            BOOST_CHECK_EQUAL(chainman.GetAll().size(), 2);

            CChainState& background_cs{*[&] {
                for (CChainState* cs : chainman.GetAll()) {
                    if (cs != &chainman.ActiveChainstate()) {
                        return cs;
                    }
                }
                assert(false);
            }()};

            // Create a block to append to the validation chain.
            std::vector<CMutableTransaction> noTxns;
            CScript scriptPubKey = CScript() << ToByteVector(coinbaseKey.GetPubKey()) << OP_CHECKSIG;
            CBlock validation_block = this->CreateBlock(noTxns, scriptPubKey, background_cs);
            auto pblock = std::make_shared<const CBlock>(validation_block);
            BlockValidationState state;
            CBlockIndex* pindex = nullptr;
            const CChainParams& chainparams = Params();
            bool newblock = false;

            // TODO: much of this is inlined from ProcessNewBlock(); just reuse PNB()
            // once it is changed to support multiple chainstates.
            {
                LOCK(::cs_main);
                bool checked = CheckBlock(*pblock, state, chainparams.GetConsensus());
                BOOST_CHECK(checked);
                bool accepted = background_cs.AcceptBlock(
                    pblock, state, &pindex, true, nullptr, &newblock);
                BOOST_CHECK(accepted);
            }
            // UpdateTip is called here
            bool block_added = background_cs.ActivateBestChain(state, pblock);

            // Ensure tip is as expected
            BOOST_CHECK_EQUAL(background_cs.m_chain.Tip()->GetBlockHash(), validation_block.GetHash());

            // g_best_block should be unchanged after adding a block to the background
            // validation chain.
            BOOST_CHECK(block_added);
            BOOST_CHECK_EQUAL(curr_tip, ::g_best_block);

        */
    }
}
