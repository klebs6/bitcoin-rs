// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_mod_info.rs ]
crate::ix!();

pub struct ModInv32ModInfo {

    /// The modulus in signed30 notation, must
    /// be odd and in [3, 2^256].
    /// 
    pub(crate) modulus:       ModInv32Signed30,

    /// modulus^{-1} mod 2^30
    /// 
    pub(crate) modulus_inv30: u32,
}

#[cfg(test)]
mod modinv32_mod_info_computation_validation {
    use super::*;

    #[traced_test]
    fn modinfo_includes_correct_inverse_mod_2pow30_for_selected_moduli() {
        let moduli: [u64; 10] = [
            3,
            5,
            7,
            11,
            101,
            257,
            65537,
            1_000_000_007,
            (1u64 << 40) + 15,
            (1u64 << 60) - 93,
        ];

        for &modulus in moduli.iter() {
            let mi = support::modinfo_from_u64(modulus);

            let low = (mi.modulus.v[0] as u64) & support::M30_U64;
            let inv = mi.modulus_inv30 as u64;
            let check = (inv.wrapping_mul(low)) & support::M30_U64;

            tracing::info!(
                modulus,
                low,
                modulus_inv30 = mi.modulus_inv30,
                check,
                "validating modulus_inv30"
            );

            assert!(check == 1);

            let modulus_rt = support::signed30_to_u128_horner(&mi.modulus) as u64;
            assert!(modulus_rt == modulus);
        }
    }

    #[traced_test]
    fn modinfo_layout_is_stable_for_ffi_assumptions() {
        let size_signed30 = core::mem::size_of::<ModInv32Signed30>();
        let size_modinfo = core::mem::size_of::<ModInv32ModInfo>();
        let align_signed30 = core::mem::align_of::<ModInv32Signed30>();
        let align_modinfo = core::mem::align_of::<ModInv32ModInfo>();

        tracing::info!(
            size_signed30,
            size_modinfo,
            align_signed30,
            align_modinfo,
            "checking ModInv32ModInfo layout"
        );

        assert!(size_signed30 == 9 * 4);
        assert!(align_signed30 == 4);
        assert!(size_modinfo >= size_signed30 + 4);
        assert!(align_modinfo >= 4);
    }
}
