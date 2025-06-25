crate::ix!();

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
