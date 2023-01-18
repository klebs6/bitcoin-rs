crate::ix!();

pub struct ConnmanTotalBytesSent {

    pub n_total_bytes_sent:                       u64,

    /**
      | outbound limit & stats
      |
      */
    pub n_max_outbound_total_bytes_sent_in_cycle: u64,
    pub n_max_outbound_cycle_start_time:          Option<Instant>, 
    pub n_max_outbound_limit:                     u64,
}

impl Default for ConnmanTotalBytesSent {

    fn default() -> Self {
        Self {
            n_total_bytes_sent:                       0, 
            n_max_outbound_total_bytes_sent_in_cycle: 0, 
            n_max_outbound_cycle_start_time:          None, 
            n_max_outbound_limit:                     0,
        }
    }
}

//------------------------------------
pub struct ConnmanTotalBytesRecv {
    pub n_total_bytes_recv: u64,
}

impl Default for ConnmanTotalBytesRecv {

    fn default() -> Self {
        Self {
            n_total_bytes_recv: 0,
        }
    }
}
