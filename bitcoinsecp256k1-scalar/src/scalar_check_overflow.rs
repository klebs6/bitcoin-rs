// ---------------- [ File: bitcoinsecp256k1-scalar/src/check_overflow.rs ]
crate::ix!();

#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_check_overflow(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            int yes = 0;
        int no = 0;
        no |= (a->d[3] < N_3); /* No need for a > check. */
        no |= (a->d[2] < N_2);
        yes |= (a->d[2] > N_2) & ~no;
        no |= (a->d[1] < N_1);
        yes |= (a->d[1] > N_1) & ~no;
        yes |= (a->d[0] >= N_0) & ~no;
        return yes;
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_check_overflow(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            int yes = 0;
        int no = 0;
        no |= (a->d[7] < N_7); /* No need for a > check. */
        no |= (a->d[6] < N_6); /* No need for a > check. */
        no |= (a->d[5] < N_5); /* No need for a > check. */
        no |= (a->d[4] < N_4);
        yes |= (a->d[4] > N_4) & ~no;
        no |= (a->d[3] < N_3) & ~yes;
        yes |= (a->d[3] > N_3) & ~no;
        no |= (a->d[2] < N_2) & ~yes;
        yes |= (a->d[2] > N_2) & ~no;
        no |= (a->d[1] < N_1) & ~yes;
        yes |= (a->d[1] > N_1) & ~no;
        yes |= (a->d[0] >= N_0) & ~no;
        return yes;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
#[inline] pub fn scalar_check_overflow(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return *a >= EXHAUSTIVE_TEST_ORDER;
        */
}
