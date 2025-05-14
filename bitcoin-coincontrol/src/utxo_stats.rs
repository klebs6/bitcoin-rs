// ---------------- [ File: bitcoin-coincontrol/src/utxo_stats.rs ]
crate::ix!();

/**
  | Calculate statistics about the unspent
  | transaction output set
  |
  */
pub fn get_utxo_stats_with_hash_obj<'a, T>(
        view:               &'a mut dyn CoinsView,
        blockman:           &mut BlockManager,
        stats:              &mut CoinsStats,
        hash_obj:           T,
        interruption_point: &fn() -> (),
        pindex:             *const BlockIndex) -> bool {

    todo!();
        /*
            std::unique_ptr<CCoinsViewCursor> pcursor(view->Cursor());
        assert(pcursor);

        if (!pindex) {
            {
                LOCK(cs_main);
                pindex = blockman.LookupBlockIndex(view->GetBestBlock());
            }
        }
        stats.nHeight = Assert(pindex)->nHeight;
        stats.hashBlock = pindex->GetBlockHash();

        // Use CoinStatsIndex if it is requested and available and a hash_type of Muhash or None was requested
        if ((stats.m_hash_type == CoinStatsHashType::MUHASH || stats.m_hash_type == CoinStatsHashType::NONE) && g_coin_stats_index && stats.index_requested) {
            stats.index_used = true;
            return g_coin_stats_index->LookUpStats(pindex, stats);
        }

        PrepareHash(hash_obj, stats);

        uint256 prevkey;
        std::map<uint32_t, Coin> outputs;
        while (pcursor->Valid()) {
            interruption_point();
            OutPoint key;
            Coin coin;
            if (pcursor->GetKey(key) && pcursor->GetValue(coin)) {
                if (!outputs.empty() && key.hash != prevkey) {
                    ApplyStats(stats, prevkey, outputs);
                    ApplyHash(hash_obj, prevkey, outputs);
                    outputs.clear();
                }
                prevkey = key.hash;
                outputs[key.n] = std::move(coin);
                stats.coins_count++;
            } else {
                return error("%s: unable to read value", __func__);
            }
            pcursor->Next();
        }
        if (!outputs.empty()) {
            ApplyStats(stats, prevkey, outputs);
            ApplyHash(hash_obj, prevkey, outputs);
        }

        FinalizeHash(hash_obj, stats);

        stats.nDiskSize = view->EstimateSize();
        return true;
        */
}

/**
  | Calculate statistics about the unspent
  | transaction output set
  |
  */
pub fn get_utxo_stats(
        view:               Rc<RefCell<dyn CoinsView>>,
        blockman:           &mut BlockManager,
        stats:              &mut CoinsStats,
        interruption_point: Option<fn() -> ()>,
        pindex:             Option<*const BlockIndex>) -> bool {

    todo!();
        /*
            switch (stats.m_hash_type) {
        case(CoinStatsHashType::HASH_SERIALIZED): {
            CHashWriter ss(SER_GETHASH, PROTOCOL_VERSION);
            return GetUTXOStats(view, blockman, stats, ss, interruption_point, pindex);
        }
        case(CoinStatsHashType::MUHASH): {
            MuHash3072 muhash;
            return GetUTXOStats(view, blockman, stats, muhash, interruption_point, pindex);
        }
        case(CoinStatsHashType::NONE): {
            return GetUTXOStats(view, blockman, stats, nullptr, interruption_point, pindex);
        }
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
}
