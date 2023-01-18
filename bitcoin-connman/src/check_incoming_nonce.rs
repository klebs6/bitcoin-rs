crate::ix!();

impl Connman {

    pub fn check_incoming_nonce(&mut self, nonce: u64) -> bool {

        let guard = self.cs_v_nodes.get();

        for pnode in guard.nodes.iter() {

            let node = pnode.get();

            unsafe {
                if !node.successfully_connected() 
                && !node.is_inbound_conn() 
                && node.get_local_nonce() == nonce 
                {
                    return false;
                }
            }
        }

        true
    }
}
