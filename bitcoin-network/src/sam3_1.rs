// ---------------- [ File: bitcoin-network/src/sam3_1.rs ]
crate::ix!();

/**
  | SAM 3.1 and earlier do not support specifying
  | ports and force the port to 0.
  |
  */
pub const I2P_SAM31_PORT: u16 = 0;

#[cfg(test)]
mod sam31_constant_spec {
    use super::*;

    #[traced_test]
    fn i2p_sam31_port_is_zero() {
        assert_eq!(I2P_SAM31_PORT, 0);
    }
}
