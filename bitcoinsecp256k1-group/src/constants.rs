// ---------------- [ File: bitcoinsecp256k1-group/src/constants.rs ]
/*!
  | These exhaustive group test orders
  | and generators are chosen such that:
  | 
  | - The field size is equal to that of secp256k1,
  | so field code is the same.
  | 
  | - The curve equation is of the form y^2=x^3+B
  | for some constant B.
  | 
  | - The subgroup has a generator 2*P, where
  | P.x=1.
  | 
  | - The subgroup has size less than 1000
  | to permit exhaustive testing.
  | 
  | - The subgroup admits an endomorphism
  | of the form lambda*(x,y) == (beta*x,y).
  | 
  | These parameters are generated using
  | sage/gen_exhaustive_groups.sage.
  |
  */
crate::ix!();

#[cfg(all(
    EXHAUSTIVE_TEST_ORDER,
    not(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199"))
))]
compile_error!("No known generator for the specified exhaustive test group order.");

#[cfg(EXHAUSTIVE_TEST_ORDER = "13")]
pub const EXHAUSTIVE_TEST_ORDER_U32: u32 = 13;

#[cfg(EXHAUSTIVE_TEST_ORDER = "199")]
pub const EXHAUSTIVE_TEST_ORDER_U32: u32 = 199;

#[cfg(EXHAUSTIVE_TEST_ORDER = "13")]
#[allow(non_upper_case_globals)]
pub static ge_const_g: Ge = ge_const!(
    0xc3459c3d_u32,
    0x35326167_u32,
    0xcd86cce8_u32,
    0x07a2417f_u32,
    0x5b8bd567_u32,
    0xde8538ee_u32,
    0x0d507b0c_u32,
    0xd128f5bb_u32,
    0x8e467fec_u32,
    0xcd30000a_u32,
    0x6cc1184e_u32,
    0x25d382c2_u32,
    0xa2f4494e_u32,
    0x2fbe9abc_u32,
    0x8b64abac_u32,
    0xd005fb24_u32
);

#[cfg(EXHAUSTIVE_TEST_ORDER = "13")]
#[allow(non_upper_case_globals)]
pub static fe_const_b: Fe = fe_const!(
    0x3d3486b2_u32,
    0x159a9ca5_u32,
    0xc75638be_u32,
    0xb23a69bc_u32,
    0x946a45ab_u32,
    0x24801247_u32,
    0xb4ed2b8e_u32,
    0x26b6a417_u32
);

#[cfg(EXHAUSTIVE_TEST_ORDER = "199")]
#[allow(non_upper_case_globals)]
pub static ge_const_g: Ge = ge_const!(
    0x226e653f_u32,
    0xc8df7744_u32,
    0x9bacbf12_u32,
    0x7d1dcbf9_u32,
    0x87f05b2a_u32,
    0xe7edbd28_u32,
    0x1f564575_u32,
    0xc48dcf18_u32,
    0xa13872c2_u32,
    0xe933bb17_u32,
    0x5d9ffd5b_u32,
    0xb5b6e10c_u32,
    0x57fe3c00_u32,
    0xbaaaa15a_u32,
    0xe003ec3e_u32,
    0x9c269bae_u32
);

#[cfg(EXHAUSTIVE_TEST_ORDER = "199")]
#[allow(non_upper_case_globals)]
pub static fe_const_b: Fe = fe_const!(
    0x2cca28fa_u32,
    0xfc614b80_u32,
    0x2a3db42b_u32,
    0x00ba00b1_u32,
    0xbea8d943_u32,
    0xdace9ab2_u32,
    0x9536daea_u32,
    0x0074defb_u32
);

#[cfg(not(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199")))]
#[allow(non_upper_case_globals)]
pub static ge_const_g: Ge = ge_const!(
    0x79BE667E_u32,
    0xF9DCBBAC_u32,
    0x55A06295_u32,
    0xCE870B07_u32,
    0x029BFCDB_u32,
    0x2DCE28D9_u32,
    0x59F2815B_u32,
    0x16F81798_u32,
    0x483ADA77_u32,
    0x26A3C465_u32,
    0x5DA4FBFC_u32,
    0x0E1108A8_u32,
    0xFD17B448_u32,
    0xA6855419_u32,
    0x9C47D08F_u32,
    0xFB10D4B8_u32
);

#[cfg(not(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199")))]
#[allow(non_upper_case_globals)]
pub static fe_const_b: Fe = fe_const!(0, 0, 0, 0, 0, 0, 0, 7);

#[cfg(test)]
mod constants_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn generator_and_curve_constant_are_self_consistent() {
        tracing::info!("Checking generator (ge_const_g) and curve constant (fe_const_b) invariants.");

        let g_ptr: *const Ge = core::ptr::addr_of!(ge_const_g);
        assert!(ge_is_infinity(g_ptr) == 0);
        assert!(ge_is_valid_var(g_ptr) != 0);

        let b_ptr: *const Fe = core::ptr::addr_of!(fe_const_b);
        assert!(fe_is_zero(b_ptr) == 0);

        tracing::debug!("Checking subgroup predicate behavior for generator.");
        assert!(ge_is_in_correct_subgroup(g_ptr) != 0);

        #[cfg(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199"))]
        unsafe {
            tracing::debug!("EXHAUSTIVE_TEST_ORDER enabled; verifying order * G == infinity via repeated addition.");

            let mut acc: Gej = core::mem::zeroed();
            gej_set_infinity(core::ptr::addr_of_mut!(acc));

            let mut i: u32 = 0;
            while i < EXHAUSTIVE_TEST_ORDER_U32 {
                let mut next: Gej = core::mem::zeroed();
                gej_add_ge_var(
                    core::ptr::addr_of_mut!(next),
                    core::ptr::addr_of!(acc),
                    g_ptr,
                    core::ptr::null_mut(),
                );
                acc = next;
                i += 1;
            }

            assert!(gej_is_infinity(core::ptr::addr_of!(acc)) != 0);
        }
    }

    #[cfg(EXHAUSTIVE_TEST_ORDER = "13")]
    #[traced_test]
    fn exhaustive_order_constant_is_13_when_configured() {
        tracing::info!("Validating EXHAUSTIVE_TEST_ORDER_U32 == 13.");
        assert!(EXHAUSTIVE_TEST_ORDER_U32 == 13);
    }

    #[cfg(EXHAUSTIVE_TEST_ORDER = "199")]
    #[traced_test]
    fn exhaustive_order_constant_is_199_when_configured() {
        tracing::info!("Validating EXHAUSTIVE_TEST_ORDER_U32 == 199.");
        assert!(EXHAUSTIVE_TEST_ORDER_U32 == 199);
    }

    #[cfg(not(any(EXHAUSTIVE_TEST_ORDER = "13", EXHAUSTIVE_TEST_ORDER = "199")))]
    #[traced_test]
    fn secp256k1_b_constant_is_seven_in_non_exhaustive_mode() {
        tracing::info!("Validating fe_const_b == 7 in normal (secp256k1) mode.");

        unsafe {
            let expected: Fe = secp256k1_group_exhaustive_test_support::fe_int(7);
            assert!(
                fe_equal_var(
                    core::ptr::addr_of!(fe_const_b),
                    core::ptr::addr_of!(expected)
                ) != 0
            );
        }
    }
}
