// ---------------- [ File: bitcoin-indexed-chain/src/load_mempool.rs ]
crate::ix!();

/**
  | Load the mempool from disk.
  |
  */
pub fn load_mempool(
    pool:                    &mut TxMemPool,
    active_chainstate:       &mut ChainState,
    mockable_fopen_function: Option<FopenFn>) -> bool {

    let mockable_fopen_function: FopenFn = mockable_fopen_function.unwrap_or(libc::fopen);

    todo!();
        /*
            const ChainParams& chainparams = Params();
        int64_t nExpiryTimeout = gArgs.GetIntArg("-mempoolexpiry", DEFAULT_MEMPOOL_EXPIRY) * 60 * 60;
        FILE* filestr{mockable_fopen_function(gArgs.GetDataDirNet() / "mempool.dat", "rb")};
        CAutoFile file(filestr, SER_DISK, CLIENT_VERSION);
        if (file.IsNull()) {
            LogPrintf("Failed to open mempool file from disk. Continuing anyway.\n");
            return false;
        }

        int64_t count = 0;
        int64_t expired = 0;
        int64_t failed = 0;
        int64_t already_there = 0;
        int64_t unbroadcast = 0;
        int64_t nNow = GetTime();

        try {
            uint64_t version;
            file >> version;
            if (version != MEMPOOL_DUMP_VERSION) {
                return false;
            }
            uint64_t num;
            file >> num;
            while (num--) {
                CTransactionRef tx;
                int64_t nTime;
                int64_t nFeeDelta;
                file >> tx;
                file >> nTime;
                file >> nFeeDelta;

                CAmount amountdelta = nFeeDelta;
                if (amountdelta) {
                    pool.PrioritiseTransaction(tx->GetHash(), amountdelta);
                }
                if (nTime > nNow - nExpiryTimeout) {
                    LOCK(cs_main);
                    if (AcceptToMemoryPoolWithTime(chainparams, pool, active_chainstate, tx, nTime, false /* bypass_limits */,
                                                   false /* test_accept */).m_result_type == MempoolAcceptResult::ResultType::VALID) {
                        ++count;
                    } else {
                        // mempool may contain the transaction already, e.g. from
                        // wallet(s) having loaded it while we were processing
                        // mempool transactions; consider these as valid, instead of
                        // failed, but mark them as 'already there'
                        if (pool.exists(GenTxId::Txid(tx->GetHash()))) {
                            ++already_there;
                        } else {
                            ++failed;
                        }
                    }
                } else {
                    ++expired;
                }
                if (ShutdownRequested())
                    return false;
            }
            std::map<uint256, CAmount> mapDeltas;
            file >> mapDeltas;

            for (const auto& i : mapDeltas) {
                pool.PrioritiseTransaction(i.first, i.second);
            }

            std::set<uint256> unbroadcast_txids;
            file >> unbroadcast_txids;
            unbroadcast = unbroadcast_txids.size();
            for (const auto& txid : unbroadcast_txids) {
                // Ensure transactions were accepted to mempool then add to
                // unbroadcast set.
                if (pool.get(txid) != nullptr) pool.AddUnbroadcastTx(txid);
            }
        } catch (const std::exception& e) {
            LogPrintf("Failed to deserialize mempool data on disk: %s. Continuing anyway.\n", e.what());
            return false;
        }

        LogPrintf("Imported mempool transactions from disk: %i succeeded, %i failed, %i expired, %i already there, %i waiting for initial broadcast\n", count, failed, expired, already_there, unbroadcast);
        return true;
        */
}
