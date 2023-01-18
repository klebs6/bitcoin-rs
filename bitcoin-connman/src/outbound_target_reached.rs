crate::ix!();

impl Connman {

    pub fn n_receive_flood_size(&self) -> u32 {
        self.n_receive_flood_size.load(atomic::Ordering::Relaxed)
    }

    /**
      | check if the outbound target is reached
      |
      | if param historicalBlockServingLimit is
      | set true, the function will response true
      | if the limit for serving historical blocks
      | has been reached
      */
    pub fn outbound_target_reached(&self, historical_block_serving_limit: bool) -> bool {
        
        let guard = self.cs_total_bytes_sent.get();

        if guard.n_max_outbound_limit == 0 {
            return false;
        }

        if historical_block_serving_limit {

            // keep a large enough buffer to at least relay each block once
            let time_left_in_cycle: Duration = self.get_max_outbound_time_left_in_cycle();

            let buffer: u64 = {

                let max:     u64 = MAX_BLOCK_SERIALIZED_SIZE.try_into().unwrap();

                let ten_minutes = Duration::minutes(10);

                let n: u64 = time_left_in_cycle.whole_seconds().try_into().unwrap();
                let d: u64 = ten_minutes.whole_seconds().try_into().unwrap();

                n / d * max
            };

            if buffer >= guard.n_max_outbound_limit 
            || guard.n_max_outbound_total_bytes_sent_in_cycle >= guard.n_max_outbound_limit - buffer {
                return true;
            }

        } else {

            if guard.n_max_outbound_total_bytes_sent_in_cycle >= guard.n_max_outbound_limit {
                return true;
            }
        }

        false
    }
}
