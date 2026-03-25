// ---------------- [ File: bitcoinleveldbt-randomized/tests/bisect.rs ]
use traced_test::*;
use tracing_setup::*;
use bitcoinleveldb_rand::*;
use bitcoin_imports::*;
use bitcoinleveldbt_dbtest::*;
use bitcoinleveldbt_util::*;
use bitcoinleveldbt_snapshot::*;
use bitcoinleveldb_snapshot::*;
use bitcoinleveldb_options::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_batch::*;
use bitcoinleveldb_dbinterface::*;
use bitcoinleveldb_modeldb::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Getters, Builder)]
#[getset(get = "pub")]
#[builder(pattern = "owned")]
struct BitcoinLevelDbTRandomizedBisectOperationKind {
#[builder(default = "BitcoinLevelDbTRandomizedBisectOperationKindField::NoneYet")]
    field: BitcoinLevelDbTRandomizedBisectOperationKindField,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BitcoinLevelDbTRandomizedBisectOperationKindField {
    NoneYet,
    Put,
    Delete,
    WriteBatch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BitcoinLevelDbTRandomizedBisectFailureMode {
    OptionConfigurationUnavailable,
    ModelPutStatusNotOk,
    DatabasePutStatusNotOk,
    ModelDeleteStatusNotOk,
    DatabaseDeleteStatusNotOk,
    ModelWriteBatchStatusNotOk,
    DatabaseWriteBatchStatusNotOk,
    CompareIteratorsCurrentStateMismatch,
    CompareIteratorsSnapshotStateMismatch,
    CompareIteratorsAfterReopenMismatch,
}

#[derive(Clone, Debug, Getters, Builder)]
#[getset(get = "pub")]
#[builder(pattern = "owned")]
struct BitcoinLevelDbTRandomizedBisectedFailureObservation {
    /// This is the deterministic PRNG seed for the prefix run.
    /// Re-running with the same seed and step limit must preserve the generated operation stream.
    seed: u32,
    /// This is the zero-based option-configuration ordinal used by the test fixture.
    /// The ordinal must remain stable across reruns of the same workspace revision.
    option_configuration_index: i32,
    /// This is the resolved option-configuration discriminator for the ordinal.
    /// It records which DBTest configuration surface was active when the failure was observed.
    option_configuration: DBTestOptionConfig,
    /// This is the zero-based logical step at which the first observed failure occurred.
    /// The step index must refer to the first failing prefix, not a later derived mismatch.
    failing_step_index: i32,
    /// This is the executed prefix length used for the run that produced this observation.
    /// Prefix runs are monotone: if this prefix fails, any longer prefix must include this failure site.
    executed_prefix_length: i32,
    /// This is the random branch selector `p` for the failing step.
    /// It preserves the exact branch decision that classified the step.
    operation_selector_probability: u32,
    /// This is the logical operation family active at the failing step.
    /// It must identify the last mutating action applied before the mismatch was detected.
    operation_kind: BitcoinLevelDbTRandomizedBisectOperationKindField,
    /// This is the last logical key chosen by the generator for the failing step.
    /// It is observational only and must not be normalized or rewritten.
    last_key: String,
    /// This is the byte length of the last logical value chosen by the generator for the failing step.
    /// For delete-only steps this must remain zero.
    last_value_length_bytes: usize,
    /// This is the number of per-key updates encoded into the last write batch.
    /// For non-batch steps this must remain zero.
    last_write_batch_operation_count: i32,
    /// This is the first observed failure class for the prefix.
    /// The class must remain stable for deterministic replay of the same prefix.
    failure_mode: BitcoinLevelDbTRandomizedBisectFailureMode,
}

fn bitcoinleveldbt_randomized_bisect_option_configuration_from_index(
    option_configuration_index: i32,
) -> DBTestOptionConfig {
    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_bisect_option_configuration_from_index_entry",
        label = "bitcoinleveldbt_randomized.bisect.option_configuration_from_index.entry",
        option_configuration_index = option_configuration_index
    );

    let option_configuration = match option_configuration_index {
        0i32 => DBTestOptionConfig::Default,
        1i32 => DBTestOptionConfig::Reuse,
        2i32 => DBTestOptionConfig::Filter,
        3i32 => DBTestOptionConfig::Uncompressed,
        _ => DBTestOptionConfig::End,
    };

    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_bisect_option_configuration_from_index_exit",
        label = "bitcoinleveldbt_randomized.bisect.option_configuration_from_index.exit",
        option_configuration_index = option_configuration_index,
        option_configuration = ?option_configuration
    );

    option_configuration

}

fn bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration(
    dbtest: &mut DBTest,
    option_configuration_index: i32,
) -> bool {
    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_entry",
        label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.entry",
        option_configuration_index = option_configuration_index
    );

    let mut current_option_configuration_index: i32 = 0i32;

    while current_option_configuration_index < option_configuration_index {
        let advanced = dbtest.change_options();

        match advanced {
            true => {
                current_option_configuration_index += 1i32;

                debug!(
                    target: "bitcoinleveldbt_randomized::randomized",
                    event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_state_transition",
                    label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.state_transition",
                    current_option_configuration_index = current_option_configuration_index,
                    option_configuration_index = option_configuration_index
                );
            }
            false => {
                warn!(
                    target: "bitcoinleveldbt_randomized::randomized",
                    event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_unavailable",
                    label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.unavailable",
                    current_option_configuration_index = current_option_configuration_index,
                    option_configuration_index = option_configuration_index
                );

                trace!(
                    target: "bitcoinleveldbt_randomized::randomized",
                    event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_exit",
                    label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.exit",
                    option_configuration_index = option_configuration_index,
                    available = false
                );

                return false;
            }
        }
    }

    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_exit",
        label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.exit",
        option_configuration_index = option_configuration_index,
        available = true
    );

    true

}

fn bitcoinleveldbt_randomized_bisect_build_failure_observation(
    seed: u32,
    option_configuration_index: i32,
    option_configuration: DBTestOptionConfig,
    failing_step_index: i32,
    executed_prefix_length: i32,
    operation_selector_probability: u32,
    operation_kind: BitcoinLevelDbTRandomizedBisectOperationKindField,
    last_key: &String,
    last_value_length_bytes: usize,
    last_write_batch_operation_count: i32,
    failure_mode: BitcoinLevelDbTRandomizedBisectFailureMode,
) -> BitcoinLevelDbTRandomizedBisectedFailureObservation {
    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_bisect_build_failure_observation_entry",
        label = "bitcoinleveldbt_randomized.bisect.build_failure_observation.entry",
        seed = seed,
        option_configuration_index = option_configuration_index,
        option_configuration = ?option_configuration,
        failing_step_index = failing_step_index,
        executed_prefix_length = executed_prefix_length,
        operation_selector_probability = operation_selector_probability,
        operation_kind = ?operation_kind,
        last_value_length_bytes = last_value_length_bytes,
        last_write_batch_operation_count = last_write_batch_operation_count,
        failure_mode = ?failure_mode
    );

    let observation_builder =
        BitcoinLevelDbTRandomizedBisectedFailureObservationBuilder::default()
        .seed(seed)
        .option_configuration_index(option_configuration_index)
        .option_configuration(option_configuration)
        .failing_step_index(failing_step_index)
        .executed_prefix_length(executed_prefix_length)
        .operation_selector_probability(operation_selector_probability)
        .operation_kind(operation_kind)
        .last_key(last_key.clone())
        .last_value_length_bytes(last_value_length_bytes)
        .last_write_batch_operation_count(last_write_batch_operation_count)
        .failure_mode(failure_mode);

    let observation = match observation_builder.build() {
        Ok(value) => value,
        Err(builder_error) => {
            error!(
                target: "bitcoinleveldbt_randomized::randomized",
                event = "bitcoinleveldbt_randomized_bisect_build_failure_observation_builder_failed",
                label = "bitcoinleveldbt_randomized.bisect.build_failure_observation.builder_failed",
                builder_error = ?builder_error
            );
            panic!();
        }
    };

    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_bisect_build_failure_observation_exit",
        label = "bitcoinleveldbt_randomized.bisect.build_failure_observation.exit",
        seed = *observation.seed(),
        option_configuration_index = *observation.option_configuration_index(),
        failing_step_index = *observation.failing_step_index(),
        failure_mode = ?observation.failure_mode()
    );

    observation

}

fn bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure(
    seed: u32,
    option_configuration_index: i32,
    executed_prefix_length: i32,
) -> Option<BitcoinLevelDbTRandomizedBisectedFailureObservation> {
    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_entry",
        label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.entry",
        seed = seed,
        option_configuration_index = option_configuration_index,
        executed_prefix_length = executed_prefix_length
    );

    let option_configuration =
        bitcoinleveldbt_randomized_bisect_option_configuration_from_index(
            option_configuration_index,
        );

    match option_configuration {
        DBTestOptionConfig::End => {
            let failure_observation =
                bitcoinleveldbt_randomized_bisect_build_failure_observation(
                    seed,
                    option_configuration_index,
                    option_configuration,
                    0i32,
                    executed_prefix_length,
                    0u32,
                    BitcoinLevelDbTRandomizedBisectOperationKindField::NoneYet,
                    &String::new(),
                    0usize,
                    0i32,
                    BitcoinLevelDbTRandomizedBisectFailureMode::OptionConfigurationUnavailable,
                );

            trace!(
                target: "bitcoinleveldbt_randomized::randomized",
                event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                seed = seed,
                option_configuration_index = option_configuration_index,
                executed_prefix_length = executed_prefix_length,
                failed = true
            );

            return Some(failure_observation);
        }
        DBTestOptionConfig::Default
            | DBTestOptionConfig::Reuse
            | DBTestOptionConfig::Filter
            | DBTestOptionConfig::Uncompressed => {}
    }

    let mut rnd = Random::new(seed);
    let mut dbtest = DBTest::default();

    let option_configuration_available =
        bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration(
            &mut dbtest,
            option_configuration_index,
        );

    match option_configuration_available {
        true => {}
        false => {
            let failure_observation =
                bitcoinleveldbt_randomized_bisect_build_failure_observation(
                    seed,
                    option_configuration_index,
                    option_configuration,
                    0i32,
                    executed_prefix_length,
                    0u32,
                    BitcoinLevelDbTRandomizedBisectOperationKindField::NoneYet,
                    &String::new(),
                    0usize,
                    0i32,
                    BitcoinLevelDbTRandomizedBisectFailureMode::OptionConfigurationUnavailable,
                );

            trace!(
                target: "bitcoinleveldbt_randomized::randomized",
                event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                seed = seed,
                option_configuration_index = option_configuration_index,
                executed_prefix_length = executed_prefix_length,
                failed = true
            );

            return Some(failure_observation);
        }
    }

    let opts = dbtest.current_options();
    let mut model = ModelDB::new(&opts);

    let mut model_snap: Option<Box<dyn Snapshot>> = None;
    let mut db_snap: Option<Box<dyn Snapshot>> = None;

    let mut k = String::new();
    let mut v = String::new();

    let mut last_key = String::new();
    let mut last_value_length_bytes: usize = 0usize;
    let mut last_write_batch_operation_count: i32 = 0i32;
    let mut last_operation_kind =
        BitcoinLevelDbTRandomizedBisectOperationKindField::NoneYet;
    let mut last_operation_selector_probability: u32 = 0u32;

    let mut step: i32 = 0i32;
    while step < executed_prefix_length {
        let p = rnd.uniform(100);
        last_operation_selector_probability = p;
        last_write_batch_operation_count = 0i32;

        match p {
            0u32..=44u32 => {
                last_operation_kind =
                    BitcoinLevelDbTRandomizedBisectOperationKindField::Put;

                k = dbtest_random_key((&mut rnd) as *mut Random);
                v = dbtest_random_string(
                    (&mut rnd) as *mut Random,
                    match rnd.one_in(20) {
                        true => 100i32 + (rnd.uniform(100) as i32),
                        false => rnd.uniform(8) as i32,
                    },
                );

                last_key = k.clone();
                last_value_length_bytes = v.len();

                let write_options = WriteOptions::default();
                let ks = Slice::from(&k);
                let vs = Slice::from(&v);

                let model_put_status = model.put(&write_options, &ks, &vs);

                match model_put_status.is_ok() {
                    true => {}
                    false => {
                        let failure_observation =
                            bitcoinleveldbt_randomized_bisect_build_failure_observation(
                                seed,
                                option_configuration_index,
                                option_configuration,
                                step,
                                executed_prefix_length,
                                last_operation_selector_probability,
                                last_operation_kind,
                                &last_key,
                                last_value_length_bytes,
                                last_write_batch_operation_count,
                                BitcoinLevelDbTRandomizedBisectFailureMode::ModelPutStatusNotOk,
                            );

                        match model_snap.take() {
                            Some(snapshot) => {
                                model.release_snapshot(snapshot);
                            }
                            None => {}
                        }

                        match db_snap.take() {
                            Some(snapshot) => {
                                unsafe {
                                    (*dbtest.dbfull()).release_snapshot(snapshot);
                                }
                            }
                            None => {}
                        }

                        trace!(
                            target: "bitcoinleveldbt_randomized::randomized",
                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                            seed = seed,
                            option_configuration_index = option_configuration_index,
                            executed_prefix_length = executed_prefix_length,
                            failed = true
                        );

                        return Some(failure_observation);
                    }
                }

                let database_put_status = dbtest.put(&k, &v);

                match database_put_status.is_ok() {
                    true => {}
                    false => {
                        let failure_observation =
                            bitcoinleveldbt_randomized_bisect_build_failure_observation(
                                seed,
                                option_configuration_index,
                                option_configuration,
                                step,
                                executed_prefix_length,
                                last_operation_selector_probability,
                                last_operation_kind,
                                &last_key,
                                last_value_length_bytes,
                                last_write_batch_operation_count,
                                BitcoinLevelDbTRandomizedBisectFailureMode::DatabasePutStatusNotOk,
                            );

                        match model_snap.take() {
                            Some(snapshot) => {
                                model.release_snapshot(snapshot);
                            }
                            None => {}
                        }

                        match db_snap.take() {
                            Some(snapshot) => {
                                unsafe {
                                    (*dbtest.dbfull()).release_snapshot(snapshot);
                                }
                            }
                            None => {}
                        }

                        trace!(
                            target: "bitcoinleveldbt_randomized::randomized",
                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                            seed = seed,
                            option_configuration_index = option_configuration_index,
                            executed_prefix_length = executed_prefix_length,
                            failed = true
                        );

                        return Some(failure_observation);
                    }
                }
            }
            45u32..=89u32 => {
                last_operation_kind =
                    BitcoinLevelDbTRandomizedBisectOperationKindField::Delete;

                k = dbtest_random_key((&mut rnd) as *mut Random);

                last_key = k.clone();
                last_value_length_bytes = 0usize;

                let write_options = WriteOptions::default();
                let ks = Slice::from(&k);

                let model_delete_status = model.delete(&write_options, &ks);

                match model_delete_status.is_ok() {
                    true => {}
                    false => {
                        let failure_observation =
                            bitcoinleveldbt_randomized_bisect_build_failure_observation(
                                seed,
                                option_configuration_index,
                                option_configuration,
                                step,
                                executed_prefix_length,
                                last_operation_selector_probability,
                                last_operation_kind,
                                &last_key,
                                last_value_length_bytes,
                                last_write_batch_operation_count,
                                BitcoinLevelDbTRandomizedBisectFailureMode::ModelDeleteStatusNotOk,
                            );

                        match model_snap.take() {
                            Some(snapshot) => {
                                model.release_snapshot(snapshot);
                            }
                            None => {}
                        }

                        match db_snap.take() {
                            Some(snapshot) => {
                                unsafe {
                                    (*dbtest.dbfull()).release_snapshot(snapshot);
                                }
                            }
                            None => {}
                        }

                        trace!(
                            target: "bitcoinleveldbt_randomized::randomized",
                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                            seed = seed,
                            option_configuration_index = option_configuration_index,
                            executed_prefix_length = executed_prefix_length,
                            failed = true
                        );

                        return Some(failure_observation);
                    }
                }

                let database_delete_status = dbtest.delete(&k);

                match database_delete_status.is_ok() {
                    true => {}
                    false => {
                        let failure_observation =
                            bitcoinleveldbt_randomized_bisect_build_failure_observation(
                                seed,
                                option_configuration_index,
                                option_configuration,
                                step,
                                executed_prefix_length,
                                last_operation_selector_probability,
                                last_operation_kind,
                                &last_key,
                                last_value_length_bytes,
                                last_write_batch_operation_count,
                                BitcoinLevelDbTRandomizedBisectFailureMode::DatabaseDeleteStatusNotOk,
                            );

                        match model_snap.take() {
                            Some(snapshot) => {
                                model.release_snapshot(snapshot);
                            }
                            None => {}
                        }

                        match db_snap.take() {
                            Some(snapshot) => {
                                unsafe {
                                    (*dbtest.dbfull()).release_snapshot(snapshot);
                                }
                            }
                            None => {}
                        }

                        trace!(
                            target: "bitcoinleveldbt_randomized::randomized",
                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                            seed = seed,
                            option_configuration_index = option_configuration_index,
                            executed_prefix_length = executed_prefix_length,
                            failed = true
                        );

                        return Some(failure_observation);
                    }
                }
            }
            90u32..=99u32 => {
                last_operation_kind =
                    BitcoinLevelDbTRandomizedBisectOperationKindField::WriteBatch;

                let mut b = WriteBatch::default();
                let num = rnd.uniform(8) as i32;
                last_write_batch_operation_count = num;

                let mut i: i32 = 0i32;
                while i < num {
                    match i == 0i32 || !rnd.one_in(10) {
                        true => {
                            k = dbtest_random_key((&mut rnd) as *mut Random);
                        }
                        false => {
                            // Periodically re-use the same key from the previous iter, so
                            // we have multiple entries in the write batch for the same key
                        }
                    }

                    match rnd.one_in(2) {
                        true => {
                            v = dbtest_random_string(
                                (&mut rnd) as *mut Random,
                                rnd.uniform(10) as i32,
                            );
                            let ks = Slice::from(&k);
                            let vs = Slice::from(&v);
                            b.put(&ks, &vs);
                            last_value_length_bytes = v.len();
                        }
                        false => {
                            let ks = Slice::from(&k);
                            b.delete(&ks);
                            last_value_length_bytes = 0usize;
                        }
                    }

                    last_key = k.clone();

                    i += 1i32;
                }

                let write_options = WriteOptions::default();

                let model_batch_status =
                    model.write(&write_options, (&mut b) as *mut WriteBatch);

                match model_batch_status.is_ok() {
                    true => {}
                    false => {
                        let failure_observation =
                            bitcoinleveldbt_randomized_bisect_build_failure_observation(
                                seed,
                                option_configuration_index,
                                option_configuration,
                                step,
                                executed_prefix_length,
                                last_operation_selector_probability,
                                last_operation_kind,
                                &last_key,
                                last_value_length_bytes,
                                last_write_batch_operation_count,
                                BitcoinLevelDbTRandomizedBisectFailureMode::ModelWriteBatchStatusNotOk,
                            );

                        match model_snap.take() {
                            Some(snapshot) => {
                                model.release_snapshot(snapshot);
                            }
                            None => {}
                        }

                        match db_snap.take() {
                            Some(snapshot) => {
                                unsafe {
                                    (*dbtest.dbfull()).release_snapshot(snapshot);
                                }
                            }
                            None => {}
                        }

                        trace!(
                            target: "bitcoinleveldbt_randomized::randomized",
                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                            seed = seed,
                            option_configuration_index = option_configuration_index,
                            executed_prefix_length = executed_prefix_length,
                            failed = true
                        );

                        return Some(failure_observation);
                    }
                }

                let database_batch_status = unsafe {
                    (*dbtest.dbfull()).write(
                        &write_options,
                        (&mut b) as *mut WriteBatch,
                    )
                };

                match database_batch_status.is_ok() {
                    true => {}
                    false => {
                        let failure_observation =
                            bitcoinleveldbt_randomized_bisect_build_failure_observation(
                                seed,
                                option_configuration_index,
                                option_configuration,
                                step,
                                executed_prefix_length,
                                last_operation_selector_probability,
                                last_operation_kind,
                                &last_key,
                                last_value_length_bytes,
                                last_write_batch_operation_count,
                                BitcoinLevelDbTRandomizedBisectFailureMode::DatabaseWriteBatchStatusNotOk,
                            );

                        match model_snap.take() {
                            Some(snapshot) => {
                                model.release_snapshot(snapshot);
                            }
                            None => {}
                        }

                        match db_snap.take() {
                            Some(snapshot) => {
                                unsafe {
                                    (*dbtest.dbfull()).release_snapshot(snapshot);
                                }
                            }
                            None => {}
                        }

                        trace!(
                            target: "bitcoinleveldbt_randomized::randomized",
                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                            seed = seed,
                            option_configuration_index = option_configuration_index,
                            executed_prefix_length = executed_prefix_length,
                            failed = true
                        );

                        return Some(failure_observation);
                    }
                }
            }
            _ => {
                error!(
                    target: "bitcoinleveldbt_randomized::randomized",
                    event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_invalid_probability",
                    label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.invalid_probability",
                    p = p
                );
                panic!();
            }
        }

        match (step % 100i32) == 0i32 {
            true => {
                let model_ptr: *mut dyn DB =
                    (&mut model as *mut ModelDB) as *mut dyn DB;
                let db_ptr: *mut dyn DB =
                    dbtest.dbfull() as *mut dyn DB;

                let current_state_matches =
                    compare_iterators(step, model_ptr, db_ptr, None, None);

                match current_state_matches {
                    true => {}
                    false => {
                        let failure_observation =
                            bitcoinleveldbt_randomized_bisect_build_failure_observation(
                                seed,
                                option_configuration_index,
                                option_configuration,
                                step,
                                executed_prefix_length,
                                last_operation_selector_probability,
                                last_operation_kind,
                                &last_key,
                                last_value_length_bytes,
                                last_write_batch_operation_count,
                                BitcoinLevelDbTRandomizedBisectFailureMode::CompareIteratorsCurrentStateMismatch,
                            );

                        match model_snap.take() {
                            Some(snapshot) => {
                                model.release_snapshot(snapshot);
                            }
                            None => {}
                        }

                        match db_snap.take() {
                            Some(snapshot) => {
                                unsafe {
                                    (*dbtest.dbfull()).release_snapshot(snapshot);
                                }
                            }
                            None => {}
                        }

                        trace!(
                            target: "bitcoinleveldbt_randomized::randomized",
                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                            seed = seed,
                            option_configuration_index = option_configuration_index,
                            executed_prefix_length = executed_prefix_length,
                            failed = true
                        );

                        return Some(failure_observation);
                    }
                }

                let snapshot_state_matches =
                    compare_iterators(
                        step,
                        model_ptr,
                        db_ptr,
                        model_snap.as_deref(),
                        db_snap.as_deref(),
                    );

                match snapshot_state_matches {
                    true => {}
                    false => {
                        let failure_observation =
                            bitcoinleveldbt_randomized_bisect_build_failure_observation(
                                seed,
                                option_configuration_index,
                                option_configuration,
                                step,
                                executed_prefix_length,
                                last_operation_selector_probability,
                                last_operation_kind,
                                &last_key,
                                last_value_length_bytes,
                                last_write_batch_operation_count,
                                BitcoinLevelDbTRandomizedBisectFailureMode::CompareIteratorsSnapshotStateMismatch,
                            );

                        match model_snap.take() {
                            Some(snapshot) => {
                                model.release_snapshot(snapshot);
                            }
                            None => {}
                        }

                        match db_snap.take() {
                            Some(snapshot) => {
                                unsafe {
                                    (*dbtest.dbfull()).release_snapshot(snapshot);
                                }
                            }
                            None => {}
                        }

                        trace!(
                            target: "bitcoinleveldbt_randomized::randomized",
                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                            seed = seed,
                            option_configuration_index = option_configuration_index,
                            executed_prefix_length = executed_prefix_length,
                            failed = true
                        );

                        return Some(failure_observation);
                    }
                }

                match model_snap.take() {
                    Some(snapshot) => {
                        model.release_snapshot(snapshot);
                    }
                    None => {}
                }

                match db_snap.take() {
                    Some(snapshot) => {
                        unsafe {
                            (*dbtest.dbfull()).release_snapshot(snapshot);
                        }
                    }
                    None => {}
                }

                dbtest.reopen(None);

                let reopened_db_ptr: *mut dyn DB =
                    dbtest.dbfull() as *mut dyn DB;

                let reopened_state_matches =
                    compare_iterators(
                        step,
                        model_ptr,
                        reopened_db_ptr,
                        None,
                        None,
                    );

                match reopened_state_matches {
                    true => {}
                    false => {
                        let failure_observation =
                            bitcoinleveldbt_randomized_bisect_build_failure_observation(
                                seed,
                                option_configuration_index,
                                option_configuration,
                                step,
                                executed_prefix_length,
                                last_operation_selector_probability,
                                last_operation_kind,
                                &last_key,
                                last_value_length_bytes,
                                last_write_batch_operation_count,
                                BitcoinLevelDbTRandomizedBisectFailureMode::CompareIteratorsAfterReopenMismatch,
                            );

                        trace!(
                            target: "bitcoinleveldbt_randomized::randomized",
                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
                            seed = seed,
                            option_configuration_index = option_configuration_index,
                            executed_prefix_length = executed_prefix_length,
                            failed = true
                        );

                        return Some(failure_observation);
                    }
                }

                model_snap = Some(model.get_snapshot());
                db_snap = Some(unsafe { (*dbtest.dbfull()).get_snapshot() });
            }
            false => {}
        }

        step += 1i32;
    }

    match model_snap.take() {
        Some(snapshot) => {
            model.release_snapshot(snapshot);
        }
        None => {}
    }

    match db_snap.take() {
        Some(snapshot) => {
            unsafe {
                (*dbtest.dbfull()).release_snapshot(snapshot);
            }
        }
        None => {}
    }

    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",
        label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",
        seed = seed,
        option_configuration_index = option_configuration_index,
        executed_prefix_length = executed_prefix_length,
        failed = false
    );

    None

}

fn bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure(
    seed: u32,
    executed_prefix_length: i32,
) -> Option<BitcoinLevelDbTRandomizedBisectedFailureObservation> {
    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure_entry",
        label = "bitcoinleveldbt_randomized.run_across_option_configurations_and_return_first_failure.entry",
        seed = seed,
        executed_prefix_length = executed_prefix_length
    );

    let mut option_configuration_index: i32 = 0i32;

    loop {
        let option_configuration =
            bitcoinleveldbt_randomized_bisect_option_configuration_from_index(
                option_configuration_index,
            );

        match option_configuration {
            DBTestOptionConfig::End => {
                trace!(
                    target: "bitcoinleveldbt_randomized::randomized",
                    event = "bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure_exit",
                    label = "bitcoinleveldbt_randomized.run_across_option_configurations_and_return_first_failure.exit",
                    seed = seed,
                    executed_prefix_length = executed_prefix_length,
                    failed = false
                );

                return None;
            }
            DBTestOptionConfig::Default
                | DBTestOptionConfig::Reuse
                | DBTestOptionConfig::Filter
                | DBTestOptionConfig::Uncompressed => {
                    info!(
                        target: "bitcoinleveldbt_randomized::randomized",
                        event = "bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure_decision",
                        label = "bitcoinleveldbt_randomized.run_across_option_configurations_and_return_first_failure.execute_option_configuration",
                        seed = seed,
                        executed_prefix_length = executed_prefix_length,
                        option_configuration_index = option_configuration_index,
                        option_configuration = ?option_configuration
                    );

                    let maybe_failure =
                        bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure(
                            seed,
                            option_configuration_index,
                            executed_prefix_length,
                        );

                    match maybe_failure {
                        Some(failure_observation) => {
                            trace!(
                                target: "bitcoinleveldbt_randomized::randomized",
                                event = "bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure_exit",
                                label = "bitcoinleveldbt_randomized.run_across_option_configurations_and_return_first_failure.exit",
                                seed = seed,
                                executed_prefix_length = executed_prefix_length,
                                failed = true
                            );

                            return Some(failure_observation);
                        }
                        None => {
                            option_configuration_index += 1i32;
                        }
                    }
                }
        }
    }

}

fn bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration(
    seed: u32,
    option_configuration_index: i32,
    upper_bound_prefix_length: i32,
) -> Option<BitcoinLevelDbTRandomizedBisectedFailureObservation> {
    trace!(
        target: "bitcoinleveldbt_randomized::randomized",
        event = "bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration_entry",
        label = "bitcoinleveldbt_randomized.bisect.minimal_failing_prefix_for_option_configuration.entry",
        seed = seed,
        option_configuration_index = option_configuration_index,
        upper_bound_prefix_length = upper_bound_prefix_length
    );

    let initial_failure =
        bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure(
            seed,
            option_configuration_index,
            upper_bound_prefix_length,
        );

    match initial_failure {
        None => {
            trace!(
                target: "bitcoinleveldbt_randomized::randomized",
                event = "bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration_exit",
                label = "bitcoinleveldbt_randomized.bisect.minimal_failing_prefix_for_option_configuration.exit",
                seed = seed,
                option_configuration_index = option_configuration_index,
                upper_bound_prefix_length = upper_bound_prefix_length,
                failed = false
            );

            None
        }
        Some(mut best_failure_observation) => {
            let mut low_prefix_length: i32 = 1i32;
            let mut high_prefix_length: i32 = upper_bound_prefix_length;

            while low_prefix_length < high_prefix_length {
                let mid_prefix_length =
                    low_prefix_length
                    + ((high_prefix_length - low_prefix_length) / 2i32);

                eprintln!(
                    "RANDOMIZED_BISECT_PROGRESS seed={} option_index={} low={} mid={} high={}",
                    seed,
                    option_configuration_index,
                    low_prefix_length,
                    mid_prefix_length,
                    high_prefix_length,
                );

                let mid_failure =
                    bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure(
                        seed,
                        option_configuration_index,
                        mid_prefix_length,
                    );

                match mid_failure {
                    Some(failure_observation) => {
                        best_failure_observation = failure_observation;
                        high_prefix_length = mid_prefix_length;
                    }
                    None => {
                        low_prefix_length = mid_prefix_length + 1i32;
                    }
                }
            }

            trace!(
                target: "bitcoinleveldbt_randomized::randomized",
                event = "bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration_exit",
                label = "bitcoinleveldbt_randomized.bisect.minimal_failing_prefix_for_option_configuration.exit",
                seed = seed,
                option_configuration_index = option_configuration_index,
                upper_bound_prefix_length = upper_bound_prefix_length,
                failed = true,
                failing_step_index = *best_failure_observation.failing_step_index(),
                executed_prefix_length = *best_failure_observation.executed_prefix_length()
            );

            Some(best_failure_observation)
        }
    }

}

#[traced_test]
fn db_test_bisect_randomized() {
    let seed = random_seed() as u32;
    let executed_prefix_length: i32 = 10000i32;

    let first_failure_observation =
        bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure(
            seed,
            executed_prefix_length,
        );

    match first_failure_observation {
        None => {}
        Some(first_failure) => {
            let bisected_failure =
                bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration(
                    *first_failure.seed(),
                    *first_failure.option_configuration_index(),
                    executed_prefix_length,
                );

            match bisected_failure {
                Some(minimal_failure) => {
                    eprintln!(
                        "RANDOMIZED_BISECT_RESULT seed={} option_index={} option_config={:?} failing_step={} prefix_length={} failure_mode={:?} operation_kind={:?} branch_p={} key={} value_len={} batch_ops={}",
                        *minimal_failure.seed(),
                        *minimal_failure.option_configuration_index(),
                        *minimal_failure.option_configuration(),
                        *minimal_failure.failing_step_index(),
                        *minimal_failure.executed_prefix_length(),
                        *minimal_failure.failure_mode(),
                        *minimal_failure.operation_kind(),
                        *minimal_failure.operation_selector_probability(),
                        minimal_failure.last_key(),
                        *minimal_failure.last_value_length_bytes(),
                        *minimal_failure.last_write_batch_operation_count(),
                    );

                    panic!();
                }
                None => {
                    eprintln!(
                        "RANDOMIZED_BISECT_RESULT seed={} option_index={} option_config={:?} inconsistency=full_run_failed_but_bisection_found_no_failing_prefix",
                        *first_failure.seed(),
                        *first_failure.option_configuration_index(),
                        *first_failure.option_configuration(),
                    );

                    panic!();
                }
            }
        }
    }

}
