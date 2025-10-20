// ---------------- [ File: bitcoin-blockpolicy/src/flush.rs ]
crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Empty mempool transactions on shutdown
      | to record failure to confirm for txs
      | still in mempool
      |
      */
    pub fn flush_unconfirmed(&mut self)  {
        
        todo!();
        /*
            int64_t startclear = GetTimeMicros();
        LOCK(m_cs_fee_estimator);
        size_t num_entries = mapMemPoolTxs.size();
        // Remove every entry in mapMemPoolTxs
        while (!mapMemPoolTxs.empty()) {
            auto mi = mapMemPoolTxs.begin();
            removeTx(mi->first, false); // this calls erase() on mapMemPoolTxs
        }
        int64_t endclear = GetTimeMicros();
        LogPrint(LogFlags::ESTIMATEFEE, "Recorded %u unconfirmed txs from mempool in %gs\n", num_entries, (endclear - startclear)*0.000001);
        */
    }

    /**
      | Drop still unconfirmed transactions
      | and record current estimations, if
      | the fee estimation file is present.
      |
      */
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            FlushUnconfirmed();

        fs::path est_filepath = gArgs.GetDataDirNet() / FEE_ESTIMATES_FILENAME;
        CAutoFile est_file(fsbridge::fopen(est_filepath, "wb"), SER_DISK, CLIENT_VERSION);
        if (est_file.IsNull() || !Write(est_file)) {
            LogPrintf("Failed to write fee estimates to %s. Continue anyway.\n", fs::PathToString(est_filepath));
        }
        */
    }
}
