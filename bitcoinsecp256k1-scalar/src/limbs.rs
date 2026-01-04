// ---------------- [ File: bitcoinsecp256k1-scalar/src/limbs.rs ]
crate::ix!();

/**
  | Limbs of the secp256k1 order.
  |
  */
#[cfg(feature="widemul-int128")] pub const N_0: u64 = 0xBFD25E8CD0364141;
#[cfg(feature="widemul-int128")] pub const N_1: u64 = 0xBAAEDCE6AF48A03B;
#[cfg(feature="widemul-int128")] pub const N_2: u64 = 0xFFFFFFFFFFFFFFFE;
#[cfg(feature="widemul-int128")] pub const N_3: u64 = 0xFFFFFFFFFFFFFFFF;

/**
  | Limbs of 2^256 minus the secp256k1 order.
  |
  */
#[cfg(feature="widemul-int128")] pub const N_C_0: u64 = !N_0 + 1;
#[cfg(feature="widemul-int128")] pub const N_C_1: u64 = !N_1;
#[cfg(feature="widemul-int128")] pub const N_C_2: u64 = 1;

/**
  | Limbs of half the secp256k1 order.
  |
  */
#[cfg(feature="widemul-int128")] pub const N_H_0: u64 = 0xDFE92F46681B20A0;
#[cfg(feature="widemul-int128")] pub const N_H_1: u64 = 0x5D576E7357A4501D;
#[cfg(feature="widemul-int128")] pub const N_H_2: u64 = 0xFFFFFFFFFFFFFFFF;
#[cfg(feature="widemul-int128")] pub const N_H_3: u64 = 0x7FFFFFFFFFFFFFFF;

//-----------------------------

/**
  | Limbs of the secp256k1 order.
  |
  */
#[cfg(feature="widemul-int64")] pub const N_0: u32 = 0xD0364141;
#[cfg(feature="widemul-int64")] pub const N_1: u32 = 0xBFD25E8C;
#[cfg(feature="widemul-int64")] pub const N_2: u32 = 0xAF48A03B;
#[cfg(feature="widemul-int64")] pub const N_3: u32 = 0xBAAEDCE6;
#[cfg(feature="widemul-int64")] pub const N_4: u32 = 0xFFFFFFFE;
#[cfg(feature="widemul-int64")] pub const N_5: u32 = 0xFFFFFFFF;
#[cfg(feature="widemul-int64")] pub const N_6: u32 = 0xFFFFFFFF;
#[cfg(feature="widemul-int64")] pub const N_7: u32 = 0xFFFFFFFF;

/**
  | Limbs of 2^256 minus the secp256k1 order.
  |
  */
#[cfg(feature="widemul-int64")] pub const N_C_0: u32 = !N_0 + 1;
#[cfg(feature="widemul-int64")] pub const N_C_1: u32 = !N_1;
#[cfg(feature="widemul-int64")] pub const N_C_2: u32 = !N_2;
#[cfg(feature="widemul-int64")] pub const N_C_3: u32 = !N_3;
#[cfg(feature="widemul-int64")] pub const N_C_4: u32 = 1;

/**
  | Limbs of half the secp256k1 order.
  |
  */
#[cfg(feature="widemul-int64")] pub const N_H_0: u32 = 0x681B20A0;
#[cfg(feature="widemul-int64")] pub const N_H_1: u32 = 0xDFE92F46;
#[cfg(feature="widemul-int64")] pub const N_H_2: u32 = 0x57A4501D;
#[cfg(feature="widemul-int64")] pub const N_H_3: u32 = 0x5D576E73;
#[cfg(feature="widemul-int64")] pub const N_H_4: u32 = 0xFFFFFFFF;
#[cfg(feature="widemul-int64")] pub const N_H_5: u32 = 0xFFFFFFFF;
#[cfg(feature="widemul-int64")] pub const N_H_6: u32 = 0xFFFFFFFF;
#[cfg(feature="widemul-int64")] pub const N_H_7: u32 = 0x7FFFFFFF;

#[cfg(test)]
mod scalar_limb_constants_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn group_order_limbs_match_known_order_bytes() {
        info!("validating N_* limbs match secp256k1 scalar order");

        #[cfg(feature = "widemul-int128")]
        {
            let mut be = [0u8; 32];
            be[0..8].copy_from_slice(&N_3.to_be_bytes());
            be[8..16].copy_from_slice(&N_2.to_be_bytes());
            be[16..24].copy_from_slice(&N_1.to_be_bytes());
            be[24..32].copy_from_slice(&N_0.to_be_bytes());
            debug!(?be, "order bytes reconstructed from 4x64 limbs");
            assert_eq!(be, SECP256K1_ORDER_BE);
        }

        #[cfg(feature = "widemul-int64")]
        {
            let mut be = [0u8; 32];
            be[0..4].copy_from_slice(&N_7.to_be_bytes());
            be[4..8].copy_from_slice(&N_6.to_be_bytes());
            be[8..12].copy_from_slice(&N_5.to_be_bytes());
            be[12..16].copy_from_slice(&N_4.to_be_bytes());
            be[16..20].copy_from_slice(&N_3.to_be_bytes());
            be[20..24].copy_from_slice(&N_2.to_be_bytes());
            be[24..28].copy_from_slice(&N_1.to_be_bytes());
            be[28..32].copy_from_slice(&N_0.to_be_bytes());
            debug!(?be, "order bytes reconstructed from 8x32 limbs");
            assert_eq!(be, SECP256K1_ORDER_BE);
        }
    }

    #[traced_test]
    fn group_order_complement_reconstructs_2_to_256() {
        info!("validating N_C_* limbs represent 2^256 - n by checking (n + N_C) == 0 with carry");

        #[cfg(feature = "widemul-int128")]
        {
            let mut nc_be = [0u8; 32];
            nc_be[0..8].copy_from_slice(&0u64.to_be_bytes());
            nc_be[8..16].copy_from_slice(&N_C_2.to_be_bytes());
            nc_be[16..24].copy_from_slice(&N_C_1.to_be_bytes());
            nc_be[24..32].copy_from_slice(&N_C_0.to_be_bytes());

            let (sum, carry) = be_add_32(&SECP256K1_ORDER_BE, &nc_be);
            debug!(?nc_be, "2^256 - n bytes from limbs");
            debug!(?sum, carry, "n + (2^256 - n)");
            assert_eq!(sum, [0u8; 32]);
            assert_eq!(carry, 1u8);
        }

        #[cfg(feature = "widemul-int64")]
        {
            let mut nc_be = [0u8; 32];
            nc_be[0..4].copy_from_slice(&0u32.to_be_bytes());
            nc_be[4..8].copy_from_slice(&0u32.to_be_bytes());
            nc_be[8..12].copy_from_slice(&0u32.to_be_bytes());
            nc_be[12..16].copy_from_slice(&N_C_4.to_be_bytes());
            nc_be[16..20].copy_from_slice(&N_C_3.to_be_bytes());
            nc_be[20..24].copy_from_slice(&N_C_2.to_be_bytes());
            nc_be[24..28].copy_from_slice(&N_C_1.to_be_bytes());
            nc_be[28..32].copy_from_slice(&N_C_0.to_be_bytes());

            let (sum, carry) = be_add_32(&SECP256K1_ORDER_BE, &nc_be);
            debug!(?nc_be, "2^256 - n bytes from limbs");
            debug!(?sum, carry, "n + (2^256 - n)");
            assert_eq!(sum, [0u8; 32]);
            assert_eq!(carry, 1u8);
        }
    }

    #[traced_test]
    fn half_order_limbs_match_right_shifted_order() {
        info!("validating N_H_* limbs match n >> 1");

        #[cfg(feature = "widemul-int128")]
        {
            let mut nh_be = [0u8; 32];
            nh_be[0..8].copy_from_slice(&N_H_3.to_be_bytes());
            nh_be[8..16].copy_from_slice(&N_H_2.to_be_bytes());
            nh_be[16..24].copy_from_slice(&N_H_1.to_be_bytes());
            nh_be[24..32].copy_from_slice(&N_H_0.to_be_bytes());

            let shifted = be_shr1_256(&SECP256K1_ORDER_BE);
            debug!(?nh_be, "n/2 bytes from limbs (4x64)");
            debug!(?shifted, "n >> 1 bytes");
            assert_eq!(nh_be, SECP256K1_ORDER_HALF_BE);
            assert_eq!(shifted, SECP256K1_ORDER_HALF_BE);
        }

        #[cfg(feature = "widemul-int64")]
        {
            let mut nh_be = [0u8; 32];
            nh_be[0..4].copy_from_slice(&N_H_7.to_be_bytes());
            nh_be[4..8].copy_from_slice(&N_H_6.to_be_bytes());
            nh_be[8..12].copy_from_slice(&N_H_5.to_be_bytes());
            nh_be[12..16].copy_from_slice(&N_H_4.to_be_bytes());
            nh_be[16..20].copy_from_slice(&N_H_3.to_be_bytes());
            nh_be[20..24].copy_from_slice(&N_H_2.to_be_bytes());
            nh_be[24..28].copy_from_slice(&N_H_1.to_be_bytes());
            nh_be[28..32].copy_from_slice(&N_H_0.to_be_bytes());

            let shifted = be_shr1_256(&SECP256K1_ORDER_BE);
            debug!(?nh_be, "n/2 bytes from limbs (8x32)");
            debug!(?shifted, "n >> 1 bytes");
            assert_eq!(nh_be, SECP256K1_ORDER_HALF_BE);
            assert_eq!(shifted, SECP256K1_ORDER_HALF_BE);
        }
    }
}
