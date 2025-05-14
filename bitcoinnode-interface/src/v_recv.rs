// ---------------- [ File: bitcoinnode-interface/src/v_recv.rs ]
crate::ix!();

pub struct NodeVRecv {
    pub map_recv_bytes_per_msg_cmd: MapMsgCmdSize,
    pub n_recv_bytes:               u64, // default = { 0 }
}

impl Default for NodeVRecv {

    fn default() -> Self {
        Self {
            map_recv_bytes_per_msg_cmd: MapMsgCmdSize::default(),
            n_recv_bytes:               0,
        }
    }
}
