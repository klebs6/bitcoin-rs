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

//-------------------------------------------[.cpp/bitcoin/src/mapport.cpp]

/**
  | The minimum supported miniUPnPc API version is
  | set to 10. This keeps compatibility with Ubuntu
  | 16.04 LTS and Debian 8 libminiupnpc-dev
  | packages.
  */
#[cfg(feature="upnpp")]
const_assert!{
    MINIUPNPC_API_VERSION >= 10
}

/// 20 minutes
#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub const PORT_MAPPING_REANNOUNCE_PERIOD: Duration = Duration::seconds(20 * 60);

/// 5 minutes
#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub const PORT_MAPPING_RETRY_PERIOD:      Duration = Duration::seconds(5  * 60);

#[cfg(feature="natpmp")]
pub fn natpmp_init(natpmp: *mut Natpmp) -> bool {
    
    todo!();
        /*
            const int r_init = initnatpmp(natpmp, /* detect gateway automatically */ 0, /* forced gateway - NOT APPLIED*/ 0);
        if (r_init == 0) return true;
        LogPrintf("natpmp: initnatpmp() failed with %d error.\n", r_init);
        return false;
        */
}
