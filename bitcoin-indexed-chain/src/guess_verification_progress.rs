// ---------------- [ File: bitcoin-indexed-chain/src/guess_verification_progress.rs ]
crate::ix!();

/**
  | Guess verification progress (as a fraction
  | between 0.0=genesis and 1.0=current
  | tip).
  |
  -----------------
  | Guess how far we are in the verification
  | process at the given block index require
  | cs_main if pindex has not been validated yet
  | (because nChainTx might be unset)
  */
pub fn guess_verification_progress(
        data:   &ChainTxData,
        pindex: Arc<BlockIndex>) -> f64 {
    
    todo!();
        /*
            if (pindex == nullptr)
            return 0.0;

        int64_t nNow = time(nullptr);

        double fTxTotal;

        if (pindex->nChainTx <= data.nTxCount) {
            fTxTotal = data.nTxCount + (nNow - data.nTime) * data.dTxRate;
        } else {
            fTxTotal = pindex->nChainTx + (nNow - pindex->GetBlockTime()) * data.dTxRate;
        }

        return std::min<double>(pindex->nChainTx / fTxTotal, 1.0);
        */
}
