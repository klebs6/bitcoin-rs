// ---------------- [ File: bitcoin-validation/tests/validation_block.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/validation_block_tests.cpp]

pub mod validation_block_tests {
    use super::*;

    pub struct MinerTestingSetup {
        base: RegTestingSetup,
    }

    impl MinerTestingSetup {
        
        pub fn block(&mut self, prev_hash: &u256) -> Arc<Block> {
            
            todo!();
            /*
            
            */
        }
        
        pub fn good_block(&mut self, prev_hash: &u256) -> Arc<Block> {
            
            todo!();
            /*
            
            */
        }
        
        pub fn bad_block(&mut self, prev_hash: &u256) -> Arc<Block> {
            
            todo!();
            /*
            
            */
        }
        
        pub fn finalize_block(&mut self, pblock: Arc<Block>) -> Arc<Block> {
            
            todo!();
            /*
            
            */
        }
        
        pub fn build_chain(&mut self, 
            root:         &u256,
            height:       i32,
            invalid_rate: u32,
            branch_rate:  u32,
            max_size:     u32,
            blocks:       &mut Vec<Arc<Block>>)  {
            
            todo!();
            /*
            
            */
        }
    }
}

#[cfg(test)]
#[MinerTestingSetup]
pub mod validation_block_tests {

    pub struct TestSubscriber {
        base:         ValidationInterface,
        expected_tip: u256,
    }

    impl TestSubscriber {
        
        pub fn new(tip: u256) -> Self {
        
            todo!();
            /*
            : expected_tip(tip),

            
            */
        }
        
        pub fn updated_block_tip(&mut self, 
            pindex_new:       *const BlockIndex,
            pindex_fork:      *const BlockIndex,
            initial_download: bool)  {
            
            todo!();
            /*
                BOOST_CHECK_EQUAL(m_expected_tip, pindexNew->GetBlockHash());
            */
        }
        
        pub fn block_connected(&mut self, 
            block:  &Arc<Block>,
            pindex: *const BlockIndex)  {
            
            todo!();
            /*
                BOOST_CHECK_EQUAL(m_expected_tip, block->hashPrevBlock);
                BOOST_CHECK_EQUAL(m_expected_tip, pindex->pprev->GetBlockHash());

                m_expected_tip = block->GetHash();
            */
        }
        
        pub fn block_disconnected(&mut self, 
            block:  &Arc<Block>,
            pindex: *const BlockIndex)  {
            
            todo!();
            /*
                BOOST_CHECK_EQUAL(m_expected_tip, block->GetHash());
                BOOST_CHECK_EQUAL(m_expected_tip, pindex->GetBlockHash());

                m_expected_tip = block->hashPrevBlock;
            */
        }
    }

    ///-------------------------
    impl MinerTestingSetup {

        pub fn block(&mut self, prev_hash: &u256) -> Arc<Block> {
            
            todo!();
            /*
                static int i = 0;
            static uint64_t time = Params().GenesisBlock().nTime;

            auto ptemplate = BlockAssembler(m_node.chainman->ActiveChainstate(), *m_node.mempool, Params()).CreateNewBlock(CScript{} << i++ << OP_TRUE);
            auto pblock = std::make_shared<CBlock>(ptemplate->block);
            pblock->hashPrevBlock = prev_hash;
            pblock->nTime = ++time;

            // Make the coinbase transaction with two outputs:
            // One zero-value one that has a unique pubkey to make sure that blocks at the same height can have a different hash
            // Another one that has the coinbase reward in a P2WSH with OP_TRUE as witness program to make it easy to spend
            CMutableTransaction txCoinbase(*pblock->vtx[0]);
            txCoinbase.vout.resize(2);
            txCoinbase.vout[1].scriptPubKey = P2WSH_OP_TRUE;
            txCoinbase.vout[1].nValue = txCoinbase.vout[0].nValue;
            txCoinbase.vout[0].nValue = 0;
            txCoinbase.vin[0].scriptWitness.SetNull();
            // Always pad with OP_0 at the end to avoid bad-cb-length error
            txCoinbase.vin[0].scriptSig = CScript{} << 
            [&]() { LOCK(::cs_main);  return m_node.chainman->m_blockman.LookupBlockIndex(prev_hash)->nHeight + 1 }()
            << OP_0;
            pblock->vtx[0] = MakeTransactionRef(std::move(txCoinbase));

            return pblock;
            */
        }
        
        pub fn finalize_block(&mut self, pblock: Arc<Block>) -> Arc<Block> {
            
            todo!();
            /*
                const CBlockIndex* prev_block{
            [&]() { LOCK(::cs_main);  return m_node.chainman->m_blockman.LookupBlockIndex(pblock->hashPrevBlock) }()
            };
            GenerateCoinbaseCommitment(*pblock, prev_block, Params().GetConsensus());

            pblock->hashMerkleRoot = BlockMerkleRoot(*pblock);

            while (!CheckProofOfWork(pblock->GetHash(), pblock->nBits, Params().GetConsensus())) {
                ++(pblock->nNonce);
            }

            // submit block header, so that miner can get the block height from the
            // global state and the node has the topology of the chain
            BlockValidationState ignored;
            BOOST_CHECK(Assert(m_node.chainman)->ProcessNewBlockHeaders({pblock->GetBlockHeader()}, ignored, Params()));

            return pblock;
            */
        }

        /**
          | construct a valid block
          |
          */
        pub fn good_block(&mut self, prev_hash: &u256) -> Arc<Block> {
            
            todo!();
            /*
                return FinalizeBlock(Block(prev_hash));
            */
        }

        /**
          | construct an invalid block (but with
          | a valid header)
          |
          */
        pub fn bad_block(&mut self, prev_hash: &u256) -> Arc<Block> {
            
            todo!();
            /*
                auto pblock = Block(prev_hash);

            CMutableTransaction coinbase_spend;
            coinbase_spend.vin.push_back(CTxIn(OutPoint(pblock->vtx[0]->GetHash(), 0), CScript(), 0));
            coinbase_spend.vout.push_back(pblock->vtx[0]->vout[0]);

            CTransactionRef tx = MakeTransactionRef(coinbase_spend);
            pblock->vtx.push_back(tx);

            auto ret = FinalizeBlock(pblock);
            return ret;
            */
        }
        
        pub fn build_chain(&mut self, 
            root:         &u256,
            height:       i32,
            invalid_rate: u32,
            branch_rate:  u32,
            max_size:     u32,
            blocks:       &mut Vec<Arc<Block>>)  {
            
            todo!();
            /*
                if (height <= 0 || blocks.size() >= max_size) return;

            bool gen_invalid = InsecureRandRange(100) < invalid_rate;
            bool gen_fork = InsecureRandRange(100) < branch_rate;

            const std::shared_ptr<const CBlock> pblock = gen_invalid ? BadBlock(root) : GoodBlock(root);
            blocks.push_back(pblock);
            if (!gen_invalid) {
                BuildChain(pblock->GetHash(), height - 1, invalid_rate, branch_rate, max_size, blocks);
            }

            if (gen_fork) {
                blocks.push_back(GoodBlock(root));
                BuildChain(blocks.back()->GetHash(), height - 1, invalid_rate, branch_rate, max_size, blocks);
            }
            */
        }
    }

    #[test] fn processnewblock_signals_ordering() {
        todo!();
        /*
        
            // build a large-ish chain that's likely to have some forks
            std::vector<std::shared_ptr<const CBlock>> blocks;
            while (blocks.size() < 50) {
                blocks.clear();
                BuildChain(Params().GenesisBlock().GetHash(), 100, 15, 10, 500, blocks);
            }

            bool ignored;
            // Connect the genesis block and drain any outstanding events
            BOOST_CHECK(Assert(m_node.chainman)->ProcessNewBlock(Params(), std::make_shared<CBlock>(Params().GenesisBlock()), true, &ignored));
            SyncWithValidationInterfaceQueue();

            // subscribe to events (this subscriber will validate event ordering)
            const CBlockIndex* initial_tip = nullptr;
            {
                LOCK(cs_main);
                initial_tip = m_node.chainman->ActiveChain().Tip();
            }
            auto sub = std::make_shared<TestSubscriber>(initial_tip->GetBlockHash());
            RegisterSharedValidationInterface(sub);

            // create a bunch of threads that repeatedly process a block generated above at random
            // this will create parallelism and randomness inside validation - the ValidationInterface
            // will subscribe to events generated during block validation and assert on ordering invariance
            std::vector<std::thread> threads;
            for (int i = 0; i < 10; i++) {
                threads.emplace_back([&]() {
                    bool ignored;
                    FastRandomContext insecure;
                    for (int i = 0; i < 1000; i++) {
                        auto block = blocks[insecure.randrange(blocks.size() - 1)];
                        Assert(m_node.chainman)->ProcessNewBlock(Params(), block, true, &ignored);
                    }

                    // to make sure that eventually we process the full chain - do it here
                    for (auto block : blocks) {
                        if (block->vtx.size() == 1) {
                            bool processed = Assert(m_node.chainman)->ProcessNewBlock(Params(), block, true, &ignored);
                            assert(processed);
                        }
                    }
                });
            }

            for (auto& t : threads) {
                t.join();
            }
            SyncWithValidationInterfaceQueue();

            UnregisterSharedValidationInterface(sub);

            LOCK(cs_main);
            BOOST_CHECK_EQUAL(sub->m_expected_tip, m_node.chainman->ActiveChain().Tip()->GetBlockHash());

        */
    }

    /**
      | Test that mempool updates happen atomically
      | with reorgs.
      | 
      | This prevents RPC clients, among others,
      | from retrieving immediately-out-of-date
      | mempool data during large reorgs.
      | 
      | The test verifies this by creating a
      | chain of `num_txs` blocks, matures
      | their coinbases, and then submits txns
      | spending from their coinbase to the
      | mempool. A fork chain is then processed,
      | invalidating the txns and evicting
      | them from the mempool.
      | 
      | We verify that the mempool updates atomically
      | by polling it continuously from another
      | thread during the reorg and checking
      | that its size only changes once. The
      | size changing exactly once indicates
      | that the polling thread's view of the
      | mempool is either consistent with the
      | chain state before reorg, or consistent
      | with the chain state after the reorg,
      | and not just consistent with some intermediate
      | state during the reorg.
      |
      */
    #[test] fn mempool_locks_reorg() {
        todo!();
        /*
        
            bool ignored;
            auto ProcessBlock = [&](std::shared_ptr<const CBlock> block) -> bool {
                return Assert(m_node.chainman)->ProcessNewBlock(Params(), block, /* fForceProcessing */ true, /* fNewBlock */ &ignored);
            };

            // Process all mined blocks
            BOOST_REQUIRE(ProcessBlock(std::make_shared<CBlock>(Params().GenesisBlock())));
            auto last_mined = GoodBlock(Params().GenesisBlock().GetHash());
            BOOST_REQUIRE(ProcessBlock(last_mined));

            // Run the test multiple times
            for (int test_runs = 3; test_runs > 0; --test_runs) {
                BOOST_CHECK_EQUAL(last_mined->GetHash(), m_node.chainman->ActiveChain().Tip()->GetBlockHash());

                // Later on split from here
                const uint256 split_hash{last_mined->hashPrevBlock};

                // Create a bunch of transactions to spend the miner rewards of the
                // most recent blocks
                std::vector<CTransactionRef> txs;
                for (int num_txs = 22; num_txs > 0; --num_txs) {
                    CMutableTransaction mtx;
                    mtx.vin.push_back(CTxIn{OutPoint{last_mined->vtx[0]->GetHash(), 1}, CScript{}});
                    mtx.vin[0].scriptWitness.stack.push_back(WITNESS_STACK_ELEM_OP_TRUE);
                    mtx.vout.push_back(last_mined->vtx[0]->vout[1]);
                    mtx.vout[0].nValue -= 1000;
                    txs.push_back(MakeTransactionRef(mtx));

                    last_mined = GoodBlock(last_mined->GetHash());
                    BOOST_REQUIRE(ProcessBlock(last_mined));
                }

                // Mature the inputs of the txs
                for (int j = COINBASE_MATURITY; j > 0; --j) {
                    last_mined = GoodBlock(last_mined->GetHash());
                    BOOST_REQUIRE(ProcessBlock(last_mined));
                }

                // Mine a reorg (and hold it back) before adding the txs to the mempool
                const uint256 tip_init{last_mined->GetHash()};

                std::vector<std::shared_ptr<const CBlock>> reorg;
                last_mined = GoodBlock(split_hash);
                reorg.push_back(last_mined);
                for (size_t j = COINBASE_MATURITY + txs.size() + 1; j > 0; --j) {
                    last_mined = GoodBlock(last_mined->GetHash());
                    reorg.push_back(last_mined);
                }

                // Add the txs to the tx pool
                {
                    LOCK(cs_main);
                    for (const auto& tx : txs) {
                        const MempoolAcceptResult result = AcceptToMemoryPool(m_node.chainman->ActiveChainstate(), *m_node.mempool, tx, false /* bypass_limits */);
                        BOOST_REQUIRE(result.m_result_type == MempoolAcceptResult::ResultType::VALID);
                    }
                }

                // Check that all txs are in the pool
                {
                    LOCK(m_node.mempool->cs);
                    BOOST_CHECK_EQUAL(m_node.mempool->mapTx.size(), txs.size());
                }

                // Run a thread that simulates an RPC caller that is polling while
                // validation is doing a reorg
                std::thread rpc_thread{[&]() {
                    // This thread is checking that the mempool either contains all of
                    // the transactions invalidated by the reorg, or none of them, and
                    // not some intermediate amount.
                    while (true) {
                        LOCK(m_node.mempool->cs);
                        if (m_node.mempool->mapTx.size() == 0) {
                            // We are done with the reorg
                            break;
                        }
                        // Internally, we might be in the middle of the reorg, but
                        // externally the reorg to the most-proof-of-work chain should
                        // be atomic. So the caller assumes that the returned mempool
                        // is consistent. That is, it has all txs that were there
                        // before the reorg.
                        assert(m_node.mempool->mapTx.size() == txs.size());
                        continue;
                    }
                    LOCK(cs_main);
                    // We are done with the reorg, so the tip must have changed
                    assert(tip_init != m_node.chainman->ActiveChain().Tip()->GetBlockHash());
                }};

                // Submit the reorg in this thread to invalidate and remove the txs from the tx pool
                for (const auto& b : reorg) {
                    ProcessBlock(b);
                }
                // Check that the reorg was eventually successful
                BOOST_CHECK_EQUAL(last_mined->GetHash(), m_node.chainman->ActiveChain().Tip()->GetBlockHash());

                // We can join the other thread, which returns when the reorg was successful
                rpc_thread.join();
            }

        */
    }

    #[test] fn witness_commitment_index() {
        todo!();
        /*
        
            CScript pubKey;
            pubKey << 1 << OP_TRUE;
            auto ptemplate = BlockAssembler(m_node.chainman->ActiveChainstate(), *m_node.mempool, Params()).CreateNewBlock(pubKey);
            CBlock pblock = ptemplate->block;

            CTxOut witness;
            witness.scriptPubKey.resize(MINIMUM_WITNESS_COMMITMENT);
            witness.scriptPubKey[0] = OP_RETURN;
            witness.scriptPubKey[1] = 0x24;
            witness.scriptPubKey[2] = 0xaa;
            witness.scriptPubKey[3] = 0x21;
            witness.scriptPubKey[4] = 0xa9;
            witness.scriptPubKey[5] = 0xed;

            // A witness larger than the minimum size is still valid
            CTxOut min_plus_one = witness;
            min_plus_one.scriptPubKey.resize(MINIMUM_WITNESS_COMMITMENT + 1);

            CTxOut invalid = witness;
            invalid.scriptPubKey[0] = OP_VERIFY;

            CMutableTransaction txCoinbase(*pblock.vtx[0]);
            txCoinbase.vout.resize(4);
            txCoinbase.vout[0] = witness;
            txCoinbase.vout[1] = witness;
            txCoinbase.vout[2] = min_plus_one;
            txCoinbase.vout[3] = invalid;
            pblock.vtx[0] = MakeTransactionRef(std::move(txCoinbase));

            BOOST_CHECK_EQUAL(GetWitnessCommitmentIndex(pblock), 2);

        */
    }
}
