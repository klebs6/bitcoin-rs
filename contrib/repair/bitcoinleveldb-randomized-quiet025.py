#!/usr/bin/env python3
"""Independent quiet recovery repair and twelve fresh guarded randomized cases; retain the failed023 journal and all limits. Master is not promoted."""
import argparse
import datetime as dt
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shlex
import signal
import stat
import subprocess
import sys
import types
import uuid
MIB023=1024*1024
CONTROLLER_REVISION=1
WORKFLOW='leveldb-randomized-quiet025'
BASE='904156f818d38b443f02ff8fba6f2a2bfd5ed8a7'
BASE_TREE='82ba98d00b662dc309447d13cd1bb366a0b39089'
BRANCH='repair/leveldb-randomized-quiet025'
SUFFIX='-leveldb-randomized-quiet025'
REPLAY_PATH='contrib/repair/bitcoinleveldb-randomized-quiet025.py'
DOCUMENT_PATH='doc/testing/leveldb-randomized-quiet025.md'
RECEIPT_PATH='doc/repair/leveldb-randomized-quiet025.json'
PRIOR020_RUN='20261108T074029Z-db4c138d'
PRIOR020_SHA='5a18ae65a4889f8eeb4c0a100aa85fb22092ea7df1b5c6a5c4c3383a1c4eff7d'
PRIOR020_BYTES=207952
INTAKE_RUN='20261108T143903Z-74099602'
INTAKE_CONTROLLER_SHA='67adc16ef92827a19ebaacdcae7939da63e510cc507b55aeef26c0c38eee6dae'
INTAKE_REPORT_SHA='53889a8cc00418f6c4a836be87dcd9f881b5268e953407858451539723acac63'
INTAKE_REPORT_BYTES=819777
COMMIT_MESSAGE=b'test: quiet duplicate recovery diagnostics for controlled randomized runs\n\nRetain structured tracing, original operation streams and finite resource\ncontrols; qualify both full original workloads on an isolated candidate.\n'


SOURCE_PAYLOADS={'bitcoinleveldbt-randomized/tests/randomized.rs': b'// ---------------- [ File: bitcoinleveldbt-randomized/tests/randomized.rs ]\nuse traced_test::*;\nuse tracing_setup::*;\nuse bitcoinleveldb_rand::*;\nuse bitcoinleveldbt_dbtest::*;\nuse bitcoinleveldbt_util::*;\nuse bitcoinleveldbt_snapshot::*;\nuse bitcoinleveldb_snapshot::*;\nuse bitcoinleveldb_options::*;\nuse bitcoinleveldb_slice::*;\nuse bitcoinleveldb_batch::*;\nuse bitcoinleveldb_dbinterface::*;\nuse bitcoinleveldb_modeldb::*;\n\n#[path = "support/randomized_operations.rs"]\nmod randomized_operations;\n#[path = "support/execution_controls.rs"]\nmod execution_controls;\nuse randomized_operations::{BatchMutation, Mutation, OperationGenerator, LEGACY_STEPS_PER_CONFIGURATION};\n\n#[test]\n#[ignore = "long stress; use a resource-controlled runner explicitly"]\nfn db_test_randomized() {\n    execution_controls::require_stress_protection("db_test_randomized");\n    let mut operations = OperationGenerator::new(bitcoinleveldbt_util::random_seed() as u32);\n\n    let mut completed_configurations: usize = 0;\n    let mut completed_steps: usize = 0;\n    let mut body = |dbtest: &mut DBTest| {\n        let configuration = completed_configurations;\n        execution_controls::progress("db_test_randomized", configuration, 0, LEGACY_STEPS_PER_CONFIGURATION, "configuration.entry");\n        operations.begin_configuration();\n        let opts = dbtest.current_options();\n        let mut model = ModelDB::new(&opts);\n        const N: i32 = LEGACY_STEPS_PER_CONFIGURATION as i32;\n\n        let mut model_snap: Option<Box<dyn Snapshot>> = None;\n        let mut db_snap = execution_controls::DatabaseSnapshotLease::new();\n\n        let mut step: i32 = 0;\n        while step < N {\n            if step % 100 == 0 {\n                execution_controls::progress("db_test_randomized", configuration, step as usize, N as usize, "iteration");\n            }\n\n            // TODO(sanjay): Test Get() works\n            let operation = operations.next_operation();\n            match operation.mutation {\n                Mutation::Put { key: k, value: v } => {\n                    let write_options = WriteOptions::default();\n                    let ks = Slice::from(&k);\n                    let vs = Slice::from(&v);\n                    assert!(model.put(&write_options, &ks, &vs).is_ok());\n                    assert!(dbtest.put(&k, &v).is_ok());\n                }\n                Mutation::Delete { key: k } => {\n                    let write_options = WriteOptions::default();\n                    let ks = Slice::from(&k);\n                    assert!(model.delete(&write_options, &ks).is_ok());\n                    assert!(dbtest.delete(&k).is_ok());\n                }\n                Mutation::Batch(updates) => {\n                    let mut b = WriteBatch::default();\n                    for update in updates {\n                        match update {\n                            BatchMutation::Put { key, value } => {\n                                b.put(&Slice::from(&key), &Slice::from(&value));\n                            }\n                            BatchMutation::Delete { key } => {\n                                b.delete(&Slice::from(&key));\n                            }\n                        }\n                    }\n                    let write_options = WriteOptions::default();\n                    assert!(model.write(&write_options, (&mut b) as *mut WriteBatch).is_ok());\n                    assert!(unsafe {\n                        (*dbtest.dbfull()).write(&write_options, (&mut b) as *mut WriteBatch)\n                    }.is_ok());\n                }\n            }\n\n            if (step % 100) == 0 {\n                let model_ptr: *mut dyn DB = (&mut model as *mut ModelDB) as *mut dyn DB;\n                let db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;\n\n                assert!(compare_iterators(step, model_ptr, db_ptr, None, None));\n                assert!(compare_iterators(\n                    step,\n                    model_ptr,\n                    db_ptr,\n                    model_snap.as_deref(),\n                    db_snap.as_deref(),\n                ));\n\n                // Save a snapshot from each DB this time that we\'ll use next\n                // time we compare things, to make sure the current state is\n                // preserved with the snapshot\n                if let Some(snapshot) = model_snap.take() {\n                    model.release_snapshot(snapshot);\n                }\n                if let Some(snapshot) = db_snap.take() {\n                    unsafe {\n                        (*dbtest.dbfull()).release_snapshot(snapshot);\n                    }\n                }\n\n                dbtest.reopen(None);\n\n                let reopened_db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;\n                assert!(compare_iterators(step, model_ptr, reopened_db_ptr, None, None));\n\n                model_snap = Some(model.get_snapshot());\n                unsafe { db_snap.acquire(dbtest.dbfull() as *mut dyn DB); }\n            }\n\n            step += 1;\n        }\n\n        if let Some(snapshot) = model_snap.take() {\n            model.release_snapshot(snapshot);\n        }\n        if let Some(snapshot) = db_snap.take() {\n            unsafe {\n                (*dbtest.dbfull()).release_snapshot(snapshot);\n            }\n        }\n        completed_configurations += 1;\n        completed_steps += step as usize;\n        execution_controls::progress("db_test_randomized", configuration, step as usize, N as usize, "configuration.return");\n    };\n\n    dbtest_fixture_run_across_option_configurations(&mut body);\n    // The fixture and its database have returned from teardown before success.\n    execution_controls::complete("db_test_randomized", completed_configurations, completed_steps);\n}\n\n', 'bitcoinleveldbt-randomized/tests/bisect.rs': b'// ---------------- [ File: bitcoinleveldbt-randomized/tests/bisect.rs ]\nuse traced_test::*;\nuse tracing_setup::*;\nuse bitcoinleveldb_rand::*;\nuse bitcoin_imports::*;\nuse bitcoinleveldbt_dbtest::*;\nuse bitcoinleveldbt_util::*;\nuse bitcoinleveldbt_snapshot::*;\nuse bitcoinleveldb_snapshot::*;\nuse bitcoinleveldb_options::*;\nuse bitcoinleveldb_slice::*;\nuse bitcoinleveldb_batch::*;\nuse bitcoinleveldb_dbinterface::*;\nuse bitcoinleveldb_modeldb::*;\n\n\n#[path = "support/randomized_operations.rs"]\nmod randomized_operations;\n#[path = "support/execution_controls.rs"]\nmod execution_controls;\nuse randomized_operations::{BatchMutation, Mutation, OperationGenerator, LEGACY_STEPS_PER_CONFIGURATION, OPTION_CONFIGURATION_COUNT};\n\n#[derive(Clone, Copy, Debug, PartialEq, Eq, Getters, Builder)]\n#[getset(get = "pub")]\n#[builder(pattern = "owned")]\nstruct BitcoinLevelDbTRandomizedBisectOperationKind {\n#[builder(default = "BitcoinLevelDbTRandomizedBisectOperationKindField::NoneYet")]\n    field: BitcoinLevelDbTRandomizedBisectOperationKindField,\n}\n\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\nenum BitcoinLevelDbTRandomizedBisectOperationKindField {\n    NoneYet,\n    Put,\n    Delete,\n    WriteBatch,\n}\n\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\nenum BitcoinLevelDbTRandomizedBisectFailureMode {\n    OptionConfigurationUnavailable,\n    ModelPutStatusNotOk,\n    DatabasePutStatusNotOk,\n    ModelDeleteStatusNotOk,\n    DatabaseDeleteStatusNotOk,\n    ModelWriteBatchStatusNotOk,\n    DatabaseWriteBatchStatusNotOk,\n    CompareIteratorsCurrentStateMismatch,\n    CompareIteratorsSnapshotStateMismatch,\n    CompareIteratorsAfterReopenMismatch,\n}\n\n#[derive(Clone, Debug, Getters, Builder)]\n#[getset(get = "pub")]\n#[builder(pattern = "owned")]\nstruct BitcoinLevelDbTRandomizedBisectedFailureObservation {\n    /// This is the deterministic PRNG seed for the prefix run.\n    /// Re-running with the same seed and step limit must preserve the generated operation stream.\n    seed: u32,\n    /// This is the zero-based option-configuration ordinal used by the test fixture.\n    /// The ordinal must remain stable across reruns of the same workspace revision.\n    option_configuration_index: i32,\n    /// This is the resolved option-configuration discriminator for the ordinal.\n    /// It records which DBTest configuration surface was active when the failure was observed.\n    option_configuration: DBTestOptionConfig,\n    /// This is the zero-based logical step at which the first observed failure occurred.\n    /// The step index must refer to the first failing prefix, not a later derived mismatch.\n    failing_step_index: i32,\n    /// This is the executed prefix length used for the run that produced this observation.\n    /// Generated operation prefixes are stable at the selected configuration.\n    /// Repeatable and monotone database failures remain assumptions: background\n    /// work can change execution even when the operation stream is identical.\n    executed_prefix_length: i32,\n    /// This is the random branch selector `p` for the failing step.\n    /// It preserves the exact branch decision that classified the step.\n    operation_selector_probability: u32,\n    /// This is the logical operation family active at the failing step.\n    /// It must identify the last mutating action applied before the mismatch was detected.\n    operation_kind: BitcoinLevelDbTRandomizedBisectOperationKindField,\n    /// This is the last logical key chosen by the generator for the failing step.\n    /// It is observational only and must not be normalized or rewritten.\n    last_key: String,\n    /// This is the byte length of the last logical value chosen by the generator for the failing step.\n    /// For delete-only steps this must remain zero.\n    last_value_length_bytes: usize,\n    /// This is the number of per-key updates encoded into the last write batch.\n    /// For non-batch steps this must remain zero.\n    last_write_batch_operation_count: i32,\n    /// This is the first observed failure class for the prefix.\n    /// Repeated database runs can observe different classes when background work\n    /// changes execution; matching operation streams alone do not rule that out.\n    failure_mode: BitcoinLevelDbTRandomizedBisectFailureMode,\n}\n\nfn bitcoinleveldbt_randomized_bisect_option_configuration_from_index(\n    option_configuration_index: i32,\n) -> DBTestOptionConfig {\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_bisect_option_configuration_from_index_entry",\n        label = "bitcoinleveldbt_randomized.bisect.option_configuration_from_index.entry",\n        option_configuration_index = option_configuration_index\n    );\n\n    let option_configuration = match option_configuration_index {\n        0i32 => DBTestOptionConfig::Default,\n        1i32 => DBTestOptionConfig::Reuse,\n        2i32 => DBTestOptionConfig::Filter,\n        3i32 => DBTestOptionConfig::Uncompressed,\n        _ => DBTestOptionConfig::End,\n    };\n\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_bisect_option_configuration_from_index_exit",\n        label = "bitcoinleveldbt_randomized.bisect.option_configuration_from_index.exit",\n        option_configuration_index = option_configuration_index,\n        option_configuration = ?option_configuration\n    );\n\n    option_configuration\n\n}\n\nfn bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration(\n    dbtest: &mut DBTest,\n    option_configuration_index: i32,\n) -> bool {\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_entry",\n        label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.entry",\n        option_configuration_index = option_configuration_index\n    );\n\n    let mut current_option_configuration_index: i32 = 0i32;\n\n    while current_option_configuration_index < option_configuration_index {\n        let advanced = dbtest.change_options();\n\n        match advanced {\n            true => {\n                current_option_configuration_index += 1i32;\n\n                debug!(\n                    target: "bitcoinleveldbt_randomized::randomized",\n                    event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_state_transition",\n                    label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.state_transition",\n                    current_option_configuration_index = current_option_configuration_index,\n                    option_configuration_index = option_configuration_index\n                );\n            }\n            false => {\n                warn!(\n                    target: "bitcoinleveldbt_randomized::randomized",\n                    event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_unavailable",\n                    label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.unavailable",\n                    current_option_configuration_index = current_option_configuration_index,\n                    option_configuration_index = option_configuration_index\n                );\n\n                trace!(\n                    target: "bitcoinleveldbt_randomized::randomized",\n                    event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_exit",\n                    label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.exit",\n                    option_configuration_index = option_configuration_index,\n                    available = false\n                );\n\n                return false;\n            }\n        }\n    }\n\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration_exit",\n        label = "bitcoinleveldbt_randomized.bisect.advance_fixture_to_option_configuration.exit",\n        option_configuration_index = option_configuration_index,\n        available = true\n    );\n\n    true\n\n}\n\nfn bitcoinleveldbt_randomized_bisect_build_failure_observation(\n    seed: u32,\n    option_configuration_index: i32,\n    option_configuration: DBTestOptionConfig,\n    failing_step_index: i32,\n    executed_prefix_length: i32,\n    operation_selector_probability: u32,\n    operation_kind: BitcoinLevelDbTRandomizedBisectOperationKindField,\n    last_key: &String,\n    last_value_length_bytes: usize,\n    last_write_batch_operation_count: i32,\n    failure_mode: BitcoinLevelDbTRandomizedBisectFailureMode,\n) -> BitcoinLevelDbTRandomizedBisectedFailureObservation {\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_bisect_build_failure_observation_entry",\n        label = "bitcoinleveldbt_randomized.bisect.build_failure_observation.entry",\n        seed = seed,\n        option_configuration_index = option_configuration_index,\n        option_configuration = ?option_configuration,\n        failing_step_index = failing_step_index,\n        executed_prefix_length = executed_prefix_length,\n        operation_selector_probability = operation_selector_probability,\n        operation_kind = ?operation_kind,\n        last_value_length_bytes = last_value_length_bytes,\n        last_write_batch_operation_count = last_write_batch_operation_count,\n        failure_mode = ?failure_mode\n    );\n\n    let observation_builder =\n        BitcoinLevelDbTRandomizedBisectedFailureObservationBuilder::default()\n        .seed(seed)\n        .option_configuration_index(option_configuration_index)\n        .option_configuration(option_configuration)\n        .failing_step_index(failing_step_index)\n        .executed_prefix_length(executed_prefix_length)\n        .operation_selector_probability(operation_selector_probability)\n        .operation_kind(operation_kind)\n        .last_key(last_key.clone())\n        .last_value_length_bytes(last_value_length_bytes)\n        .last_write_batch_operation_count(last_write_batch_operation_count)\n        .failure_mode(failure_mode);\n\n    let observation = match observation_builder.build() {\n        Ok(value) => value,\n        Err(builder_error) => {\n            error!(\n                target: "bitcoinleveldbt_randomized::randomized",\n                event = "bitcoinleveldbt_randomized_bisect_build_failure_observation_builder_failed",\n                label = "bitcoinleveldbt_randomized.bisect.build_failure_observation.builder_failed",\n                builder_error = ?builder_error\n            );\n            panic!();\n        }\n    };\n\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_bisect_build_failure_observation_exit",\n        label = "bitcoinleveldbt_randomized.bisect.build_failure_observation.exit",\n        seed = *observation.seed(),\n        option_configuration_index = *observation.option_configuration_index(),\n        failing_step_index = *observation.failing_step_index(),\n        failure_mode = ?observation.failure_mode()\n    );\n\n    observation\n\n}\n\nfn bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure(\n    seed: u32,\n    option_configuration_index: i32,\n    executed_prefix_length: i32,\n) -> Option<BitcoinLevelDbTRandomizedBisectedFailureObservation> {\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_entry",\n        label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.entry",\n        seed = seed,\n        option_configuration_index = option_configuration_index,\n        executed_prefix_length = executed_prefix_length\n    );\n\n    let option_configuration =\n        bitcoinleveldbt_randomized_bisect_option_configuration_from_index(\n            option_configuration_index,\n        );\n\n    match option_configuration {\n        DBTestOptionConfig::End => {\n            let failure_observation =\n                bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                    seed,\n                    option_configuration_index,\n                    option_configuration,\n                    0i32,\n                    executed_prefix_length,\n                    0u32,\n                    BitcoinLevelDbTRandomizedBisectOperationKindField::NoneYet,\n                    &String::new(),\n                    0usize,\n                    0i32,\n                    BitcoinLevelDbTRandomizedBisectFailureMode::OptionConfigurationUnavailable,\n                );\n\n            trace!(\n                target: "bitcoinleveldbt_randomized::randomized",\n                event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                seed = seed,\n                option_configuration_index = option_configuration_index,\n                executed_prefix_length = executed_prefix_length,\n                failed = true\n            );\n\n            return Some(failure_observation);\n        }\n        DBTestOptionConfig::Default\n            | DBTestOptionConfig::Reuse\n            | DBTestOptionConfig::Filter\n            | DBTestOptionConfig::Uncompressed => {}\n    }\n\n    let mut operations = OperationGenerator::for_configuration(\n        seed,\n        option_configuration_index as usize,\n    );\n    let mut dbtest = DBTest::default();\n\n    let option_configuration_available =\n        bitcoinleveldbt_randomized_bisect_advance_fixture_to_option_configuration(\n            &mut dbtest,\n            option_configuration_index,\n        );\n\n    match option_configuration_available {\n        true => {}\n        false => {\n            let failure_observation =\n                bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                    seed,\n                    option_configuration_index,\n                    option_configuration,\n                    0i32,\n                    executed_prefix_length,\n                    0u32,\n                    BitcoinLevelDbTRandomizedBisectOperationKindField::NoneYet,\n                    &String::new(),\n                    0usize,\n                    0i32,\n                    BitcoinLevelDbTRandomizedBisectFailureMode::OptionConfigurationUnavailable,\n                );\n\n            trace!(\n                target: "bitcoinleveldbt_randomized::randomized",\n                event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                seed = seed,\n                option_configuration_index = option_configuration_index,\n                executed_prefix_length = executed_prefix_length,\n                failed = true\n            );\n\n            return Some(failure_observation);\n        }\n    }\n\n    let opts = dbtest.current_options();\n    let mut model = ModelDB::new(&opts);\n\n    let mut model_snap: Option<Box<dyn Snapshot>> = None;\n    let mut db_snap = execution_controls::DatabaseSnapshotLease::new();\n\n    let mut last_key = String::new();\n    let mut last_value_length_bytes: usize = 0usize;\n    let mut last_write_batch_operation_count: i32 = 0i32;\n    let mut last_operation_kind =\n        BitcoinLevelDbTRandomizedBisectOperationKindField::NoneYet;\n    let mut last_operation_selector_probability: u32 = 0u32;\n\n    execution_controls::progress("db_test_bisect_randomized", option_configuration_index as usize, 0, executed_prefix_length as usize, "configuration.entry");\n    let mut step: i32 = 0i32;\n    while step < executed_prefix_length {\n        if step % 100 == 0 {\n            execution_controls::progress("db_test_bisect_randomized", option_configuration_index as usize, step as usize, executed_prefix_length as usize, "iteration");\n        }\n        let operation = operations.next_operation();\n        last_operation_selector_probability = operation.selector;\n        last_key = operation.last_key.clone();\n        last_value_length_bytes = operation.last_value_length_bytes;\n        last_write_batch_operation_count = 0i32;\n\n        match operation.mutation {\n            Mutation::Put { key: k, value: v } => {\n                last_operation_kind =\n                    BitcoinLevelDbTRandomizedBisectOperationKindField::Put;\n\n                let write_options = WriteOptions::default();\n                let ks = Slice::from(&k);\n                let vs = Slice::from(&v);\n\n                let model_put_status = model.put(&write_options, &ks, &vs);\n\n                match model_put_status.is_ok() {\n                    true => {}\n                    false => {\n                        let failure_observation =\n                            bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                                seed,\n                                option_configuration_index,\n                                option_configuration,\n                                step,\n                                executed_prefix_length,\n                                last_operation_selector_probability,\n                                last_operation_kind,\n                                &last_key,\n                                last_value_length_bytes,\n                                last_write_batch_operation_count,\n                                BitcoinLevelDbTRandomizedBisectFailureMode::ModelPutStatusNotOk,\n                            );\n\n                        match model_snap.take() {\n                            Some(snapshot) => {\n                                model.release_snapshot(snapshot);\n                            }\n                            None => {}\n                        }\n\n                        match db_snap.take() {\n                            Some(snapshot) => {\n                                unsafe {\n                                    (*dbtest.dbfull()).release_snapshot(snapshot);\n                                }\n                            }\n                            None => {}\n                        }\n\n                        trace!(\n                            target: "bitcoinleveldbt_randomized::randomized",\n                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                            seed = seed,\n                            option_configuration_index = option_configuration_index,\n                            executed_prefix_length = executed_prefix_length,\n                            failed = true\n                        );\n\n                        return Some(failure_observation);\n                    }\n                }\n\n                let database_put_status = dbtest.put(&k, &v);\n\n                match database_put_status.is_ok() {\n                    true => {}\n                    false => {\n                        let failure_observation =\n                            bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                                seed,\n                                option_configuration_index,\n                                option_configuration,\n                                step,\n                                executed_prefix_length,\n                                last_operation_selector_probability,\n                                last_operation_kind,\n                                &last_key,\n                                last_value_length_bytes,\n                                last_write_batch_operation_count,\n                                BitcoinLevelDbTRandomizedBisectFailureMode::DatabasePutStatusNotOk,\n                            );\n\n                        match model_snap.take() {\n                            Some(snapshot) => {\n                                model.release_snapshot(snapshot);\n                            }\n                            None => {}\n                        }\n\n                        match db_snap.take() {\n                            Some(snapshot) => {\n                                unsafe {\n                                    (*dbtest.dbfull()).release_snapshot(snapshot);\n                                }\n                            }\n                            None => {}\n                        }\n\n                        trace!(\n                            target: "bitcoinleveldbt_randomized::randomized",\n                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                            seed = seed,\n                            option_configuration_index = option_configuration_index,\n                            executed_prefix_length = executed_prefix_length,\n                            failed = true\n                        );\n\n                        return Some(failure_observation);\n                    }\n                }\n            }\n            Mutation::Delete { key: k } => {\n                last_operation_kind =\n                    BitcoinLevelDbTRandomizedBisectOperationKindField::Delete;\n\n                let write_options = WriteOptions::default();\n                let ks = Slice::from(&k);\n\n                let model_delete_status = model.delete(&write_options, &ks);\n\n                match model_delete_status.is_ok() {\n                    true => {}\n                    false => {\n                        let failure_observation =\n                            bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                                seed,\n                                option_configuration_index,\n                                option_configuration,\n                                step,\n                                executed_prefix_length,\n                                last_operation_selector_probability,\n                                last_operation_kind,\n                                &last_key,\n                                last_value_length_bytes,\n                                last_write_batch_operation_count,\n                                BitcoinLevelDbTRandomizedBisectFailureMode::ModelDeleteStatusNotOk,\n                            );\n\n                        match model_snap.take() {\n                            Some(snapshot) => {\n                                model.release_snapshot(snapshot);\n                            }\n                            None => {}\n                        }\n\n                        match db_snap.take() {\n                            Some(snapshot) => {\n                                unsafe {\n                                    (*dbtest.dbfull()).release_snapshot(snapshot);\n                                }\n                            }\n                            None => {}\n                        }\n\n                        trace!(\n                            target: "bitcoinleveldbt_randomized::randomized",\n                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                            seed = seed,\n                            option_configuration_index = option_configuration_index,\n                            executed_prefix_length = executed_prefix_length,\n                            failed = true\n                        );\n\n                        return Some(failure_observation);\n                    }\n                }\n\n                let database_delete_status = dbtest.delete(&k);\n\n                match database_delete_status.is_ok() {\n                    true => {}\n                    false => {\n                        let failure_observation =\n                            bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                                seed,\n                                option_configuration_index,\n                                option_configuration,\n                                step,\n                                executed_prefix_length,\n                                last_operation_selector_probability,\n                                last_operation_kind,\n                                &last_key,\n                                last_value_length_bytes,\n                                last_write_batch_operation_count,\n                                BitcoinLevelDbTRandomizedBisectFailureMode::DatabaseDeleteStatusNotOk,\n                            );\n\n                        match model_snap.take() {\n                            Some(snapshot) => {\n                                model.release_snapshot(snapshot);\n                            }\n                            None => {}\n                        }\n\n                        match db_snap.take() {\n                            Some(snapshot) => {\n                                unsafe {\n                                    (*dbtest.dbfull()).release_snapshot(snapshot);\n                                }\n                            }\n                            None => {}\n                        }\n\n                        trace!(\n                            target: "bitcoinleveldbt_randomized::randomized",\n                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                            seed = seed,\n                            option_configuration_index = option_configuration_index,\n                            executed_prefix_length = executed_prefix_length,\n                            failed = true\n                        );\n\n                        return Some(failure_observation);\n                    }\n                }\n            }\n            Mutation::Batch(updates) => {\n                last_operation_kind =\n                    BitcoinLevelDbTRandomizedBisectOperationKindField::WriteBatch;\n\n                let mut b = WriteBatch::default();\n                last_write_batch_operation_count = updates.len() as i32;\n                for update in updates {\n                    match update {\n                        BatchMutation::Put { key, value } => {\n                            b.put(&Slice::from(&key), &Slice::from(&value));\n                        }\n                        BatchMutation::Delete { key } => {\n                            b.delete(&Slice::from(&key));\n                        }\n                    }\n                }\n\n                let write_options = WriteOptions::default();\n\n                let model_batch_status =\n                    model.write(&write_options, (&mut b) as *mut WriteBatch);\n\n                match model_batch_status.is_ok() {\n                    true => {}\n                    false => {\n                        let failure_observation =\n                            bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                                seed,\n                                option_configuration_index,\n                                option_configuration,\n                                step,\n                                executed_prefix_length,\n                                last_operation_selector_probability,\n                                last_operation_kind,\n                                &last_key,\n                                last_value_length_bytes,\n                                last_write_batch_operation_count,\n                                BitcoinLevelDbTRandomizedBisectFailureMode::ModelWriteBatchStatusNotOk,\n                            );\n\n                        match model_snap.take() {\n                            Some(snapshot) => {\n                                model.release_snapshot(snapshot);\n                            }\n                            None => {}\n                        }\n\n                        match db_snap.take() {\n                            Some(snapshot) => {\n                                unsafe {\n                                    (*dbtest.dbfull()).release_snapshot(snapshot);\n                                }\n                            }\n                            None => {}\n                        }\n\n                        trace!(\n                            target: "bitcoinleveldbt_randomized::randomized",\n                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                            seed = seed,\n                            option_configuration_index = option_configuration_index,\n                            executed_prefix_length = executed_prefix_length,\n                            failed = true\n                        );\n\n                        return Some(failure_observation);\n                    }\n                }\n\n                let database_batch_status = unsafe {\n                    (*dbtest.dbfull()).write(\n                        &write_options,\n                        (&mut b) as *mut WriteBatch,\n                    )\n                };\n\n                match database_batch_status.is_ok() {\n                    true => {}\n                    false => {\n                        let failure_observation =\n                            bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                                seed,\n                                option_configuration_index,\n                                option_configuration,\n                                step,\n                                executed_prefix_length,\n                                last_operation_selector_probability,\n                                last_operation_kind,\n                                &last_key,\n                                last_value_length_bytes,\n                                last_write_batch_operation_count,\n                                BitcoinLevelDbTRandomizedBisectFailureMode::DatabaseWriteBatchStatusNotOk,\n                            );\n\n                        match model_snap.take() {\n                            Some(snapshot) => {\n                                model.release_snapshot(snapshot);\n                            }\n                            None => {}\n                        }\n\n                        match db_snap.take() {\n                            Some(snapshot) => {\n                                unsafe {\n                                    (*dbtest.dbfull()).release_snapshot(snapshot);\n                                }\n                            }\n                            None => {}\n                        }\n\n                        trace!(\n                            target: "bitcoinleveldbt_randomized::randomized",\n                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                            seed = seed,\n                            option_configuration_index = option_configuration_index,\n                            executed_prefix_length = executed_prefix_length,\n                            failed = true\n                        );\n\n                        return Some(failure_observation);\n                    }\n                }\n            }\n        }\n\n        match (step % 100i32) == 0i32 {\n            true => {\n                let model_ptr: *mut dyn DB =\n                    (&mut model as *mut ModelDB) as *mut dyn DB;\n                let db_ptr: *mut dyn DB =\n                    dbtest.dbfull() as *mut dyn DB;\n\n                let current_state_matches =\n                    compare_iterators(step, model_ptr, db_ptr, None, None);\n\n                match current_state_matches {\n                    true => {}\n                    false => {\n                        let failure_observation =\n                            bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                                seed,\n                                option_configuration_index,\n                                option_configuration,\n                                step,\n                                executed_prefix_length,\n                                last_operation_selector_probability,\n                                last_operation_kind,\n                                &last_key,\n                                last_value_length_bytes,\n                                last_write_batch_operation_count,\n                                BitcoinLevelDbTRandomizedBisectFailureMode::CompareIteratorsCurrentStateMismatch,\n                            );\n\n                        match model_snap.take() {\n                            Some(snapshot) => {\n                                model.release_snapshot(snapshot);\n                            }\n                            None => {}\n                        }\n\n                        match db_snap.take() {\n                            Some(snapshot) => {\n                                unsafe {\n                                    (*dbtest.dbfull()).release_snapshot(snapshot);\n                                }\n                            }\n                            None => {}\n                        }\n\n                        trace!(\n                            target: "bitcoinleveldbt_randomized::randomized",\n                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                            seed = seed,\n                            option_configuration_index = option_configuration_index,\n                            executed_prefix_length = executed_prefix_length,\n                            failed = true\n                        );\n\n                        return Some(failure_observation);\n                    }\n                }\n\n                let snapshot_state_matches =\n                    compare_iterators(\n                        step,\n                        model_ptr,\n                        db_ptr,\n                        model_snap.as_deref(),\n                        db_snap.as_deref(),\n                    );\n\n                match snapshot_state_matches {\n                    true => {}\n                    false => {\n                        let failure_observation =\n                            bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                                seed,\n                                option_configuration_index,\n                                option_configuration,\n                                step,\n                                executed_prefix_length,\n                                last_operation_selector_probability,\n                                last_operation_kind,\n                                &last_key,\n                                last_value_length_bytes,\n                                last_write_batch_operation_count,\n                                BitcoinLevelDbTRandomizedBisectFailureMode::CompareIteratorsSnapshotStateMismatch,\n                            );\n\n                        match model_snap.take() {\n                            Some(snapshot) => {\n                                model.release_snapshot(snapshot);\n                            }\n                            None => {}\n                        }\n\n                        match db_snap.take() {\n                            Some(snapshot) => {\n                                unsafe {\n                                    (*dbtest.dbfull()).release_snapshot(snapshot);\n                                }\n                            }\n                            None => {}\n                        }\n\n                        trace!(\n                            target: "bitcoinleveldbt_randomized::randomized",\n                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                            seed = seed,\n                            option_configuration_index = option_configuration_index,\n                            executed_prefix_length = executed_prefix_length,\n                            failed = true\n                        );\n\n                        return Some(failure_observation);\n                    }\n                }\n\n                match model_snap.take() {\n                    Some(snapshot) => {\n                        model.release_snapshot(snapshot);\n                    }\n                    None => {}\n                }\n\n                match db_snap.take() {\n                    Some(snapshot) => {\n                        unsafe {\n                            (*dbtest.dbfull()).release_snapshot(snapshot);\n                        }\n                    }\n                    None => {}\n                }\n\n                dbtest.reopen(None);\n\n                let reopened_db_ptr: *mut dyn DB =\n                    dbtest.dbfull() as *mut dyn DB;\n\n                let reopened_state_matches =\n                    compare_iterators(\n                        step,\n                        model_ptr,\n                        reopened_db_ptr,\n                        None,\n                        None,\n                    );\n\n                match reopened_state_matches {\n                    true => {}\n                    false => {\n                        let failure_observation =\n                            bitcoinleveldbt_randomized_bisect_build_failure_observation(\n                                seed,\n                                option_configuration_index,\n                                option_configuration,\n                                step,\n                                executed_prefix_length,\n                                last_operation_selector_probability,\n                                last_operation_kind,\n                                &last_key,\n                                last_value_length_bytes,\n                                last_write_batch_operation_count,\n                                BitcoinLevelDbTRandomizedBisectFailureMode::CompareIteratorsAfterReopenMismatch,\n                            );\n\n                        trace!(\n                            target: "bitcoinleveldbt_randomized::randomized",\n                            event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n                            label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n                            seed = seed,\n                            option_configuration_index = option_configuration_index,\n                            executed_prefix_length = executed_prefix_length,\n                            failed = true\n                        );\n\n                        return Some(failure_observation);\n                    }\n                }\n\n                model_snap = Some(model.get_snapshot());\n                unsafe { db_snap.acquire(dbtest.dbfull() as *mut dyn DB); }\n            }\n            false => {}\n        }\n\n        step += 1i32;\n    }\n\n    match model_snap.take() {\n        Some(snapshot) => {\n            model.release_snapshot(snapshot);\n        }\n        None => {}\n    }\n\n    match db_snap.take() {\n        Some(snapshot) => {\n            unsafe {\n                (*dbtest.dbfull()).release_snapshot(snapshot);\n            }\n        }\n        None => {}\n    }\n\n    execution_controls::progress("db_test_bisect_randomized", option_configuration_index as usize, step as usize, executed_prefix_length as usize, "configuration.return");\n\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure_exit",\n        label = "bitcoinleveldbt_randomized.execute_prefix_and_return_first_failure.exit",\n        seed = seed,\n        option_configuration_index = option_configuration_index,\n        executed_prefix_length = executed_prefix_length,\n        failed = false\n    );\n\n    None\n\n}\n\nfn bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure(\n    seed: u32,\n    executed_prefix_length: i32,\n) -> Option<BitcoinLevelDbTRandomizedBisectedFailureObservation> {\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure_entry",\n        label = "bitcoinleveldbt_randomized.run_across_option_configurations_and_return_first_failure.entry",\n        seed = seed,\n        executed_prefix_length = executed_prefix_length\n    );\n\n    let mut option_configuration_index: i32 = 0i32;\n\n    loop {\n        let option_configuration =\n            bitcoinleveldbt_randomized_bisect_option_configuration_from_index(\n                option_configuration_index,\n            );\n\n        match option_configuration {\n            DBTestOptionConfig::End => {\n                trace!(\n                    target: "bitcoinleveldbt_randomized::randomized",\n                    event = "bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure_exit",\n                    label = "bitcoinleveldbt_randomized.run_across_option_configurations_and_return_first_failure.exit",\n                    seed = seed,\n                    executed_prefix_length = executed_prefix_length,\n                    failed = false\n                );\n\n                return None;\n            }\n            DBTestOptionConfig::Default\n                | DBTestOptionConfig::Reuse\n                | DBTestOptionConfig::Filter\n                | DBTestOptionConfig::Uncompressed => {\n                    info!(\n                        target: "bitcoinleveldbt_randomized::randomized",\n                        event = "bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure_decision",\n                        label = "bitcoinleveldbt_randomized.run_across_option_configurations_and_return_first_failure.execute_option_configuration",\n                        seed = seed,\n                        executed_prefix_length = executed_prefix_length,\n                        option_configuration_index = option_configuration_index,\n                        option_configuration = ?option_configuration\n                    );\n\n                    let maybe_failure =\n                        bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure(\n                            seed,\n                            option_configuration_index,\n                            executed_prefix_length,\n                        );\n\n                    match maybe_failure {\n                        Some(failure_observation) => {\n                            trace!(\n                                target: "bitcoinleveldbt_randomized::randomized",\n                                event = "bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure_exit",\n                                label = "bitcoinleveldbt_randomized.run_across_option_configurations_and_return_first_failure.exit",\n                                seed = seed,\n                                executed_prefix_length = executed_prefix_length,\n                                failed = true\n                            );\n\n                            return Some(failure_observation);\n                        }\n                        None => {\n                            option_configuration_index += 1i32;\n                        }\n                    }\n                }\n        }\n    }\n\n}\n\nfn bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration(\n    seed: u32,\n    option_configuration_index: i32,\n    upper_bound_prefix_length: i32,\n) -> Option<BitcoinLevelDbTRandomizedBisectedFailureObservation> {\n    trace!(\n        target: "bitcoinleveldbt_randomized::randomized",\n        event = "bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration_entry",\n        label = "bitcoinleveldbt_randomized.bisect.minimal_failing_prefix_for_option_configuration.entry",\n        seed = seed,\n        option_configuration_index = option_configuration_index,\n        upper_bound_prefix_length = upper_bound_prefix_length\n    );\n\n    let initial_failure =\n        bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure(\n            seed,\n            option_configuration_index,\n            upper_bound_prefix_length,\n        );\n\n    match initial_failure {\n        None => {\n            trace!(\n                target: "bitcoinleveldbt_randomized::randomized",\n                event = "bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration_exit",\n                label = "bitcoinleveldbt_randomized.bisect.minimal_failing_prefix_for_option_configuration.exit",\n                seed = seed,\n                option_configuration_index = option_configuration_index,\n                upper_bound_prefix_length = upper_bound_prefix_length,\n                failed = false\n            );\n\n            None\n        }\n        Some(mut best_failure_observation) => {\n            let mut low_prefix_length: i32 = 1i32;\n            let mut high_prefix_length: i32 = upper_bound_prefix_length;\n\n            while low_prefix_length < high_prefix_length {\n                let mid_prefix_length =\n                    low_prefix_length\n                    + ((high_prefix_length - low_prefix_length) / 2i32);\n\n                eprintln!(\n                    "RANDOMIZED_BISECT_PROGRESS seed={} option_index={} low={} mid={} high={}",\n                    seed,\n                    option_configuration_index,\n                    low_prefix_length,\n                    mid_prefix_length,\n                    high_prefix_length,\n                );\n\n                let mid_failure =\n                    bitcoinleveldbt_randomized_execute_prefix_and_return_first_failure(\n                        seed,\n                        option_configuration_index,\n                        mid_prefix_length,\n                    );\n\n                match mid_failure {\n                    Some(failure_observation) => {\n                        best_failure_observation = failure_observation;\n                        high_prefix_length = mid_prefix_length;\n                    }\n                    None => {\n                        low_prefix_length = mid_prefix_length + 1i32;\n                    }\n                }\n            }\n\n            trace!(\n                target: "bitcoinleveldbt_randomized::randomized",\n                event = "bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration_exit",\n                label = "bitcoinleveldbt_randomized.bisect.minimal_failing_prefix_for_option_configuration.exit",\n                seed = seed,\n                option_configuration_index = option_configuration_index,\n                upper_bound_prefix_length = upper_bound_prefix_length,\n                failed = true,\n                failing_step_index = *best_failure_observation.failing_step_index(),\n                executed_prefix_length = *best_failure_observation.executed_prefix_length()\n            );\n\n            Some(best_failure_observation)\n        }\n    }\n\n}\n\n#[test]\n#[ignore = "long stress; use a resource-controlled runner explicitly"]\nfn db_test_bisect_randomized() {\n    execution_controls::require_stress_protection("db_test_bisect_randomized");\n    let seed = random_seed() as u32;\n    let executed_prefix_length: i32 = LEGACY_STEPS_PER_CONFIGURATION as i32;\n\n    let first_failure_observation =\n        bitcoinleveldbt_randomized_run_across_option_configurations_and_return_first_failure(\n            seed,\n            executed_prefix_length,\n        );\n\n    match first_failure_observation {\n        None => {\n            execution_controls::complete("db_test_bisect_randomized", OPTION_CONFIGURATION_COUNT,\n                                         OPTION_CONFIGURATION_COUNT * LEGACY_STEPS_PER_CONFIGURATION);\n        }\n        Some(first_failure) => {\n            let bisected_failure =\n                bitcoinleveldbt_randomized_bisect_minimal_failing_prefix_for_option_configuration(\n                    *first_failure.seed(),\n                    *first_failure.option_configuration_index(),\n                    executed_prefix_length,\n                );\n\n            match bisected_failure {\n                Some(minimal_failure) => {\n                    eprintln!(\n                        "RANDOMIZED_BISECT_RESULT seed={} option_index={} option_config={:?} failing_step={} prefix_length={} failure_mode={:?} operation_kind={:?} branch_p={} key={} value_len={} batch_ops={}",\n                        *minimal_failure.seed(),\n                        *minimal_failure.option_configuration_index(),\n                        *minimal_failure.option_configuration(),\n                        *minimal_failure.failing_step_index(),\n                        *minimal_failure.executed_prefix_length(),\n                        *minimal_failure.failure_mode(),\n                        *minimal_failure.operation_kind(),\n                        *minimal_failure.operation_selector_probability(),\n                        minimal_failure.last_key(),\n                        *minimal_failure.last_value_length_bytes(),\n                        *minimal_failure.last_write_batch_operation_count(),\n                    );\n\n                    panic!();\n                }\n                None => {\n                    eprintln!(\n                        "RANDOMIZED_BISECT_RESULT seed={} option_index={} option_config={:?} inconsistency=full_run_failed_but_bisection_found_no_failing_prefix",\n                        *first_failure.seed(),\n                        *first_failure.option_configuration_index(),\n                        *first_failure.option_configuration(),\n                    );\n\n                    panic!();\n                }\n            }\n        }\n    }\n\n}\n\n\n', 'bitcoinleveldbt-randomized/tests/support/execution_controls.rs': b'//! Admission and bounded evidence for the two original ignored stress tests.\n//! The operation generator, mutation order and historical comparison cadence\n//! remain separate and unchanged. This helper constructs no database itself.\n\nuse std::path::{Component, Path};\nuse bitcoinleveldb_dbinterface::{DB, DBGetSnapshot, DBReleaseSnapshot};\nuse bitcoinleveldb_snapshot::Snapshot;\n\npub fn stress_limits_valid(memory: &str, pids: &str, swap_limit_text: &str, oom: &str, cpu: &str) -> bool {\n    let memory = match memory.parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    let pids = match pids.parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    let fields: Vec<&str> = cpu.split_whitespace().collect();\n    if fields.len() != 2 { return false; }\n    let quota = match fields[0].parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    let period = match fields[1].parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    memory > 0 && memory <= 2 * 1024 * 1024 * 1024\n        && pids > 0 && pids <= 64 && swap_limit_text == "0" && oom == "1"\n        && quota > 0 && period > 0 && quota <= period\n}\n\npub fn stress_policy_valid(logging: &str, seed: &str, data: &str,\n                           test_tmpdir: &str, tmpdir: &str, tmp: &str, temp: &str,\n                           case: &str) -> bool {\n    if logging != "off" || seed != "301"\n        || !matches!(case, "db_test_randomized" | "db_test_bisect_randomized")\n        || data != test_tmpdir || data != tmpdir || data != tmp || data != temp {\n        return false;\n    }\n    let path = Path::new(data);\n    path.is_absolute()\n        && path.components().all(|part| matches!(part, Component::RootDir | Component::Normal(_)))\n        && path.file_name().and_then(|name| name.to_str()) == Some(format!("case-data-{}", case).as_str())\n        && path.parent().is_some_and(|parent| parent != Path::new("/"))\n}\n\nfn read_control(path: &Path) -> String {\n    match std::fs::read_to_string(path) {\n        Ok(value) => value.trim().to_owned(),\n        Err(error) => panic!("use the protected randomized runner: cannot read {}: {}", path.display(), error),\n    }\n}\n\nfn variable(name: &str) -> String {\n    std::env::var(name).unwrap_or_default()\n}\n\npub fn require_stress_protection(case: &str) {\n    // This must precede DBTest construction, including fixture-driven creation.\n    assert_eq!(variable("BITCOIN_RS_RANDOMIZED_STRESS_GUARD"), "v1",\n               "use the protected randomized runner; a verified stress service is required");\n    let data = variable("BITCOIN_RS_RANDOMIZED_CASE_DATA");\n    assert!(stress_policy_valid(&variable("RUST_LOG"), &variable("TEST_RANDOM_SEED"), &data,\n                                &variable("TEST_TMPDIR"), &variable("TMPDIR"),\n                                &variable("TMP"), &variable("TEMP"), case),\n            "use the protected randomized runner: seed 301, quiet logging and one private data root are required");\n    let data_path = Path::new(&data);\n    for component in data_path.ancestors() {\n        let metadata = std::fs::symlink_metadata(component).expect("private data ancestor must exist");\n        assert!(metadata.is_dir() && !metadata.file_type().is_symlink(),\n                "private data path must contain only ordinary directories");\n    }\n    #[cfg(unix)] {\n        use std::os::unix::fs::PermissionsExt;\n        assert_eq!(std::fs::metadata(data_path).expect("private data root must exist").permissions().mode() & 0o777,\n                   0o700, "private data root must have mode 0700");\n    }\n    #[cfg(not(unix))]\n    panic!("protected randomized stress admission requires Unix directory permissions");\n\n    let memberships = read_control(Path::new("/proc/self/cgroup"));\n    let roots: Vec<&str> = memberships.lines().filter_map(|line| line.strip_prefix("0::")).collect();\n    assert_eq!(roots.len(), 1, "exactly one unified cgroup membership is required");\n    let membership = Path::new(roots[0]);\n    assert!(membership.is_absolute() && membership.components().all(|part|\n            matches!(part, Component::RootDir | Component::Normal(_))),\n            "invalid unified cgroup path");\n    let group = Path::new("/sys/fs/cgroup").join(roots[0].trim_start_matches(\'/\'));\n    let memory = read_control(&group.join("memory.max"));\n    let pids = read_control(&group.join("pids.max"));\n    let swap_limit_text = read_control(&group.join("memory.swap.max"));\n    let oom = read_control(&group.join("memory.oom.group"));\n    let cpu = read_control(&group.join("cpu.max"));\n    assert!(stress_limits_valid(&memory, &pids, &swap_limit_text, &oom, &cpu),\n            "use the protected randomized runner: memory<=2GiB,pids<=64,swap=0,grouped OOM and CPU<=1 required");\n    eprintln!("\\nRANDOMIZED_STRESS_GUARD case={} cgroup={} memory={} pids={} cpu={}",\n              case, group.display(), memory, pids, cpu);\n}\n\npub fn progress(case: &str, config: usize, done: usize, total: usize, phase: &str) {\n    eprintln!("RANDOMIZED_STRESS_PROGRESS case={} config={} phase={} done={} total={}",\n              case, config, phase, done, total);\n}\n\npub fn complete(case: &str, configurations: usize, steps: usize) {\n    eprintln!("\\nRANDOMIZED_STRESS_OK case={} configurations={} steps={} seed=301",\n              case, configurations, steps);\n}\n\n/// A held database snapshot must be unlinked through its owning live database.\n///\n/// The fixture and its heap database must remain alive and stable while this\n/// lease is nonempty. Declare the lease after the fixture/model, empty it before\n/// reopen, and acquire again from the new database pointer only after reopen.\n/// Existing explicit release paths use take(); Drop covers assertion unwinding.\npub struct DatabaseSnapshotLease {\n    owner: Option<*mut dyn DB>,\n    snapshot: Option<Box<dyn Snapshot>>,\n}\n\nimpl DatabaseSnapshotLease {\n    pub fn new() -> Self { Self { owner: None, snapshot: None } }\n\n    pub fn as_deref(&self) -> Option<&dyn Snapshot> { self.snapshot.as_deref() }\n\n    pub fn take(&mut self) -> Option<Box<dyn Snapshot>> {\n        self.owner = None;\n        self.snapshot.take()\n    }\n\n    /// Safety: owner is a live, stable database until this lease is emptied.\n    pub unsafe fn acquire(&mut self, owner: *mut dyn DB) {\n        // Check before GetSnapshot: a newly linked snapshot must not be dropped\n        // as the argument to an assertion or a rejected insertion.\n        assert!(self.owner.is_none() && self.snapshot.is_none(), "release the previous snapshot before acquire");\n        assert!(!owner.is_null(), "snapshot acquisition requires a live database");\n        let snapshot = unsafe { DBGetSnapshot::get_snapshot(&mut *owner) };\n        self.owner = Some(owner);\n        self.snapshot = Some(snapshot);\n    }\n}\n\nimpl Drop for DatabaseSnapshotLease {\n    fn drop(&mut self) {\n        if let Some(snapshot) = self.snapshot.take() {\n            match self.owner.take() {\n                Some(owner) => unsafe { DBReleaseSnapshot::release_snapshot(&mut *owner, snapshot); },\n                None => {\n                    // A broken owner invariant must not ordinarily drop a node\n                    // that could still be linked into a database snapshot list.\n                    std::mem::forget(snapshot);\n                    eprintln!("RANDOMIZED_STRESS_FATAL snapshot lease has no live owner");\n                    std::process::abort();\n                }\n            }\n        }\n    }\n}\n', 'bitcoinleveldbt-randomized/tests/execution_control_contracts.rs': b'// Pure admission contracts: no database, environment worker or native child.\n#[path = "support/execution_controls.rs"]\nmod execution_controls;\n\n#[test]\nfn randomized_stress_limits_reject_unbounded_controls() {\n    use execution_controls::stress_limits_valid as valid;\n    assert!(valid("2147483648", "64", "0", "1", "100000 100000"));\n    assert!(valid("536870912", "32", "0", "1", "50000 100000"));\n    for controls in [\n        ("max", "64", "0", "1", "100000 100000"),\n        ("0", "64", "0", "1", "100000 100000"),\n        ("2147483649", "64", "0", "1", "100000 100000"),\n        ("536870912", "max", "0", "1", "100000 100000"),\n        ("536870912", "65", "0", "1", "100000 100000"),\n        ("536870912", "0", "0", "1", "100000 100000"),\n        ("536870912", "64", "max", "1", "100000 100000"),\n        ("536870912", "64", "1", "1", "100000 100000"),\n        ("536870912", "64", "0", "0", "100000 100000"),\n        ("536870912", "64", "0", "1", "max 100000"),\n        ("536870912", "64", "0", "1", "100001 100000"),\n        ("536870912", "64", "0", "1", "0 100000"),\n        ("536870912", "64", "0", "1", "100000 0"),\n        ("536870912", "64", "0", "1", "100000"),\n    ] { assert!(!valid(controls.0, controls.1, controls.2, controls.3, controls.4)); }\n    eprintln!("\\nRANDOMIZED_CONTROL_OK case=randomized_stress_limits_reject_unbounded_controls");\n}\n\n#[test]\nfn randomized_stress_policy_rejects_wrong_seed_logging_and_data_paths() {\n    use execution_controls::stress_policy_valid as valid;\n    let case = "db_test_randomized";\n    let data = "/private/owned-run/case-data-db_test_randomized";\n    assert!(valid("off", "301", data, data, data, data, data, case));\n    for controls in [\n        ("trace", "301", data, data, data, data, data, case),\n        ("off", "302", data, data, data, data, data, case),\n        ("off", "301", data, "/tmp", data, data, data, case),\n        ("off", "301", data, data, "/tmp", data, data, case),\n        ("off", "301", data, data, data, "/tmp", data, case),\n        ("off", "301", data, data, data, data, "/tmp", case),\n        ("off", "301", "relative", "relative", "relative", "relative", "relative", case),\n        ("off", "301", "/", "/", "/", "/", "/", case),\n        ("off", "301", data, data, data, data, data, "other_test"),\n        ("off", "301", data, data, data, data, data, "db_test_bisect_randomized"),\n    ] { assert!(!valid(controls.0, controls.1, controls.2, controls.3, controls.4, controls.5, controls.6, controls.7)); }\n    eprintln!("\\nRANDOMIZED_CONTROL_OK case=randomized_stress_policy_rejects_wrong_seed_logging_and_data_paths");\n}\n', 'bitcoinleveldb-dbimpl/src/recover_log_file.rs': b'// ---------------- [ File: bitcoinleveldb-dbimpl/src/recover_log_file.rs ]\ncrate::ix!();\n\nimpl DBImpl {\n    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]\n    pub fn recover_log_file(\n        &mut self,\n        log_number: u64,\n        last_log: bool,\n        save_manifest: *mut bool,\n        edit: *mut VersionEdit,\n        max_sequence: *mut SequenceNumber,\n    ) -> crate::Status {\n        self.mutex.assert_held();\n\n        let tid = std::thread::current().id();\n        let t_enter = std::time::Instant::now();\n        let reuse_logs_enabled: bool = *self.options.reuse_logs();\n        let paranoid_checks: bool = *self.options.paranoid_checks();\n        let write_buffer_size: u64 = *self.options.write_buffer_size() as u64;\n\n        tracing::info!(\n            target: "bitcoinleveldb_dbimpl::recover_log_file",\n            event = "dbimpl_recover_log_file_entry",\n            ?tid,\n            dbname = %self.dbname,\n            log_number,\n            last_log,\n            save_manifest_ptr = save_manifest as usize,\n            edit_ptr = edit as usize,\n            max_sequence_ptr = max_sequence as usize,\n            reuse_logs_enabled,\n            paranoid_checks,\n            write_buffer_size,\n            "DBImpl::recover_log_file: enter"\n        );\n\n\n        // Open the log file\n        let fname: String = log_file_name(&self.dbname, log_number);\n\n        let mut file: *mut Box<dyn SequentialFile> = core::ptr::null_mut();\n        let mut status: Status = self.env.new_sequential_file(&fname, &mut file);\n\n        tracing::debug!(\n            target: "bitcoinleveldb_dbimpl::recover_log_file",\n            event = "dbimpl_recover_log_file_open_sequential",\n            ?tid,\n            dbname = %self.dbname,\n            log_number,\n            file = %fname,\n            status_ok = status.is_ok(),\n            status = %status.to_string(),\n            file_ptr = file as usize,\n            "DBImpl::recover_log_file: new_sequential_file returned"\n        );\n\n        if !status.is_ok() {\n            let status_before_ignore: String = status.to_string();\n            self.maybe_ignore_error(&mut status as *mut Status);\n            let status_after_ignore: String = status.to_string();\n\n            tracing::warn!(\n                target: "bitcoinleveldb_dbimpl::recover_log_file",\n                event = "dbimpl_recover_log_file_open_failure",\n                ?tid,\n                dbname = %self.dbname,\n                log_number,\n                file = %fname,\n                status_before_ignore = %status_before_ignore,\n                status_after_ignore = %status_after_ignore,\n                elapsed_ms = t_enter.elapsed().as_millis() as u64,\n                "DBImpl::recover_log_file: sequential-file open failed"\n            );\n\n\n            return status;\n        }\n\n        if file.is_null() {\n            tracing::error!(\n                target: "bitcoinleveldb_dbimpl::recover_log_file",\n                event = "dbimpl_recover_log_file_open_null_output",\n                ?tid,\n                dbname = %self.dbname,\n                log_number,\n                file = %fname,\n                "Env returned ok but output sequential file pointer was null"\n            );\n\n\n            let msg: Slice = Slice::from_str("new_sequential_file returned ok but output was null");\n            let fname_slice: Slice = Slice::from(&fname);\n            return Status::corruption(&msg, Some(&fname_slice));\n        }\n\n        // Take ownership of the opened file immediately.\n        let file_box: Box<dyn SequentialFile> = unsafe { *Box::from_raw(file) };\n\n        // Create the log reader.\n        let info_log_ptr: *mut dyn Logger = match self.options.info_log().as_ref().copied() {\n            Some(ptr) => ptr,\n            None => {\n                tracing::error!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_missing_info_log",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    "DBImpl::recover_log_file: Options.info_log is None"\n                );\n\n\n                let msg: Slice = Slice::from_str("Options.info_log missing during recover_log_file");\n                let fname_slice: Slice = Slice::from(&fname);\n                return Status::corruption(&msg, Some(&fname_slice));\n            }\n        };\n\n        let reporter: LogReporter = LogReporter {\n            info_log: info_log_ptr,\n            fname: fname.clone(),\n            status: if paranoid_checks {\n                &mut status as *mut Status\n            } else {\n                core::ptr::null_mut()\n            },\n        };\n\n        // We intentionally make LogReader do checksumming even if\n        // paranoid_checks==false so that corruptions cause entire commits\n        // to be skipped instead of propagating bad information (like overly\n        // large sequence numbers).\n        let mut reader: LogReader = LogReader::new(file_box, Box::new(reporter), true, 0);\n\n        tracing::info!(\n            target: "bitcoinleveldb_dbimpl::recover_log_file",\n            event = "dbimpl_recover_log_file_reader_created",\n            ?tid,\n            dbname = %self.dbname,\n            log_number,\n            file = %fname,\n            last_log,\n            "Recovering log"\n        );\n\n        // Read all the records and add to a memtable\n        let mut scratch: Vec<u8> = Vec::new();\n        let mut record: Slice = Slice::default();\n\n        let mut batch: WriteBatch = WriteBatch::default();\n        let mut compactions: i32 = 0;\n        let mut mem: *mut MemTable = core::ptr::null_mut();\n\n        let mut records_seen: u64 = 0;\n        let mut records_skipped_short: u64 = 0;\n        let mut batches_inserted: u64 = 0;\n        let mut bytes_replayed: u64 = 0;\n        let mut memtable_allocations: u64 = 0;\n        let mut max_batch_count: u64 = 0;\n        let mut max_record_len: u64 = 0;\n        let mut last_batch_sequence_start: SequenceNumber = 0;\n        let mut last_batch_sequence_end: SequenceNumber = 0;\n\n        while reader.read_record(&mut record, &mut scratch) && status.is_ok() {\n            records_seen = records_seen.saturating_add(1);\n\n            let record_len: usize = *record.size();\n            bytes_replayed = bytes_replayed.saturating_add(record_len as u64);\n            max_record_len = max_record_len.max(record_len as u64);\n\n            if record_len < 12 {\n                records_skipped_short = records_skipped_short.saturating_add(1);\n\n                let msg: Slice = Slice::from_str("log record too small");\n                let fname_slice: Slice = Slice::from(&fname);\n                let reason: Status = Status::corruption(&msg, Some(&fname_slice));\n\n                tracing::warn!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_short_record",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    record_index = records_seen,\n                    record_len,\n                    reason = %reason.to_string(),\n                    "Skipping short log record"\n                );\n\n                if records_seen == 1 || (records_seen % 128) == 0 {\n                }\n\n                reader.emit_corruption_to_reporter(record_len, &reason);\n                continue;\n            }\n\n            write_batch_internal::set_contents(&mut batch, &record);\n\n            if mem.is_null() {\n                mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));\n                unsafe {\n                    (*mem).ref_();\n                }\n                memtable_allocations = memtable_allocations.saturating_add(1);\n\n                tracing::debug!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_memtable_allocated",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    mem_ptr = mem as usize,\n                    memtable_allocations,\n                    "Allocated new memtable for recovery"\n                );\n            }\n\n            status = write_batch_internal::insert_into(&batch, mem);\n\n            if !status.is_ok() {\n                let status_before_ignore: String = status.to_string();\n                self.maybe_ignore_error(&mut status as *mut Status);\n                let status_after_ignore: String = status.to_string();\n\n                tracing::error!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_insert_failure",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    record_index = records_seen,\n                    status_before_ignore = %status_before_ignore,\n                    status_after_ignore = %status_after_ignore,\n                    "Failed inserting batch into memtable during recovery"\n                );\n\n\n                break;\n            }\n\n            batches_inserted = batches_inserted.saturating_add(1);\n\n            let count_u64: u64 = write_batch_internal::count(&batch) as u64;\n            max_batch_count = max_batch_count.max(count_u64);\n\n            let batch_sequence_start: SequenceNumber = write_batch_internal::sequence(&batch);\n            let last_seq: SequenceNumber = batch_sequence_start\n                .wrapping_add(count_u64)\n                .wrapping_sub(1);\n\n            last_batch_sequence_start = batch_sequence_start;\n            last_batch_sequence_end = last_seq;\n\n            unsafe {\n                if !max_sequence.is_null() && last_seq > *max_sequence {\n                    *max_sequence = last_seq;\n                }\n            }\n\n            if records_seen == 1 || (records_seen % 128) == 0 {\n                let approx_mem: u64 = unsafe { (*mem).approximate_memory_usage() } as u64;\n\n                tracing::debug!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_progress",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    records_seen,\n                    batches_inserted,\n                    bytes_replayed,\n                    record_len,\n                    batch_count = count_u64,\n                    batch_sequence_start,\n                    batch_sequence_end = last_seq,\n                    approx_mem,\n                    compactions,\n                    records_skipped_short,\n                    "DBImpl::recover_log_file: replay progress"\n                );\n\n            }\n\n            if unsafe { (*mem).approximate_memory_usage() } > *self.options.write_buffer_size() {\n                compactions = compactions.saturating_add(1);\n\n                unsafe {\n                    if !save_manifest.is_null() {\n                        *save_manifest = true;\n                    }\n                }\n\n                let approx_mem: u64 = unsafe { (*mem).approximate_memory_usage() } as u64;\n\n                tracing::info!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_memtable_full",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    records_seen,\n                    batches_inserted,\n                    approx_mem,\n                    write_buffer_size,\n                    compactions,\n                    "Memtable full during recovery; compacting to level-0"\n                );\n\n\n                status = self.write_level_0table(mem, edit, core::ptr::null_mut());\n\n                unsafe {\n                    (*mem).unref();\n                }\n                mem = core::ptr::null_mut();\n\n                if !status.is_ok() {\n                    tracing::error!(\n                        target: "bitcoinleveldb_dbimpl::recover_log_file",\n                        event = "dbimpl_recover_log_file_write_l0_failure",\n                        ?tid,\n                        dbname = %self.dbname,\n                        log_number,\n                        file = %fname,\n                        status = %status.to_string(),\n                        compactions,\n                        "write_level_0table failed during recovery"\n                    );\n\n\n                    // Reflect errors immediately so that conditions like full\n                    // file-systems cause the DB::Open() to fail.\n                    break;\n                }\n            }\n        }\n\n        drop(reader);\n\n        let mut reuse_attempted: bool = false;\n        let mut reused_existing_log: bool = false;\n        let mut reused_mem_from_recovery: bool = false;\n        let mut created_empty_mem_for_reuse: bool = false;\n        let mut file_size_status_text: String = String::from("not_attempted");\n        let mut append_status_text: String = String::from("not_attempted");\n        let mut append_output_null: bool = false;\n        let mut lfile_size: u64 = 0;\n\n        // See if we should keep reusing the last log file.\n        if status.is_ok() && reuse_logs_enabled && last_log && compactions == 0 {\n            reuse_attempted = true;\n\n            assert!(self.log.is_null());\n            assert!(self.mem.is_null());\n\n            let mut append_file: *mut Box<dyn WritableFile> = core::ptr::null_mut();\n\n            let file_size_status: Status = self.env.get_file_size(&fname, &mut lfile_size);\n            file_size_status_text = file_size_status.to_string();\n\n            tracing::debug!(\n                target: "bitcoinleveldb_dbimpl::recover_log_file",\n                event = "dbimpl_recover_log_file_reuse_decision",\n                ?tid,\n                dbname = %self.dbname,\n                log_number,\n                file = %fname,\n                reuse_logs_enabled,\n                last_log,\n                compactions,\n                file_size_status_ok = file_size_status.is_ok(),\n                file_size_status = %file_size_status_text,\n                lfile_size,\n                "DBImpl::recover_log_file: evaluated file-size precondition for log reuse"\n            );\n\n            if file_size_status.is_ok() {\n                let append_status: Status = self.env.new_appendable_file(&fname, &mut append_file);\n                append_status_text = append_status.to_string();\n                append_output_null = append_file.is_null();\n\n                tracing::debug!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_append_open_result",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    append_status_ok = append_status.is_ok(),\n                    append_status = %append_status_text,\n                    append_output_null,\n                    "DBImpl::recover_log_file: append-open attempt completed"\n                );\n\n                if append_status.is_ok() && !append_file.is_null() {\n                    tracing::info!(\n                        target: "bitcoinleveldb_dbimpl::recover_log_file",\n                        event = "dbimpl_recover_log_file_reuse_success",\n                        ?tid,\n                        dbname = %self.dbname,\n                        log_number,\n                        file = %fname,\n                        lfile_size,\n                        "Reusing old log"\n                    );\n\n\n                    let writable_box: Box<dyn WritableFile> = unsafe { *Box::from_raw(append_file) };\n                    self.logfile = Rc::new(RefCell::new(writable_box));\n                    self.log = Box::into_raw(Box::new(LogWriter::new(self.logfile.clone(), lfile_size)));\n                    self.logfile_number = log_number;\n                    reused_existing_log = true;\n\n                    if !mem.is_null() {\n                        self.mem = mem;\n                        mem = core::ptr::null_mut();\n                        reused_mem_from_recovery = true;\n                    } else {\n                        // mem can be null if lognum exists but was empty.\n                        self.mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));\n                        unsafe {\n                            (*self.mem).ref_();\n                        }\n                        created_empty_mem_for_reuse = true;\n                    }\n                } else {\n                    tracing::debug!(\n                        target: "bitcoinleveldb_dbimpl::recover_log_file",\n                        event = "dbimpl_recover_log_file_reuse_fallback",\n                        ?tid,\n                        dbname = %self.dbname,\n                        log_number,\n                        file = %fname,\n                        append_status = %append_status_text,\n                        append_output_null,\n                        "ReuseLogs enabled but could not open appendable file; falling back"\n                    );\n\n                }\n            } else {\n                tracing::debug!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_reuse_skipped_file_size",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    file_size_status = %file_size_status_text,\n                    "DBImpl::recover_log_file: file-size query failed; skipping log reuse"\n                );\n\n            }\n        } else {\n            tracing::debug!(\n                target: "bitcoinleveldb_dbimpl::recover_log_file",\n                event = "dbimpl_recover_log_file_reuse_not_attempted",\n                ?tid,\n                dbname = %self.dbname,\n                log_number,\n                file = %fname,\n                status_ok = status.is_ok(),\n                reuse_logs_enabled,\n                last_log,\n                compactions,\n                "DBImpl::recover_log_file: reuse preconditions not satisfied"\n            );\n        }\n\n        let mut tail_mem_flushed_to_l0: bool = false;\n\n        if !mem.is_null() {\n            if status.is_ok() {\n                unsafe {\n                    if !save_manifest.is_null() {\n                        *save_manifest = true;\n                    }\n                }\n\n                tracing::info!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_tail_flush_begin",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    mem_ptr = mem as usize,\n                    "DBImpl::recover_log_file: flushing remaining recovery memtable"\n                );\n\n                status = self.write_level_0table(mem, edit, core::ptr::null_mut());\n                tail_mem_flushed_to_l0 = status.is_ok();\n\n                tracing::info!(\n                    target: "bitcoinleveldb_dbimpl::recover_log_file",\n                    event = "dbimpl_recover_log_file_tail_flush_end",\n                    ?tid,\n                    dbname = %self.dbname,\n                    log_number,\n                    file = %fname,\n                    status_ok = status.is_ok(),\n                    status = %status.to_string(),\n                    tail_mem_flushed_to_l0,\n                    "DBImpl::recover_log_file: tail memtable flush completed"\n                );\n            }\n\n            unsafe {\n                (*mem).unref();\n            }\n        }\n\n        tracing::info!(\n            target: "bitcoinleveldb_dbimpl::recover_log_file",\n            event = "dbimpl_recover_log_file_exit",\n            ?tid,\n            dbname = %self.dbname,\n            log_number,\n            file = %fname,\n            status_ok = status.is_ok(),\n            status = %status.to_string(),\n            elapsed_ms = t_enter.elapsed().as_millis() as u64,\n            records_seen,\n            records_skipped_short,\n            batches_inserted,\n            bytes_replayed,\n            max_record_len,\n            max_batch_count,\n            last_batch_sequence_start,\n            last_batch_sequence_end,\n            compactions,\n            memtable_allocations,\n            reuse_attempted,\n            reused_existing_log,\n            reused_mem_from_recovery,\n            created_empty_mem_for_reuse,\n            file_size_status = %file_size_status_text,\n            append_status = %append_status_text,\n            append_output_null,\n            lfile_size,\n            tail_mem_flushed_to_l0,\n            save_manifest_value = if save_manifest.is_null() { false } else { unsafe { *save_manifest } },\n            max_sequence_value = if max_sequence.is_null() { 0 } else { unsafe { *max_sequence } },\n            "DBImpl::recover_log_file: exit"\n        );\n\n\n        status\n    }\n}\n\n#[cfg(test)]\nmod recover_log_file_interface_and_error_handling_suite {\n    use super::*;\n\n    fn build_temp_db_path_for_recover_log_file_suite() -> String {\n        let tmp = TempDir::new().unwrap();\n        let dbname = tmp.path().to_string_lossy().to_string();\n        dbname\n    }\n\n    fn build_options_with_env_and_paranoid_flag(paranoid: bool) -> Options {\n        let env = PosixEnv::shared();\n        let mut options: Options = Options::with_env(env);\n\n        if options.env().is_none() {\n            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run recover_log_file suite");\n            panic!();\n        }\n\n        options.set_paranoid_checks(paranoid);\n        options\n    }\n\n    #[traced_test]\n    fn recover_log_file_signature_is_stable() {\n        tracing::info!("Asserting DBImpl::recover_log_file signature is stable");\n\n        type Sig = fn(\n            &mut DBImpl,\n            u64,\n            bool,\n            *mut bool,\n            *mut VersionEdit,\n            *mut SequenceNumber,\n        ) -> Status;\n\n        let _sig: Sig = DBImpl::recover_log_file;\n\n        tracing::debug!("Signature check compiled");\n    }\n\n    #[traced_test]\n    fn recover_log_file_returns_ok_for_missing_log_when_paranoid_checks_is_false() {\n        let dbname = build_temp_db_path_for_recover_log_file_suite();\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = build_options_with_env_and_paranoid_flag(false);\n        let mut db: DBImpl = DBImpl::new(&options, &dbname);\n\n        let mut save_manifest: bool = false;\n        let mut edit: VersionEdit = Default::default();\n        let mut max_sequence: SequenceNumber = 0;\n\n        db.mutex.lock();\n\n        tracing::info!(\n            dbname = %dbname,\n            paranoid_checks = *db.options.paranoid_checks(),\n            "Calling recover_log_file on missing log; expecting OK in non-paranoid mode"\n        );\n\n        let s: Status = db.recover_log_file(\n            999_999,\n            false,\n            &mut save_manifest as *mut bool,\n            &mut edit as *mut VersionEdit,\n            &mut max_sequence as *mut SequenceNumber,\n        );\n\n        unsafe { db.mutex.unlock() };\n\n        tracing::debug!(status = %s.to_string(), "recover_log_file returned");\n        assert!(s.is_ok(), "Non-paranoid mode must ignore missing-log errors via maybe_ignore_error");\n\n        drop(db);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n\n    #[traced_test]\n    fn recover_log_file_returns_error_for_missing_log_when_paranoid_checks_is_true() {\n        let dbname = build_temp_db_path_for_recover_log_file_suite();\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = build_options_with_env_and_paranoid_flag(true);\n        let mut db: DBImpl = DBImpl::new(&options, &dbname);\n\n        let mut save_manifest: bool = false;\n        let mut edit: VersionEdit = Default::default();\n        let mut max_sequence: SequenceNumber = 0;\n\n        db.mutex.lock();\n\n        tracing::info!(\n            dbname = %dbname,\n            paranoid_checks = *db.options.paranoid_checks(),\n            "Calling recover_log_file on missing log; expecting non-OK in paranoid mode"\n        );\n\n        let s: Status = db.recover_log_file(\n            999_999,\n            false,\n            &mut save_manifest as *mut bool,\n            &mut edit as *mut VersionEdit,\n            &mut max_sequence as *mut SequenceNumber,\n        );\n\n        unsafe { db.mutex.unlock() };\n\n        tracing::debug!(status = %s.to_string(), "recover_log_file returned");\n        assert!(\n            !s.is_ok(),\n            "Paranoid mode must preserve missing-log error (must not be ignored)"\n        );\n\n        drop(db);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n}\n'}
ORIGINAL_SOURCE_HASHES={'bitcoinleveldbt-randomized/tests/randomized.rs': '6e9ee7f73f775a713f6ad72df81f2d5010c1b0c24b5872a180911e8b4b1c04ef', 'bitcoinleveldbt-randomized/tests/bisect.rs': '84225e5b9b6b15d614d2696b2b8a0bdff55f13bdd9b434d945ce17a3b9379acc'}
ORIGINAL_RANDOMIZED_INPUT_HASHES={'bitcoinleveldbt-randomized/tests/randomized.rs': '6e9ee7f73f775a713f6ad72df81f2d5010c1b0c24b5872a180911e8b4b1c04ef', 'bitcoinleveldbt-randomized/tests/bisect.rs': '84225e5b9b6b15d614d2696b2b8a0bdff55f13bdd9b434d945ce17a3b9379acc', 'bitcoinleveldbt-randomized/tests/bounded_contracts.rs': 'af6c2e76de680d22dafea5ea90bdcf79c02ed1fe7e3868de549631636cc39d72', 'bitcoinleveldbt-randomized/tests/replay_contracts.rs': 'ebe8fb64e06e5c13605f73c9a42d9eabd6c39609df8d9e57c57182b24e04326e', 'bitcoinleveldbt-randomized/tests/support/randomized_operations.rs': '9ce6a44cf8efcad93d5d3662f687d243ea37108d0ec9bb0dcf1c2640aaa8349e'}
QUIET_RECOVERY_ORIGINAL_HASHES={'bitcoinleveldb-dbimpl/src/recover_log_file.rs': '94f58d8d36c52bdf052de8ead5696e7784d798fdc0ddf792f121d59c45be2529'}
QUIET_RECOVERY_POLICY={'path': 'bitcoinleveldb-dbimpl/src/recover_log_file.rs', 'before_sha256': '94f58d8d36c52bdf052de8ead5696e7784d798fdc0ddf792f121d59c45be2529', 'after_sha256': 'f24b9d82cba9134e10aab16bb79e393dae011b99f62dc10e4f05b0aa04b6fa75', 'before_bytes': 32027, 'after_bytes': 25158, 'stderr_statements_removed': 13, 'events': [{'stderr_event': 'dbimpl_recover_log_file_entry', 'retained_tracing_event': 'dbimpl_recover_log_file_entry', 'source_line': 38, 'removed_bytes': 521, 'read_argument_count': 9, 'retained_trace_sha256': '246993037ea9f0e6108a98357e8da0e1c9610f2228c8d79cb20c7f5ee53a011f'}, {'stderr_event': 'dbimpl_recover_log_file_open_failure', 'retained_tracing_event': 'dbimpl_recover_log_file_open_failure', 'source_line': 88, 'removed_bytes': 436, 'read_argument_count': 6, 'retained_trace_sha256': '18daca2157df4a97a6e3da53117f33ed7b546fd97230757106f56d2953972ad5'}, {'stderr_event': 'dbimpl_recover_log_file_open_null_output', 'retained_tracing_event': 'dbimpl_recover_log_file_open_null_output', 'source_line': 112, 'removed_bytes': 246, 'read_argument_count': 3, 'retained_trace_sha256': 'd38e9569b917b24541a0f34d82b2cdd26adac0cd85c7ac9f26fc2265767617ce'}, {'stderr_event': 'dbimpl_recover_log_file_missing_info_log', 'retained_tracing_event': 'dbimpl_recover_log_file_missing_info_log', 'source_line': 141, 'removed_bytes': 270, 'read_argument_count': 3, 'retained_trace_sha256': '649be69d08dc276f900579fdf355981a78f395378af0ac3cc850bf7be653ac6e'}, {'stderr_event': 'dbimpl_recover_log_file_progress', 'retained_tracing_event': 'dbimpl_recover_log_file_short_record', 'source_line': 227, 'removed_bytes': 609, 'read_argument_count': 8, 'retained_trace_sha256': '27037eb7fa5d09a94f0d263ccfcc2cc507fd5f86005af0d2914a3ce9188c2974'}, {'stderr_event': 'dbimpl_recover_log_file_insert_failure', 'retained_tracing_event': 'dbimpl_recover_log_file_insert_failure', 'source_line': 286, 'removed_bytes': 452, 'read_argument_count': 6, 'retained_trace_sha256': '45275d0f7c72e3f060f60161bc66a0c2e1d4a77c3e718e1984d23c341b7d7dd5'}, {'stderr_event': 'dbimpl_recover_log_file_progress', 'retained_tracing_event': 'dbimpl_recover_log_file_progress', 'source_line': 341, 'removed_bytes': 751, 'read_argument_count': 12, 'retained_trace_sha256': '299bf6a57a40d121cc56d5a6b6f99624134ab8d76cb4a034b34536de154dbf5f'}, {'stderr_event': 'dbimpl_recover_log_file_memtable_full', 'retained_tracing_event': 'dbimpl_recover_log_file_memtable_full', 'source_line': 384, 'removed_bytes': 492, 'read_argument_count': 7, 'retained_trace_sha256': 'f4e02e1fbfe9c828a6f5d26b7fd0b1d518db07085b61f64d7b9315b2dbafbaa7'}, {'stderr_event': 'dbimpl_recover_log_file_write_l0_failure', 'retained_tracing_event': 'dbimpl_recover_log_file_write_l0_failure', 'source_line': 415, 'removed_bytes': 402, 'read_argument_count': 5, 'retained_trace_sha256': '02cdc8a6e7cf4a51eabb2b7c018ce3cf2464012568c7ef153bbe8dded5f50c63'}, {'stderr_event': 'dbimpl_recover_log_file_reuse_success', 'retained_tracing_event': 'dbimpl_recover_log_file_reuse_success', 'source_line': 500, 'removed_bytes': 341, 'read_argument_count': 4, 'retained_trace_sha256': '1f39b115d6aa1a8c8365e4f0c6204b9188c9a65dc23a5f2406de845ca286564f'}, {'stderr_event': 'dbimpl_recover_log_file_reuse_fallback', 'retained_tracing_event': 'dbimpl_recover_log_file_reuse_fallback', 'source_line': 539, 'removed_bytes': 421, 'read_argument_count': 5, 'retained_trace_sha256': 'b96049b73812e6e7d63406879c7ced3e2ffa75d26a740fe637c84bb6106b9b1d'}, {'stderr_event': 'dbimpl_recover_log_file_reuse_skipped_file_size', 'retained_tracing_event': 'dbimpl_recover_log_file_reuse_skipped_file_size', 'source_line': 560, 'removed_bytes': 342, 'read_argument_count': 4, 'retained_trace_sha256': '737aafd5189435f1785a359717a2ec52f0230875445033996ab8c2beeb4f3f7e'}, {'stderr_event': 'dbimpl_recover_log_file_exit', 'retained_tracing_event': 'dbimpl_recover_log_file_exit', 'source_line': 661, 'removed_bytes': 1586, 'read_argument_count': 27, 'retained_trace_sha256': '8e91e39c7c30e808cc443c5a2c0504e73224b810b965d99f73fc14d8c41167c8'}], 'change': 'remove thirteen redundant direct stderr statements; retain all structured tracing and non-stderr source bytes', 'qualification': 'exact source transformation only; native regression execution remains required'}
OPERATION_GENERATOR_SHA='9ce6a44cf8efcad93d5d3662f687d243ea37108d0ec9bb0dcf1c2640aaa8349e'
ORACLE_SCOPE='Historical checks after mutations 0,100,...,9900; final99 mutations are unchecked; compare_iterators does not inspect iterator status.'
CASES=[{'target': 'execution_control_contracts', 'suffix': 'randomized_stress_limits_reject_unbounded_controls', 'profile': 'pure_contract', 'ignored': False, 'marker': 'RANDOMIZED_CONTROL_OK case=randomized_stress_limits_reject_unbounded_controls'}, {'target': 'execution_control_contracts', 'suffix': 'randomized_stress_policy_rejects_wrong_seed_logging_and_data_paths', 'profile': 'pure_contract', 'ignored': False, 'marker': 'RANDOMIZED_CONTROL_OK case=randomized_stress_policy_rejects_wrong_seed_logging_and_data_paths'}, {'target': 'replay_contracts', 'suffix': 'replay_matches_legacy_tape', 'profile': 'replay', 'ignored': False, 'marker': 'REPLAY_CONTRACT_OK case=replay_matches_legacy_tape'}, {'target': 'replay_contracts', 'suffix': 'replay_preserves_configuration_offsets', 'profile': 'replay', 'ignored': False, 'marker': 'REPLAY_CONTRACT_OK case=replay_preserves_configuration_offsets'}, {'target': 'replay_contracts', 'suffix': 'replay_preserves_short_prefixes', 'profile': 'replay', 'ignored': False, 'marker': 'REPLAY_CONTRACT_OK case=replay_preserves_short_prefixes'}, {'target': 'replay_contracts', 'suffix': 'replay_covers_batch_order', 'profile': 'replay', 'ignored': False, 'marker': 'REPLAY_CONTRACT_OK case=replay_covers_batch_order'}, {'target': 'bounded_contracts', 'suffix': 'bounded_default', 'profile': 'bounded', 'ignored': False, 'marker': 'BOUNDED_RANDOMIZED_OK config=0 steps=34'}, {'target': 'bounded_contracts', 'suffix': 'bounded_reuse', 'profile': 'bounded', 'ignored': False, 'marker': 'BOUNDED_RANDOMIZED_OK config=1 steps=34'}, {'target': 'bounded_contracts', 'suffix': 'bounded_filter', 'profile': 'bounded', 'ignored': False, 'marker': 'BOUNDED_RANDOMIZED_OK config=2 steps=34'}, {'target': 'bounded_contracts', 'suffix': 'bounded_uncompressed', 'profile': 'bounded', 'ignored': False, 'marker': 'BOUNDED_RANDOMIZED_OK config=3 steps=34'}, {'target': 'randomized', 'suffix': 'db_test_randomized', 'profile': 'full_original', 'ignored': True, 'marker': 'RANDOMIZED_STRESS_OK case=db_test_randomized configurations=4 steps=40000 seed=301'}, {'target': 'bisect', 'suffix': 'db_test_bisect_randomized', 'profile': 'full_original', 'ignored': True, 'marker': 'RANDOMIZED_STRESS_OK case=db_test_bisect_randomized configurations=4 steps=40000 seed=301'}]
DOCUMENT_BYTES=b'# Controlled randomized full-workload qualification025\n\nThis repair is bound to master 904156f818d38b443f02ff8fba6f2a2bfd5ed8a7 and\nthe selected current source captured by source022. It creates the isolated\nbranch repair/leveldb-randomized-quiet025 and its own worktree/cache.\nIt does not promote master. A qualified receipt is published only after all\ntwelve exact cases pass and original-state preservation verifies.\n\nRun the downloaded single-file controller with cached Python:\n\n```sh\nnix shell --offline --option max-jobs 0 --option builders \'\' \\\n  nixpkgs#python3 --command python3 \\\n  "$HOME/Downloads/BitcoinRS-LevelDB-Randomized-QuietRepair025-20261108.py" \\\n  --repo "$HOME/bitcoin-rs"\n```\n\nThe two original tests remain ignored by default. Their first action now\nrequires the runner marker, seed 301, RUST_LOG=off, one ordinary 0700 case-data\ndirectory shared by TEST_TMPDIR/TMPDIR/TMP/TEMP, and actual cgroup controls:\nmemory at most 2 GiB, at most 64 tasks, swap 0, grouped OOM and at most one CPU.\nTheir historical operation generator, four configurations, 10,000 mutations\nper configuration, mutation ordering and comparison cadence are unchanged.\nProgress and full completion markers are explicit stderr records. The main\ntest\'s completion marker follows fixture teardown; the bisect marker follows\nthe successful return of all four complete configurations.\n\nA test-local database snapshot lease preserves normal explicit release order\nand releases an outstanding snapshot through its still-live owner on unwind.\nThe fixture/model outlive the lease. The lease is emptied before every reopen\nand reacquires from the new database afterward. This repair\'s native suite does\nnot deliberately inject a panic into that cleanup path.\n\nCases run sequentially in separate guarded processes: two pure admission\ncontracts, four unchanged replay contracts, four unchanged 34-step bounded\ncases, then db_test_randomized and db_test_bisect_randomized. Each original\nperforms 40,000 logical database mutation steps with seed 301. Pure operation\ngeneration used to skip earlier configurations is additional CPU work, not\nadditional database mutations. Successful execution does not prove that a\nbackground-dependent failure is repeatable or monotone under bisection.\n\nThe existing comparison oracle is retained. It compares after mutations\n0,100,...,9900; the final 99 mutations have no terminal comparison. The shared\ncompare_iterators helper does not check iterator statuses. A passing full run\ntherefore establishes the declared historical workload and assertions, not\ncomplete database correctness or exhaustive coverage. The earlier machine\nfreeze\'s cause remains unestablished by this workflow.\n\nThe Cargo no-run build is offline/locked with one job. Nix uses the retained\nshell with local jobs 0, empty builders and substitution disabled. Exact Cargo\nJSON records bind six test executables to package/target/source paths. Library\nmetadata must list zero cases; exactly the two original stress cases must be\nignored. Case command, executable bytes, guard admission, private data root,\nresult, exact test name and success marker are retained and reverified.\n\n| Phase | Hard / soft memory | Deadline | Tasks | Output |\n| --- | --- | --- | --- | --- |\n| Build or independent cache seed |4GiB /3GiB |600s |128 |2MiB |\n| Each ordinary case/list |512MiB /384MiB |30s |64 |1MiB |\n| Each original stress case |2GiB /1536MiB |600s |64 |1MiB |\n| Source/identity verification |512MiB /384MiB |180s |32 |1MiB |\n| Guard preflight |128MiB /96MiB |10s |32 |1MiB |\n\nAll service phases have swap disabled and at most one aggregate CPU. Stress\ndata monitoring is capped at 512 MiB/4096 entries; ordinary cases at 128 MiB/4096.\nThe cache seed makes independent ordinary-file copies, excludes test binaries,\nrespects finite byte/file/disk allowances, and holds the source cache locks.\nMissing controls or cached requirements cause failure without a fallback.\n\nThe journal records native_pending before dispatch and retains native_failure\non a failed case. An incomplete/failed attempt is not automatically retried.\nAn already-complete exact candidate may reverify its retained qualification;\nthat is reported as validation reuse, not a fresh native run. This installer\nis tied to its reviewed base; later source revisions need a new qualification.\n\nRANDOMIZED_REPAIR_RETURN.json and capped logs remain in this workflow\'s own\nmetadata directory. Only after coordinator exit and printed/saved JSON checks\ndoes the transport replace the authorized return-buffer. Original user work,\nunrelated refs/indices and earlier workflow journals must remain unchanged.\n\n\n## Quiet recovery diagnostics025\n\nThis candidate is created on a new `repair/leveldb-randomized-quiet025` branch and\nlinked worktree from master 904156f818d38b443f02ff8fba6f2a2bfd5ed8a7. The earlier\nrandomized023 journal, failed candidate, reports, logs, and executable bytes remain\nprotected inputs. The script never resets or rewrites that journal and never\nuses its ten passing cases as new qualification.\n\nThe retained Revision002 full `db_test_randomized` invocation reached database\nwork, then the guard stopped it with `outcome=output_limit`: 1,049,927 observed\nbytes exceeded the 1,048,576-byte ceiling after about 3.87 seconds. Its output\nprefix and retained rolling tail reconstruct the complete counted transcript.\nThe termination record is SIGTERM 15; the wrapper\'s exit 0 does not make the native\ncase pass. The full completion marker and libtest success result are absent.\nThe second full case was never started. This establishes the controlled output\nstop only; it does not explain the historical workstation freeze or prove a\nstorage correctness defect.\n\n`bitcoinleveldb-dbimpl/src/recover_log_file.rs` has duplicate direct `eprintln!`\ndiagnostics alongside its existing `tracing` events. This candidate removes only\nthose duplicate print statements. Every surrounding control-flow statement,\nrecovery operation, error/status return, assertion, counter, structured tracing\nevent, and argument outside the print statements remains byte-for-byte unchanged.\nExisting tracing events preserve configurable recovery diagnostics. With the\nselected `RUST_LOG=off` environment, those events remain quiet.\n\nNo guard output is filtered or discarded by an extra adapter, and no resource\nlimit is raised. Explicit randomized progress, completion markers, libtest\nresults, other diagnostic output, and error assertions remain observable. Other\nunconditional prints outside this recovery method are unchanged; if they exceed\nthe same limit, qualification stops and retains that evidence.\n\nThe four Rust test/control payloads from randomized023 are reused byte-for-byte.\nThe corrected legacy `BITCOIN_RS_RANDOMIZED_GUARD=v1` admission token is supplied\nto all four replay and four bounded cases. Both original 40,000-operation cases\nstill require their dedicated explicit stress admission token, seed 301, private\ndata root, one CPU, zero swap, finite task/memory limits, and verified group cleanup.\n\nQualification compiles six exact test executables and executes twelve fresh\nselected cases: two pure admission contracts, four replay contracts, four bounded\ncases, and both full original workloads. It uses an independent file-copy cache\nseed from randomized023; it never hardlinks or writes the retained source cache.\nExisting allocation, snapshot lifetime, oracle cadence, and generator limitations\nremain as documented above. A source-only transformation check and Python/Git\nfixtures do not replace the required native execution on Meerkat.\n\nThe new candidate is committed only after all twelve cases pass and all original\nand retained evidence verifies. Master is never promoted by this script. A later\nidentical invocation may reverify an already complete025 candidate, but any new\nfailed, pending, or incomplete025 attempt remains blocked for separate review.\nThe terminal transport overwrites the authorized original `return-buffer` only\nafter the coordinator exits and printed/saved report identities validate.\n'
INITIAL_PATHS=frozenset(SOURCE_PAYLOADS) | {REPLAY_PATH, DOCUMENT_PATH}

def bootstrap_read(path, limit):
    path = Path(path)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Expected an absolute ordinary path.')
    fd = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:-1]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd); fd = child
        child = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=fd)
        with os.fdopen(child,'rb') as stream:
            before = os.fstat(stream.fileno())
            if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
                raise RuntimeError('File is indirect or too large.')
            raw = stream.read(limit+1)
            after = os.fstat(stream.fileno())
        fields = ('st_dev','st_ino','st_mode','st_size','st_mtime_ns','st_ctime_ns')
        current = path.lstat()
        if len(raw)>limit or any(getattr(before,k)!=getattr(after,k) or getattr(before,k)!=getattr(current,k) for k in fields):
            raise RuntimeError('File changed during read.')
        return raw
    finally:
        os.close(fd)


"""Exact source/index qualification for the isolated Randomized023 candidate.

These functions perform only bounded reads and read-only Git metadata queries.
The optional adapter configures the previously pinned finite Git worker; it does
not invoke it, install source, reserve a journal or read an identity by itself.
"""
import hashlib
import os
from pathlib import Path
import re


WORKFLOW = 'leveldb-randomized-quiet025'
BASE = '904156f818d38b443f02ff8fba6f2a2bfd5ed8a7'
BASE_TREE = '82ba98d00b662dc309447d13cd1bb366a0b39089'
BRANCH = 'repair/leveldb-randomized-quiet025'
SUFFIX = '-leveldb-randomized-quiet025'
REPLAY_PATH = 'contrib/repair/bitcoinleveldb-randomized-quiet025.py'
DOCUMENT_PATH = 'doc/testing/leveldb-randomized-quiet025.md'
RECEIPT_PATH = 'doc/repair/leveldb-randomized-quiet025.json'


def _candidate_initial023():
    """The embedding payload supplies its exact finite path inventory."""
    paths = globals().get('INITIAL_PATHS')
    if (not isinstance(paths, (list, tuple, set, frozenset)) or not paths
            or len(set(paths)) != len(paths) or not {REPLAY_PATH, DOCUMENT_PATH}.issubset(paths)
            or RECEIPT_PATH in paths):
        raise RuntimeError('Embedding randomized payload did not supply its exact initial path inventory.')
    for name in paths:
        if not isinstance(name, str): raise RuntimeError('Candidate source path is not text.')
        path = Path(name)
        if (not re.fullmatch(r'[A-Za-z0-9_./-]+', name) or path.is_absolute() or '..' in path.parts
                or str(path) != name or path.parts[0] == '.git'):
            raise RuntimeError('Candidate source path is outside its literal relative plan.')
    return frozenset(paths)


def _candidate_oid023(value):
    return isinstance(value, str) and re.fullmatch(r'[0-9a-f]{40}', value) is not None


def _candidate_blob023(raw):
    return hashlib.sha1(b'blob ' + str(len(raw)).encode() + b'\0' + raw).hexdigest()


def _candidate_files023(core, files):
    if (not isinstance(files, dict) or set(files) not in (_candidate_initial023(), _candidate_initial023() | {RECEIPT_PATH})
            or any(not isinstance(raw, bytes) or len(raw) > 2 * core.MIB for raw in files.values())):
        raise RuntimeError('Randomized candidate requires the exact planned initial/final bounded file payloads.')
    return files


def candidate_original_guard023(core, repo):
    """Verify original master identity while allowing unrelated staged/user work."""
    repo = Path(repo)
    if not repo.is_absolute() or '..' in repo.parts:
        raise RuntimeError('Original Randomized repository is not an absolute ordinary checkout.')
    fd = core.open_dir(repo); os.close(fd)
    inv = core.Inventory(repo); core.repository(inv)
    _, head = inv.git('rev-parse', '--verify', 'HEAD')
    _, branch = inv.git('symbolic-ref', '--quiet', 'HEAD')
    _, tree = inv.git('rev-parse', '--verify', 'HEAD^{tree}')
    if (head.decode().strip(), branch.decode().strip(), tree.decode().strip()) != (BASE, 'refs/heads/master', BASE_TREE):
        raise RuntimeError('Original master differs from the exact reviewed Randomized base/tree.')
    code, filters = inv.git('config', '--name-only', '--get-regexp', r'^filter\.', allowed=(0, 1))
    if code == 0 and filters.strip():
        raise RuntimeError('Configured Git filters require review before candidate inspection.')
    return inv


def _candidate_location023(core, repo, worktree, expected_head):
    repo, worktree = Path(repo), Path(worktree)
    if (not _candidate_oid023(expected_head) or not worktree.is_absolute() or '..' in worktree.parts
            or worktree != repo.parent / (repo.name + SUFFIX)):
        raise RuntimeError('Randomized candidate location or expected head differs from the isolated plan.')
    fd = core.open_dir(worktree); os.close(fd)
    inv = candidate_original_guard023(core, repo)
    _, top = inv.git('rev-parse', '--show-toplevel', cwd=worktree)
    _, common = inv.git('rev-parse', '--path-format=absolute', '--git-common-dir', cwd=worktree)
    _, head = inv.git('rev-parse', '--verify', 'HEAD', cwd=worktree)
    _, branch = inv.git('symbolic-ref', '--quiet', 'HEAD', cwd=worktree)
    if (Path(os.fsdecode(top).strip()) != worktree or Path(os.fsdecode(common).strip()) != repo / '.git'
            or head.decode().strip() != expected_head or branch.decode().strip() != 'refs/heads/' + BRANCH):
        raise RuntimeError('Randomized candidate top level, common root, branch or checkpoint differs.')
    code, filters = inv.git('config', '--name-only', '--get-regexp', r'^filter\.', allowed=(0, 1), cwd=worktree)
    if code == 0 and filters.strip():
        raise RuntimeError('Configured candidate Git filters require review before index/source inspection.')
    _, flags = inv.git('ls-files', '-v', '-z', cwd=worktree)
    if any(row and row[:2] != b'H ' for row in flags.split(b'\0')):
        raise RuntimeError('Randomized candidate index contains a hidden/nonstandard visibility flag.')
    return inv


def _candidate_reservation023(core, inv, worktree, files, expected_head):
    if expected_head != BASE or set(files) != _candidate_initial023():
        raise RuntimeError('Pristine reservation validation is restricted to BASE and the initial planned file scope.')
    _, status = inv.git('status', '--porcelain=v1', '-z', '--untracked-files=all', cwd=worktree)
    _, untracked = inv.git('ls-files', '--others', '-z', cwd=worktree)
    if status or untracked:
        raise RuntimeError('Randomized reservation must remain a pristine BASE checkout before installation.')
    _, tree = inv.git('ls-tree', '-r', '-z', BASE, '--', *sorted(files), cwd=worktree)
    existing = {}
    for row in tree.split(b'\0'):
        if not row: continue
        header, name = row.split(b'\t', 1); name = os.fsdecode(name)
        mode, kind, blob = header.decode('ascii').split()
        if name not in files or name in existing or mode not in {'100644', '100755'} or kind != 'blob' or not _candidate_oid023(blob):
            raise RuntimeError('Reserved randomized BASE inputs have an unexpected path/type.')
        existing[name] = (mode, blob)
    for name in sorted(set(files) | {RECEIPT_PATH}):
        path = worktree / name
        descriptor = core.open_dir(worktree)
        try:
            for part in Path(name).parts[:-1]:
                try:
                    child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=descriptor)
                except FileNotFoundError:
                    break
                os.close(descriptor); descriptor = child
        finally:
            os.close(descriptor)
        if name not in existing:
            if os.path.lexists(path):
                raise RuntimeError('A new randomized candidate path is occupied before source installation: ' + name)
            continue
        mode, blob = existing[name]
        _, committed = inv.git('cat-file', 'blob', blob, cwd=worktree)
        fingerprint, live = core.read_file(path, 2 * core.MIB)
        if live != committed or fingerprint['mode'] != (int(mode, 8) & 0o777):
            raise RuntimeError('Reserved randomized source differs from committed BASE bytes/mode: ' + name)
    return {'head': expected_head, 'branch': 'refs/heads/' + BRANCH, 'phase': 'pristine_base',
            'changed_paths': [], 'index_entries': {}}


def check_candidate023(core, repo, worktree, files, expectedhead, allow_base_files=False):
    """Validate pristine reservation, exact staged initial/final scope, or exact committed final scope."""
    if type(allow_base_files) is not bool:
        raise RuntimeError('Randomized reservation-mode flag must be boolean.')
    files = _candidate_files023(core, files)
    repo, worktree = Path(repo), Path(worktree)
    inv = _candidate_location023(core, repo, worktree, expectedhead)
    if allow_base_files:
        return _candidate_reservation023(core, inv, worktree, files, expectedhead)
    staged = expectedhead == BASE
    if not staged and set(files) != _candidate_initial023() | {RECEIPT_PATH}:
        raise RuntimeError('Committed Randomized qualification requires the complete planned final scope.')
    _, unstaged = inv.git('diff', '--name-only', '--no-ext-diff', '--no-textconv', '-z', cwd=worktree)
    _, untracked = inv.git('ls-files', '--others', '-z', cwd=worktree)
    if unstaged or untracked:
        raise RuntimeError('Randomized candidate has unstaged or untracked bytes outside its exact installed tree.')
    _, raw_staged = inv.git('diff', '--cached', '--name-only', '--no-ext-diff', '--no-textconv', '-z', cwd=worktree)
    changed = [os.fsdecode(item) for item in raw_staged.split(b'\0') if item]
    wanted = set(files) if staged else set()
    if set(changed) != wanted or len(changed) != len(wanted):
        raise RuntimeError('Randomized candidate staged paths differ from the exact installed scope.')
    expected = {}
    for name, data in files.items():
        fingerprint, live = core.read_file(worktree / name, 2 * core.MIB)
        mode = '100755' if name == REPLAY_PATH else '100644'
        if live != data or fingerprint['mode'] != (int(mode, 8) & 0o777):
            raise RuntimeError('Randomized candidate live bytes or mode differ: ' + name)
        expected[name] = [mode, _candidate_blob023(data)]
    _, raw_index = inv.git('ls-files', '--stage', '-z', '--', *sorted(files), cwd=worktree)
    indexed = {}
    for row in raw_index.split(b'\0'):
        if not row:
            continue
        header, name = row.split(b'\t', 1)
        mode, blob, stage = header.decode('ascii').split()
        name = os.fsdecode(name)
        if stage != '0' or name in indexed or not _candidate_oid023(blob):
            raise RuntimeError('Randomized candidate index has ambiguous/unmerged entries.')
        indexed[name] = [mode, blob]
    if indexed != expected:
        raise RuntimeError('Randomized candidate index blobs/modes differ from the reviewed live bytes.')
    result = {'head': expectedhead, 'branch': 'refs/heads/' + BRANCH,
              'phase': 'staged' if staged else 'committed', 'changed_paths': sorted(files), 'index_entries': indexed}
    if not staged:
        _, parents = inv.git('rev-list', '--parents', '-n', '1', expectedhead, cwd=worktree)
        if parents.decode().split() != [expectedhead, BASE]:
            raise RuntimeError('Randomized committed candidate must have the exact single BASE parent.')
        _, raw = inv.git('diff-tree', '--no-commit-id', '-r', '--raw', '-z', '--no-abbrev',
                         '--no-renames', '--no-ext-diff', '--no-textconv', BASE, expectedhead, cwd=worktree)
        parts = raw.split(b'\0')
        if parts and not parts[-1]: parts.pop()
        if len(parts) != 2 * len(files):
            raise RuntimeError('Randomized committed candidate has a different path count.')
        committed = {}
        for offset in range(0, len(parts), 2):
            fields = parts[offset].decode('ascii').split(); name = os.fsdecode(parts[offset + 1])
            if (len(fields) != 5 or not fields[0].startswith(':') or fields[4] not in {'A', 'M'}
                    or name in committed):
                raise RuntimeError('Randomized committed candidate has an unexpected operation.')
            committed[name] = [fields[1], fields[3]]
        if committed != expected:
            raise RuntimeError('Randomized committed candidate source/path/mode scope differs.')
        _, tree = inv.git('rev-parse', '--verify', expectedhead + '^{tree}', cwd=worktree)
        result['tree'] = tree.decode().strip()
    return result


def configure_git_helper023(helper):
    """Configure only the pinned015 generic finite Git/commit helper surface."""
    helper.WORKFLOW, helper.BASE, helper.BRANCH, helper.SUFFIX = WORKFLOW, BASE, BRANCH, SUFFIX
    helper.REPLAY_PATH, helper.DOCUMENT_PATH, helper.RECEIPT_PATH = REPLAY_PATH, DOCUMENT_PATH, RECEIPT_PATH
    helper._w015_initial_paths = lambda: set(_candidate_initial023())
    helper._w015_original = candidate_original_guard023
    def exact_candidate(core, repo, worktree, files, head, staged=True):
        if type(staged) is not bool or staged != (head == BASE):
            raise RuntimeError('Randomized Git helper requested a different candidate phase.')
        return check_candidate023(core, repo, worktree, files, head)
    helper.check_candidate = exact_candidate
    return helper


"""Finite Cargo observation for the controlled randomized library/integration-test build.

The parent must launch this observer under the reviewed resource guard. This
module never builds unless cargo_observer is explicitly invoked with its one exact
offline no-run command shape. Cargo stderr remains attached to that guard.
"""
import json
import os
from pathlib import Path
import re
import selectors
import subprocess
import sys
import time
import uuid
from urllib.parse import unquote, urlsplit

MAX_MACHINE_BYTES = 64 * 1024 * 1024
MAX_RECORD_BYTES = 1024 * 1024
MAX_RECORDS = 50000
MAX_EVIDENCE_BYTES = 512 * 1024
MAX_ERROR_RECORD_BYTES = 16 * 1024
MAX_ERROR_RECORDS = 32
MAX_ERROR_BYTES = 256 * 1024
MAX_WARNING_RENDERED_BYTES = 8 * 1024
MAX_ERROR_SPANS = 8
OBSERVER_SECONDS = 580
ALLOWED_PACKAGES = ("bitcoinleveldbt-randomized",)
RANDOMIZED_COMMAND = ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", "bitcoinleveldbt-randomized", "--no-run", "--message-format=json"]
_RANDOMIZED_COMMAND_SHAPE = tuple(RANDOMIZED_COMMAND)
RANDOMIZED_TARGETS = {
    "bitcoinleveldbt_randomized": ("lib", "src/lib.rs"),
    "randomized": ("test", "tests/randomized.rs"),
    "bisect": ("test", "tests/bisect.rs"),
    "bounded_contracts": ("test", "tests/bounded_contracts.rs"),
    "replay_contracts": ("test", "tests/replay_contracts.rs"),
    "execution_control_contracts": ("test", "tests/execution_control_contracts.rs"),
}


MAX_EXECUTABLES = 40


def _observer_spec(spec):
    if not isinstance(spec, dict):
        raise RuntimeError("Cargo observer specification is not an object.")
    command = spec.get("command")
    if not isinstance(command, list):
        raise RuntimeError("Cargo observer command must be the exact reviewed argument list.")
    if tuple(command) != _RANDOMIZED_COMMAND_SHAPE:
        raise RuntimeError("Cargo observer requires the one exact reviewed randomized library/integration-test command; no alternate target is admitted.")
    expected = ["bitcoinleveldbt-randomized"]
    targets = RANDOMIZED_TARGETS
    supplied = spec.get("expected_packages", expected)
    if not isinstance(supplied, list) or supplied != expected:
        raise RuntimeError("Cargo expected package must equal the singleton derived from the exact command.")
    paths = {}
    for key in ("cwd", "target_directory", "evidence_path"):
        value = spec.get(key)
        if not isinstance(value, str) or not value or "\x00" in value:
            raise RuntimeError("Cargo observer path is absent or malformed: " + key)
        path = Path(value)
        if not path.is_absolute() or ".." in path.parts:
            raise RuntimeError("Cargo observer requires an absolute path: " + key)
        paths[key] = path
    if not paths["cwd"].is_dir() or not paths["evidence_path"].parent.is_dir():
        raise RuntimeError("Cargo observer working or evidence directory is absent.")
    if os.path.lexists(paths["evidence_path"]):
        raise RuntimeError("Cargo observer refuses to replace existing evidence.")
    paths["expected_packages"] = tuple(expected)
    paths["expected_artifacts"] = {
        expected[0] + "::" + name: {"kind": kind, "source": paths["cwd"] / expected[0] / source}
        for name, (kind, source) in targets.items()
    }
    return list(command), expected, paths



def _atomic_evidence(path, record):
    raw = json.dumps(record, sort_keys=True, ensure_ascii=False, separators=(",", ":"), allow_nan=False).encode("utf-8") + b"\n"
    if len(raw) > MAX_EVIDENCE_BYTES:
        raise RuntimeError("Cargo evidence exceeds its retained-byte allowance.")
    path = Path(path)
    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        # Linking installs the completed file atomically and cannot overwrite a
        # concurrently created receipt. Both files are in the same directory.
        os.link(temporary, path, follow_symlinks=False)
    finally:
        try:
            temporary.unlink()
        except FileNotFoundError:
            pass


def _stop_cargo(child):
    if child.poll() is None:
        child.terminate()
        try:
            child.wait(timeout=2)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait(timeout=2)
    else:
        child.wait(timeout=2)


def _artifact(record, expected, paths, evidence):
    target = record.get("target")
    profile = record.get("profile")
    if not isinstance(target, dict) or not isinstance(profile, dict):
        raise RuntimeError("Cargo compiler artifact lacks target/profile data.")
    if target.get("kind") == ["custom-build"]:
        evidence["build_script_artifacts_discarded"] += 1
        return
    executable = record.get("executable")
    if executable is None:
        evidence["dependency_artifacts_discarded"] += 1
        return
    kind = target.get("kind")
    if profile.get("test") is not True or kind not in (["lib"], ["test"]):
        raise RuntimeError("Cargo emitted an executable outside the expected lib/test profile.")
    name = target.get("name")
    if not isinstance(name, str) or not re.fullmatch(r"[A-Za-z0-9_][A-Za-z0-9_-]{0,127}", name):
        raise RuntimeError("Cargo test target name is malformed.")
    source = target.get("src_path")
    if not isinstance(source, str) or not source or len(source) > 4096 or "\x00" in source:
        raise RuntimeError("Cargo test source path is absent or malformed.")
    source_path = Path(source)
    if not source_path.is_absolute() or ".." in source_path.parts:
        raise RuntimeError("Cargo test source path is not an absolute package-local path.")
    try:
        relative = source_path.relative_to(paths["cwd"])
    except ValueError as error:
        raise RuntimeError("Cargo test source is outside the private randomized worktree.") from error
    if len(relative.parts) < 2 or relative.parts[0] not in expected:
        raise RuntimeError("Cargo test source does not belong to a reviewed package.")
    package_name = relative.parts[0]
    package_id = record.get("package_id")
    if not isinstance(package_id, str) or not package_id or len(package_id) > 4096:
        raise RuntimeError("Cargo package identity is absent or too long.")
    # Modern Cargo path IDs may end in either #version or #name@version.
    # Source membership, rather than a guessed fragment format, selects the
    # package. A supplied path identity must independently name the same root.
    if package_id.startswith("path+"):
        identity = urlsplit(package_id[5:])
        identity_path = Path(unquote(identity.path))
        if (identity.scheme != "file" or identity.netloc not in ("", "localhost")
                or not identity_path.is_absolute() or ".." in identity_path.parts
                or identity_path != paths["cwd"] / package_name):
            raise RuntimeError("Cargo path package identity differs from its source package.")
    elif "://" in package_id:
        raise RuntimeError("Cargo expected local source has an external package identity.")
    key = package_name + "::" + name
    binding = paths["expected_artifacts"].get(key)
    if binding is None or kind != [binding["kind"]] or source_path != binding["source"]:
        raise RuntimeError("Cargo executable target kind/name/source differs from the exact selected command.")
    if key in evidence["artifacts"]:
        raise RuntimeError("Cargo emitted a duplicate package/target test executable.")
    if len(evidence["artifacts"]) >= MAX_EXECUTABLES:
        raise RuntimeError("Cargo test executable count exceeds its allowance.")
    if not isinstance(executable, str) or not executable or len(executable) > 4096 or "\x00" in executable:
        raise RuntimeError("Cargo executable path is malformed or too long.")
    executable_path = Path(executable)
    prefixes = {name, name.replace("-", "_")}
    filename_ok = any(re.fullmatch(re.escape(prefix) + r"-[0-9a-f]+", executable_path.name) for prefix in prefixes)
    if (not executable_path.is_absolute() or ".." in executable_path.parts
            or executable_path.parent != paths["target_directory"] / "debug" / "deps"
            or not filename_ok):
        raise RuntimeError("Cargo executable does not belong to the isolated debug test directory.")
    if any(item["executable"] == executable for item in evidence["artifacts"].values()):
        raise RuntimeError("Cargo reused an executable path for distinct package/target identities.")
    evidence["artifacts"][key] = {
        "package": package_name, "target_name": name, "kind": list(kind),
        "src_path": source, "executable": executable,
        "package_id": package_id, "fresh": record.get("fresh") is True,
    }


def _diagnostic_json(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False,
                      separators=(",", ":"), allow_nan=False).encode("utf-8")


def _diagnostic_text(value, allowance):
    if not isinstance(value, str):
        raise RuntimeError("Cargo diagnostic text has an unexpected type.")
    raw = value.encode("utf-8")
    retained = raw[:max(0, allowance)].decode("utf-8", errors="ignore")
    return retained, len(raw) - len(retained.encode("utf-8"))


def _diagnostic_path(value, paths):
    """Classify a lexical path without opening any compiler-provided filename."""
    if not isinstance(value, str) or "\x00" in value:
        raise RuntimeError("Cargo diagnostic path is absent or malformed.")
    if len(value.encode("utf-8")) > 4096:
        text, omitted = _diagnostic_text(value, 512)
        return {"file_name": text, "qualified_path": None,
                "path_scope": "overlong_path_omitted", "omitted_path_bytes": omitted}
    if value.startswith("<") and value.endswith(">"):
        return {"file_name": value, "qualified_path": None, "path_scope": "synthetic"}
    path = Path(value)
    if not value or ".." in path.parts:
        return {"file_name": value, "qualified_path": None,
                "path_scope": "unqualified_empty_or_parent_components"}
    absolute = path if path.is_absolute() else paths["cwd"] / path
    if len(str(absolute).encode("utf-8")) > 4096:
        return {"file_name": value, "qualified_path": None,
                "path_scope": "qualified_path_exceeds_allowance"}
    scope = "external"
    try:
        relative = absolute.relative_to(paths["target_directory"])
        scope = "target_output"
    except ValueError:
        try:
            relative = absolute.relative_to(paths["cwd"])
            scope = ("expected_package_source" if relative.parts
                     and relative.parts[0] in paths.get("expected_packages", ALLOWED_PACKAGES) else "other_worktree_source")
        except ValueError:
            pass
    return {"file_name": value, "qualified_path": str(absolute), "path_scope": scope,
            "qualification": "lexical_absolute" if path.is_absolute() else "lexical_observer_cwd",
            "file_bytes_verified": False}


def _diagnostic_target(record, paths):
    target = record.get("target")
    if not isinstance(target, dict):
        raise RuntimeError("Cargo compiler diagnostic lacks a target object.")
    name, omitted_name = _diagnostic_text(target.get("name", ""), 256)
    kinds = target.get("kind", [])
    if (not isinstance(kinds, list) or any(not isinstance(kind, str) for kind in kinds)):
        raise RuntimeError("Cargo compiler diagnostic target kind is malformed.")
    retained_kinds = [_diagnostic_text(kind, 64)[0] for kind in kinds[:8]]
    package, omitted_package = _diagnostic_text(record.get("package_id", ""), 2048)
    return {"name": name, "omitted_name_bytes": omitted_name,
            "kind": retained_kinds, "omitted_kinds": max(0, len(kinds) - 8),
            "src_path": _diagnostic_path(target.get("src_path", ""), paths),
            "package_id": package, "omitted_package_id_bytes": omitted_package}


def _priority_diagnostic(record, message, paths):
    level = message["level"]
    code = message.get("code")
    if code is not None:
        if not isinstance(code, dict) or not isinstance(code.get("code"), str):
            raise RuntimeError("Cargo compiler error code is malformed.")
        code, omitted_code = _diagnostic_text(code["code"], 128)
    else:
        omitted_code = 0
    text, omitted_message = _diagnostic_text(message.get("message", ""), 4096)
    spans = message.get("spans", [])
    if not isinstance(spans, list) or any(not isinstance(span, dict) for span in spans):
        raise RuntimeError("Cargo compiler diagnostic spans are malformed.")
    children = message.get("children", [])
    if not isinstance(children, list):
        raise RuntimeError("Cargo compiler diagnostic children are malformed.")
    result = {"level": level, "code": code, "message": text,
              "target": _diagnostic_target(record, paths), "spans": [], "rendered": "",
              "omitted_code_bytes": omitted_code, "omitted_message_bytes": omitted_message,
              "spans_observed": len(spans), "spans_retained": 0, "spans_omitted": len(spans),
              "children_observed": len(children), "children_retained_separately": False,
              "rendered_bytes_observed": 0, "rendered_bytes_retained": 0,
              "rendered_bytes_omitted": 0}
    # Retain primary locations before secondary locations, preserving order
    # within either group. No span opens or executes any named source.
    spans = sorted(spans, key=lambda span: span.get("is_primary") is not True)
    for span in spans:
        if len(result["spans"]) >= MAX_ERROR_SPANS:
            break
        location = _diagnostic_path(span.get("file_name", ""), paths)
        location["is_primary"] = span.get("is_primary") is True
        for field in ("byte_start", "byte_end", "line_start", "line_end", "column_start", "column_end"):
            value = span.get(field)
            if value is not None and (type(value) is not int or not 0 <= value <= 2 ** 63 - 1):
                raise RuntimeError("Cargo diagnostic span coordinate is malformed.")
            location[field] = value
        label = span.get("label")
        if label is not None:
            location["label"], location["omitted_label_bytes"] = _diagnostic_text(label, 512)
        candidate = dict(result, spans=result["spans"] + [location],
                         spans_retained=len(result["spans"]) + 1,
                         spans_omitted=len(spans) - len(result["spans"]) - 1)
        # Reserve a few bytes for subsequent decimal omission-count growth.
        if len(_diagnostic_json(candidate)) <= MAX_ERROR_RECORD_BYTES - 64:
            result = candidate
    rendered = message.get("rendered") or ""
    raw_rendered = rendered.encode("utf-8")
    result["rendered_bytes_observed"] = len(raw_rendered)
    # The exact encoded record, including JSON escaping and omission fields,
    # determines admission. Warning output never consumes this allowance.
    allowance = min(len(raw_rendered), MAX_ERROR_RECORD_BYTES)
    while True:
        snippet, omitted = _diagnostic_text(rendered, allowance)
        result.update(rendered=snippet, rendered_bytes_retained=len(snippet.encode("utf-8")),
                      rendered_bytes_omitted=omitted)
        excess = len(_diagnostic_json(result)) - MAX_ERROR_RECORD_BYTES
        if excess <= 0:
            return result
        if allowance == 0:
            return None
        allowance = max(0, allowance - max(excess, 64))


def _compiler_diagnostic(record, paths, evidence):
    message = record.get("message")
    if not isinstance(message, dict):
        raise RuntimeError("Cargo compiler message is not an object.")
    level = message.get("level")
    if level not in {"error", "warning", "note", "help", "failure-note", "ice"}:
        raise RuntimeError("Cargo compiler diagnostic level is absent or unknown.")
    rendered = message.get("rendered")
    if rendered is not None and not isinstance(rendered, str):
        raise RuntimeError("Cargo rendered diagnostic is not text.")
    evidence["diagnostic_records"] += 1
    evidence["diagnostic_levels"][level] = evidence["diagnostic_levels"].get(level, 0) + 1
    raw = (rendered or "").encode("utf-8")
    if level in {"error", "failure-note", "ice"}:
        evidence["priority_records_observed"] += 1
        candidate = _priority_diagnostic(record, message, paths)
        encoded = _diagnostic_json(candidate) if candidate is not None else b""
        reason = ("record_byte_allowance" if candidate is None else
                  "record_count_allowance" if evidence["priority_records_retained"] >= MAX_ERROR_RECORDS else
                  "total_byte_allowance" if evidence["priority_bytes_retained"] + len(encoded) > MAX_ERROR_BYTES else None)
        if reason is not None:
            evidence["priority_records_omitted"] += 1
            counts = evidence["priority_omission_reasons"]
            counts[reason] = counts.get(reason, 0) + 1
            snippet = b""
        else:
            evidence["priority_diagnostics"].append(candidate)
            evidence["priority_records_retained"] += 1
            evidence["priority_bytes_retained"] += len(encoded)
            snippet = candidate["rendered"].encode("utf-8")
            evidence["priority_rendered_bytes"] += len(snippet)
    else:
        allowance = max(0, MAX_WARNING_RENDERED_BYTES - evidence["warning_rendered_bytes"])
        snippet = _diagnostic_text(rendered or "", allowance)[0].encode("utf-8")
        evidence["warning_rendered_bytes"] += len(snippet)
    if snippet:
        sys.stderr.write(snippet.decode("utf-8"))
        sys.stderr.flush()
    evidence["rendered_bytes"] += len(snippet)
    evidence["rendered_bytes_omitted"] += len(raw) - len(snippet)


def _record(raw, expected, paths, evidence):
    if len(raw) > MAX_RECORD_BYTES:
        raise RuntimeError("Cargo machine record exceeds its per-record allowance.")
    if not raw.strip():
        return
    evidence["records"] += 1
    if evidence["records"] > MAX_RECORDS:
        raise RuntimeError("Cargo machine record count exceeds its allowance.")
    try:
        record = json.loads(raw.decode("utf-8"))
    except (ValueError, UnicodeError) as error:
        raise RuntimeError("Cargo stdout contains malformed machine JSON.") from error
    if not isinstance(record, dict):
        raise RuntimeError("Cargo machine record is not an object.")
    reason = record.get("reason")
    if reason == "compiler-artifact":
        _artifact(record, expected, paths, evidence)
    elif reason == "build-script-executed":
        evidence["build_script_records_discarded"] += 1
    elif reason == "compiler-message":
        _compiler_diagnostic(record, paths, evidence)
    elif reason == "build-finished":
        if evidence["build_finished"] is not None or type(record.get("success")) is not bool:
            raise RuntimeError("Cargo build-finished record is duplicate or malformed.")
        evidence["build_finished"] = record["success"]
    elif reason == "future-incompat-report":
        evidence["other_records_discarded"] += 1
    else:
        raise RuntimeError("Cargo emitted an unknown machine-record reason.")


def cargo_observer(spec):
    """Run only the fixed offline compilation command and retain small evidence.

    Return 0 only when the exact command succeeded and emitted its complete selected target inventory.
    On malformed output or an output allowance violation, stop Cargo promptly.
    """
    evidence = {
        "schema": "bitcoinrs-randomized025-cargo-observer-v1", "command": None,
        "cwd": None, "target_directory": None, "expected_packages": [],
        "cargo_exit_code": None, "observer_exit_code": 2, "error": None,
        "artifacts": {}, "machine_bytes": 0, "records": 0,
        "dependency_artifacts_discarded": 0, "build_script_records_discarded": 0,
        "build_script_artifacts_discarded": 0,
        "diagnostic_records": 0, "other_records_discarded": 0,
        "rendered_bytes": 0, "rendered_bytes_omitted": 0,
        "diagnostic_levels": {}, "priority_diagnostics": [],
        "priority_records_observed": 0, "priority_records_retained": 0,
        "priority_records_omitted": 0, "priority_bytes_retained": 0,
        "priority_omission_reasons": {}, "warning_rendered_bytes": 0,
        "priority_rendered_bytes": 0,
        "build_finished": None,
    }
    child, paths = None, None
    try:
        command, expected, paths = _observer_spec(spec)
        evidence.update(command=command, cwd=str(paths["cwd"]),
                        target_directory=str(paths["target_directory"]), expected_packages=expected)
        env = dict(os.environ)
        env.update(CARGO_TARGET_DIR=str(paths["target_directory"]), CARGO_BUILD_JOBS="1", CARGO_INCREMENTAL="0")
        child = subprocess.Popen(command, cwd=paths["cwd"], env=env,
                                 stdin=subprocess.DEVNULL, stdout=subprocess.PIPE)
        os.set_blocking(child.stdout.fileno(), False)
        buffer = bytearray()
        deadline = time.monotonic() + OBSERVER_SECONDS
        with selectors.DefaultSelector() as selector:
            selector.register(child.stdout, selectors.EVENT_READ)
            while selector.get_map():
                if time.monotonic() >= deadline:
                    raise RuntimeError("Cargo observer reached its wall-clock allowance.")
                for key, _ in selector.select(0.1):
                    try:
                        chunk = os.read(key.fileobj.fileno(), 65536)
                    except BlockingIOError:
                        continue
                    if not chunk:
                        selector.unregister(key.fileobj)
                        if buffer:
                            _record(bytes(buffer), expected, paths, evidence)
                            buffer.clear()
                        continue
                    evidence["machine_bytes"] += len(chunk)
                    if evidence["machine_bytes"] > MAX_MACHINE_BYTES:
                        raise RuntimeError("Cargo machine stdout exceeds its total-byte allowance.")
                    buffer.extend(chunk)
                    while b"\n" in buffer:
                        newline = buffer.index(b"\n")
                        _record(bytes(buffer[:newline]), expected, paths, evidence)
                        del buffer[:newline + 1]
                    if len(buffer) > MAX_RECORD_BYTES:
                        raise RuntimeError("Cargo machine record exceeds its per-record allowance.")
        remaining = max(0.1, deadline - time.monotonic())
        evidence["cargo_exit_code"] = child.wait(timeout=remaining)
        if evidence["cargo_exit_code"] != 0:
            raise RuntimeError("Cargo exited unsuccessfully.")
        if evidence["build_finished"] is not True:
            raise RuntimeError("Cargo did not report a successful completed build.")
        # rustc may label explanatory output (for example --explain E0133
        # following an unsafe-op warning) as failure-note even on success.
        # Keep that output prioritized, but use error/ICE plus Cargo's actual
        # process/build-finished result to decide whether compilation passed.
        if (evidence["diagnostic_levels"].get("error", 0)
                or evidence["diagnostic_levels"].get("ice", 0)):
            raise RuntimeError("Cargo reported error diagnostics despite a successful process/build result.")
        if set(evidence["artifacts"]) != set(paths["expected_artifacts"]):
            raise RuntimeError("Cargo did not emit the exact complete executable inventory for its selected command.")
        evidence["observer_exit_code"] = 0
    except Exception as error:
        evidence["error"] = (type(error).__name__ + ": " + str(error))[:2048]
        if child is not None:
            try:
                _stop_cargo(child)
            except Exception as stop_error:
                evidence["cleanup_error"] = str(stop_error)[:512]
            evidence["cargo_exit_code"] = child.poll()
    finally:
        if child is not None and child.stdout is not None:
            child.stdout.close()
    if paths is None:
        sys.stderr.write("RANDOMIZED025_CARGO_OBSERVER_REJECTED " + (evidence["error"] or "invalid specification") + "\n")
        return 2
    try:
        _atomic_evidence(paths["evidence_path"], evidence)
    except Exception as error:
        sys.stderr.write("RANDOMIZED025_CARGO_EVIDENCE_ERROR " + str(error)[:1024] + "\n")
        return 3
    print("RANDOMIZED025_CARGO_OBSERVER records=" + str(evidence["records"])
          + " executables=" + str(len(evidence["artifacts"]))
          + " native_exit=" + str(evidence["cargo_exit_code"])
          + " observer_exit=" + str(evidence["observer_exit_code"]), flush=True)
    return evidence["observer_exit_code"]


"""Read-only proof of the retained023 output stop; never clears its journal."""
import hashlib
import json
from pathlib import Path
import re
import sys
import types

MIB_PRIOR025 = 1024 * 1024
OWNER_PRIOR025 = 'leveldb-randomized-controlled023'
BASE_PRIOR025 = '904156f818d38b443f02ff8fba6f2a2bfd5ed8a7'
TREE_PRIOR025 = '82ba98d00b662dc309447d13cd1bb366a0b39089'
CAPTURE_RUN025 = '20261108T164953Z-4454a923'
CAPTURE_SHA025 = '27df39f627a5c3afd80d7be79547cb8549a84d8897db336ac759d4e175b800ce'
CAPTURE_BYTES025 = 2995957
NATIVE_RUN025 = '20261108T160858Z-d4ff9dd1'
NATIVE_SHA025 = 'dc7e22f745abdff8db57d8a746afdfdcd355d14528c1204013f47853ca60616b'
NATIVE_BYTES025 = 1619357
STATE_SHA025 = '151fde201f8bafeac9c6832ebd92931bf34c572ab50acadd6b0d685b7df97bd5'
STATE_BYTES025 = 44088
CONTROLLER_SHA025 = '4e17f9c6c84bd1b76f15d7420cc02808281431518a305bb9ed00cd41bdf63a94'
CONTROLLER_BYTES025 = 199837
CASE_PRIOR025 = 'db_test_randomized'


def _prior_encoded025(value):
    return (json.dumps(value, sort_keys=True, indent=2) + '\n').encode()


def _prior_digest025(raw):
    return hashlib.sha256(raw).hexdigest()


def _load_controller_prior025(raw, path):
    """Load only a byte-pinned historical module's definitions, not its main."""
    module = types.ModuleType('randomized025_pinned_prior_controller')
    module.__file__ = str(path)
    sys.modules[module.__name__] = module
    exec(compile(raw, str(path), 'exec'), module.__dict__)
    return module


def _reconstruct_output_prior025(prefix, tail, observed):
    if len(prefix) != MIB_PRIOR025 or len(tail) != 65536 or observed != 1049927:
        raise RuntimeError('Prior capped prefix, retained tail or observed byte count differs.')
    # The exact full length fixes the sole possible suffix/prefix overlap.
    overlap = len(prefix) + len(tail) - observed
    if overlap != 64185 or prefix[-overlap:] != tail[:overlap]:
        raise RuntimeError('Prior capped log and retained tail do not reconstruct the complete observed output.')
    return prefix + tail[overlap:], overlap


def _classify_output_stop_prior025(core, report, old, prefix):
    selected = old.choose_cases023(report['test_lists']['all'], report['test_lists']['ignored'])
    cases, rows = report.get('cases', []), report.get('readouts', {})
    if (report.get('workflow') != OWNER_PRIOR025 or report.get('controller_revision') != 2
            or report.get('outcome') != 'failed' or report.get('phase') != CASE_PRIOR025
            or report.get('test_count') != 11 or report.get('expected_test_count') != 12
            or report.get('repair_qualified') is not False or report.get('native_tests_started') is not True
            or report.get('original_preserved') is not True or report.get('master_updated') is not False
            or report.get('other_refs_updated') is not False or report.get('validation_reused') is not False
            or report.get('completed_full_cases') != [] or report.get('original_full_cases_completed') is not False
            or report.get('full_randomized_stress') != 'not rerun'
            or report.get('preservation_before_sha256') != report.get('preservation_after_sha256')
            or report.get('intake_before') != report.get('intake_after')
            or report.get('controller_transition_reverified') is not True
            or len(selected) != 12 or len(cases) != 11
            or selected[10]['suffix'] != CASE_PRIOR025 or selected[10]['profile'] != 'full_original'
            or selected[11]['profile'] != 'full_original'):
        raise RuntimeError('Historical return is not the exact preserved eleventh-case output stop.')
    phase_names = {'preflight', 'inputs-before', 'read-configured-identity', 'refresh-controller',
                   'git-add', 'compile-randomized-tests'}
    phase_names.update('list-' + target + '-' + group for target in old.RANDOMIZED_TARGETS for group in ('all', 'ignored'))
    phase_names.update(item['suffix'] for item in selected[:11])
    if set(rows) != phase_names or len(rows) != 29:
        raise RuntimeError('Historical native attempt has an unreviewed process/phase inventory.')
    for label, row in rows.items():
        wanted = 'output_limit' if label == CASE_PRIOR025 else 'exited'
        if row.get('verified') is not True or row.get('outcome') != wanted or row.get('exit_code') != 0:
            raise RuntimeError('Historical attempt contains a different process/control failure.')
    for index, (case, item) in enumerate(zip(cases, selected[:11])):
        if any(case.get(k) != v for k, v in {'label': item['suffix'], 'expected_name': item['name'],
                'target': item['target'], 'profile': item['profile'], 'marker': item['marker'],
                'ignored_by_default': item['profile'] == 'full_original'}.items()):
            raise RuntimeError('Historical selected case order or metadata differs.')
        assessed = core.assess_case(rows[item['suffix']], case['log'], item['name'], item['marker'])
        if any(case.get(k) != assessed.get(k) for k in ('outcome', 'passed', 'failed', 'ignored', 'error')):
            raise RuntimeError('Historical recorded assessment contradicts its retained log.')
        if case['outcome'] != ('failed' if index == 10 else 'passed'):
            raise RuntimeError('Historical cases are not ten passes followed by the controlled output stop.')
    if set(report.get('case_guards', {})) != {item['suffix'] for item in selected[:10]}:
        raise RuntimeError('Historical successful-case guard inventory differs.')
    cargo = report.get('cargo_evidence', {})
    expected = {'bitcoinleveldbt-randomized::' + target for target in old.RANDOMIZED_TARGETS}
    if (report.get('build_command') != old.RANDOMIZED_COMMAND or cargo.get('command') != old.RANDOMIZED_COMMAND
            or cargo.get('cargo_exit_code') != 0 or cargo.get('observer_exit_code') != 0
            or cargo.get('build_finished') is not True or cargo.get('error') is not None
            or cargo.get('priority_records_omitted') != 0 or set(cargo.get('artifacts', {})) != expected
            or set(report.get('artifacts', {})) != expected
            or any(cargo.get('diagnostic_levels', {}).get(level, 0) for level in ('error', 'ice'))):
        raise RuntimeError('Historical successful six-executable build differs.')
    row = rows[CASE_PRIOR025]
    full, overlap = _reconstruct_output_prior025(prefix, row['tail'].encode(), row['output_bytes'])
    if (row.get('kind') != 'stress' or row.get('budget') != core.load_guard().BUDGETS['stress']
            or row['budget']['output'] != MIB_PRIOR025 or row.get('elapsed_seconds') >= row['budget']['seconds']
            or row.get('data_bytes', 1 << 60) > row['budget']['data']
            or row.get('data_entries', 1 << 60) > row['budget']['entries']
            or row.get('stop_request') != {'exit_code': 0, 'stdout': '', 'stderr': ''}
            or b'Main processes terminated with: code=killed, status=15/TERM' not in full
            or b'RANDOMIZED_STRESS_PROGRESS case=db_test_randomized config=1 phase=iteration done=6500 total=10000' not in full
            or any(token in full for token in (b'panicked at ', b'test result: FAILED', b'RANDOMIZED_STRESS_OK'))):
        raise RuntimeError('Historical workload stop is not solely the reviewed output-limit interruption.')
    snapshots = [row['probe']['kernel']] + row.get('live_cgroup', [])
    if not row.get('live_cgroup') or any(any(sample.get('memory.events', {}).get(key) != 0
            for key in ('max', 'oom', 'oom_kill', 'oom_group_kill')) for sample in snapshots):
        raise RuntimeError('Historical retained memory-event observations differ from the reviewed output stop.')
    return {'classification': 'guard_output_limit_during_original_database_workload',
            'native_cases_invoked': 11, 'native_cases_passed': 10, 'native_cases_expected': 12,
            'failed_case': CASE_PRIOR025, 'full_original_completed': False,
            'observed_output_bytes': len(full), 'output_limit_bytes': MIB_PRIOR025,
            'capped_log_bytes': len(prefix), 'retained_tail_bytes': 65536, 'overlap_bytes': overlap,
            'reconstructed_output_sha256': _prior_digest025(full),
            'last_recorded_iteration': {'configuration': 1, 'done': 6500, 'total': 10000},
            'memory_oom_events_observed': 0, 'memory_snapshot_count': len(snapshots),
            'test_validation_reused': False, 'old_failure_cleared': False,
            'cause_of_historical_host_freeze_established': False}


def retained_output_stop025(core, repo):
    """Verify frozen old inputs and return a stable proof without modifying them."""
    repo = Path(repo)
    if not repo.is_absolute() or '..' in repo.parts:
        raise RuntimeError('Prior evidence repository is not an ordinary absolute path.')
    owned = repo / '.git/codex-repairs' / OWNER_PRIOR025
    run = owned / 'runs' / NATIVE_RUN025
    capture_path = repo / '.git/codex-repairs/leveldb-randomized-state024/runs' / CAPTURE_RUN025 / 'STATE_EVIDENCE_RETURN.json'
    bindings = {}
    def read(path, limit, expected=None, content=True):
        fingerprint, raw = core.read_file(path, limit, content=content)
        if expected is not None and fingerprint != expected:
            raise RuntimeError('Frozen prior evidence bytes/mode differ: ' + str(path))
        bindings[str(path)] = fingerprint
        return fingerprint, raw
    capture_fp, raw = read(capture_path, 3 * MIB_PRIOR025)
    if capture_fp['size'] != CAPTURE_BYTES025 or capture_fp['sha256'] != CAPTURE_SHA025:
        raise RuntimeError('Pinned state024 capture bytes differ.')
    capture = json.loads(raw); evidence = capture['evidence_capture']
    if (capture.get('workflow') != 'leveldb-randomized-state024' or capture.get('outcome') != 'captured'
            or capture.get('phase') != 'complete' or capture.get('original_preserved') is not True
            or any(capture.get(k) is not False for k in ('native_tests_started', 'builds_started', 'source_edits_started'))
            or capture.get('repository') != str(repo) or capture.get('return_file') != str(capture_path)
            or evidence.get('repository') != str(repo) or evidence.get('base') != BASE_PRIOR025):
        raise RuntimeError('Pinned state024 capture identity/preservation differs.')
    retained = evidence['runs'][NATIVE_RUN025]
    report_fp, report_raw = read(run / 'RANDOMIZED_REPAIR_RETURN.json', 2 * MIB_PRIOR025, retained['report_fingerprint'])
    if report_fp['size'] != NATIVE_BYTES025 or report_fp['sha256'] != NATIVE_SHA025:
        raise RuntimeError('Pinned earlier revision002 native report differs.')
    report = json.loads(report_raw)
    if report != retained['report'] or report.get('repository') != str(repo) or report.get('run_directory') != str(run):
        raise RuntimeError('Actual earlier revision002 report differs from the read-only capture.')
    controller_path = run / 'randomized-controller023.py'
    controller_fp, controller_raw = read(controller_path, 2 * MIB_PRIOR025, retained['controller']['fingerprint'])
    if (controller_fp['size'] != CONTROLLER_BYTES025 or controller_fp['sha256'] != CONTROLLER_SHA025
            or report.get('controller_sha256') != CONTROLLER_SHA025):
        raise RuntimeError('Pinned earlier revision002 controller differs.')
    state_path = owned / 'state.json'
    state_fp, state_raw = read(state_path, MIB_PRIOR025, evidence['state']['fingerprint'])
    if (state_fp['size'] != STATE_BYTES025 or state_fp['sha256'] != STATE_SHA025
            or state_raw != _prior_encoded025(evidence['state']['value'])):
        raise RuntimeError('The retained023 failed/pending state is no longer its exact frozen value.')
    state = json.loads(state_raw); failure = {'run': str(run), 'case': CASE_PRIOR025, 'profile': 'full_original'}
    expected_identity = {'workflow': OWNER_PRIOR025, 'base': BASE_PRIOR025, 'branch': 'repair/leveldb-randomized-controlled023',
                         'worktree': str(repo.parent / (repo.name + '-leveldb-randomized-controlled023')),
                         'file_hashes': report['source_plan'], 'intake_sha256': core.digest(core.encoded(report['intake_before'])),
                         'controller_revision': 2}
    if (set(state) != {'identity', 'phase', 'controller_transition', 'native_failure', 'native_pending'}
            or state['identity'] != expected_identity or state['phase'] != 'installed'
            or state['native_failure'] != failure or state['native_pending'] != failure
            or state['controller_transition'] != report.get('controller_transition_before')
            or evidence['references'] != {'native_failure': failure, 'native_pending': failure, 'qualified_report': None}):
        raise RuntimeError('Retained failure/history is not the exact unqualified eleventh original case.')
    old = _load_controller_prior025(controller_raw, controller_path)
    worktree = Path(expected_identity['worktree']); old_files = old.files023(controller_raw)
    if (set(old_files) != set(evidence['candidate']['files'])
            or report['source_plan'] != {name: core.digest(data) for name, data in old_files.items()}):
        raise RuntimeError('Historical candidate payload/source inventory differs.')
    for name, data in old_files.items():
        saved = evidence['candidate']['files'][name]
        fp, live = read(worktree / name, 2 * MIB_PRIOR025, saved['fingerprint'])
        if live != data or fp['sha256'] != report['source_plan'][name] or not saved['matches_journal_sha256'] or not saved['matches_expected_mode']:
            raise RuntimeError('Historical candidate source bytes/mode differ: ' + name)
    candidate = old.check_candidate023(core, repo, worktree, old_files, BASE_PRIOR025)
    captured_git = evidence['candidate']['git']
    indexed = {name: [[entry['mode'], entry['blob'], entry['stage']] for entry in entries]
               for name, entries in captured_git['index_entries'].items()}
    expected_index = {name: [[value[0], value[1], '0']] for name, value in candidate['index_entries'].items()}
    if (candidate['phase'] != 'staged' or captured_git.get('head') != BASE_PRIOR025 or captured_git.get('tree') != TREE_PRIOR025
            or captured_git.get('branch') != candidate['branch'] or captured_git.get('common_directory') != str(repo / '.git')
            or indexed != expected_index):
        raise RuntimeError('Retained023 candidate branch/index differs from the captured exact staged payload.')
    selected = retained['selected_rows'][CASE_PRIOR025]
    row = report['readouts'][CASE_PRIOR025]; group = Path(row['log']).parent
    if (group.parent != run / 'guard' or not re.fullmatch(r'db_test_randomized-[0-9a-f]{12}', group.name)
            or selected['result'] != row or selected['result_matches_report'] is not True):
        raise RuntimeError('Historical failed guard group/result differs from the captured report.')
    values = {}
    for leaf in ('output.log', 'plan.json', 'probe.json', 'permit', 'result.json'):
        saved = selected['guard_files'][leaf]
        if saved['path'] != str(group / leaf): raise RuntimeError('Historical guard input path differs.')
        _, value = read(group / leaf, MIB_PRIOR025, saved['fingerprint'])
        values[leaf] = value
    plan = json.loads(values['plan.json']); probe = json.loads(values['probe.json'])
    item = old.choose_cases023(report['test_lists']['all'], report['test_lists']['ignored'])[10]
    data = run / ('case-data-' + CASE_PRIOR025)
    command = old.case_command023(report['artifacts']['bitcoinleveldbt-randomized::randomized']['path'], item)
    environment = old.case_environment023(item, data)
    if (plan.get('kind') != 'stress' or plan.get('command') != command or row['command'] != command
            or plan.get('cwd') != str(worktree) or row['cwd'] != str(worktree) or row['data_directory'] != str(data)
            or not isinstance(plan.get('env'), dict) or any(plan['env'].get(k) != v for k, v in environment.items())
            or plan.get('probe') != str(group / 'probe.json') or plan.get('permit') != str(group / 'permit')
            or not isinstance(plan.get('nonce'), str) or not re.fullmatch(r'[0-9a-f]{32}', plan['nonce'])
            or values['permit'] != plan['nonce'].encode() or probe != row['probe']
            or json.loads(values['result.json']) != row or values['output.log'].decode() != selected['log']):
        raise RuntimeError('Historical output-stop plan/probe/permit/result differs from the exact controlled invocation.')
    result = _classify_output_stop_prior025(core, report, old, values['output.log'])
    for key, saved in retained['cargo'].items():
        fp, value = read(run / key, MIB_PRIOR025, saved['fingerprint'])
        if saved['path'] != str(run / key) or json.loads(value) != saved['value']:
            raise RuntimeError('Historical external Cargo evidence differs.')
    if retained['cargo']['cargo-artifacts.json']['value'] != report['cargo_evidence']:
        raise RuntimeError('Historical Cargo observer evidence differs from its report.')
    for label, guard in report['case_guards'].items():
        for path, expected in guard['guard_files'].items():
            read(Path(path), MIB_PRIOR025, expected)
    artifacts = {}
    target = owned / 'cargo-target'
    for key, item in report['artifacts'].items():
        name = key.split('::', 1)[1]; path = Path(item['path']); compiled = report['cargo_evidence']['artifacts'][key]
        kind, source = old.RANDOMIZED_TARGETS[name]
        if (path.parent != target / 'debug/deps' or not re.fullmatch(re.escape(name) + r'-[0-9a-f]+', path.name)
                or compiled.get('executable') != str(path) or compiled.get('kind') != [kind]
                or compiled.get('package') != 'bitcoinleveldbt-randomized' or compiled.get('target_name') != name
                or compiled.get('src_path') != str(worktree / 'bitcoinleveldbt-randomized' / source)):
            raise RuntimeError('Historical executable binding differs.')
        fp, _ = read(path, 256 * MIB_PRIOR025, item['fingerprint'], content=False)
        if not fp['mode'] & 0o111: raise RuntimeError('Historical executable mode differs.')
        artifacts[str(path)] = fp
    result.update(schema='bitcoinrs-randomized025-retained-output-stop-v1',
                  repository=str(repo), retained_workflow=OWNER_PRIOR025,
                  capture_path=str(capture_path), capture_fingerprint=capture_fp,
                  native_report_path=str(run / 'RANDOMIZED_REPAIR_RETURN.json'), native_report_fingerprint=report_fp,
                  state_path=str(state_path), state_fingerprint=state_fp,
                  controller_path=str(controller_path), controller_fingerprint=controller_fp,
                  retained_bindings=bindings, artifact_bindings=artifacts,
                  artifact_bytes_verified=sum(fp['size'] for fp in artifacts.values()),
                  candidate_index_sha256=core.digest(core.encoded(candidate)),
                  source_plan=report['source_plan'])
    return result


"""Pinned runtime, finite worker inputs and exact native evidence for repair025."""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import types
import uuid


def load_checked023(name, path, expected, expected_bytes=None):
    raw = bootstrap_read(path, 2 * MIB023)
    if (hashlib.sha256(raw).hexdigest() != expected
            or (expected_bytes is not None and len(raw) != expected_bytes)):
        raise RuntimeError('Retained reviewed controller differs: ' + path.name)
    module = types.ModuleType(name); module.__file__ = str(path); sys.modules[name] = module
    exec(compile(raw, str(path), 'exec'), module.__dict__)
    return module


def runtime023(repo):
    retained = repo / '.git/codex-repairs'
    prior = load_checked023('randomized023_runtime020',
        retained / 'leveldb-issue320-controlled020/runs/20261108T074029Z-db4c138d/issue320-controller020.py',
        '5a18ae65a4889f8eeb4c0a100aa85fb22092ea7df1b5c6a5c4c3383a1c4eff7d', 207952)
    finalizer, core, helper, _ = prior.runtime020(repo)
    configure_git_helper023(helper)
    core.WORKFLOW, core.BRANCH, core.WORKTREE_SUFFIX = WORKFLOW, BRANCH, SUFFIX
    intake = load_checked023('randomized023_source022',
        retained / 'leveldb-randomized-source022/runs/20261108T143903Z-74099602/randomized-source-controller022.py',
        '67adc16ef92827a19ebaacdcae7939da63e510cc507b55aeef26c0c38eee6dae', 62120)
    return finalizer, core, helper, intake


def checked_environment023(core):
    core.checked_environment()
    exact = {'CARGO_BUILD_TARGET', 'CARGO_BUILD_RUSTFLAGS', 'CARGO_BUILD_RUSTC', 'CARGO_BUILD_RUSTC_WRAPPER',
             'RUSTC', 'RUSTDOC', 'RUSTUP_TOOLCHAIN'}
    bad = [key for key, value in os.environ.items() if value and
           (key in exact or key.startswith('CARGO_PROFILE_') or
            (key.startswith('CARGO_TARGET_') and key.endswith(('_RUSTFLAGS', '_RUSTC_WRAPPER', '_RUNNER'))))]
    if bad: raise RuntimeError('Build overrides require review: ' + repr(sorted(bad)))


def intake_binding023(core, intake, repo, full=False):
    if type(full) is not bool: raise RuntimeError('Source verification mode must be boolean.')
    candidate_original_guard023(core, repo)
    directory = repo / '.git/codex-repairs/leveldb-randomized-source022/runs/20261108T143903Z-74099602'
    path = directory / 'RANDOMIZED_SOURCE_RETURN.json'
    fp, raw = core.read_file(path, 2 * MIB023)
    if fp['size'] != 819777 or fp['sha256'] != '53889a8cc00418f6c4a836be87dcd9f881b5268e953407858451539723acac63':
        raise RuntimeError('Retained successful randomized source intake differs.')
    report = json.loads(raw)
    expected = {'workflow': 'leveldb-randomized-source022', 'controller_revision': 1,
                'outcome': 'captured', 'phase': 'complete', 'source_capture_complete': True,
                'original_preserved': True, 'base': BASE, 'base_tree': BASE_TREE,
                'builds_started': False, 'native_tests_started': False, 'repair_qualified': False}
    if any(report.get(key) != value for key, value in expected.items()):
        raise RuntimeError('Retained intake is not the complete reviewed source-only capture.')
    source = report['source_capture']
    if (source.get('source_omissions') != [] or source.get('source_text_complete') is not True
            or source.get('input_count') != 90 or len(source.get('files', {})) != 90
            or report.get('baseline_before') != report.get('baseline_after')
            or report.get('preservation_before_sha256') != report.get('preservation_after_sha256')):
        raise RuntimeError('Retained randomized source input inventory or preservation differs.')
    if not isinstance(ORIGINAL_SOURCE_HASHES, dict) or not ORIGINAL_SOURCE_HASHES:
        raise RuntimeError('Exact original randomized source bindings are absent.')
    for name, expected_sha in ORIGINAL_SOURCE_HASHES.items():
        row = source['files'].get(name, {})
        if (not isinstance(expected_sha, str) or not re.fullmatch(r'[0-9a-f]{64}', expected_sha)
                or row.get('live_fingerprint', {}).get('sha256') != expected_sha
                or row.get('committed_sha256') != expected_sha
                or row.get('live_matches_committed') is not True
                or row.get('live_mode_matches_committed') is not True):
            raise RuntimeError('Original source differs from the reviewed committed/live input: ' + name)
    if full:
        proof = intake.verify_sources022(core, repo, source)
        if proof.get('inputs_preserved') is not True or proof.get('fingerprint_count') != 90:
            raise RuntimeError('Current original source inputs differ from the reviewed intake.')
    return {'report': str(path), 'fingerprint': fp,
            'original_source_hashes': dict(ORIGINAL_SOURCE_HASHES), 'input_count': 90,
            'quiet_recovery_input': quiet_recovery_input025(core, repo),
            'retained_output_stop': retained_output_stop025(core, repo),
            'qualification': 'selected_source_and_prior_stop_preservation_only'}



def quiet_recovery_input025(core, repo):
    """Bind the reviewed recovery diagnostic source to current BASE/live/index."""
    inv = candidate_original_guard023(core, repo)
    if set(QUIET_RECOVERY_ORIGINAL_HASHES) != {'bitcoinleveldb-dbimpl/src/recover_log_file.rs'}:
        raise RuntimeError('Quiet recovery replacement scope differs.')
    proof = {}
    for name, expected_sha in QUIET_RECOVERY_ORIGINAL_HASHES.items():
        _, tree = inv.git('ls-tree', '-r', '-l', '-z', BASE, '--', name)
        rows = [row for row in tree.split(b'\0') if row]
        if len(rows) != 1: raise RuntimeError('Committed recovery source is absent or ambiguous.')
        header, tree_name = rows[0].split(b'\t', 1)
        mode, kind, blob, size = header.decode('ascii').split()
        if (tree_name.decode() != name or mode != '100644' or kind != 'blob'
                or re.fullmatch(r'[0-9a-f]{40}', blob) is None
                or not size.isdecimal() or int(size) > MIB023):
            raise RuntimeError('Committed recovery source mode/type/size differs.')
        _, committed = inv.git('cat-file', 'blob', blob)
        fp, live = core.read_file(repo / name, MIB023)
        _, index = inv.git('ls-files', '--stage', '-z', '--', name)
        expected_index = ('100644 ' + blob + ' 0\t' + name + '\0').encode()
        if (len(committed) != int(size) or core.digest(committed) != expected_sha
                or live != committed or fp['mode'] != 0o644 or index != expected_index):
            raise RuntimeError('Reviewed recovery source differs from committed/live/index input: ' + name)
        proof[name] = {'live_fingerprint': fp, 'committed_blob': blob,
                       'committed_sha256': expected_sha, 'index_sha256': core.digest(index),
                       'quiet_output_sha256': core.digest(SOURCE_PAYLOADS[name])}
    return {'files': proof, 'diagnostic_change': QUIET_RECOVERY_POLICY}

def files023(raw):
    if (not isinstance(raw, bytes) or not isinstance(SOURCE_PAYLOADS, dict)
            or any(not isinstance(value, bytes) for value in SOURCE_PAYLOADS.values())
            or REPLAY_PATH in SOURCE_PAYLOADS or DOCUMENT_PATH in SOURCE_PAYLOADS
            or RECEIPT_PATH in SOURCE_PAYLOADS):
        raise RuntimeError('Exact randomized source/replay/document payloads are absent or overlap.')
    files = dict(SOURCE_PAYLOADS); files.update({REPLAY_PATH: raw, DOCUMENT_PATH: DOCUMENT_BYTES})
    if set(files) != set(INITIAL_PATHS): raise RuntimeError('Initial randomized file inventory differs.')
    return files


def owned_worker023(core, repo, output):
    own = repo / '.git/codex-repairs' / WORKFLOW / 'runs'
    if (not output.is_absolute() or '..' in output.parts or output.parent.parent != own
            or not re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', output.parent.name)):
        raise RuntimeError('Randomized worker output is outside its owned run.')
    fd = core.open_dir(output.parent); os.close(fd)


def inputs_worker023(repo, output):
    _, core, _, intake = runtime023(repo); owned_worker023(core, repo, output)
    if output.name not in {'inputs-before.json', 'inputs-after.json'}:
        raise RuntimeError('Randomized input proof filename differs.')
    core.atomic_json(output, intake_binding023(core, intake, repo, full=True))
    print('RANDOMIZED025_SOURCE_INPUTS_REVERIFIED', flush=True)
    return 0


def identity_worker023(repo, spec_path, output):
    finalizer, core, _, _ = runtime023(repo); owned_worker023(core, repo, output)
    if spec_path.parent != output.parent or spec_path.name != 'identity-input.json' or output.name != 'identity.json':
        raise RuntimeError('Configured identity input/output differs from the owned identity plan.')
    spec = json.loads(core.read_file(spec_path, MIB023)[1])
    if not isinstance(spec, dict): raise RuntimeError('Configured identity input is malformed.')
    inv = candidate_original_guard023(core, repo); env = dict(inv.env)
    for key in finalizer.IDENTITY_ENV: env.pop(key, None)
    for key, value in spec.items():
        if key not in finalizer.IDENTITY_ENV or not isinstance(value, str) or '\0' in value:
            raise RuntimeError('Unexpected configured identity environment override.')
        env[key] = value
    result = {}
    for role in ('AUTHOR', 'COMMITTER'):
        command = inv.prefix + ['-c', 'user.useConfigOnly=true', 'var', 'GIT_' + role + '_IDENT']
        p = subprocess.run(command, cwd=repo, env=env, stdin=subprocess.DEVNULL,
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10, check=False)
        if p.returncode or len(p.stdout) > 4096 or len(p.stderr) > 8192:
            raise RuntimeError('Configured Git identity is unavailable; no identity was guessed.')
        result[role.lower()] = finalizer.parse_ident(p.stdout.decode().rstrip('\n'))
    core.atomic_json(output, result); print('RANDOMIZED025_CONFIGURED_IDENTITIES_RECORDED', flush=True)
    return 0


def install_worker023(repo):
    _, core, helper, intake = runtime023(repo); intake_binding023(core, intake, repo, full=True)
    worktree = repo.parent / (repo.name + SUFFIX)
    raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB023); files = files023(raw)
    check_candidate023(core, repo, worktree, files, BASE, allow_base_files=True)
    if not set(ORIGINAL_SOURCE_HASHES).issubset(SOURCE_PAYLOADS):
        raise RuntimeError('Original replacement inputs are outside the exact source payload.')
    for name, expected_sha in dict(ORIGINAL_SOURCE_HASHES, **QUIET_RECOVERY_ORIGINAL_HASHES).items():
        fp, _ = core.read_file(worktree / name, MIB023, content=False)
        if fp['sha256'] != expected_sha:
            raise RuntimeError('Pristine randomized source differs from the captured original: ' + name)
    for name, content in sorted(files.items()):
        destination = worktree / name
        destination.parent.mkdir(parents=True, exist_ok=True); fd = core.open_dir(destination.parent); os.close(fd)
        mode = 0o755 if name == REPLAY_PATH else 0o644
        if name in ORIGINAL_SOURCE_HASHES or name in QUIET_RECOVERY_ORIGINAL_HASHES:
            temporary = destination.with_name(destination.name + '.randomized023-' + uuid.uuid4().hex)
            helper.candidate_exclusive015(core, temporary, content, mode); os.replace(temporary, destination)
        else:
            if os.path.lexists(destination): raise RuntimeError('New candidate path is occupied: ' + name)
            helper.candidate_exclusive015(core, destination, content, mode)
    print('RANDOMIZED025_SOURCE_INSTALLED', flush=True)
    return 0


def seed_worker023(repo, source, target, output):
    _, core, helper, _ = runtime023(repo); owned_worker023(core, repo, output)
    if (source != repo / '.git/codex-repairs/leveldb-randomized-controlled023/cargo-target'
            or target != repo / '.git/codex-repairs' / WORKFLOW / 'cargo-target'
            or output.name != 'cache-copy.json'):
        raise RuntimeError('Cache seed is outside the exact independent-copy plan.')
    return helper.copy_cache_worker_revision002(core, source, target, output)


def artifact_check023(core, artifacts):
    expected = {'bitcoinleveldbt-randomized::' + target for target in RANDOMIZED_TARGETS}
    if not isinstance(artifacts, dict) or set(artifacts) != expected:
        raise RuntimeError('Randomized qualification requires its exact six test executables.')
    paths = set()
    for value in artifacts.values():
        path = Path(value['path'])
        if not path.is_absolute() or '..' in path.parts or str(path) in paths:
            raise RuntimeError('Randomized test executable paths are malformed or aliased.')
        fp, _ = core.read_file(path, 256 * MIB023, content=False)
        if fp != value.get('fingerprint') or not fp.get('mode', 0) & 0o111:
            raise RuntimeError('Tested randomized executable changed or is not executable.')
        paths.add(str(path))


def choose_cases023(all_lists, ignored_lists):
    expected = {'bitcoinleveldbt-randomized::' + target for target in RANDOMIZED_TARGETS}
    if not isinstance(all_lists, dict) or not isinstance(ignored_lists, dict) or set(all_lists) != expected or set(ignored_lists) != expected:
        raise RuntimeError('Randomized compiled test-target inventory differs from the exact six targets.')
    if len(CASES) != 12 or len({item['suffix'] for item in CASES}) != 12:
        raise RuntimeError('Reviewed randomized twelve-case inventory is absent or ambiguous.')
    selected = []
    for key in sorted(expected):
        all_names, ignored_names = all_lists[key], ignored_lists[key]
        if (not isinstance(all_names, list) or not isinstance(ignored_names, list)
                or any(not isinstance(name, str) for name in all_names + ignored_names)
                or len(all_names) != len(set(all_names)) or len(ignored_names) != len(set(ignored_names))):
            raise RuntimeError('Repeated or malformed names in randomized executable listing.')
        items = [item for item in CASES if 'bitcoinleveldbt-randomized::' + item['target'] == key]
        bound = []
        for item in items:
            if type(item.get('ignored')) is not bool: raise RuntimeError('Reviewed ignored-test policy is not boolean.')
            matches = [name for name in all_names if name == item['suffix'] or name.endswith('::' + item['suffix'])]
            if len(matches) != 1: raise RuntimeError('Selected randomized case absent or ambiguous: ' + item['suffix'])
            bound.append(dict(item, name=matches[0]))
        if set(all_names) != {item['name'] for item in bound}:
            raise RuntimeError('Compiled randomized test inventory differs from the reviewed twelve cases.')
        if set(ignored_names) != {item['name'] for item in bound if item['ignored']}:
            raise RuntimeError('Compiled default/ignored randomized execution policy differs.')
        selected.extend(bound)
    by_suffix = {item['suffix']: item for item in selected}
    return [by_suffix[item['suffix']] for item in CASES]


def case_command023(artifact, item):
    command = [str(artifact), '--exact', item['name']]
    if item['ignored']: command.append('--ignored')
    return command + ['--nocapture', '--test-threads=1']


def case_environment023(item, data):
    data = Path(data)
    if not data.is_absolute() or '..' in data.parts: raise RuntimeError('Case data root must be an absolute ordinary plan path.')
    env = {'RUST_LOG': 'off', 'RUST_BACKTRACE': '0', 'TEST_RANDOM_SEED': '301',
           'TMPDIR': str(data), 'TMP': str(data), 'TEMP': str(data), 'TEST_TMPDIR': str(data)}
    if item['profile'] in {'replay', 'bounded'}:
        env['BITCOIN_RS_RANDOMIZED_GUARD'] = 'v1'
    if item['profile'] == 'full_original':
        env.update(BITCOIN_RS_RANDOMIZED_STRESS_GUARD='v1', BITCOIN_RS_RANDOMIZED_CASE_DATA=str(data))
    return env


def case_guard_binding023(core, report, item, row):
    run = Path(report['run_directory']); group = Path(row.get('log', '')).parent
    worktree = Path(report['worktree']); data = run / ('case-data-' + item['suffix'])
    key = 'bitcoinleveldbt-randomized::' + item['target']
    artifact = report['artifacts'][key]['path']; kind = 'stress' if item['profile'] == 'full_original' else 'case'
    command = case_command023(artifact, item); expected_env = case_environment023(item, data)
    if (Path(row.get('log', '')).name != 'output.log' or group.parent != run / 'guard'
            or row.get('cwd') != str(worktree) or row.get('command') != command
            or row.get('kind') != kind or row.get('data_directory') != str(data)
            or row.get('budget') != core.load_guard().BUDGETS[kind]):
        raise RuntimeError('Retained native case command, controls or data path differs.')
    values, fingerprints = {}, {}
    for name in ('plan.json', 'probe.json', 'permit', 'result.json'):
        fp, raw = core.read_file(group / name, MIB023)
        fingerprints[name] = fp; values[name] = raw if name == 'permit' else json.loads(raw)
    plan, probe = values['plan.json'], values['probe.json']
    if (plan.get('kind') != kind or plan.get('command') != command or plan.get('cwd') != str(worktree)
            or not isinstance(plan.get('env'), dict) or any(plan['env'].get(k) != v for k, v in expected_env.items())
            or plan.get('probe') != str(group / 'probe.json') or plan.get('permit') != str(group / 'permit')
            or not isinstance(plan.get('nonce'), str) or not re.fullmatch(r'[0-9a-f]{32}', plan['nonce'])
            or values['permit'] != plan['nonce'].encode() or probe != row.get('probe')
            or probe.get('ok') is not True or values['result.json'] != row):
        raise RuntimeError('Retained case plan, required environment, admission or result contradicts its native record.')
    return {'label': item['suffix'], 'kind': kind, 'required_environment': expected_env,
            'guard_files': {str(group / name): fingerprints[name] for name in fingerprints}}


def receipt023(core, report, files):
    return {'workflow': WORKFLOW, 'base': BASE, 'controller_revision': CONTROLLER_REVISION,
            'diagnostic_change': QUIET_RECOVERY_POLICY,
            'source_plan': {name: core.digest(raw) for name, raw in files.items()},
            'intake_binding': report['intake_before'], 'execution_policy': 'two pure control contracts, four replay contracts, four bounded database cases and two ignored full original stress cases; one exact guarded process per case',
            'original_profile': {'seed': 301, 'option_configurations': 4, 'steps_per_configuration': 10000,
                                 'steps_per_full_case': 40000, 'operation_stream': 'unchanged shared legacy generator',
                                 'comparison_cadence': 'historical step%100 checkpoints; last99 tail mutations not separately compared'},
            'oracle_scope': 'Existing compare_iterators is unchanged and does not check iterator status; full coverage and historical freeze cause are not established.',
            'cases': [{key: value for key, value in case.items() if key != 'log'} for case in report['cases']],
            'case_guards': report['case_guards'], 'artifacts': report['artifacts'],
            'build_command': RANDOMIZED_COMMAND, 'test_count': 12, 'run_directory': report['run_directory'],
            'original_full_cases_completed': True, 'completed_full_cases': ['db_test_randomized', 'db_test_bisect_randomized'],
            'historical_freeze_cause_established': False, 'full_coverage_claimed': False, 'full_randomized_stress': 'passed'}


def reuse023(finalizer, core, helper, repo, state, files):
    helper.check_native_journal015(state)
    if state.get('phase') != 'complete' or not state.get('qualified_report'):
        raise RuntimeError('An incomplete randomized attempt is retained; automatic rerunning is blocked.')
    path = Path(state['qualified_report']); owned_worker023(core, repo, path)
    if path.name != 'RANDOMIZED_REPAIR_RETURN.json': raise RuntimeError('Qualified randomized return layout differs.')
    fp, raw = core.read_file(path, 4 * MIB023)
    if fp != state.get('qualified_report_fingerprint'): raise RuntimeError('Qualified randomized return changed.')
    prior = json.loads(raw); receipt = state['receipt']
    if (prior.get('outcome') != 'qualified_candidate' or prior.get('repair_qualified') is not True
            or prior.get('controller_revision') != CONTROLLER_REVISION or receipt.get('controller_revision') != CONTROLLER_REVISION
            or receipt.get('diagnostic_change') != QUIET_RECOVERY_POLICY
            or prior.get('original_preserved') is not True or prior.get('original_full_cases_completed') is not True
            or prior.get('full_randomized_stress') != 'passed' or prior.get('test_count') != 12
            or prior.get('completed_full_cases') != ['db_test_randomized', 'db_test_bisect_randomized']
            or prior.get('historical_freeze_cause_established') is not False
            or prior.get('full_coverage_claimed') is not False
            or len(prior.get('cases', [])) != 12 or prior.get('receipt') != receipt
            or prior.get('artifacts') != receipt.get('artifacts') or prior.get('case_guards') != receipt.get('case_guards')
            or prior.get('intake_before') != prior.get('intake_after') or prior.get('intake_before') != receipt.get('intake_binding')
            or prior.get('build_command') != RANDOMIZED_COMMAND or receipt.get('build_command') != RANDOMIZED_COMMAND
            or receipt.get('test_count') != 12 or receipt.get('original_full_cases_completed') is not True
            or receipt.get('completed_full_cases') != ['db_test_randomized', 'db_test_bisect_randomized']
            or receipt.get('full_randomized_stress') != 'passed'
            or [{k: v for k, v in c.items() if k != 'log'} for c in prior['cases']] != receipt.get('cases')):
        raise RuntimeError('Retained randomized qualification is incomplete or contradictory.')
    if (receipt.get('source_plan') != {name: core.digest(raw) for name, raw in files.items()}
            or state.get('identity', {}).get('file_hashes') != receipt['source_plan']
            or state.get('identity', {}).get('intake_sha256') != core.digest(core.encoded(prior['intake_before']))):
        raise RuntimeError('Retained randomized source/intake identity differs.')
    complete = dict(files); complete[RECEIPT_PATH] = core.encoded(receipt)
    helper.verify_candidate_commit(finalizer, core, repo, state['head'], state['commit_intent'], complete)
    artifact_check023(core, prior['artifacts'])
    selected = choose_cases023(prior['test_lists']['all'], prior['test_lists']['ignored'])
    if {item['suffix'] for item in selected} != {case['label'] for case in prior['cases']} or set(prior['case_guards']) != {item['suffix'] for item in selected}:
        raise RuntimeError('Retained selected randomized case inventory differs.')
    by_label = {item['suffix']: item for item in selected}
    for case in prior['cases']:
        item = by_label[case['label']]; row = prior['readouts'][case['label']]
        if (case.get('outcome') != 'passed' or case.get('profile') != item['profile'] or case.get('target') != item['target']
                or type(case.get('passed')) is not int or case['passed'] != 1
                or type(case.get('failed')) is not int or case['failed'] != 0
                or type(case.get('ignored')) is not int or case['ignored'] != 0
                or case.get('ignored_by_default') is not item['ignored']
                or case.get('expected_name') != item['name'] or case.get('marker') != item['marker']):
            raise RuntimeError('Retained randomized case classification contradicts its exact selected inventory.')
        guard = case_guard_binding023(core, prior, item, row)
        if guard != prior['case_guards'][case['label']]:
            raise RuntimeError('Retained exact native plan/environment evidence changed.')
        text = core.read_file(Path(row['log']), MIB023)[1].decode(errors='replace')
        if text != case['log'] or core.assess_case(row, text, item['name'], item['marker']).get('outcome') != 'passed':
            raise RuntimeError('Retained exact randomized native case evidence does not verify.')
    return prior


"""Post-exit transport for the controlled randomized repair025.

Only the separately authorized original return-buffer is replaced after the
coordinator has exited and printed/saved return JSON identities agree.
"""
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import stat
import subprocess
import sys
import threading
import uuid


_TRANSPORT_MAX_BYTES = 8 * 1024 * 1024


_TRANSPORT_WORKFLOW = "leveldb-randomized-quiet025"


_TRANSPORT_BEGIN = b"BEGIN_RANDOMIZED_REPAIR_RETURN"


_TRANSPORT_END = b"END_RANDOMIZED_REPAIR_RETURN"


class _TransportReplacementError(RuntimeError):
    buffer_replaced = True


def _transport_open_directory(path):
    path = Path(path)
    if not path.is_absolute() or ".." in path.parts:
        raise RuntimeError("Transport requires an absolute ordinary directory.")
    fd = os.open("/", os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd)
            fd = child
        return fd
    except BaseException:
        os.close(fd)
        raise


def _transport_stamp(info):
    return tuple(getattr(info, key) for key in
                 ("st_dev", "st_ino", "st_mode", "st_uid", "st_size", "st_mtime_ns", "st_ctime_ns"))


def _transport_read_at(directory_fd, name, limit, own=False):
    fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=directory_fd)
    try:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
            raise RuntimeError("Transport input is not a bounded ordinary file: " + name)
        if own and before.st_uid != os.getuid():
            raise RuntimeError("Transport return-buffer is not owned by the current user.")
        pieces, total, digest = [], 0, hashlib.sha256()
        while True:
            chunk = os.read(fd, min(65536, limit + 1 - total))
            if not chunk:
                break
            total += len(chunk)
            if total > limit:
                raise RuntimeError("Transport input grew beyond its byte allowance.")
            pieces.append(chunk)
            digest.update(chunk)
        after = os.fstat(fd)
        current = os.stat(name, dir_fd=directory_fd, follow_symlinks=False)
        if (total != before.st_size or _transport_stamp(before) != _transport_stamp(after)
                or _transport_stamp(before) != _transport_stamp(current)):
            raise RuntimeError("Transport input changed during its ordinary-file read.")
        fp = {"kind": "file", "mode": stat.S_IMODE(before.st_mode), "size": total,
              "sha256": digest.hexdigest()}
        return b"".join(pieces), fp, _transport_stamp(before)
    finally:
        os.close(fd)


def _transport_read_file(path, limit):
    path = Path(path)
    parent = _transport_open_directory(path.parent)
    try:
        return _transport_read_at(parent, path.name, limit)
    finally:
        os.close(parent)


def _transport_emit(raw):
    view = memoryview(raw)
    while view:
        try:
            written = os.write(1, view)
        except InterruptedError:
            continue
        if written <= 0:
            raise RuntimeError("Terminal output did not accept captured bytes.")
        view = view[written:]


def _transport_unique_pairs(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise RuntimeError("Intake transport JSON contains duplicate fields.")
        result[key] = value
    return result


def _transport_report(capture, repo, child_status):
    lines = capture.splitlines()
    begins = [index for index, line in enumerate(lines) if line == _TRANSPORT_BEGIN]
    ends = [index for index, line in enumerate(lines) if line == _TRANSPORT_END]
    if len(begins) != 1 or len(ends) != 1 or begins[0] >= ends[0]:
        raise RuntimeError("Completed output lacks one unambiguous randomized repair return JSON block.")
    report = json.loads(b"\n".join(lines[begins[0] + 1:ends[0]]).decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if (not isinstance(report, dict) or report.get("workflow") != _TRANSPORT_WORKFLOW
            or report.get("repository") != str(repo) or not isinstance(report.get("outcome"), str)):
        raise RuntimeError("Printed randomized repair return does not identify this coordinator/repository.")
    value = report.get("return_file")
    if not isinstance(value, str) or not value or "\x00" in value:
        raise RuntimeError("randomized repair return-file path is absent or malformed.")
    path = Path(value)
    root = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW / "runs"
    if not path.is_absolute() or ".." in path.parts:
        raise RuntimeError("randomized repair return-file path is not ordinary and absolute.")
    try:
        relative = path.relative_to(root)
    except ValueError as error:
        raise RuntimeError("randomized repair return file is outside the owned workflow.") from error
    if len(relative.parts) != 2 or relative.name != "RANDOMIZED_REPAIR_RETURN.json":
        raise RuntimeError("randomized repair return-file layout differs from the owned run layout.")
    if "run_directory" in report and report["run_directory"] != str(path.parent):
        raise RuntimeError("Intake run directory differs from its return file.")
    raw, fp, _ = _transport_read_file(path, _TRANSPORT_MAX_BYTES)
    retained = json.loads(raw.decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if retained != report:
        raise RuntimeError("Printed randomized repair return differs from its retained JSON file.")
    successful = report["outcome"] in {"qualified_candidate", "already_qualified"}
    if successful and report.get("original_preserved") is not True:
        raise RuntimeError("Successful randomized repair return lacks verified original preservation.")
    if (child_status == 0) != successful:
        raise RuntimeError("Coordinator exit status disagrees with its recorded outcome.")
    return report, path, raw, fp


def _transport_existing_state(repo_fd):
    try:
        raw, fp, stamp = _transport_read_at(repo_fd, "return-buffer", _TRANSPORT_MAX_BYTES, own=True)
        return fp, stamp
    except FileNotFoundError:
        return {"kind": "absent"}, None


def _transport_replace_buffer(repo, payload):
    if len(payload) > _TRANSPORT_MAX_BYTES:
        raise RuntimeError("Complete output plus return JSON exceeds the 8 MiB return-buffer allowance.")
    repo_fd = _transport_open_directory(repo)
    temporary = ".return-buffer-transport-" + uuid.uuid4().hex + ".tmp"
    made = False
    replacement_done = False
    try:
        before, stamp = _transport_existing_state(repo_fd)
        mode = before["mode"] if before["kind"] == "file" else 0o600
        fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=repo_fd)
        made = True
        with os.fdopen(fd, "wb") as stream:
            os.fchmod(stream.fileno(), mode)
            stream.write(payload)
            stream.flush()
            os.fsync(stream.fileno())
        try:
            current = os.stat("return-buffer", dir_fd=repo_fd, follow_symlinks=False)
        except FileNotFoundError:
            current = None
        if (stamp is None and current is not None) or (stamp is not None and (current is None or _transport_stamp(current) != stamp)):
            raise RuntimeError("Return-buffer changed before its authorized atomic replacement.")
        # Replacing the directory entry never follows a target symlink and
        # never writes through a source hard link to another pathname.
        os.replace(temporary, "return-buffer", src_dir_fd=repo_fd, dst_dir_fd=repo_fd)
        replacement_done = True
        made = False
        os.fsync(repo_fd)
        written, after, _ = _transport_read_at(repo_fd, "return-buffer", _TRANSPORT_MAX_BYTES, own=True)
        if written != payload or after["mode"] != mode:
            raise RuntimeError("Published return-buffer does not match the complete authorized payload.")
        return before, after
    except BaseException as error:
        if replacement_done:
            raise _TransportReplacementError("Return-buffer was replaced, but final durability/readback verification failed: " + str(error)) from error
        raise
    finally:
        if made:
            try:
                os.unlink(temporary, dir_fd=repo_fd)
            except FileNotFoundError:
                pass
        os.close(repo_fd)


def _transport_write_receipt(directory, value):
    raw = (json.dumps(value, sort_keys=True, indent=2) + "\n").encode()
    if len(raw) > 65536:
        raise RuntimeError("Transport receipt exceeds its finite allowance.")
    parent = _transport_open_directory(directory)
    try:
        fd = os.open("TRANSPORT_RETURN.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=parent)
        with os.fdopen(fd, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        os.fsync(parent)
    finally:
        os.close(parent)


def _transport_lock(repo):
    workflow = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW
    parent = _transport_open_directory(workflow)
    try:
        fd = os.open("run.lock", os.O_RDWR | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=parent)
    finally:
        os.close(parent)
    try:
        info = os.fstat(fd)
        if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid():
            raise RuntimeError("Existing randomized repair transport lock is nonordinary or foreign-owned.")
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return fd
    except BaseException:
        os.close(fd)
        raise


def run_with_transport(script_path, repo, apply):
    """Run one coordinator child, mirror every byte, then replace one buffer."""
    child = None
    child_status = None
    capture = bytearray()
    overflow = False
    mirror_error = None
    report = return_path = return_fp = None
    buffer_written = False
    buffer_replaced = False
    lock_fd = None
    previous_handlers = {}
    total_output = 0
    try:
        if type(apply) is not bool:
            raise RuntimeError("Transport apply/preview selection is malformed.")
        repo = Path(os.path.abspath(Path(repo).expanduser()))
        script_path = Path(os.path.abspath(Path(script_path).expanduser()))
        if any(char in str(repo) for char in ("\x00", "\n", "\r")):
            raise RuntimeError("Transport repository path framing differs.")
        fd = _transport_open_directory(repo)
        os.close(fd)
        _transport_read_file(script_path, 2 * 1024 * 1024)
        command = [sys.executable, "-I", "-S", str(script_path), "--coordinator", str(repo), "apply" if apply else "preview"]
        child = subprocess.Popen(command, cwd=repo, stdin=subprocess.DEVNULL,
                                 stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                 bufsize=0, start_new_session=True)
        if threading.current_thread() is threading.main_thread():
            def forward(signum, frame):
                if child.poll() is None:
                    try:
                        child.send_signal(signum)
                    except ProcessLookupError:
                        pass
            for signum in (signal.SIGINT, signal.SIGTERM):
                previous_handlers[signum] = signal.signal(signum, forward)
        while True:
            try:
                chunk = os.read(child.stdout.fileno(), 65536)
            except InterruptedError:
                continue
            if not chunk:
                break
            total_output += len(chunk)
            if mirror_error is None:
                try:
                    _transport_emit(chunk)
                except OSError as error:
                    mirror_error = str(error)
            room = _TRANSPORT_MAX_BYTES - len(capture)
            if len(chunk) > room:
                overflow = True
            if room > 0:
                capture.extend(chunk[:room])
        child_status = child.wait()
        if overflow:
            raise RuntimeError("Complete coordinator output exceeds the 8 MiB capture allowance; prior return-buffer was retained.")
        report, return_path, return_raw, return_fp = _transport_report(bytes(capture), repo, child_status)
        if mirror_error is not None:
            raise RuntimeError("Terminal mirroring failed: " + mirror_error)
        confirmation = ("RETURN_BUFFER_UPDATED_AFTER_COORDINATOR_EXIT " + str(repo / "return-buffer") + "\n").encode()
        payload = (bytes(capture) + b"\nBEGIN_RANDOMIZED_REPAIR_RETURN_FILE_JSON\n" + return_raw
                   + (b"" if return_raw.endswith(b"\n") else b"\n")
                   + b"END_RANDOMIZED_REPAIR_RETURN_FILE_JSON\n" + confirmation)
        if len(payload) > _TRANSPORT_MAX_BYTES:
            raise RuntimeError("Complete coordinator output plus exact return file exceeds the 8 MiB payload allowance; prior return-buffer was retained.")
        # No lock is held during the coordinator. Reacquire its existing own
        # workflow lock afterward to avoid overlapping a new snapshot/run.
        lock_fd = _transport_lock(repo)
        before, after = _transport_replace_buffer(repo, payload)
        buffer_written = True
        buffer_replaced = True
        transport = {"schema": "bitcoinrs-randomized-quiet025-buffer-transport-v1", "outcome": "written",
                     "coordinator_exit_code": child_status, "coordinator_outcome": report["outcome"],
                     "coordinator_return_file": str(return_path), "coordinator_return_fingerprint": return_fp,
                     "target": str(repo / "return-buffer"), "captured_output_bytes": total_output,
                     "payload_bytes": len(payload), "payload_sha256": hashlib.sha256(payload).hexdigest(),
                     "full_output_captured": True, "authorized_overwrite_after_coordinator": True,
                     "coordinator_preservation_verified": (report.get("original_preserved") is True),
                     "preservation_scope": "Buffer publication occurs after coordinator exit and return-file validation; preservation results belong to the coordinator report.",
                     "buffer_before": before, "buffer_after": after}
        _transport_write_receipt(return_path.parent, transport)
        _transport_emit(confirmation)
        return 0 if child_status == 0 else 1
    except BaseException as error:
        buffer_replaced = buffer_replaced or getattr(error, "buffer_replaced", False)
        if child is not None and child_status is None:
            # Transport failures must not terminate/restart an in-flight
            # repair. Finish draining its bounded coordinator lifecycle.
            try:
                while True:
                    chunk = os.read(child.stdout.fileno(), 65536)
                    if not chunk:
                        break
                    try:
                        _transport_emit(chunk)
                    except OSError:
                        pass
                child_status = child.wait()
            except BaseException as cleanup_error:
                sys.stderr.write("Transport could not finish observing the coordinator: " + str(cleanup_error)[:512] + "\n")
        actual = "coordinator_exit=" + str(child_status)
        if report is not None:
            actual += " coordinator_outcome=" + report["outcome"]
        suffix = ("; return-buffer was already written and verified" if buffer_written
                  else "; return-buffer was already replaced; final verification is incomplete" if buffer_replaced
                  else "; prior return-buffer was retained")
        sys.stderr.write("Return-buffer transport failed: " + type(error).__name__ + ": " + str(error)[:2048] + "; " + actual + suffix + ". No coordinator retry was started.\n")
        sys.stderr.flush()
        return 1
    finally:
        for signum, handler in previous_handlers.items():
            signal.signal(signum, handler)
        if child is not None and child.stdout is not None:
            child.stdout.close()
        if lock_fd is not None:
            os.close(lock_fd)


def run023(repo):
    report = {'workflow': WORKFLOW, 'controller_revision': CONTROLLER_REVISION, 'outcome': 'failed', 'phase': 'preflight', 'repair_qualified': False,
              'base': BASE, 'branch': BRANCH, 'master_updated': False, 'other_refs_updated': False,
              'builds_started': False, 'native_tests_started': False, 'debuggers_started': False,
              'validation_reused': False, 'prior_native_validation_reused': False, 'prior_failure_cleared': False, 'test_count': 0, 'expected_test_count': 12,
              'cases': [], 'case_guards': {}, 'completed_full_cases': [], 'readouts': {}, 'build_command': RANDOMIZED_COMMAND,
              'historical_freeze_cause_established': False, 'original_full_cases_completed': False,
              'full_coverage_claimed': False, 'full_randomized_stress': 'not rerun',
              'other_active_targets': ['bitcoin-leveldb', 'bitcoinleveldb-harness'],
              'other_active_targets_executed': False,
              'operation_generator_sha256': OPERATION_GENERATOR_SHA,
              'oracle_scope': ORACLE_SCOPE,
              'scope': 'Independent quiet randomized repair: four unchanged023 test-control payloads and removal of duplicate direct recovery diagnostics; two pure admission contracts, four retained replay contracts, four bounded cases and both original40000-operation cases all run fresh under unchanged finite controls.',
              'postrun_transport': 'Replace the authorized return-buffer only after coordinator exit.',
              'preservation_scope': 'Original user work, refs, indices, the failed023 journal/worktree/reports/cache/artifacts and read intake remain preserved through coordinator exit; only own025 candidate branch/worktree/journal/private cache/reports may change. Subsequent authorized buffer replacement is separate.'}
    held, core, initial, worktree, directory, state, state_path, lineage = [], None, None, None, None, None, None, None
    success = False
    try:
        repo = Path(os.path.abspath(Path(repo).expanduser())); report['repository'] = str(repo)
        finalizer, core, helper, intake = runtime023(repo)
        checked_environment023(core)
        common = core.repository(core.Inventory(repo)); held = core.acquire_locks(common)
        own = common / 'codex-repairs' / WORKFLOW; own.mkdir(mode=0o700, exist_ok=True)
        fd = core.open_dir(own); os.close(fd)
        lock = os.open(own / 'run.lock', os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600)
        held.append((lock, own / 'run.lock'))
        if not stat.S_ISREG(os.fstat(lock).st_mode): raise RuntimeError('Randomized workflow lock is not ordinary.')
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        candidate_original_guard023(core, repo)
        worktree = repo.parent / (repo.name + SUFFIX)
        initial = core.normalized_original(core.protected_snapshot(core, repo), worktree)
        lineage = intake_binding023(core, intake, repo)
        report.update(preservation_before_sha256=core.digest(core.encoded(initial)), intake_before=lineage,
                      locks_held=[str(path) for _, path in held], worktree=str(worktree))
        runs = own / 'runs'; runs.mkdir(mode=0o700, exist_ok=True)
        fd = core.open_dir(runs); os.close(fd)
        directory = runs / (dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ') + '-' + uuid.uuid4().hex[:8])
        directory.mkdir(mode=0o700)
        script = directory / 'randomized-controller025.py'; raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB023)
        core.exclusive(script, raw, 0o700); files = files023(raw)
        report.update(run_directory=str(directory), controller_sha256=core.digest(raw), guard_sha256=core.GUARD_SHA,
                      source_plan={name: core.digest(value) for name, value in files.items()})
        state_path = own / 'state.json'
        identity = {'workflow': WORKFLOW, 'base': BASE, 'branch': BRANCH, 'worktree': str(worktree),
                    'file_hashes': report['source_plan'], 'intake_sha256': core.digest(core.encoded(lineage)),
                    'controller_revision': CONTROLLER_REVISION}
        guard_module = core.load_guard()
        guard = guard_module.Guard(directory / 'guard', script,
                                   dict(core.Inventory(repo).env, BITCOIN_RS_RANDOMIZED025_REPO=str(repo)))
        def preserve():
            candidate_original_guard023(core, repo)
            if core.normalized_original(core.protected_snapshot(core, repo), worktree) != initial:
                raise RuntimeError('Protected original or predecessor state changed.')
            if intake_binding023(core, intake, repo) != lineage:
                raise RuntimeError('Read source-intake evidence changed.')
        def phase(label, command, kind='source', cwd=None, env=None):
            preserve(); report['phase'] = label
            print('Protected Randomized repair phase: ' + label, flush=True)
            row = guard.run(label, command, cwd or repo, env or {}, kind=kind)
            report['readouts'][label] = row; core.require_guard(row)
            return row
        def git(label, args, cwd, extra=None, tag=None):
            tag = tag or label
            spec, output = directory / (tag + '.git.spec.json'), directory / (tag + '.git.json')
            value = {'label': label, 'args': args, 'cwd': str(cwd), 'output': str(output)}
            value.update(extra or {}); core.exclusive(spec, core.encoded(value))
            phase('git-' + tag, [sys.executable, '-I', '-S', str(script), '--git', str(repo), str(spec)], 'build', cwd)
            evidence = json.loads(core.read_file(output, 4 * MIB023)[1]); report.setdefault('git_evidence', {})[tag] = evidence
            if evidence.get('exit_code') != 0 or evidence.get('args') != args:
                raise RuntimeError('Exact isolated Git operation evidence differs.')
            return evidence['stdout'].strip()
        phase('preflight', [sys.executable, '-I', '-S', '-c', "print('GUARD_PREFLIGHT_OK')"], 'probe')
        before_inputs = directory / 'inputs-before.json'
        phase('inputs-before', [sys.executable, '-I', '-S', str(script), '--inputs', str(repo), str(before_inputs)])
        if json.loads(core.read_file(before_inputs, MIB023)[1]) != lineage:
            raise RuntimeError('Initial current source proof differs from retained intake binding.')
        if os.path.lexists(state_path):
            state = json.loads(core.read_file(state_path, 4 * MIB023)[1])
            if state.get('identity') != identity:
                raise RuntimeError('Existing Randomized journal belongs to a different exact plan; retained.')
            prior = reuse023(finalizer, core, helper, repo, state, files)
            report.update(outcome='already_qualified', phase='complete', repair_qualified=True, validation_reused=True,
                          test_count=12, cases=prior['cases'], artifacts=prior['artifacts'],
                          candidate_head=state['head'], candidate_tree=state['commit_intent']['tree'],
                          qualified_report=state['qualified_report'], original_full_cases_completed=True,
                          test_lists=prior['test_lists'], receipt=prior['receipt'],
                          target_directory=prior['target_directory'], case_guards=prior['case_guards'],
                          completed_full_cases=prior['completed_full_cases'], full_randomized_stress='passed')
            report['qualified_readouts'] = prior['readouts']
            success = True
        else:
            inv = candidate_original_guard023(core, repo)
            _, existing = inv.git('for-each-ref', '--format=%(refname)', 'refs/heads/' + BRANCH)
            if existing.strip() or os.path.lexists(worktree):
                raise RuntimeError('Unowned candidate branch or checkout already exists; retained.')
            state = {'identity': identity, 'phase': 'reserved'}; core.atomic_json(state_path, state)
            identity_spec, identity_out = directory / 'identity-input.json', directory / 'identity.json'
            core.exclusive(identity_spec, core.encoded({key: os.environ[key] for key in finalizer.IDENTITY_ENV if key in os.environ}))
            phase('read-configured-identity', [sys.executable, '-I', '-S', str(script), '--identity', str(repo), str(identity_spec), str(identity_out)])
            ident = json.loads(core.read_file(identity_out, MIB023)[1]); report['configured_identity'] = ident
            identity_env = {'GIT_' + role.upper() + '_' + field.upper(): ident[role][field.lower()]
                            for role in ('author', 'committer') for field in ('NAME', 'EMAIL')}
            git('worktree', ['worktree', 'add', '-b', BRANCH, str(worktree), BASE], repo)
            phase('install-source', [sys.executable, '-I', '-S', str(script), '--install', str(repo)], 'source', worktree)
            git('add', ['add', '--', *sorted(files)], worktree)
            check_candidate023(core, repo, worktree, files, BASE)
            state['phase'] = 'installed'; core.atomic_json(state_path, state)
            source_cache = common / 'codex-repairs/leveldb-randomized-controlled023/cargo-target'
            target = own / 'cargo-target'; report['target_directory'] = str(target)
            cache_locks = []
            try:
                for name in ('.cargo-build-lock', '.cargo-lock'):
                    fd = os.open(source_cache / 'debug' / name, os.O_RDWR | os.O_NOFOLLOW)
                    cache_locks.append(fd)
                    if not stat.S_ISREG(os.fstat(fd).st_mode): raise RuntimeError('Retained Cargo cache lock is not ordinary.')
                    fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                phase('seed-private-cache', [sys.executable, '-I', '-S', str(script), '--copy-cache', str(repo),
                                             str(source_cache), str(target), str(directory / 'cache-copy.json')], 'build')
            finally:
                for fd in cache_locks: os.close(fd)
            report['cache_copy'] = json.loads(core.read_file(directory / 'cache-copy.json', MIB023)[1])
            if report['cache_copy'].get('outcome') != 'copied': raise RuntimeError('Independent private cache seed did not complete.')
            spec, evidence = directory / 'cargo.spec.json', directory / 'cargo-artifacts.json'
            core.exclusive(spec, core.encoded({'command': RANDOMIZED_COMMAND, 'cwd': str(worktree),
                                              'target_directory': str(target), 'evidence_path': str(evidence)}))
            nix = ['nix-shell', '--option', 'max-jobs', '0', '--option', 'builders', '', '--option', 'substitute', 'false',
                   str(worktree / 'shell.nix'), '--run', shlex.join([sys.executable, '-I', '-S', str(script), '--cargo-observer', str(spec)])]
            report['builds_started'] = True
            try:
                phase('compile-randomized-tests', nix, 'build', worktree,
                      {'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '1', 'CARGO_INCREMENTAL': '0'})
            finally:
                if evidence.exists(): report['cargo_evidence'] = json.loads(core.read_file(evidence, MIB023)[1])
            check_candidate023(core, repo, worktree, files, BASE)
            compiled = report['cargo_evidence']['artifacts']
            expected_artifacts = {'bitcoinleveldbt-randomized::' + target for target in RANDOMIZED_TARGETS}
            if set(compiled) != expected_artifacts:
                raise RuntimeError('Exact six selected test executables were not compiled.')
            artifacts = {key: {'path': value['executable'],
                               'fingerprint': core.read_file(Path(value['executable']), 256 * MIB023, content=False)[0]}
                         for key, value in compiled.items()}
            report['artifacts'] = artifacts
            lists = {'all': {}, 'ignored': {}}
            for key in sorted(artifacts):
                target_name = key.split('::', 1)[1]
                executable = artifacts[key]['path']
                for label, arguments in (('all', ['--list']), ('ignored', ['--list', '--ignored'])):
                    row = phase('list-' + target_name + '-' + label, [executable, *arguments], 'case', worktree, {'RUST_LOG': 'off'})
                    text = core.read_file(Path(row['log']), MIB023)[1].decode(errors='replace')
                    lists[label][key] = [line[:-6] for line in text.splitlines() if line.endswith(': test')]
            report['test_lists'] = lists
            selected = choose_cases023(lists['all'], lists['ignored'])
            for item in selected:
                preserve(); check_candidate023(core, repo, worktree, files, BASE); artifact_check023(core, artifacts)
                label = item['suffix']; data = directory / ('case-data-' + label)
                data.mkdir(mode=0o700)
                state['native_pending'] = {'run': str(directory), 'case': item['name'], 'profile': item['profile']}
                core.atomic_json(state_path, state)
                report.update(phase=label, native_tests_started=True)
                print('Protected Randomized repair phase: ' + label, flush=True)
                executable = artifacts['bitcoinleveldbt-randomized::' + item['target']]['path']
                command = case_command023(executable, item)
                env = case_environment023(item, data)
                kind = 'stress' if item['profile'] == 'full_original' else 'case'
                row = guard.run(label, command, worktree, env, kind=kind, tmpdir=data)
                report['readouts'][label] = row
                text = core.read_file(Path(row['log']), MIB023)[1].decode(errors='replace') if row.get('log') else ''
                assessed = core.assess_case(row, text, item['name'], item['marker'])
                report['cases'].append(dict(assessed, label=label, target=item['target'], profile=item['profile'],
                                            ignored_by_default=item['ignored'], log=text))
                report['test_count'] = len(report['cases'])
                if assessed.get('outcome') != 'passed':
                    state['native_failure'] = dict(state['native_pending']); core.atomic_json(state_path, state)
                    raise RuntimeError('Selected Randomized case failed; exact evidence is retained and automatic qualification is blocked.')
                report['case_guards'][label] = case_guard_binding023(core, report, item, row)
                if item['profile'] == 'full_original':
                    report['completed_full_cases'].append(label)
                    expected_full = [case['suffix'] for case in CASES if case['profile'] == 'full_original']
                    report['original_full_cases_completed'] = report['completed_full_cases'] == expected_full
                    if report['original_full_cases_completed']: report['full_randomized_stress'] = 'passed'
                state.pop('native_pending'); core.atomic_json(state_path, state)
            helper.check_native_journal015(state)
            if report['test_count'] != 12 or report['original_full_cases_completed'] is not True:
                raise RuntimeError('Selected Randomized qualification is incomplete.')
            after_inputs = directory / 'inputs-after.json'
            phase('inputs-after', [sys.executable, '-I', '-S', str(script), '--inputs', str(repo), str(after_inputs)])
            if json.loads(core.read_file(after_inputs, MIB023)[1]) != lineage:
                raise RuntimeError('Original selected source inputs changed during candidate testing.')
            preserve(); check_candidate023(core, repo, worktree, files, BASE); artifact_check023(core, artifacts)
            receipt = receipt023(core, report, files); receipt_path = worktree / RECEIPT_PATH
            receipt_path.parent.mkdir(parents=True, exist_ok=True); fd = core.open_dir(receipt_path.parent); os.close(fd)
            helper.candidate_exclusive015(core, receipt_path, core.encoded(receipt), 0o644); files[RECEIPT_PATH] = core.encoded(receipt)
            git('add', ['add', '--', *sorted(files)], worktree, tag='add-final')
            check_candidate023(core, repo, worktree, files, BASE)
            tree = git('tree', ['write-tree'], worktree)
            if not _candidate_oid023(tree): raise RuntimeError('Candidate tree identity is absent.')
            message_bytes = COMMIT_MESSAGE
            message = directory / 'commit-message.txt'; core.exclusive(message, message_bytes)
            intent = {'tree': tree, 'parents': [BASE], 'identity': ident, 'message_sha256': core.digest(message_bytes)}
            state.update(phase='commit-intent', receipt=receipt, commit_intent=intent); core.atomic_json(state_path, state)
            head = git('commit-tree', ['commit-tree', tree, '-p', BASE, '-F', str(message)], worktree,
                       {'tree': tree, 'identity_env': identity_env})
            if not _candidate_oid023(head): raise RuntimeError('Candidate commit identity is absent.')
            state['created_commit'] = head; core.atomic_json(state_path, state)
            git('head', ['update-ref', 'refs/heads/' + BRANCH, head, BASE], worktree, {'head': head})
            helper.verify_candidate_commit(finalizer, core, repo, head, intent, files)
            artifact_check023(core, artifacts); preserve()
            state['head'] = head
            report.update(outcome='qualified_candidate', phase='complete', repair_qualified=True,
                          candidate_head=head, candidate_tree=tree, receipt=receipt, changed_paths=sorted(files))
            success = True
    except BaseException as error:
        report.update(outcome='failed', repair_qualified=False, error=type(error).__name__ + ': ' + str(error))
    finally:
        if initial is not None:
            try:
                after = core.normalized_original(core.protected_snapshot(core, repo), worktree)
                report['preservation_after_sha256'] = core.digest(core.encoded(after))
                report['intake_after'] = intake_binding023(core, intake, repo)
                report['original_preserved'] = after == initial and report['intake_after'] == lineage
                if report['original_preserved'] is not True:
                    raise RuntimeError('Protected original or read source-intake state changed.')
            except BaseException as error:
                success = False; report.update(outcome='failed', repair_qualified=False, original_preserved=False, preservation_error=str(error))
        if directory is not None:
            report['return_file'] = str(directory / 'RANDOMIZED_REPAIR_RETURN.json')
            try:
                core.atomic_json(Path(report['return_file']), report)
                if success and report['outcome'] == 'qualified_candidate':
                    state.update(phase='complete', qualified_report=report['return_file'],
                                 qualified_report_fingerprint=core.read_file(Path(report['return_file']), 4 * MIB023)[0])
                    core.atomic_json(state_path, state)
            except BaseException as error:
                report.update(outcome='failed', repair_qualified=False, publication_error=str(error))
                core.atomic_json(Path(report['return_file']), report)
        for fd, _ in held: os.close(fd)
    return report


def git_worker023(repo, spec):
    _, core, helper, _ = runtime023(repo)
    previous_umask = os.umask(0o022)
    try:
        return helper.run_git_operation(core, repo, spec)
    finally:
        os.umask(previous_umask)


def main023():
    sys.dont_write_bytecode = True; resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    if len(sys.argv) == 3 and sys.argv[1] == '--guard-child':
        plan = json.loads(bootstrap_read(Path(sys.argv[2]), MIB023))
        _, core, _, _ = runtime023(Path(plan['env']['BITCOIN_RS_RANDOMIZED025_REPO']))
        return core.load_guard().guard_child(Path(sys.argv[2]))
    if len(sys.argv) == 4 and sys.argv[1] == '--inputs':
        return inputs_worker023(Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 5 and sys.argv[1] == '--identity':
        return identity_worker023(Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]))
    if len(sys.argv) == 4 and sys.argv[1] == '--git':
        return git_worker023(Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 3 and sys.argv[1] == '--install':
        return install_worker023(Path(sys.argv[2]))
    if len(sys.argv) == 6 and sys.argv[1] == '--copy-cache':
        return seed_worker023(Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]), Path(sys.argv[5]))
    if len(sys.argv) == 3 and sys.argv[1] == '--cargo-observer':
        return cargo_observer(json.loads(bootstrap_read(Path(sys.argv[2]), MIB023)))
    if len(sys.argv) == 4 and sys.argv[1] == '--coordinator' and sys.argv[3] == 'apply':
        resource.setrlimit(resource.RLIMIT_AS, (768 * MIB023,) * 2)
        signal.signal(signal.SIGALRM, lambda *_: (_ for _ in ()).throw(RuntimeError('Randomized coordinator deadline reached.')))
        signal.alarm(7200)
        report = run023(Path(sys.argv[2]))
        print('BEGIN_RANDOMIZED_REPAIR_RETURN', flush=True); print(json.dumps(report, indent=2), flush=True)
        print('END_RANDOMIZED_REPAIR_RETURN', flush=True)
        return 0 if report['outcome'] in {'qualified_candidate', 'already_qualified'} else 1
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=Path.home() / 'bitcoin-rs')
    args = parser.parse_args()
    return run_with_transport(Path(__file__).absolute(), Path(os.path.abspath(args.repo.expanduser())), True)


if __name__ == '__main__': raise SystemExit(main023())
