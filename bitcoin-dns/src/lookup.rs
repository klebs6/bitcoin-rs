// ---------------- [ File: bitcoin-dns/src/lookup.rs ]
crate::ix!();

pub type DNSLookupFn = fn(_0: &String, _1: bool) -> Vec<NetAddr>;

lazy_static!{
    pub static ref G_DNS_LOOKUP: DNSLookupFn = wrapped_get_addr_info;
}

pub fn lookup_intern(
    name:                &String,
    vip:                 &mut Vec<NetAddr>,
    n_max_solutions:     u32,
    allow_lookup:        bool,
    dns_lookup_function: DNSLookupFn) -> bool {
    
    todo!();
        /*
            vIP.clear();

        if (!ValidAsCString(name)) {
            return false;
        }

        {
            CNetAddr addr;
            // From our perspective, onion addresses are not hostnames but rather
            // direct encodings of CNetAddr much like IPv4 dotted-decimal notation
            // or IPv6 colon-separated hextet notation. Since we can't use
            // getaddrinfo to decode them and it wouldn't make sense to resolve
            // them, we return a network address representing it instead. See
            // CNetAddr::SetSpecial(const std::string&) for more details.
            if (addr.SetSpecial(name)) {
                vIP.push_back(addr);
                return true;
            }
        }

        for (const CNetAddr& resolved : dns_lookup_function(name, fAllowLookup)) {
            if (nMaxSolutions > 0 && vIP.size() >= nMaxSolutions) {
                break;
            }
            /* Never allow resolving to an internal address. Consider any such result invalid */
            if (!resolved.IsInternal()) {
                vIP.push_back(resolved);
            }
        }

        return (vIP.size() > 0);
        */
}

/**
  | Resolve a host string to its corresponding
  | network addresses.
  | 
  | -----------
  | @param name
  | 
  | The string representing a host. Could
  | be a name or a numerical
  | 
  | IP address (IPv6 addresses in their
  | bracketed form are allowed).
  | ----------
  | @param[out] vIP
  | 
  | The resulting network addresses to
  | which the specified host string resolved.
  | 
  | -----------
  | @return
  | 
  | Whether or not the specified host string
  | successfully resolved to any resulting
  | network addresses. @see Lookup(const
  | std::string&, std::vector<CService>&,
  | uint16_t, bool, unsigned int, DNSLookupFn)
  | for additional parameter descriptions.
  |
  */
pub fn lookup_host_multi(
    name:                &String,
    vip:                 &mut Vec<NetAddr>,
    n_max_solutions:     u32,
    allow_lookup:        bool,
    dns_lookup_function: Option<DNSLookupFn>) -> bool {

    let dns_lookup_function: DNSLookupFn = dns_lookup_function.unwrap_or(*G_DNS_LOOKUP);
    
    todo!();
        /*
            if (!ValidAsCString(name)) {
            return false;
        }
        std::string strHost = name;
        if (strHost.empty())
            return false;
        if (strHost.front() == '[' && strHost.back() == ']') {
            strHost = strHost.substr(1, strHost.size() - 2);
        }

        return LookupIntern(strHost, vIP, nMaxSolutions, fAllowLookup, dns_lookup_function);
        */
}

/**
  | Resolve a host string to its first corresponding
  | network address.
  | 
  | -----------
  | @note
  | 
  | see LookupHost(const std::string&,
  | std::vector<CNetAddr>&, uint16_t,
  | bool, DNSLookupFn) for additional
  | parameter descriptions.
  |
  */
pub fn lookup_host(
    name:                &String,
    addr:                &mut NetAddr,
    allow_lookup:        bool,
    dns_lookup_function: Option<DNSLookupFn>) -> bool {

    let dns_lookup_function: DNSLookupFn = dns_lookup_function.unwrap_or(*G_DNS_LOOKUP);
    
    todo!();
        /*
            if (!ValidAsCString(name)) {
            return false;
        }
        std::vector<CNetAddr> vIP;
        LookupHost(name, vIP, 1, fAllowLookup, dns_lookup_function);
        if(vIP.empty())
            return false;
        addr = vIP.front();
        return true;
        */
}

/**
  | Resolve a service string to its corresponding
  | service.
  | 
  | -----------
  | @param name
  | 
  | The string representing a service.
  | Could be a name or a numerical IP address
  | (IPv6 addresses should be in their disambiguated
  | bracketed form), optionally followed
  | by a uint16_t port number. (e.g. example.com:8333
  | or [2001:db8:85a3:8d3:1319:8a2e:370:7348]:420)
  | ----------
  | @param[out] vAddr
  | 
  | The resulting services to which the
  | specified service string resolved.
  | ----------
  | @param portDefault
  | 
  | The default port for resulting services
  | if not specified by the service string.
  | ----------
  | @param fAllowLookup
  | 
  | Whether or not hostname lookups are
  | permitted. If yes, external queries
  | may be performed.
  | ----------
  | @param nMaxSolutions
  | 
  | The maximum number of results we want,
  | specifying 0 means "as many solutions
  | as we get."
  | 
  | -----------
  | @return
  | 
  | Whether or not the service string successfully
  | resolved to any resulting services.
  |
  */
pub fn lookup_multi(
    name:                &String,
    addr:                &mut Vec<Service>,
    port_default:        u16,
    allow_lookup:        bool,
    n_max_solutions:     u32,
    dns_lookup_function: Option<DNSLookupFn>) -> bool {

    let dns_lookup_function: DNSLookupFn = dns_lookup_function.unwrap_or(*G_DNS_LOOKUP);

    todo!();
        /*
            if (name.empty() || !ValidAsCString(name)) {
            return false;
        }
        uint16_t port{portDefault};
        std::string hostname;
        SplitHostPort(name, port, hostname);

        std::vector<CNetAddr> vIP;
        bool fRet = LookupIntern(hostname, vIP, nMaxSolutions, fAllowLookup, dns_lookup_function);
        if (!fRet)
            return false;
        vAddr.resize(vIP.size());
        for (unsigned int i = 0; i < vIP.size(); i++)
            vAddr[i] = CService(vIP[i], port);
        return true;
        */
}

/**
  | Resolve a service string to its first
  | corresponding service.
  | 
  | -----------
  | @note
  | 
  | see Lookup(const std::string&, std::vector<CService>&,
  | uint16_t, bool, unsigned int, DNSLookupFn)
  | for additional parameter descriptions.
  |
  */
pub fn lookup(
    name:                &String,
    addr:                &mut Service,
    port_default:        u16,
    allow_lookup:        bool,
    dns_lookup_function: Option<DNSLookupFn>) -> bool {

    let dns_lookup_function: DNSLookupFn = dns_lookup_function.unwrap_or(*G_DNS_LOOKUP);
    
    todo!();
        /*
            if (!ValidAsCString(name)) {
            return false;
        }
        std::vector<CService> vService;
        bool fRet = Lookup(name, vService, portDefault, fAllowLookup, 1, dns_lookup_function);
        if (!fRet)
            return false;
        addr = vService[0];
        return true;
        */
}

/**
  | Resolve a service string with a numeric
  | IP to its first corresponding service.
  | 
  | -----------
  | @note
  | 
  | see Lookup(const std::string&, std::vector<CService>&,
  | uint16_t, bool, unsigned int, DNSLookupFn)
  | for additional parameter descriptions.
  | 
  | -----------
  | @return
  | 
  | The resulting CService if the resolution
  | was successful, [::]:0 otherwise.
  |
  */
pub fn lookup_numeric(
    name:                &String,
    port_default:        Option<u16>,
    dns_lookup_function: Option<DNSLookupFn>) -> Service {

    let port_default: u16 = port_default.unwrap_or(0);
    let dns_lookup_function: DNSLookupFn = dns_lookup_function.unwrap_or(*G_DNS_LOOKUP);
    
    todo!();
        /*
            if (!ValidAsCString(name)) {
            return {};
        }
        CService addr;
        // "1.2:345" will fail to resolve the ip, but will still set the port.
        // If the ip fails to resolve, re-init the result.
        if(!Lookup(name, addr, portDefault, false, dns_lookup_function))
            addr = CService();
        return addr;
        */
}
