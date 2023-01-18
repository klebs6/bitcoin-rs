crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Write estimation data to a file
      |
      */
    pub fn write(&self, fileout: &mut AutoFile) -> bool {
        
        todo!();
        /*
            try {
            LOCK(m_cs_fee_estimator);
            fileout << 149900; // version required to read: 0.14.99 or later
            fileout << CLIENT_VERSION; // version that wrote the file
            fileout << nBestSeenHeight;
            if (BlockSpan() > HistoricalBlockSpan()/2) {
                fileout << firstRecordedHeight << nBestSeenHeight;
            }
            else {
                fileout << historicalFirst << historicalBest;
            }
            fileout << Using<VectorFormatter<EncodedDoubleFormatter>>(buckets);
            feeStats->Write(fileout);
            shortStats->Write(fileout);
            longStats->Write(fileout);
        }
        catch (const std::exception&) {
            LogPrintf("CBlockPolicyEstimator::Write(): unable to write policy estimator data (non-fatal)\n");
            return false;
        }
        return true;
        */
    }
}
