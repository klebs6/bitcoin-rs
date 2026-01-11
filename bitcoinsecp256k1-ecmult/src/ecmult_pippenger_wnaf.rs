// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_pippenger_wnaf.rs ]
crate::ix!();

/// pippenger_wnaf computes the result of a multi-point multiplication as follows:
///
/// The scalars are brought into wnaf with n_wnaf elements each.
///
/// Then for every i < n_wnaf, first each point is added to a "bucket" corresponding to the point's
/// wnaf[i].
///
/// Second, the buckets are added together such that r += 1*bucket[0] + 3*bucket[1] + 5*bucket[2]
/// + ...
///
pub fn ecmult_pippenger_wnaf(
    buckets:       *mut Gej,
    bucket_window: i32,
    state:         *mut PippengerState,
    r:             *mut Gej,
    sc:            *const Scalar,
    pt:            *const Ge,
    num:           usize,
) -> i32 {
    tracing::trace!(
        target: "secp256k1::ecmult",
        num = num,
        bucket_window = bucket_window,
        "ecmult_pippenger_wnaf"
    );

    unsafe {
        let n_wnaf: usize = wnaf_size!(bucket_window + 1);
        let mut np: usize;
        let mut no: usize = 0;
        let mut i: i32;
        let mut j: i32;

        np = 0;
        while np < num {
            if scalar_is_zero(sc.add(np)) != 0 || ge_is_infinity(pt.add(np)) != 0 {
                np += 1;
                continue;
            }
            (*(*state).ps.add(no)).input_pos = np;
            (*(*state).ps.add(no)).skew_na = wnaf_fixed(
                (*state).wnaf_na.add(no * n_wnaf),
                sc.add(np),
                bucket_window + 1,
            );
            no += 1;
            np += 1;
        }
        gej_set_infinity(r);

        if no == 0 {
            return 1;
        }

        i = (n_wnaf as i32) - 1;
        while i >= 0 {
            let mut running_sum = Gej::new();
            let table_size: usize = ecmult_table_size!(bucket_window + 2);

            j = 0;
            while (j as usize) < table_size {
                gej_set_infinity(buckets.add(j as usize));
                j += 1;
            }

            np = 0;
            while np < no {
                let n: i32 = *(*state).wnaf_na.add(np * n_wnaf + (i as usize));
                let point_state: PippengerPointState = core::ptr::read((*state).ps.add(np));
                let mut tmp = Ge::new();
                let idx: i32;

                if i == 0 {
                    /* correct for wnaf skew */
                    let skew: i32 = point_state.skew_na;
                    if skew != 0 {
                        ge_neg(core::ptr::addr_of_mut!(tmp), pt.add(point_state.input_pos));
                        gej_add_ge_var(
                            buckets.add(0),
                            buckets.add(0),
                            core::ptr::addr_of!(tmp),
                            core::ptr::null_mut(),
                        );
                    }
                }
                if n > 0 {
                    idx = (n - 1) / 2;
                    gej_add_ge_var(
                        buckets.add(idx as usize),
                        buckets.add(idx as usize),
                        pt.add(point_state.input_pos),
                        core::ptr::null_mut(),
                    );
                } else if n < 0 {
                    idx = -((n + 1) / 2);
                    ge_neg(core::ptr::addr_of_mut!(tmp), pt.add(point_state.input_pos));
                    gej_add_ge_var(
                        buckets.add(idx as usize),
                        buckets.add(idx as usize),
                        core::ptr::addr_of!(tmp),
                        core::ptr::null_mut(),
                    );
                }

                np += 1;
            }

            j = 0;
            while j < bucket_window {
                gej_double_var(r, r, core::ptr::null_mut());
                j += 1;
            }

            gej_set_infinity(core::ptr::addr_of_mut!(running_sum));
            /* Accumulate the sum: bucket[0] + 3*bucket[1] + 5*bucket[2] + 7*bucket[3] + ...
             *                   = bucket[0] +   bucket[1] +   bucket[2] +   bucket[3] + ...
             *                   +         2 *  (bucket[1] + 2*bucket[2] + 3*bucket[3] + ...)
             * using an intermediate running sum:
             * running_sum = bucket[0] +   bucket[1] +   bucket[2] + ...
             *
             * The doubling is done implicitly by deferring the final window doubling (of 'r').
             */
            j = (table_size as i32) - 1;
            while j > 0 {
                gej_add_var(
                    core::ptr::addr_of_mut!(running_sum),
                    core::ptr::addr_of!(running_sum),
                    buckets.add(j as usize),
                    core::ptr::null_mut(),
                );
                gej_add_var(
                    r,
                    r,
                    core::ptr::addr_of!(running_sum),
                    core::ptr::null_mut(),
                );
                j -= 1;
            }

            gej_add_var(
                core::ptr::addr_of_mut!(running_sum),
                core::ptr::addr_of!(running_sum),
                buckets.add(0),
                core::ptr::null_mut(),
            );
            gej_double_var(r, r, core::ptr::null_mut());
            gej_add_var(
                r,
                r,
                core::ptr::addr_of!(running_sum),
                core::ptr::null_mut(),
            );

            i -= 1;
        }
        1
    }
        /*
            size_t n_wnaf = WNAF_SIZE(bucket_window+1);
        size_t np;
        size_t no = 0;
        int i;
        int j;

        for (np = 0; np < num; ++np) {
            if (scalar_is_zero(&sc[np]) || ge_is_infinity(&pt[np])) {
                continue;
            }
            state->ps[no].input_pos = np;
            state->ps[no].skew_na = wnaf_fixed(&state->wnaf_na[no*n_wnaf], &sc[np], bucket_window+1);
            no++;
        }
        gej_set_infinity(r);

        if (no == 0) {
            return 1;
        }

        for (i = n_wnaf - 1; i >= 0; i--) {
            gej running_sum;

            for(j = 0; j < ECMULT_TABLE_SIZE(bucket_window+2); j++) {
                gej_set_infinity(&buckets[j]);
            }

            for (np = 0; np < no; ++np) {
                int n = state->wnaf_na[np*n_wnaf + i];
                struct pippenger_point_state point_state = state->ps[np];
                ge tmp;
                int idx;

                if (i == 0) {
                    /* correct for wnaf skew */
                    int skew = point_state.skew_na;
                    if (skew) {
                        ge_neg(&tmp, &pt[point_state.input_pos]);
                        gej_add_ge_var(&buckets[0], &buckets[0], &tmp, NULL);
                    }
                }
                if (n > 0) {
                    idx = (n - 1)/2;
                    gej_add_ge_var(&buckets[idx], &buckets[idx], &pt[point_state.input_pos], NULL);
                } else if (n < 0) {
                    idx = -(n + 1)/2;
                    ge_neg(&tmp, &pt[point_state.input_pos]);
                    gej_add_ge_var(&buckets[idx], &buckets[idx], &tmp, NULL);
                }
            }

            for(j = 0; j < bucket_window; j++) {
                gej_double_var(r, r, NULL);
            }

            gej_set_infinity(&running_sum);
            /* Accumulate the sum: bucket[0] + 3*bucket[1] + 5*bucket[2] + 7*bucket[3] + ...
             *                   = bucket[0] +   bucket[1] +   bucket[2] +   bucket[3] + ...
             *                   +         2 *  (bucket[1] + 2*bucket[2] + 3*bucket[3] + ...)
             * using an intermediate running sum:
             * running_sum = bucket[0] +   bucket[1] +   bucket[2] + ...
             *
             * The doubling is done implicitly by deferring the final window doubling (of 'r').
             */
            for(j = ECMULT_TABLE_SIZE(bucket_window+2) - 1; j > 0; j--) {
                gej_add_var(&running_sum, &running_sum, &buckets[j], NULL);
                gej_add_var(r, r, &running_sum, NULL);
            }

            gej_add_var(&running_sum, &running_sum, &buckets[0], NULL);
            gej_double_var(r, r, NULL);
            gej_add_var(r, r, &running_sum, NULL);
        }
        return 1;
        */

}
pub fn ecmult_pippenger_wnaf(
    buckets:       *mut Gej,
    bucket_window: i32,
    state:         *mut PippengerState,
    r:             *mut Gej,
    sc:            *const Scalar,
    pt:            *const Ge,
    num:           usize,
) -> i32 {
    tracing::trace!(
        target: "secp256k1::ecmult",
        num = num,
        bucket_window = bucket_window,
        "ecmult_pippenger_wnaf"
    );

    unsafe {
        let n_wnaf: usize = wnaf_size!(bucket_window + 1);
        let mut np: usize;
        let mut no: usize = 0;
        let mut i: i32;
        let mut j: i32;

        let ps: *mut PippengerPointState = PippengerState::ps_ptr(state as *const PippengerState);
        let wnaf_na: *mut i32 = PippengerState::wnaf_na_ptr(state as *const PippengerState);

        np = 0;
        while np < num {
            if scalar_is_zero(sc.add(np)) != 0 || ge_is_infinity(pt.add(np)) != 0 {
                np += 1;
                continue;
            }
            let skew = wnaf_fixed(wnaf_na.add(no * n_wnaf), sc.add(np), bucket_window + 1);
            core::ptr::write(ps.add(no), PippengerPointState::new(skew, np));
            no += 1;
            np += 1;
        }
        gej_set_infinity(r);

        if no == 0 {
            return 1;
        }

        i = (n_wnaf as i32) - 1;
        while i >= 0 {
            let mut running_sum = Gej::new();
            let table_size: usize = ecmult_table_size!(bucket_window + 2);

            j = 0;
            while (j as usize) < table_size {
                gej_set_infinity(buckets.add(j as usize));
                j += 1;
            }

            np = 0;
            while np < no {
                let n: i32 = *wnaf_na.add(np * n_wnaf + (i as usize));
                let point_state: PippengerPointState = core::ptr::read(ps.add(np));
                let input_pos: usize = *point_state.input_pos();
                let mut tmp = Ge::new();
                let idx: i32;

                if i == 0 {
                    /* correct for wnaf skew */
                    let skew: i32 = *point_state.skew_na();
                    if skew != 0 {
                        ge_neg(core::ptr::addr_of_mut!(tmp), pt.add(input_pos));
                        gej_add_ge_var(
                            buckets.add(0),
                            buckets.add(0),
                            core::ptr::addr_of!(tmp),
                            core::ptr::null_mut(),
                        );
                    }
                }
                if n > 0 {
                    idx = (n - 1) / 2;
                    gej_add_ge_var(
                        buckets.add(idx as usize),
                        buckets.add(idx as usize),
                        pt.add(input_pos),
                        core::ptr::null_mut(),
                    );
                } else if n < 0 {
                    idx = -((n + 1) / 2);
                    ge_neg(core::ptr::addr_of_mut!(tmp), pt.add(input_pos));
                    gej_add_ge_var(
                        buckets.add(idx as usize),
                        buckets.add(idx as usize),
                        core::ptr::addr_of!(tmp),
                        core::ptr::null_mut(),
                    );
                }

                np += 1;
            }

            j = 0;
            while j < bucket_window {
                gej_double_var(r, r, core::ptr::null_mut());
                j += 1;
            }

            gej_set_infinity(core::ptr::addr_of_mut!(running_sum));
            /* Accumulate the sum: bucket[0] + 3*bucket[1] + 5*bucket[2] + 7*bucket[3] + ...
             *                   = bucket[0] +   bucket[1] +   bucket[2] +   bucket[3] + ...
             *                   +         2 *  (bucket[1] + 2*bucket[2] + 3*bucket[3] + ...)
             * using an intermediate running sum:
             * running_sum = bucket[0] +   bucket[1] +   bucket[2] + ...
             *
             * The doubling is done implicitly by deferring the final window doubling (of 'r').
             */
            j = (table_size as i32) - 1;
            while j > 0 {
                gej_add_var(
                    core::ptr::addr_of_mut!(running_sum),
                    core::ptr::addr_of!(running_sum),
                    buckets.add(j as usize),
                    core::ptr::null_mut(),
                );
                gej_add_var(r, r, core::ptr::addr_of!(running_sum), core::ptr::null_mut());
                j -= 1;
            }

            gej_add_var(
                core::ptr::addr_of_mut!(running_sum),
                core::ptr::addr_of!(running_sum),
                buckets.add(0),
                core::ptr::null_mut(),
            );
            gej_double_var(r, r, core::ptr::null_mut());
            gej_add_var(r, r, core::ptr::addr_of!(running_sum), core::ptr::null_mut());

            i -= 1;
        }
        1
    }
}
