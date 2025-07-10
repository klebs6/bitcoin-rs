// ---------------- [ File: bitcoin-portmap/src/is_mapport_thread_joinable.rs ]
crate::ix!();

/// Is the background thread joinable (alive)?
#[inline]
pub fn is_mapport_thread_joinable() -> bool {
    !g_mapport_thread()
        .lock()
        .as_ref()
        .expect("G_MAPPORT_THREAD lock")
        .is_finished()
}
