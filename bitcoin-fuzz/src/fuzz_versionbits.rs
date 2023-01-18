crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/versionbits.cpp]

pub struct TestConditionChecker {
    cache:                 RefCell<ThresholdConditionCache>,
    dummy_params:          ChainConsensusParams,
    begin:                 i64,
    end:                   i64,
    period:                i32,
    threshold:             i32,
    min_activation_height: i32,
    bit:                   i32,
}

impl abstract_threshold_condition_checker::Interface for TestConditionChecker { }

impl abstract_threshold_condition_checker::Threshold           for TestConditionChecker { 

    fn threshold(&self, params: &ChainConsensusParams) -> i32 {
        
        todo!();
        /*
            return m_threshold;
        */
    }
}

impl abstract_threshold_condition_checker::Period              for TestConditionChecker { 

    fn period(&self, params: &ChainConsensusParams) -> i32 {
        
        todo!();
        /*
            return m_period;
        */
    }
}

impl abstract_threshold_condition_checker::MinActivationHeight for TestConditionChecker { 

    fn min_activation_height(&self, params: &ChainConsensusParams) -> i32 {
        
        todo!();
        /*
            return m_min_activation_height;
        */
    }
}

impl abstract_threshold_condition_checker::EndTime             for TestConditionChecker { 

    fn end_time(&self, params: &ChainConsensusParams) -> i64 {
        
        todo!();
        /*
            return m_end;
        */
    }
}

impl abstract_threshold_condition_checker::BeginTime           for TestConditionChecker { 

    fn begin_time(&self, params: &ChainConsensusParams) -> i64 {
        
        todo!();
        /*
            return m_begin;
        */
    }
}

impl abstract_threshold_condition_checker::Condition           for TestConditionChecker { 

    fn condition(&self, 
        pindex: *const BlockIndex,
        params: &ChainConsensusParams) -> bool {
        
        todo!();
        /*
            return Condition(pindex->nVersion);
        */
    }
}

impl TestConditionChecker {
    
    pub fn new(
        begin:                 i64,
        end:                   i64,
        period:                i32,
        threshold:             i32,
        min_activation_height: i32,
        bit:                   i32) -> Self {
    
        todo!();
        /*


            : m_begin{begin}, m_end{end}, m_period{period}, m_threshold{threshold}, m_min_activation_height{min_activation_height}, m_bit{bit}

            assert(m_period > 0);
            assert(0 <= m_threshold && m_threshold <= m_period);
            assert(0 <= m_bit && m_bit < 32 && m_bit < VERSIONBITS_NUM_BITS);
            assert(0 <= m_min_activation_height);
        */
    }
    
    pub fn get_state_for(&self, pindex_prev: *const BlockIndex) -> ThresholdState {
        
        todo!();
        /*
            return AbstractThresholdConditionChecker::GetStateFor(pindexPrev, dummy_params, m_cache);
        */
    }
    
    pub fn get_state_since_height_for(&self, pindex_prev: *const BlockIndex) -> i32 {
        
        todo!();
        /*
            return AbstractThresholdConditionChecker::GetStateSinceHeightFor(pindexPrev, dummy_params, m_cache);
        */
    }
    
    pub fn get_state_statistics_for(&self, pindex_prev: *const BlockIndex) -> BIP9Stats {
        
        todo!();
        /*
            return AbstractThresholdConditionChecker::GetStateStatisticsFor(pindexPrev, dummy_params);
        */
    }
    
    pub fn condition_with_version(&self, version: i32) -> bool {
        
        todo!();
        /*
            uint32_t mask = ((uint32_t)1) << m_bit;
            return (((version & VERSIONBITS_TOP_MASK) == VERSIONBITS_TOP_BITS) && (version & mask) != 0);
        */
    }
    
    pub fn condition_with_blockindex(&self, pindex: *const BlockIndex) -> bool {
        
        todo!();
        /*
            return Condition(pindex->nVersion);
        */
    }
}

/**
  | Track blocks mined for test
  |
  */
pub struct Blocks {
    blocks:     Vec<Box<BlockIndex>>,
    start_time: u32,
    interval:   u32,
    signal:     i32,
    no_signal:  i32,
}

impl Blocks {
    
    pub fn new(
        start_time: u32,
        interval:   u32,
        signal:     i32,
        no_signal:  i32) -> Self {
    
        todo!();
        /*


            : m_start_time{start_time}, m_interval{interval}, m_signal{signal}, m_no_signal{no_signal}
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return m_blocks.size();
        */
    }
    
    pub fn tip(&self) -> Option<Arc<BlockIndex>> {
        
        todo!();
        /*
            return m_blocks.empty() ? nullptr : m_blocks.back().get();
        */
    }
    
    pub fn mine_block(&mut self, signal: bool) -> *mut BlockIndex {
        
        todo!();
        /*
            CBlockHeader header;
            header.nVersion = signal ? m_signal : m_no_signal;
            header.nTime = m_start_time + m_blocks.size() * m_interval;
            header.nBits = 0x1d00ffff;

            auto current_block = std::make_unique<CBlockIndex>(header);
            current_block->pprev = tip();
            current_block->nHeight = m_blocks.size();
            current_block->BuildSkip();

            return m_blocks.emplace_back(std::move(current_block)).get();
        */
    }
}

lazy_static!{
    /*
    std::unique_ptr<const CChainParams> g_params;
    */
}

pub fn initialize()  {
    
    todo!();
        /*
            // this is actually comparatively slow, so only do it once
        g_params = CreateChainParams(ArgsManager{}, CBaseChainParams::MAIN);
        assert(g_params != nullptr);
        */
}

pub const MAX_START_TIME: u32 = 4102444800; // 2100-01-01

#[fuzz_test(initializer = "initialize")]
fn versionbits() {
    todo!();
    /*
    
        const CChainParams& params = *g_params;
        const int64_t interval = params.GetConsensus().nPowTargetSpacing;
        assert(interval > 1); // need to be able to halve it
        assert(interval < std::numeric_limits<int32_t>::max());

        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());

        // making period/max_periods larger slows these tests down significantly
        const int period = 32;
        const size_t max_periods = 16;
        const size_t max_blocks = 2 * period * max_periods;

        const int threshold = fuzzed_data_provider.ConsumeIntegralInRange(1, period);
        assert(0 < threshold && threshold <= period); // must be able to both pass and fail threshold!

        // too many blocks at 10min each might cause uint32_t time to overflow if
        // block_start_time is at the end of the range above
        assert(std::numeric_limits<uint32_t>::max() - MAX_START_TIME > interval * max_blocks);

        const int64_t block_start_time = fuzzed_data_provider.ConsumeIntegralInRange<uint32_t>(params.GenesisBlock().nTime, MAX_START_TIME);

        // what values for version will we use to signal / not signal?
        const int32_t ver_signal = fuzzed_data_provider.ConsumeIntegral<int32_t>();
        const int32_t ver_nosignal = fuzzed_data_provider.ConsumeIntegral<int32_t>();

        // select deployment parameters: bit, start time, timeout
        const int bit = fuzzed_data_provider.ConsumeIntegralInRange<int>(0, VERSIONBITS_NUM_BITS - 1);

        bool always_active_test = false;
        bool never_active_test = false;
        int64_t start_time;
        int64_t timeout;
        if (fuzzed_data_provider.ConsumeBool()) {
            // pick the timestamp to switch based on a block
            // note states will change *after* these blocks because mediantime lags
            int start_block = fuzzed_data_provider.ConsumeIntegralInRange<int>(0, period * (max_periods - 3));
            int end_block = fuzzed_data_provider.ConsumeIntegralInRange<int>(0, period * (max_periods - 3));

            start_time = block_start_time + start_block * interval;
            timeout = block_start_time + end_block * interval;

            // allow for times to not exactly match a block
            if (fuzzed_data_provider.ConsumeBool()) start_time += interval / 2;
            if (fuzzed_data_provider.ConsumeBool()) timeout += interval / 2;
        } else {
            if (fuzzed_data_provider.ConsumeBool()) {
                start_time = consensus::BIP9Deployment::ALWAYS_ACTIVE;
                always_active_test = true;
            } else {
                start_time = consensus::BIP9Deployment::NEVER_ACTIVE;
                never_active_test = true;
            }
            timeout = fuzzed_data_provider.ConsumeBool() ? consensus::BIP9Deployment::NO_TIMEOUT : fuzzed_data_provider.ConsumeIntegral<int64_t>();
        }
        int min_activation = fuzzed_data_provider.ConsumeIntegralInRange<int>(0, period * max_periods);

        TestConditionChecker checker(start_time, timeout, period, threshold, min_activation, bit);

        // Early exit if the versions don't signal sensibly for the deployment
        if (!checker.Condition(ver_signal)) return;
        if (checker.Condition(ver_nosignal)) return;
        if (ver_nosignal < 0) return;

        // TOP_BITS should ensure version will be positive and meet min
        // version requirement
        assert(ver_signal > 0);
        assert(ver_signal >= VERSIONBITS_LAST_OLD_BLOCK_VERSION);

        // Now that we have chosen time and versions, setup to mine blocks
        Blocks blocks(block_start_time, interval, ver_signal, ver_nosignal);

        /* Strategy:
         *  * we will mine a final period worth of blocks, with
         *    randomised signalling according to a mask
         *  * but before we mine those blocks, we will mine some
         *    randomised number of prior periods; with either all
         *    or no blocks in the period signalling
         *
         * We establish the mask first, then consume "bools" until
         * we run out of fuzz data to work out how many prior periods
         * there are and which ones will signal.
         */

        // establish the mask
        const uint32_t signalling_mask = fuzzed_data_provider.ConsumeIntegral<uint32_t>();

        // mine prior periods
        while (fuzzed_data_provider.remaining_bytes() > 0) {
            // all blocks in these periods either do or don't signal
            bool signal = fuzzed_data_provider.ConsumeBool();
            for (int b = 0; b < period; ++b) {
                blocks.mine_block(signal);
            }

            // don't risk exceeding max_blocks or times may wrap around
            if (blocks.size() + 2 * period > max_blocks) break;
        }
        // NOTE: fuzzed_data_provider may be fully consumed at this point and should not be used further

        // now we mine the final period and check that everything looks sane

        // count the number of signalling blocks
        int blocks_sig = 0;

        // get the info for the first block of the period
        CBlockIndex* prev = blocks.tip();
        const int exp_since = checker.GetStateSinceHeightFor(prev);
        const ThresholdState exp_state = checker.GetStateFor(prev);
        BIP9Stats last_stats = checker.GetStateStatisticsFor(prev);

        int prev_next_height = (prev == nullptr ? 0 : prev->nHeight + 1);
        assert(exp_since <= prev_next_height);

        // mine (period-1) blocks and check state
        for (int b = 1; b < period; ++b) {
            const bool signal = (signalling_mask >> (b % 32)) & 1;
            if (signal) ++blocks_sig;

            CBlockIndex* current_block = blocks.mine_block(signal);

            // verify that signalling attempt was interpreted correctly
            assert(checker.Condition(current_block) == signal);

            // state and since don't change within the period
            const ThresholdState state = checker.GetStateFor(current_block);
            const int since = checker.GetStateSinceHeightFor(current_block);
            assert(state == exp_state);
            assert(since == exp_since);

            // GetStateStatistics may crash when state is not STARTED
            if (state != ThresholdState::STARTED) continue;

            // check that after mining this block stats change as expected
            const BIP9Stats stats = checker.GetStateStatisticsFor(current_block);
            assert(stats.period == period);
            assert(stats.threshold == threshold);
            assert(stats.elapsed == b);
            assert(stats.count == last_stats.count + (signal ? 1 : 0));
            assert(stats.possible == (stats.count + period >= stats.elapsed + threshold));
            last_stats = stats;
        }

        if (exp_state == ThresholdState::STARTED) {
            // double check that stats.possible is sane
            if (blocks_sig >= threshold - 1) assert(last_stats.possible);
        }

        // mine the final block
        bool signal = (signalling_mask >> (period % 32)) & 1;
        if (signal) ++blocks_sig;
        CBlockIndex* current_block = blocks.mine_block(signal);
        assert(checker.Condition(current_block) == signal);

        // GetStateStatistics is safe on a period boundary
        // and has progressed to a new period
        const BIP9Stats stats = checker.GetStateStatisticsFor(current_block);
        assert(stats.period == period);
        assert(stats.threshold == threshold);
        assert(stats.elapsed == 0);
        assert(stats.count == 0);
        assert(stats.possible == true);

        // More interesting is whether the state changed.
        const ThresholdState state = checker.GetStateFor(current_block);
        const int since = checker.GetStateSinceHeightFor(current_block);

        // since is straightforward:
        assert(since % period == 0);
        assert(0 <= since && since <= current_block->nHeight + 1);
        if (state == exp_state) {
            assert(since == exp_since);
        } else {
            assert(since == current_block->nHeight + 1);
        }

        // state is where everything interesting is
        switch (state) {
        case ThresholdState::DEFINED:
            assert(since == 0);
            assert(exp_state == ThresholdState::DEFINED);
            assert(current_block->GetMedianTimePast() < checker.m_begin);
            break;
        case ThresholdState::STARTED:
            assert(current_block->GetMedianTimePast() >= checker.m_begin);
            if (exp_state == ThresholdState::STARTED) {
                assert(blocks_sig < threshold);
                assert(current_block->GetMedianTimePast() < checker.m_end);
            } else {
                assert(exp_state == ThresholdState::DEFINED);
            }
            break;
        case ThresholdState::LOCKED_IN:
            if (exp_state == ThresholdState::LOCKED_IN) {
                assert(current_block->nHeight + 1 < min_activation);
            } else {
                assert(exp_state == ThresholdState::STARTED);
                assert(blocks_sig >= threshold);
            }
            break;
        case ThresholdState::ACTIVE:
            assert(always_active_test || min_activation <= current_block->nHeight + 1);
            assert(exp_state == ThresholdState::ACTIVE || exp_state == ThresholdState::LOCKED_IN);
            break;
        case ThresholdState::FAILED:
            assert(never_active_test || current_block->GetMedianTimePast() >= checker.m_end);
            if (exp_state == ThresholdState::STARTED) {
                assert(blocks_sig < threshold);
            } else {
                assert(exp_state == ThresholdState::FAILED);
            }
            break;
        default:
            assert(false);
        }

        if (blocks.size() >= period * max_periods) {
            // we chose the timeout (and block times) so that by the time we have this many blocks it's all over
            assert(state == ThresholdState::ACTIVE || state == ThresholdState::FAILED);
        }

        if (always_active_test) {
            // "always active" has additional restrictions
            assert(state == ThresholdState::ACTIVE);
            assert(exp_state == ThresholdState::ACTIVE);
            assert(since == 0);
        } else if (never_active_test) {
            // "never active" does too
            assert(state == ThresholdState::FAILED);
            assert(exp_state == ThresholdState::FAILED);
            assert(since == 0);
        } else {
            // for signalled deployments, the initial state is always DEFINED
            assert(since > 0 || state == ThresholdState::DEFINED);
            assert(exp_since > 0 || exp_state == ThresholdState::DEFINED);
        }

    */
}
