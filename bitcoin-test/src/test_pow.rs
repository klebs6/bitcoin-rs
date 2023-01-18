crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/pow_tests.cpp]

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod pow_tests {

    /* Test calculation of next difficulty target with no constraints applying */
    #[test] fn get_next_work() {
        todo!();
        /*
        
            const auto chainParams = CreateChainParams(*m_node.args, CBaseChainParams::MAIN);
            int64_t nLastRetargetTime = 1261130161; // Block #30240
            CBlockIndex pindexLast;
            pindexLast.nHeight = 32255;
            pindexLast.nTime = 1262152739;  // Block #32255
            pindexLast.nBits = 0x1d00ffff;
            BOOST_CHECK_EQUAL(CalculateNextWorkRequired(&pindexLast, nLastRetargetTime, chainParams->GetConsensus()), 0x1d00d86aU);

        */
    }

    /**
      | Test the constraint on the upper bound
      | for next work
      |
      */
    #[test] fn get_next_work_pow_limit() {
        todo!();
        /*
        
            const auto chainParams = CreateChainParams(*m_node.args, CBaseChainParams::MAIN);
            int64_t nLastRetargetTime = 1231006505; // Block #0
            CBlockIndex pindexLast;
            pindexLast.nHeight = 2015;
            pindexLast.nTime = 1233061996;  // Block #2015
            pindexLast.nBits = 0x1d00ffff;
            BOOST_CHECK_EQUAL(CalculateNextWorkRequired(&pindexLast, nLastRetargetTime, chainParams->GetConsensus()), 0x1d00ffffU);

        */
    }

    /**
      | Test the constraint on the lower bound
      | for actual time taken
      |
      */
    #[test] fn get_next_work_lower_limit_actual() {
        todo!();
        /*
        
            const auto chainParams = CreateChainParams(*m_node.args, CBaseChainParams::MAIN);
            int64_t nLastRetargetTime = 1279008237; // Block #66528
            CBlockIndex pindexLast;
            pindexLast.nHeight = 68543;
            pindexLast.nTime = 1279297671;  // Block #68543
            pindexLast.nBits = 0x1c05a3f4;
            BOOST_CHECK_EQUAL(CalculateNextWorkRequired(&pindexLast, nLastRetargetTime, chainParams->GetConsensus()), 0x1c0168fdU);

        */
    }

    /**
      | Test the constraint on the upper bound
      | for actual time taken
      |
      */
    #[test] fn get_next_work_upper_limit_actual() {
        todo!();
        /*
        
            const auto chainParams = CreateChainParams(*m_node.args, CBaseChainParams::MAIN);
            int64_t nLastRetargetTime = 1263163443; // NOTE: Not an actual block time
            CBlockIndex pindexLast;
            pindexLast.nHeight = 46367;
            pindexLast.nTime = 1269211443;  // Block #46367
            pindexLast.nBits = 0x1c387f6f;
            BOOST_CHECK_EQUAL(CalculateNextWorkRequired(&pindexLast, nLastRetargetTime, chainParams->GetConsensus()), 0x1d00e1fdU);

        */
    }

    #[test] fn check_proof_of_work_test_negative_target() {
        todo!();
        /*
        
            const auto consensus = CreateChainParams(*m_node.args, CBaseChainParams::MAIN)->GetConsensus();
            uint256 hash;
            unsigned int nBits;
            nBits = UintToArith256(consensus.powLimit).GetCompact(true);
            hash.SetHex("0x1");
            BOOST_CHECK(!CheckProofOfWork(hash, nBits, consensus));

        */
    }

    #[test] fn check_proof_of_work_test_overflow_target() {
        todo!();
        /*
        
            const auto consensus = CreateChainParams(*m_node.args, CBaseChainParams::MAIN)->GetConsensus();
            uint256 hash;
            unsigned int nBits = ~0x00800000;
            hash.SetHex("0x1");
            BOOST_CHECK(!CheckProofOfWork(hash, nBits, consensus));

        */
    }

    #[test] fn check_proof_of_work_test_too_easy_target() {
        todo!();
        /*
        
            const auto consensus = CreateChainParams(*m_node.args, CBaseChainParams::MAIN)->GetConsensus();
            uint256 hash;
            unsigned int nBits;
            arith_uint256 nBits_arith = UintToArith256(consensus.powLimit);
            nBits_arith *= 2;
            nBits = nBits_arith.GetCompact();
            hash.SetHex("0x1");
            BOOST_CHECK(!CheckProofOfWork(hash, nBits, consensus));

        */
    }

    #[test] fn check_proof_of_work_test_biger_hash_than_target() {
        todo!();
        /*
        
            const auto consensus = CreateChainParams(*m_node.args, CBaseChainParams::MAIN)->GetConsensus();
            uint256 hash;
            unsigned int nBits;
            arith_uint256 hash_arith = UintToArith256(consensus.powLimit);
            nBits = hash_arith.GetCompact();
            hash_arith *= 2; // hash > nBits
            hash = ArithToUint256(hash_arith);
            BOOST_CHECK(!CheckProofOfWork(hash, nBits, consensus));

        */
    }

    #[test] fn check_proof_of_work_test_zero_target() {
        todo!();
        /*
        
            const auto consensus = CreateChainParams(*m_node.args, CBaseChainParams::MAIN)->GetConsensus();
            uint256 hash;
            unsigned int nBits;
            arith_uint256 hash_arith{0};
            nBits = hash_arith.GetCompact();
            hash = ArithToUint256(hash_arith);
            BOOST_CHECK(!CheckProofOfWork(hash, nBits, consensus));

        */
    }

    #[test] fn get_block_proof_equivalent_time_test() {
        todo!();
        /*
        
            const auto chainParams = CreateChainParams(*m_node.args, CBaseChainParams::MAIN);
            std::vector<CBlockIndex> blocks(10000);
            for (int i = 0; i < 10000; i++) {
                blocks[i].pprev = i ? &blocks[i - 1] : nullptr;
                blocks[i].nHeight = i;
                blocks[i].nTime = 1269211443 + i * chainParams->GetConsensus().nPowTargetSpacing;
                blocks[i].nBits = 0x207fffff; /* target 0x7fffff000... */
                blocks[i].nChainWork = i ? blocks[i - 1].nChainWork + GetBlockProof(blocks[i - 1]) : arith_uint256(0);
            }

            for (int j = 0; j < 1000; j++) {
                CBlockIndex *p1 = &blocks[InsecureRandRange(10000)];
                CBlockIndex *p2 = &blocks[InsecureRandRange(10000)];
                CBlockIndex *p3 = &blocks[InsecureRandRange(10000)];

                int64_t tdiff = GetBlockProofEquivalentTime(*p1, *p2, *p3, chainParams->GetConsensus());
                BOOST_CHECK_EQUAL(tdiff, p1->GetBlockTime() - p2->GetBlockTime());
            }

        */
    }

    pub fn sanity_check_chainparams(
            args:       &ArgsManager,
            chain_name: String)  {
        
        todo!();
            /*
                const auto chainParams = CreateChainParams(args, chainName);
                const auto consensus = chainParams->GetConsensus();

                // hash genesis is correct
                BOOST_CHECK_EQUAL(consensus.hashGenesisBlock, chainParams->GenesisBlock().GetHash());

                // target timespan is an even multiple of spacing
                BOOST_CHECK_EQUAL(consensus.nPowTargetTimespan % consensus.nPowTargetSpacing, 0);

                // genesis nBits is positive, doesn't overflow and is lower than powLimit
                arith_uint256 pow_compact;
                bool neg, over;
                pow_compact.SetCompact(chainParams->GenesisBlock().nBits, &neg, &over);
                BOOST_CHECK(!neg && pow_compact != 0);
                BOOST_CHECK(!over);
                BOOST_CHECK(UintToArith256(consensus.powLimit) >= pow_compact);

                // check max target * 4*nPowTargetTimespan doesn't overflow -- see pow.cpp:CalculateNextWorkRequired()
                if (!consensus.fPowNoRetargeting) {
                    arith_uint256 targ_max("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
                    targ_max /= consensus.nPowTargetTimespan*4;
                    BOOST_CHECK(UintToArith256(consensus.powLimit) < targ_max);
                }
            */
    }

    #[test] fn chain_params_main_sanity() {
        todo!();
        /*
        
            sanity_check_chainparams(*m_node.args, CBaseChainParams::MAIN);

        */
    }

    #[test] fn chain_params_regtest_sanity() {
        todo!();
        /*
        
            sanity_check_chainparams(*m_node.args, CBaseChainParams::REGTEST);

        */
    }

    #[test] fn chain_params_testnet_sanity() {
        todo!();
        /*
        
            sanity_check_chainparams(*m_node.args, CBaseChainParams::TESTNET);

        */
    }

    #[test] fn chain_params_signet_sanity() {
        todo!();
        /*
        
            sanity_check_chainparams(*m_node.args, CBaseChainParams::SIGNET);

        */
    }
}
