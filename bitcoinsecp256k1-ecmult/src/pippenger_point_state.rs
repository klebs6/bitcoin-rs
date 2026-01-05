// ---------------- [ File: bitcoinsecp256k1-ecmult/src/pippenger_point_state.rs ]
crate::ix!();

pub struct PippengerPointState {
    skew_na:   i32,
    input_pos: usize,
}

pub struct PippengerState {
    wnaf_na: *mut i32,
    ps:      *mut PippengerPointState,
}
