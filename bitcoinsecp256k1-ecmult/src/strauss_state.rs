// ---------------- [ File: bitcoinsecp256k1-ecmult/src/strauss_state.rs ]
crate::ix!();

pub struct StraussPointState {
    na_1:        Scalar,
    na_lam:      Scalar,
    wnaf_na_1:   [i32; 129],
    wnaf_na_lam: [i32; 129],
    bits_na_1:   i32,
    bits_na_lam: i32,
    input_pos:   usize,
}

pub struct StraussState {
    prej:      *mut Gej,
    zr:        *mut Fe,
    pre_a:     *mut Ge,
    pre_a_lam: *mut Ge,
    ps:        *mut StraussPointState,
}
