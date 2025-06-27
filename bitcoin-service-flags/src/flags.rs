// ---------------- [ File: bitcoin-service-flags/src/flags.rs ]
crate::ix!();

lazy_static!{
    pub static ref INITIAL_BLOCK_DOWNLOAD_COMPLETED: AtomicBool = AtomicBool::new(false);
}

/**
  | nServices flags
  |
  */
//#[repr(u64)]
bitflags!{

    #[derive(Serialize,Deserialize)]
    pub struct ServiceFlags: u64 {

        /**
          | @note
          | 
          | When adding here, be sure to update
          | serviceFlagToStr too Nothing
          |
          */
        const NODE_NONE = 0;

        /**
          | NODE_NETWORK means that the node is capable
          | of serving the complete block chain. It is
          | currently set by all Bitcoin Core non
          | pruned nodes, and is unset by SPV clients
          | or other light clients.
          */
        const NODE_NETWORK = (1 << 0);

        /**
          | NODE_BLOOM means the node is capable and
          | willing to handle bloom-filtered
          | connections.
          |
          | Bitcoin Core nodes used to support this by
          | default, without advertising this bit, but
          | no longer do as of protocol version 70011
          | (= NO_BLOOM_VERSION)
          */
        const NODE_BLOOM = 1 << 2;

        /**
          | NODE_WITNESS indicates that a node
          | can be asked for blocks and transactions
          | including witness data.
          |
          */
        const NODE_WITNESS = 1 << 3;

        /**
          | NODE_COMPACT_FILTERS means the node will
          | service basic block filter requests.
          |
          | See BIP157 and BIP158 for details on how
          | this is implemented.
          */
        const NODE_COMPACT_FILTERS = 1 << 6;

        /**
          | NODE_NETWORK_LIMITED means the same as
          | NODE_NETWORK with the limitation of only
          | serving the last 288 (2 day) blocks
          |
          | See BIP159 for details on how this is
          | implemented.
          */
        const NODE_NETWORK_LIMITED = 1 << 10;

        /*
          | Bits 24-31 are reserved for temporary
          | experiments. Just pick a bit that isn't
          | getting used, or one not being used much,
          | and notify the bitcoin-development mailing
          | list. Remember that service bits are just
          | unauthenticated advertisements, so your
          | code must be robust against collisions and
          | other cases where nodes may be advertising
          | a service they do not actually
          | support. Other service bits should be
          | allocated via the BIP process.
          */
    }
}

impl Default for ServiceFlags {
    fn default() -> Self {
        Self::NODE_NONE
    }
}

impl From<u64> for ServiceFlags {
    /// Loss‑less conversion from a raw `u64` bitfield coming off the wire.
    ///
    /// * All *known* bits are mapped onto the corresponding `ServiceFlags`
    ///   constants.
    /// * Any *unknown* bits are **retained** so that round‑tripping back to
    ///   `u64` yields the exact same value.  
    ///
    /// This matches the C++ behaviour that treats the flag word as an
    /// **opaque bitmap** while still letting the type system reason about the
    /// well‑defined bits.
    #[inline]
    fn from(raw: u64) -> Self {
        use tracing::{debug, trace};

        trace!(target: "service_flags", raw, "Converting raw service‑flag word");

        // `from_bits_unchecked` keeps *all* bits (known and unknown).  Safe
        // because the bit pattern is explicitly carried through from an
        // external source we wish to preserve verbatim.
        //
        // SAFETY: No invariants are violated because we intentionally allow
        // any combination of bits in order to echo the original bitmap.
        let flags = unsafe { ServiceFlags::from_bits_unchecked(raw) };

        debug!(
            target: "service_flags",
            ?flags,
            "Finished conversion from raw service‑flag word"
        );

        flags
    }
}

#[cfg(test)]
mod service_flags_roundtrip {
    use super::*;

    /// Verify that every advertised constant round‑trips through `u64`.
    #[traced_test]
    fn known_bits_roundtrip() {
        for constant in [
            ServiceFlags::NODE_NONE,
            ServiceFlags::NODE_NETWORK,
            ServiceFlags::NODE_BLOOM,
            ServiceFlags::NODE_WITNESS,
            ServiceFlags::NODE_COMPACT_FILTERS,
            ServiceFlags::NODE_NETWORK_LIMITED,
        ] {
            let raw: u64 = constant.bits();
            let reconstructed = ServiceFlags::from(raw);
            info!(
                target: "service_flags::test",
                ?constant,
                raw,
                ?reconstructed,
                "Verifying round‑trip equality for a known flag"
            );
            assert_eq!(reconstructed, constant);
        }
    }

    /// Ensure that **unknown** bits are preserved by the conversion.
    #[traced_test]
    fn unknown_bits_are_retained() {
        // Pick a bit that is currently outside the defined range (e.g. bit 42).
        let raw_unknown: u64 = 1u64 << 42;
        let flags          = ServiceFlags::from(raw_unknown);
        info!(
            target: "service_flags::test",
            raw_unknown,
            ?flags,
            "Verifying that an unknown bit survives the conversion"
        );
        assert_eq!(flags.bits(), raw_unknown);
    }
}

