// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_trans2x2.rs ]
crate::ix!();

/// Data type for transition matrices (see section 3 of explanation).
/// 
/// t = [ u  v ]
///     [ q  r ]
/// 
pub struct ModInv32Trans2x2 {
    pub(crate) u: i32,
    pub(crate) v: i32,
    pub(crate) q: i32,
    pub(crate) r: i32,
}

#[cfg(test)]
mod modinv32_trans2x2_structure_validation {
    use super::*;

    #[traced_test]
    fn trans2x2_layout_matches_expected_size_and_alignment() {
        let size = core::mem::size_of::<ModInv32Trans2x2>();
        let align = core::mem::align_of::<ModInv32Trans2x2>();

        tracing::info!(size, align, "checking ModInv32Trans2x2 layout");

        assert!(size == 4 * 4);
        assert!(align == 4);
    }

    #[traced_test]
    fn trans2x2_determinant_formula_behaves_as_expected_on_small_matrices() {
        let cases: [(i32, i32, i32, i32, i128); 4] = [
            (1, 0, 0, 1, 1),
            (0, 1, 1, 0, -1),
            (2, 3, 5, 7, (2i128 * 7i128) - (3i128 * 5i128)),
            (-1, 2, 3, -4, ((-1i128) * (-4i128)) - (2i128 * 3i128)),
        ];

        for &(u, v, q, r, expected) in cases.iter() {
            let t = ModInv32Trans2x2 { u, v, q, r };
            let det = (t.u as i128) * (t.r as i128) - (t.v as i128) * (t.q as i128);

            tracing::debug!(u, v, q, r, det, expected, "determinant check");

            assert!(det == expected);
        }
    }
}
