// ---------------- [ File: bitcoin-net/src/net.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/net.h]

/**
  | find 'best' local address for a particular
  | peer
  |
  */
pub fn get_local(
        addr:       &mut Service,
        paddr_peer: Option<*const NetAddr>) -> bool {
    
    todo!();
        /*
            if (!fListen)
            return false;

        int nBestScore = -1;
        int nBestReachability = -1;
        {
            LOCK(cs_mapLocalHost);
            for (const auto& entry : mapLocalHost)
            {
                int nScore = entry.second.nScore;
                int nReachability = entry.first.GetReachabilityFrom(paddrPeer);
                if (nReachability > nBestReachability || (nReachability == nBestReachability && nScore > nBestScore))
                {
                    addr = CService(entry.first, entry.second.nPort);
                    nBestReachability = nReachability;
                    nBestScore = nScore;
                }
            }
        }
        return nBestScore >= 0;
        */
}

/**
  | Convert the serialized seeds into usable
  | address objects.
  |
  */
pub fn convert_seeds(seeds_in: &Vec<u8>) -> Vec<Address> {
    
    todo!();
        /*
            // It'll only connect to one or two seed nodes because once it connects,
        // it'll get a pile of addresses with newer timestamps.
        // Seed nodes are given a random 'last seen time' of between one and two
        // weeks ago.
        const int64_t nOneWeek = 7*24*60*60;
        std::vector<CAddress> vSeedsOut;
        FastRandomContext rng;
        DataStream s(vSeedsIn, SER_NETWORK, PROTOCOL_VERSION | ADDRV2_FORMAT);
        while (!s.eof()) {
            CService endpoint;
            s >> endpoint;
            CAddress addr{endpoint, GetDesirableServiceFlags(NODE_NONE)};
            addr.nTime = GetTime() - rng.randrange(nOneWeek) - nOneWeek;
            LogPrint(LogFlags::NET, "Added hardcoded seed: %s\n", addr.ToString());
            vSeedsOut.push_back(addr);
        }
        return vSeedsOut;
        */
}

/**
  | get best local address for a particular peer as
  | a CAddress Otherwise, return the unroutable
  | 0.0.0.0 but filled in with the normal
  | parameters, since the IP may be changed to
  | a useful one by discovery.
  */
pub fn get_local_address(
        paddr_peer:       &NetAddr,
        n_local_services: ServiceFlags) -> Address {
    
    todo!();
        /*
            CAddress ret(CService(CNetAddr(),GetListenPort()), nLocalServices);
        CService addr;
        if (GetLocal(addr, paddrPeer))
        {
            ret = CAddress(addr, nLocalServices);
        }
        ret.nTime = GetAdjustedTime();
        return ret;
        */
}

pub fn getn_score(addr: &Service) -> i32 {
    
    todo!();
        /*
            LOCK(cs_mapLocalHost);
        const auto it = mapLocalHost.find(addr);
        return (it != mapLocalHost.end()) ? it->second.nScore : 0;
        */
}

/**
  | learn a new local address
  |
  */
pub fn add_local(
        addr:    &Service,
        n_score: Option<i32>) -> bool {
    let n_score: i32 = n_score.unwrap_or(LOCAL_NONE.try_into().unwrap());
    
    todo!();
        /*
            if (!addr.IsRoutable())
            return false;

        if (!fDiscover && nScore < LOCAL_MANUAL)
            return false;

        if (!IsReachable(addr))
            return false;

        LogPrintf("AddLocal(%s,%i)\n", addr.ToString(), nScore);

        {
            LOCK(cs_mapLocalHost);
            const auto [it, is_newly_added] = mapLocalHost.emplace(addr, LocalServiceInfo());
            LocalServiceInfo &info = it->second;
            if (is_newly_added || nScore >= info.nScore) {
                info.nScore = nScore + (is_newly_added ? 0 : 1);
                info.nPort = addr.GetPort();
            }
        }

        return true;
        */
}

pub fn add_local_from_net_addr(
        addr:    &NetAddr,
        n_score: Option<i32>) -> bool {
    let n_score: i32 = n_score.unwrap_or(LOCAL_NONE.try_into().unwrap());
    
    todo!();
        /*
            return AddLocal(CService(addr, GetListenPort()), nScore);
        */
}

pub fn remove_local(addr: &Service)  {
    
    todo!();
        /*
            LOCK(cs_mapLocalHost);
        LogPrintf("RemoveLocal(%s)\n", addr.ToString());
        mapLocalHost.erase(addr);
        */
}

/**
  | Mark a network as reachable or unreachable
  | (no automatic connects to it)
  | 
  | -----------
  | @note
  | 
  | Networks are reachable by default
  |
  */
pub fn set_reachable(
        net:       Network,
        reachable: bool)  {
    
    todo!();
        /*
            if (net == NET_UNROUTABLE || net == NET_INTERNAL)
            return;
        LOCK(cs_mapLocalHost);
        vfLimited[net] = !reachable;
        */
}



/**
  | check whether a given address is potentially
  | local
  |
  */
pub fn is_local(addr: &Service) -> bool {
    
    todo!();
        /*
            LOCK(cs_mapLocalHost);
        return mapLocalHost.count(addr) > 0;
        */
}

/**
  | Get the bind address for a socket as CAddress
  |
  */
pub fn get_bind_address(sock: CSocket) -> Address {
    
    todo!();
        /*
            CAddress addr_bind;
        struct sockaddr_storage sockaddr_bind;
        socklen_t sockaddr_bind_len = sizeof(sockaddr_bind);
        if (sock != INVALID_SOCKET) {
            if (!getsockname(sock, (struct sockaddr*)&sockaddr_bind, &sockaddr_bind_len)) {
                addr_bind.SetSockAddr((const struct sockaddr*)&sockaddr_bind);
            } else {
                LogPrint(LogFlags::NET, "Warning: getsockname failed\n");
            }
        }
        return addr_bind;
        */
}

pub fn add_time_data(
        ip:              &NetAddr,
        n_offset_sample: Duration)  {
    
    todo!();
        /*
            LOCK(g_timeoffset_mutex);
        // Ignore duplicates
        static std::set<CNetAddr> setKnown;
        if (setKnown.size() == BITCOIN_TIMEDATA_MAX_SAMPLES)
            return;
        if (!setKnown.insert(ip).second)
            return;

        // Add data
        static CMedianFilter<int64_t> vTimeOffsets(BITCOIN_TIMEDATA_MAX_SAMPLES, 0);
        vTimeOffsets.input(nOffsetSample);
        LogPrint(LogFlags::NET, "added time data, samples %d, offset %+d (%+d minutes)\n", vTimeOffsets.size(), nOffsetSample, nOffsetSample / 60);

        // There is a known issue here (see issue #4521):
        //
        // - The structure vTimeOffsets contains up to 200 elements, after which
        // any new element added to it will not increase its size, replacing the
        // oldest element.
        //
        // - The condition to update nTimeOffset includes checking whether the
        // number of elements in vTimeOffsets is odd, which will never happen after
        // there are 200 elements.
        //
        // But in this case the 'bug' is protective against some attacks, and may
        // actually explain why we've never seen attacks which manipulate the
        // clock offset.
        //
        // So we should hold off on fixing this and clean it up as part of
        // a timing cleanup that strengthens it in a number of other ways.
        //
        if (vTimeOffsets.size() >= 5 && vTimeOffsets.size() % 2 == 1) {
            int64_t nMedian = vTimeOffsets.median();
            std::vector<int64_t> vSorted = vTimeOffsets.sorted();
            // Only let other nodes change our time by so much
            int64_t max_adjustment = std::max<int64_t>(0, gArgs.GetIntArg("-maxtimeadjustment", DEFAULT_MAX_TIME_ADJUSTMENT));
            if (nMedian >= -max_adjustment && nMedian <= max_adjustment) {
                nTimeOffset = nMedian;
            } else {
                nTimeOffset = 0;

                static bool fDone;
                if (!fDone) {
                    // If nobody has a time different than ours but within 5 minutes of ours, give a warning
                    bool fMatch = false;
                    for (const int64_t nOffset : vSorted) {
                        if (nOffset != 0 && nOffset > -5 * 60 && nOffset < 5 * 60) fMatch = true;
                    }

                    if (!fMatch) {
                        fDone = true;
                        bilingual_str strMessage = strprintf(_("Please check that your computer's date and time are correct! If your clock is wrong, %s will not work properly."), PACKAGE_NAME);
                        SetMiscWarning(strMessage);
                        uiInterface.ThreadSafeMessageBox(strMessage, "", CClientUIInterface::MSG_WARNING);
                    }
                }
            }

            if (LogAcceptCategory(LogFlags::NET)) {
                std::string log_message{"time data samples: "};
                for (const int64_t n : vSorted) {
                    log_message += strprintf("%+d  ", n);
                }
                log_message += strprintf("|  median offset = %+d  (%+d minutes)", nTimeOffset, nTimeOffset / 60);
                LogPrint(LogFlags::NET, "%s\n", log_message);
            }
        }
        */
}
