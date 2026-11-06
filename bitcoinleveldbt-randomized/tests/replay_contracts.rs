// Finite operation-tape checks. These construct no database or tracing subscriber.
// The independent reference below retains the pre-extraction generator's call
// order; comparing only final maps would miss reordered or cancelled updates.
#[path = "support/randomized_operations.rs"]
mod randomized_operations;

use bitcoinleveldb_rand::Random;
use bitcoinleveldbt_util::{dbtest_random_key, dbtest_random_string};
use randomized_operations::{
    BatchMutation, Mutation, Operation, OperationGenerator,
    LEGACY_STEPS_PER_CONFIGURATION, OPTION_CONFIGURATION_COUNT,
};
use std::path::{Component, Path};

const MEMORY_LIMIT: u64 = 512 * 1024 * 1024;

fn read_control(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(value) => value.trim().to_owned(),
        Err(error) => panic!("use the protected repair runner: cannot read {}: {}", path.display(), error),
    }
}

fn control_number(value: &str, name: &str) -> u64 {
    match value.parse::<u64>() {
        Ok(number) => number,
        Err(_) => panic!("use the protected repair runner: {} is not a finite limit", name),
    }
}

fn verify_protection() {
    assert!(matches!(std::env::var("BITCOIN_RS_RANDOMIZED_GUARD"), Ok(value) if value == "v1"),
            "use the protected repair runner; this test requires verified cgroup limits");
    let memberships = read_control(Path::new("/proc/self/cgroup"));
    let unified = match memberships.lines().find_map(|line| line.strip_prefix("0::")) {
        Some(value) => value,
        None => panic!("use the protected repair runner: unified cgroup membership missing"),
    };
    assert!(Path::new(unified).is_absolute(), "invalid unified cgroup path");
    assert!(Path::new(unified).components().all(|part| matches!(part, Component::RootDir | Component::Normal(_))),
            "invalid unified cgroup path components");
    let group = Path::new("/sys/fs/cgroup").join(unified.trim_start_matches('/'));
    let memory = control_number(&read_control(&group.join("memory.max")), "memory.max");
    let pids = control_number(&read_control(&group.join("pids.max")), "pids.max");
    assert!(memory > 0 && memory <= MEMORY_LIMIT, "use the protected repair runner: memory.max exceeds 512 MiB");
    assert!(pids > 0 && pids <= 64, "use the protected repair runner: pids.max exceeds 64");
    assert_eq!(read_control(&group.join("memory.swap.max")), "0", "use the protected repair runner: swap must be disabled");
    assert_eq!(read_control(&group.join("memory.oom.group")), "1", "use the protected repair runner: grouped OOM handling required");
    let cpu = read_control(&group.join("cpu.max"));
    let fields: Vec<&str> = cpu.split_whitespace().collect();
    assert_eq!(fields.len(), 2, "invalid cpu.max");
    let quota = control_number(fields[0], "cpu.max quota");
    let period = control_number(fields[1], "cpu.max period");
    assert!(quota > 0 && period > 0 && quota <= period,
            "use the protected repair runner: CPU quota must be at most one CPU");
    eprintln!("REPLAY_CONTRACT_GUARD cgroup={} memory={} pids={} cpu={}/{}", group.display(), memory, pids, quota, period);
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ReferenceUpdate {
    Put(String, String),
    Delete(String),
}

impl ReferenceUpdate {
    fn key(&self) -> &str {
        match self {
            Self::Put(key, _) | Self::Delete(key) => key,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ReferenceOperation {
    selector: u32,
    kind: &'static str,
    updates: Vec<ReferenceUpdate>,
    last_key: String,
    last_value_length_bytes: usize,
}

// This is deliberately separate from OperationGenerator. It follows the old
// inline test's branches and helper calls and adds only owned observation data.
// Only PRNG state crosses configuration boundaries; key/diagnostic state resets.
struct LegacyReference {
    random: Random,
    key: String,
    last_key: String,
    last_value_length_bytes: usize,
}

impl LegacyReference {
    fn new(seed: u32) -> Self {
        Self { random: Random::new(seed), key: String::new(),
               last_key: String::new(), last_value_length_bytes: 0 }
    }

    fn begin_configuration(&mut self) {
        self.key.clear();
        self.last_key.clear();
        self.last_value_length_bytes = 0;
    }

    fn next(&mut self) -> ReferenceOperation {
        let selector = self.random.uniform(100);
        let mut updates = Vec::new();
        let kind;
        if selector < 45 {
            kind = "put";
            self.key = dbtest_random_key((&mut self.random) as *mut Random);
            let value = dbtest_random_string(
                (&mut self.random) as *mut Random,
                if self.random.one_in(20) {
                    100 + self.random.uniform(100) as i32
                } else {
                    self.random.uniform(8) as i32
                },
            );
            self.last_key = self.key.clone();
            self.last_value_length_bytes = value.len();
            updates.push(ReferenceUpdate::Put(self.key.clone(), value));
        } else if selector < 90 {
            kind = "delete";
            self.key = dbtest_random_key((&mut self.random) as *mut Random);
            self.last_key = self.key.clone();
            self.last_value_length_bytes = 0;
            updates.push(ReferenceUpdate::Delete(self.key.clone()));
        } else {
            kind = "batch";
            let count = self.random.uniform(8) as i32;
            let mut i = 0;
            while i < count {
                // The short circuit at i == 0 is part of the legacy PRNG tape.
                if i == 0 || !self.random.one_in(10) {
                    self.key = dbtest_random_key((&mut self.random) as *mut Random);
                }
                if self.random.one_in(2) {
                    let value = dbtest_random_string(
                        (&mut self.random) as *mut Random,
                        self.random.uniform(10) as i32,
                    );
                    self.last_value_length_bytes = value.len();
                    updates.push(ReferenceUpdate::Put(self.key.clone(), value));
                } else {
                    self.last_value_length_bytes = 0;
                    updates.push(ReferenceUpdate::Delete(self.key.clone()));
                }
                self.last_key = self.key.clone();
                i += 1;
            }
        }
        ReferenceOperation { selector, kind, updates, last_key: self.last_key.clone(),
                             last_value_length_bytes: self.last_value_length_bytes }
    }
}

fn observed(operation: &Operation) -> ReferenceOperation {
    let (kind, updates) = match &operation.mutation {
        Mutation::Put { key, value } => ("put", vec![ReferenceUpdate::Put(key.clone(), value.clone())]),
        Mutation::Delete { key } => ("delete", vec![ReferenceUpdate::Delete(key.clone())]),
        Mutation::Batch(batch) => ("batch", batch.iter().map(|update| match update {
            BatchMutation::Put { key, value } => ReferenceUpdate::Put(key.clone(), value.clone()),
            BatchMutation::Delete { key } => ReferenceUpdate::Delete(key.clone()),
        }).collect()),
    };
    ReferenceOperation { selector: operation.selector, kind, updates,
                         last_key: operation.last_key.clone(),
                         last_value_length_bytes: operation.last_value_length_bytes }
}

fn reference_configuration(seed: u32, index: usize) -> LegacyReference {
    let mut reference = LegacyReference::new(seed);
    for _ in 0..index {
        reference.begin_configuration();
        for _ in 0..LEGACY_STEPS_PER_CONFIGURATION {
            let _ = reference.next();
        }
    }
    reference.begin_configuration();
    reference
}

fn complete(name: &str) {
    // Standalone even if libtest left its "test NAME ... " prefix unfinished.
    eprintln!("\nREPLAY_CONTRACT_OK case={}", name);
}

#[test]
fn replay_matches_legacy_tape() {
    verify_protection();
    for seed in [0, 1, 301, 0x7fff_ffff, u32::MAX] {
        let mut actual = OperationGenerator::new(seed);
        let mut reference = LegacyReference::new(seed);
        for config in 0..OPTION_CONFIGURATION_COUNT {
            actual.begin_configuration();
            reference.begin_configuration();
            for step in 0..128 {
                let generated = observed(&actual.next_operation());
                assert_eq!(generated, reference.next(),
                           "legacy tape differs: seed={} config={} step={}", seed, config, step);
                if seed == 301 && config == 0 && step == 0 {
                    // Independently reproduced from the pre-extraction PRNG rules.
                    assert_eq!(generated.selector, 7);
                    assert_eq!(generated.updates,
                               vec![ReferenceUpdate::Put("\0\u{fd}".to_owned(), "%w".to_owned())]);
                }
            }
        }
    }
    complete("replay_matches_legacy_tape");
}

#[test]
fn replay_preserves_configuration_offsets() {
    verify_protection();
    assert_eq!(LEGACY_STEPS_PER_CONFIGURATION, 10_000);
    assert_eq!(OPTION_CONFIGURATION_COUNT, 4);
    let seed = 301;
    let first_selectors = [[7, 35, 7], [81, 24, 49], [94, 35, 15], [56, 20, 24]];
    let mut reference = LegacyReference::new(seed);
    for config in 0..OPTION_CONFIGURATION_COUNT {
        reference.begin_configuration();
        let mut replay = OperationGenerator::for_configuration(seed, config);
        let mut old_reset_policy = LegacyReference::new(seed);
        old_reset_policy.begin_configuration();
        let mut distinguishes_old_reset = false;
        for step in 0..LEGACY_STEPS_PER_CONFIGURATION {
            let expected = reference.next();
            if step < 3 {
                assert_eq!(expected.selector, first_selectors[config][step],
                           "legacy selector anchor differs: config={} step={}", config, step);
            }
            if step < 64 {
                let reset = old_reset_policy.next();
                distinguishes_old_reset |= reset != expected;
                assert_eq!(observed(&replay.next_operation()), expected,
                           "configuration offset differs: config={} step={}", config, step);
            }
        }
        if config > 0 {
            assert!(distinguishes_old_reset,
                    "selected tape must distinguish the old per-configuration reset at config {}", config);
        } else {
            assert!(!distinguishes_old_reset, "configuration zero must retain its original seed origin");
        }
    }
    complete("replay_preserves_configuration_offsets");
}

#[test]
fn replay_preserves_short_prefixes() {
    verify_protection();
    let seed = 301;
    for config in 0..OPTION_CONFIGURATION_COUNT {
        let mut reference = reference_configuration(seed, config);
        let expected: Vec<ReferenceOperation> = (0..32).map(|_| reference.next()).collect();
        for prefix in [0, 1, 7, 31] {
            let mut replay = OperationGenerator::for_configuration(seed, config);
            let actual: Vec<ReferenceOperation> = (0..prefix)
                .map(|_| observed(&replay.next_operation())).collect();
            assert_eq!(actual.as_slice(), &expected[..prefix],
                       "short prefix differs: config={} prefix={}", config, prefix);
            assert_eq!(observed(&replay.next_operation()), expected[prefix],
                       "prefix stopping changed next operation: config={} prefix={}", config, prefix);
        }
    }
    complete("replay_preserves_short_prefixes");
}

#[test]
fn replay_covers_batch_order() {
    verify_protection();
    let mut actual = OperationGenerator::new(301);
    actual.begin_configuration();
    let mut reference = LegacyReference::new(301);
    reference.begin_configuration();
    let mut empty_batch = false;
    let mut repeated_key_batch = false;
    let mut mixed_batch = false;
    let mut multibyte_key = false;
    let mut long_value = false;
    let mut previous: Option<ReferenceOperation> = None;
    for step in 0..4096 {
        let expected = reference.next();
        assert_eq!(observed(&actual.next_operation()), expected,
                   "batch tape differs at step {}", step);
        if expected.kind == "batch" {
            if expected.updates.is_empty() {
                empty_batch = true;
                if let Some(before) = &previous {
                    assert_eq!(expected.last_key, before.last_key, "empty batch lost diagnostic key");
                    assert_eq!(expected.last_value_length_bytes, before.last_value_length_bytes,
                               "empty batch lost diagnostic value length");
                }
            }
            repeated_key_batch |= expected.updates.windows(2).any(|pair| pair[0].key() == pair[1].key());
            mixed_batch |= expected.updates.iter().any(|entry| matches!(entry, ReferenceUpdate::Put(_, _)))
                        && expected.updates.iter().any(|entry| matches!(entry, ReferenceUpdate::Delete(_)));
        }
        for update in &expected.updates {
            multibyte_key |= update.key().as_bytes().iter().any(|byte| *byte >= 128);
            if let ReferenceUpdate::Put(_, value) = update {
                long_value |= value.len() >= 100;
            }
        }
        previous = Some(expected);
    }
    assert!(empty_batch, "fixed tape must include an empty batch");
    assert!(repeated_key_batch, "fixed tape must include ordered repeated-key updates");
    assert!(mixed_batch, "fixed tape must include puts and deletes in the same batch");
    assert!(multibyte_key, "fixed tape must include UTF-8 keys whose byte and character lengths differ");
    assert!(long_value, "fixed tape must include the legacy long-value branch");

    // Place a configuration boundary immediately before a known empty batch.
    // Its mutation cannot overwrite stale metadata and hide a missing reset.
    let mut reset_actual = OperationGenerator::new(301);
    let mut reset_reference = LegacyReference::new(301);
    for _ in 0..112 {
        assert_eq!(observed(&reset_actual.next_operation()), reset_reference.next());
    }
    assert_eq!(reset_reference.last_key.as_bytes(), &[0xc3, 0xbe]);
    assert_eq!(reset_reference.last_value_length_bytes, 4);
    reset_actual.begin_configuration();
    reset_reference.begin_configuration();
    let after_reset = reset_reference.next();
    assert_eq!(after_reset.selector, 91);
    assert_eq!(after_reset.kind, "batch");
    assert!(after_reset.updates.is_empty());
    assert!(after_reset.last_key.is_empty());
    assert_eq!(after_reset.last_value_length_bytes, 0);
    assert_eq!(observed(&reset_actual.next_operation()), after_reset,
               "configuration reset must clear metadata while preserving the PRNG position");
    complete("replay_covers_batch_order");
}
