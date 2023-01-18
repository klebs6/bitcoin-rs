crate::ix!();

/**
  | Parse and resolve a specified subnet
  | string into the appropriate internal
  | representation.
  | 
  | -----------
  | @param strSubnet
  | 
  | A string representation of a subnet
  | of the form `network address [ "/", (
  | CIDR-style suffix | netmask ) ]`(e.g.
  | `2001:db8::/32`, `192.0.2.0/255.255.255.0`,
  | or `8.8.8.8`).
  | 
  | -----------
  | @return
  | 
  | Whether the operation succeeded or
  | not.
  |
  */
pub fn lookup_sub_net(
        str_subnet:          &str,
        ret:                 &mut SubNet,
        dns_lookup_function: Option<DNSLookupFn>) -> bool {

    let dns_lookup_function: DNSLookupFn = dns_lookup_function.unwrap_or(*G_DNS_LOOKUP);
    
    todo!();
        /*
            if (!ValidAsCString(strSubnet)) {
            return false;
        }
        size_t slash = strSubnet.find_last_of('/');
        std::vector<CNetAddr> vIP;

        std::string strAddress = strSubnet.substr(0, slash);
        // TODO: Use LookupHost(const std::string&, CNetAddr&, bool) instead to just get
        //       one CNetAddr.
        if (LookupHost(strAddress, vIP, 1, false, dns_lookup_function))
        {
            CNetAddr network = vIP[0];
            if (slash != strSubnet.npos)
            {
                std::string strNetmask = strSubnet.substr(slash + 1);
                uint8_t n;
                if (ParseUInt8(strNetmask, &n)) {
                    // If valid number, assume CIDR variable-length subnet masking
                    ret = CSubNet(network, n);
                    return ret.IsValid();
                }
                else // If not a valid number, try full netmask syntax
                {
                    // Never allow lookup for netmask
                    if (LookupHost(strNetmask, vIP, 1, false, dns_lookup_function)) {
                        ret = CSubNet(network, vIP[0]);
                        return ret.IsValid();
                    }
                }
            }
            else
            {
                ret = CSubNet(network);
                return ret.IsValid();
            }
        }
        return false;
        */
}
