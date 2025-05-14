// ---------------- [ File: bitcoin-time/src/mock_time.rs ]
crate::ix!();

lazy_static!{
    /*
    static std::atomic<int64_t> nMockTime(0); /// For testing
    */
}

/**
  | For testing. Set e.g. with the setmocktime
  | rpc, or -mocktime argument
  |
  */
pub fn set_mock_time(mock_time_in: Instant)  {
    
    todo!();
        /*
            nMockTime.store(mock_time_in.count(), std::memory_order_relaxed);
        */
}
