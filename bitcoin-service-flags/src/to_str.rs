// ---------------- [ File: bitcoin-service-flags/src/to_str.rs ]
crate::ix!();

/// Convert a single service‑flag **bit index** (`0‥=63`) to a human‑readable
/// string.
///
/// Unknown bits are rendered as `"UNKNOWN[2^{n}]"`, exactly mirroring the C++
/// implementation.
pub fn service_flag_to_str(bit: usize) -> String {
    trace!(target: "service_flag_to_str", bit, "Translating bit to string");

    let service_flag: u64 = 1u64 << bit;

    let out = match service_flag {
        x if x == ServiceFlags::NODE_NETWORK.bits()         => "NETWORK",
        x if x == ServiceFlags::NODE_BLOOM.bits()           => "BLOOM",
        x if x == ServiceFlags::NODE_WITNESS.bits()         => "WITNESS",
        x if x == ServiceFlags::NODE_COMPACT_FILTERS.bits() => "COMPACT_FILTERS",
        x if x == ServiceFlags::NODE_NETWORK_LIMITED.bits() => "NETWORK_LIMITED",
        _ => {
            let s = format!("UNKNOWN[2^{}]", bit);
            debug!(
                target: "service_flag_to_str",
                service_flag,
                bit,
                unknown = true,
                s = %s,
                "Encountered unknown service flag"
            );
            return s;
        }
    };

    out.to_string()
}

/// Convert a full service‑flag **bitmask** to an ordered list of human‑readable
/// strings.
///
/// The order follows the bit ordering from LSB to MSB, identical to the C++
/// loop.
pub fn service_flags_to_str(flags: u64) -> Vec<String> {
    let mut str_flags = Vec::new();

    for bit in 0..u64::BITS {
        if flags & (1u64 << bit) != 0 {
            str_flags.push(service_flag_to_str(bit as usize));
        }
    }

    str_flags
}

#[cfg(test)]
mod service_flag_stringification {
    use super::*;

    #[traced_test]
    fn single_known_flag() {
        assert_eq!(service_flag_to_str(0), "NETWORK");          // NODE_NETWORK
        assert_eq!(service_flag_to_str(3), "WITNESS");          // NODE_WITNESS
    }

    #[traced_test]
    fn unknown_flag() {
        let unknown = service_flag_to_str(42);
        assert_eq!(&unknown, "UNKNOWN[2^42]");
    }

    #[traced_test]
    fn multi_flag_mask() {
        let mask = ServiceFlags::NODE_NETWORK.bits()
            | ServiceFlags::NODE_BLOOM.bits()
            | (1u64 << 42); // deliberately include an unknown bit

        let mut strings = service_flags_to_str(mask);
        // Preserve insertion order from LSB…MSB
        assert_eq!(
            strings,
            vec![
                "NETWORK".to_string(),
                "BLOOM".to_string(),
                "UNKNOWN[2^42]".to_string()
            ]
        );
    }
}
