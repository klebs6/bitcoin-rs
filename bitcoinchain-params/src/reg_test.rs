// ---------------- [ File: bitcoinchain-params/src/reg_test.rs ]
crate::ix!();

/**
  | Regression test: intended for private
  | networks only. Has minimal difficulty
  | to ensure that blocks can be found instantly.
  |
  */
pub struct RegTestParams {
    base: ChainParams,
}

impl RegTestParams {

    pub fn new(args: &ArgsManager) -> Self {
    
        todo!();
        /*


            strNetworkID =  CBaseChainParams::REGTEST;
            consensus.signet_blocks = false;
            consensus.signet_challenge.clear();
            consensus.nSubsidyHalvingInterval = 150;
            consensus.BIP16Exception = uint256();
            consensus.BIP34Height = 1; // Always active unless overridden
            consensus.BIP34Hash = uint256();
            consensus.BIP65Height = 1;  // Always active unless overridden
            consensus.BIP66Height = 1;  // Always active unless overridden
            consensus.CSVHeight = 1;    // Always active unless overridden
            consensus.SegwitHeight = 1; // Always active unless overridden
            consensus.MinBIP9WarningHeight = 0;
            consensus.powLimit = uint256S("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
            consensus.nPowTargetTimespan = 14 * 24 * 60 * 60; // two weeks
            consensus.nPowTargetSpacing = 10 * 60;
            consensus.fPowAllowMinDifficultyBlocks = true;
            consensus.fPowNoRetargeting = true;
            consensus.nRuleChangeActivationThreshold = 108; // 75% for testchains
            consensus.nMinerConfirmationWindow = 144; // Faster than normal for regtest (144 instead of 2016)

            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].bit = 28;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].nStartTime = 0;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].nTimeout = consensus::BIP9Deployment::NO_TIMEOUT;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].min_activation_height = 0; // No activation delay

            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].bit = 2;
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].nStartTime = consensus::BIP9Deployment::ALWAYS_ACTIVE;
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].nTimeout = consensus::BIP9Deployment::NO_TIMEOUT;
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].min_activation_height = 0; // No activation delay

            consensus.nMinimumChainWork = uint256{};
            consensus.defaultAssumeValid = uint256{};

            pchMessageStart[0] = 0xfa;
            pchMessageStart[1] = 0xbf;
            pchMessageStart[2] = 0xb5;
            pchMessageStart[3] = 0xda;
            nDefaultPort = 18444;
            nPruneAfterHeight = args.GetBoolArg("-fastprune", false) ? 100 : 1000;
            m_assumed_blockchain_size = 0;
            m_assumed_chain_state_size = 0;

            UpdateActivationParametersFromArgs(args);

            genesis = CreateGenesisBlock(1296688602, 2, 0x207fffff, 1, 50 * COIN);
            consensus.hashGenesisBlock = genesis.GetHash();
            assert(consensus.hashGenesisBlock == uint256S("0x0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206"));
            assert(genesis.hashMerkleRoot == uint256S("0x4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"));

            vFixedSeeds.clear(); ///Regtest mode doesn't have any fixed seeds.
            vSeeds.clear();
            vSeeds.emplace_back("dummySeed.invalid.");

            fDefaultConsistencyChecks = true;
            fRequireStandard = true;
            m_is_test_chain = true;
            m_is_mockable_chain = true;

            checkpointData = {
                {
                    {0, uint256S("0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206")},
                }
            };

            m_assumeutxo_data = MapAssumeUtxo{
                {
                    110,
                    {AssumeUtxoHash{uint256S("0x1ebbf5850204c0bdb15bf030f47c7fe91d45c44c712697e4509ba67adb01c618")}, 110},
                },
                {
                    200,
                    {AssumeUtxoHash{uint256S("0x51c8d11d8b5c1de51543c579736e786aa2736206d1e11e627568029ce092cf62")}, 200},
                },
            };

            chainTxData = ChainTxData{
                0,
                0,
                0
            };

            base58Prefixes[PUBKEY_ADDRESS] = std::vector<unsigned char>(1,111);
            base58Prefixes[SCRIPT_ADDRESS] = std::vector<unsigned char>(1,196);
            base58Prefixes[SECRET_KEY] =     std::vector<unsigned char>(1,239);
            base58Prefixes[EXT_PUBLIC_KEY] = {0x04, 0x35, 0x87, 0xCF};
            base58Prefixes[EXT_SECRET_KEY] = {0x04, 0x35, 0x83, 0x94};

            bech32_hrp = "bcrt";
        */
    }

    /**
      | Allows modifying the Version Bits regtest
      | parameters.
      |
      */
    pub fn update_version_bits_parameters(&mut self, 
        d:                     ConsensusDeploymentPos,
        n_start_time:          i64,
        n_timeout:             i64,
        min_activation_height: i32)  {
        
        todo!();
        /*
            consensus.vDeployments[d].nStartTime = nStartTime;
            consensus.vDeployments[d].nTimeout = nTimeout;
            consensus.vDeployments[d].min_activation_height = min_activation_height;
        */
    }
    
    pub fn update_activation_parameters_from_args(&mut self, args: &ArgsManager)  {
        
        todo!();
        /*
            MaybeUpdateHeights(args, consensus);

        if (!args.IsArgSet("-vbparams")) return;

        for (const std::string& strDeployment : args.GetArgs("-vbparams")) {
            std::vector<std::string> vDeploymentParams;
            boost::split(vDeploymentParams, strDeployment, boost::is_any_of(":"));
            if (vDeploymentParams.size() < 3 || 4 < vDeploymentParams.size()) {
                throw std::runtime_error("Version bits parameters malformed, expecting deployment:start:end[:min_activation_height]");
            }
            int64_t nStartTime, nTimeout;
            int min_activation_height = 0;
            if (!ParseInt64(vDeploymentParams[1], &nStartTime)) {
                throw std::runtime_error(strprintf("Invalid nStartTime (%s)", vDeploymentParams[1]));
            }
            if (!ParseInt64(vDeploymentParams[2], &nTimeout)) {
                throw std::runtime_error(strprintf("Invalid nTimeout (%s)", vDeploymentParams[2]));
            }
            if (vDeploymentParams.size() >= 4 && !ParseInt32(vDeploymentParams[3], &min_activation_height)) {
                throw std::runtime_error(strprintf("Invalid min_activation_height (%s)", vDeploymentParams[3]));
            }
            bool found = false;
            for (int j=0; j < (int)consensus::MAX_VERSION_BITS_DEPLOYMENTS; ++j) {
                if (vDeploymentParams[0] == VersionBitsDeploymentInfo[j].name) {
                    UpdateVersionBitsParameters(ConsensusDeploymentPos(j), nStartTime, nTimeout, min_activation_height);
                    found = true;
                    LogPrintf("Setting version bits activation parameters for %s to start=%ld, timeout=%ld, min_activation_height=%d\n", vDeploymentParams[0], nStartTime, nTimeout, min_activation_height);
                    break;
                }
            }
            if (!found) {
                throw std::runtime_error(strprintf("Invalid deployment (%s)", vDeploymentParams[0]));
            }
        }
        */
    }
}
