// ---------------- [ File: bitcoinleveldb-compactionstats/src/stats.rs ]
crate::ix!();

/// Per level compaction stats. stats_[level] stores the stats for compactions
/// that produced data for the specified "level".
/// 
#[derive(Getters,MutGetters,Setters,Builder)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
#[builder(pattern = "owned")]
pub struct CompactionStats {
    micros:        i64,
    bytes_read:    i64,
    bytes_written: i64,
}

impl Default for CompactionStats {

    fn default() -> Self {
        Self {
            micros:        0,
            bytes_read:    0,
            bytes_written: 0,
        }
    }
}

impl CompactionStats {

    pub fn add(&mut self, c: &CompactionStats)  {
        self.micros        = self.micros.saturating_add(c.micros);
        self.bytes_read    = self.bytes_read.saturating_add(c.bytes_read);
        self.bytes_written = self.bytes_written.saturating_add(c.bytes_written);
    }
}
