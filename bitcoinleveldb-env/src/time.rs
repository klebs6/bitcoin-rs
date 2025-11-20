// ---------------- [ File: bitcoinleveldb-env/src/time.rs ]
crate::ix!();

pub trait NowMicros {

    /**
      | Returns the number of micro-seconds
      | since some fixed point in time. Only
      | useful for computing deltas of time.
      |
      */
    fn now_micros(&mut self) -> u64;
}

pub trait SleepForMicroseconds {

    /**
      | Sleep/delay the thread for the prescribed
      | number of micro-seconds.
      |
      */
    fn sleep_for_microseconds(&mut self, micros: i32);
}
