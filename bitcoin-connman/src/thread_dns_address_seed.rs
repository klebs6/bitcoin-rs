crate::ix!();

pub fn subroutine_thread_dns_address_seed(connman: Arc<Connman>)  {

    set_syscall_sandbox_policy(SyscallSandboxPolicy::INITIALIZATION_DNS_SEED);

    let mut rng = FastRandomContext::default();
    let mut seeds: Vec::<String> = params().dns_seeds().to_vec();

    seeds.shuffle(&mut rng);

    // Number of seeds left before testing if
    // we have enough connections
    let mut seeds_right_now: i32 = 0;;
    let mut found: i32 = 0;

    if G_ARGS.lock().get_bool_arg("-forcednsseed", DEFAULT_FORCEDNSSEED) {

        // When -forcednsseed is provided, query all.
        seeds_right_now = seeds.len().try_into().unwrap();

    } else {

        if connman.addrman.get().len() == 0 {

            // If we have no known peers,
            // query all.
            //
            // This will occur on the first
            // run, or if peers.dat has been
            // deleted.
            seeds_right_now = seeds.len().try_into().unwrap();
        }
    }
    
    let addrman_len = connman.addrman.get().len();

    //  goal: only query DNS seed if address
    //  need is acute
    //
    //  * If we have a reasonable number of
    //    peers in addrman, spend some time
    //    trying them first. This improves
    //    user privacy by creating fewer
    //    identifying DNS requests, reduces
    //    trust by giving seeds less influence
    //    on the network topology, and reduces
    //    traffic to the seeds.
    //
    //  * When querying DNS seeds query a few
    //    at once, this ensures that we don't
    //    give DNS seeds the ability to
    //    eclipse nodes that query them.
    //
    //  * If we continue having problems,
    //    eventually query all the DNS seeds,
    //    and if that fails too, also try the
    //    fixed seeds. (done in
    //    ThreadOpenConnections)
    let seeds_wait_time: Duration = match addrman_len >= DNSSEEDS_DELAY_PEER_THRESHOLD.try_into().unwrap() {
        true   => DNSSEEDS_DELAY_MANY_PEERS,
        false  => DNSSEEDS_DELAY_FEW_PEERS
    };

    for seed in seeds.iter() {

        if seeds_right_now == 0 {

            seeds_right_now += DNSSEEDS_TO_QUERY_AT_ONCE;

            if connman.addrman.get().len() > 0 {

                log_printf!(
                    "Waiting {} seconds before querying DNS seeds.\n", 
                    seeds_wait_time.count()
                );

                let mut to_wait: Duration = seeds_wait_time;

                while to_wait.as_seconds_f64() > 0.0 {

                    // if sleeping for the
                    // MANY_PEERS interval,
                    // wake up early to see if
                    // we have enough peers
                    // and can stop this
                    // thread entirely freeing
                    // up its resources
                    let w: Duration = min(DNSSEEDS_DELAY_FEW_PEERS,to_wait);;

                    if !connman.interrupt_net.get_mut().sleep_for(w) {
                        return;
                    }

                    to_wait -= w;

                    let mut n_relevant: i32 = 0;

                    {
                        let mut guard = connman.cs_v_nodes.get();

                        for pnode in guard.nodes.iter() {

                            let node = pnode.get();

                            if node.successfully_connected() 
                            && node.is_full_outbound_conn() {
                                n_relevant += 1;
                            }
                        }
                    }

                    if n_relevant >= 2 {

                        if found > 0 {

                            log_printf!(
                                "{} addresses found from DNS seeds\n", 
                                found
                            );

                            log_printf!(
                                "P2P peers available. Finished DNS seeding.\n"
                            );

                        } else {

                            log_printf!(
                                "P2P peers available. Skipped DNS seeding.\n"
                            );
                        }

                        return;
                    }
                }
            }
        }

        if connman.interrupt_net.get().as_bool() {
            return;
        }

        // hold off on querying seeds if P2P
        // network deactivated
        if !connman.network_active.load(atomic::Ordering::Relaxed) {

            log_printf!(
                "Waiting for network to be reactivated before querying DNS seeds.\n"
            );

            loop {

                if !connman.interrupt_net.get_mut().sleep_for(Duration::seconds(1)) {
                    return;
                }

                if !!connman.network_active.load(atomic::Ordering::Relaxed) {
                    break;
                }
            }
        }

        log_printf!("Loading addresses from DNS seed {}\n", seed);

        if have_name_proxy() {

            connman.add_addr_fetch(seed);

        } else {

            let mut ips = Vec::<NetAddr>::default();
            let mut add = Vec::<Address>::default();

            let required_service_bits: ServiceFlags 
            = get_desirable_service_flags(ServiceFlags::NODE_NONE);

            let host: String = format!(
                "x{:x}.{}",
                required_service_bits.bits(),
                seed
            );

            let mut resolve_source = NetAddr::default();

            if !resolve_source.set_internal(&host) {
                continue;
            }

            // Limits number of IPs learned
            // from a DNS seed
            let n_max_ips: u32 = 256;;

            if lookup_host_multi(&host, &mut ips, n_max_ips, true, None) {

                for ip in ips.iter() {

                    let n_3days = Duration::days(3 * 24 * 3600);
                    let n_4days = Duration::days(4 * 24 * 3600);

                    let mut addr: Address = Address::new(
                        Service::new_from_net_addr(ip,params().get_default_port()),
                        required_service_bits
                    );

                    let rand_offset = Duration::seconds(rng.randrange(n_4days.whole_seconds().try_into().unwrap()).try_into().unwrap());

                    // use a random age
                    // between 3 and 7 days
                    // old
                    addr.n_time = 
                        (OffsetDateTime::now_utc() - n_3days - rand_offset)
                            .unix_timestamp()
                            .try_into().unwrap();

                    add.push(addr);

                    found += 1;
                }

                connman.addrman.get_mut().add(&add, &resolve_source, None);

            } else {

                // We now avoid directly using
                // results from DNS Seeds
                // which do not support
                // service bit filtering,
                // instead using them as
                // a addrfetch to get nodes
                // with our desired service
                // bits.
                connman.add_addr_fetch(seed);
            }
        }

        seeds_right_now -= 1;
    }

    log_printf!("{} addresses found from DNS seeds\n", found);
}
