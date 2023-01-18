crate::ix!();

pub const NETINFO_REQUEST_HANDLER_NETWORKS: &[&'static str] = &["ipv4", "ipv6", "onion", "i2p"];
pub const NETINFO_REQUEST_HANDLER_MAX_DETAIL_LEVEL:  u8 = 4;
pub const NETINFO_REQUEST_HANDLER_ID_PEERINFO:      usize = 0;
pub const NETINFO_REQUEST_HANDLER_ID_NETWORKINFO:   usize = 1;

/**
  | Process netinfo requests
  |
  */
pub struct NetinfoRequestHandler {

    /**
      | Peer counts by (in/out/total, networks/total)
      |
      */
    pub counts:                       [[u16; NETINFO_REQUEST_HANDLER_NETWORKS.len() + 1]; 3],

    pub block_relay_peers_count:      u8,
    pub manual_peers_count:           u8,

    /**
      | Optional user-supplied arg to set dashboard
      | details level
      |
      */
    pub details_level:                u8,

    pub is_asmap_on:                  bool,
    pub max_addr_length:              usize,
    pub max_addr_processed_length:    usize,
    pub max_addr_rate_limited_length: usize,
    pub max_age_length:               usize,
    pub max_id_length:                usize,
    pub peers:                        Vec<NetInfoRequestHandlerPeer>,

    pub time_now:                     i64,

    pub help_doc:                     String,
}

impl Default for NetinfoRequestHandler {
    fn default() -> Self {
        Self {
            counts:                       [[0; NETINFO_REQUEST_HANDLER_NETWORKS.len() + 1]; 3],
            block_relay_peers_count:      0,
            manual_peers_count:           0,
            details_level:                0,
            is_asmap_on:                  false,
            max_addr_length:              0,
            max_addr_processed_length:    5,
            max_addr_rate_limited_length: 6,
            max_age_length:               5,
            max_id_length:                2,
            peers:                        vec![],
            time_now:                     get_time_seconds_since_epoch(),
            help_doc:                     Self::default_help_doc(),
        }
    }
}

impl BaseRequestHandler for NetinfoRequestHandler {

    fn prepare_request(&mut self, 
        method: &str,
        args:   &Vec<String>) -> Result<UniValue,StdException> {

        if !args.is_empty() {

            let mut n: u8 = 0;

            if parse_uint8(&args[0],&mut n) {

                self.details_level = min(n,NETINFO_REQUEST_HANDLER_MAX_DETAIL_LEVEL);

            } else {

                let msg = format!(
                    "invalid -netinfo argument: {}\nFor more information, run: bitcoin-cli -netinfo help",
                    args[0]
                );

                return Err(runtime_error(&msg));
            }
        }

        let mut result: UniValue = UniValue::from(uni_value::VType::VARR);

        result.push_back(
            &jsonrpc_request_obj(
                "getpeerinfo",   
                &NULL_UNI_VALUE,
                &UniValue::from(NETINFO_REQUEST_HANDLER_ID_PEERINFO)
            )
        );

        result.push_back(
            &jsonrpc_request_obj(
                "getnetworkinfo",
                &NULL_UNI_VALUE,
                &UniValue::from(NETINFO_REQUEST_HANDLER_ID_NETWORKINFO)
            )
        );

        Ok(result)
    }
    
    fn process_reply(&mut self, batch_in: &UniValue) -> Result<UniValue,StdException> {

        let batch: Vec::<UniValue> = jsonrpc_process_batch_reply(batch_in);

        if !batch[NETINFO_REQUEST_HANDLER_ID_PEERINFO]["error"].is_null() {
            return Ok(batch[NETINFO_REQUEST_HANDLER_ID_PEERINFO].clone());
        }

        if !batch[NETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["error"].is_null() {
            return Ok(batch[NETINFO_REQUEST_HANDLER_ID_NETWORKINFO].clone());
        }

        let networkinfo: &UniValue = &batch[NETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["result"];

        if networkinfo["version"].get_int() < 209900 {
            return Err(runtime_error("-netinfo requires bitcoind server to be running v0.21.0 and up"));
        }

        // Count peer connection totals, and if
        // DetailsRequested(), store peer data in
        // a vector of structs.
        for peer in batch[NETINFO_REQUEST_HANDLER_ID_PEERINFO]["result"].get_values().unwrap().iter() {

            let network: String = peer["network"].get_str().to_string();
            let network_id: usize = self.network_string_to_id(&network).try_into().unwrap();

            if network_id == UNKNOWN_NETWORK as usize {
                continue;
            }

            let is_outbound:    bool = !peer["inbound"].get_bool();
            let is_block_relay: bool = !peer["relaytxes"].get_bool();

            let conn_type: String = peer["connection_type"].get_str().to_string();

            let is_outbound_idx: usize = is_outbound.into();

            // in/out by network
            self.counts[is_outbound_idx][network_id] += 1;

            // in/out overall
            self.counts[is_outbound_idx][NETINFO_REQUEST_HANDLER_NETWORKS.len()] += 1;

            // total by network
            self.counts[2][network_id] += 1;

            // total overall
            self.counts[2][NETINFO_REQUEST_HANDLER_NETWORKS.len()] += 1;

            if conn_type == "block-relay-only" {
                self.block_relay_peers_count += 1;
            }

            if conn_type == "manual" {
                self.manual_peers_count += 1;
            }

            if self.details_requested() {

                //  Push data for this peer to the peers vector.
                let peer_id:   i32 = peer["id"].get_int();

                let mapped_as: i32 = match peer["mapped_as"].is_null() {
                    true   => 0,
                    false  => peer["mapped_as"].get_int()
                };

                let version:        i32 = peer["version"].get_int();

                let addr_processed: i64 = match peer["addr_processed"].is_null() {
                    true   => 0,
                    false  => peer["addr_processed"].get_int64()
                };

                let addr_rate_limited: i64 = match peer["addr_rate_limited"].is_null() {
                    true   => 0,
                    false  => peer["addr_rate_limited"].get_int64()
                };

                let conn_time: i64 = peer["conntime"].get_int64();
                let last_blck: i64 = peer["last_block"].get_int64();
                let last_recv: i64 = peer["lastrecv"].get_int64();
                let last_send: i64 = peer["lastsend"].get_int64();
                let last_trxn: i64 = peer["last_transaction"].get_int64();

                let min_ping: f64 = match peer["minping"].is_null() {
                    true   => -1.0,
                    false  => peer["minping"].get_real()
                };

                let ping: f64 = match peer["pingtime"].is_null() {
                    true   => -1.0,
                    false  => peer["pingtime"].get_real()
                };

                let addr: String = peer["addr"].get_str().to_string();

                let age:  String = match conn_time == 0 {
                    true   => "".to_string(),
                    false  => to_string(&((self.time_now - conn_time) / 60))
                };

                let sub_version: String = peer["subver"].get_str().to_string();

                let is_addr_relay_enabled: bool = match peer["addr_relay_enabled"].is_null() {
                    true   => false,
                    false  => peer["addr_relay_enabled"].get_bool()
                };

                let is_bip152_hb_from: bool = peer["bip152_hb_from"].get_bool();
                let is_bip152_hb_to:   bool = peer["bip152_hb_to"].get_bool();

                self.peers.push(
                    NetInfoRequestHandlerPeer {
                        addr:               addr.clone(),
                        sub_version,
                        conn_type,
                        network,
                        age:                age.clone(),
                        min_ping:           FloatOrd(min_ping),
                        ping:               FloatOrd(ping),
                        addr_processed,
                        addr_rate_limited,
                        last_blck,
                        last_recv,
                        last_send,
                        last_trxn,
                        id: peer_id,
                        mapped_as,
                        version,
                        is_addr_relay_enabled,
                        is_bip152_hb_from,
                        is_bip152_hb_to,
                        is_block_relay,
                        is_outbound,
                    }
                );

                self.max_addr_length = max(
                    addr.len() + 1,
                    self.max_addr_length
                );

                self.max_addr_processed_length = max(
                    to_string(&addr_processed).len(),
                    self.max_addr_processed_length
                );

                self.max_addr_rate_limited_length = max(
                    to_string(&addr_rate_limited).len(),
                    self.max_addr_rate_limited_length
                );

                self.max_age_length = max(
                    age.len(),
                    self.max_age_length
                );

                self.max_id_length = max(
                    to_string(&peer_id).len(),
                    self.max_id_length
                );

                self.is_asmap_on |= (mapped_as != 0);
            }
        }

        //  Generate report header.
        let mut result: String = format!(
            "{} client {}{} - server {}{}\n\n",
            PACKAGE_NAME,
            format_full_version(),
            self.chain_to_string(),
            networkinfo["protocolversion"].get_int(),
            networkinfo["subversion"].get_str()
        );

        // Report detailed peer connections list
        // sorted by direction and minimum pi ng
        // time.
        if self.details_requested() && !self.peers.is_empty() {

            self.peers.sort();

            let max_addr_processed_length    = self.max_addr_processed_length;
            let max_addr_rate_limited_length = self.max_addr_rate_limited_length;
            let max_age_length               = self.max_age_length;
            let max_id_length                = self.max_id_length;

            result += format!(
                "<-> type net mping ping send recv txn blkhb {:max_addr_processed_length$}{:max_addr_rate_limited_length$}{:max_age_length$}",
                "addrp",
                "addrl",
                "age"
            ).as_str();

            if self.is_asmap_on {
                result += " asmap ";
            }

            let maybe_max_addr_length = match self.is_address_selected() {
                true   => self.max_addr_length,
                false  => 0
            };

            result += format!(
                "{:max_id_length$} {:<maybe_max_addr_length$}{}\n",
                "id",
                match self.is_address_selected() {
                    true   => "address",
                    false  => ""
                },
                match self.is_version_selected() {
                    true   => "version",
                    false  => ""
                }
            ).as_str();

            for peer in self.peers.iter() {

                let version: String = to_string(&peer.version) + &peer.sub_version;

                result += format!("{:3} {:6} {:5}{:7}{:7}{:5}{:5}{:5}{:5}  {:2} {:width0$}{:width1$}{:width2$}{:width3$}{:width4$} {:<width5$}{}\n",

                    match peer.is_outbound {
                        true   => "out",
                        false  => "in"
                    },

                    self.connection_type_for_netinfo(&peer.conn_type),

                    peer.network,
                    self.ping_time_to_string(peer.min_ping.0),
                    self.ping_time_to_string(peer.ping.0),

                    match peer.last_send != 0 {
                        true   => to_string(&(self.time_now - peer.last_send)),
                        false  => "".to_string()
                    },

                    match peer.last_recv != 0 {
                        true   => to_string(&(self.time_now - peer.last_recv)),
                        false  => "".to_string()
                    },

                    match peer.last_trxn != 0 {
                        true   => to_string(&((self.time_now - peer.last_trxn) / 60)),
                        false  => match peer.is_block_relay {
                            true   => "*".to_string(),
                            false  => "".to_string()
                        }
                    },

                    match peer.last_blck != 0 {
                        true   => to_string(&((self.time_now - peer.last_blck) / 60)),
                        false  => "".to_string()
                    },

                    format!(
                        "{}{}",
                        match peer.is_bip152_hb_to {
                            true   => ".".to_string(),
                            false  => " ".to_string()
                        },
                        match peer.is_bip152_hb_from {
                            true   => "*".to_string(),
                            false  => " ".to_string()
                        }
                    ),

                    //-------------------------------
                    match peer.addr_processed != 0 {
                        true   => to_string(&peer.addr_processed),
                        false  => match peer.is_addr_relay_enabled {
                            true   => "".to_string(),
                            false  => ".".to_string()
                        }
                    },
                    match peer.addr_rate_limited != 0 {
                        true   => to_string(&peer.addr_rate_limited),
                        false  => "".to_string()
                    },
                    peer.age.clone(),
                    match self.is_asmap_on && peer.mapped_as != 0 {
                        true   => to_string(&peer.mapped_as),
                        false  => "".to_string()
                    },
                    peer.id.clone(),
                    match self.is_address_selected() {
                        true   => peer.addr.clone(),
                        false  => "".to_string()
                    },
                    match self.is_version_selected() && version != "0" {
                        true   => version,
                        false  => "".to_string()
                    },

                    //-------------------------------
                    width0 = self.max_addr_processed_length,
                    width1 = self.max_addr_rate_limited_length,
                    width2 = self.max_age_length,
                    width3 = match self.is_asmap_on {
                        true   => 7,
                        false  => 0
                    },
                    width4 = self.max_id_length,
                    width5 = match self.is_address_selected() {
                        true   => self.max_addr_length,
                        false  => 0
                    },
                ).as_str();
            }

            result += format!(
                "                     ms     ms  sec  sec  min  min{:max_age_length$}\n\n",
                "min"
            ).as_str();
        }

        // Report peer connection totals by type.
        result += "     ";

        let mut reachable_networks = Vec::<i8>::default();

        for network in networkinfo["networks"].get_values().unwrap().iter() {

            if network["reachable"].get_bool() {

                let network_name: &str = network["name"].get_str();
                let network_id:   i8 = self.network_string_to_id(network_name);

                if network_id == UNKNOWN_NETWORK {
                    continue;
                }

                //  column header
                result += format!("{:8}",network_name).as_str();
                reachable_networks.push(network_id);
            }
        }

        result += "   total   block";

        if self.manual_peers_count != 0 {
            result += "  manual";
        }

        let rows = vec!["in","out","total"];

        for i in 0..rows.len() {

            // row header
            result += format!("\n{:<5}",rows[i]).as_str();

            for n in reachable_networks.iter() {

                let n = *n;
                let n: usize = n.try_into().unwrap();

                // network peers count
                result += format!("{:8}",self.counts[i][n]).as_str();
            }

            // total peers count
            result += format!("   {:5}",self.counts[i][NETINFO_REQUEST_HANDLER_NETWORKS.len()]).as_str();

            if i == 1 {

                // the outbound row has two extra
                // columns for block relay and
                // manual peer counts
                result += format!("   {:5}",self.block_relay_peers_count).as_str();

                if self.manual_peers_count != 0 {
                    result += format!("   {:5}",self.manual_peers_count).as_str();
                }
            }
        }

        // Report local addresses, ports, and scores.
        result += "\n\nLocal addresses";

        let local_addrs: &Vec::<UniValue> = networkinfo["localaddresses"].get_values().unwrap();

        if local_addrs.is_empty() {
            result += ": n/a\n";
        } else {

            let mut max_addr_size: usize = 0;

            for addr in local_addrs.iter() {
                max_addr_size = max(addr["address"].get_str().len() + 1,max_addr_size);
            }

            for addr in local_addrs.iter() {
                result += format!(
                    "\n{:<max_addr_size$}    port {:6}    score {:6}",
                    addr["address"].get_str(),
                    addr["port"].get_int(),
                    addr["score"].get_int()
                ).as_str();
            }
        }

        Ok(
            jsonrpc_reply_obj(
                &UniValue::from(result.as_str()),
                &NULL_UNI_VALUE,
                &UniValue::from(1_i32)
            )
        )
    }
}

impl NetinfoRequestHandler {

    pub fn default_help_doc() -> String {

        todo!();

        formatdoc!{
            "-netinfo level|\"help\" 

            Returns a network peer connections dashboard with information from
            the remote server.

            This human-readable interface will change regularly and is not
            intended to be a stable API.

            Under the hood, -netinfo fetches the data by calling getpeerinfo and
            getnetworkinfo.

            {}

            Pass \"help\" to see this detailed help documentation.

            If more than one argument is passed, only the first one is read and
            parsed.

            Suggestion: use with the Linux watch(1) command for a live
            dashboard; see example below.

            Arguments:

            {}

                0 - Connection counts and local addresses
                1 - Like 0 but with a peers listing 
                    (without address or version columns)
                2 - Like 1 but with an address column
                3 - Like 1 but with a version column
                4 - Like 1 but with both address
                    and version columns

            2. help (string \"help\", optional)
               Print this help documentation instead of the dashboard.

            Result:

            {}
                \n\n
              Column   Description
              ------   -----------
              <->      Direction
                       \"in\"  - inbound connections are those initiated by the peer
                       \"out\" - outbound connections are those initiated by us
              type     Type of peer connection
                       \"full\"   - full relay, the default
                       \"block\"  - block relay; like full relay but does not relay transactions or addresses
                       \"manual\" - peer we manually added using RPC addnode or the -addnode/-connect config options
                       \"feeler\" - short-lived connection for testing addresses
                       \"addr\"   - address fetch; short-lived connection for requesting addresses

              net      Network the peer connected through 
                       (\"ipv4\", \"ipv6\", \"onion\", \"i2p\", or \"cjdns\")

              mping    Minimum observed ping time, in milliseconds (ms)

              ping     Last observed ping time, in milliseconds (ms)

              send     Time since last message sent to the peer, in seconds

              recv     Time since last message received from the peer, in seconds

              txn      Time since last novel transaction received from the peer 
                       and accepted into our mempool, in minutes \"*\" - the peer 
                       requested we not relay transactions to it (relaytxes is false)

              blk      Time since last novel block passing initial validity checks
                       received from the peer, in minutes

              hb       High-bandwidth BIP152 compact block relay
                       \".\" (to)   - we selected the peer as a high-bandwidth peer
                       \"*\" (from) - the peer selected us as a high-bandwidth peer

              addrp    Total number of addresses processed, excluding those
                       dropped due to rate limiting \".\" - we do not relay addresses to
                       this peer (addr_relay_enabled is false)

              addrl    Total number of addresses dropped due to rate limiting

              age      Duration of connection to the peer, in minutes

              asmap    Mapped AS (Autonomous System) number in the BGP route to 
                       the peer, used for diversifying peer selection (only 
                        displayed if the -asmap config option is set)

              id       Peer index, in increasing order of peer connections since
                       node startup

              address  IP address and port of the peer

              version  Peer version and subversion concatenated,
                       e.g. \"70016/Satoshi:21.0.0/\"

            * The connection counts table displays the number of peers by direction,
              network, and the totals for each, as well as two special outbound
              columns for block relay peers and manual peers.

            * The local addresses table lists each local address broadcast by the
              node, the port, and the score.
                
                
            Examples:

            Connection counts and local addresses only
            > bitcoin-cli -netinfo

            Compact peers listing
            > bitcoin-cli -netinfo 1

            Full dashboard

            {}

            Full live dashboard, adjust --interval or --no-title as needed (Linux)

            {}

            See this help

            > bitcoin-cli -netinfo help\n",

            formatdoc!(
                "An optional integer argument from 0 to {} can be passed for
                different peers listings; {} to 255 are parsed as {}.\n", 
                NETINFO_REQUEST_HANDLER_MAX_DETAIL_LEVEL,
                NETINFO_REQUEST_HANDLER_MAX_DETAIL_LEVEL,
                NETINFO_REQUEST_HANDLER_MAX_DETAIL_LEVEL
            ),

            formatdoc!(
                "1. level (integer 0-{}, optional) Specify the info level of the
                peers dashboard (default 0):\n", 
                NETINFO_REQUEST_HANDLER_MAX_DETAIL_LEVEL
            ),

            formatdoc!(
                "* The peers listing in levels 1-{} displays all of the peers
                sorted by direction and minimum ping time:\n\n", 
                NETINFO_REQUEST_HANDLER_MAX_DETAIL_LEVEL
            ),

            formatdoc!(
                "> bitcoin-cli -netinfo {}\n\n", 
                NETINFO_REQUEST_HANDLER_MAX_DETAIL_LEVEL
            ),
            formatdoc!(
                "> watch --interval 1 --no-title bitcoin-cli -netinfo {}\n\n", 
                NETINFO_REQUEST_HANDLER_MAX_DETAIL_LEVEL
            ),
        }
    }

    pub fn network_string_to_id(&self, str_: &str) -> i8 {
        
        for i in 0..NETINFO_REQUEST_HANDLER_NETWORKS.len() {
            if str_ == NETINFO_REQUEST_HANDLER_NETWORKS[i] {
                return i.try_into().unwrap();
            }
        }

        UNKNOWN_NETWORK
    }
    
    pub fn details_requested(&self) -> bool {
        
        self.details_level > 0 && self.details_level < 5
    }
    
    pub fn is_address_selected(&self) -> bool {
        
        self.details_level == 2 || self.details_level == 4
    }
    
    pub fn is_version_selected(&self) -> bool {
        
        self.details_level == 3 || self.details_level == 4
    }
    
    pub fn chain_to_string(&self) -> &'static str {
        
        match G_ARGS
            .lock()
            //.unwrap()
            .get_chain_name()
            .unwrap()
            .as_str() 
        {
            base_chain_params::TESTNET => " testnet",
            base_chain_params::SIGNET  => " signet",
            base_chain_params::REGTEST => " regtest",
            _ => "",
        }
    }
    
    pub fn ping_time_to_string(&self, seconds: f64) -> String {
        
        if seconds < 0.0 {
            return "".to_string();
        }

        let milliseconds: f64 = (1000.0 * seconds).round();

        match milliseconds > 999999.0 {
            true   => "-".to_string(),
            false  => to_string(&milliseconds)
        }
    }
    
    pub fn connection_type_for_netinfo(&self, conn_type: &str) -> String {

        let res = match conn_type {
            "outbound-full-relay" => "full",
            "block-relay-only"    => "block",
            "addr-fetch"          => "addr",
            "manual" | "feeler"   => conn_type,
            _                     => "",
        };

        res.to_string()
    }
}
