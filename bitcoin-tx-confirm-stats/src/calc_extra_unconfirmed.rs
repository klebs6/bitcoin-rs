// ---------------- [ File: bitcoin-tx-confirm-stats/src/calc_extra_unconfirmed.rs ]
crate::ix!();

impl TxConfirmStats {

    /// Compute the number of transactions still unconfirmed in this bucket
    /// for `conf_target` or longer, including old unconfirmed.
    ///
    /// Bit-for-bit equivalent to Core's extraNum loop.
    pub fn calc_extra_unconfirmed(
        &self,
        bucket_index:   usize,
        conf_target:    usize,
        n_block_height: u32,
        bins:           usize,

    ) -> i32 {

        let mut extra = 0;

        // extraNum: txs still in mempool for confTarget or longer
        for confct in conf_target..self.get_max_confirms() as usize {

            // emulate (nBlockHeight - confct) % bins with unsigned wraparound
            let idx = ((n_block_height as isize - confct as isize)
                .rem_euclid(bins as isize)) as usize;

            extra += self.unconf_txs()[idx][bucket_index];
        }

        extra + self.old_unconf_txs()[bucket_index]
    }
}

#[cfg(test)]
mod index_math_spec {
    use super::*;

    /// Reference: exact C++ semantics using Euclidean remainder.
    /// Mirrors: (nBlockHeight - confct) % bins on unsigned, but done safely.
    fn idx_cxx(n_block_height: usize, confct: usize, bins: usize) -> usize {
        let n = n_block_height as isize;
        let c = confct as isize;
        let m = bins as isize;
        ((n - c).rem_euclid(m)) as usize
    }

    /// Production form we intend to use everywhere.
    #[inline]
    fn idx_rust(n_block_height: usize, confct: usize, bins: usize) -> usize {
        // Keep this in one place: if the production code changes, update here and the test suite
        // will immediately tell us whether semantics shifted.
        idx_cxx(n_block_height, confct, bins)
    }

    /// Algebraically equivalent form sometimes used in ports:
    /// ((n % bins) + bins - (confct % bins)) % bins
    fn idx_alt_mod_identity(n_block_height: usize, confct: usize, bins: usize) -> usize {
        let n = n_block_height % bins;
        let c = confct % bins;
        (n + bins - c) % bins
    }

    #[traced_test]
    fn agrees_with_cxx_for_small_examples() {
        // A few hand-picked cases including underflow-like scenarios (confct > n)
        let cases = [
            (100usize, 0usize, 25usize),
            (100, 5, 25),
            (100, 24, 25),
            (100, 25, 25),
            (100, 26, 25),
            (0, 0, 7),
            (0, 1, 7),
            (3, 10, 7),
            (42, 999, 64),
        ];

        for &(n, c, bins) in &cases {
            assert_eq!(idx_rust(n, c, bins), idx_cxx(n, c, bins), "n={n} c={c} bins={bins}");
        }
    }

    #[traced_test]
    fn exhaustive_small_bins_equivalence() {
        // Exhaustive for small bins; broad-ish for values.
        for bins in 1..=16 {
            for n in 0..(3 * bins + 3) {
                for c in 0..(3 * bins + 3) {
                    assert_eq!(idx_rust(n, c, bins), idx_cxx(n, c, bins),
                        "equivalence failed: bins={bins} n={n} confct={c}");
                }
            }
        }
    }

    #[traced_test]
    fn large_bins_broad_sweep_equivalence() {
        // Cover a variety of larger bin sizes without random deps.
        let bin_set = [25usize, 32, 64, 100, 256, 1000];
        for &bins in &bin_set {
            for n in [0, 1, bins / 2, bins - 1, bins, bins + 1, 2 * bins + 7, 10_000, 123_456] {
                // Check multiple conf_target starting points
                for conf_target in [0usize, 1, bins / 4, bins / 2, bins - 1] {
                    // In Core, unconfTxs.len() == bins and confct ∈ [conf_target, bins)
                    for c in conf_target..bins {
                        assert_eq!(idx_rust(n, c, bins), idx_cxx(n, c, bins),
                            "bins={bins} n={n} conf_target={conf_target} confct={c}");
                    }
                }
            }
        }
    }

    #[traced_test]
    fn sequence_steps_backwards_mod_bins() {
        // Property: for fixed (n, bins), idx(n, confct+1) == (idx(n, confct) + bins - 1) % bins
        let bin_set = [5usize, 7, 8, 25, 64];

        for &bins in &bin_set {
            for n in 0..(2 * bins + 3) {
                for c in 0..(bins.saturating_sub(1)) {
                    let a = idx_rust(n, c, bins);
                    let b = idx_rust(n, c + 1, bins);
                    let expect = (a + bins - 1) % bins;
                    assert_eq!(b, expect, "bins={bins} n={n} c={c} a={a} b={b} expect={expect}");
                }
            }
        }
    }

    #[traced_test]
    fn sequence_covers_expected_unique_indices() {
        // For any conf_target, the set of indices produced for confct in [conf_target, bins)
        // is exactly (bins - conf_target) distinct indices (one per step).
        let bin_set = [1usize, 2, 3, 5, 7, 8, 25];
        for &bins in &bin_set {
            for n in 0..(2 * bins + 1) {
                for conf_target in 0..bins {
                    let mut seen = vec![false; bins];
                    let mut count = 0usize;
                    for c in conf_target..bins {
                        let idx = idx_rust(n, c, bins);
                        assert!(idx < bins, "index out of range: idx={idx} bins={bins}");
                        // Every index in this span should be hit at most once
                        assert!(!seen[idx], "duplicate index in span: bins={bins} n={n} conf_target={conf_target} idx={idx}");
                        seen[idx] = true;
                        count += 1;
                    }
                    assert_eq!(count, bins - conf_target, "wrong number of indices: bins={bins} conf_target={conf_target}");
                }
            }
        }
    }

    #[traced_test]
    fn algebraic_identity_holds() {
        // Demonstrate ((n % bins) + bins - (confct % bins)) % bins == (n - confct) mod bins
        // using Euclidean remainder. This guards against future “simplifications”.
        let bin_set = [3usize, 5, 7, 25, 64];

        for &bins in &bin_set {
            for n in 0..(3 * bins + 5) {
                for c in 0..(3 * bins + 5) {
                    let a = idx_cxx(n, c, bins);
                    let b = idx_alt_mod_identity(n, c, bins);
                    assert_eq!(a, b, "algebraic identity failed: bins={bins} n={n} c={c} a={a} b={b}");
                }
            }
        }
    }
}

#[cfg(test)]
mod extra_unconfirmed_spec {
    use super::*;

    fn mk_stats(nb: usize, mp: usize, decay: f64, scale: u32) -> TxConfirmStats {
        let buckets: Vec<f64> = (0..nb).map(|i| (i + 1) as f64).collect();
        TxConfirmStats::new(&buckets, &Default::default(), mp as u32, decay, scale)
    }

    #[traced_test]
    fn sums_all_indices_from_conf_target_to_max_minus_one_and_adds_old() {
        // bins = get_max_confirms = scale * periods
        let mut s = mk_stats(3, 4, 0.0, 2); // periods=4 -> bins=8
        let bins = s.get_max_confirms() as usize;
        assert_eq!(bins, 8);

        // Put 1 unconfirmed tx in every time bin for bucket 1
        s.unconf_txs_mut().iter_mut().for_each(|row| row[1] = 1);
        // And 3 old unconfirmed for bucket 1
        s.old_unconf_txs_mut()[1] = 3;

        // For any (n_block_height, conf_target), sum should be (#terms) + old
        // #terms = max_confirms - conf_target
        let n_block_height = 17u32; // arbitrary
        let conf_target = 2usize;

        let got = s.calc_extra_unconfirmed(1, conf_target, n_block_height, bins);
        let expect = (s.get_max_confirms() as usize - conf_target) as i32 + 3;
        assert_eq!(got, expect);
    }

    #[traced_test]
    fn wraparound_indexes_are_handled_with_euclidean_remainder() {
        let mut s = mk_stats(2, 3, 0.0, 1); // periods=3, scale=1 -> bins=3
        let bins = s.get_max_confirms() as usize;
        assert_eq!(bins, 3);

        // Encode a recognizable pattern in the circular buffer for bucket 0
        // unconf_txs[idx][0] = idx
        for (idx, row) in s.unconf_txs_mut().iter_mut().enumerate() {
            row[0] = idx as i32;
        }
        s.old_unconf_txs_mut()[0] = 10;

        // n_block_height small, conf_target larger -> forces negative (n - c)
        let n_block_height = 1u32;
        let conf_target = 1usize;

        // extra = sum_{c=1..2} unconf[(n-c) mod 3][0] + old
        // c=1: (1-1) mod 3 = 0 -> value 0
        // c=2: (1-2) mod 3 = -1 mod 3 = 2 -> value 2
        // total so far = 0 + 2 = 2; + old (10) = 12
        let got = s.calc_extra_unconfirmed(0, conf_target, n_block_height, bins);
        assert_eq!(got, 12);
    }
}
