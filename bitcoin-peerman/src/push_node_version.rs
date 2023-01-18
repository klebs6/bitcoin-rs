crate::ix!();

pub trait PushNodeVersion {

    fn push_node_version(&self, 
        pnode:  &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        n_time: &OffsetDateTime);
}

impl PushNodeVersion for PeerManager {

    /**
      | Send a version message to a peer
      |
      */
    fn push_node_version(&self, 
        mut pnode: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        n_time:    &OffsetDateTime)  {
        
        // Note that pnode->GetLocalServices() is
        // a reflection of the local services we
        // were offering when the Node object was
        // created for this peer.
        let my_services: u64 = pnode.get_local_services().bits() as u64;
        let nonce:       u64 = pnode.get_local_nonce();

        let n_node_starting_height: i32 = self.best_height.load(atomic::Ordering::Relaxed);

        let nodeid: NodeId = pnode.get_id();
        let addr:  Address = pnode.addr().clone();

        let addr_you: Service 
        = match 
            addr.is_routable() 
            && !is_proxy(&addr.service.base) 
            && addr.is_addr_v1compatible() 
        {
            true   => addr.service,
            false  => Service::default()
        };

        let your_services: u64 = addr.n_services.bits() as u64;

        let tx_relay: bool = 
        !self.ignore_incoming_txs 
        && pnode.has_tx_relay();

        self.connman.get_mut().push_message(
            &mut *pnode, 
            NetMsgMaker::new(INIT_PROTO_VERSION)
                .make(
                    NetMsgType::VERSION, 
                    &[
                        &PROTOCOL_VERSION, 
                        &my_services, 
                        n_time, 
                        &your_services, 
                        &addr_you, /* Together the pre-version-31402 serialization of CAddress "addrYou" (without nTime) */ 
                        &my_services,  
                        &Service::default(), /* Together the pre-version-31402 serialization of CAddress "addrMe" (without nTime) */ 
                        &nonce, 
                        &STR_SUBVERSION, 
                        &n_node_starting_height, 
                        &tx_relay
                    ]
                )
        );

        if *LOG_IPS {

            log_print!(
                LogFlags::NET, 
                "send version message: version %d, blocks=%d, them=%s, txrelay=%d, peer=%d\n", 
                PROTOCOL_VERSION, 
                n_node_starting_height, 
                addr_you.to_string(), 
                tx_relay, 
                nodeid
            );

        } else {

            log_print!(
                LogFlags::NET, 
                "send version message: version %d, blocks=%d, txrelay=%d, peer=%d\n", 
                PROTOCOL_VERSION, 
                n_node_starting_height, 
                tx_relay, 
                nodeid
            );
        }
    }
}
