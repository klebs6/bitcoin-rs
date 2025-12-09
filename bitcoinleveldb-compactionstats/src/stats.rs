// ---------------- [ File: bitcoinleveldb-compactionstats/src/stats.rs ]
crate::ix!();

/**
  | Per level compaction stats. stats_[level]
  | stores the stats for compactions that
  | produced data for the specified "level".
  |
  */
pub struct CompactionStats {
    micros:        i64,
    bytes_read:    i64,
    bytes_written: i64,
}

impl Default for CompactionStats {

    fn default() -> Self {
        todo!();
        /*
           : micros(0),
           : bytes_read(0),
           : bytes_written(0),
           */
    }
}

impl CompactionStats {

    pub fn add(&mut self, c: &CompactionStats)  {

        todo!();
        /*
           this->micros += c.micros;
           this->bytes_read += c.bytes_read;
           this->bytes_written += c.bytes_written;
           */
    }
}
