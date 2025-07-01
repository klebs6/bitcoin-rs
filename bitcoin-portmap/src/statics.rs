// ---------------- [ File: bitcoin-portmap/src/statics.rs ]
crate::ix!();

/// Single global instance mirroring `g_mapport_interrupt` in C++.
#[cfg(any(feature = "natpmp", feature = "upnp"))]
static G_MAPPORT_INTERRUPT: Lazy<ThreadInterrupt> = Lazy::new(ThreadInterrupt::new);

/// Enabled protocol bit‑mask (UPnP/NAT‑PMP).
#[cfg(any(feature = "natpmp", feature = "upnp"))]
static G_MAPPORT_ENABLED_PROTOS_STATIC: AtomicU32 = AtomicU32::new(MapPortProtoFlag::NONE as u32);

/// Protocol currently in use by the background thread.
#[cfg(any(feature = "natpmp", feature = "upnp"))]
static G_MAPPORT_CURRENT_PROTO_STATIC:  AtomicU32 = AtomicU32::new(MapPortProtoFlag::NONE as u32);

#[cfg(any(feature = "natpmp", feature = "upnp"))]
static G_MAPPORT_INTERRUPT_STATIC:      Lazy<ThreadInterrupt> = Lazy::new(ThreadInterrupt::new);

#[cfg(feature="natpmp")]
static G_MAPPORT_EXTERNAL_PORT_STATIC:  Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0u16));

/// Global thread handle (`Option<JoinHandle<()>>`)
static G_MAPPORT_THREAD: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

// Public accessor fns matching the old “callable‐static” style --------------------

#[inline]
#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub fn g_mapport_enabled_protos() -> &'static AtomicU32 {
    &G_MAPPORT_ENABLED_PROTOS_STATIC
}

#[inline]
#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub fn g_mapport_current_proto() -> &'static AtomicU32 {
    &G_MAPPORT_CURRENT_PROTO_STATIC
}

#[inline]
#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub fn g_mapport_interrupt() -> &'static ThreadInterrupt {
    &G_MAPPORT_INTERRUPT_STATIC
}

#[inline]
#[cfg(feature="natpmp")]
pub fn g_mapport_external_port() -> &'static Mutex<u16> {
    &G_MAPPORT_EXTERNAL_PORT_STATIC
}

/// Accessor identical in spirit to the C++ static.
#[inline]
pub fn g_mapport_thread() -> &'static Mutex<Option<JoinHandle<()>>> {
    &G_MAPPORT_THREAD
}
