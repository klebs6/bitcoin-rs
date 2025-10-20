// ---------------- [ File: bitcoin-sam/src/i2p.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/i2p.h]
//-------------------------------------------[.cpp/bitcoin/src/i2p.cpp]

/**
  | Binary data.
  |
  */
pub type Binary = Vec<u8>;

/**
  | An established connection with another
  | peer.
  |
  */
#[derive(Default)]
pub struct Connection {

    /**
      | Connected socket.
      |
      */
    pub sock: Box<Sock>,

    /**
      | Our I2P address.
      |
      */
    pub me:   Service,

    /**
      | The peer's I2P address.
      |
      */
    pub peer: Service,
}

/**
  | Generic interface for managing an event
  | handler or callback function registered with
  | another interface. Has a single disconnect
  | method to cancel the registration and prevent
  | any future notifications.
  */
pub trait Handler: Disconnect { }

pub trait Disconnect {

    /**
      | Disconnect the handler.
      |
      */
    fn disconnect(&mut self);
}

//-------------------------------------------[.cpp/bitcoin/src/interfaces/handler.h]
//-------------------------------------------[.cpp/bitcoin/src/interfaces/handler.cpp]

pub type CleanupHandlerFn = fn() -> ();

impl From<Connection> for Box<dyn Handler> {

    /**
      | Return handler wrapping a boost signal
      | connection.
      |
      */
    fn from(connection: Connection) -> Box<dyn Handler> {
        
        todo!();
            /*
                return std::make_unique<HandlerImpl>(std::move(connection));
            */
    }
}

impl From<CleanupHandlerFn> for Box<dyn Handler> {

    /**
      | Return handler wrapping a cleanup function.
      |
      */
    fn from(cleanup: CleanupHandlerFn) -> Box<dyn Handler> {
        
        todo!();
            /*
                return std::make_unique<CleanupHandler>(std::move(cleanup));
            */
    }
}

/**
  | Swap Standard Base64 <-> I2P Base64.
  | 
  | Standard Base64 uses `+` and `/` as last
  | two characters of its alphabet.
  | 
  | I2P Base64 uses `-` and `~` respectively.
  | 
  | So it is easy to detect in which one is
  | the input and convert to the other.
  | 
  | -----------
  | @param[in] from
  | 
  | Input to convert.
  | 
  | -----------
  | @return
  | 
  | converted `from`
  |
  */
pub fn swap_base64(from: &str) -> String {
    
    todo!();
        /*
            std::string to;
        to.resize(from.size());
        for (size_t i = 0; i < from.size(); ++i) {
            switch (from[i]) {
            case '-':
                to[i] = '+';
                break;
            case '~':
                to[i] = '/';
                break;
            case '+':
                to[i] = '-';
                break;
            case '/':
                to[i] = '~';
                break;
            default:
                to[i] = from[i];
                break;
            }
        }
        return to;
        */
}

/**
  | Decode an I2P-style Base64 string.
  | 
  | -----------
  | @param[in] i2p_b64
  | 
  | I2P-style Base64 string.
  | 
  | -----------
  | @return
  | 
  | decoded `i2p_b64` @throw std::runtime_error
  | if decoding fails
  |
  */
pub fn decode_i2p_base64(i2p_b64: &str) -> Binary {
    
    todo!();
        /*
            const std::string& std_b64 = SwapBase64(i2p_b64);
        bool invalid;
        Binary decoded = DecodeBase64(std_b64.c_str(), &invalid);
        if (invalid) {
            throw std::runtime_error(strprintf("Cannot decode Base64: \"%s\"", i2p_b64));
        }
        return decoded;
        */
}

/**
  | Derive the .b32.i2p address of an I2P
  | destination (binary).
  | 
  | -----------
  | @param[in] dest
  | 
  | I2P destination.
  | 
  | -----------
  | @return
  | 
  | the address that corresponds to `dest`
  | @throw std::runtime_error if conversion
  | fails
  |
  */
pub fn dest_bin_to_addr(dest: &Binary) -> NetAddr {
    
    todo!();
        /*
            CSHA256 hasher;
        hasher.Write(dest.data(), dest.size());
        unsigned char hash[CSHA256::OUTPUT_SIZE];
        hasher.Finalize(hash);

        CNetAddr addr;
        const std::string addr_str = EncodeBase32(hash, false) + ".b32.i2p";
        if (!addr.SetSpecial(addr_str)) {
            throw std::runtime_error(strprintf("Cannot parse I2P address: \"%s\"", addr_str));
        }

        return addr;
        */
}

/**
  | Derive the .b32.i2p address of an I2P
  | destination (I2P-style Base64).
  | 
  | -----------
  | @param[in] dest
  | 
  | I2P destination.
  | 
  | -----------
  | @return
  | 
  | the address that corresponds to `dest`
  | @throw std::runtime_error if conversion
  | fails
  |
  */
pub fn dest_b64to_addr(dest: &str) -> NetAddr {
    
    todo!();
        /*
            const Binary& decoded = DecodeI2PBase64(dest);
        return DestBinToAddr(decoded);
        */
}

/**
  | The maximum size of an incoming message
  | from the I2P SAM proxy (in bytes).
  | 
  | Used to avoid a runaway proxy from sending
  | us an "unlimited" amount of data without
  | a terminator.
  | 
  | The longest known message is ~1400 bytes,
  | so this is high enough not to be triggered
  | during normal operation, yet low enough
  | to avoid a malicious proxy from filling
  | our memory.
  |
  */
pub const SAM_MAX_MSG_SIZE: usize = 65536;

/**
  | I2P SAM session.
  |
  */
pub struct SAMSession {

    /**
      | The name of the file where this peer's
      | private key is stored (in binary).
      |
      */
    private_key_file: Box<Path>,


    /**
      | The host and port of the SAM control service.
      |
      */
    control_host:     Service,


    /**
      | Cease network activity when this is
      | signaled.
      |
      */
    interrupt:        *const ThreadInterrupt,


    /**
      | Mutex protecting the members that can
      | be concurrently accessed.
      |
      */
    mutex:            Arc<Mutex<SAMSessionInner>>,


    /**
      | The private key of this peer. @see The
      | reply to the "DEST GENERATE" command
      | in https://geti2p.net/en/docs/api/samv3
      | #[GUARDED_BY(m_mutex)]
      |
      */
    private_key:      Binary,

}

unsafe impl Send for SAMSession {}
unsafe impl Sync for SAMSession {}

impl Drop for SAMSession {

    /**
      | Destroy the session, closing the internally
      | used sockets. The sockets that have
      | been returned by `Accept()` or `Connect()`
      | will not be closed, but they will be closed
      | by the SAM proxy because the session
      | is destroyed. So they will return an
      | error next time we try to read or write
      | to them.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
           LOCK(m_mutex);
           Disconnect();
        */
    }
}

impl SAMSession {

    /**
      | Construct a session. This will not initiate
      | any IO, the session will be lazily created
      | later when first used.
      | 
      | -----------
      | @param[in] private_key_file
      | 
      | Path to a private key file. If the file
      | does not exist then the private key will
      | be generated and saved into the file.
      | ----------
      | @param[in] control_host
      | 
      | Location of the SAM proxy.
      | ----------
      | @param[in,out] interrupt
      | 
      | If this is signaled then all operations
      | are canceled as soon as possible and
      | executing methods throw an exception.
      | Notice: only a pointer to the `CThreadInterrupt`
      | object is saved, so it must not be destroyed
      | earlier than this `Session` object.
      |
      */
    pub fn new(
        private_key_file: &Path,
        control_host:     &Service,
        interrupt:        Amo<ThreadInterrupt>) -> Self {
    
        todo!();
        /*


            : m_private_key_file(private_key_file), m_control_host(control_host), m_interrupt(interrupt),
          m_control_sock(std::make_unique<Sock>(INVALID_SOCKET))
        */
    }
    
    /**
      | Start listening for an incoming connection.
      | 
      | -----------
      | @param[out] conn
      | 
      | Upon successful completion the `sock`
      | and `me` members will be set to the listening
      | socket and address.
      | 
      | -----------
      | @return
      | 
      | true on success
      |
      */
    pub fn listen(&mut self, conn: &mut Connection) -> bool {
        
        todo!();
        /*
            try {
            LOCK(m_mutex);
            CreateIfNotCreatedAlready();
            conn.me = m_my_addr;
            conn.sock = StreamAccept();
            return true;
        } catch (const std::runtime_error& e) {
            Log("Error listening: %s", e.what());
            CheckControlSock();
        }
        return false;
        */
    }
    
    /**
      | Wait for and accept a new incoming connection.
      | 
      | -----------
      | @param[in,out] conn
      | 
      | The `sock` member is used for waiting
      | and accepting. Upon successful completion
      | the `peer` member will be set to the address
      | of the incoming peer.
      | 
      | -----------
      | @return
      | 
      | true on success
      |
      */
    pub fn accept(&mut self, conn: &mut Connection) -> bool {
        
        todo!();
        /*
            try {
            while (!*m_interrupt) {
                Sock::Event occurred;
                if (!conn.sock->Wait(MAX_WAIT_FOR_IO, Sock::RECV, &occurred)) {
                    throw std::runtime_error("wait on socket failed");
                }

                if ((occurred & Sock::RECV) == 0) {
                    // Timeout, no incoming connections within MAX_WAIT_FOR_IO.
                    continue;
                }

                const std::string& peer_dest =
                    conn.sock->RecvUntilTerminator('\n', MAX_WAIT_FOR_IO, *m_interrupt, MAX_MSG_SIZE);

                conn.peer = CService(DestB64ToAddr(peer_dest), I2P_SAM31_PORT);

                return true;
            }
        } catch (const std::runtime_error& e) {
            Log("Error accepting: %s", e.what());
            CheckControlSock();
        }
        return false;
        */
    }
    
    /**
      | Connect to an I2P peer.
      | 
      | -----------
      | @param[in] to
      | 
      | Peer to connect to.
      | ----------
      | @param[out] conn
      | 
      | Established connection. Only set if
      | `true` is returned.
      | ----------
      | @param[out] proxy_error
      | 
      | If an error occurs due to proxy or general
      | network failure, then this is set to
      | `true`. If an error occurs due to unreachable
      | peer (likely peer is down), then it is
      | set to `false`. Only set if `false` is
      | returned.
      | 
      | -----------
      | @return
      | 
      | true on success
      |
      */
    pub fn connect(&mut self, 
        to:          &Service,
        conn:        &mut Connection,
        proxy_error: &mut bool) -> bool {
        
        todo!();
        /*
            // Refuse connecting to arbitrary ports. We don't specify any destination port to the SAM proxy
        // when connecting (SAM 3.1 does not use ports) and it forces/defaults it to I2P_SAM31_PORT.
        if (to.GetPort() != I2P_SAM31_PORT) {
            proxy_error = false;
            return false;
        }

        proxy_error = true;

        std::string session_id;
        std::unique_ptr<Sock> sock;
        conn.peer = to;

        try {
            {
                LOCK(m_mutex);
                CreateIfNotCreatedAlready();
                session_id = m_session_id;
                conn.me = m_my_addr;
                sock = Hello();
            }

            const Reply& lookup_reply =
                SendRequestAndGetReply(*sock, strprintf("NAMING LOOKUP NAME=%s", to.ToStringIP()));

            const std::string& dest = lookup_reply.Get("VALUE");

            const Reply& connect_reply = SendRequestAndGetReply(
                *sock, strprintf("STREAM CONNECT ID=%s DESTINATION=%s SILENT=false", session_id, dest),
                false);

            const std::string& result = connect_reply.Get("RESULT");

            if (result == "OK") {
                conn.sock = std::move(sock);
                return true;
            }

            if (result == "INVALID_ID") {
                LOCK(m_mutex);
                Disconnect();
                throw std::runtime_error("Invalid session id");
            }

            if (result == "CANT_REACH_PEER" || result == "TIMEOUT") {
                proxy_error = false;
            }

            throw std::runtime_error(strprintf("\"%s\"", connect_reply.full));
        } catch (const std::runtime_error& e) {
            Log("Error connecting to %s: %s", to.ToString(), e.what());
            CheckControlSock();
            return false;
        }
        */
    }
    
    /**
      | Log a message in the `LogFlags::I2P` category.
      | 
      | -----------
      | @param[in] fmt
      | 
      | printf(3)-like format string.
      | ----------
      | @param[in] args
      | 
      | printf(3)-like arguments that correspond
      | to `fmt`.
      |
      */
    pub fn log<Args>(&self, 
        fmt:  &str,
        args: &Args)  {
    
        todo!();
        /*
            LogPrint(LogFlags::I2P, "I2P: %s\n", tfm::format(fmt, args...));
        */
    }
    
    /**
      | Send request and get a reply from the
      | SAM proxy.
      | 
      | -----------
      | @param[in] sock
      | 
      | A socket that is connected to the SAM
      | proxy.
      | ----------
      | @param[in] request
      | 
      | Raw request to send, a newline terminator
      | is appended to it.
      | ----------
      | @param[in] check_result_ok
      | 
      | If true then after receiving the reply
      | a check is made whether it contains "RESULT=OK"
      | and an exception is thrown if it does
      | not. @throws std::runtime_error if
      | an error occurs
      |
      */
    pub fn send_request_and_get_reply(&self, 
        sock:            &Sock,
        request:         &str,
        check_result_ok: Option<bool>) -> SAMSessionReply {

        let check_result_ok: bool = check_result_ok.unwrap_or(true);
        
        todo!();
        /*
            sock.SendComplete(request + "\n", MAX_WAIT_FOR_IO, *m_interrupt);

        Reply reply;

        // Don't log the full "SESSION CREATE ..." because it contains our private key.
        reply.request = request.substr(0, 14) == "SESSION CREATE" ? "SESSION CREATE ..." : request;

        // It could take a few minutes for the I2P router to reply as it is querying the I2P network
        // (when doing name lookup, for example). Notice: `RecvUntilTerminator()` is checking
        // `m_interrupt` more often, so we would not be stuck here for long if `m_interrupt` is
        // signaled.
        static constexpr auto recv_timeout = 3min;

        reply.full = sock.RecvUntilTerminator('\n', recv_timeout, *m_interrupt, MAX_MSG_SIZE);

        for (const auto& kv : spanparsing::Split(reply.full, ' ')) {
            const auto& pos = std::find(kv.begin(), kv.end(), '=');
            if (pos != kv.end()) {
                reply.keys.emplace(std::string{kv.begin(), pos}, std::string{pos + 1, kv.end()});
            } else {
                reply.keys.emplace(std::string{kv.begin(), kv.end()}, std::nullopt);
            }
        }

        if (check_result_ok && reply.Get("RESULT") != "OK") {
            throw std::runtime_error(
                strprintf("Unexpected reply to \"%s\": \"%s\"", request, reply.full));
        }

        return reply;
        */
    }
    
    /**
      | Open a new connection to the SAM proxy.
      | 
      | 
      | -----------
      | @return
      | 
      | a connected socket @throws std::runtime_error
      | if an error occurs
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mutex)]
    pub fn hello(&self) -> Box<Sock> {
        
        todo!();
        /*
            auto sock = CreateSock(m_control_host);

        if (!sock) {
            throw std::runtime_error("Cannot create socket");
        }

        if (!ConnectSocketDirectly(m_control_host, *sock, nConnectTimeout, true)) {
            throw std::runtime_error(strprintf("Cannot connect to %s", m_control_host.ToString()));
        }

        SendRequestAndGetReply(*sock, "HELLO VERSION MIN=3.1 MAX=3.1");

        return sock;
        */
    }
    
    /**
      | Check the control socket for errors
      | and possibly disconnect.
      |
      */
    pub fn check_control_sock(&mut self)  {
        
        todo!();
        /*
            LOCK(m_mutex);

        std::string errmsg;
        if (!m_control_sock->IsConnected(errmsg)) {
            Log("Control socket error: %s", errmsg);
            Disconnect();
        }
        */
    }
    
    /**
      | Generate a new destination with the
      | SAM proxy and set `m_private_key` to
      | it.
      | 
      | -----------
      | @param[in] sock
      | 
      | Socket to use for talking to the SAM proxy.
      | @throws std::runtime_error if an error
      | occurs
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mutex)]
    pub fn dest_generate(&mut self, sock: &Sock)  {
        
        todo!();
        /*
            // https://geti2p.net/spec/common-structures#key-certificates
        // "7" or "EdDSA_SHA512_Ed25519" - "Recent Router Identities and Destinations".
        // Use "7" because i2pd <2.24.0 does not recognize the textual form.
        const Reply& reply = SendRequestAndGetReply(sock, "DEST GENERATE SIGNATURE_TYPE=7", false);

        m_private_key = DecodeI2PBase64(reply.Get("PRIV"));
        */
    }
    
    /**
      | Generate a new destination with the
      | SAM proxy, set `m_private_key` to it
      | and save it on disk to `m_private_key_file`.
      | 
      | -----------
      | @param[in] sock
      | 
      | Socket to use for talking to the SAM proxy.
      | @throws std::runtime_error if an error
      | occurs
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mutex)]
    pub fn generate_and_save_private_key(&mut self, sock: &Sock)  {
        
        todo!();
        /*
            DestGenerate(sock);

        // umask is set to 077 in init.cpp, which is ok (unless -sysperms is given)
        if (!WriteBinaryFile(m_private_key_file,
                             std::string(m_private_key.begin(), m_private_key.end()))) {
            throw std::runtime_error(
                strprintf("Cannot save I2P private key to %s", fs::quoted(fs::PathToString(m_private_key_file))));
        }
        */
    }
    
    /**
      | Derive own destination from `m_private_key`.
      | @see https://geti2p.net/spec/common-structures#destination
      | 
      | 
      | -----------
      | @return
      | 
      | an I2P destination
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mutex)]
    pub fn my_destination(&self) -> Binary {
        
        todo!();
        /*
            // From https://geti2p.net/spec/common-structures#destination:
        // "They are 387 bytes plus the certificate length specified at bytes 385-386, which may be
        // non-zero"
        static constexpr size_t DEST_LEN_BASE = 387;
        static constexpr size_t CERT_LEN_POS = 385;

        uint16_t cert_len;
        memcpy(&cert_len, &m_private_key.at(CERT_LEN_POS), sizeof(cert_len));
        cert_len = be16toh(cert_len);

        const size_t dest_len = DEST_LEN_BASE + cert_len;

        return Binary{m_private_key.begin(), m_private_key.begin() + dest_len};
        */
    }
    
    /**
      | Create the session if not already created.
      | Reads the private key file and connects
      | to the
      | 
      | SAM proxy. @throws std::runtime_error
      | if an error occurs
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mutex)]
    pub fn create_if_not_created_already(&mut self)  {
        
        todo!();
        /*
            std::string errmsg;
        if (m_control_sock->IsConnected(errmsg)) {
            return;
        }

        Log("Creating SAM session with %s", m_control_host.ToString());

        auto sock = Hello();

        const auto& [read_ok, data] = ReadBinaryFile(m_private_key_file);
        if (read_ok) {
            m_private_key.assign(data.begin(), data.end());
        } else {
            GenerateAndSavePrivateKey(*sock);
        }

        const std::string& session_id = GetRandHash().GetHex().substr(0, 10); // full is an overkill, too verbose in the logs
        const std::string& private_key_b64 = SwapBase64(EncodeBase64(m_private_key));

        SendRequestAndGetReply(*sock, strprintf("SESSION CREATE STYLE=STREAM ID=%s DESTINATION=%s",
                                                session_id, private_key_b64));

        m_my_addr = CService(DestBinToAddr(MyDestination()), I2P_SAM31_PORT);
        m_session_id = session_id;
        m_control_sock = std::move(sock);

        LogPrintf("I2P: SAM session created: session id=%s, my address=%s\n", m_session_id,
                  m_my_addr.ToString());
        */
    }
    
    /**
      | Open a new connection to the SAM proxy
      | and issue "STREAM ACCEPT" request using
      | the existing session id.
      | 
      | 
      | -----------
      | @return
      | 
      | the idle socket that is waiting for a
      | peer to connect to us @throws std::runtime_error
      | if an error occurs
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mutex)]
    pub fn stream_accept(&mut self) -> Box<Sock> {
        
        todo!();
        /*
            auto sock = Hello();

        const Reply& reply = SendRequestAndGetReply(
            *sock, strprintf("STREAM ACCEPT ID=%s SILENT=false", m_session_id), false);

        const std::string& result = reply.Get("RESULT");

        if (result == "OK") {
            return sock;
        }

        if (result == "INVALID_ID") {
            // If our session id is invalid, then force session re-creation on next usage.
            Disconnect();
        }

        throw std::runtime_error(strprintf("\"%s\"", reply.full));
        */
    }
    
    /**
      | Destroy the session, closing the internally
      | used sockets.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_mutex)]
    pub fn disconnect(&mut self)  {
        
        todo!();
        /*
            if (m_control_sock->Get() != INVALID_SOCKET) {
            if (m_session_id.empty()) {
                Log("Destroying incomplete session");
            } else {
                Log("Destroying session %s", m_session_id);
            }
        }
        m_control_sock->Reset();
        m_session_id.clear();
        */
    }
}


pub struct SAMSessionInner {

    /**
      | SAM control socket.
      | 
      | Used to connect to the I2P SAM service
      | and create a session ("SESSION CREATE").
      | With the established session id we later
      | open other connections to the SAM service
      | to accept incoming I2P connections
      | and make outgoing ones.
      | 
      | See https://geti2p.net/en/docs/api/samv3
      | 
      |
      */
    control_sock:     Box<Sock>,


    /**
      | Our .b32.i2p address.
      | 
      | Derived from `m_private_key`.
      | 
      */
    my_addr:          Service,


    /**
      | SAM session id.
      | 
      */
    session_id:       String,
}

/**
  | A reply from the SAM proxy.
  |
  */
pub struct SAMSessionReply {

    /**
      | Full, unparsed reply.
      |
      */
    full:    String,


    /**
      | Request, used for detailed error reporting.
      |
      */
    request: String,


    /**
      | A map of keywords from the parsed reply.
      | 
      | For example, if the reply is "A=X B C=YZ",
      | then the map will be
      | 
      | Keys["A"] == "X"
      | 
      | Keys["B"] == (empty std::optional)
      | 
      | Keys["C"] == "YZ"
      |
      */
    keys:    HashMap<String,Option<String>>,
}

impl SAMSessionReply {
    
    /**
      | Get the value of a given key.
      | 
      | For example if the reply is "A=X B" then:
      | 
      | Value("A") -> "X"
      | 
      | Value("B") -> throws
      | 
      | Value("C") -> throws
      | 
      | -----------
      | @param[in] key
      | 
      | Key whose value to retrieve
      | 
      | -----------
      | @return
      | 
      | the key's value @throws std::runtime_error
      | if the key is not present or if it has no
      | value
      |
      */
    pub fn get(&self, key: &str) -> String {
        
        todo!();
        /*
            const auto& pos = keys.find(key);
                if (pos == keys.end() || !pos->second.has_value()) {
                    throw std::runtime_error(
                        strprintf("Missing %s= in the reply to \"%s\": \"%s\"", key, request, full));
                }
                return pos->second.value();
        */
    }
}
