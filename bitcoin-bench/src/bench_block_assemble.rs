crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/block_assemble.cpp]

#[bench]
fn assemble_block(b: &mut Bencher)  {
    
    todo!();
        /*
        const auto test_setup = MakeNoLogFileContext<const TestingSetup>();

        CScriptWitness witness;
        witness.stack.push_back(WITNESS_STACK_ELEM_OP_TRUE);

        // Collect some loose transactions that spend the coinbases of our mined blocks
        constexpr size_t NUM_BLOCKS{200};
        std::array<CTransactionRef, NUM_BLOCKS - COINBASE_MATURITY + 1> txs;
        for (size_t b{0}; b < NUM_BLOCKS; ++b) {
            CMutableTransaction tx;
            tx.vin.push_back(MineBlock(test_setup->m_node, P2WSH_OP_TRUE));
            tx.vin.back().scriptWitness = witness;
            tx.vout.emplace_back(1337, P2WSH_OP_TRUE);
            if (NUM_BLOCKS - b >= COINBASE_MATURITY)
                txs.at(b) = MakeTransactionRef(tx);
        }
        {
            LOCK(::cs_main); // Required for ::AcceptToMemoryPool.

            for (const auto& txr : txs) {
                const MempoolAcceptResult res = ::AcceptToMemoryPool(test_setup->m_node.chainman->ActiveChainstate(), *test_setup->m_node.mempool, txr, false /* bypass_limits */);
                assert(res.m_result_type == MempoolAcceptResult::ResultType::VALID);
            }
        }

        bench.run([&] {
            PrepareBlock(test_setup->m_node, P2WSH_OP_TRUE);
        });
        */
}
