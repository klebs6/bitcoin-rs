// ---------------- [ File: bitcoin-proxy/src/netbase.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/netbase.h]

lazy_static!{
    /*
    extern int nConnectTimeout;
    extern bool fNameLookup;
    */
}

/**
  | -timeout default
  |
  */
pub const DEFAULT_CONNECT_TIMEOUT: i32 = 5000;

/**
  | -dns default
  |
  */
pub const DEFAULT_NAME_LOOKUP: i32 = 1;

bitflags!{
    pub struct ConnectionDirection: u32 {
        const None = 0;
        const In   = 1 << 0;
        const Out  = 1 << 1;
        const Both = Self::In.bits | Self::Out.bits;
    }
}

///--------------------
pub struct ProxyType {
    pub proxy:                 Service,
    pub randomize_credentials: bool,
}

impl Default for ProxyType {
    
    fn default() -> Self {
        todo!();
        /*
        : randomize_credentials(false),

        
        */
    }
}

impl ProxyType {

    pub fn new(
        proxy:                 &Service,
        randomize_credentials: Option<bool>) -> Self {
        let randomize_credentials: bool =
                 randomize_credentials.unwrap_or(false);
        todo!();
        /*
        : proxy(_proxy),
        : randomize_credentials(_randomize_credentials),

        
        */
    }
    
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return proxy.IsValid();
        */
    }
}

/**
  | Credentials for proxy authentication
  |
  */
pub struct ProxyCredentials
{
    pub username: String,
    pub password: String,
}

/**
  | Socket factory. Defaults to `CreateSockTCP()`,
  | but can be overridden by unit tests.
  |
  */
lazy_static!{
    /*
    extern std::function<std::unique_ptr<Sock>(const CService&)> CreateSock;
    */
}

//-------------------------------------------[.cpp/bitcoin/src/netbase.cpp]

/* ------------------- Settings  ------------------- */


#[derive(Default)]
pub struct ProxyInfo {
    proxy_info: [ProxyType; Network::NET_MAX as usize],
    name_proxy: ProxyType,
}

lazy_static!{
    pub static ref G_PROXYINFO:           Arc<Mutex<ProxyInfo>> = Arc::new(Mutex::new(ProxyInfo::default()));
    pub static ref N_CONNECT_TIMEOUT:     i32 = DEFAULT_CONNECT_TIMEOUT;
    pub static ref NAME_LOOKUP:           Atomic<bool> = Atomic::new(DEFAULT_NAME_LOOKUP != 0);
    pub static ref INTERRUPT_SOCKS5_RECV: Atomic<bool> = Atomic::new(false);

    // Need ample time for negotiation for very
    // slow proxies such as Tor (milliseconds)
    pub static ref G_SOCKS5_RECV_TIMEOUT: i32 = 20 * 1000;
}

/**
  | Wrapper for getaddrinfo(3). Do not
  | use directly: call Lookup/LookupHost/LookupNumeric/LookupSubNet.
  |
  */
pub fn wrapped_get_addr_info(
        name:         &String,
        allow_lookup: bool) -> Vec<NetAddr> {
    
    todo!();
        /*
            addrinfo ai_hint{};
        // We want a TCP port, which is a streaming socket type
        ai_hint.ai_socktype = SOCK_STREAM;
        ai_hint.ai_protocol = IPPROTO_TCP;
        // We don't care which address family (IPv4 or IPv6) is returned
        ai_hint.ai_family = AF_UNSPEC;
        // If we allow lookups of hostnames, use the AI_ADDRCONFIG flag to only
        // return addresses whose family we have an address configured for.
        //
        // If we don't allow lookups, then use the AI_NUMERICHOST flag for
        // getaddrinfo to only decode numerical network addresses and suppress
        // hostname lookups.
        ai_hint.ai_flags = allow_lookup ? AI_ADDRCONFIG : AI_NUMERICHOST;

        addrinfo* ai_res{nullptr};
        const int n_err{getaddrinfo(name.c_str(), nullptr, &ai_hint, &ai_res)};
        if (n_err != 0) {
            return {};
        }

        // Traverse the linked list starting with ai_trav.
        addrinfo* ai_trav{ai_res};
        std::vector<CNetAddr> resolved_addresses;
        while (ai_trav != nullptr) {
            if (ai_trav->ai_family == AF_INET) {
                assert(ai_trav->ai_addrlen >= sizeof(sockaddr_in));
                resolved_addresses.emplace_back(reinterpret_cast<sockaddr_in*>(ai_trav->ai_addr)->sin_addr);
            }
            if (ai_trav->ai_family == AF_INET6) {
                assert(ai_trav->ai_addrlen >= sizeof(sockaddr_in6));
                const sockaddr_in6* s6{reinterpret_cast<sockaddr_in6*>(ai_trav->ai_addr)};
                resolved_addresses.emplace_back(s6->sin6_addr, s6->sin6_scope_id);
            }
            ai_trav = ai_trav->ai_next;
        }
        freeaddrinfo(ai_res);

        return resolved_addresses;
        */
}

pub fn parse_network(net_in: &String) -> Network {
    
    todo!();
        /*
            std::string net = ToLower(net_in);
        if (net == "ipv4") return NET_IPV4;
        if (net == "ipv6") return NET_IPV6;
        if (net == "onion") return NET_ONION;
        if (net == "tor") {
            LogPrintf("Warning: net name 'tor' is deprecated and will be removed in the future. You should use 'onion' instead.\n");
            return NET_ONION;
        }
        if (net == "i2p") {
            return NET_I2P;
        }
        return NET_UNROUTABLE;
        */
}

pub fn get_network_name(net: Network) -> String {
    
    todo!();
        /*
            switch (net) {
        case NET_UNROUTABLE: return "not_publicly_routable";
        case NET_IPV4: return "ipv4";
        case NET_IPV6: return "ipv6";
        case NET_ONION: return "onion";
        case NET_I2P: return "i2p";
        case NET_CJDNS: return "cjdns";
        case NET_INTERNAL: return "internal";
        case NET_MAX: assert(false);
        } // no default case, so the compiler can warn about missing cases

        assert(false);
        */
}

/**
  | Return a vector of publicly routable
  | Network names; optionally append NET_UNROUTABLE.
  |
  */
pub fn get_network_names(append_unroutable: Option<bool>) -> Vec<String> {
    let append_unroutable: bool = append_unroutable.unwrap_or(false);
    
    todo!();
        /*
            std::vector<std::string> names;
        for (int n = 0; n < NET_MAX; ++n) {
            const enum Network network{static_cast<Network>(n)};
            if (network == NET_UNROUTABLE || network == NET_CJDNS || network == NET_INTERNAL) continue;
            names.emplace_back(GetNetworkName(network));
        }
        if (append_unroutable) {
            names.emplace_back(GetNetworkName(NET_UNROUTABLE));
        }
        return names;
        */
}

/**
  | SOCKS version
  |
  */
#[repr(u8)]
pub enum SOCKSVersion {
    SOCKS4 = 0x04,
    SOCKS5 = 0x05
}

/**
  | Values defined for METHOD in RFC1928
  |
  */
#[repr(u8)]
pub enum SOCKS5Method {

    /**
      | No authentication required
      |
      */
    NOAUTH        = 0x00, 

    /**
      | GSSAPI
      |
      */
    GSSAPI        = 0x01, 

    /**
      | Username/password
      |
      */
    USER_PASS     = 0x02, 

    /**
      | No acceptable methods
      |
      */
    NO_ACCEPTABLE = 0xff, 
}

/**
  | Values defined for CMD in RFC1928
  |
  */
#[repr(u8)]
pub enum SOCKS5Command {
    CONNECT       = 0x01,
    BIND          = 0x02,
    UDP_ASSOCIATE = 0x03
}

/**
  | Values defined for REP in RFC1928
  |
  */
#[repr(u8)]
pub enum SOCKS5Reply {

    /**
      | Succeeded
      |
      */
    SUCCEEDED        = 0x00,        

    /**
      | General failure
      |
      */
    GENFAILURE       = 0x01,       

    /**
      | Connection not allowed by ruleset
      |
      */
    NOTALLOWED       = 0x02,       

    /**
      | Network unreachable
      |
      */
    NETUNREACHABLE   = 0x03,   

    /**
      | Network unreachable
      |
      */
    HOSTUNREACHABLE  = 0x04,  

    /**
      | Connection refused
      |
      */
    CONNREFUSED      = 0x05,      

    /**
      | TTL expired
      |
      */
    TTLEXPIRED       = 0x06,       

    /**
      | Command not supported
      |
      */
    CMDUNSUPPORTED   = 0x07,   

    /**
      | Address type not supported
      |
      */
    ATYPEUNSUPPORTED = 0x08, 
}

/**
  | Values defined for ATYPE in RFC1928
  |
  */
#[repr(u8)]
pub enum SOCKS5Atyp {
    IPV4       = 0x01,
    DOMAINNAME = 0x03,
    IPV6       = 0x04,
}

/**
  | Status codes that can be returned by
  | InterruptibleRecv
  |
  */
pub enum IntrRecvError {
    OK,
    Timeout,
    Disconnected,
    NetworkError,
    Interrupted
}

/**
  | Try to read a specified number of bytes
  | from a socket. Please read the "see also"
  | section for more detail.
  | 
  | -----------
  | @param data
  | 
  | The buffer where the read bytes should
  | be stored.
  | ----------
  | @param len
  | 
  | The number of bytes to read into the specified
  | buffer.
  | ----------
  | @param timeout
  | 
  | The total timeout in milliseconds for
  | this read.
  | ----------
  | @param sock
  | 
  | The socket (has to be in non-blocking
  | mode) from which to read bytes.
  | 
  | -----------
  | @return
  | 
  | An IntrRecvError indicating the resulting
  | status of this read.
  | 
  | IntrRecvError::OK only if all of the
  | specified number of bytes were read.
  | @see This function can be interrupted
  | by calling InterruptSocks5(bool).
  | 
  | Sockets can be made non-blocking with
  | SetSocketNonBlocking(const
  | 
  | Socket&, bool).
  |
  */
pub fn interruptible_recv(
        data:    *mut u8,
        len:     usize,
        timeout: i32,
        sock:    &Sock) -> IntrRecvError {
    
    todo!();
        /*
            int64_t curTime = GetTimeMillis();
        int64_t endTime = curTime + timeout;
        while (len > 0 && curTime < endTime) {
            ssize_t ret = sock.Recv(data, len, 0); // Optimistically try the recv first
            if (ret > 0) {
                len -= ret;
                data += ret;
            } else if (ret == 0) { // Unexpected disconnection
                return IntrRecvError::Disconnected;
            } else { // Other error or blocking
                int nErr = WSAGetLastError();
                if (nErr == WSAEINPROGRESS || nErr == WSAEWOULDBLOCK || nErr == WSAEINVAL) {
                    // Only wait at most MAX_WAIT_FOR_IO at a time, unless
                    // we're approaching the end of the specified total timeout
                    const auto remaining = std::chrono::milliseconds{endTime - curTime};
                    const auto timeout = std::min(remaining, std::chrono::milliseconds{MAX_WAIT_FOR_IO});
                    if (!sock.Wait(timeout, Sock::RECV)) {
                        return IntrRecvError::NetworkError;
                    }
                } else {
                    return IntrRecvError::NetworkError;
                }
            }
            if (interruptSocks5Recv)
                return IntrRecvError::Interrupted;
            curTime = GetTimeMillis();
        }
        return len == 0 ? IntrRecvError::OK : IntrRecvError::Timeout;
        */
}

/**
  | Convert SOCKS5 reply to an error message
  |
  */
pub fn socks_5error_string(err: u8) -> String {
    
    todo!();
        /*
            switch(err) {
            case SOCKS5Reply::GENFAILURE:
                return "general failure";
            case SOCKS5Reply::NOTALLOWED:
                return "connection not allowed";
            case SOCKS5Reply::NETUNREACHABLE:
                return "network unreachable";
            case SOCKS5Reply::HOSTUNREACHABLE:
                return "host unreachable";
            case SOCKS5Reply::CONNREFUSED:
                return "connection refused";
            case SOCKS5Reply::TTLEXPIRED:
                return "TTL expired";
            case SOCKS5Reply::CMDUNSUPPORTED:
                return "protocol error";
            case SOCKS5Reply::ATYPEUNSUPPORTED:
                return "address type not supported";
            default:
                return "unknown";
        }
        */
}

/**
  | Connect to a specified destination
  | service through an already connected
  | 
  | SOCKS5 proxy.
  | 
  | -----------
  | @note
  | 
  | The specified SOCKS5 proxy socket must
  | already be connected to the
  | 
  | SOCKS5 proxy. @see <a href="https://www.ietf.org/rfc/rfc1928.txt">RFC1928:
  | SOCKS Protocol
  | 
  | Version 5</a>
  | 
  | -----------
  | @param strDest
  | 
  | The destination fully-qualified domain
  | name.
  | ----------
  | @param port
  | 
  | The destination port.
  | ----------
  | @param auth
  | 
  | The credentials with which to authenticate
  | with the specified
  | 
  | SOCKS5 proxy.
  | ----------
  | @param socket
  | 
  | The SOCKS5 proxy socket.
  | 
  | -----------
  | @return
  | 
  | Whether or not the operation succeeded.
  |
  */
pub fn socks5(
        str_dest: &String,
        port:     u16,
        auth:     *const ProxyCredentials,
        sock:     &Sock) -> bool {
    
    todo!();
        /*
            IntrRecvError recvr;
        LogPrint(LogFlags::NET, "SOCKS5 connecting %s\n", strDest);
        if (strDest.size() > 255) {
            return error("Hostname too long");
        }
        // Construct the version identifier/method selection message
        std::vector<uint8_t> vSocks5Init;
        vSocks5Init.push_back(SOCKSVersion::SOCKS5); // We want the SOCK5 protocol
        if (auth) {
            vSocks5Init.push_back(0x02); // 2 method identifiers follow...
            vSocks5Init.push_back(SOCKS5Method::NOAUTH);
            vSocks5Init.push_back(SOCKS5Method::USER_PASS);
        } else {
            vSocks5Init.push_back(0x01); // 1 method identifier follows...
            vSocks5Init.push_back(SOCKS5Method::NOAUTH);
        }
        ssize_t ret = sock.Send(vSocks5Init.data(), vSocks5Init.size(), MSG_NOSIGNAL);
        if (ret != (ssize_t)vSocks5Init.size()) {
            return error("Error sending to proxy");
        }
        uint8_t pchRet1[2];
        if ((recvr = InterruptibleRecv(pchRet1, 2, g_socks5_recv_timeout, sock)) != IntrRecvError::OK) {
            LogPrintf("Socks5() connect to %s:%d failed: InterruptibleRecv() timeout or other failure\n", strDest, port);
            return false;
        }
        if (pchRet1[0] != SOCKSVersion::SOCKS5) {
            return error("Proxy failed to initialize");
        }
        if (pchRet1[1] == SOCKS5Method::USER_PASS && auth) {
            // Perform username/password authentication (as described in RFC1929)
            std::vector<uint8_t> vAuth;
            vAuth.push_back(0x01); // Current (and only) version of user/pass subnegotiation
            if (auth->username.size() > 255 || auth->password.size() > 255)
                return error("Proxy username or password too long");
            vAuth.push_back(auth->username.size());
            vAuth.insert(vAuth.end(), auth->username.begin(), auth->username.end());
            vAuth.push_back(auth->password.size());
            vAuth.insert(vAuth.end(), auth->password.begin(), auth->password.end());
            ret = sock.Send(vAuth.data(), vAuth.size(), MSG_NOSIGNAL);
            if (ret != (ssize_t)vAuth.size()) {
                return error("Error sending authentication to proxy");
            }
            LogPrint(LogFlags::PROXY, "SOCKS5 sending proxy authentication %s:%s\n", auth->username, auth->password);
            uint8_t pchRetA[2];
            if ((recvr = InterruptibleRecv(pchRetA, 2, g_socks5_recv_timeout, sock)) != IntrRecvError::OK) {
                return error("Error reading proxy authentication response");
            }
            if (pchRetA[0] != 0x01 || pchRetA[1] != 0x00) {
                return error("Proxy authentication unsuccessful");
            }
        } else if (pchRet1[1] == SOCKS5Method::NOAUTH) {
            // Perform no authentication
        } else {
            return error("Proxy requested wrong authentication method %02x", pchRet1[1]);
        }
        std::vector<uint8_t> vSocks5;
        vSocks5.push_back(SOCKSVersion::SOCKS5); // VER protocol version
        vSocks5.push_back(SOCKS5Command::CONNECT); // CMD CONNECT
        vSocks5.push_back(0x00); // RSV Reserved must be 0
        vSocks5.push_back(SOCKS5Atyp::DOMAINNAME); // ATYP DOMAINNAME
        vSocks5.push_back(strDest.size()); // Length<=255 is checked at beginning of function
        vSocks5.insert(vSocks5.end(), strDest.begin(), strDest.end());
        vSocks5.push_back((port >> 8) & 0xFF);
        vSocks5.push_back((port >> 0) & 0xFF);
        ret = sock.Send(vSocks5.data(), vSocks5.size(), MSG_NOSIGNAL);
        if (ret != (ssize_t)vSocks5.size()) {
            return error("Error sending to proxy");
        }
        uint8_t pchRet2[4];
        if ((recvr = InterruptibleRecv(pchRet2, 4, g_socks5_recv_timeout, sock)) != IntrRecvError::OK) {
            if (recvr == IntrRecvError::Timeout) {
                /* If a timeout happens here, this effectively means we timed out while connecting
                 * to the remote node. This is very common for Tor, so do not print an
                 * error message. */
                return false;
            } else {
                return error("Error while reading proxy response");
            }
        }
        if (pchRet2[0] != SOCKSVersion::SOCKS5) {
            return error("Proxy failed to accept request");
        }
        if (pchRet2[1] != SOCKS5Reply::SUCCEEDED) {
            // Failures to connect to a peer that are not proxy errors
            LogPrintf("Socks5() connect to %s:%d failed: %s\n", strDest, port, Socks5ErrorString(pchRet2[1]));
            return false;
        }
        if (pchRet2[2] != 0x00) { // Reserved field must be 0
            return error("Error: malformed proxy response");
        }
        uint8_t pchRet3[256];
        switch (pchRet2[3])
        {
            case SOCKS5Atyp::IPV4: recvr = InterruptibleRecv(pchRet3, 4, g_socks5_recv_timeout, sock); break;
            case SOCKS5Atyp::IPV6: recvr = InterruptibleRecv(pchRet3, 16, g_socks5_recv_timeout, sock); break;
            case SOCKS5Atyp::DOMAINNAME:
            {
                recvr = InterruptibleRecv(pchRet3, 1, g_socks5_recv_timeout, sock);
                if (recvr != IntrRecvError::OK) {
                    return error("Error reading from proxy");
                }
                int nRecv = pchRet3[0];
                recvr = InterruptibleRecv(pchRet3, nRecv, g_socks5_recv_timeout, sock);
                break;
            }
            default: return error("Error: malformed proxy response");
        }
        if (recvr != IntrRecvError::OK) {
            return error("Error reading from proxy");
        }
        if ((recvr = InterruptibleRecv(pchRet3, 2, g_socks5_recv_timeout, sock)) != IntrRecvError::OK) {
            return error("Error reading from proxy");
        }
        LogPrint(LogFlags::NET, "SOCKS5 connected %s\n", strDest);
        return true;
        */
}

/**
  | Create a TCP socket in the given address
  | family.
  | 
  | -----------
  | @param[in] address_family
  | 
  | The socket is created in the same address
  | family as this address.
  | 
  | -----------
  | @return
  | 
  | pointer to the created Sock object or
  | unique_ptr that owns nothing in case
  | of failure
  |
  */
pub fn create_socktcp(address_family: &Service) -> Option<Box<Sock>> {
    
    todo!();
        /*
            // Create a sockaddr from the specified service.
        struct sockaddr_storage sockaddr;
        socklen_t len = sizeof(sockaddr);
        if (!address_family.GetSockAddr((struct sockaddr*)&sockaddr, &len)) {
            LogPrintf("Cannot create socket for %s: unsupported network\n", address_family.ToString());
            return nullptr;
        }

        // Create a TCP socket in the address family of the specified service.
        Socket hSocket = socket(((struct sockaddr*)&sockaddr)->sa_family, SOCK_STREAM, IPPROTO_TCP);
        if (hSocket == INVALID_SOCKET) {
            return nullptr;
        }

        // Ensure that waiting for I/O on this socket won't result in undefined
        // behavior.
        if (!IsSelectableSocket(hSocket)) {
            CloseSocket(hSocket);
            LogPrintf("Cannot create connection: non-selectable socket created (fd >= FD_SETSIZE ?)\n");
            return nullptr;
        }

    #ifdef SO_NOSIGPIPE
        int set = 1;
        // Set the no-sigpipe option on the socket for BSD systems, other UNIXes
        // should use the MSG_NOSIGNAL flag for every send.
        setsockopt(hSocket, SOL_SOCKET, SO_NOSIGPIPE, (c_void*)&set, sizeof(int));
    #endif

        // Set the no-delay option (disable Nagle's algorithm) on the TCP socket.
        SetSocketNoDelay(hSocket);

        // Set the non-blocking option on the socket.
        if (!SetSocketNonBlocking(hSocket, true)) {
            CloseSocket(hSocket);
            LogPrintf("Error setting socket to non-blocking: %s\n", NetworkErrorString(WSAGetLastError()));
            return nullptr;
        }
        return std::make_unique<Sock>(hSocket);
        */
}

lazy_static!{
    /*
    std::function<std::unique_ptr<Sock>(const CService&)> CreateSock = CreateSockTCP;
    */
}

pub fn log_connect_failure<Args>(
        manual_connection: bool,
        fmt:               *const u8,
        args:              &Args)  {

    todo!();
        /*
            std::string error_message = tfm::format(fmt, args...);
        if (manual_connection) {
            LogPrintf("%s\n", error_message);
        } else {
            LogPrint(LogFlags::NET, "%s\n", error_message);
        }
        */
}

/**
  | Try to connect to the specified service
  | on the specified socket.
  | 
  | -----------
  | @param addrConnect
  | 
  | The service to which to connect.
  | ----------
  | @param sock
  | 
  | The socket on which to connect.
  | ----------
  | @param nTimeout
  | 
  | Wait this many milliseconds for the
  | connection to be established.
  | ----------
  | @param manual_connection
  | 
  | Whether or not the connection was manually
  | requested (e.g. through the addnode
  | RPC)
  | 
  | -----------
  | @return
  | 
  | Whether or not a connection was successfully
  | made.
  |
  */
pub fn connect_socket_directly(
        addr_connect:      &Service,
        sock:              &Sock,
        n_timeout:         i32,
        manual_connection: bool) -> bool {
    
    todo!();
        /*
            // Create a sockaddr from the specified service.
        struct sockaddr_storage sockaddr;
        socklen_t len = sizeof(sockaddr);
        if (sock.Get() == INVALID_SOCKET) {
            LogPrintf("Cannot connect to %s: invalid socket\n", addrConnect.ToString());
            return false;
        }
        if (!addrConnect.GetSockAddr((struct sockaddr*)&sockaddr, &len)) {
            LogPrintf("Cannot connect to %s: unsupported network\n", addrConnect.ToString());
            return false;
        }

        // Connect to the addrConnect service on the hSocket socket.
        if (sock.Connect(reinterpret_cast<struct sockaddr*>(&sockaddr), len) == SOCKET_ERROR) {
            int nErr = WSAGetLastError();
            // WSAEINVAL is here because some legacy version of winsock uses it
            if (nErr == WSAEINPROGRESS || nErr == WSAEWOULDBLOCK || nErr == WSAEINVAL)
            {
                // Connection didn't actually fail, but is being established
                // asynchronously. Thus, use async I/O api (select/poll)
                // synchronously to check for successful connection with a timeout.
                const Sock::Event requested = Sock::RECV | Sock::SEND;
                Sock::Event occurred;
                if (!sock.Wait(std::chrono::milliseconds{nTimeout}, requested, &occurred)) {
                    LogPrintf("wait for connect to %s failed: %s\n",
                              addrConnect.ToString(),
                              NetworkErrorString(WSAGetLastError()));
                    return false;
                } else if (occurred == 0) {
                    LogPrint(LogFlags::NET, "connection attempt to %s timed out\n", addrConnect.ToString());
                    return false;
                }

                // Even if the wait was successful, the connect might not
                // have been successful. The reason for this failure is hidden away
                // in the SO_ERROR for the socket in modern systems. We read it into
                // sockerr here.
                int sockerr;
                socklen_t sockerr_len = sizeof(sockerr);
                if (sock.GetSockOpt(SOL_SOCKET, SO_ERROR, (sockopt_arg_type)&sockerr, &sockerr_len) ==
                    SOCKET_ERROR) {
                    LogPrintf("getsockopt() for %s failed: %s\n", addrConnect.ToString(), NetworkErrorString(WSAGetLastError()));
                    return false;
                }
                if (sockerr != 0) {
                    LogConnectFailure(manual_connection,
                                      "connect() to %s failed after wait: %s",
                                      addrConnect.ToString(),
                                      NetworkErrorString(sockerr));
                    return false;
                }
            }
    #ifdef WIN32
            else if (WSAGetLastError() != WSAEISCONN)
    #else
            else
    #endif
            {
                LogConnectFailure(manual_connection, "connect() to %s failed: %s", addrConnect.ToString(), NetworkErrorString(WSAGetLastError()));
                return false;
            }
        }
        return true;
        */
}

pub fn set_proxy(
        net:        Network,
        addr_proxy: &ProxyType) -> bool {
    
    todo!();
        /*
            assert(net >= 0 && net < NET_MAX);
        if (!addrProxy.IsValid())
            return false;
        LOCK(g_proxyinfo_mutex);
        proxyInfo[net] = addrProxy;
        return true;
        */
}

pub fn get_proxy(
        net:            Network,
        proxy_info_out: &mut ProxyType) -> bool {
    
    todo!();
        /*
            assert(net >= 0 && net < NET_MAX);
        LOCK(g_proxyinfo_mutex);
        if (!proxyInfo[net].IsValid())
            return false;
        proxyInfoOut = proxyInfo[net];
        return true;
        */
}

/**
  | Set the name proxy to use for all connections
  | to nodes specified by a hostname. After
  | setting this proxy, connecting to a
  | node specified by a hostname won't result
  | in a local lookup of said hostname, rather,
  | connect to the node by asking the name
  | proxy for a proxy connection to the hostname,
  | effectively delegating the hostname
  | lookup to the specified proxy.
  | 
  | This delegation increases privacy
  | for those who set the name proxy as they
  | no longer leak their external hostname
  | queries to their DNS servers.
  | 
  | -----------
  | @note
  | 
  | SOCKS5's support for UDP-over-SOCKS5
  | has been considered, but no SOCK5 server
  | in common use (most notably Tor) actually
  | implements UDP support, and a DNS resolver
  | is beyond the scope of this project.
  | 
  | -----------
  | @return
  | 
  | Whether or not the operation succeeded.
  |
  */
pub fn set_name_proxy(addr_proxy: &ProxyType) -> bool {
    
    todo!();
        /*
            if (!addrProxy.IsValid())
            return false;
        LOCK(g_proxyinfo_mutex);
        nameProxy = addrProxy;
        return true;
        */
}

pub fn get_name_proxy(name_proxy_out: &mut ProxyType) -> bool {
    
    todo!();
        /*
            LOCK(g_proxyinfo_mutex);
        if(!nameProxy.IsValid())
            return false;
        nameProxyOut = nameProxy;
        return true;
        */
}

pub fn have_name_proxy() -> bool {
    
    todo!();
        /*
            LOCK(g_proxyinfo_mutex);
        return nameProxy.IsValid();
        */
}

pub fn is_proxy(addr: &NetAddr) -> bool {
    
    todo!();
        /*
            LOCK(g_proxyinfo_mutex);
        for (int i = 0; i < NET_MAX; i++) {
            if (addr == static_cast<CNetAddr>(proxyInfo[i].proxy))
                return true;
        }
        return false;
        */
}

/**
  | Connect to a specified destination
  | service through a SOCKS5 proxy by first
  | connecting to the SOCKS5 proxy.
  | 
  | -----------
  | @param proxy
  | 
  | The SOCKS5 proxy.
  | ----------
  | @param strDest
  | 
  | The destination service to which to
  | connect.
  | ----------
  | @param port
  | 
  | The destination port.
  | ----------
  | @param sock
  | 
  | The socket on which to connect to the
  | SOCKS5 proxy.
  | ----------
  | @param nTimeout
  | 
  | Wait this many milliseconds for the
  | connection to the SOCKS5 proxy to be
  | established.
  | ----------
  | @param[out] outProxyConnectionFailed
  | 
  | Whether or not the connection to the
  | 
  | SOCKS5 proxy failed.
  | 
  | -----------
  | @return
  | 
  | Whether or not the operation succeeded.
  |
  */
pub fn connect_through_proxy(
        proxy:                       &ProxyType,
        str_dest:                    &String,
        port:                        u16,
        sock:                        &Sock,
        n_timeout:                   i32,
        out_proxy_connection_failed: &mut bool) -> bool {
    
    todo!();
        /*
            // first connect to proxy server
        if (!ConnectSocketDirectly(proxy.proxy, sock, nTimeout, true)) {
            outProxyConnectionFailed = true;
            return false;
        }
        // do socks negotiation
        if (proxy.randomize_credentials) {
            ProxyCredentials random_auth;
            static std::atomic_int counter(0);
            random_auth.username = random_auth.password = strprintf("%i", counter++);
            if (!Socks5(strDest, port, &random_auth, sock)) {
                return false;
            }
        } else {
            if (!Socks5(strDest, port, 0, sock)) {
                return false;
            }
        }
        return true;
        */
}


/**
  | Disable or enable blocking-mode for
  | a socket
  |
  */
pub fn set_socket_non_blocking(
        h_socket:     &CSocket,
        non_blocking: bool) -> bool {
    
    todo!();
        /*
            if (fNonBlocking) {
    #ifdef WIN32
            u_long nOne = 1;
            if (ioctlsocket(hSocket, FIONBIO, &nOne) == SOCKET_ERROR) {
    #else
            int fFlags = fcntl(hSocket, F_GETFL, 0);
            if (fcntl(hSocket, F_SETFL, fFlags | O_NONBLOCK) == SOCKET_ERROR) {
    #endif
                return false;
            }
        } else {
    #ifdef WIN32
            u_long nZero = 0;
            if (ioctlsocket(hSocket, FIONBIO, &nZero) == SOCKET_ERROR) {
    #else
            int fFlags = fcntl(hSocket, F_GETFL, 0);
            if (fcntl(hSocket, F_SETFL, fFlags & ~O_NONBLOCK) == SOCKET_ERROR) {
    #endif
                return false;
            }
        }

        return true;
        */
}

/**
  | Set the TCP_NODELAY flag on a socket
  |
  */
pub fn set_socket_no_delay(h_socket: &CSocket) -> bool {
    
    todo!();
        /*
            int set = 1;
        int rc = setsockopt(hSocket, IPPROTO_TCP, TCP_NODELAY, (const char*)&set, sizeof(int));
        return rc == 0;
        */
}
