// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_is_high.rs ]
crate::ix!();

/// Check whether a scalar is higher than
/// the group order divided by 2.
/// 
#[cfg(WIDEMUL_INT128)]
pub fn scalar_is_high(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            int yes = 0;
        int no = 0;
        no |= (a->d[3] < N_H_3);
        yes |= (a->d[3] > N_H_3) & ~no;
        no |= (a->d[2] < N_H_2) & ~yes; /* No need for a > check. */
        no |= (a->d[1] < N_H_1) & ~yes;
        yes |= (a->d[1] > N_H_1) & ~no;
        yes |= (a->d[0] > N_H_0) & ~no;
        return yes;
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_is_high(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            int yes = 0;
        int no = 0;
        no |= (a->d[7] < N_H_7);
        yes |= (a->d[7] > N_H_7) & ~no;
        no |= (a->d[6] < N_H_6) & ~yes; /* No need for a > check. */
        no |= (a->d[5] < N_H_5) & ~yes; /* No need for a > check. */
        no |= (a->d[4] < N_H_4) & ~yes; /* No need for a > check. */
        no |= (a->d[3] < N_H_3) & ~yes;
        yes |= (a->d[3] > N_H_3) & ~no;
        no |= (a->d[2] < N_H_2) & ~yes;
        yes |= (a->d[2] > N_H_2) & ~no;
        no |= (a->d[1] < N_H_1) & ~yes;
        yes |= (a->d[1] > N_H_1) & ~no;
        yes |= (a->d[0] > N_H_0) & ~no;
        return yes;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_is_high(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return *a > EXHAUSTIVE_TEST_ORDER / 2;
        */
}
