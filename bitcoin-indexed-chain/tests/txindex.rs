// ---------------- [ File: bitcoin-indexed-chain/tests/txindex.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/txindex_tests.cpp]

#[cfg(test)]
pub mod txindex_tests {

    #[test] fn txindex_initial_sync_test_chain_100setup() {
        todo!();
        /*
        
            TxIndex txindex(1 << 20, true);

            CTransactionRef tx_disk;
            uint256 block_hash;

            // Transaction should not be found in the index before it is started.
            for (const auto& txn : m_coinbase_txns) {
                BOOST_CHECK(!txindex.FindTx(txn->GetHash(), block_hash, tx_disk));
            }

            // BlockUntilSyncedToCurrentChain should return false before txindex is started.
            BOOST_CHECK(!txindex.BlockUntilSyncedToCurrentChain());

            BOOST_REQUIRE(txindex.Start(m_node.chainman->ActiveChainstate()));

            // Allow tx index to catch up with the block index.
            constexpr int64_t timeout_ms = 10 * 1000;
            int64_t time_start = GetTimeMillis();
            while (!txindex.BlockUntilSyncedToCurrentChain()) {
                BOOST_REQUIRE(time_start + timeout_ms > GetTimeMillis());
                UninterruptibleSleep(std::chrono::milliseconds{100});
            }

            // Check that txindex excludes genesis block transactions.
            const CBlock& genesis_block = Params().GenesisBlock();
            for (const auto& txn : genesis_block.vtx) {
                BOOST_CHECK(!txindex.FindTx(txn->GetHash(), block_hash, tx_disk));
            }

            // Check that txindex has all txs that were in the chain before it started.
            for (const auto& txn : m_coinbase_txns) {
                if (!txindex.FindTx(txn->GetHash(), block_hash, tx_disk)) {
                    BOOST_ERROR("FindTx failed");
                } else if (tx_disk->GetHash() != txn->GetHash()) {
                    BOOST_ERROR("Read incorrect tx");
                }
            }

            // Check that new transactions in new blocks make it into the index.
            for (int i = 0; i < 10; i++) {
                CScript coinbase_script_pub_key = GetScriptForDestination(PKHash(coinbaseKey.GetPubKey()));
                std::vector<CMutableTransaction> no_txns;
                const CBlock& block = CreateAndProcessBlock(no_txns, coinbase_script_pub_key);
                const CTransaction& txn = *block.vtx[0];

                BOOST_CHECK(txindex.BlockUntilSyncedToCurrentChain());
                if (!txindex.FindTx(txn.GetHash(), block_hash, tx_disk)) {
                    BOOST_ERROR("FindTx failed");
                } else if (tx_disk->GetHash() != txn.GetHash()) {
                    BOOST_ERROR("Read incorrect tx");
                }
            }

            // shutdown sequence (c.f. Shutdown() in init.cpp)
            txindex.Stop();

            // Let scheduler events finish running to avoid accessing any memory related to txindex after it is destructed
            SyncWithValidationInterfaceQueue();

        */
    }
}
