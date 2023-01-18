crate::ix!();

pub fn discover() -> bool  {
    
    todo!();
        /*
            if (!fDiscover)
            return;

    #ifdef WIN32
        // Get local host IP
        char pszHostName[256] = "";
        if (gethostname(pszHostName, sizeof(pszHostName)) != SOCKET_ERROR)
        {
            std::vector<CNetAddr> vaddr;
            if (LookupHost(pszHostName, vaddr, 0, true))
            {
                for (const CNetAddr &addr : vaddr)
                {
                    if (AddLocal(addr, LOCAL_IF))
                        LogPrintf("%s: %s - %s\n", __func__, pszHostName, addr.ToString());
                }
            }
        }
    #elif (HAVE_DECL_GETIFADDRS && HAVE_DECL_FREEIFADDRS)
        // Get local host ip
        struct ifaddrs* myaddrs;
        if (getifaddrs(&myaddrs) == 0)
        {
            for (struct ifaddrs* ifa = myaddrs; ifa != nullptr; ifa = ifa->ifa_next)
            {
                if (ifa->ifa_addr == nullptr) continue;
                if ((ifa->ifa_flags & IFF_UP) == 0) continue;
                if (strcmp(ifa->ifa_name, "lo") == 0) continue;
                if (strcmp(ifa->ifa_name, "lo0") == 0) continue;
                if (ifa->ifa_addr->sa_family == AF_INET)
                {
                    struct sockaddr_in* s4 = (struct sockaddr_in*)(ifa->ifa_addr);
                    CNetAddr addr(s4->sin_addr);
                    if (AddLocal(addr, LOCAL_IF))
                        LogPrintf("%s: IPv4 %s: %s\n", __func__, ifa->ifa_name, addr.ToString());
                }
                else if (ifa->ifa_addr->sa_family == AF_INET6)
                {
                    struct sockaddr_in6* s6 = (struct sockaddr_in6*)(ifa->ifa_addr);
                    CNetAddr addr(s6->sin6_addr);
                    if (AddLocal(addr, LOCAL_IF))
                        LogPrintf("%s: IPv6 %s: %s\n", __func__, ifa->ifa_name, addr.ToString());
                }
            }
            freeifaddrs(myaddrs);
        }
    #endif
        */
}

