crate::ix!();

/**
  | Main network on which people trade goods
  | and services.
  |
  */
pub struct MainParams {
    base: ChainParams,
}

impl Default for MainParams {
    
    fn default() -> Self {
        todo!();
        /*


            strNetworkID = CBaseChainParams::MAIN;
            consensus.signet_blocks = false;
            consensus.signet_challenge.clear();
            consensus.nSubsidyHalvingInterval = 210000;
            consensus.BIP16Exception = uint256S("0x00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22");
            consensus.BIP34Height = 227931;
            consensus.BIP34Hash = uint256S("0x000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8");
            consensus.BIP65Height = 388381; // 000000000000000004c2b624ed5d7756c508d90fd0da2c7c679febfa6c4735f0
            consensus.BIP66Height = 363725; // 00000000000000000379eaa19dce8c9b722d46ae6a57c2f1a988119488b50931
            consensus.CSVHeight = 419328; // 000000000000000004a1b34462cb8aeebd5799177f7a29cf28f2d1961716b5b5
            consensus.SegwitHeight = 481824; // 0000000000000000001c8018d9cb3b742ef25114f27563e3fc4a1902167f9893
            consensus.MinBIP9WarningHeight = 483840; // segwit activation height + miner confirmation window
            consensus.powLimit = uint256S("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
            consensus.nPowTargetTimespan = 14 * 24 * 60 * 60; // two weeks
            consensus.nPowTargetSpacing = 10 * 60;
            consensus.fPowAllowMinDifficultyBlocks = false;
            consensus.fPowNoRetargeting = false;
            consensus.nRuleChangeActivationThreshold = 1815; // 90% of 2016
            consensus.nMinerConfirmationWindow = 2016; // nPowTargetTimespan / nPowTargetSpacing
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].bit = 28;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].nStartTime = consensus::BIP9Deployment::NEVER_ACTIVE;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].nTimeout = consensus::BIP9Deployment::NO_TIMEOUT;
            consensus.vDeployments[consensus::DEPLOYMENT_TESTDUMMY].min_activation_height = 0; // No activation delay

            // Deployment of Taproot (BIPs 340-342)
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].bit = 2;
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].nStartTime = 1619222400; // April 24th, 2021
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].nTimeout = 1628640000; // August 11th, 2021
            consensus.vDeployments[consensus::DEPLOYMENT_TAPROOT].min_activation_height = 709632; // Approximately November 12th, 2021

            consensus.nMinimumChainWork = uint256S("0x00000000000000000000000000000000000000001fa4663bbbe19f82de910280");
            consensus.defaultAssumeValid = uint256S("0x00000000000000000008a89e854d57e5667df88f1cdef6fde2fbca1de5b639ad"); // 691719

            /**
             * The message start string is designed to be unlikely to occur in normal data.
             * The characters are rarely used upper ASCII, not valid as UTF-8, and produce
             * a large 32-bit integer with any alignment.
             */
            pchMessageStart[0] = 0xf9;
            pchMessageStart[1] = 0xbe;
            pchMessageStart[2] = 0xb4;
            pchMessageStart[3] = 0xd9;
            nDefaultPort = 8333;
            nPruneAfterHeight = 100000;
            m_assumed_blockchain_size = 420;
            m_assumed_chain_state_size = 6;

            genesis = CreateGenesisBlock(1231006505, 2083236893, 0x1d00ffff, 1, 50 * COIN);
            consensus.hashGenesisBlock = genesis.GetHash();
            assert(consensus.hashGenesisBlock == uint256S("0x000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"));
            assert(genesis.hashMerkleRoot == uint256S("0x4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"));

            // Note that of those which support the service bits prefix, most only support a subset of
            // possible options.
            // This is fine at runtime as we'll fall back to using them as an addrfetch if they don't support the
            // service bits we want, but we should get them updated to support all service bits wanted by any
            // release ASAP to avoid it where possible.
            vSeeds.emplace_back("seed.bitcoin.sipa.be."); // Pieter Wuille, only supports x1, x5, x9, and xd
            vSeeds.emplace_back("dnsseed.bluematt.me."); // Matt Corallo, only supports x9
            vSeeds.emplace_back("dnsseed.bitcoin.dashjr.org."); // Luke Dashjr
            vSeeds.emplace_back("seed.bitcoinstats.com."); // Christian Decker, supports x1 - xf
            vSeeds.emplace_back("seed.bitcoin.jonasschnelli.ch."); // Jonas Schnelli, only supports x1, x5, x9, and xd
            vSeeds.emplace_back("seed.btc.petertodd.org."); // Peter Todd, only supports x1, x5, x9, and xd
            vSeeds.emplace_back("seed.bitcoin.sprovoost.nl."); // Sjors Provoost
            vSeeds.emplace_back("dnsseed.emzy.de."); // Stephan Oeste
            vSeeds.emplace_back("seed.bitcoin.wiz.biz."); // Jason Maurice

            base58Prefixes[PUBKEY_ADDRESS] = std::vector<unsigned char>(1,0);
            base58Prefixes[SCRIPT_ADDRESS] = std::vector<unsigned char>(1,5);
            base58Prefixes[SECRET_KEY] =     std::vector<unsigned char>(1,128);
            base58Prefixes[EXT_PUBLIC_KEY] = {0x04, 0x88, 0xB2, 0x1E};
            base58Prefixes[EXT_SECRET_KEY] = {0x04, 0x88, 0xAD, 0xE4};

            bech32_hrp = "bc";

            vFixedSeeds = std::vector<uint8_t>(std::begin(chainparams_seed_main), std::end(chainparams_seed_main));

            fDefaultConsistencyChecks = false;
            fRequireStandard = true;
            m_is_test_chain = false;
            m_is_mockable_chain = false;

            checkpointData = {
                {
                    { 11111, uint256S("0x0000000069e244f73d78e8fd29ba2fd2ed618bd6fa2ee92559f542fdb26e7c1d")},
                    { 33333, uint256S("0x000000002dd5588a74784eaa7ab0507a18ad16a236e7b1ce69f00d7ddfb5d0a6")},
                    { 74000, uint256S("0x0000000000573993a3c9e41ce34471c079dcf5f52a0e824a81e7f953b8661a20")},
                    {105000, uint256S("0x00000000000291ce28027faea320c8d2b054b2e0fe44a773f3eefb151d6bdc97")},
                    {134444, uint256S("0x00000000000005b12ffd4cd315cd34ffd4a594f430ac814c91184a0d42d2b0fe")},
                    {168000, uint256S("0x000000000000099e61ea72015e79632f216fe6cb33d7899acb35b75c8303b763")},
                    {193000, uint256S("0x000000000000059f452a5f7340de6682a977387c17010ff6e6c3bd83ca8b1317")},
                    {210000, uint256S("0x000000000000048b95347e83192f69cf0366076336c639f9b7228e9ba171342e")},
                    {216116, uint256S("0x00000000000001b4f4b433e81ee46494af945cf96014816a4e2370f11b23df4e")},
                    {225430, uint256S("0x00000000000001c108384350f74090433e7fcf79a606b8e797f065b130575932")},
                    {250000, uint256S("0x000000000000003887df1f29024b06fc2200b55f8af8f35453d7be294df2d214")},
                    {279000, uint256S("0x0000000000000001ae8c72a0b0c301f67e3afca10e819efa9041e458e9bd7e40")},
                    {295000, uint256S("0x00000000000000004d9b4ef50f0f9d686fd69db2e03af35a100370c64632a983")},
                }
            };

            m_assumeutxo_data = MapAssumeUtxo{
             // TODO to be specified in a future patch.
            };

            chainTxData = ChainTxData{
                // Data from RPC: getchaintxstats 4096 00000000000000000008a89e854d57e5667df88f1cdef6fde2fbca1de5b639ad
                /* nTime    */ 1626697539,
                /* nTxCount */ 656509474,
                /* dTxRate  */ 2.424920418708139,
            };
        */
    }
}
