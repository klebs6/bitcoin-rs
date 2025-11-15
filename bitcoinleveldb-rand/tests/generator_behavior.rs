// ---------------- [ File: bitcoinleveldb-rand/tests/generator_behavior.rs ]
use bitcoinleveldb_rand::*;
use bitcoin_imports::*;

#[traced_test]
fn next_produces_expected_minstd_prefix_for_seed_one() {
    // First ten values of the Park-Miller MINSTD generator with seed 1.
    let expected: [u32; 10] = [
        16_807,
        282_475_249,
        1_622_650_073,
        984_943_658,
        1_144_108_930,
        470_211_272,
        101_027_544,
        1_457_850_878,
        1_458_777_923,
        2_007_237_709,
    ];

    let mut rng = Random::new(1);

    for (index, expected_value) in expected.iter().enumerate() {
        let generated = rng.next();
        trace!(
            "minstd_prefix_check: index={}, expected_value={}, generated_value={}",
            index,
            expected_value,
            generated
        );
        assert_eq!(
            *expected_value,
            generated,
            "mismatch at sequence index {}",
            index
        );
    }
}

#[traced_test]
fn next_matches_park_miller_checkpoint_after_10000_steps() {
    let mut rng = Random::new(1);
    let iterations = 10_000usize;
    let mut value = 0u32;

    for iteration in 0..iterations {
        value = rng.next();
        if iteration % 1_000 == 0 {
            trace!(
                "park_miller_checkpoint_progress: iteration={}, value={}",
                iteration + 1,
                value
            );
        }
    }

    let expected = 1_043_618_065u32;
    info!(
        "park_miller_checkpoint_complete: iterations={}, final_value={}",
        iterations,
        value
    );
    assert_eq!(
        expected, value,
        "Random::next failed Park-Miller 10_000-iteration checkpoint"
    );
}

#[traced_test]
fn sequences_from_identical_seeds_are_identical() {
    let mut rng_a = Random::new(123_456_789);
    let mut rng_b = Random::new(123_456_789);
    let steps = 1_000usize;

    for step in 0..steps {
        let value_a = rng_a.next();
        let value_b = rng_b.next();

        trace!(
            "identical_seed_check: step={}, value_a={}, value_b={}",
            step,
            value_a,
            value_b
        );

        assert_eq!(
            value_a, value_b,
            "sequences diverged at step {} for identical seeds",
            step
        );
    }
}

#[traced_test]
fn sequences_from_distinct_seeds_diverge_quickly() {
    let mut rng_a = Random::new(1);
    let mut rng_b = Random::new(2);
    let steps = 100usize;
    let mut found_difference = false;

    for step in 0..steps {
        let value_a = rng_a.next();
        let value_b = rng_b.next();

        if value_a != value_b {
            trace!(
                "distinct_seed_divergence: step={}, value_a={}, value_b={}",
                step,
                value_a,
                value_b
            );
            found_difference = true;
            break;
        }
    }

    assert!(
        found_difference,
        "sequences from distinct seeds did not diverge within {} steps",
        steps
    );
}

#[traced_test]
fn uniform_outputs_are_within_requested_ranges() {
    let mut rng = Random::new(987_654_321);
    let ranges: [i32; 7] = [1, 2, 3, 5, 7, 31, 1_000];

    for &range in ranges.iter() {
        for iteration in 0..5_000 {
            let value = rng.uniform(range);

            if value >= range as u32 {
                error!(
                    "uniform_range_violation: n={}, iteration={}, value={}",
                    range,
                    iteration,
                    value
                );
            }

            assert!(
                value < range as u32,
                "uniform() produced out-of-range value {} for n={}",
                value,
                range
            );
        }
    }
}

#[traced_test]
fn one_in_probability_is_reasonable() {
    let mut rng = Random::new(42);
    let n = 10;
    let trials = 20_000usize;
    let mut success_count = 0usize;

    for _ in 0..trials {
        if rng.one_in(n) {
            success_count += 1;
        }
    }

    let expected = trials as f64 / n as f64;
    let tolerance = expected * 0.25;
    let diff = (success_count as f64 - expected).abs();

    info!(
        "one_in_probability_check: n={}, trials={}, success_count={}, expected≈{}, \
         tolerance={}, diff={}",
        n,
        trials,
        success_count,
        expected,
        tolerance,
        diff
    );

    assert!(
        diff <= tolerance,
        "one_in() observed count {} differs from expected {} by more than tolerance {}",
        success_count,
        expected,
        tolerance
    );
}

#[traced_test]
fn skewed_values_do_not_exceed_upper_bound_for_max_log_10() {
    let mut rng = Random::new(4_242);
    let max_log = 10;
    let upper_bound = 1u32 << max_log;
    let trials = 50_000usize;

    for iteration in 0..trials {
        let value = rng.skewed(max_log);

        if value >= upper_bound {
            error!(
                "skewed_range_violation: max_log={}, iteration={}, value={}, upper_bound={}",
                max_log,
                iteration,
                value,
                upper_bound
            );
        }

        assert!(
            value < upper_bound,
            "skewed() produced out-of-range value {} for max_log={}",
            value,
            max_log
        );
    }
}

#[traced_test]
fn constructor_normalizes_zero_and_maximum_seed_to_seed_one_behavior() {
    let sequence_length = 32usize;

    let baseline = generate_sequence_from_seed(1u32, sequence_length);
    let zero_sequence = generate_sequence_from_seed(0u32, sequence_length);
    let max_seed_sequence =
        generate_sequence_from_seed(2_147_483_647u32, sequence_length);

    assert_eq!(
        baseline, zero_sequence,
        "sequence from seed 0 did not match sequence from seed 1"
    );
    assert_eq!(
        baseline, max_seed_sequence,
        "sequence from maximum seed did not match sequence from seed 1"
    );
}

fn generate_sequence_from_seed(seed: u32, length: usize) -> Vec<u32> {
    let mut rng = Random::new(seed);
    let mut values = Vec::with_capacity(length);

    for index in 0..length {
        let value = rng.next();
        trace!(
            "generate_sequence_from_seed: index={}, seed={}, value={}",
            index,
            seed,
            value
        );
        values.push(value);
    }

    values
}
