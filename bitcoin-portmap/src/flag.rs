// ---------------- [ File: bitcoin-portmap/src/flag.rs ]
crate::ix!();

/// Enabled protocol bit‑mask (UPnP/NAT‑PMP).
#[cfg(any(feature = "natpmp", feature = "upnp"))]
static G_MAPPORT_ENABLED_PROTOS: AtomicU32 = AtomicU32::new(MapPortProtoFlag::NONE as u32);

/// Protocol currently in use by the background thread.
#[cfg(any(feature = "natpmp", feature = "upnp"))]
static G_MAPPORT_CURRENT_PROTO: AtomicU32 = AtomicU32::new(MapPortProtoFlag::NONE as u32);

/// Atomically enable or disable a `MapPortProtoFlag`.
///
/// Mirrors the original C++ logic while adding extensive
/// [`tracing`] diagnostics.
///
/// # Thread Safety
/// Uses `Ordering::SeqCst` to maintain the strictest memory‑ordering
/// guarantees, exactly matching the semantics of `std::atomic_uint`
/// from the reference implementation.
#[cfg(any(feature = "natpmp", feature = "upnp"))]
pub fn map_port_proto_set_enabled(proto: MapPortProtoFlag, enabled: bool) {
    trace!(
        target: "portmap",
        "map_port_proto_set_enabled: proto={:?} enabled={}",
        proto,
        enabled
    );

    if enabled {
        let previous = G_MAPPORT_ENABLED_PROTOS.fetch_or(proto as u32, atomic::Ordering::SeqCst);
        trace!(
            target: "portmap",
            "enable proto={:?} previous_mask={:#04x} new_mask={:#04x}",
            proto,
            previous,
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst)
        );
    } else {
        let previous = G_MAPPORT_ENABLED_PROTOS.fetch_and(!(proto as u32), atomic::Ordering::SeqCst);
        trace!(
            target: "portmap",
            "disable proto={:?} previous_mask={:#04x} new_mask={:#04x}",
            proto,
            previous,
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst)
        );
    }
}

#[cfg(any(feature = "natpmp", feature = "upnp"))]
#[cfg(test)]
mod map_port_proto_flag_behavior {
    use super::*;

    /// Verifies that enabling and disabling port‑mapping protocols
    /// adjusts the global mask exactly as in the original C++ logic.
    #[traced_test]
    fn verifies_flag_mask_transitions() {
        // Reset to a known state.
        G_MAPPORT_ENABLED_PROTOS.store(MapPortProtoFlag::NONE as u32, atomic::Ordering::SeqCst);

        // Enable UPNP.
        map_port_proto_set_enabled(MapPortProtoFlag::UPNP, true);
        assert_eq!(
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst),
            MapPortProtoFlag::UPNP as u32
        );

        // Enable NAT‑PMP in addition.
        map_port_proto_set_enabled(MapPortProtoFlag::NAT_PMP, true);
        assert_eq!(
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst),
            (MapPortProtoFlag::UPNP as u32) | (MapPortProtoFlag::NAT_PMP as u32)
        );

        // Disable UPNP.
        map_port_proto_set_enabled(MapPortProtoFlag::UPNP, false);
        assert_eq!(
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst),
            MapPortProtoFlag::NAT_PMP as u32
        );

        // Disable NAT‑PMP, leaving mask empty.
        map_port_proto_set_enabled(MapPortProtoFlag::NAT_PMP, false);
        assert_eq!(
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst),
            MapPortProtoFlag::NONE as u32
        );
    }

    /// Ensures each transition produces the exact same bit‑mask
    /// as the reference implementation.
    #[traced_test]
    fn transitions_match_reference() {
        // Start from a clean slate.
        G_MAPPORT_ENABLED_PROTOS.store(MapPortProtoFlag::NONE as u32, atomic::Ordering::SeqCst);

        // Enable UPnP.
        map_port_proto_set_enabled(MapPortProtoFlag::UPNP, true);
        assert_eq!(
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst),
            MapPortProtoFlag::UPNP as u32
        );

        // Enable NAT‑PMP alongside.
        map_port_proto_set_enabled(MapPortProtoFlag::NAT_PMP, true);
        assert_eq!(
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst),
            (MapPortProtoFlag::UPNP as u32) | (MapPortProtoFlag::NAT_PMP as u32)
        );

        // Disable UPnP.
        map_port_proto_set_enabled(MapPortProtoFlag::UPNP, false);
        assert_eq!(
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst),
            MapPortProtoFlag::NAT_PMP as u32
        );

        // Disable NAT‑PMP.
        map_port_proto_set_enabled(MapPortProtoFlag::NAT_PMP, false);
        assert_eq!(
            G_MAPPORT_ENABLED_PROTOS.load(atomic::Ordering::SeqCst),
            MapPortProtoFlag::NONE as u32
        );
    }
}
