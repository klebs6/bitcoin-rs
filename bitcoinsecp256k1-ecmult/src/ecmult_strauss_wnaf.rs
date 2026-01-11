// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_strauss_wnaf.rs ]
crate::ix!();

pub fn ecmult_strauss_wnaf(
    ctx:   *const EcMultContext,
    state: *const StraussState,
    r:     *mut Gej,
    num:   usize,
    a:     *const Gej,
    na:    *const Scalar,
    ng:    *const Scalar,
) {
    tracing::trace!(
        target: "secp256k1::ecmult",
        num = num,
        ng_is_null = ng.is_null(),
        "ecmult_strauss_wnaf"
    );

    unsafe {
        let mut tmpa = Ge::new();
        let mut Z: Fe = core::mem::MaybeUninit::<Fe>::uninit().assume_init();
        /* Splitted G factors. */
        let mut ng_1 = Scalar::new();
        let mut ng_128 = Scalar::new();
        let mut wnaf_ng_1: [i32; 129] = [0; 129];
        let mut bits_ng_1: i32 = 0;
        let mut wnaf_ng_128: [i32; 129] = [0; 129];
        let mut bits_ng_128: i32 = 0;
        let mut i: i32;
        let mut bits: i32 = 0;
        let mut np: usize;
        let mut no: usize = 0;

        np = 0;
        while np < num {
            if scalar_is_zero(na.add(np)) != 0 || gej_is_infinity(a.add(np)) != 0 {
                np += 1;
                continue;
            }
            (*(*state).ps.add(no)).input_pos = np;
            /* split na into na_1 and na_lam (where na = na_1 + na_lam*lambda, and na_1 and na_lam are ~128 bit) */
            scalar_split_lambda(
                core::ptr::addr_of_mut!((*(*state).ps.add(no)).na_1),
                core::ptr::addr_of_mut!((*(*state).ps.add(no)).na_lam),
                na.add(np),
            );

            /* build wnaf representation for na_1 and na_lam. */
            (*(*state).ps.add(no)).bits_na_1 = ecmult_wnaf(
                (*(*state).ps.add(no)).wnaf_na_1.as_mut_ptr(),
                129,
                core::ptr::addr_of!((*(*state).ps.add(no)).na_1),
                WINDOW_A as i32,
            );
            (*(*state).ps.add(no)).bits_na_lam = ecmult_wnaf(
                (*(*state).ps.add(no)).wnaf_na_lam.as_mut_ptr(),
                129,
                core::ptr::addr_of!((*(*state).ps.add(no)).na_lam),
                WINDOW_A as i32,
            );
            verify_check!((*(*state).ps.add(no)).bits_na_1 <= 129);
            verify_check!((*(*state).ps.add(no)).bits_na_lam <= 129);
            if (*(*state).ps.add(no)).bits_na_1 > bits {
                bits = (*(*state).ps.add(no)).bits_na_1;
            }
            if (*(*state).ps.add(no)).bits_na_lam > bits {
                bits = (*(*state).ps.add(no)).bits_na_lam;
            }
            no += 1;
            np += 1;
        }

        /* Calculate odd multiples of a.
         * All multiples are brought to the same Z 'denominator', which is stored
         * in Z. Due to secp256k1' isomorphism we can do all operations pretending
         * that the Z coordinate was 1, use affine addition formulae, and correct
         * the Z coordinate of the result once at the end.
         * The exception is the precomputed G table points, which are actually
         * affine. Compared to the base used for other points, they have a Z ratio
         * of 1/Z, so we can use gej_add_zinv_var, which uses the same
         * isomorphism to efficiently add with a known Z inverse.
         */
        if no > 0 {
            /* Compute the odd multiples in Jacobian form. */
            ecmult_odd_multiples_table(
                ecmult_table_size!(WINDOW_A) as i32,
                (*state).prej,
                (*state).zr,
                a.add((*(*state).ps.add(0)).input_pos),
            );
            np = 1;
            while np < no {
                let mut tmp: Gej = core::ptr::read(a.add((*(*state).ps.add(np)).input_pos));
                #[cfg(VERIFY)]
                {
                    fe_normalize_var(
                        gej_z_mut(
                            (*state).prej.add(
                                (np - 1) * ecmult_table_size!(WINDOW_A) + (ecmult_table_size!(WINDOW_A) - 1),
                            )
                        )
                    );
                }
                gej_rescale(
                    core::ptr::addr_of_mut!(tmp),
                    gej_z(
                        (*state).prej.add(
                            (np - 1) * ecmult_table_size!(WINDOW_A) + (ecmult_table_size!(WINDOW_A) - 1),
                        )
                    ),
                );
                ecmult_odd_multiples_table(
                    ecmult_table_size!(WINDOW_A) as i32,
                    (*state).prej.add(np * ecmult_table_size!(WINDOW_A)),
                    (*state).zr.add(np * ecmult_table_size!(WINDOW_A)),
                    core::ptr::addr_of!(tmp),
                );
                fe_mul(
                    (*state).zr.add(np * ecmult_table_size!(WINDOW_A)),
                    (*state).zr.add(np * ecmult_table_size!(WINDOW_A)),
                    gej_z(a.add((*(*state).ps.add(np)).input_pos)),
                );
                np += 1;
            }
            /* Bring them to the same Z denominator. */
            ge_globalz_set_table_gej(
                ecmult_table_size!(WINDOW_A) * no,
                (*state).pre_a,
                core::ptr::addr_of_mut!(Z),
                (*state).prej,
                (*state).zr,
            );
        } else {
            fe_set_int(core::ptr::addr_of_mut!(Z), 1);
        }

        np = 0;
        while np < no {
            i = 0;
            while (i as usize) < ecmult_table_size!(WINDOW_A) {
                ge_mul_lambda(
                    (*state).pre_a_lam.add(np * ecmult_table_size!(WINDOW_A) + (i as usize)),
                    (*state).pre_a.add(np * ecmult_table_size!(WINDOW_A) + (i as usize)),
                );
                i += 1;
            }
            np += 1;
        }

        if !ng.is_null() {
            /* split ng into ng_1 and ng_128 (where gn = gn_1 + gn_128*2^128, and gn_1 and gn_128 are ~128 bit) */
            scalar_split_128(core::ptr::addr_of_mut!(ng_1), core::ptr::addr_of_mut!(ng_128), ng);

            /* Build wnaf representation for ng_1 and ng_128 */
            bits_ng_1 = ecmult_wnaf(
                wnaf_ng_1.as_mut_ptr(),
                129,
                core::ptr::addr_of!(ng_1),
                WINDOW_G as i32,
            );
            bits_ng_128 = ecmult_wnaf(
                wnaf_ng_128.as_mut_ptr(),
                129,
                core::ptr::addr_of!(ng_128),
                WINDOW_G as i32,
            );
            if bits_ng_1 > bits {
                bits = bits_ng_1;
            }
            if bits_ng_128 > bits {
                bits = bits_ng_128;
            }
        }

        gej_set_infinity(r);

        i = bits - 1;
        while i >= 0 {
            let n: i32;
            gej_double_var(r, r, core::ptr::null_mut());
            np = 0;
            while np < no {
                if i < (*(*state).ps.add(np)).bits_na_1
                    && {
                        n = (*(*state).ps.add(np)).wnaf_na_1[i as usize];
                        n != 0
                    }
                {
                    ecmult_table_get_ge!(
                        core::ptr::addr_of_mut!(tmpa),
                        (*state).pre_a.add(np * ecmult_table_size!(WINDOW_A)),
                        n,
                        WINDOW_A
                    );
                    gej_add_ge_var(r, r, core::ptr::addr_of!(tmpa), core::ptr::null_mut());
                }
                if i < (*(*state).ps.add(np)).bits_na_lam
                    && {
                        n = (*(*state).ps.add(np)).wnaf_na_lam[i as usize];
                        n != 0
                    }
                {
                    ecmult_table_get_ge!(
                        core::ptr::addr_of_mut!(tmpa),
                        (*state).pre_a_lam.add(np * ecmult_table_size!(WINDOW_A)),
                        n,
                        WINDOW_A
                    );
                    gej_add_ge_var(r, r, core::ptr::addr_of!(tmpa), core::ptr::null_mut());
                }
                np += 1;
            }
            if i < bits_ng_1 && {
                n = wnaf_ng_1[i as usize];
                n != 0
            } {
                ecmult_table_get_ge_storage!(
                    core::ptr::addr_of_mut!(tmpa),
                    (*ctx).pre_g,
                    n,
                    WINDOW_G
                );
                gej_add_zinv_var(r, r, core::ptr::addr_of!(tmpa), core::ptr::addr_of!(Z));
            }
            if i < bits_ng_128 && {
                n = wnaf_ng_128[i as usize];
                n != 0
            } {
                ecmult_table_get_ge_storage!(
                    core::ptr::addr_of_mut!(tmpa),
                    (*ctx).pre_g_128,
                    n,
                    WINDOW_G
                );
                gej_add_zinv_var(r, r, core::ptr::addr_of!(tmpa), core::ptr::addr_of!(Z));
            }

            i -= 1;
        }

        if gej_is_infinity(r) == 0 {
            fe_mul(gej_z_mut(r), gej_z(r), core::ptr::addr_of!(Z));
        }
    }
        /*
        ge tmpa;
        fe Z;
        /* Splitted G factors. */
        scalar ng_1, ng_128;
        int wnaf_ng_1[129];
        int bits_ng_1 = 0;
        int wnaf_ng_128[129];
        int bits_ng_128 = 0;
        int i;
        int bits = 0;
        size_t np;
        size_t no = 0;

        for (np = 0; np < num; ++np) {
            if (scalar_is_zero(&na[np]) || gej_is_infinity(&a[np])) {
                continue;
            }
            state->ps[no].input_pos = np;
            /* split na into na_1 and na_lam (where na = na_1 + na_lam*lambda, and na_1 and na_lam are ~128 bit) */
            scalar_split_lambda(&state->ps[no].na_1, &state->ps[no].na_lam, &na[np]);

            /* build wnaf representation for na_1 and na_lam. */
            state->ps[no].bits_na_1   = ecmult_wnaf(state->ps[no].wnaf_na_1,   129, &state->ps[no].na_1,   WINDOW_A);
            state->ps[no].bits_na_lam = ecmult_wnaf(state->ps[no].wnaf_na_lam, 129, &state->ps[no].na_lam, WINDOW_A);
            VERIFY_CHECK(state->ps[no].bits_na_1 <= 129);
            VERIFY_CHECK(state->ps[no].bits_na_lam <= 129);
            if (state->ps[no].bits_na_1 > bits) {
                bits = state->ps[no].bits_na_1;
            }
            if (state->ps[no].bits_na_lam > bits) {
                bits = state->ps[no].bits_na_lam;
            }
            ++no;
        }

        /* Calculate odd multiples of a.
         * All multiples are brought to the same Z 'denominator', which is stored
         * in Z. Due to secp256k1' isomorphism we can do all operations pretending
         * that the Z coordinate was 1, use affine addition formulae, and correct
         * the Z coordinate of the result once at the end.
         * The exception is the precomputed G table points, which are actually
         * affine. Compared to the base used for other points, they have a Z ratio
         * of 1/Z, so we can use gej_add_zinv_var, which uses the same
         * isomorphism to efficiently add with a known Z inverse.
         */
        if (no > 0) {
            /* Compute the odd multiples in Jacobian form. */
            ecmult_odd_multiples_table(ECMULT_TABLE_SIZE(WINDOW_A), state->prej, state->zr, &a[state->ps[0].input_pos]);
            for (np = 1; np < no; ++np) {
                gej tmp = a[state->ps[np].input_pos];
    #ifdef VERIFY
                fe_normalize_var(&(state->prej[(np - 1) * ECMULT_TABLE_SIZE(WINDOW_A) + ECMULT_TABLE_SIZE(WINDOW_A) - 1].z));
    #endif
                gej_rescale(&tmp, &(state->prej[(np - 1) * ECMULT_TABLE_SIZE(WINDOW_A) + ECMULT_TABLE_SIZE(WINDOW_A) - 1].z));
                ecmult_odd_multiples_table(ECMULT_TABLE_SIZE(WINDOW_A), state->prej + np * ECMULT_TABLE_SIZE(WINDOW_A), state->zr + np * ECMULT_TABLE_SIZE(WINDOW_A), &tmp);
                fe_mul(state->zr + np * ECMULT_TABLE_SIZE(WINDOW_A), state->zr + np * ECMULT_TABLE_SIZE(WINDOW_A), &(a[state->ps[np].input_pos].z));
            }
            /* Bring them to the same Z denominator. */
            ge_globalz_set_table_gej(ECMULT_TABLE_SIZE(WINDOW_A) * no, state->pre_a, &Z, state->prej, state->zr);
        } else {
            fe_set_int(&Z, 1);
        }

        for (np = 0; np < no; ++np) {
            for (i = 0; i < ECMULT_TABLE_SIZE(WINDOW_A); i++) {
                ge_mul_lambda(&state->pre_a_lam[np * ECMULT_TABLE_SIZE(WINDOW_A) + i], &state->pre_a[np * ECMULT_TABLE_SIZE(WINDOW_A) + i]);
            }
        }

        if (ng) {
            /* split ng into ng_1 and ng_128 (where gn = gn_1 + gn_128*2^128, and gn_1 and gn_128 are ~128 bit) */
            scalar_split_128(&ng_1, &ng_128, ng);

            /* Build wnaf representation for ng_1 and ng_128 */
            bits_ng_1   = ecmult_wnaf(wnaf_ng_1,   129, &ng_1,   WINDOW_G);
            bits_ng_128 = ecmult_wnaf(wnaf_ng_128, 129, &ng_128, WINDOW_G);
            if (bits_ng_1 > bits) {
                bits = bits_ng_1;
            }
            if (bits_ng_128 > bits) {
                bits = bits_ng_128;
            }
        }

        gej_set_infinity(r);

        for (i = bits - 1; i >= 0; i--) {
            int n;
            gej_double_var(r, r, NULL);
            for (np = 0; np < no; ++np) {
                if (i < state->ps[np].bits_na_1 && (n = state->ps[np].wnaf_na_1[i])) {
                    ECMULT_TABLE_GET_GE(&tmpa, state->pre_a + np * ECMULT_TABLE_SIZE(WINDOW_A), n, WINDOW_A);
                    gej_add_ge_var(r, r, &tmpa, NULL);
                }
                if (i < state->ps[np].bits_na_lam && (n = state->ps[np].wnaf_na_lam[i])) {
                    ECMULT_TABLE_GET_GE(&tmpa, state->pre_a_lam + np * ECMULT_TABLE_SIZE(WINDOW_A), n, WINDOW_A);
                    gej_add_ge_var(r, r, &tmpa, NULL);
                }
            }
            if (i < bits_ng_1 && (n = wnaf_ng_1[i])) {
                ECMULT_TABLE_GET_GE_STORAGE(&tmpa, *ctx->pre_g, n, WINDOW_G);
                gej_add_zinv_var(r, r, &tmpa, &Z);
            }
            if (i < bits_ng_128 && (n = wnaf_ng_128[i])) {
                ECMULT_TABLE_GET_GE_STORAGE(&tmpa, *ctx->pre_g_128, n, WINDOW_G);
                gej_add_zinv_var(r, r, &tmpa, &Z);
            }
        }

        if (!r->infinity) {
            fe_mul(&r->z, &r->z, &Z);
        }
        */
}

pub fn ecmult_strauss_wnaf(
    ctx:   *const EcMultContext,
    state: *const StraussState,
    r:     *mut Gej,
    num:   usize,
    a:     *const Gej,
    na:    *const Scalar,
    ng:    *const Scalar,
) {
    tracing::trace!(
        target: "secp256k1::ecmult",
        num = num,
        ng_is_null = ng.is_null(),
        "ecmult_strauss_wnaf"
    );

    unsafe {
        let mut tmpa = Ge::new();
        let mut Z: Fe = core::mem::MaybeUninit::<Fe>::uninit().assume_init();
        /* Splitted G factors. */
        let mut ng_1 = Scalar::new();
        let mut ng_128 = Scalar::new();
        let mut wnaf_ng_1: [i32; 129] = [0; 129];
        let mut bits_ng_1: i32 = 0;
        let mut wnaf_ng_128: [i32; 129] = [0; 129];
        let mut bits_ng_128: i32 = 0;
        let mut i: i32;
        let mut bits: i32 = 0;
        let mut np: usize;
        let mut no: usize = 0;

        let ps: *mut StraussPointState = *(*state).ps();
        let prej: *mut Gej = *(*state).prej();
        let zr: *mut Fe = *(*state).zr();
        let pre_a: *mut Ge = *(*state).pre_a();
        let pre_a_lam: *mut Ge = *(*state).pre_a_lam();

        np = 0;
        while np < num {
            if scalar_is_zero(na.add(np)) != 0 || gej_is_infinity(a.add(np)) != 0 {
                np += 1;
                continue;
            }

            StraussPointState::write_input_pos(ps.add(no), np);

            /* split na into na_1 and na_lam (where na = na_1 + na_lam*lambda, and na_1 and na_lam are ~128 bit) */
            let mut na_tmp: Scalar = core::ptr::read(na.add(np));
            scalar_split_lambda(
                StraussPointState::na_1_mut_ptr(ps.add(no)),
                StraussPointState::na_lam_mut_ptr(ps.add(no)),
                core::ptr::addr_of_mut!(na_tmp),
            );

            /* build wnaf representation for na_1 and na_lam. */
            let bits_na_1: i32 = ecmult_wnaf(
                StraussPointState::wnaf_na_1_mut_ptr(ps.add(no)),
                129,
                StraussPointState::na_1_ptr(ps.add(no)),
                WINDOW_A as i32,
            );
            StraussPointState::write_bits_na_1(ps.add(no), bits_na_1);

            let bits_na_lam: i32 = ecmult_wnaf(
                StraussPointState::wnaf_na_lam_mut_ptr(ps.add(no)),
                129,
                StraussPointState::na_lam_ptr(ps.add(no)),
                WINDOW_A as i32,
            );
            StraussPointState::write_bits_na_lam(ps.add(no), bits_na_lam);

            verify_check!(bits_na_1 <= 129);
            verify_check!(bits_na_lam <= 129);

            if bits_na_1 > bits {
                bits = bits_na_1;
            }
            if bits_na_lam > bits {
                bits = bits_na_lam;
            }

            no += 1;
            np += 1;
        }

        /* Calculate odd multiples of a.
         * All multiples are brought to the same Z 'denominator', which is stored
         * in Z. Due to secp256k1' isomorphism we can do all operations pretending
         * that the Z coordinate was 1, use affine addition formulae, and correct
         * the Z coordinate of the result once at the end.
         * The exception is the precomputed G table points, which are actually
         * affine. Compared to the base used for other points, they have a Z ratio
         * of 1/Z, so we can use gej_add_zinv_var, which uses the same
         * isomorphism to efficiently add with a known Z inverse.
         */
        if no > 0 {
            /* Compute the odd multiples in Jacobian form. */
            let pos0: usize = StraussPointState::input_pos(ps.add(0));
            ecmult_odd_multiples_table(
                ecmult_table_size!(WINDOW_A) as i32,
                prej,
                zr,
                a.add(pos0),
            );

            np = 1;
            while np < no {
                let pos: usize = StraussPointState::input_pos(ps.add(np));
                let mut tmp: Gej = core::ptr::read(a.add(pos));

                #[cfg(VERIFY)]
                {
                    fe_normalize_var(gej_z_mut(
                        prej.add(
                            (np - 1) * ecmult_table_size!(WINDOW_A)
                                + (ecmult_table_size!(WINDOW_A) - 1),
                        ),
                    ));
                }

                gej_rescale(
                    core::ptr::addr_of_mut!(tmp),
                    gej_z(
                        prej.add(
                            (np - 1) * ecmult_table_size!(WINDOW_A)
                                + (ecmult_table_size!(WINDOW_A) - 1),
                        ),
                    ),
                );

                ecmult_odd_multiples_table(
                    ecmult_table_size!(WINDOW_A) as i32,
                    prej.add(np * ecmult_table_size!(WINDOW_A)),
                    zr.add(np * ecmult_table_size!(WINDOW_A)),
                    core::ptr::addr_of!(tmp),
                );

                fe_mul(
                    zr.add(np * ecmult_table_size!(WINDOW_A)),
                    zr.add(np * ecmult_table_size!(WINDOW_A)),
                    gej_z(a.add(pos)),
                );

                np += 1;
            }

            /* Bring them to the same Z denominator. */
            ge_globalz_set_table_gej(
                ecmult_table_size!(WINDOW_A) * no,
                pre_a,
                core::ptr::addr_of_mut!(Z),
                prej,
                zr,
            );
        } else {
            fe_set_int(core::ptr::addr_of_mut!(Z), 1);
        }

        np = 0;
        while np < no {
            i = 0;
            while (i as usize) < ecmult_table_size!(WINDOW_A) {
                ge_mul_lambda(
                    pre_a_lam.add(np * ecmult_table_size!(WINDOW_A) + (i as usize)),
                    pre_a.add(np * ecmult_table_size!(WINDOW_A) + (i as usize)),
                );
                i += 1;
            }
            np += 1;
        }

        if !ng.is_null() {
            /* split ng into ng_1 and ng_128 (where gn = gn_1 + gn_128*2^128, and gn_1 and gn_128 are ~128 bit) */
            scalar_split_128(core::ptr::addr_of_mut!(ng_1), core::ptr::addr_of_mut!(ng_128), ng);

            /* Build wnaf representation for ng_1 and ng_128 */
            bits_ng_1 = ecmult_wnaf(
                wnaf_ng_1.as_mut_ptr(),
                129,
                core::ptr::addr_of!(ng_1),
                WINDOW_G as i32,
            );
            bits_ng_128 = ecmult_wnaf(
                wnaf_ng_128.as_mut_ptr(),
                129,
                core::ptr::addr_of!(ng_128),
                WINDOW_G as i32,
            );
            if bits_ng_1 > bits {
                bits = bits_ng_1;
            }
            if bits_ng_128 > bits {
                bits = bits_ng_128;
            }
        }

        gej_set_infinity(r);

        i = bits - 1;
        while i >= 0 {
            let n: i32;
            gej_double_var(r, r, core::ptr::null_mut());

            np = 0;
            while np < no {
                let ps_np: *const StraussPointState = ps.add(np) as *const StraussPointState;

                if i < StraussPointState::bits_na_1(ps_np) {
                    n = *StraussPointState::wnaf_na_1_ptr(ps_np).add(i as usize);
                    if n != 0 {
                        ecmult_table_get_ge!(
                            core::ptr::addr_of_mut!(tmpa),
                            pre_a.add(np * ecmult_table_size!(WINDOW_A)),
                            n,
                            WINDOW_A
                        );
                        gej_add_ge_var(r, r, core::ptr::addr_of!(tmpa), core::ptr::null_mut());
                    }
                }

                if i < StraussPointState::bits_na_lam(ps_np) {
                    n = *StraussPointState::wnaf_na_lam_ptr(ps_np).add(i as usize);
                    if n != 0 {
                        ecmult_table_get_ge!(
                            core::ptr::addr_of_mut!(tmpa),
                            pre_a_lam.add(np * ecmult_table_size!(WINDOW_A)),
                            n,
                            WINDOW_A
                        );
                        gej_add_ge_var(r, r, core::ptr::addr_of!(tmpa), core::ptr::null_mut());
                    }
                }

                np += 1;
            }

            if i < bits_ng_1 && {
                n = wnaf_ng_1[i as usize];
                n != 0
            } {
                ecmult_table_get_ge_storage!(
                    core::ptr::addr_of_mut!(tmpa),
                    *(*ctx).pre_g(),
                    n,
                    WINDOW_G
                );
                gej_add_zinv_var(r, r, core::ptr::addr_of!(tmpa), core::ptr::addr_of!(Z));
            }

            if i < bits_ng_128 && {
                n = wnaf_ng_128[i as usize];
                n != 0
            } {
                ecmult_table_get_ge_storage!(
                    core::ptr::addr_of_mut!(tmpa),
                    *(*ctx).pre_g_128(),
                    n,
                    WINDOW_G
                );
                gej_add_zinv_var(r, r, core::ptr::addr_of!(tmpa), core::ptr::addr_of!(Z));
            }

            i -= 1;
        }

        if gej_is_infinity(r) == 0 {
            fe_mul(gej_z_mut(r), gej_z(r), core::ptr::addr_of!(Z));
        }
    }
}
