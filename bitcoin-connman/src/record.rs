// ---------------- [ File: bitcoin-connman/src/record.rs ]
crate::ix!();

impl Connman {

    pub fn record_bytes_recv(&self, bytes: u64)  {
        
        let mut guard = self.cs_total_bytes_recv.get_mut();

        guard.n_total_bytes_recv += bytes;
    }
    
    pub fn record_bytes_sent(&self, bytes: u64)  {
        
        let mut guard = self.cs_total_bytes_sent.get_mut();

        guard.n_total_bytes_sent += bytes;

        let now = Instant::now();

        if (guard.n_max_outbound_cycle_start_time.unwrap() + MAX_UPLOAD_TIMEFRAME) < now {

            // timeframe expired, reset cycle
            guard.n_max_outbound_cycle_start_time = Some(now);
            guard.n_max_outbound_total_bytes_sent_in_cycle = 0;
        }

        // TODO, exclude peers with download permission
        guard.n_max_outbound_total_bytes_sent_in_cycle += bytes;
    }
}
