// ---------------- [ File: bitcoin-blockpolicy/src/read.rs ]
crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Read estimation data from a file
      |
      */
    pub fn read(&mut self, filein: &mut AutoFile) -> bool {
        
        todo!();
        /*
            try {
            LOCK(m_cs_fee_estimator);
            int nVersionRequired, nVersionThatWrote;
            filein >> nVersionRequired >> nVersionThatWrote;
            if (nVersionRequired > CLIENT_VERSION) {
                throw std::runtime_error(strprintf("up-version (%d) fee estimate file", nVersionRequired));
            }

            // Read fee estimates file into temporary variables so existing data
            // structures aren't corrupted if there is an exception.
            unsigned int nFileBestSeenHeight;
            filein >> nFileBestSeenHeight;

            if (nVersionRequired < 149900) {
                LogPrintf("%s: incompatible old fee estimation data (non-fatal). Version: %d\n", __func__, nVersionRequired);
            } else { // New format introduced in 149900
                unsigned int nFileHistoricalFirst, nFileHistoricalBest;
                filein >> nFileHistoricalFirst >> nFileHistoricalBest;
                if (nFileHistoricalFirst > nFileHistoricalBest || nFileHistoricalBest > nFileBestSeenHeight) {
                    throw std::runtime_error("Corrupt estimates file. Historical block range for estimates is invalid");
                }
                std::vector<double> fileBuckets;
                filein >> Using<VectorFormatter<EncodedDoubleFormatter>>(fileBuckets);
                size_t numBuckets = fileBuckets.size();
                if (numBuckets <= 1 || numBuckets > 1000) {
                    throw std::runtime_error("Corrupt estimates file. Must have between 2 and 1000 feerate buckets");
                }

                std::unique_ptr<TxConfirmStats> fileFeeStats(new TxConfirmStats(buckets, bucketMap, MED_BLOCK_PERIODS, MED_DECAY, MED_SCALE));
                std::unique_ptr<TxConfirmStats> fileShortStats(new TxConfirmStats(buckets, bucketMap, SHORT_BLOCK_PERIODS, SHORT_DECAY, SHORT_SCALE));
                std::unique_ptr<TxConfirmStats> fileLongStats(new TxConfirmStats(buckets, bucketMap, LONG_BLOCK_PERIODS, LONG_DECAY, LONG_SCALE));
                fileFeeStats->Read(filein, nVersionThatWrote, numBuckets);
                fileShortStats->Read(filein, nVersionThatWrote, numBuckets);
                fileLongStats->Read(filein, nVersionThatWrote, numBuckets);

                // Fee estimates file parsed correctly
                // Copy buckets from file and refresh our bucketmap
                buckets = fileBuckets;
                bucketMap.clear();
                for (unsigned int i = 0; i < buckets.size(); i++) {
                    bucketMap[buckets[i]] = i;
                }

                // Destroy old TxConfirmStats and point to new ones that already reference buckets and bucketMap
                feeStats = std::move(fileFeeStats);
                shortStats = std::move(fileShortStats);
                longStats = std::move(fileLongStats);

                nBestSeenHeight = nFileBestSeenHeight;
                historicalFirst = nFileHistoricalFirst;
                historicalBest = nFileHistoricalBest;
            }
        }
        catch (const std::exception& e) {
            LogPrintf("CBlockPolicyEstimator::Read(): unable to read policy estimator data (non-fatal): %s\n",e.what());
            return false;
        }
        return true;
        */
    }
}
