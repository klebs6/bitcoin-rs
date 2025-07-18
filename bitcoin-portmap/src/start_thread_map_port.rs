// ---------------- [ File: bitcoin-portmap/src/start_thread_map_port.rs ]
crate::ix!();

#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub fn start_thread_map_port() {
    // Already running?
    if is_mapport_thread_joinable() {
        debug!("map‑port thread already active");
        return;
    }

    assert!(
        !g_mapport_interrupt().is_set(),
        "interrupt must be clear when spawning thread"
    );

    let handle = std::thread::Builder::new()
        .name("mapport".into())
        .spawn(|| trace_thread("mapport", thread_map_port))
        .expect("failed to spawn map‑port thread");

    *g_mapport_thread().lock().expect("G_MAPPORT_THREAD lock") = Some(handle);
    info!("map‑port thread spawned");
}
