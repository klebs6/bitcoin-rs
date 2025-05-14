// ---------------- [ File: bitcoin-cli/src/netinfo_peer.rs ]
crate::ix!();

//------------------------------
#[derive(PartialEq,Eq,Ord)]
pub struct NetInfoRequestHandlerPeer {
    pub addr:                  String,
    pub sub_version:           String,
    pub conn_type:             String,
    pub network:               String,
    pub age:                   String,
    pub min_ping:              FloatOrd<f64>,
    pub ping:                  FloatOrd<f64>,
    pub addr_processed:        i64,
    pub addr_rate_limited:     i64,
    pub last_blck:             i64,
    pub last_recv:             i64,
    pub last_send:             i64,
    pub last_trxn:             i64,
    pub id:                    i32,
    pub mapped_as:             i32,
    pub version:               i32,
    pub is_addr_relay_enabled: bool,
    pub is_bip152_hb_from:     bool,
    pub is_bip152_hb_to:       bool,
    pub is_block_relay:        bool,
    pub is_outbound:           bool,
}

impl NetInfoRequestHandlerPeer {

    #[inline] fn cmp_peer(&self, other: &NetInfoRequestHandlerPeer) -> Ordering {

        let tup = (self.is_outbound,self.min_ping);

        tup.cmp(&(other.is_outbound,other.min_ping))
    }
}

impl PartialOrd<NetInfoRequestHandlerPeer> for NetInfoRequestHandlerPeer {

    #[inline] fn partial_cmp(&self, other: &NetInfoRequestHandlerPeer) -> Option<Ordering> {

        Some(self.cmp_peer(other))
    }
}
