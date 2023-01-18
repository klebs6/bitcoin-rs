crate::ix!();

/** 
  | \class BlockPolicyEstimator
  |
  | The BlockPolicyEstimator is used for estimating
  | the feerate needed for a transaction to be
  | included in a block within a certain number of
  | blocks.
  |
  | ----------------------
  | At a high level the algorithm works by grouping
  | transactions into buckets based on having
  | similar feerates and then tracking how long it
  | takes transactions in the various buckets to be
  | mined.  It operates under the assumption that
  | in general transactions of higher feerate will
  | be included in blocks before transactions of
  | lower feerate.   
  |
  | So for example if you wanted to know what
  | feerate you should put on a transaction to be
  | included in a block within the next 5 blocks,
  | you would start by looking at the bucket with
  | the highest feerate transactions and verifying
  | that a sufficiently high percentage of them
  | were confirmed within 5 blocks and then you
  | would look at the next highest feerate bucket,
  | and so on, stopping at the last bucket to pass
  | the test.   
  |
  | The average feerate of transactions in this
  | bucket will give you an indication of the
  | lowest feerate you can put on a transaction and
  | still have a sufficiently high chance of being
  | confirmed within your desired 5 blocks.
  |
  | ----------------------
  | Here is a brief description of the
  | implementation: When a transaction enters the
  | mempool, we track the height of the block chain
  | at entry.  All further calculations are
  | conducted only on this set of "seen"
  | transactions. 
  |
  | Whenever a block comes in, we count the number
  | of transactions in each bucket and the total
  | amount of feerate paid in each bucket. Then we
  | calculate how many blocks Y it took each
  | transaction to be mined.  We convert from
  | a number of blocks to a number of periods Y'
  | each encompassing "scale" blocks.  
  |
  | This is tracked in 3 different data sets each
  | up to a maximum number of periods. Within each
  | data set we have an array of counters in each
  | feerate bucket and we increment all the
  | counters from Y' up to max periods representing
  | that a tx was successfully confirmed in less
  | than or equal to that many periods. 
  |
  | We want to save a history of this information,
  | so at any time we have a counter of the total
  | number of transactions that happened in a given
  | feerate bucket and the total number that were
  | confirmed in each of the periods or less for
  | any bucket.  
  |
  | We save this history by keeping an
  | exponentially decaying moving average of each
  | one of these stats.  This is done for
  | a different decay in each of the 3 data sets to
  | keep relevant data from different time
  | horizons.  
  |
  | Furthermore we also keep track of the number
  | unmined (in mempool or left mempool without
  | being included in a block) transactions in
  | each bucket and for how many blocks they have
  | been outstanding and use both of these numbers
  | to increase the number of transactions we've
  | seen in that feerate bucket when calculating an
  | estimate for any number of confirmations below
  | the number of blocks they've been outstanding.
  |
  | --------------------
  | We want to be able to estimate feerates that
  | are needed on tx's to be included in a certain
  | number of blocks.  Every time a block is added
  | to the best chain, this class records stats on
  | the transactions included in that block
  */
pub struct BlockPolicyEstimator {
    pub cs_fee_estimator: Arc<Mutex<BlockPolicyEstimatorInner>>,
}

pub struct BlockPolicyEstimatorInner {

    pub n_best_seen_height:    u32,
    pub first_recorded_height: u32,
    pub historical_first:      u32,
    pub historical_best:       u32,

    /**
      | map of txids to information about that
      | transaction
      |
      */
    pub map_mem_pool_txs:      HashMap<u256,TxStatsInfo>,

    /*
      | Classes to track historical data on
      | transaction confirmations
      |
      */
    pub fee_stats:     Box<TxConfirmStats>,
    pub short_stats:   Box<TxConfirmStats>,
    pub long_stats:    Box<TxConfirmStats>,
    pub tracked_txs:   u32,
    pub untracked_txs: u32,

    /**
      | The upper-bound of the range for the
      | bucket (inclusive)
      |
      */
    pub buckets:       Vec<f64>,

    /**
      | Map of bucket upper-bound to index into
      | all vectors by bucket
      |
      */
    pub bucket_map:    HashMap<f64,u32>,
}

impl Default for BlockPolicyEstimator {
    
    /**
      | Create new BlockPolicyEstimator and
      | initialize stats tracking classes
      | with default values
      |
      */
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}

impl BlockPolicyEstimator {

    pub fn new() -> Self {
    
        todo!();
        /*
        : n_best_seen_height(0),
        : first_recorded_height(0),
        : historical_first(0),
        : historical_best(0),
        : tracked_txs(0),
        : untracked_txs(0),

            const_assert(MIN_BUCKET_FEERATE > 0, "Min feerate must be nonzero");
        size_t bucketIndex = 0;

        for (double bucketBoundary = MIN_BUCKET_FEERATE; bucketBoundary <= MAX_BUCKET_FEERATE; bucketBoundary *= FEE_SPACING, bucketIndex++) {
            buckets.push_back(bucketBoundary);
            bucketMap[bucketBoundary] = bucketIndex;
        }
        buckets.push_back(INF_FEERATE);
        bucketMap[INF_FEERATE] = bucketIndex;
        assert(bucketMap.size() == buckets.size());

        feeStats = std::unique_ptr<TxConfirmStats>(new TxConfirmStats(buckets, bucketMap, MED_BLOCK_PERIODS, MED_DECAY, MED_SCALE));
        shortStats = std::unique_ptr<TxConfirmStats>(new TxConfirmStats(buckets, bucketMap, SHORT_BLOCK_PERIODS, SHORT_DECAY, SHORT_SCALE));
        longStats = std::unique_ptr<TxConfirmStats>(new TxConfirmStats(buckets, bucketMap, LONG_BLOCK_PERIODS, LONG_DECAY, LONG_SCALE));

        // If the fee estimation file is present, read recorded estimations
        fs::path est_filepath = gArgs.GetDataDirNet() / FEE_ESTIMATES_FILENAME;
        CAutoFile est_file(fsbridge::fopen(est_filepath, "rb"), SER_DISK, CLIENT_VERSION);
        if (est_file.IsNull() || !Read(est_file)) {
            LogPrintf("Failed to read fee estimates from %s. Continue anyway.\n", fs::PathToString(est_filepath));
        }
        */
    }
}
