// ---------------- [ File: bitcoinchain-params/src/test_net.rs ]
crate::ix!();

/**
  | Testnet (v3): public test network which
  | is reset from time to time.
  |
  */
pub struct TestNetParams {
    base: ChainParams,
}

impl Default for TestNetParams {
    
    fn default() -> Self {
        todo!();
        /*


            strNetworkID = CBaseChainParams::TESTNET;
            consensus.signet_blocks = false;
            consensus.signet_challenge.clear();
            consensus.nSubsidyHalvingInterval = 210000;
            consensus.BIP16Exception = uint256S("0x00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105");
            consensus.BIP34Height = 21111;
            consensus.BIP34Hash = uint256S("0x0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8");
            consensus.BIP65Height = 581885; // 00000000007f6655f22f98e72ed80d8b06dc761d5da09df0fa1dc4be4f861eb6
            consensus.BIP66Height = 330776; // 000000002104c8c45e99a8853285a3b592602a3ccde2b832481da85e9e4ba182
            consensus.CSVHeight = 770112; // 00000000025e930139bac5c6c31a403776da130831ab85be56578f3fa75369bb
            consensus.SegwitHeight = 834624; // 00000000002b980fcd729daaa248fd9316a5200e9b367f4ff2c42453e84201ca
            consensus.MinBIP9WarningHeight = 836640; // segwit activation height + miner confirmation window
            consensus.powLimit = uint256S("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
            consensus.nPowTargetTimespan = 14 * 24 * 60 * 60; // two weeks
            consensus.nPowTargetSpacing = 10 * 60;
            consensus.fPowAllowMinDifficultyBlocks = true;
            consensus.fPowNoRetargeting = false;
            consensus.nRuleChangeActivationThreshold = 1512; // 75% for testchains
            consensus.nMinerConfirmationWindow = 2016; // nPowTargetTimespan / nPowTargetSpacing
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].bit = 28;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].nStartTime = consensus::BIP9Deployment::NEVER_ACTIVE;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].nTimeout = consensus::BIP9Deployment::NO_TIMEOUT;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].min_activation_height = 0; // No activation delay

            // Deployment of Taproot (BIPs 340-342)
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].bit = 2;
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].nStartTime = 1619222400; // April 24th, 2021
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].nTimeout = 1628640000; // August 11th, 2021
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].min_activation_height = 0; // No activation delay

            consensus.nMinimumChainWork = uint256S("0x0000000000000000000000000000000000000000000005180c3bd8290da33a1a");
            consensus.defaultAssumeValid = uint256S("0x0000000000004ae2f3896ca8ecd41c460a35bf6184e145d91558cece1c688a76"); // 2010000

            pchMessageStart[0] = 0x0b;
            pchMessageStart[1] = 0x11;
            pchMessageStart[2] = 0x09;
            pchMessageStart[3] = 0x07;
            nDefaultPort = 18333;
            nPruneAfterHeight = 1000;
            m_assumed_blockchain_size = 40;
            m_assumed_chain_state_size = 2;

            genesis = CreateGenesisBlock(1296688602, 414098458, 0x1d00ffff, 1, 50 * COIN);
            consensus.hashGenesisBlock = genesis.GetHash();
            assert(consensus.hashGenesisBlock == uint256S("0x000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943"));
            assert(genesis.hashMerkleRoot == uint256S("0x4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"));

            vFixedSeeds.clear();
            vSeeds.clear();
            // nodes with support for servicebits filtering should be at the top
            vSeeds.emplace_back("testnet-seed.bitcoin.jonasschnelli.ch.");
            vSeeds.emplace_back("seed.tbtc.petertodd.org.");
            vSeeds.emplace_back("seed.testnet.bitcoin.sprovoost.nl.");
            vSeeds.emplace_back("testnet-seed.bluematt.me."); // Just a static list of stable node(s), only supports x9

            base58Prefixes[PUBKEY_ADDRESS] = std::vector<unsigned char>(1,111);
            base58Prefixes[SCRIPT_ADDRESS] = std::vector<unsigned char>(1,196);
            base58Prefixes[SECRET_KEY] =     std::vector<unsigned char>(1,239);
            base58Prefixes[EXT_PUBLIC_KEY] = {0x04, 0x35, 0x87, 0xCF};
            base58Prefixes[EXT_SECRET_KEY] = {0x04, 0x35, 0x83, 0x94};

            bech32_hrp = "tb";

            vFixedSeeds = std::vector<uint8_t>(std::begin(chainparams_seed_test), std::end(chainparams_seed_test));

            fDefaultConsistencyChecks = false;
            fRequireStandard = false;
            m_is_test_chain = true;
            m_is_mockable_chain = false;

            checkpointData = {
                {
                    {546, uint256S("000000002a936ca763904c3c35fce2f3556c559c0214345d31b1bcebf76acb70")},
                }
            };

            m_assumeutxo_data = MapAssumeUtxo{
                // TODO to be specified in a future patch.
            };

            chainTxData = ChainTxData{
                // Data from RPC: getchaintxstats 4096 0000000000004ae2f3896ca8ecd41c460a35bf6184e145d91558cece1c688a76
                /* nTime    */ 1625727096,
                /* nTxCount */ 60408943,
                /* dTxRate  */ 0.08379062270367649,
            };
        */
    }
}
