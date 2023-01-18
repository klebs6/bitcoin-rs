crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/mapport.h]

#[cfg(USE_UPNP)]        pub const DEFAULT_UPNP:   bool = USE_UPNP;
#[cfg(not(USE_UPNP))]   pub const DEFAULT_UPNP:   bool = false;

#[cfg(USE_NATPMP)]      pub const DEFAULT_NATPMP: bool = USE_NATPMP;
#[cfg(not(USE_NATPMP))] pub const DEFAULT_NATPMP: bool = false;

#[repr(u32)]
pub enum MapPortProtoFlag {
    NONE    = 0x00,
    UPNP    = 0x01,
    NAT_PMP = 0x02,
}

//-------------------------------------------[.cpp/bitcoin/src/mapport.cpp]

/**
  | The minimum supported miniUPnPc API version is
  | set to 10. This keeps compatibility with Ubuntu
  | 16.04 LTS and Debian 8 libminiupnpc-dev
  | packages.
  */
#[cfg(USE_UPNP)]
const_assert!{
    MINIUPNPC_API_VERSION >= 10
} //"miniUPnPc API version >= 10 assumed"

#[cfg(any(USE_NATPMP,USE_UPNP))]
lazy_static!{
    /*
    static CThreadInterrupt g_mapport_interrupt;
    static std::thread g_mapport_thread;
    static std::atomic_uint g_mapport_enabled_protos{MapPortProtoFlag::NONE};
    static std::atomic<MapPortProtoFlag> g_mapport_current_proto{MapPortProtoFlag::NONE};
    */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub const PORT_MAPPING_REANNOUNCE_PERIOD: Minutes = 20;

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub const PORT_MAPPING_RETRY_PERIOD:      Minutes = 5;

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_NATPMP)]
lazy_static!{
    /*
    static uint16_t g_mapport_external_port = 0;
    */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_NATPMP)]
pub fn natpmp_init(natpmp: *mut NatPmp) -> bool {
    
    todo!();
        /*
            const int r_init = initnatpmp(natpmp, /* detect gateway automatically */ 0, /* forced gateway - NOT APPLIED*/ 0);
        if (r_init == 0) return true;
        LogPrintf("natpmp: initnatpmp() failed with %d error.\n", r_init);
        return false;
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_NATPMP)]
pub fn natpmp_discover(
        natpmp:             *mut NatPmp,
        external_ipv4_addr: &mut InAddr) -> bool {
    
    todo!();
        /*
        const int r_send = sendpublicaddressrequest(natpmp);

        if (r_send == 2 ) { /* OK */
            int r_read;
            natpmpresp_t response;
            do {
                r_read = readnatpmpresponseorretry(natpmp, &response);
            } while (r_read == NATPMP_TRYAGAIN);

            if (r_read == 0) {
                external_ipv4_addr = response.pnu.publicaddress.addr;
                return true;
            } else if (r_read == NATPMP_ERR_NOGATEWAYSUPPORT) {
                LogPrintf("natpmp: The gateway does not support NAT-PMP.\n");
            } else {
                LogPrintf("natpmp: readnatpmpresponseorretry() for public address failed with %d error.\n", r_read);
            }
        } else {
            LogPrintf("natpmp: sendpublicaddressrequest() failed with %d error.\n", r_send);
        }

        return false;
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_NATPMP)]
pub fn natpmp_mapping(
        natpmp:                 *mut NatPmp,
        external_ipv4_addr:     &InAddr,
        private_port:           u16,
        external_ip_discovered: &mut bool) -> bool {
    
    todo!();
        /*
        const uint16_t suggested_external_port = g_mapport_external_port ? g_mapport_external_port : private_port;
        const int r_send = sendnewportmappingrequest(natpmp, NATPMP_PROTOCOL_TCP, private_port, suggested_external_port, /*seconds*/ 3600 );

        if (r_send == 12 ) {/* OK */
            int r_read;
            natpmpresp_t response;
            do {
                r_read = readnatpmpresponseorretry(natpmp, &response);
            } while (r_read == NATPMP_TRYAGAIN);

            if (r_read == 0) {
                auto pm = response.pnu.newportmapping;
                if (private_port == pm.privateport && pm.lifetime > 0) {
                    g_mapport_external_port = pm.mappedpublicport;
                    const CService external{external_ipv4_addr, pm.mappedpublicport};
                    if (!external_ip_discovered && fDiscover) {
                        AddLocal(external, LOCAL_MAPPED);
                        external_ip_discovered = true;
                    }
                    LogPrintf("natpmp: Port mapping successful. External address = %s\n", external.ToString());
                    return true;
                } else {
                    LogPrintf("natpmp: Port mapping failed.\n");
                }
            } else if (r_read == NATPMP_ERR_NOGATEWAYSUPPORT) {
                LogPrintf("natpmp: The gateway does not support NAT-PMP.\n");
            } else {
                LogPrintf("natpmp: readnatpmpresponseorretry() for port mapping failed with %d error.\n", r_read);
            }
        } else {
            LogPrintf("natpmp: sendnewportmappingrequest() failed with %d error.\n", r_send);
        }

        return false;
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_NATPMP)]
pub fn process_natpmp() -> bool {
    
    todo!();
        /*
        bool ret = false;
        natpmp_t natpmp;
        struct in_addr external_ipv4_addr;
        if (NatpmpInit(&natpmp) && NatpmpDiscover(&natpmp, external_ipv4_addr)) {
            bool external_ip_discovered = false;
            const uint16_t private_port = GetListenPort();
            do {
                ret = NatpmpMapping(&natpmp, external_ipv4_addr, private_port, external_ip_discovered);
            } while (ret && g_mapport_interrupt.sleep_for(PORT_MAPPING_REANNOUNCE_PERIOD));
            g_mapport_interrupt.reset();

            const int r_send = sendnewportmappingrequest(&natpmp, NATPMP_PROTOCOL_TCP, private_port, g_mapport_external_port, /* remove a port mapping */ 0);
            g_mapport_external_port = 0;
            if (r_send == 12 ) { /* OK */
                LogPrintf("natpmp: Port mapping removed successfully.\n");
            } else {
                LogPrintf("natpmp: sendnewportmappingrequest(0) failed with %d error.\n", r_send);
            }
        }

        closenatpmp(&natpmp);
        return ret;
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_UPNP)]
pub fn process_upnp() -> bool {
    
    todo!();
        /*
        bool ret = false;
        std::string port = strprintf("%u", GetListenPort());
        const char * multicastif = nullptr;
        const char * minissdpdpath = nullptr;
        struct UPNPDev * devlist = nullptr;
        char lanaddr[64];

        int error = 0;
    #if MINIUPNPC_API_VERSION < 14
        devlist = upnpDiscover(2000, multicastif, minissdpdpath, 0, 0, &error);
    #else
        devlist = upnpDiscover(2000, multicastif, minissdpdpath, 0, 0, 2, &error);
    #endif

        struct UPNPUrls urls;
        struct IGDdatas data;
        int r;

        r = UPNP_GetValidIGD(devlist, &urls, &data, lanaddr, sizeof(lanaddr));
        if (r == 1)
        {
            if (fDiscover) {
                char externalIPAddress[40];
                r = UPNP_GetExternalIPAddress(urls.controlURL, data.first.servicetype, externalIPAddress);
                if (r != UPNPCOMMAND_SUCCESS) {
                    LogPrintf("UPnP: GetExternalIPAddress() returned %d\n", r);
                } else {
                    if (externalIPAddress[0]) {
                        CNetAddr resolved;
                        if (LookupHost(externalIPAddress, resolved, false)) {
                            LogPrintf("UPnP: ExternalIPAddress = %s\n", resolved.ToString());
                            AddLocal(resolved, LOCAL_MAPPED);
                        }
                    } else {
                        LogPrintf("UPnP: GetExternalIPAddress failed.\n");
                    }
                }
            }

            std::string strDesc = PACKAGE_NAME " " + FormatFullVersion();

            do {
                r = UPNP_AddPortMapping(urls.controlURL, data.first.servicetype, port.c_str(), port.c_str(), lanaddr, strDesc.c_str(), "TCP", 0, "0");

                if (r != UPNPCOMMAND_SUCCESS) {
                    ret = false;
                    LogPrintf("AddPortMapping(%s, %s, %s) failed with code %d (%s)\n", port, port, lanaddr, r, strupnperror(r));
                    break;
                } else {
                    ret = true;
                    LogPrintf("UPnP Port Mapping successful.\n");
                }
            } while (g_mapport_interrupt.sleep_for(PORT_MAPPING_REANNOUNCE_PERIOD));
            g_mapport_interrupt.reset();

            r = UPNP_DeletePortMapping(urls.controlURL, data.first.servicetype, port.c_str(), "TCP", 0);
            LogPrintf("UPNP_DeletePortMapping() returned: %d\n", r);
            freeUPNPDevlist(devlist); devlist = nullptr;
            FreeUPNPUrls(&urls);
        } else {
            LogPrintf("No valid UPnP IGDs found\n");
            freeUPNPDevlist(devlist); devlist = nullptr;
            if (r != 0)
                FreeUPNPUrls(&urls);
        }

        return ret;
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub fn thread_map_port()  {
    
    todo!();
        /*
        SetSyscallSandboxPolicy(SyscallSandboxPolicy::INITIALIZATION_MAP_PORT);
        bool ok;
        do {
            ok = false;

    #ifdef USE_UPNP
            // High priority protocol.
            if (g_mapport_enabled_protos & MapPortProtoFlag::UPNP) {
                g_mapport_current_proto = MapPortProtoFlag::UPNP;
                ok = ProcessUpnp();
                if (ok) continue;
            }
    #endif // USE_UPNP

    #ifdef USE_NATPMP
            // Low priority protocol.
            if (g_mapport_enabled_protos & MapPortProtoFlag::NAT_PMP) {
                g_mapport_current_proto = MapPortProtoFlag::NAT_PMP;
                ok = ProcessNatpmp();
                if (ok) continue;
            }
    #endif // USE_NATPMP

            g_mapport_current_proto = MapPortProtoFlag::NONE;
            if (g_mapport_enabled_protos == MapPortProtoFlag::NONE) {
                return;
            }

        } while (ok || g_mapport_interrupt.sleep_for(PORT_MAPPING_RETRY_PERIOD));
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub fn start_thread_map_port()  {
    
    todo!();
        /*
            if (!g_mapport_thread.joinable()) {
            assert(!g_mapport_interrupt);
            g_mapport_thread = std::thread(&util::TraceThread, "mapport", &ThreadMapPort);
        }
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub fn dispatch_map_port()  {
    
    todo!();
        /*
            if (g_mapport_current_proto == MapPortProtoFlag::NONE && g_mapport_enabled_protos == MapPortProtoFlag::NONE) {
            return;
        }

        if (g_mapport_current_proto == MapPortProtoFlag::NONE && g_mapport_enabled_protos != MapPortProtoFlag::NONE) {
            StartThreadMapPort();
            return;
        }

        if (g_mapport_current_proto != MapPortProtoFlag::NONE && g_mapport_enabled_protos == MapPortProtoFlag::NONE) {
            InterruptMapPort();
            StopMapPort();
            return;
        }

        if (g_mapport_enabled_protos & g_mapport_current_proto) {
            // Enabling another protocol does not cause switching from the currently used one.
            return;
        }

        assert(g_mapport_thread.joinable());
        assert(!g_mapport_interrupt);
        // Interrupt a protocol-specific loop in the ThreadUpnp() or in the ThreadNatpmp()
        // to force trying the next protocol in the ThreadMapPort() loop.
        g_mapport_interrupt();
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub fn map_port_proto_set_enabled(
        proto:   MapPortProtoFlag,
        enabled: bool)  {
    
    todo!();
        /*
            if (enabled) {
            g_mapport_enabled_protos |= proto;
        } else {
            g_mapport_enabled_protos &= ~proto;
        }
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub fn start_map_port(
        use_upnp:   bool,
        use_natpmp: bool)  {
    
    todo!();
        /*
            MapPortProtoSetEnabled(MapPortProtoFlag::UPNP, use_upnp);
        MapPortProtoSetEnabled(MapPortProtoFlag::NAT_PMP, use_natpmp);
        DispatchMapPort();
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub fn interrupt_map_port()  {
    
    todo!();
        /*
            g_mapport_enabled_protos = MapPortProtoFlag::NONE;
        if (g_mapport_thread.joinable()) {
            g_mapport_interrupt();
        }
        */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub fn stop_map_port()  {
    
    todo!();
        /*
            if (g_mapport_thread.joinable()) {
            g_mapport_thread.join();
            g_mapport_interrupt.reset();
        }
        */
}

#[cfg(not(any(USE_NATPMP,USE_UPNP)))]
pub fn start_map_port(
        use_upnp:   bool,
        use_natpmp: bool)  {

    // Intentionally left blank.
}

#[cfg(not(any(USE_NATPMP,USE_UPNP)))]
pub fn interrupt_map_port()  {
    // Intentionally left blank.
}

#[cfg(not(any(USE_NATPMP,USE_UPNP)))]
pub fn stop_map_port()  {
    // Intentionally left blank.
}
