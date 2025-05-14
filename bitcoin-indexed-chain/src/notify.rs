// ---------------- [ File: bitcoin-indexed-chain/src/notify.rs ]
crate::ix!();

#[LOCKS_EXCLUDED(cs_main)]
pub fn notify_header_tip(chainstate: &mut ChainState) -> bool {
    
    todo!();
        /*
            bool fNotify = false;
        bool fInitialBlockDownload = false;
        static CBlockIndex* pindexHeaderOld = nullptr;
        CBlockIndex* pindexHeader = nullptr;
        {
            LOCK(cs_main);
            pindexHeader = pindexBestHeader;

            if (pindexHeader != pindexHeaderOld) {
                fNotify = true;
                fInitialBlockDownload = chainstate.IsInitialBlockDownload();
                pindexHeaderOld = pindexHeader;
            }
        }
        // Send block tip changed notifications without cs_main
        if (fNotify) {
            uiInterface.NotifyHeaderTip(GetSynchronizationState(fInitialBlockDownload), pindexHeader);
        }
        return fNotify;
        */
}

