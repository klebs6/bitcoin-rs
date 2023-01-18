crate::ix!();

pub fn check_legacy_txindex(
    block_tree_db: &mut BlockTreeDB) -> Option<BilingualStr> {
    
    todo!();
        /*
            BlockLocator ignored{};
        if (block_tree_db.Read(DB_TXINDEX_BLOCK, ignored)) {
            return _("The -txindex upgrade started by a previous version can not be completed. Restart with the previous version or run a full -reindex.");
        }
        bool txindex_legacy_flag{false};
        block_tree_db.ReadFlag("txindex", txindex_legacy_flag);
        if (txindex_legacy_flag) {
            // Disable legacy txindex and warn once about occupied disk space
            if (!block_tree_db.WriteFlag("txindex", false)) {
                return Untranslated("Failed to write block index db flag 'txindex'='0'");
            }
            return _("The block index db contains a legacy 'txindex'. To clear the occupied disk space, run a full -reindex, otherwise ignore this error. This error message will not be displayed again.");
        }
        return std::nullopt;
        */
}


