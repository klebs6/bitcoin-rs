// ---------------- [ File: bitcoin-portmap/src/portmap.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/mapport.h]

#[cfg(USE_UPNP)]        pub const DEFAULT_UPNP:   bool = USE_UPNP;
#[cfg(not(USE_UPNP))]   pub const DEFAULT_UPNP:   bool = false;

#[cfg(USE_NATPMP)]      pub const DEFAULT_NATPMP: bool = USE_NATPMP;
#[cfg(not(USE_NATPMP))] pub const DEFAULT_NATPMP: bool = false;

#[repr(u32)]
pub enum MapPortProtoFlag {
    NONE    = 0x00,
    UPNP    = 0x01,
    NAT_PMP = 0x02,
}

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
lazy_static!{
    /*
    static CThreadInterrupt g_mapport_interrupt;
    static std::thread g_mapport_thread;
    static std::atomic_uint g_mapport_enabled_protos{MapPortProtoFlag::NONE};
    static std::atomic<MapPortProtoFlag> g_mapport_current_proto{MapPortProtoFlag::NONE};
    */
}

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

#[cfg(any(USE_NATPMP,USE_UPNP))]
pub fn map_port_proto_set_enabled(
        proto:   MapPortProtoFlag,
        enabled: bool)  {
    
    todo!();
        /*
            if (enabled) {
            g_mapport_enabled_protos |= proto;
        } else {
            g_mapport_enabled_protos &= ~proto;
        }
        */
}
