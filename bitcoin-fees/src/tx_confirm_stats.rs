// ---------------- [ File: bitcoin-fees/src/tx_confirm_stats.rs ]
crate::ix!();

/**
  | We will instantiate an instance of this
  | class to track transactions that were
  | included in a block. We will lump transactions
  | into a bucket according to their approximate
  | feerate and then track how long it took
  | for those txs to be included in a block
  | 
  | The tracking of unconfirmed (mempool)
  | transactions is completely independent
  | of the historical tracking of transactions
  | that have been confirmed in a block.
  |
  */
pub struct TxConfirmStats {

    /**
      | Define the buckets we will group transactions
      | into
      | 
      | The upper-bound of the range for the
      | bucket (inclusive)
      |
      */
    buckets:        Arc<Vec<f64>>,

    /**
      | Map of bucket upper-bound to index into
      | all vectors by bucket
      |
      */
    bucket_map:     Arc<HashMap<f64,u32>>,

    /**
      | For each bucket X:
      | 
      | Count the total # of txs in each bucket
      | 
      | Track the historical moving average
      | of this total over blocks
      |
      */
    tx_ct_avg:      Vec<f64>,

    /**
      | Count the total # of txs confirmed within
      | 
      | Y blocks in each bucket
      | 
      | Track the historical moving average
      | of these totals over blocks confAvg[Y][X]
      |
      */
    conf_avg:       Vec<Vec<f64>>,

    /**
      | Track moving avg of txs which have been
      | evicted from the mempool after failing
      | to be confirmed within Y blocks failAvg[Y][X]
      |
      */
    fail_avg:       Vec<Vec<f64>>,

    /**
      | Sum the total feerate of all tx's in each
      | bucket
      | 
      | Track the historical moving average
      | of this total over blocks
      |
      */
    feerate_avg:    Vec<f64>,

    /**
      | Combine the conf counts with tx counts
      | to calculate the confirmation % for
      | each Y,X
      | 
      | Combine the total value with the tx counts
      | to calculate the avg feerate per bucket
      |
      */
    decay:          f64,

    /**
      | Resolution (# of blocks) with which
      | confirmations are tracked
      |
      */
    scale:          u32,

    /**
      | Mempool counts of outstanding transactions
      |
      | For each bucket X, track the number of
      | transactions in the mempool that are
      | unconfirmed for each possible confirmation
      | value Y unconfTxs[Y][X]
      */
    unconf_txs:     Vec<Vec<i32>>,

    /**
      | transactions still unconfirmed after
      | 
      | GetMaxConfirms for each bucket
      |
      */
    old_unconf_txs: Vec<i32>,
}

impl TxConfirmStats {

    /**
      | Return the max number of confirms we're
      | tracking
      |
      */
    pub fn get_max_confirms(&self) -> u32 {
        
        todo!();
        /*
            return scale * confAvg.size();
        */
    }

    /**
      | Create new TxConfirmStats. This is
      | called by BlockPolicyEstimator's
      | constructor with default values.
      | 
      | -----------
      | @param defaultBuckets
      | 
      | contains the upper limits for the bucket
      | boundaries
      | ----------
      | @param maxPeriods
      | 
      | max number of periods to track
      | ----------
      | @param decay
      | 
      | how much to decay the historical moving
      | average per block
      |
      */
    pub fn new(
        default_buckets:    &Vec<f64>,
        default_bucket_map: &HashMap<f64,u32>,
        max_periods:        u32,
        decay:              f64,
        scale:              u32) -> Self {
    
        todo!();
        /*


            : buckets(defaultBuckets), bucketMap(defaultBucketMap), decay(_decay), scale(_scale)

        assert(_scale != 0 && "_scale must be non-zero");
        confAvg.resize(maxPeriods);
        failAvg.resize(maxPeriods);
        for (unsigned int i = 0; i < maxPeriods; i++) {
            confAvg[i].resize(buckets.size());
            failAvg[i].resize(buckets.size());
        }

        txCtAvg.resize(buckets.size());
        m_feerate_avg.resize(buckets.size());

        resizeInMemoryCounters(buckets.size());
        */
    }
    
    pub fn resize_in_memory_counters(&mut self, newbuckets: usize)  {
        
        todo!();
        /*
            // newbuckets must be passed in because the buckets referred to during Read have not been updated yet.
        unconfTxs.resize(GetMaxConfirms());
        for (unsigned int i = 0; i < unconfTxs.size(); i++) {
            unconfTxs[i].resize(newbuckets);
        }
        oldUnconfTxs.resize(newbuckets);
        */
    }

    /**
      | Roll the circular buffer for unconfirmed
      | txs
      |
      */
    pub fn clear_current(&mut self, n_block_height: u32)  {
        
        todo!();
        /*
            for (unsigned int j = 0; j < buckets.size(); j++) {
            oldUnconfTxs[j] += unconfTxs[nBlockHeight % unconfTxs.size()][j];
            unconfTxs[nBlockHeight%unconfTxs.size()][j] = 0;
        }
        */
    }
    
    /**
      | Record a new transaction data point
      | in the current block stats
      | 
      | -----------
      | @param blocksToConfirm
      | 
      | the number of blocks it took this transaction
      | to confirm
      | ----------
      | @param val
      | 
      | the feerate of the transaction
      | 
      | -----------
      | @warning
      | 
      | blocksToConfirm is 1-based and has
      | to be >= 1
      |
      */
    pub fn record(&mut self, 
        blocks_to_confirm: i32,
        feerate:           f64)  {
        
        todo!();
        /*
            // blocksToConfirm is 1-based
        if (blocksToConfirm < 1)
            return;
        int periodsToConfirm = (blocksToConfirm + scale - 1) / scale;
        unsigned int bucketindex = bucketMap.lower_bound(feerate)->second;
        for (size_t i = periodsToConfirm; i <= confAvg.size(); i++) {
            confAvg[i - 1][bucketindex]++;
        }
        txCtAvg[bucketindex]++;
        m_feerate_avg[bucketindex] += feerate;
        */
    }
    
    /**
      | Update our estimates by decaying our
      | historical moving average and updating
      | with the data gathered from the current
      | block
      |
      */
    pub fn update_moving_averages(&mut self)  {
        
        todo!();
        /*
            assert(confAvg.size() == failAvg.size());
        for (unsigned int j = 0; j < buckets.size(); j++) {
            for (unsigned int i = 0; i < confAvg.size(); i++) {
                confAvg[i][j] *= decay;
                failAvg[i][j] *= decay;
            }
            m_feerate_avg[j] *= decay;
            txCtAvg[j] *= decay;
        }
        */
    }

    /**
      | Calculate a feerate estimate. Find
      | the lowest value bucket (or range of
      | buckets to make sure we have enough data
      | points) whose transactions still have
      | sufficient likelihood of being confirmed
      | within the target number of confirmations
      | 
      | -----------
      | @param confTarget
      | 
      | target number of confirmations
      | ----------
      | @param sufficientTxVal
      | 
      | required average number of transactions
      | per block in a bucket range
      | ----------
      | @param minSuccess
      | 
      | the success probability we require
      | ----------
      | @param nBlockHeight
      | 
      | the current block height
      |
      | returns -1 on error conditions
      |
      */
    pub fn estimate_median_val(&self, 
        conf_target:         i32,
        sufficient_tx_val:   f64,
        success_break_point: f64,
        n_block_height:      u32,
        result:              *mut EstimationResult) -> f64 {
        
        todo!();
        /*
            // Counters for a bucket (or range of buckets)
        double nConf = 0; // Number of tx's confirmed within the confTarget
        double totalNum = 0; // Total number of tx's that were ever confirmed
        int extraNum = 0;  // Number of tx's still in mempool for confTarget or longer
        double failNum = 0; // Number of tx's that were never confirmed but removed from the mempool after confTarget
        const int periodTarget = (confTarget + scale - 1) / scale;
        const int maxbucketindex = buckets.size() - 1;

        // We'll combine buckets until we have enough samples.
        // The near and far variables will define the range we've combined
        // The best variables are the last range we saw which still had a high
        // enough confirmation rate to count as success.
        // The cur variables are the current range we're counting.
        unsigned int curNearBucket = maxbucketindex;
        unsigned int bestNearBucket = maxbucketindex;
        unsigned int curFarBucket = maxbucketindex;
        unsigned int bestFarBucket = maxbucketindex;

        bool foundAnswer = false;
        unsigned int bins = unconfTxs.size();
        bool newBucketRange = true;
        bool passing = true;
        EstimatorBucket passBucket;
        EstimatorBucket failBucket;

        // Start counting from highest feerate transactions
        for (int bucket = maxbucketindex; bucket >= 0; --bucket) {
            if (newBucketRange) {
                curNearBucket = bucket;
                newBucketRange = false;
            }
            curFarBucket = bucket;
            nConf += confAvg[periodTarget - 1][bucket];
            totalNum += txCtAvg[bucket];
            failNum += failAvg[periodTarget - 1][bucket];
            for (unsigned int confct = confTarget; confct < GetMaxConfirms(); confct++)
                extraNum += unconfTxs[(nBlockHeight - confct) % bins][bucket];
            extraNum += oldUnconfTxs[bucket];
            // If we have enough transaction data points in this range of buckets,
            // we can test for success
            // (Only count the confirmed data points, so that each confirmation count
            // will be looking at the same amount of data and same bucket breaks)
            if (totalNum >= sufficientTxVal / (1 - decay)) {
                double curPct = nConf / (totalNum + failNum + extraNum);

                // Check to see if we are no longer getting confirmed at the success rate
                if (curPct < successBreakPoint) {
                    if (passing == true) {
                        // First time we hit a failure record the failed bucket
                        unsigned int failMinBucket = std::min(curNearBucket, curFarBucket);
                        unsigned int failMaxBucket = std::max(curNearBucket, curFarBucket);
                        failBucket.start = failMinBucket ? buckets[failMinBucket - 1] : 0;
                        failBucket.end = buckets[failMaxBucket];
                        failBucket.withinTarget = nConf;
                        failBucket.totalConfirmed = totalNum;
                        failBucket.inMempool = extraNum;
                        failBucket.leftMempool = failNum;
                        passing = false;
                    }
                    continue;
                }
                // Otherwise update the cumulative stats, and the bucket variables
                // and reset the counters
                else {
                    failBucket = EstimatorBucket(); // Reset any failed bucket, currently passing
                    foundAnswer = true;
                    passing = true;
                    passBucket.withinTarget = nConf;
                    nConf = 0;
                    passBucket.totalConfirmed = totalNum;
                    totalNum = 0;
                    passBucket.inMempool = extraNum;
                    passBucket.leftMempool = failNum;
                    failNum = 0;
                    extraNum = 0;
                    bestNearBucket = curNearBucket;
                    bestFarBucket = curFarBucket;
                    newBucketRange = true;
                }
            }
        }

        double median = -1;
        double txSum = 0;

        // Calculate the "average" feerate of the best bucket range that met success conditions
        // Find the bucket with the median transaction and then report the average feerate from that bucket
        // This is a compromise between finding the median which we can't since we don't save all tx's
        // and reporting the average which is less accurate
        unsigned int minBucket = std::min(bestNearBucket, bestFarBucket);
        unsigned int maxBucket = std::max(bestNearBucket, bestFarBucket);
        for (unsigned int j = minBucket; j <= maxBucket; j++) {
            txSum += txCtAvg[j];
        }
        if (foundAnswer && txSum != 0) {
            txSum = txSum / 2;
            for (unsigned int j = minBucket; j <= maxBucket; j++) {
                if (txCtAvg[j] < txSum)
                    txSum -= txCtAvg[j];
                else { // we're in the right bucket
                    median = m_feerate_avg[j] / txCtAvg[j];
                    break;
                }
            }

            passBucket.start = minBucket ? buckets[minBucket-1] : 0;
            passBucket.end = buckets[maxBucket];
        }

        // If we were passing until we reached last few buckets with insufficient data, then report those as failed
        if (passing && !newBucketRange) {
            unsigned int failMinBucket = std::min(curNearBucket, curFarBucket);
            unsigned int failMaxBucket = std::max(curNearBucket, curFarBucket);
            failBucket.start = failMinBucket ? buckets[failMinBucket - 1] : 0;
            failBucket.end = buckets[failMaxBucket];
            failBucket.withinTarget = nConf;
            failBucket.totalConfirmed = totalNum;
            failBucket.inMempool = extraNum;
            failBucket.leftMempool = failNum;
        }

        float passed_within_target_perc = 0.0;
        float failed_within_target_perc = 0.0;
        if ((passBucket.totalConfirmed + passBucket.inMempool + passBucket.leftMempool)) {
            passed_within_target_perc = 100 * passBucket.withinTarget / (passBucket.totalConfirmed + passBucket.inMempool + passBucket.leftMempool);
        }
        if ((failBucket.totalConfirmed + failBucket.inMempool + failBucket.leftMempool)) {
            failed_within_target_perc = 100 * failBucket.withinTarget / (failBucket.totalConfirmed + failBucket.inMempool + failBucket.leftMempool);
        }

        LogPrint(LogFlags::ESTIMATEFEE, "FeeEst: %d > %.0f%% decay %.5f: feerate: %g from (%g - %g) %.2f%% %.1f/(%.1f %d mem %.1f out) Fail: (%g - %g) %.2f%% %.1f/(%.1f %d mem %.1f out)\n",
                 confTarget, 100.0 * successBreakPoint, decay,
                 median, passBucket.start, passBucket.end,
                 passed_within_target_perc,
                 passBucket.withinTarget, passBucket.totalConfirmed, passBucket.inMempool, passBucket.leftMempool,
                 failBucket.start, failBucket.end,
                 failed_within_target_perc,
                 failBucket.withinTarget, failBucket.totalConfirmed, failBucket.inMempool, failBucket.leftMempool);

        if (result) {
            result->pass = passBucket;
            result->fail = failBucket;
            result->decay = decay;
            result->scale = scale;
        }
        return median;
        */
    }
    
    /**
      | Write state of estimation data to a file
      |
      */
    pub fn write(&self, fileout: &mut AutoFile)  {
        
        todo!();
        /*
            fileout << Using<EncodedDoubleFormatter>(decay);
        fileout << scale;
        fileout << Using<VectorFormatter<EncodedDoubleFormatter>>(m_feerate_avg);
        fileout << Using<VectorFormatter<EncodedDoubleFormatter>>(txCtAvg);
        fileout << Using<VectorFormatter<VectorFormatter<EncodedDoubleFormatter>>>(confAvg);
        fileout << Using<VectorFormatter<VectorFormatter<EncodedDoubleFormatter>>>(failAvg);
        */
    }
    
    /**
      | Read saved state of estimation data
      | from a file and replace all internal
      | data structures and variables with
      | this state.
      |
      */
    pub fn read(&mut self, 
        filein:         &mut AutoFile,
        n_file_version: i32,
        num_buckets:    usize)  {
        
        todo!();
        /*
            // Read data file and do some very basic sanity checking
        // buckets and bucketMap are not updated yet, so don't access them
        // If there is a read failure, we'll just discard this entire object anyway
        size_t maxConfirms, maxPeriods;

        // The current version will store the decay with each individual TxConfirmStats and also keep a scale factor
        filein >> Using<EncodedDoubleFormatter>(decay);
        if (decay <= 0 || decay >= 1) {
            throw std::runtime_error("Corrupt estimates file. Decay must be between 0 and 1 (non-inclusive)");
        }
        filein >> scale;
        if (scale == 0) {
            throw std::runtime_error("Corrupt estimates file. Scale must be non-zero");
        }

        filein >> Using<VectorFormatter<EncodedDoubleFormatter>>(m_feerate_avg);
        if (m_feerate_avg.size() != numBuckets) {
            throw std::runtime_error("Corrupt estimates file. Mismatch in feerate average bucket count");
        }
        filein >> Using<VectorFormatter<EncodedDoubleFormatter>>(txCtAvg);
        if (txCtAvg.size() != numBuckets) {
            throw std::runtime_error("Corrupt estimates file. Mismatch in tx count bucket count");
        }
        filein >> Using<VectorFormatter<VectorFormatter<EncodedDoubleFormatter>>>(confAvg);
        maxPeriods = confAvg.size();
        maxConfirms = scale * maxPeriods;

        if (maxConfirms <= 0 || maxConfirms > 6 * 24 * 7) { // one week
            throw std::runtime_error("Corrupt estimates file.  Must maintain estimates for between 1 and 1008 (one week) confirms");
        }
        for (unsigned int i = 0; i < maxPeriods; i++) {
            if (confAvg[i].size() != numBuckets) {
                throw std::runtime_error("Corrupt estimates file. Mismatch in feerate conf average bucket count");
            }
        }

        filein >> Using<VectorFormatter<VectorFormatter<EncodedDoubleFormatter>>>(failAvg);
        if (maxPeriods != failAvg.size()) {
            throw std::runtime_error("Corrupt estimates file. Mismatch in confirms tracked for failures");
        }
        for (unsigned int i = 0; i < maxPeriods; i++) {
            if (failAvg[i].size() != numBuckets) {
                throw std::runtime_error("Corrupt estimates file. Mismatch in one of failure average bucket counts");
            }
        }

        // Resize the current block variables which aren't stored in the data file
        // to match the number of confirms and buckets
        resizeInMemoryCounters(numBuckets);

        LogPrint(LogFlags::ESTIMATEFEE, "Reading estimates: %u buckets counting confirms up to %u blocks\n",
                 numBuckets, maxConfirms);
        */
    }
    
    /**
      | Record a new transaction entering the
      | mempool
      |
      */
    pub fn new_tx(&mut self, 
        n_block_height: u32,
        val:            f64) -> u32 {
        
        todo!();
        /*
            unsigned int bucketindex = bucketMap.lower_bound(val)->second;
        unsigned int blockIndex = nBlockHeight % unconfTxs.size();
        unconfTxs[blockIndex][bucketindex]++;
        return bucketindex;
        */
    }
    
    /**
      | Remove a transaction from mempool tracking
      | stats
      |
      */
    pub fn remove_tx(&mut self, 
        entry_height:       u32,
        n_best_seen_height: u32,
        bucketindex:        u32,
        in_block:           bool)  {
        
        todo!();
        /*
            //nBestSeenHeight is not updated yet for the new block
        int blocksAgo = nBestSeenHeight - entryHeight;
        if (nBestSeenHeight == 0)  // the BlockPolicyEstimator hasn't seen any blocks yet
            blocksAgo = 0;
        if (blocksAgo < 0) {
            LogPrint(LogFlags::ESTIMATEFEE, "Blockpolicy error, blocks ago is negative for mempool tx\n");
            return;  //This can't happen because we call this with our best seen height, no entries can have higher
        }

        if (blocksAgo >= (int)unconfTxs.size()) {
            if (oldUnconfTxs[bucketindex] > 0) {
                oldUnconfTxs[bucketindex]--;
            } else {
                LogPrint(LogFlags::ESTIMATEFEE, "Blockpolicy error, mempool tx removed from >25 blocks,bucketIndex=%u already\n",
                         bucketindex);
            }
        }
        else {
            unsigned int blockIndex = entryHeight % unconfTxs.size();
            if (unconfTxs[blockIndex][bucketindex] > 0) {
                unconfTxs[blockIndex][bucketindex]--;
            } else {
                LogPrint(LogFlags::ESTIMATEFEE, "Blockpolicy error, mempool tx removed from blockIndex=%u,bucketIndex=%u already\n",
                         blockIndex, bucketindex);
            }
        }
        if (!inBlock && (unsigned int)blocksAgo >= scale) { // Only counts as a failure if not confirmed for entire period
            assert(scale != 0);
            unsigned int periodsAgo = blocksAgo / scale;
            for (size_t i = 0; i < periodsAgo && i < failAvg.size(); i++) {
                failAvg[i][bucketindex]++;
            }
        }
        */
    }
}
