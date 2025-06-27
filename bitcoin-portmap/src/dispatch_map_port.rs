// ---------------- [ File: bitcoin-portmap/src/dispatch_map_port.rs ]
crate::ix!();

#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub fn dispatch_map_port() {

    trace!(target: "portmap", "dispatch_map_port() invoked");

    let current = G_MAPPORT_CURRENT_PROTO.load(atomic::Ordering::SeqCst);
    let enabled = G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst);

    /* ---- 1. nothing enabled, nothing running ---- */
    if current == MapPortProtoFlag::NONE as u32 && enabled == MapPortProtoFlag::NONE as u32 {
        trace!(target: "portmap", "no protocols enabled or active – return");
        return;
    }

    /* ---- 2. something enabled but not yet running ---- */
    if current == MapPortProtoFlag::NONE as u32 && enabled != MapPortProtoFlag::NONE as u32 {
        trace!(target: "portmap", "starting background map‑port thread");
        start_thread_map_port();
        return;
    }

    /* ---- 3. thread running but user disabled everything ---- */
    if current != MapPortProtoFlag::NONE as u32 && enabled == MapPortProtoFlag::NONE as u32 {
        trace!(
            target: "portmap",
            "all protocols disabled – interrupting & stopping thread"
        );
        interrupt_map_port();
        stop_map_port();
        return;
    }

    /* ---- 4. current proto still allowed – keep going ---- */
    if enabled & current != 0 {
        trace!(
            target: "portmap",
            "current protocol still enabled – no thread switch required"
        );
        return;
    }

    /* ---- 5. switch to a different protocol ---- */
    assert!(is_thread_joinable(), "expected running thread");
    assert!(
        !G_MAPPORT_INTERRUPT.is_interrupted(),
        "interrupt must be clear before signalling"
    );

    trace!(
        target: "portmap",
        "signalling thread to switch protocol (interrupt)"
    );
    G_MAPPORT_INTERRUPT.interrupt();
}

#[cfg(all(test, any(feature = "natpmp", feature = "upnp")))]
mod dispatch_logic_behaviour {
    use super::*;

    /// Only verifies early‑return cases; deeper paths rely on the
    /// thread implementation and are covered elsewhere.
    #[traced_test]
    fn returns_immediately_on_no_proto() {
        G_MAPPORT_CURRENT_PROTO.store(MapPortProtoFlag::NONE as u32, atomic::Ordering::SeqCst);
        G_MAPPORT_ENABLED_PROTOS.store(MapPortProtoFlag::NONE as u32, atomic::Ordering::SeqCst);

        // Must not panic.
        dispatch_map_port();
    }
}
