// ---------------- [ File: bitcoinsecp256k1-ecmult/src/strauss_scratch_size.rs ]
crate::ix!();

pub fn strauss_scratch_size(n_points: usize) -> usize {
    
    todo!();
        /*
            static const size_t point_size = (2 * sizeof(ge) + sizeof(gej) + sizeof(fe)) * ECMULT_TABLE_SIZE(WINDOW_A) + sizeof(struct strauss_point_state) + sizeof(gej) + sizeof(scalar);
        return n_points*point_size;
        */
}
