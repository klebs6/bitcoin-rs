// ---------------- [ File: bitcoinsecp256k1-scalar/src/constants.rs ]
crate::ix!();

pub const SCALAR_ONE:  Scalar = scalar_const!(0, 0, 0, 0, 0, 0, 0, 1);
pub const SCALAR_ZERO: Scalar = scalar_const!(0, 0, 0, 0, 0, 0, 0, 0);

/**
  | The curve has an endomorphism, where
  | lambda * (x, y) = (beta * x, y), where lambda
  | is:
  |
  */
#[cfg(not(EXHAUSTIVE_TEST_ORDER))]
lazy_static!{
    /*
    static const scalar const_lambda = SCALAR_CONST(
        0x5363AD4CUL, 0xC05C30E0UL, 0xA5261C02UL, 0x8812645AUL,
        0x122E22EAUL, 0x20816678UL, 0xDF02967CUL, 0x1B23BD72UL
    );
    */
}

#[cfg(WIDEMUL_INT128)]
lazy_static!{
    /*
    static const modinv64_modinfo const_modinfo_scalar = {
        {{0x3FD25E8CD0364141LL, 0x2ABB739ABD2280EELL, -0x15LL, 0, 256}},
        0x34F20099AA774EC1LL
    };
    */
}

#[cfg(WIDEMUL_INT64)]
lazy_static!{
    /*
    static const modinv32_modinfo const_modinfo_scalar = {
        {{0x10364141L, 0x3F497A33L, 0x348A03BBL, 0x2BB739ABL, -0x146L, 0, 0, 0, 65536}},
        0x2A774EC1L
    };
    */
}
