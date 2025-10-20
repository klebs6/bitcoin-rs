// ---------------- [ File: bitcoin-tor/src/controller.rs ]
crate::ix!();

/****** Bitcoin specific TorController implementation ********/

/**
  | Controller that connects to Tor control
  | socket, authenticate, then create
  | and maintain an ephemeral onion service.
  |
  */
pub struct TorController {
    base:               *mut libevent_sys::event_base,
    tor_control_center: String,
    conn:               TorControlConnection,
    private_key:        String,
    service_id:         String,
    reconnect:          bool,
    reconnect_ev:       *mut libevent_sys::event, // default = nullptr
    reconnect_timeout:  f32,
    service:            Service,
    target:             Service,

    /**
      | Cookie for SAFECOOKIE auth
      |
      */
    cookie:             Vec<u8>,


    /**
      | ClientNonce for SAFECOOKIE auth
      |
      */
    client_nonce:       Vec<u8>,
}

impl Default for TorController {
    
    fn default() -> Self {
        todo!();
        /*
            : conn{nullptr} 
            // Used for testing only.
        */
    }
}

impl Drop for TorController {

    fn drop(&mut self) {
        todo!();
        /*
            if (reconnect_ev) {
            event_free(reconnect_ev);
            reconnect_ev = nullptr;
        }
        if (service.IsValid()) {
            RemoveLocal(service);
        }
        */
    }
}

impl TorController {
    
    pub fn new(
        base:               *mut libevent_sys::event_base,
        tor_control_center: &String,
        target:             &Service) -> Self {
    
        todo!();
        /*
           :
        base(_base),
        m_tor_control_center(tor_control_center), conn(base), reconnect(true), reconnect_ev(0),
        reconnect_timeout(RECONNECT_TIMEOUT_START),
        m_target(target)

        reconnect_ev = event_new(base, -1, 0, reconnect_cb, this);
        if (!reconnect_ev)
            LogPrintf("tor: Failed to create event for reconnection: out of memory?\n");
        // Start connection attempts immediately
        if (!conn.Connect(m_tor_control_center, std::bind(&TorController::connected_cb, this, std::placeholders::_1),
             std::bind(&TorController::disconnected_cb, this, std::placeholders::_1) )) {
            LogPrintf("tor: Initiating connection to Tor control port %s failed\n", m_tor_control_center);
        }
        // Read service private key if cached
        std::pair<bool,std::string> pkf = ReadBinaryFile(GetPrivateKeyFile());
        if (pkf.first) {
            LogPrint(LogFlags::TOR, "tor: Reading cached private key from %s\n", fs::PathToString(GetPrivateKeyFile()));
            private_key = pkf.second;
        }
        */
    }
    
    /**
      | Callback for ADD_ONION result
      |
      */
    pub fn add_onion_cb(&mut self, 
        conn:  &mut TorControlConnection,
        reply: &TorControlReply)  {
        
        todo!();
        /*
            if (reply.code == 250) {
            LogPrint(LogFlags::TOR, "tor: ADD_ONION successful\n");
            for (const std::string &s : reply.lines) {
                std::map<std::string,std::string> m = ParseTorReplyMapping(s);
                std::map<std::string,std::string>::iterator i;
                if ((i = m.find("ServiceID")) != m.end())
                    service_id = i->second;
                if ((i = m.find("PrivateKey")) != m.end())
                    private_key = i->second;
            }
            if (service_id.empty()) {
                LogPrintf("tor: Error parsing ADD_ONION parameters:\n");
                for (const std::string &s : reply.lines) {
                    LogPrintf("    %s\n", SanitizeString(s));
                }
                return;
            }
            service = LookupNumeric(std::string(service_id+".onion"), Params().GetDefaultPort());
            LogPrintf("tor: Got service ID %s, advertising service %s\n", service_id, service.ToString());
            if (WriteBinaryFile(GetPrivateKeyFile(), private_key)) {
                LogPrint(LogFlags::TOR, "tor: Cached service private key to %s\n", fs::PathToString(GetPrivateKeyFile()));
            } else {
                LogPrintf("tor: Error writing service private key to %s\n", fs::PathToString(GetPrivateKeyFile()));
            }
            AddLocal(service, LOCAL_MANUAL);
            // ... onion requested - keep connection open
        } else if (reply.code == 510) { // 510 Unrecognized command
            LogPrintf("tor: Add onion failed with unrecognized command (You probably need to upgrade Tor)\n");
        } else {
            LogPrintf("tor: Add onion failed; error code %d\n", reply.code);
        }
        */
    }
    
    /**
      | Callback for AUTHENTICATE result
      |
      */
    pub fn auth_cb(&mut self, 
        conn:  &mut TorControlConnection,
        reply: &TorControlReply)  {
        
        todo!();
        /*
            if (reply.code == 250) {
            LogPrint(LogFlags::TOR, "tor: Authentication successful\n");

            // Now that we know Tor is running setup the proxy for onion addresses
            // if -onion isn't set to something else.
            if (gArgs.GetArg("-onion", "") == "") {
                CService resolved(LookupNumeric("127.0.0.1", 9050));
                proxyType addrOnion = proxyType(resolved, true);
                SetProxy(NET_ONION, addrOnion);
                SetReachable(NET_ONION, true);
            }

            // Finally - now create the service
            if (private_key.empty()) { // No private key, generate one
                private_key = "NEW:ED25519-V3"; // Explicitly request key type - see issue #9214
            }
            // Request onion service, redirect port.
            // Note that the 'virtual' port is always the default port to avoid decloaking nodes using other ports.
            _conn.Command(strprintf("ADD_ONION %s Port=%i,%s", private_key, Params().GetDefaultPort(), m_target.ToStringIPPort()),
                std::bind(&TorController::add_onion_cb, this, std::placeholders::_1, std::placeholders::_2));
        } else {
            LogPrintf("tor: Authentication failed\n");
        }
        */
    }
    
    /**
      | Callback for AUTHCHALLENGE result
      |
      */
    pub fn authchallenge_cb(&mut self, 
        conn:  &mut TorControlConnection,
        reply: &TorControlReply)  {
        
        todo!();
        /*
            if (reply.code == 250) {
            LogPrint(LogFlags::TOR, "tor: SAFECOOKIE authentication challenge successful\n");
            std::pair<std::string,std::string> l = SplitTorReplyLine(reply.lines[0]);
            if (l.first == "AUTHCHALLENGE") {
                std::map<std::string,std::string> m = ParseTorReplyMapping(l.second);
                if (m.empty()) {
                    LogPrintf("tor: Error parsing AUTHCHALLENGE parameters: %s\n", SanitizeString(l.second));
                    return;
                }
                std::vector<uint8_t> serverHash = ParseHex(m["SERVERHASH"]);
                std::vector<uint8_t> serverNonce = ParseHex(m["SERVERNONCE"]);
                LogPrint(LogFlags::TOR, "tor: AUTHCHALLENGE ServerHash %s ServerNonce %s\n", HexStr(serverHash), HexStr(serverNonce));
                if (serverNonce.size() != 32) {
                    LogPrintf("tor: ServerNonce is not 32 bytes, as required by spec\n");
                    return;
                }

                std::vector<uint8_t> computedServerHash = ComputeResponse(TOR_SAFE_SERVERKEY, cookie, clientNonce, serverNonce);
                if (computedServerHash != serverHash) {
                    LogPrintf("tor: ServerHash %s does not match expected ServerHash %s\n", HexStr(serverHash), HexStr(computedServerHash));
                    return;
                }

                std::vector<uint8_t> computedClientHash = ComputeResponse(TOR_SAFE_CLIENTKEY, cookie, clientNonce, serverNonce);
                _conn.Command("AUTHENTICATE " + HexStr(computedClientHash), std::bind(&TorController::auth_cb, this, std::placeholders::_1, std::placeholders::_2));
            } else {
                LogPrintf("tor: Invalid reply to AUTHCHALLENGE\n");
            }
        } else {
            LogPrintf("tor: SAFECOOKIE authentication challenge failed\n");
        }
        */
    }
    
    /**
      | Callback for PROTOCOLINFO result
      |
      */
    pub fn protocolinfo_cb(&mut self, 
        conn:  &mut TorControlConnection,
        reply: &TorControlReply)  {
        
        todo!();
        /*
            if (reply.code == 250) {
            std::set<std::string> methods;
            std::string cookiefile;
            /*
             * 250-AUTH METHODS=COOKIE,SAFECOOKIE COOKIEFILE="/home/x/.tor/control_auth_cookie"
             * 250-AUTH METHODS=NULL
             * 250-AUTH METHODS=HASHEDPASSWORD
             */
            for (const std::string &s : reply.lines) {
                std::pair<std::string,std::string> l = SplitTorReplyLine(s);
                if (l.first == "AUTH") {
                    std::map<std::string,std::string> m = ParseTorReplyMapping(l.second);
                    std::map<std::string,std::string>::iterator i;
                    if ((i = m.find("METHODS")) != m.end())
                        boost::split(methods, i->second, boost::is_any_of(","));
                    if ((i = m.find("COOKIEFILE")) != m.end())
                        cookiefile = i->second;
                } else if (l.first == "VERSION") {
                    std::map<std::string,std::string> m = ParseTorReplyMapping(l.second);
                    std::map<std::string,std::string>::iterator i;
                    if ((i = m.find("Tor")) != m.end()) {
                        LogPrint(LogFlags::TOR, "tor: Connected to Tor version %s\n", i->second);
                    }
                }
            }
            for (const std::string &s : methods) {
                LogPrint(LogFlags::TOR, "tor: Supported authentication method: %s\n", s);
            }
            // Prefer NULL, otherwise SAFECOOKIE. If a password is provided, use HASHEDPASSWORD
            /* Authentication:
             *   cookie:   hex-encoded ~/.tor/control_auth_cookie
             *   password: "password"
             */
            std::string torpassword = gArgs.GetArg("-torpassword", "");
            if (!torpassword.empty()) {
                if (methods.count("HASHEDPASSWORD")) {
                    LogPrint(LogFlags::TOR, "tor: Using HASHEDPASSWORD authentication\n");
                    boost::replace_all(torpassword, "\"", "\\\"");
                    _conn.Command("AUTHENTICATE \"" + torpassword + "\"", std::bind(&TorController::auth_cb, this, std::placeholders::_1, std::placeholders::_2));
                } else {
                    LogPrintf("tor: Password provided with -torpassword, but HASHEDPASSWORD authentication is not available\n");
                }
            } else if (methods.count("NULL")) {
                LogPrint(LogFlags::TOR, "tor: Using NULL authentication\n");
                _conn.Command("AUTHENTICATE", std::bind(&TorController::auth_cb, this, std::placeholders::_1, std::placeholders::_2));
            } else if (methods.count("SAFECOOKIE")) {
                // Cookie: hexdump -e '32/1 "%02x""\n"'  ~/.tor/control_auth_cookie
                LogPrint(LogFlags::TOR, "tor: Using SAFECOOKIE authentication, reading cookie authentication from %s\n", cookiefile);
                std::pair<bool,std::string> status_cookie = ReadBinaryFile(fs::PathFromString(cookiefile), TOR_COOKIE_SIZE);
                if (status_cookie.first && status_cookie.second.size() == TOR_COOKIE_SIZE) {
                    // _conn.Command("AUTHENTICATE " + HexStr(status_cookie.second), std::bind(&TorController::auth_cb, this, std::placeholders::_1, std::placeholders::_2));
                    cookie = std::vector<uint8_t>(status_cookie.second.begin(), status_cookie.second.end());
                    clientNonce = std::vector<uint8_t>(TOR_NONCE_SIZE, 0);
                    GetRandBytes(clientNonce.data(), TOR_NONCE_SIZE);
                    _conn.Command("AUTHCHALLENGE SAFECOOKIE " + HexStr(clientNonce), std::bind(&TorController::authchallenge_cb, this, std::placeholders::_1, std::placeholders::_2));
                } else {
                    if (status_cookie.first) {
                        LogPrintf("tor: Authentication cookie %s is not exactly %i bytes, as is required by the spec\n", cookiefile, TOR_COOKIE_SIZE);
                    } else {
                        LogPrintf("tor: Authentication cookie %s could not be opened (check permissions)\n", cookiefile);
                    }
                }
            } else if (methods.count("HASHEDPASSWORD")) {
                LogPrintf("tor: The only supported authentication mechanism left is password, but no password provided with -torpassword\n");
            } else {
                LogPrintf("tor: No supported authentication method\n");
            }
        } else {
            LogPrintf("tor: Requesting protocol info failed\n");
        }
        */
    }
    
    /**
      | Callback after successful connection
      |
      */
    pub fn connected_cb(&mut self, conn: &mut TorControlConnection)  {
        
        todo!();
        /*
            reconnect_timeout = RECONNECT_TIMEOUT_START;
        // First send a PROTOCOLINFO command to figure out what authentication is expected
        if (!_conn.Command("PROTOCOLINFO 1", std::bind(&TorController::protocolinfo_cb, this, std::placeholders::_1, std::placeholders::_2)))
            LogPrintf("tor: Error sending initial protocolinfo command\n");
        */
    }
    
    /**
      | Callback after connection lost or failed
      | connection attempt
      |
      */
    pub fn disconnected_cb(&mut self, conn: &mut TorControlConnection)  {
        
        todo!();
        /*
            // Stop advertising service when disconnected
        if (service.IsValid())
            RemoveLocal(service);
        service = CService();
        if (!reconnect)
            return;

        LogPrint(LogFlags::TOR, "tor: Not connected to Tor control port %s, trying to reconnect\n", m_tor_control_center);

        // Single-shot timer for reconnect. Use exponential backoff.
        struct timeval time = MillisToTimeval(int64_t(reconnect_timeout * 1000.0));
        if (reconnect_ev)
            event_add(reconnect_ev, &time);
        reconnect_timeout *= RECONNECT_TIMEOUT_EXP;
        */
    }
    
    /**
      | Reconnect, after getting disconnected
      |
      */
    pub fn reconnect(&mut self)  {
        
        todo!();
        /*
            /* Try to reconnect and reestablish if we get booted - for example, Tor
         * may be restarting.
         */
        if (!conn.Connect(m_tor_control_center, std::bind(&TorController::connected_cb, this, std::placeholders::_1),
             std::bind(&TorController::disconnected_cb, this, std::placeholders::_1) )) {
            LogPrintf("tor: Re-initiating connection to Tor control port %s failed\n", m_tor_control_center);
        }
        */
    }
    
    /**
      | Get name of file to store private key
      | in
      |
      */
    pub fn get_private_key_file(&mut self) -> Box<Path> {
        
        todo!();
        /*
            return gArgs.GetDataDirNet() / "onion_v3_private_key";
        */
    }
    
    /**
      | Callback for reconnect timer
      |
      */
    pub fn reconnect_cb(&mut self, 
        fd:   EvutilSocket,
        what: i16,
        arg:  *mut c_void)  {
        
        todo!();
        /*
            TorController *self = static_cast<TorController*>(arg);
        self->Reconnect();
        */
    }
}
