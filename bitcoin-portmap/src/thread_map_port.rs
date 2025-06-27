// ---------------- [ File: bitcoin-portmap/src/thread_map_port.rs ]
crate::ix!();

#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub fn start_thread_map_port() {
    use std::thread;

    // Skip spawning if the thread is already running.
    if G_MAPPORT_THREAD()
        .lock()
        .expect("G_MAPPORT_THREAD lock")
        .as_ref()
        .map(|h| !h.is_finished())
        .unwrap_or(false)
    {
        debug!("map‑port thread already active");
        return;
    }

    // Invariant from the original C++: the interrupt flag must be clear.
    assert!(
        !G_MAPPORT_INTERRUPT().is_set(),
        "map‑port interrupt must be clear when spawning thread"
    );

    let handle = thread::Builder::new()
        .name("mapport".into())
        .spawn(|| trace_thread("mapport", thread_map_port))
        .expect("failed to spawn map‑port thread");

    *G_MAPPORT_THREAD()
        .lock()
        .expect("G_MAPPORT_THREAD lock") = Some(handle);

    info!("map‑port thread spawned");
}

#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub fn thread_map_port() {

    set_syscall_sandbox_policy(SyscallSandboxPolicy::InitializationMapPort);
    debug!("map‑port thread main loop entered");

    loop {
        let mut ok = false;

        // ── High‑priority protocol: UPnP ───────────────────────────────────────
        #[cfg(feature = "upnp")]
        {
            if G_MAPPORT_ENABLED_PROTOS().contains(MapPortProtoFlag::UPNP) {
                *G_MAPPORT_CURRENT_PROTO() = MapPortProtoFlag::UPNP;
                ok = process_upnp();
                if ok {
                    debug!("UPnP mapping succeeded");
                    continue;
                }
            }
        }

        // ── Low‑priority protocol: NAT‑PMP ─────────────────────────────────────
        #[cfg(feature = "natpmp")]
        {
            if G_MAPPORT_ENABLED_PROTOS().contains(MapPortProtoFlag::NAT_PMP) {
                *G_MAPPORT_CURRENT_PROTO() = MapPortProtoFlag::NAT_PMP;
                ok = process_natpmp();
                if ok {
                    debug!("NAT‑PMP mapping succeeded");
                    continue;
                }
            }
        }

        *G_MAPPORT_CURRENT_PROTO() = MapPortProtoFlag::NONE;

        // Exit if every protocol has been disabled.
        if G_MAPPORT_ENABLED_PROTOS() == MapPortProtoFlag::NONE {
            info!("no port‑mapping protocols enabled – exiting");
            return;
        }

        // Match the original `do … while(ok || sleep_for(...))` behaviour.
        if ok {
            continue;
        }

        if !G_MAPPORT_INTERRUPT().sleep_for(PORT_MAPPING_RETRY_PERIOD) {
            warn!("map‑port thread interrupted – terminating");
            break;
        }
    }
}
