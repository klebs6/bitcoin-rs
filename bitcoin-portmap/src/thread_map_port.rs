// ---------------- [ File: bitcoin-portmap/src/thread_map_port.rs ]
crate::ix!();

#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub fn thread_map_port() {
    set_syscall_sandbox_policy(SyscallSandboxPolicy::InitializationMapPort);
    debug!("map‑port thread main loop entered");

    loop {
        let mut ok = false;

        // High‑priority: UPnP
        #[cfg(feature = "upnp")]
        if g_mapport_enabled_protos().load(atomic::Ordering::SeqCst)
            & MapPortProtoFlag::UPNP as u32
            != 0
        {
            *g_mapport_current_proto() = MapPortProtoFlag::UPNP;
            ok = process_upnp();
            if ok {
                continue;
            }
        }

        // Low‑priority: NAT‑PMP
        #[cfg(feature = "natpmp")]
        if g_mapport_enabled_protos().load(atomic::Ordering::SeqCst)
            & MapPortProtoFlag::NAT_PMP as u32
            != 0
        {
            *g_mapport_current_proto() = MapPortProtoFlag::NAT_PMP;
            ok = process_natpmp();
            if ok {
                continue;
            }
        }

        *g_mapport_current_proto() = MapPortProtoFlag::NONE;

        if g_mapport_enabled_protos().load(atomic::Ordering::SeqCst) == MapPortProtoFlag::NONE as u32
        {
            info!("no port‑mapping protocols enabled – exiting");
            return;
        }

        if ok {
            continue;
        }

        if !g_mapport_interrupt().sleep_for(PORT_MAPPING_RETRY_PERIOD) {
            warn!("map‑port thread interrupted – terminating");
            break;
        }
    }
}
