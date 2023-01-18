crate::ix!();

pub fn subroutine_thread_open_connections(
    connman: Arc<Connman>, 
    connect: Vec<String>)  {

    set_syscall_sandbox_policy(SyscallSandboxPolicy::NET_OPEN_CONNECTION);

    //  Connect to specific addresses
    if !connect.is_empty() {

        let mut n_loop: i64 = 0;

        loop {

            connman.process_addr_fetch();

            for str_addr in connect.iter() {

                let mut addr: Address = Address::new(
                    Service::default(), 
                    ServiceFlags::NODE_NONE
                );

                connman.open_network_connection(
                    &addr, 
                    false, 
                    None, 
                    str_addr.as_ptr(), 
                    ConnectionType::MANUAL
                );

                let mut i: i32 = 0;

                while i < 10 && i64::from(i) < n_loop {

                    if !connman.interrupt_net.get_mut()
                        .sleep_for(Duration::milliseconds(500)) 
                    {
                        return;
                    }

                    i += 1;
                }
            }

            if !connman.interrupt_net.get_mut().sleep_for(Duration::milliseconds(500)) {
                return;
            }

            n_loop += 1;
        }
    }

    // Initiate network connections
    let start = OffsetDateTime::now_utc();

    // Minimum time before next feeler
    // connection (in microseconds).
    let mut next_feeler = poisson_next_send(
        start,
        FEELER_INTERVAL
    );

    let mut next_extra_block_relay = poisson_next_send(
        start,
        EXTRA_BLOCK_RELAY_ONLY_PEER_INTERVAL
    );

    let dnsseed: bool = G_ARGS
        .lock()
        .get_bool_arg("-dnsseed", DEFAULT_DNSSEED);

    let mut add_fixed_seeds: bool = G_ARGS
        .lock()
        .get_bool_arg("-fixedseeds", DEFAULT_FIXEDSEEDS);

    if !add_fixed_seeds {
        log_printf!("Fixed seeds are disabled\n");
    }
    
    while !connman.interrupt_net.get().as_bool() {

        connman.process_addr_fetch();

        if !connman.interrupt_net.get_mut().sleep_for(Duration::milliseconds(500)) {
            return;
        }

        let mut grant: SemaphoreGrant 
        = SemaphoreGrant::new(
            connman.sem_outbound.clone(), 
            None
        );

        let interrupt_net: bool = connman.interrupt_net.get().as_bool();

        if interrupt_net {
            return;
        }

        if add_fixed_seeds && connman.addrman.get().len() == 0 {

            //  When the node starts with an
            //  empty peers.dat, there are
            //  a few other sources of peers
            //  before we fallback on to fixed
            //  seeds: -dnsseed, -seednode,
            //  -addnode
            //
            //  If none of those are
            //  available, we fallback on to
            //  fixed seeds immediately, else
            //  we allow
            //  60 seconds for any of those
            //  sources to populate addrman.
            let mut add_fixed_seeds_now: bool = false;;

            // It is cheapest to check if
            // enough time has passed first.
            if get_datetime() > start + Duration::minutes(1) {
                add_fixed_seeds_now = true;
                log_printf!(
                    "Adding fixed seeds as 60 seconds have passed and addrman is empty\n"
                );
            }

            // Checking !dnsseed is cheaper
            // before locking 2 mutexes.
            if !add_fixed_seeds_now && !dnsseed {

                let lock0 = connman.addr_fetches_mutex.get();
                let lock1 = connman.cs_v_added_nodes.get();

                if lock0.addr_fetches.is_empty() && lock1.added_nodes.is_empty() {

                    add_fixed_seeds_now = true;

                    log_printf!(
                        "Adding fixed seeds as -dnsseed=0, -addnode is not provided and all -seednode(s) attempted\n"
                    );
                }
            }

            if add_fixed_seeds_now {

                let mut local = NetAddr::default();

                local.set_internal("fixedseeds");

                connman.addrman
                    .get_mut()
                    .add(
                        &convert_seeds(params().fixed_seeds()), 
                        &local,
                        None
                    );

                add_fixed_seeds = false;
            }
        }

        // Choose an address to connect to
        // based on most recently seen
        let mut addr_connect = Address::default();;

        // Only connect out to one peer per
        // network group (/16 for IPv4).
        let mut n_outbound_full_relay:  i32 = 0;;
        let mut n_outbound_block_relay: i32 = 0;

        let mut set_connected = HashSet::<Vec::<u8>>::default();

        {
            let mut guard = connman.cs_v_nodes.get();

            for pnode in guard.nodes.iter() {

                let node = pnode.get();

                if node.is_full_outbound_conn() {
                    n_outbound_full_relay += 1;
                }

                if node.is_block_only_conn() {
                    n_outbound_block_relay += 1;
                }

                // Netgroups for inbound and
                // manual peers are not
                // excluded because our goal
                // here is to not use multiple
                // of our limited outbound
                // slots on a single netgroup
                // but inbound and manual
                // peers do not use our
                // outbound slots. Inbound
                // peers also have the added
                // issue that they could be
                // attacker controlled and
                // used to prevent us from
                // connecting to particular
                // hosts if we used them here.
                match node.conn_type() {

                    Some(ConnectionType::INBOUND) 
                        | Some(ConnectionType::MANUAL)  => {
                            break;
                        },

                    Some(ConnectionType::OUTBOUND_FULL_RELAY)  
                        | Some(ConnectionType::BLOCK_RELAY)  
                        | Some(ConnectionType::ADDR_FETCH)  
                        | Some(ConnectionType::FEELER)  
                        => {
                            set_connected.insert(
                                node.service().base
                                    .get_group(connman.addrman.get().get_asmap())
                            );
                        },
                    _ => {}
                }
            }
        }

        let mut conn_type: ConnectionType = ConnectionType::OUTBOUND_FULL_RELAY;

        let now = OffsetDateTime::now_utc();

        let mut anchor: bool = false;
        let mut feeler: bool = false;

        //  Determine what type of connection
        //  to open. Opening BLOCK_RELAY
        //  connections to addresses from
        //  anchors.dat gets the highest
        //  priority. Then we open
        //  OUTBOUND_FULL_RELAY priority until
        //  we meet our full-relay
        //  capacity. Then we open BLOCK_RELAY
        //  connection until we hit our
        //  block-relay-only peer limit.
        //
        //  GetTryNewOutboundPeer() gets set
        //  when a stale tip is detected, so
        //  we try opening an additional
        //  OUTBOUND_FULL_RELAY connection. If
        //  none of these conditions are met,
        //  check to see if it's time to try
        //  an extra block-relay-only peer (to
        //  confirm our tip is current, see
        //  below) or the next_feeler timer to
        //  decide if we should open a FEELER.
        if !connman.anchors.get().is_empty() 
        && (n_outbound_block_relay < connman.max_outbound_block_relay.load(atomic::Ordering::Relaxed)) 
        {
            conn_type = ConnectionType::BLOCK_RELAY;
            anchor = true;
        } else {

            if n_outbound_full_relay < connman.max_outbound_full_relay.load(atomic::Ordering::Relaxed) {
                //  OUTBOUND_FULL_RELAY
            } else {

                if n_outbound_block_relay < connman.max_outbound_block_relay.load(atomic::Ordering::Relaxed) {

                    conn_type = ConnectionType::BLOCK_RELAY;

                } else {

                    if connman.get_try_new_outbound_peer() {
                        //  OUTBOUND_FULL_RELAY
                    } else {

                        if now > next_extra_block_relay && connman.start_extra_block_relay_peers.load(atomic::Ordering::Relaxed) {

                            //  Periodically connect to a peer (using regular outbound selection
                            //  methodology from addrman) and stay connected long enough to sync
                            //  headers, but not much else.
                            //
                            //  Then disconnect the peer, if we haven't learned anything new.
                            //
                            //  The idea is to make eclipse attacks very difficult to pull off,
                            //  because every few minutes we're finding a new peer to learn headers
                            //  from.
                            //
                            //  This is similar to the logic for trying extra outbound (full-relay)
                            //  peers, except:
                            //
                            //  - we do this all the time on a poisson timer, rather than just when
                            //    our tip is stale
                            //
                            //  - we potentially disconnect our next-youngest block-relay-only peer, if our
                            //    newest block-relay-only peer delivers a block more recently.
                            //    See the eviction logic in net_processing.cpp.
                            //
                            //  Because we can promote these connections to block-relay-only
                            //  connections, they do not get their own ConnectionType enum
                            //
                            //  (similar to how we deal with extra outbound peers).
                            next_extra_block_relay = poisson_next_send(
                                now,
                                EXTRA_BLOCK_RELAY_ONLY_PEER_INTERVAL
                            );

                            conn_type = ConnectionType::BLOCK_RELAY;

                        } else {

                            if now > next_feeler {

                                next_feeler = poisson_next_send(
                                    now,
                                    FEELER_INTERVAL
                                );

                                conn_type = ConnectionType::FEELER;

                                feeler = true;

                            } else {
                                //  skip to next iteration of while loop
                                continue;
                            }
                        }
                    }
                }
            }
        }

        connman.addrman.get_mut().resolve_collisions();

        let n_anow:  i64 = get_adjusted_time();
        let mut n_tries: i32 = 0;

        while !connman.interrupt_net.get().as_bool() {

            {
                let mut anchors = connman.anchors.get_mut();

                if anchor && !anchors.is_empty() {

                    let mut addr: Address = anchors.last().unwrap().clone();

                    anchors.pop();

                    if !addr.service.base.is_valid() 
                    || is_local(&addr.service) 
                    || !addr.service.base.is_reachable() 
                    || !has_all_desirable_service_flags(addr.n_services) 
                    || set_connected.contains(&addr.service.base.get_group(connman.addrman.get().get_asmap())) {
                        continue;
                    }

                    addr_connect = addr;

                    log_print!(
                        bc_log::NET, 
                        "Trying to make an anchor connection to {}\n", 
                        addr_connect.to_string()
                    );

                    break;
                }
            }

            // If we didn't find an
            // appropriate destination after
            // trying 100 addresses fetched
            // from addrman, stop this loop,
            // and let the outer loop run
            // again (which sleeps, adds seed
            // nodes, recalculates
            // already-connected network
            // ranges, ...) before trying new
            // addrman addresses.
            {
                let old = n_tries;
                n_tries += 1;
                old
            };

            if n_tries > 100 {
                break;
            }

            let mut addr = Address::default();

            let mut addr_last_try: i64 = 0;

            if feeler {

                // First, try to get a tried
                // table collision
                // address. This returns an
                // empty (invalid) address if
                // there are no collisions to
                // try.
                (addr,addr_last_try) = connman.addrman.get_mut().select_tried_collision();

                if !addr.service.base.is_valid() {

                    // No tried table
                    // collisions. Select
                    // a new table address for
                    // our feeler.
                    (addr,addr_last_try) = connman.addrman.get().select(Some(true));

                } else {

                    if connman.already_connected_to_address(&addr) {

                        // If
                        // test-before-evict
                        // logic would have us
                        // connect to a peer
                        // that we're already
                        // connected to, just
                        // mark that address
                        // as Good(). We won't
                        // be able to initiate
                        // the connection
                        // anyway, so this
                        // avoids
                        // inadvertently
                        // evicting
                        // a currently-connected
                        // peer.
                        connman.addrman.get_mut().good(&addr.service, None);

                        // Select a new table
                        // address for our
                        // feeler instead.
                        (addr,addr_last_try) = connman.addrman.get().select(Some(true));
                    }
                }

            } else {

                // Not a feeler
                (addr,addr_last_try) = connman.addrman.get().select(None);
            }

            // Require outbound connections,
            // other than feelers, to be to
            // distinct network groups
            if !feeler && set_connected.contains(&addr.service.base.get_group(connman.addrman.get().get_asmap())) {
                break;
            }

            // if we selected an invalid or
            // local address, restart
            if !addr.service.base.is_valid() 
            || is_local(&addr.service) {
                break;
            }

            if !addr.service.base.is_reachable() {
                continue;
            }

            // only consider very recently
            // tried nodes after 30 failed
            // attempts
            if n_anow - addr_last_try < 600 && n_tries < 30 {
                continue;
            }

            // for non-feelers, require all
            // the services we'll want, for
            // feelers, only require they be
            // a full node (only because most
            // SPV clients don't have a good
            // address DB available)
            if !feeler 
            && !has_all_desirable_service_flags(addr.n_services) {

                continue;

            } else {

                if feeler && !may_have_useful_addressdb(addr.n_services) {
                    continue;
                }
            }

            // Do not allow non-default ports,
            // unless after 50 invalid
            // addresses selected
            // already. This is to prevent
            // malicious peers from
            // advertising themselves as
            // a service on another host and
            // port, causing a DoS attack as
            // nodes around the network
            // attempt to connect to it
            // fruitlessly.
            if addr.service.get_port() != params().get_default_port_with_network(addr.service.base.get_network()) 
            && n_tries < 50 {
                continue;
            }

            addr_connect = addr;
            break;
        }

        if addr_connect.service.base.is_valid() {

            if feeler {

                // Add small amount of random
                // noise before connection to
                // avoid synchronization.
                let randsleep: i32 = get_rand_int((FEELER_SLEEP_WINDOW * 1000).try_into().unwrap());

                if !connman.interrupt_net.get_mut().sleep_for(
                    Duration::milliseconds(randsleep.try_into().unwrap())
                ) {
                    return;
                }

                log_print!(
                    bc_log::NET, 
                    "Making feeler connection to {}\n", 
                    addr_connect.to_string()
                );
            }

            connman.open_network_connection(
                &addr_connect, 
                set_connected.len() as i32 >= min(connman.n_max_connections.load(atomic::Ordering::Relaxed) - 1,2), 
                Some(&mut grant), 
                null_mut(), 
                conn_type
            );
        }
    }
}

