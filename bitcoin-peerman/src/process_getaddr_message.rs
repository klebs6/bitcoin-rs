crate::ix!();

impl PeerManager {

    pub fn process_getaddr_message(self: Arc<Self>, 
        peer:               &mut Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let mut peer = peer.as_mut().unwrap();

        // This asymmetric behavior for inbound
        // and outbound connections was introduced
        // to prevent a fingerprinting attack: an
        // attacker can send specific fake
        // addresses to users' AddrMan and later
        // request them by sending getaddr
        // messages.
        //
        // Making nodes which are behind NAT and
        // can only make outgoing connections
        // ignore the getaddr message mitigates
        // the attack.
        if !pfrom.is_inbound_conn() {

            log_print!(
                LogFlags::NET, 
                "Ignoring \"getaddr\" from %s connection. peer=%d\n", 
                pfrom.connection_type_as_string(), 
                pfrom.get_id()
            );

            return;
        }

        // Since this must be an inbound
        // connection, SetupAddressRelay will
        // never fail.
        assume!(
            self.clone()
            .setup_address_relay(&***pfrom, &mut peer)
        );

        // Only send one GetAddr response per
        // connection to reduce resource waste and
        // discourage addr stamping of INV
        // announcements.
        if peer.getaddr_recvd {

            log_print!(
                LogFlags::NET, 
                "Ignoring repeated \"getaddr\". peer=%d\n", 
                pfrom.get_id()
            );

            return;
        }

        peer.getaddr_recvd = true;
        peer.addrs_to_send.lock().clear();

        let mut addr: Vec<Address> = vec![];

        if pfrom.has_permission(NetPermissionFlags::Addr) {

            addr = self.connman.get().get_addresses(
                MAX_ADDR_TO_SEND, 
                MAX_PCT_ADDR_TO_SEND, 
                /* network */ None
            );

        } else {

            addr = self.connman.get_mut().get_addresses_with_requestor(
                pfrom, 
                MAX_ADDR_TO_SEND, 
                MAX_PCT_ADDR_TO_SEND
            );
        }

        let mut insecure_rand = FastRandomContext::default();

        for addr in addr.iter() {

            peer.push_address(
                addr, 
                &mut insecure_rand
            );
        }
    }
}
