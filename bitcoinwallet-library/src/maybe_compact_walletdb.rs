// ---------------- [ File: bitcoinwallet-library/src/maybe_compact_walletdb.rs ]
crate::ix!();

/**
  | Compacts BDB state so that wallet.dat
  | is self-contained (if there are changes)
  |
  */
pub fn maybe_compact_walletdb(context: &mut WalletContext)  {
    
    todo!();
        /*
            static std::atomic<bool> fOneThread(false);
        if (fOneThread.exchange(true)) {
            return;
        }

        for (const std::shared_ptr<CWallet>& pwallet : GetWallets(context)) {
            WalletDatabase& dbh = pwallet->GetDatabase();

            unsigned int nUpdateCounter = dbh.nUpdateCounter;

            if (dbh.nLastSeen != nUpdateCounter) {
                dbh.nLastSeen = nUpdateCounter;
                dbh.nLastWalletUpdate = GetTime();
            }

            if (dbh.nLastFlushed != nUpdateCounter && GetTime() - dbh.nLastWalletUpdate >= 2) {
                if (dbh.PeriodicFlush()) {
                    dbh.nLastFlushed = nUpdateCounter;
                }
            }
        }

        fOneThread = false;
        */
}
