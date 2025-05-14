// ---------------- [ File: bitcoin-net/src/poisson_next_send.rs ]
crate::ix!();

/**
  | Return a timestamp in the future (in
  | microseconds) for exponentially distributed
  | events.
  |
  */
pub fn poisson_next_send(
        now:              OffsetDateTime, 
        average_interval: Duration) -> OffsetDateTime {
    
    todo!();
        /*
            double unscaled = -log1p(GetRand(1ULL << 48) * -0.0000000000000035527136788 /* -1/2^48 */);
        return now + duration_cast<microseconds>(unscaled * average_interval + 0.5us);
        */
}
