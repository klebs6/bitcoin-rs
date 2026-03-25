// ---------------- [ File: bitcoinleveldbt-snapshot/src/identical_snapshots_compaction_diagnostic_observation.rs ]
crate::ix!();

#[derive(Clone, Debug)]
pub enum IdenticalSnapshotsCompactionDiagnosticObservation {
    CompactionStatusNotOk {
        status:                           Status,
        internal_entries_before_compaction: String,
    },
    Observed {
        snapshot_one_sequence_number_hint:   Option<SequenceNumber>,
        snapshot_two_sequence_number_hint:   Option<SequenceNumber>,
        snapshot_three_sequence_number_hint: Option<SequenceNumber>,
        current_before_compaction_value:     String,
        snapshot_one_before_compaction_value: String,
        snapshot_two_before_compaction_value: String,
        snapshot_three_before_compaction_value: String,
        internal_entries_before_compaction:   String,
        current_after_compaction_value:       String,
        snapshot_two_after_compaction_value:  String,
        snapshot_three_after_compaction_value: String,
        internal_entries_after_compaction:     String,
    },
}

impl IdenticalSnapshotsCompactionDiagnosticObservation {

    pub fn new(
        dbtest:      &mut DBTest,
        key_owned:   &String,
        first_value: &String,
        second_value:&String,
    ) -> IdenticalSnapshotsCompactionDiagnosticObservation {
        trace!(
            target: BITCOINLEVELDBT_SNAPSHOT_GET_SNAPSHOT_TRACE_TARGET,
            label = "bitcoinleveldbt_snapshot_capture_identical_snapshots_compaction_diagnostic_observation_for_integration_test.entry",
            phase = "enter",
            key_len = key_owned.len(),
            first_value_len = first_value.len(),
            second_value_len = second_value.len()
        );

        let initial_put_status =
            dbtest_fixture_put_owned_string_pair(dbtest, key_owned, first_value);

        assert!(
            initial_put_status.is_ok(),
            "initial put must succeed before identical-snapshot observation: {}",
            initial_put_status.to_string()
        );

        let snapshot_one = unsafe { (*dbtest.dbfull()).get_snapshot() };
        let snapshot_two = unsafe { (*dbtest.dbfull()).get_snapshot() };
        let snapshot_three = unsafe { (*dbtest.dbfull()).get_snapshot() };

        let snapshot_one_dispatch_observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                &*snapshot_one,
            );
        let snapshot_two_dispatch_observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                &*snapshot_two,
            );
        let snapshot_three_dispatch_observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                &*snapshot_three,
            );

        let snapshot_one_sequence_number_hint =
            *snapshot_one_dispatch_observation.snapshot_sequence_number_hint();
        let snapshot_two_sequence_number_hint =
            *snapshot_two_dispatch_observation.snapshot_sequence_number_hint();
        let snapshot_three_sequence_number_hint =
            *snapshot_three_dispatch_observation.snapshot_sequence_number_hint();

        debug!(
            target: BITCOINLEVELDBT_SNAPSHOT_GET_SNAPSHOT_TRACE_TARGET,
            label = "bitcoinleveldbt_snapshot_capture_identical_snapshots_compaction_diagnostic_observation_for_integration_test.snapshot_sequence_hints",
            phase = "state_transition",
            snapshot_one_sequence_number_hint = ?snapshot_one_sequence_number_hint,
            snapshot_two_sequence_number_hint = ?snapshot_two_sequence_number_hint,
            snapshot_three_sequence_number_hint = ?snapshot_three_sequence_number_hint
        );

        let updated_put_status =
            dbtest_fixture_put_owned_string_pair(dbtest, key_owned, second_value);

        assert!(
            updated_put_status.is_ok(),
            "updated put must succeed before compaction observation: {}",
            updated_put_status.to_string()
        );

        let current_before_compaction_value = dbtest.get(key_owned, None);
        let snapshot_one_before_compaction_value =
            dbtest.get(key_owned, Some(&*snapshot_one));
        let snapshot_two_before_compaction_value =
            dbtest.get(key_owned, Some(&*snapshot_two));
        let snapshot_three_before_compaction_value =
            dbtest.get(key_owned, Some(&*snapshot_three));

        let user_key_slice = Slice::from(key_owned);
        let internal_entries_before_compaction = dbtest.all_entries_for(&user_key_slice);

        unsafe {
            (*dbtest.dbfull()).release_snapshot(snapshot_one);
        }

        let compaction_status = dbtest_fixture_test_compact_memtable_status(dbtest);

        let observation = match compaction_status.is_ok() {
            true => {
                let current_after_compaction_value = dbtest.get(key_owned, None);
                let snapshot_two_after_compaction_value =
                    dbtest.get(key_owned, Some(&*snapshot_two));
                let snapshot_three_after_compaction_value =
                    dbtest.get(key_owned, Some(&*snapshot_three));
                let internal_entries_after_compaction =
                    dbtest.all_entries_for(&user_key_slice);

                unsafe {
                    (*dbtest.dbfull()).release_snapshot(snapshot_two);
                    (*dbtest.dbfull()).release_snapshot(snapshot_three);
                }

                IdenticalSnapshotsCompactionDiagnosticObservation::Observed {
                    snapshot_one_sequence_number_hint,
                    snapshot_two_sequence_number_hint,
                    snapshot_three_sequence_number_hint,
                    current_before_compaction_value,
                    snapshot_one_before_compaction_value,
                    snapshot_two_before_compaction_value,
                    snapshot_three_before_compaction_value,
                    internal_entries_before_compaction,
                    current_after_compaction_value,
                    snapshot_two_after_compaction_value,
                    snapshot_three_after_compaction_value,
                    internal_entries_after_compaction,
                }
            }
            false => {
                unsafe {
                    (*dbtest.dbfull()).release_snapshot(snapshot_two);
                    (*dbtest.dbfull()).release_snapshot(snapshot_three);
                }

                IdenticalSnapshotsCompactionDiagnosticObservation::CompactionStatusNotOk {
                    status: compaction_status,
                    internal_entries_before_compaction,
                }
            }
        };

        trace!(
            target: BITCOINLEVELDBT_SNAPSHOT_GET_SNAPSHOT_TRACE_TARGET,
            label = "bitcoinleveldbt_snapshot_capture_identical_snapshots_compaction_diagnostic_observation_for_integration_test.exit",
            phase = "return"
        );

        observation
    }

}

pub const BITCOINLEVELDBT_SNAPSHOT_GET_SNAPSHOT_TRACE_TARGET: &str =
    "bitcoinleveldbt_snapshot::get_snapshot_integration";
