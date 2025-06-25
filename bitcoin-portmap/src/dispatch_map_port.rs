// ---------------- [ File: bitcoin-portmap/src/dispatch_map_port.rs ]
crate::ix!();

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
