// ---------------- [ File: bitcoin-peerman/src/process_addr_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_addr_message(self: Arc<Self>, 
        peer:               &mut Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let mut stream_version: i32 = recv.get_version();

        if msg_type == NetMsgType::ADDRV2 {

            // Add ADDRV2_FORMAT to the
            // version so that the CNetAddr
            // and CAddress unserialize
            // methods know that an address in
            // v2 format is coming.
            stream_version |= ADDRV2_FORMAT;
        }

        let s: OverrideStream::<DataStream> 
        = OverrideStream::<DataStream>::new(
            recv, 
            recv.get_type(), 
            stream_version
        );

        let mut addrs: Vec<Address> = vec![];

        s.stream_into(&mut addrs);

        if !self.clone().setup_address_relay(
            &***pfrom,
            peer.as_mut().unwrap()) 
        {
            log_print!(
                LogFlags::NET, 
                "ignoring %s message from %s peer=%d\n", 
                msg_type, 
                pfrom.connection_type_as_string(), 
                pfrom.get_id()
            );

            return;
        }

        if addrs.len() > MAX_ADDR_TO_SEND {

            self.misbehaving(
                pfrom.get_id(), 
                20, 
                format!("{} message size = {}",msg_type,addrs.len()).as_str()
            );

            return;
        }

        // Store the new addresses
        let mut addr_ok: Vec<Address> = vec![];

        let n_now:   i64 = get_adjusted_time();
        let n_since: i64 = n_now - 10 * 60;

        // Update/increment addr rate limiting
        // bucket.
        let current_time = get_datetime();

        if peer.as_ref().unwrap().addr_token_bucket < (MAX_ADDR_PROCESSING_TOKEN_BUCKET as f64) {

            // Don't increment bucket if it's already full
            let time_diff = max(
                current_time - peer.as_ref().unwrap().addr_token_timestamp,
                Duration::microseconds(0)
            );

            let increment: f64 = time_diff.as_seconds_f64() * MAX_ADDR_RATE_PER_SECOND;

            peer.as_mut().unwrap().addr_token_bucket 
                = min(
                    FloatOrd(peer.as_ref().unwrap().addr_token_bucket + increment),
                    FloatOrd(MAX_ADDR_PROCESSING_TOKEN_BUCKET as f64)
                ).0;
        }

        peer.as_mut().unwrap().addr_token_timestamp = current_time;

        let rate_limited: bool = !pfrom.has_permission(NetPermissionFlags::Addr);

        let mut num_proc:       u64 = 0;
        let mut num_rate_limit: u64 = 0;

        let mut gen = FastRandomContext::default();

        addrs.shuffle(&mut gen);

        let addrs_len = addrs.len();

        for addr in addrs.iter_mut() {

            if interrupt_msg_proc.load(atomic::Ordering::Relaxed) {
                return;
            }

            // Apply rate limiting.
            if peer.as_ref().unwrap().addr_token_bucket < 1.0 {

                if rate_limited {
                    num_rate_limit += 1;
                    continue;
                }

            } else {

                peer.as_mut().unwrap().addr_token_bucket -= 1.0;
            }

            // We only bother storing full
            // nodes, though this may include
            // things which we would not make
            // an outbound connection to, in
            // part because we may make feeler
            // connections to them.
            if !may_have_useful_addressdb(addr.n_services) 
            && !has_all_desirable_service_flags(addr.n_services) {
                continue;
            }

            if addr.n_time <= 100000000 
            || addr.n_time > (n_now + 10 * 60).try_into().unwrap() {

                addr.n_time = (n_now - 5 * 24 * 60 * 60).try_into().unwrap();
            }

            peer.as_mut().unwrap().add_address_known(addr);

            if self.banman.is_some() 
            && (
            self.banman.get().is_discouraged(&addr.service.base) 
            || self.banman.get().is_netaddr_banned(&addr.service.base)
        ) {
                // Do not process
                // banned/discouraged
                // addresses beyond
                // remembering we received
                // them
                continue;
            }
            {
                num_proc += 1;
                num_proc
            };

            let reachable: bool = addr.is_reachable();

            if i64::from(addr.n_time) > n_since 
            && !peer.as_ref().unwrap().getaddr_sent 
            && addrs_len <= 10 
            && addr.is_routable() {

                // Relay to a limited number
                // of other nodes
                self.clone().relay_address(pfrom.get_id(),addr,reachable);
            }

            // Do not store addresses outside
            // our network
            if reachable {
                addr_ok.push(addr.clone());
            }
        }

        peer.as_ref().unwrap().addr_processed.fetch_add(num_proc,          atomic::Ordering::Relaxed);
        peer.as_ref().unwrap().addr_rate_limited.fetch_add(num_rate_limit, atomic::Ordering::Relaxed);

        log_print!(
            LogFlags::NET, 
            "Received addr: %u addresses (%u processed, %u rate-limited) from peer=%d\n", 
            addr.len(), 
            num_proc, 
            num_rate_limit, 
            pfrom.get_id()
        );

        self.addrman.get_mut().add(&addr_ok, &pfrom.service().base, Some(2 * 60 * 60));

        if addrs.len() < 1000 {
            peer.as_mut().unwrap().getaddr_sent = false;
        }

        // AddrFetch: Require multiple
        // addresses to avoid disconnecting on
        // self-announcements
        if pfrom.is_addr_fetch_conn() && addrs.len() > 1 {

            log_print!(
                LogFlags::NET, 
                "addrfetch connection completed peer=%d; disconnecting\n", 
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();
        }
    }
}
