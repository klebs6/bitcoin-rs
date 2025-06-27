// ---------------- [ File: bitcoin-portmap/src/portmap.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/mapport.h]

#[cfg(feature = "upnp")]      pub const DEFAULT_UPNP: bool = true;
#[cfg(not(feature = "upnp"))] pub const DEFAULT_UPNP: bool = false;

#[cfg(feature = "natpmp")]      pub const DEFAULT_NATPMP: bool = true;
#[cfg(not(feature = "natpmp"))] pub const DEFAULT_NATPMP: bool = false;

/// Protocol bit flags (kept identical to the C++ values).
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MapPortProtoFlag {
    NONE    = 0x00,
    UPNP    = 0x01,
    NAT_PMP = 0x02,
}

#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub(crate) static G_MAPPORT_ENABLED_PROTOS: AtomicU32 =
    AtomicU32::new(MapPortProtoFlag::NONE as u32);

#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub(crate) static G_MAPPORT_CURRENT_PROTO: AtomicU32 =
    AtomicU32::new(MapPortProtoFlag::NONE as u32);


//-------------------------------------------[.cpp/bitcoin/src/mapport.cpp]

/**
  | The minimum supported miniUPnPc API version is
  | set to 10. This keeps compatibility with Ubuntu
  | 16.04 LTS and Debian 8 libminiupnpc-dev
  | packages.
  */
#[cfg(USE_UPNP)]
const_assert!{
    MINIUPNPC_API_VERSION >= 10
} //"miniUPnPc API version >= 10 assumed"

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub const PORT_MAPPING_REANNOUNCE_PERIOD: Minutes = 20;

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub const PORT_MAPPING_RETRY_PERIOD:      Minutes = 5;

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_NATPMP)]
lazy_static!{
    /*
    static uint16_t g_mapport_external_port = 0;
    */
}

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_NATPMP)]
pub fn natpmp_init(natpmp: *mut NatPmp) -> bool {
    
    todo!();
        /*
            const int r_init = initnatpmp(natpmp, /* detect gateway automatically */ 0, /* forced gateway - NOT APPLIED*/ 0);
        if (r_init == 0) return true;
        LogPrintf("natpmp: initnatpmp() failed with %d error.\n", r_init);
        return false;
        */
}

/// Single global instance mirroring `g_mapport_interrupt` in C++.
#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub(crate) static G_MAPPORT_INTERRUPT: Lazy<ThreadInterrupt> = Lazy::new(ThreadInterrupt::new);
