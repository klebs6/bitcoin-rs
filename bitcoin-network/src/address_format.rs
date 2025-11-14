// ---------------- [ File: bitcoin-network/src/address_format.rs ]
crate::ix!();

/**
  | A flag that is ORed into the protocol
  | version to designate that addresses
  | should be serialized in (unserialized
  | from) v2 format (BIP155).
  | 
  | Make sure that this does not collide
  | with any of the values in `version.h`
  | or with `SERIALIZE_TRANSACTION_NO_WITNESS`.
  |
  */
pub const ADDRV2_FORMAT: i32 = 0x20000000;

#[cfg(test)]
mod addrv2_flag_invariants_spec {
    use super::*;

    #[traced_test]
    fn addrv2_flag_is_single_bit_and_noncolliding() {
        info!(flag_hex = format_args!("{:#x}", ADDRV2_FORMAT), "Verifying ADDRV2_FORMAT invariants");
        let f = ADDRV2_FORMAT as u32;

        // Non‑zero and single‑bit
        assert_ne!(f, 0, "ADDRV2_FORMAT must not be zero");
        assert_eq!(f & (f - 1), 0, "ADDRV2_FORMAT must set exactly one bit");

        // Must not collide with commonly used serialization flags (e.g. NO_WITNESS=0x4000_0000)
        let serialize_tx_no_witness: u32 = 0x4000_0000;
        assert_eq!(
            f & serialize_tx_no_witness,
            0,
            "ADDRV2_FORMAT must not collide with SERIALIZE_TRANSACTION_NO_WITNESS"
        );

        // Must be below sign bit
        assert!(f < 0x8000_0000, "ADDRV2_FORMAT must not use the sign bit");
    }
}
