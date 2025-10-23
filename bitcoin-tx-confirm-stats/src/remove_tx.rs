// ---------------- [ File: bitcoin-tx-confirm-stats/src/remove_tx.rs ]
crate::ix!();

impl TxConfirmStats {
    
    /**
      | Remove a transaction from mempool tracking
      | stats
      |
      */
    pub fn remove_tx(
        &mut self,
        entry_height:       u32,
        n_best_seen_height: u32,
        bucketindex:        u32,
        in_block:           bool,
    ) {
        let b = bucketindex as usize;

        // blocksAgo may be 0 if no blocks seen yet
        let mut blocks_ago = n_best_seen_height as i32 - entry_height as i32;
        if n_best_seen_height == 0 {
            blocks_ago = 0;
        }
        if blocks_ago < 0 {
            trace!("Blockpolicy: blocks_ago negative; ignoring remove_tx");
            return;
        }

        let bins = self.unconf_txs().len();
        if (blocks_ago as usize) >= bins {
            if self.old_unconf_txs()[b] > 0 {
                self.old_unconf_txs_mut()[b] -= 1;
            } else {
                trace!("Blockpolicy: mempool tx removed from > bins already (bucket={})", b);
            }
        } else {
            let block_idx = (entry_height as usize) % bins;
            if self.unconf_txs()[block_idx][b] > 0 {
                self.unconf_txs_mut()[block_idx][b] -= 1;
            } else {
                trace!("Blockpolicy: mempool tx double-removed (block_idx={}, bucket={})", block_idx, b);
            }
        }

        if !in_block && (blocks_ago as u32) >= *self.scale() {
            let periods_ago = (blocks_ago as u32) / self.scale();
            for i in 0..periods_ago.min(self.fail_avg().len() as u32) as usize {
                self.fail_avg_mut()[i][b] += 1.0;
            }
        }
    }
}

#[cfg(test)]
mod remove_tx_spec {
    use super::*;

    fn mk_stats() -> TxConfirmStats {
        let buckets = vec![1.0, 2.0, 3.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 3, 0.0, 2);
        // bins = periods * scale = 3 * 2 = 6
        s
    }

    #[traced_test]
    fn negative_blocks_ago_is_ignored() {
        let mut s = mk_stats();
        // Put one tx in time bin at entry_height%bins
        let bins = s.unconf_txs().len();
        let entry_height = 10u32;
        let bucket = 1u32;
        let idx = (entry_height as usize) % bins;
        s.unconf_txs_mut()[idx][bucket as usize] = 1;

        // n_best_seen_height < entry_height => blocks_ago < 0 => ignored
        s.remove_tx(entry_height, entry_height - 1, bucket, true);

        // unchanged
        assert_eq!(s.unconf_txs()[idx][bucket as usize], 1);
    }

    #[traced_test]
    fn removes_from_old_unconf_when_older_than_ring() {
        let mut s = mk_stats();
        let b = 2usize;
        s.old_unconf_txs_mut()[b] = 2; // will be decremented if blocks_ago >= bins

        let bins = s.unconf_txs().len();
        s.remove_tx(/*entry_height*/ 1, /*best*/ (bins as u32) + 2, b as u32, false);
        assert_eq!(s.old_unconf_txs()[b], 1);
    }

    #[traced_test]
    fn removes_from_ring_and_increments_fail_avg_when_not_in_block_after_scale() {
        let mut s = mk_stats();
        let b = 0usize;

        let bins = s.unconf_txs().len();
        let entry_height = 3u32;
        let idx = (entry_height as usize) % bins;
        s.unconf_txs_mut()[idx][b] = 1;

        // Use blocks_ago = 4 (< bins=6) to stay in the ring; with scale=2 => periods_ago=2
        s.remove_tx(entry_height, entry_height + 4, b as u32, false);

        assert_eq!(s.unconf_txs()[idx][b], 0);
        // fail_avg[i][b] incremented for i = 0..periods_ago (i.e., rows 0 and 1)
        assert_eq!(s.fail_avg()[0][b], 1.0);
        assert_eq!(s.fail_avg()[1][b], 1.0);
        assert_eq!(s.fail_avg()[2][b], 0.0);
    }
}
