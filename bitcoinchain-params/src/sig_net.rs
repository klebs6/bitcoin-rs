crate::ix!();

/**
  | Signet: test network with an additional
  | consensus parameter (see BIP325).
  |
  */
pub struct SigNetParams {
    base: ChainParams,
}

impl SigNetParams {

    pub fn new(args: &ArgsManager) -> Self {
    
        todo!();
        /*


            std::vector<uint8_t> bin;
            vSeeds.clear();

            if (!args.IsArgSet("-signetchallenge")) {
                bin = ParseHex("512103ad5e0edad18cb1f0fc0d28a3d4f1f3e445640337489abb10404f2d1e086be430210359ef5021964fe22d6f8e05b2463c9540ce96883fe3b278760f048f5189f2e6c452ae");
                vSeeds.emplace_back("seed.signet.bitcoin.sprovoost.nl.");

                // Hardcoded nodes can be removed once there are more DNS seeds
                vSeeds.emplace_back("178.128.221.177");
                vSeeds.emplace_back("v7ajjeirttkbnt32wpy3c6w3emwnfr3fkla7hpxcfokr3ysd3kqtzmqd.onion:38333");

                consensus.nMinimumChainWork = uint256S("0x0000000000000000000000000000000000000000000000000000008546553c03");
                consensus.defaultAssumeValid = uint256S("0x000000187d4440e5bff91488b700a140441e089a8aaea707414982460edbfe54"); // 47200
                m_assumed_blockchain_size = 1;
                m_assumed_chain_state_size = 0;
                chainTxData = ChainTxData{
                    // Data from RPC: getchaintxstats 4096 000000187d4440e5bff91488b700a140441e089a8aaea707414982460edbfe54
                    /* nTime    */ 1626696658,
                    /* nTxCount */ 387761,
                    /* dTxRate  */ 0.04035946932424404,
                };
            } else {
                const auto signet_challenge = args.GetArgs("-signetchallenge");
                if (signet_challenge.size() != 1) {
                    throw std::runtime_error(strprintf("%s: -signetchallenge cannot be multiple values.", __func__));
                }
                bin = ParseHex(signet_challenge[0]);

                consensus.nMinimumChainWork = uint256{};
                consensus.defaultAssumeValid = uint256{};
                m_assumed_blockchain_size = 0;
                m_assumed_chain_state_size = 0;
                chainTxData = ChainTxData{
                    0,
                    0,
                    0,
                };
                LogPrintf("Signet with challenge %s\n", signet_challenge[0]);
            }

            if (args.IsArgSet("-signetseednode")) {
                vSeeds = args.GetArgs("-signetseednode");
            }

            strNetworkID = CBaseChainParams::SIGNET;
            consensus.signet_blocks = true;
            consensus.signet_challenge.assign(bin.begin(), bin.end());
            consensus.nSubsidyHalvingInterval = 210000;
            consensus.BIP16Exception = uint256{};
            consensus.BIP34Height = 1;
            consensus.BIP34Hash = uint256{};
            consensus.BIP65Height = 1;
            consensus.BIP66Height = 1;
            consensus.CSVHeight = 1;
            consensus.SegwitHeight = 1;
            consensus.nPowTargetTimespan = 14 * 24 * 60 * 60; // two weeks
            consensus.nPowTargetSpacing = 10 * 60;
            consensus.fPowAllowMinDifficultyBlocks = false;
            consensus.fPowNoRetargeting = false;
            consensus.nRuleChangeActivationThreshold = 1815; // 90% of 2016
            consensus.nMinerConfirmationWindow = 2016; // nPowTargetTimespan / nPowTargetSpacing
            consensus.MinBIP9WarningHeight = 0;
            consensus.powLimit = uint256S("00000377ae000000000000000000000000000000000000000000000000000000");
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].bit = 28;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].nStartTime = consensus::BIP9Deployment::NEVER_ACTIVE;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].nTimeout = consensus::BIP9Deployment::NO_TIMEOUT;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].min_activation_height = 0; // No activation delay

            // Activation of Taproot (BIPs 340-342)
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].bit = 2;
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].nStartTime = consensus::BIP9Deployment::ALWAYS_ACTIVE;
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].nTimeout = consensus::BIP9Deployment::NO_TIMEOUT;
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].min_activation_height = 0; // No activation delay

            // message start is defined as the first 4 bytes of the sha256d of the block script
            CHashWriter h(SER_DISK, 0);
            h << consensus.signet_challenge;
            uint256 hash = h.GetHash();
            memcpy(pchMessageStart, hash.begin(), 4);

            nDefaultPort = 38333;
            nPruneAfterHeight = 1000;

            genesis = CreateGenesisBlock(1598918400, 52613770, 0x1e0377ae, 1, 50 * COIN);
            consensus.hashGenesisBlock = genesis.GetHash();
            assert(consensus.hashGenesisBlock == uint256S("0x00000008819873e925422c1ff0f99f7cc9bbb232af63a077a480a3633bee1ef6"));
            assert(genesis.hashMerkleRoot == uint256S("0x4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"));

            vFixedSeeds.clear();

            base58Prefixes[PUBKEY_ADDRESS] = std::vector<unsigned char>(1,111);
            base58Prefixes[SCRIPT_ADDRESS] = std::vector<unsigned char>(1,196);
            base58Prefixes[SECRET_KEY] =     std::vector<unsigned char>(1,239);
            base58Prefixes[EXT_PUBLIC_KEY] = {0x04, 0x35, 0x87, 0xCF};
            base58Prefixes[EXT_SECRET_KEY] = {0x04, 0x35, 0x83, 0x94};

            bech32_hrp = "tb";

            fDefaultConsistencyChecks = false;
            fRequireStandard = true;
            m_is_test_chain = true;
            m_is_mockable_chain = false;
        */
    }
}
