// ---------------- [ File: bitcoin-service-flags/src/get_desirable_service_flags.rs ]
crate::ix!();

/// Gets the set of service flags which are "desirable" for a given peer.
/// 
/// These are the flags which are required for a peer to support for them to be "interesting" to
/// us, ie for us to wish to use one of our few outbound connection slots for or for us to wish to
/// prioritize keeping their connection around.
/// 
/// Relevant service flags may be peer- and state-specific in that the version of the peer may
/// determine which flags are required (eg in the case of NODE_NETWORK_LIMITED where we seek out
/// NODE_NETWORK peers unless they set NODE_NETWORK_LIMITED and we are out of IBD, in which case
/// NODE_NETWORK_LIMITED suffices).
/// 
/// Thus, generally, avoid calling with peerServices == NODE_NONE, unless state-specific flags must
/// absolutely be avoided. When called with peerServices == NODE_NONE, the returned desirable
/// service flags are guaranteed to not change dependent on state - ie they are suitable for use
/// when describing peers which we know to be desirable, but for which we do not have a confirmed
/// set of service flags.
/// 
/// If the NODE_NONE return value is changed, contrib/seeds/makeseeds.py should be updated
/// appropriately to filter for the same nodes.
/// 
/// Return the *current* set of flags a peer **should** advertise to be deemed “interesting”.
///
/// This mirrors the original C++ logic, including its dependency on the global
/// `g_initial_block_download_completed` flag that flips once the local node leaves IBD.
pub fn get_desirable_service_flags(services: ServiceFlags) -> ServiceFlags {

    trace!(
        target: "get_desirable_service_flags",
        ?services,
        "Computing desirable flags for peer"
    );

    let ibd_done = INITIAL_BLOCK_DOWNLOAD_COMPLETED.load(atomic::Ordering::Relaxed);

    let desirable = if services.contains(ServiceFlags::NODE_NETWORK_LIMITED) && ibd_done {
        ServiceFlags::NODE_NETWORK_LIMITED | ServiceFlags::NODE_WITNESS
    } else {
        ServiceFlags::NODE_NETWORK | ServiceFlags::NODE_WITNESS
    };

    debug!(
        target: "get_desirable_service_flags",
        ?services,
        ibd_done,
        ?desirable,
        "Selected desirable service‑flags"
    );

    desirable
}

#[cfg(test)]
mod desirable_flags_selection {
    use super::*;

    // Serialise all tests that mutate the global IBD flag so they cannot race.
    lazy_static! {
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    #[traced_test]
    fn ibd_not_completed_prefers_full_network_peers() {
        let _guard = TEST_LOCK.lock(); // ‑‑ hold for the whole test

        INITIAL_BLOCK_DOWNLOAD_COMPLETED.store(false, atomic::Ordering::Relaxed);

        let flags      = ServiceFlags::NODE_NETWORK_LIMITED | ServiceFlags::NODE_WITNESS;
        let desirable  = super::get_desirable_service_flags(flags);

        assert_eq!(
            desirable,
            ServiceFlags::NODE_NETWORK | ServiceFlags::NODE_WITNESS,
            "Before IBD finishes, we still prefer full NODE_NETWORK peers"
        );
    }

    #[traced_test]
    fn ibd_completed_accepts_limited_peers() {
        let _guard = TEST_LOCK.lock(); // ‑‑ hold for the whole test

        INITIAL_BLOCK_DOWNLOAD_COMPLETED.store(true, atomic::Ordering::Relaxed);

        let flags      = ServiceFlags::NODE_NETWORK_LIMITED | ServiceFlags::NODE_WITNESS;
        let desirable  = super::get_desirable_service_flags(flags);

        assert_eq!(
            desirable,
            ServiceFlags::NODE_NETWORK_LIMITED | ServiceFlags::NODE_WITNESS,
            "After IBD, NODE_NETWORK_LIMITED is sufficient"
        );
    }
}
