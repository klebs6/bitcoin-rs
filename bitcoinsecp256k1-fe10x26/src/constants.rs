// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/constants.rs ]
crate::ix!();

lazy_static!{
    pub static ref const_modinfo_fe: ModInv32ModInfo = ModInv32ModInfo {
        modulus: ModInv32Signed30 { v: [-0x3D1i32, -4i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 65536i32] },
        modulus_inv30: 0x2DDACACFu32,
    };
}

#[cfg(test)]
mod modinv32_constant_contract_suite {
    use super::*;

    #[traced_test]
    fn modinv32_modulus_matches_secp256k1_field_prime_structure() {
        info!("validating const_modinfo_fe.modulus signed30 limb layout encodes p = 2^256 - 2^32 - 977");
        let m = &(&*const_modinfo_fe).modulus.v;

        debug!(?m, "signed30 modulus limbs");
        assert_eq!(m[0], -0x3D1i32);
        assert_eq!(m[1], -4i32);
        assert_eq!(m[2], 0i32);
        assert_eq!(m[3], 0i32);
        assert_eq!(m[4], 0i32);
        assert_eq!(m[5], 0i32);
        assert_eq!(m[6], 0i32);
        assert_eq!(m[7], 0i32);
        assert_eq!(m[8], 65536i32);

        trace!("top limb should correspond to 2^256 via 65536 * 2^(30*8) = 2^16 * 2^240");
        assert_eq!(m[8] as u32, 1u32 << 16);
    }

    #[traced_test]
    fn modinv32_modulus_inv30_is_multiplicative_inverse_mod_2pow30() {
        info!("checking modulus_inv30 satisfies (modulus mod 2^30) * inv30 == 1 (mod 2^30)");
        let inv30 = (&*const_modinfo_fe).modulus_inv30 as u64;

        let m0 = ((1u64 << 30) - 0x3D1u64) & ((1u64 << 30) - 1u64);
        let prod = (m0.wrapping_mul(inv30)) & ((1u64 << 30) - 1u64);

        debug!(m0, inv30, prod, "m0 * inv30 mod 2^30");
        assert_eq!(prod, 1u64);
    }
}
