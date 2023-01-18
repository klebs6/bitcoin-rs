crate::ix!();

impl InitializeNode for PeerManager {

    fn initialize_node(&mut self, mut pnode: &mut AmoWriteGuard<Box<dyn NodeInterface>>)  {
        
        let nodeid: NodeId = (*pnode).get_id();

        {
            let mut guard = CS_MAIN.lock();

            MAP_NODE_STATE.lock().insert(
                nodeid, 
                Amo::<NodeState>::from(NodeState::new((*pnode).is_inbound_conn()))
            );

            assert!(self.inner.lock().txrequest.lock().count(nodeid) == 0);
        }

        {
            let peer: Amo<Peer> = Amo::<Peer>::from(Peer::new(nodeid));

            self.peer_map.get_mut().insert(nodeid, peer);
        }

        if !(*pnode).is_inbound_conn() {

            self.push_node_version(
                pnode, 
                &get_datetime()
            );
        }
    }
}
