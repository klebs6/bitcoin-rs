// ---------------- [ File: bitcoin-portmap/src/lifecycle.rs ]
crate::ix!();

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
