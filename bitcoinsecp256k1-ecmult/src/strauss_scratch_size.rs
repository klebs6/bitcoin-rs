// ---------------- [ File: bitcoinsecp256k1-ecmult/src/strauss_scratch_size.rs ]
crate::ix!();

pub fn strauss_scratch_size(n_points: usize) -> usize {
    trace!(target: "secp256k1::ecmult", n_points = n_points, "strauss_scratch_size");

    let point_size: usize = (2usize * size_of::<Ge>()
        + size_of::<Gej>()
        + size_of::<Fe>())
        * ecmult_table_size!(WINDOW_A)
        + size_of::<StraussPointState>()
        + size_of::<Gej>()
        + size_of::<Scalar>();
    n_points * point_size
}
