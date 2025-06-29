crate::ix!();

/// Is the background thread joinable (alive)?
#[inline]
pub fn is_thread_joinable() -> bool {
    g_mapport_thread()
        .lock()
        .expect("G_MAPPORT_THREAD lock")
        .as_ref()
        .map(|h| !h.is_finished())
        .unwrap_or(false)
}
