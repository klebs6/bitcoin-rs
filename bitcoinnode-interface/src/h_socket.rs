crate::ix!();

pub struct NodeHSocket {
    pub h_socket: CSocket,
}

impl Default for NodeHSocket {

    fn default() -> Self {
        Self {
            h_socket: INVALID_SOCKET,
        }
    }
}
