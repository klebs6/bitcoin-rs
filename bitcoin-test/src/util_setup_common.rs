// ---------------- [ File: bitcoin-test/src/util_setup_common.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/util/setup_common.h]

/**
  | This is connected to the logger. Can
  | be used to redirect logs to any other
  | log
  |
  */
lazy_static!{
    /*
    extern const std::function<c_void(const std::string&)> G_TEST_LOG_FUN;
    */
}

/**
  | Enable BOOST_CHECK_EQUAL for enum
  | class types
  |
  */
lazy_static!{
    /*
    template <typename T>
    std::ostream& operator<<(typename std::enable_if<std::is_enum<T>::value, std::ostream>::type& stream, const T& e)
    {
        return stream << static_cast<typename std::underlying_type<T>::type>(e);
    }
    */
}

/**
  | This global and the helpers that use
  | it are not thread-safe.
  | 
  | If thread-safety is needed, the global
  | could be made thread_local (given that
  | thread_local is supported on all architectures
  | we support) or a per-thread instance
  | could be used in the multi-threaded
  | test.
  |
  */
lazy_static!{
    /*
    extern FastRandomContext g_insecure_rand_ctx;
    */
}

/**
  | Flag to make GetRand in random.h return
  | the same number
  |
  */
lazy_static!{
    /*
    extern bool g_mock_deterministic_tests;
    */
}

pub enum SeedRand {

    /**
      | Seed with a compile time constant
      | of zeros
      |
      */
    ZEROS, 

    /**
      | Call the Seed() helper
      |
      */
    SEED,  
}

#[inline] pub fn seed_insecure_rand(seed: Option<SeedRand>)  {
    let seed: SeedRand = seed.unwrap_or(SeedRand::SEED);

    todo!();
        /*
            if (seed == SeedRand::ZEROS) {
            g_insecure_rand_ctx = FastRandomContext(/* deterministic */ true);
        } else {
            Seed(g_insecure_rand_ctx);
        }
        */
}

#[inline] pub fn insecure_rand32() -> u32 {
    
    todo!();
        /*
            return g_insecure_rand_ctx.rand32();
        */
}

#[inline] pub fn insecure_rand256() -> u256 {
    
    todo!();
        /*
            return g_insecure_rand_ctx.rand256();
        */
}

#[inline] pub fn insecure_rand_bits(bits: i32) -> u64 {
    
    todo!();
        /*
            return g_insecure_rand_ctx.randbits(bits);
        */
}

#[inline] pub fn insecure_rand_range(range: u64) -> u64 {
    
    todo!();
        /*
            return g_insecure_rand_ctx.randrange(range);
        */
}

#[inline] pub fn insecure_rand_bool() -> bool {
    
    todo!();
        /*
            return g_insecure_rand_ctx.randbool();
        */
}

pub const CENT: Amount = 1000000;

/**
  | Basic testing setup.
  | 
  | This just configures logging, data
  | dir and chain parameters.
  |
  */
pub struct BasicTestingSetup {
    global_verify_handle: ECCVerifyHandle,
    node:                 NodeContext,
    path_root:            Box<Path>,
    args:                 ArgsManager,
}

/**
  | Testing setup that performs all steps
  | up until right before
  | 
  | ChainstateManager gets initialized.
  | Meant for testing ChainstateManager
  | initialization behaviour.
  |
  */
pub struct ChainTestingSetup {
    base: BasicTestingSetup,
}

/**
  | Testing setup that configures a complete
  | environment.
  |
  */
pub struct TestingSetup {
    base: ChainTestingSetup,
}

/**
  | Identical to TestingSetup, but chain
  | set to regtest
  |
  */
pub struct RegTestingSetup {
    base: TestingSetup,
}
impl Default for RegTestingSetup {
    
    fn default() -> Self {
        todo!();
        /*

            : TestingSetup{BaseChainParams::REGTEST}
        */
    }
}

/**
  | Testing fixture that pre-creates a
  | 100-block REGTEST-mode block chain
  |
  */
pub struct TestChain100Setup {
    base: TestingSetup,

    /**
      | For convenience, coinbase transactions
      |
      */
    coinbase_txns: Vec<TransactionRef>,

    /**
      | private/public key needed to spend
      | coinbase transactions
      |
      */
    coinbase_key:  Key,
}

/**
  | Make a test setup that has disk access
  | to the debug.log file disabled. Can
  | be used in "hot loops", for example fuzzing
  | or benchmarking.
  |
  */
//let DEFAULT_T = BasicTestingSetup
pub fn make_no_log_file_context<T>(
        chain_name: Option<&str>,
        extra_args: &Vec<*const u8>) -> Box<T> {
    let chain_name: &str = chain_name.unwrap_or(base_chain_params::REGTEST);

    todo!();
        /*
            const std::vector<const char*> arguments = Cat(
            {
                "-nodebuglogfile",
                "-nodebug",
            },
            extra_args);

        return std::make_unique<T>(chain_name, arguments);
        */
}

///-----------------------
pub struct TestMemPoolEntryHelper {

    /**
      | Default values
      |
      */
    n_fee:           Amount,
    n_time:          i64,
    n_height:        u32,
    spends_coinbase: bool,
    sig_op_cost:     u32,
    lp:              LockPoints,
}

impl Default for TestMemPoolEntryHelper {
    
    fn default() -> Self {
        todo!();
        /*
        : n_fee(0),
        : n_time(0),
        : n_height(1),
        : spends_coinbase(false),
        : sig_op_cost(4),

        
        */
    }
}

impl TestMemPoolEntryHelper {
    
    /**
      | Change the default value
      |
      */
    pub fn fee(&mut self, fee: Amount) -> &mut TestMemPoolEntryHelper {
        
        todo!();
        /*
            nFee = _fee; return *this;
        */
    }
    
    pub fn time(&mut self, time: i64) -> &mut TestMemPoolEntryHelper {
        
        todo!();
        /*
            nTime = _time; return *this;
        */
    }
    
    pub fn height(&mut self, height: u32) -> &mut TestMemPoolEntryHelper {
        
        todo!();
        /*
            nHeight = _height; return *this;
        */
    }
    
    pub fn spends_coinbase(&mut self, flag: bool) -> &mut TestMemPoolEntryHelper {
        
        todo!();
        /*
            spendsCoinbase = _flag; return *this;
        */
    }
    
    pub fn sig_ops_cost(&mut self, sigops_cost: u32) -> &mut TestMemPoolEntryHelper {
        
        todo!();
        /*
            sigOpCost = _sigopsCost; return *this;
        */
    }
}

/**
  | define an implicit conversion here
  | so that uint256 may be used directly
  | in BOOST_CHECK_*
  |
  */
impl std::fmt::Display for TestMemPoolEntryHelper {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
        /*
        
        */
    }
}

/**
  | BOOST_CHECK_EXCEPTION predicates
  | to check the specific validation error.
  | 
  | Use as
  | BOOST_CHECK_EXCEPTION(code that
  | throws, exception type, HasReason("foo"));
  |
  */
pub struct HasReason {
    reason: String,
}

pub type Exception = Broken;

impl HasReason {

    pub fn new(reason: &String) -> Self {
    
        todo!();
        /*
        : reason(reason),
        */
    }
    
    pub fn invoke(&self, e: &Exception) -> bool {
        
        todo!();
        /*
            return std::string(e.what()).find(m_reason) != std::string::npos;
        }{
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/test/util/setup_common.cpp]

lazy_static!{
    /*
    const std::function<std::string(const char*)> G_TRANSLATION_FUN = nullptr;
    UrlDecodeFn* const URL_DECODE = nullptr;

    FastRandomContext g_insecure_rand_ctx;

    /** Random context to get unique temp data
     * dirs. Separate from g_insecure_rand_ctx,
     * which can be seeded from a const env var */

    static FastRandomContext g_insecure_rand_ctx_temp_path;
    */
}

/**
  | Return the unsigned from the environment
  | var if available, otherwise 0
  |
  */
pub fn get_uint_from_env(env_name: &String) -> u256 {
    
    todo!();
        /*
            const char* num = std::getenv(env_name.c_str());
        if (!num) return {};
        return uint256S(num);
        */
}

/**
  | Seed the given random ctx or use the seed
  | passed in via an environment var
  |
  */
pub fn seed(ctx: &mut FastRandomContext)  {
    
    todo!();
        /*
            // Should be enough to get the seed once for the process
        static uint256 seed{};
        static const std::string RANDOM_CTX_SEED{"RANDOM_CTX_SEED"};
        if (seed.IsNull()) seed = GetUintFromEnv(RANDOM_CTX_SEED);
        if (seed.IsNull()) seed = GetRandHash();
        LogPrintf("%s: Setting random seed for current tests to %s=%s\n", __func__, RANDOM_CTX_SEED, seed.GetHex());
        ctx = FastRandomContext(seed);
        */
}

///-----------------------------
impl Drop for BasicTestingSetup {
    fn drop(&mut self) {
        todo!();
        /*
            SetMockTime(0s); // Reset mocktime for following tests
        LogInstance().DisconnectTestLogger();
        fs::remove_all(m_path_root);
        gArgs.ClearArgs();
        ECC_Stop();
        */
    }
}

impl BasicTestingSetup {

    pub fn new(
        chain_name: Option<&str>,
        extra_args: &Vec<*const u8>) -> Self {

        let chain_name: &str =
                 chain_name.unwrap_or(base_chain_params::MAIN);
    
        todo!();
        /*


            : m_path_root{fs::temp_directory_path() / "test_common_" PACKAGE_NAME / g_insecure_rand_ctx_temp_path.rand256().ToString()},
          m_args{}
        m_node.args = &gArgs;
        const std::vector<const char*> arguments = Cat(
            {
                "dummy",
                "-printtoconsole=0",
                "-logsourcelocations",
                "-logtimemicros",
                "-logthreadnames",
                "-debug",
                "-debugexclude=libevent",
                "-debugexclude=leveldb",
            },
            extra_args);
        util::ThreadRename("test");
        fs::create_directories(m_path_root);
        m_args.ForceSetArg("-datadir", fs::PathToString(m_path_root));
        gArgs.ForceSetArg("-datadir", fs::PathToString(m_path_root));
        gArgs.ClearPathCache();
        {
            SetupServerArgs(*m_node.args);
            std::string error;
            const bool success{m_node.args->ParseParameters(arguments.size(), arguments.data(), error)};
            assert(success);
            assert(error.empty());
        }
        SelectParams(chainName);
        SeedInsecureRand();
        if (G_TEST_LOG_FUN) LogInstance().PushBackCallback(G_TEST_LOG_FUN);
        InitLogging(*m_node.args);
        AppInitParameterInteraction(*m_node.args);
        LogInstance().StartLogging();
        SHA256AutoDetect();
        ECC_Start();
        SetupEnvironment();
        SetupNetworking();
        InitSignatureCache();
        InitScriptExecutionCache();
        m_node.chain = typename interfaces::MakeChain(m_node);
        fCheckBlockIndex = true;
        static bool noui_connected = false;
        if (!noui_connected) {
            noui_connect();
            noui_connected = true;
        }
        */
    }
}

///---------------------------
impl Drop for ChainTestingSetup {
    fn drop(&mut self) {
        todo!();
        /*
            if (m_node.scheduler) m_node.scheduler->stop();
        StopScriptCheckWorkerThreads();
        GetMainSignals().FlushBackgroundCallbacks();
        GetMainSignals().UnregisterBackgroundSignalScheduler();
        m_node.connman.reset();
        m_node.banman.reset();
        m_node.addrman.reset();
        m_node.args = nullptr;
        UnloadBlockIndex(m_node.mempool.get(), *m_node.chainman);
        m_node.mempool.reset();
        m_node.scheduler.reset();
        m_node.chainman->Reset();
        m_node.chainman.reset();
        */
    }
}

impl ChainTestingSetup {
    
    pub fn new(
        chain_name: Option<&str>,
        extra_args: &Vec<*const u8>) -> Self {

        let chain_name: &str =
                 chain_name.unwrap_or(base_chain_params::MAIN);
    
        todo!();
        /*


            : BasicTestingSetup(chainName, extra_args)

        // We have to run a scheduler thread to prevent ActivateBestChain
        // from blocking due to queue overrun.
        m_node.scheduler = std::make_unique<CScheduler>();
        m_node.scheduler->m_service_thread = std::thread(util::TraceThread, "scheduler", [&] { m_node.scheduler->serviceQueue(); });
        GetMainSignals().RegisterBackgroundSignalScheduler(*m_node.scheduler);

        m_node.fee_estimator = std::make_unique<CBlockPolicyEstimator>();
        m_node.mempool = std::make_unique<CTxMemPool>(m_node.fee_estimator.get(), 1);

        m_node.chainman = std::make_unique<ChainstateManager>();
        m_node.chainman->m_blockman.m_block_tree_db = std::make_unique<CBlockTreeDB>(1 << 20, true);

        // Start script-checking threads. Set g_parallel_script_checks to true so they are used.
        constexpr int script_check_threads = 2;
        StartScriptCheckWorkerThreads(script_check_threads);
        g_parallel_script_checks = true;
        */
    }
}

impl TestingSetup {
    
    pub fn new(
        chain_name: Option<&str>,
        extra_args: &Vec<*const u8>) -> Self {

        let chain_name: &str =
                 chain_name.unwrap_or(base_chain_params::MAIN);
    
        todo!();
        /*


            : ChainTestingSetup(chainName, extra_args)

        const CChainParams& chainparams = Params();
        // Ideally we'd move all the RPC tests to the functional testing framework
        // instead of unit tests, but for now we need these here.
        RegisterAllCoreRPCCommands(tableRPC);

        m_node.chainman->InitializeChainstate(m_node.mempool.get());
        m_node.chainman->ActiveChainstate().InitCoinsDB(
            /* cache_size_bytes */ 1 << 23, /* in_memory */ true, /* should_wipe */ false);
        assert(!m_node.chainman->ActiveChainstate().CanFlushToDisk());
        m_node.chainman->ActiveChainstate().InitCoinsCache(1 << 23);
        assert(m_node.chainman->ActiveChainstate().CanFlushToDisk());
        if (!m_node.chainman->ActiveChainstate().LoadGenesisBlock()) {
            throw std::runtime_error("LoadGenesisBlock failed.");
        }

        BlockValidationState state;
        if (!m_node.chainman->ActiveChainstate().ActivateBestChain(state)) {
            throw std::runtime_error(strprintf("ActivateBestChain failed. (%s)", state.ToString()));
        }

        m_node.addrman = std::make_unique<AddrMan>(/* asmap */ std::vector<bool>(), /* deterministic */ false, /* consistency_check_ratio */ 0);
        m_node.banman = std::make_unique<BanMan>(m_args.GetDataDirBase() / "banlist", nullptr, DEFAULT_MISBEHAVING_BANTIME);
        m_node.connman = std::make_unique<CConnman>(0x1337, 0x1337, *m_node.addrman); // Deterministic randomness for tests.
        m_node.peerman = PeerManager::make(chainparams, *m_node.connman, *m_node.addrman,
                                           m_node.banman.get(), *m_node.chainman,
                                           *m_node.mempool, false);
        {
            CConnman::Options options;
            options.m_msgproc = m_node.peerman.get();
            m_node.connman->Init(options);
        }
        */
    }
}

impl TestChain100Setup {

    pub fn new(extra_args: &Vec<*const u8>) -> Self {
    
        todo!();
        /*


            : TestingSetup{CBaseChainParams::REGTEST, extra_args}

        SetMockTime(1598887952);
        constexpr std::array<unsigned char, 32> vchKey = {
            {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1}};
        coinbaseKey.Set(vchKey.begin(), vchKey.end(), true);

        // Generate a 100-block chain:
        this->mineBlocks(COINBASE_MATURITY);

        {
            LOCK(::cs_main);
            assert(
                m_node.chainman->ActiveChain().Tip()->GetBlockHash().ToString() ==
                "571d80a9967ae599cec0448b0b0ba1cfb606f584d8069bd7166b86854ba7a191");
        }
        */
    }
    
    /**
      | Mine a series of new blocks on the active
      | chain.
      |
      */
    pub fn mine_blocks(&mut self, num_blocks: i32)  {
        
        todo!();
        /*
            CScript scriptPubKey = CScript() << ToByteVector(coinbaseKey.GetPubKey()) << OP_CHECKSIG;
        for (int i = 0; i < num_blocks; i++) {
            std::vector<CMutableTransaction> noTxns;
            CBlock b = CreateAndProcessBlock(noTxns, scriptPubKey);
            SetMockTime(GetTime() + 1);
            m_coinbase_txns.push_back(b.vtx[0]);
        }
        */
    }
    
    /**
      | Create a new block with just given transactions,
      | coinbase paying to scriptPubKey.
      |
      */
    pub fn create_block(&mut self, 
        txns:           &Vec<MutableTransaction>,
        script_pub_key: &Script,
        chainstate:     &mut ChainState) -> Block {
        
        todo!();
        /*
            const CChainParams& chainparams = Params();
        CTxMemPool empty_pool;
        CBlock block = BlockAssembler(chainstate, empty_pool, chainparams).CreateNewBlock(scriptPubKey)->block;

        Assert(block.vtx.size() == 1);
        for (const CMutableTransaction& tx : txns) {
            block.vtx.push_back(MakeTransactionRef(tx));
        }
        RegenerateCommitments(block, *Assert(m_node.chainman));

        while (!CheckProofOfWork(block.GetHash(), block.nBits, chainparams.GetConsensus())) ++block.nNonce;

        return block;
        */
    }
    
    /**
      | Create a new block with just given transactions,
      | coinbase paying to scriptPubKey, and
      | try to add it to the current chain.
      | 
      | If no chainstate is specified, default
      | to the active.
      |
      */
    pub fn create_and_process_block(&mut self, 
        txns:           &Vec<MutableTransaction>,
        script_pub_key: &Script,
        chainstate:     Option<*mut ChainState>) -> Block {

        todo!();
        /*
            if (!chainstate) {
            chainstate = &Assert(m_node.chainman)->ActiveChainstate();
        }

        const CChainParams& chainparams = Params();
        const CBlock block = this->CreateBlock(txns, scriptPubKey, *chainstate);
        std::shared_ptr<const CBlock> shared_pblock = std::make_shared<const CBlock>(block);
        Assert(m_node.chainman)->ProcessNewBlock(chainparams, shared_pblock, true, nullptr);

        return block;
        */
    }
    
    /**
      | Create a transaction and submit to the
      | mempool.
      | 
      | -----------
      | @param input_transaction
      | 
      | The transaction to spend
      | ----------
      | @param input_vout
      | 
      | The vout to spend from the input_transaction
      | ----------
      | @param input_height
      | 
      | The height of the block that included
      | the input_transaction
      | ----------
      | @param input_signing_key
      | 
      | The key to spend the input_transaction
      | ----------
      | @param output_destination
      | 
      | Where to send the output
      | ----------
      | @param output_amount
      | 
      | How much to send
      | ----------
      | @param submit
      | 
      | Whether or not to submit to mempool
      |
      */
    pub fn create_valid_mempool_transaction(&mut self, 
        input_transaction:  TransactionRef,
        input_vout:         i32,
        input_height:       i32,
        input_signing_key:  Key,
        output_destination: Script,
        output_amount:      Option<Amount>,
        submit:             Option<bool>) -> MutableTransaction {

        let output_amount: Amount = output_amount.unwrap_or(1 * COIN);
        let submit: bool = submit.unwrap_or(true);
        
        todo!();
        /*
            // Transaction we will submit to the mempool
        CMutableTransaction mempool_txn;

        // Create an input
        OutPoint outpoint_to_spend(input_transaction->GetHash(), input_vout);
        CTxIn input(outpoint_to_spend);
        mempool_txn.vin.push_back(input);

        // Create an output
        CTxOut output(output_amount, output_destination);
        mempool_txn.vout.push_back(output);

        // Sign the transaction
        // - Add the signing key to a keystore
        FillableSigningProvider keystore;
        keystore.AddKey(input_signing_key);
        // - Populate a CoinsViewCache with the unspent output
        CCoinsView coins_view;
        CCoinsViewCache coins_cache(&coins_view);
        AddCoins(coins_cache, *input_transaction.get(), input_height);
        // - Use GetCoin to properly populate utxo_to_spend,
        Coin utxo_to_spend;
        assert(coins_cache.GetCoin(outpoint_to_spend, utxo_to_spend));
        // - Then add it to a map to pass in to SignTransaction
        std::map<OutPoint, Coin> input_coins;
        input_coins.insert({outpoint_to_spend, utxo_to_spend});
        // - Default signature hashing type
        int nHashType = SIGHASH_ALL;
        std::map<int, bilingual_str> input_errors;
        assert(SignTransaction(mempool_txn, &keystore, input_coins, nHashType, input_errors));

        // If submit=true, add transaction to the mempool.
        if (submit) {
            LOCK(cs_main);
            const MempoolAcceptResult result = AcceptToMemoryPool(m_node.chainman->ActiveChainstate(), *m_node.mempool.get(), MakeTransactionRef(mempool_txn), /* bypass_limits */ false);
            assert(result.m_result_type == MempoolAcceptResult::ResultType::VALID);
        }

        return mempool_txn;
        */
    }
}

impl TestMemPoolEntryHelper {
    
    pub fn from_mutable_txn(&self, tx: &MutableTransaction) -> TxMemPoolEntry {
        
        todo!();
        /*
            return FromTx(MakeTransactionRef(tx));
        */
    }
    
    pub fn from_txn_ref(&self, tx: &TransactionRef) -> TxMemPoolEntry {
        
        todo!();
        /*
            return CTxMemPoolEntry(tx, nFee, nTime, nHeight,
                               spendsCoinbase, sigOpCost, lp);
        */
    }
}

/**
  | @return
  | 
  | a real block (0000000000013b8ab2cd513b0261a14096412195a72a0c4827d229dcc7e0f7af)
  | with 9 txs.
  |
  */
pub fn get_block_13b8a() -> Block {
    
    todo!();
        /*
            CBlock block;
        DataStream stream(ParseHex("0100000090f0a9f110702f808219ebea1173056042a714bad51b916cb6800000000000005275289558f51c9966699404ae2294730c3c9f9bda53523ce50e9b95e558da2fdb261b4d4c86041b1ab1bf930901000000010000000000000000000000000000000000000000000000000000000000000000ffffffff07044c86041b0146ffffffff0100f2052a01000000434104e18f7afbe4721580e81e8414fc8c24d7cfacf254bb5c7b949450c3e997c2dc1242487a8169507b631eb3771f2b425483fb13102c4eb5d858eef260fe70fbfae0ac00000000010000000196608ccbafa16abada902780da4dc35dafd7af05fa0da08cf833575f8cf9e836000000004a493046022100dab24889213caf43ae6adc41cf1c9396c08240c199f5225acf45416330fd7dbd022100fe37900e0644bf574493a07fc5edba06dbc07c311b947520c2d514bc5725dcb401ffffffff0100f2052a010000001976a914f15d1921f52e4007b146dfa60f369ed2fc393ce288ac000000000100000001fb766c1288458c2bafcfec81e48b24d98ec706de6b8af7c4e3c29419bfacb56d000000008c493046022100f268ba165ce0ad2e6d93f089cfcd3785de5c963bb5ea6b8c1b23f1ce3e517b9f022100da7c0f21adc6c401887f2bfd1922f11d76159cbc597fbd756a23dcbb00f4d7290141042b4e8625a96127826915a5b109852636ad0da753c9e1d5606a50480cd0c40f1f8b8d898235e571fe9357d9ec842bc4bba1827daaf4de06d71844d0057707966affffffff0280969800000000001976a9146963907531db72d0ed1a0cfb471ccb63923446f388ac80d6e34c000000001976a914f0688ba1c0d1ce182c7af6741e02658c7d4dfcd388ac000000000100000002c40297f730dd7b5a99567eb8d27b78758f607507c52292d02d4031895b52f2ff010000008b483045022100f7edfd4b0aac404e5bab4fd3889e0c6c41aa8d0e6fa122316f68eddd0a65013902205b09cc8b2d56e1cd1f7f2fafd60a129ed94504c4ac7bdc67b56fe67512658b3e014104732012cb962afa90d31b25d8fb0e32c94e513ab7a17805c14ca4c3423e18b4fb5d0e676841733cb83abaf975845c9f6f2a8097b7d04f4908b18368d6fc2d68ecffffffffca5065ff9617cbcba45eb23726df6498a9b9cafed4f54cbab9d227b0035ddefb000000008a473044022068010362a13c7f9919fa832b2dee4e788f61f6f5d344a7c2a0da6ae740605658022006d1af525b9a14a35c003b78b72bd59738cd676f845d1ff3fc25049e01003614014104732012cb962afa90d31b25d8fb0e32c94e513ab7a17805c14ca4c3423e18b4fb5d0e676841733cb83abaf975845c9f6f2a8097b7d04f4908b18368d6fc2d68ecffffffff01001ec4110200000043410469ab4181eceb28985b9b4e895c13fa5e68d85761b7eee311db5addef76fa8621865134a221bd01f28ec9999ee3e021e60766e9d1f3458c115fb28650605f11c9ac000000000100000001cdaf2f758e91c514655e2dc50633d1e4c84989f8aa90a0dbc883f0d23ed5c2fa010000008b48304502207ab51be6f12a1962ba0aaaf24a20e0b69b27a94fac5adf45aa7d2d18ffd9236102210086ae728b370e5329eead9accd880d0cb070aea0c96255fae6c4f1ddcce1fd56e014104462e76fd4067b3a0aa42070082dcb0bf2f388b6495cf33d789904f07d0f55c40fbd4b82963c69b3dc31895d0c772c812b1d5fbcade15312ef1c0e8ebbb12dcd4ffffffff02404b4c00000000001976a9142b6ba7c9d796b75eef7942fc9288edd37c32f5c388ac002d3101000000001976a9141befba0cdc1ad56529371864d9f6cb042faa06b588ac000000000100000001b4a47603e71b61bc3326efd90111bf02d2f549b067f4c4a8fa183b57a0f800cb010000008a4730440220177c37f9a505c3f1a1f0ce2da777c339bd8339ffa02c7cb41f0a5804f473c9230220585b25a2ee80eb59292e52b987dad92acb0c64eced92ed9ee105ad153cdb12d001410443bd44f683467e549dae7d20d1d79cbdb6df985c6e9c029c8d0c6cb46cc1a4d3cf7923c5021b27f7a0b562ada113bc85d5fda5a1b41e87fe6e8802817cf69996ffffffff0280651406000000001976a9145505614859643ab7b547cd7f1f5e7e2a12322d3788ac00aa0271000000001976a914ea4720a7a52fc166c55ff2298e07baf70ae67e1b88ac00000000010000000586c62cd602d219bb60edb14a3e204de0705176f9022fe49a538054fb14abb49e010000008c493046022100f2bc2aba2534becbdf062eb993853a42bbbc282083d0daf9b4b585bd401aa8c9022100b1d7fd7ee0b95600db8535bbf331b19eed8d961f7a8e54159c53675d5f69df8c014104462e76fd4067b3a0aa42070082dcb0bf2f388b6495cf33d789904f07d0f55c40fbd4b82963c69b3dc31895d0c772c812b1d5fbcade15312ef1c0e8ebbb12dcd4ffffffff03ad0e58ccdac3df9dc28a218bcf6f1997b0a93306faaa4b3a28ae83447b2179010000008b483045022100be12b2937179da88599e27bb31c3525097a07cdb52422d165b3ca2f2020ffcf702200971b51f853a53d644ebae9ec8f3512e442b1bcb6c315a5b491d119d10624c83014104462e76fd4067b3a0aa42070082dcb0bf2f388b6495cf33d789904f07d0f55c40fbd4b82963c69b3dc31895d0c772c812b1d5fbcade15312ef1c0e8ebbb12dcd4ffffffff2acfcab629bbc8685792603762c921580030ba144af553d271716a95089e107b010000008b483045022100fa579a840ac258871365dd48cd7552f96c8eea69bd00d84f05b283a0dab311e102207e3c0ee9234814cfbb1b659b83671618f45abc1326b9edcc77d552a4f2a805c0014104462e76fd4067b3a0aa42070082dcb0bf2f388b6495cf33d789904f07d0f55c40fbd4b82963c69b3dc31895d0c772c812b1d5fbcade15312ef1c0e8ebbb12dcd4ffffffffdcdc6023bbc9944a658ddc588e61eacb737ddf0a3cd24f113b5a8634c517fcd2000000008b4830450221008d6df731df5d32267954bd7d2dda2302b74c6c2a6aa5c0ca64ecbabc1af03c75022010e55c571d65da7701ae2da1956c442df81bbf076cdbac25133f99d98a9ed34c014104462e76fd4067b3a0aa42070082dcb0bf2f388b6495cf33d789904f07d0f55c40fbd4b82963c69b3dc31895d0c772c812b1d5fbcade15312ef1c0e8ebbb12dcd4ffffffffe15557cd5ce258f479dfd6dc6514edf6d7ed5b21fcfa4a038fd69f06b83ac76e010000008b483045022023b3e0ab071eb11de2eb1cc3a67261b866f86bf6867d4558165f7c8c8aca2d86022100dc6e1f53a91de3efe8f63512850811f26284b62f850c70ca73ed5de8771fb451014104462e76fd4067b3a0aa42070082dcb0bf2f388b6495cf33d789904f07d0f55c40fbd4b82963c69b3dc31895d0c772c812b1d5fbcade15312ef1c0e8ebbb12dcd4ffffffff01404b4c00000000001976a9142b6ba7c9d796b75eef7942fc9288edd37c32f5c388ac00000000010000000166d7577163c932b4f9690ca6a80b6e4eb001f0a2fa9023df5595602aae96ed8d000000008a4730440220262b42546302dfb654a229cefc86432b89628ff259dc87edd1154535b16a67e102207b4634c020a97c3e7bbd0d4d19da6aa2269ad9dded4026e896b213d73ca4b63f014104979b82d02226b3a4597523845754d44f13639e3bf2df5e82c6aab2bdc79687368b01b1ab8b19875ae3c90d661a3d0a33161dab29934edeb36aa01976be3baf8affffffff02404b4c00000000001976a9144854e695a02af0aeacb823ccbc272134561e0a1688ac40420f00000000001976a914abee93376d6b37b5c2940655a6fcaf1c8e74237988ac0000000001000000014e3f8ef2e91349a9059cb4f01e54ab2597c1387161d3da89919f7ea6acdbb371010000008c49304602210081f3183471a5ca22307c0800226f3ef9c353069e0773ac76bb580654d56aa523022100d4c56465bdc069060846f4fbf2f6b20520b2a80b08b168b31e66ddb9c694e240014104976c79848e18251612f8940875b2b08d06e6dc73b9840e8860c066b7e87432c477e9a59a453e71e6d76d5fe34058b800a098fc1740ce3012e8fc8a00c96af966ffffffff02c0e1e400000000001976a9144134e75a6fcb6042034aab5e18570cf1f844f54788ac404b4c00000000001976a9142b6ba7c9d796b75eef7942fc9288edd37c32f5c388ac00000000"), SER_NETWORK, PROTOCOL_VERSION);
        stream >> block;
        return block;
        */
}
