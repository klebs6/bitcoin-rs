// ---------------- [ File: bitcoin-service-flags/src/checks.rs ]
crate::ix!();

/**
  | A shortcut for
  | ```
  | (services & get_desirable_service_flags(services))
  |     == get_desirable_service_flags(services)
  | ```
  | i.e. determine whether the given set of
  | service‑flags already satisfies every flag
  | we presently consider *desirable* for that
  | peer.
  */
#[inline]
pub fn has_all_desirable_service_flags(services: ServiceFlags) -> bool {
    use tracing::{debug, trace};

    let desirable = get_desirable_service_flags(services);
    let missing   = desirable & !services;

    trace!(
        target: "checks::has_all_desirable_service_flags",
        ?services,
        ?desirable,
        ?missing,
        "Evaluated desirability mask"
    );

    let complete = missing.is_empty();

    debug!(
        target: "checks::has_all_desirable_service_flags",
        complete,
        "Peer {} all desirable flags", if complete { "HAS" } else { "LACKS" }
    );

    complete
}

/**
  | Checks if a peer with the given service‑flags
  | may be capable of hosting a *robust* on‑disk
  | address‑storage database (addrman).
  */
#[inline]
pub fn may_have_useful_addressdb(services: ServiceFlags) -> bool {
    use tracing::trace;

    let useful = services.intersects(
        ServiceFlags::NODE_NETWORK | ServiceFlags::NODE_NETWORK_LIMITED,
    );

    trace!(
        target: "checks::may_have_useful_addressdb",
        ?services,
        useful,
        "Evaluated usefulness of peer's address DB capability"
    );

    useful
}

#[cfg(test)]
mod peer_relevance_checks {
    use super::*;

    #[traced_test]
    fn desirability_complete_vs_incomplete() {
        let complete = ServiceFlags::NODE_NETWORK | ServiceFlags::NODE_WITNESS;
        let incomplete = ServiceFlags::NODE_NETWORK; // missing WITNESS

        assert!(
            has_all_desirable_service_flags(complete),
            "Complete flag‑set should be recognised"
        );
        assert!(
            !has_all_desirable_service_flags(incomplete),
            "Missing any desirable flag must fail the check"
        );
    }

    #[traced_test]
    fn address_db_capability() {
        assert!(
            may_have_useful_addressdb(ServiceFlags::NODE_NETWORK),
            "Full NODE_NETWORK peers have useful addr DBs"
        );
        assert!(
            may_have_useful_addressdb(ServiceFlags::NODE_NETWORK_LIMITED),
            "NODE_NETWORK_LIMITED peers are also accepted"
        );
        assert!(
            !may_have_useful_addressdb(ServiceFlags::NODE_BLOOM),
            "Unrelated capabilities alone are insufficient"
        );
    }
}
