// ---------------- [ File: bitcoinnode-interface/src/v_send.rs ]
crate::ix!();

pub struct NodeVSend {
    pub map_send_bytes_per_msg_cmd: MapMsgCmdSize,

    /**
      | Total size of all vSendMsg entries
      | 
      |
      */
    pub n_send_size:          usize, // default = { 0 }

    /**
      | Offset inside the first vSendMsg already
      | sent 
      |
      */
    pub n_send_offset:        usize, // default = { 0 }
    pub n_send_bytes:         u64, // default = { 0 }
    pub send_msg:             VecDeque<Vec<u8>>,
}

impl Default for NodeVSend {
    fn default() -> Self {
        Self {
            map_send_bytes_per_msg_cmd: MapMsgCmdSize::default(),
            n_send_size:          0,
            n_send_offset:        0,
            n_send_bytes:         0,
            send_msg:             VecDeque::<Vec::<u8>>::default(),
        }
    }
}
