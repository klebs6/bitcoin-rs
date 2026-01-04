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
// ---------------- [ File: bitcoinsecp256k1-group/src/constants.rs ]
crate::ix!();

lazy_static!{
    /*
    #if defined(EXHAUSTIVE_TEST_ORDER)
    #  if EXHAUSTIVE_TEST_ORDER == 13
    static const ge ge_const_g = GE_CONST(
        0xc3459c3d, 0x35326167, 0xcd86cce8, 0x07a2417f,
        0x5b8bd567, 0xde8538ee, 0x0d507b0c, 0xd128f5bb,
        0x8e467fec, 0xcd30000a, 0x6cc1184e, 0x25d382c2,
        0xa2f4494e, 0x2fbe9abc, 0x8b64abac, 0xd005fb24
    );
    static const fe fe_const_b = fe_const!(
        0x3d3486b2, 0x159a9ca5, 0xc75638be, 0xb23a69bc,
        0x946a45ab, 0x24801247, 0xb4ed2b8e, 0x26b6a417
    );
    #  elif EXHAUSTIVE_TEST_ORDER == 199
    static const ge ge_const_g = GE_CONST(
        0x226e653f, 0xc8df7744, 0x9bacbf12, 0x7d1dcbf9,
        0x87f05b2a, 0xe7edbd28, 0x1f564575, 0xc48dcf18,
        0xa13872c2, 0xe933bb17, 0x5d9ffd5b, 0xb5b6e10c,
        0x57fe3c00, 0xbaaaa15a, 0xe003ec3e, 0x9c269bae
    );
    static const fe fe_const_b = fe_const!(
        0x2cca28fa, 0xfc614b80, 0x2a3db42b, 0x00ba00b1,
        0xbea8d943, 0xdace9ab2, 0x9536daea, 0x0074defb
    );
    #  else
    #    error No known generator for the specified exhaustive test group order.
    #  endif
    #else
    /** Generator for secp256k1, value 'g' defined in
     *  "Standards for Efficient Cryptography" (SEC2) 2.7.1.
     */
    static const ge ge_const_g = GE_CONST(
        0x79BE667EUL, 0xF9DCBBACUL, 0x55A06295UL, 0xCE870B07UL,
        0x029BFCDBUL, 0x2DCE28D9UL, 0x59F2815BUL, 0x16F81798UL,
        0x483ADA77UL, 0x26A3C465UL, 0x5DA4FBFCUL, 0x0E1108A8UL,
        0xFD17B448UL, 0xA6855419UL, 0x9C47D08FUL, 0xFB10D4B8UL
    );
    #endif
    */
}

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
pub(crate) static ge_const_g: Ge = ge_const!(
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
pub(crate) static fe_const_b: Fe = fe_const!(
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
pub(crate) static ge_const_g: Ge = ge_const!(
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
pub(crate) static fe_const_b: Fe = fe_const!(
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
pub(crate) static ge_const_g: Ge = ge_const!(
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
pub(crate) static fe_const_b: Fe = fe_const!(0, 0, 0, 0, 0, 0, 0, 7);
