//! The legacy randomized operation stream, independent of database execution.
//!
//! One PRNG continues across option configurations. Diagnostic key/value state
//! starts empty at each configuration, matching the original test closure.
use bitcoinleveldb_rand::Random;
use bitcoinleveldbt_util::{dbtest_random_key, dbtest_random_string};

pub const LEGACY_STEPS_PER_CONFIGURATION: usize = 10_000;
pub const OPTION_CONFIGURATION_COUNT: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BatchMutation {
    Put { key: String, value: String },
    Delete { key: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mutation {
    Put { key: String, value: String },
    Delete { key: String },
    Batch(Vec<BatchMutation>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Operation {
    pub selector: u32,
    pub mutation: Mutation,
    pub last_key: String,
    pub last_value_length_bytes: usize,
}

pub struct OperationGenerator {
    random: Random,
    last_key: String,
    last_value_length_bytes: usize,
}

impl OperationGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            random: Random::new(seed),
            last_key: String::new(),
            last_value_length_bytes: 0,
        }
    }

    /// Reset closure-local diagnostic state without resetting the PRNG.
    pub fn begin_configuration(&mut self) {
        self.last_key.clear();
        self.last_value_length_bytes = 0;
    }

    /// Enter a selected legacy configuration without running a database.
    ///
    /// Earlier configurations always consume their complete legacy length.
    /// In particular, their lengths do not shrink with a bisected prefix.
    pub fn for_configuration(seed: u32, configuration_index: usize) -> Self {
        assert!(configuration_index < OPTION_CONFIGURATION_COUNT);
        let mut generator = Self::new(seed);
        for _ in 0..configuration_index {
            generator.begin_configuration();
            for _ in 0..LEGACY_STEPS_PER_CONFIGURATION {
                let _ = generator.next_operation();
            }
        }
        generator.begin_configuration();
        generator
    }

    pub fn next_operation(&mut self) -> Operation {
        let selector = self.random.uniform(100);
        let mutation = if selector < 45 {
            let key = dbtest_random_key((&mut self.random) as *mut Random);
            let length = if self.random.one_in(20) {
                100 + self.random.uniform(100) as i32
            } else {
                self.random.uniform(8) as i32
            };
            let value = dbtest_random_string((&mut self.random) as *mut Random, length);
            self.last_key = key.clone();
            self.last_value_length_bytes = value.len();
            Mutation::Put { key, value }
        } else if selector < 90 {
            let key = dbtest_random_key((&mut self.random) as *mut Random);
            self.last_key = key.clone();
            self.last_value_length_bytes = 0;
            Mutation::Delete { key }
        } else {
            let count = self.random.uniform(8);
            let mut updates = Vec::with_capacity(count as usize);
            for index in 0..count {
                // Preserve short-circuiting: index zero consumes no one_in(10).
                if index == 0 || !self.random.one_in(10) {
                    self.last_key = dbtest_random_key((&mut self.random) as *mut Random);
                }
                if self.random.one_in(2) {
                    let length = self.random.uniform(10) as i32;
                    let value = dbtest_random_string((&mut self.random) as *mut Random, length);
                    self.last_value_length_bytes = value.len();
                    updates.push(BatchMutation::Put { key: self.last_key.clone(), value });
                } else {
                    self.last_value_length_bytes = 0;
                    updates.push(BatchMutation::Delete { key: self.last_key.clone() });
                }
            }
            // An empty batch retains the preceding diagnostic key/value length.
            Mutation::Batch(updates)
        };
        Operation {
            selector,
            mutation,
            last_key: self.last_key.clone(),
            last_value_length_bytes: self.last_value_length_bytes,
        }
    }
}
