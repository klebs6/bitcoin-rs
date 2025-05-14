// ---------------- [ File: bitcoin-connman/src/cached.rs ]
crate::ix!();

/**
  | Cache responses to addr requests to
  | minimize privacy leak.
  | 
  | Attack example: scraping addrs in real-time
  | may allow an attacker to infer new connections
  | of the victim by detecting new records
  | with fresh timestamps (per self-announcement).
  |
  */
#[derive(Default)]
pub struct ConnmanCachedAddrResponse {
    pub addrs_response_cache:   Vec<Address>,
    pub cache_entry_expiration: Option<Instant>, /* microseconds */
}
