// ---------------- [ File: bitcoin-netpermissions/src/permissions.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/net_permissions.h]

lazy_static!{
    /*
    extern const std::vector<std::string> NET_PERMISSIONS_DOC;
    */
}

bitflags!{
    pub struct NetPermissionFlags: u32 {

        const None = 0;

        /*
         | Can query bloomfilter even if
         | 
         | -peerbloomfilters is false
         |
         */
        const BloomFilter = 1 << 1;

        /*
         | Relay and accept transactions from
         | this peer, even if -blocksonly is true
         | This peer is also not subject to limits
         | on how many transaction INVs are tracked
         |
         */
        const Relay = 1 << 3;

        /*
         | Always relay transactions from this
         | peer, even if already in mempool Keep
         | parameter interaction: forcerelay
         | implies relay
         |
         */
        const ForceRelay = (1 << 2) | Self::Relay.bits;

        /*
         | Allow getheaders during IBD and block-download
         | after maxuploadtarget limit
         |
         */
        const Download = 1 << 6;

        /*
         | Can't be banned/disconnected/discouraged
         | for misbehavior
         |
         */
        const NoBan = (1 << 4) | Self::Download.bits;

        /*
         | Can query the mempool
         |
         */
        const Mempool = 1 << 5;

        /*
         | Can request addrs without hitting a
         | privacy-preserving cache, and send
         | us unlimited amounts of addrs.
         |
         */
        const Addr = 1 << 7;

        /*
         | True if the user did not specifically
         | set fine grained permissions
         |
         */
        const Implicit = 1 << 31;

        const All = 
            Self::BloomFilter.bits 
            | Self::ForceRelay.bits 
            | Self::Relay.bits 
            | Self::NoBan.bits 
            | Self::Mempool.bits 
            | Self::Download.bits 
            | Self::Addr.bits;
    }
}

impl Default for NetPermissionFlags {
    fn default() -> Self {
        NetPermissionFlags::None
    }
}

///------------------------
#[derive(Clone)]
pub struct NetPermissions {
    pub flags: NetPermissionFlags,
}

impl NetPermissions {

    #[inline] pub fn has_flag(
        flags: &NetPermissionFlags,
        f:     NetPermissionFlags) -> bool {
        
        todo!();
        /*
            using t = typename std::underlying_type<NetPermissionFlags>::type;
            return (static_cast<t>(flags) & static_cast<t>(f)) == static_cast<t>(f);
        */
    }
    
    #[inline] pub fn add_flag(
        flags: &mut NetPermissionFlags,
        f:     NetPermissionFlags)  {
        
        todo!();
        /*
            flags = flags | f;
        */
    }

    /**
      | ClearFlag is only called with `f` ==
      | NetPermissionFlags::Implicit.
      |
      | If that should change in the future, be
      | aware that ClearFlag should not be called
      | with a subflag of a multiflag,
      | e.g. NetPermissionFlags::Relay or
      | NetPermissionFlags::Download, as that
      | would leave `flags` in an invalid state
      | corresponding to none of the existing
      | flags.
      */
    #[inline] pub fn clear_flag(
        flags: &mut NetPermissionFlags,
        f:     NetPermissionFlags)  {
        
        todo!();
        /*
            assert(f == NetPermissionFlags::Implicit);
            using t = typename std::underlying_type<NetPermissionFlags>::type;
            flags = static_cast<NetPermissionFlags>(static_cast<t>(flags) & ~static_cast<t>(f));
        */
    }
    
    pub fn to_strings(&mut self, flags: NetPermissionFlags) -> Vec<String> {
        
        todo!();
        /*
            std::vector<std::string> strings;
        if (NetPermissions::HasFlag(flags, NetPermissionFlags::BloomFilter)) strings.push_back("bloomfilter");
        if (NetPermissions::HasFlag(flags, NetPermissionFlags::NoBan)) strings.push_back("noban");
        if (NetPermissions::HasFlag(flags, NetPermissionFlags::ForceRelay)) strings.push_back("forcerelay");
        if (NetPermissions::HasFlag(flags, NetPermissionFlags::Relay)) strings.push_back("relay");
        if (NetPermissions::HasFlag(flags, NetPermissionFlags::Mempool)) strings.push_back("mempool");
        if (NetPermissions::HasFlag(flags, NetPermissionFlags::Download)) strings.push_back("download");
        if (NetPermissions::HasFlag(flags, NetPermissionFlags::Addr)) strings.push_back("addr");
        return strings;
        */
    }
}

///------------------------
#[derive(Clone)]
pub struct NetWhitebindPermissions {
    pub base:    NetPermissions,
    pub service: Service,
}

impl NetWhitebindPermissions {

    pub fn try_parse(&mut self, 
        str_:   &String,
        output: &mut NetWhitebindPermissions,
        error:  &mut BilingualStr) -> bool {
        
        todo!();
        /*
            NetPermissionFlags flags;
        size_t offset;
        if (!TryParsePermissionFlags(str, flags, offset, error)) return false;

        const std::string strBind = str.substr(offset);
        CService addrBind;
        if (!Lookup(strBind, addrBind, 0, false)) {
            error = ResolveErrMsg("whitebind", strBind);
            return false;
        }
        if (addrBind.GetPort() == 0) {
            error = strprintf(_("Need to specify a port with -whitebind: '%s'"), strBind);
            return false;
        }

        output.m_flags = flags;
        output.m_service = addrBind;
        error = Untranslated("");
        return true;
        */
    }
}

///------------------------
#[derive(Clone)]
pub struct NetWhitelistPermissions {
    pub base:   NetPermissions,
    pub subnet: SubNet,
}

impl NetWhitelistPermissions {

    pub fn try_parse(&mut self, 
        str_:   &String,
        output: &mut NetWhitelistPermissions,
        error:  &mut BilingualStr) -> bool {
        
        todo!();
        /*
            NetPermissionFlags flags;
        size_t offset;
        if (!TryParsePermissionFlags(str, flags, offset, error)) return false;

        const std::string net = str.substr(offset);
        CSubNet subnet;
        LookupSubNet(net, subnet);
        if (!subnet.IsValid()) {
            error = strprintf(_("Invalid netmask specified in -whitelist: '%s'"), net);
            return false;
        }

        output.m_flags = flags;
        output.m_subnet = subnet;
        error = Untranslated("");
        return true;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/net_permissions.cpp]

lazy_static!{
    static ref NET_PERMISSIONS_DOC: Vec<&'static str> = vec!{
        "bloomfilter (allow requesting BIP37 filtered blocks and transactions)",
        "noban (do not ban for misbehavior; implies download)",
        "forcerelay (relay transactions that are already in the mempool; implies relay)",
        "relay (relay even in -blocksonly mode, and unlimited transaction announcements)",
        "mempool (allow requesting BIP35 mempool contents)",
        "download (allow getheaders during IBD, no disconnect after maxuploadtarget limit)",
        "addr (responses to GETADDR avoid hitting the cache and contain random records with the most up-to-date info)"
    };
}

/**
   Parse the following format:
   "perm1,perm2@xxxxxx"
  */
pub fn try_parse_permission_flags(
        str_:   &String,
        output: &mut NetPermissionFlags,
        readen: &mut usize,
        error:  &mut BilingualStr) -> bool {
    
    todo!();
        /*
            NetPermissionFlags flags = NetPermissionFlags::None;
        const auto atSeparator = str.find('@');

        // if '@' is not found (ie, "xxxxx"), the caller should apply implicit permissions
        if (atSeparator == std::string::npos) {
            NetPermissions::AddFlag(flags, NetPermissionFlags::Implicit);
            readen = 0;
        }
        // else (ie, "perm1,perm2@xxxxx"), let's enumerate the permissions by splitting by ',' and calculate the flags
        else {
            readen = 0;
            // permissions == perm1,perm2
            const auto permissions = str.substr(0, atSeparator);
            while (readen < permissions.length()) {
                const auto commaSeparator = permissions.find(',', readen);
                const auto len = commaSeparator == std::string::npos ? permissions.length() - readen : commaSeparator - readen;
                // permission == perm1
                const auto permission = permissions.substr(readen, len);
                readen += len; // We read "perm1"
                if (commaSeparator != std::string::npos) readen++; // We read ","

                if (permission == "bloomfilter" || permission == "bloom") NetPermissions::AddFlag(flags, NetPermissionFlags::BloomFilter);
                else if (permission == "noban") NetPermissions::AddFlag(flags, NetPermissionFlags::NoBan);
                else if (permission == "forcerelay") NetPermissions::AddFlag(flags, NetPermissionFlags::ForceRelay);
                else if (permission == "mempool") NetPermissions::AddFlag(flags, NetPermissionFlags::Mempool);
                else if (permission == "download") NetPermissions::AddFlag(flags, NetPermissionFlags::Download);
                else if (permission == "all") NetPermissions::AddFlag(flags, NetPermissionFlags::All);
                else if (permission == "relay") NetPermissions::AddFlag(flags, NetPermissionFlags::Relay);
                else if (permission == "addr") NetPermissions::AddFlag(flags, NetPermissionFlags::Addr);
                else if (permission.length() == 0); // Allow empty entries
                else {
                    error = strprintf(_("Invalid P2P permission: '%s'"), permission);
                    return false;
                }
            }
            readen++;
        }

        output = flags;
        error = Untranslated("");
        return true;
        */
}
