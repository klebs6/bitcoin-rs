// ---------------- [ File: bitcoin-indexed-chain/tests/coinstatsindex.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/coinstatsindex_tests.cpp]

#[test] fn coinstatsindex_initial_sync_test_chain_100setup() {
    todo!();
    /*
    
        CoinStatsIndex coin_stats_index{1 << 20, true};

        CCoinsStats coin_stats{CoinStatsHashType::MUHASH};
        const CBlockIndex* block_index;
        {
            LOCK(cs_main);
            block_index = m_node.chainman->ActiveChain().Tip();
        }

        // CoinStatsIndex should not be found before it is started.
        BOOST_CHECK(!coin_stats_index.LookUpStats(block_index, coin_stats));

        // BlockUntilSyncedToCurrentChain should return false before CoinStatsIndex
        // is started.
        BOOST_CHECK(!coin_stats_index.BlockUntilSyncedToCurrentChain());

        BOOST_REQUIRE(coin_stats_index.Start(m_node.chainman->ActiveChainstate()));

        // Allow the CoinStatsIndex to catch up with the block index that is syncing
        // in a background thread.
        const auto timeout = GetTime<std::chrono::seconds>() + 120s;
        while (!coin_stats_index.BlockUntilSyncedToCurrentChain()) {
            BOOST_REQUIRE(timeout > GetTime<std::chrono::milliseconds>());
            UninterruptibleSleep(100ms);
        }

        // Check that CoinStatsIndex works for genesis block.
        const CBlockIndex* genesis_block_index;
        {
            LOCK(cs_main);
            genesis_block_index = m_node.chainman->ActiveChain().Genesis();
        }
        BOOST_CHECK(coin_stats_index.LookUpStats(genesis_block_index, coin_stats));

        // Check that CoinStatsIndex updates with new blocks.
        coin_stats_index.LookUpStats(block_index, coin_stats);

        const CScript script_pub_key{CScript() << ToByteVector(coinbaseKey.GetPubKey()) << OP_CHECKSIG};
        std::vector<CMutableTransaction> noTxns;
        CreateAndProcessBlock(noTxns, script_pub_key);

        // Let the CoinStatsIndex to catch up again.
        BOOST_CHECK(coin_stats_index.BlockUntilSyncedToCurrentChain());

        CCoinsStats new_coin_stats{CoinStatsHashType::MUHASH};
        const CBlockIndex* new_block_index;
        {
            LOCK(cs_main);
            new_block_index = m_node.chainman->ActiveChain().Tip();
        }
        coin_stats_index.LookUpStats(new_block_index, new_coin_stats);

        BOOST_CHECK(block_index != new_block_index);

        // Shutdown sequence (c.f. Shutdown() in init.cpp)
        coin_stats_index.Stop();

        // Rest of shutdown sequence and destructors happen in ~TestingSetup()

    */
}
