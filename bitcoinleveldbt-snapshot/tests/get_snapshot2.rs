// ---------------- [ File: bitcoinleveldbt-snapshot/tests/get_snapshot2.rs ]
use bitcoinleveldbt_snapshot::*;
use bitcoinleveldbt_dbtest::*;
use bitcoinleveldbt_util::*;
use bitcoinleveldb_dbinterface::*;
use bitcoinleveldb_options::*;
use bitcoinleveldb_snapshot::*;
use bitcoinleveldb_key::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_dbimpl::*;
use traced_test::*;
use tracing::*;
use tracing_setup::*;

#[cfg(test)]
mod get_snapshot2_tests {
    use super::*;

    #[traced_test]
    fn db_test_get_snapshot2() {
        let mut body = |dbtest: &mut DBTest| {
            trace!(
                target: BITCOINLEVELDBT_SNAPSHOT_GET_SNAPSHOT_TRACE_TARGET,
                label = "db_test_get_snapshot.body.entry",
                phase = "enter"
            );

            // Try with both a short key and a long key
            let mut i: i32 = 0;

            while i < 2 {
                let key_owned = match i == 0 {
                    true => dbtest_fixture_owned_string("foo"),
                    false => "x".repeat(200),
                };

                let v1 = dbtest_fixture_owned_string("v1");
                let v2 = dbtest_fixture_owned_string("v2");

                assert!(dbtest_fixture_put_owned_string_pair(dbtest, &key_owned, &v1).is_ok());
                let s1 = unsafe { (*dbtest.dbfull()).get_snapshot() };

                assert!(dbtest_fixture_put_owned_string_pair(dbtest, &key_owned, &v2).is_ok());

                assert_eq!("v2", dbtest.get(&key_owned, None));
                assert_eq!("v1", dbtest.get(&key_owned, Some(&*s1)));

                assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());

                assert_eq!("v2", dbtest.get(&key_owned, None));
                assert_eq!("v1", dbtest.get(&key_owned, Some(&*s1)));

                unsafe {
                    (*dbtest.dbfull()).release_snapshot(s1);
                }

                i += 1;
            }

            trace!(
                target: BITCOINLEVELDBT_SNAPSHOT_GET_SNAPSHOT_TRACE_TARGET,
                label = "db_test_get_snapshot.body.exit",
                phase = "exit"
            );
        };

        dbtest_fixture_run_across_option_configurations(&mut body);
    }

    #[traced_test]
    fn db_test_get_identical_snapshots2() {
        let mut body = |dbtest: &mut DBTest| {
            trace!(
                target: BITCOINLEVELDBT_SNAPSHOT_GET_SNAPSHOT_TRACE_TARGET,
                label = "db_test_get_identical_snapshots.body.entry",
                phase = "enter"
            );

            // Try with both a short key and a long key
            let mut i: i32 = 0;

            while i < 2 {
                let key_owned = match i == 0 {
                    true => dbtest_fixture_owned_string("foo"),
                    false => "x".repeat(200),
                };

                let v1 = dbtest_fixture_owned_string("v1");
                let v2 = dbtest_fixture_owned_string("v2");

                let observation =
                    IdenticalSnapshotsCompactionDiagnosticObservation::new(
                        dbtest,
                        &key_owned,
                        &v1,
                        &v2,
                    );

                match observation {
                    IdenticalSnapshotsCompactionDiagnosticObservation::CompactionStatusNotOk {
                        status,
                        internal_entries_before_compaction,
                    } => {
                        assert!(
                            false,
                            "memtable compaction status must be OK during identical-snapshot observation: status={} entries_before_compaction={}",
                            status.to_string(),
                            internal_entries_before_compaction
                        );
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
                    } => {
                        assert_eq!(
                            Some(1),
                            snapshot_one_sequence_number_hint,
                            "released snapshot must retain the original sequence before compaction diagnostics; entries_before_compaction={}",
                            internal_entries_before_compaction
                        );
                        assert_eq!(
                            Some(1),
                            snapshot_two_sequence_number_hint,
                            "second snapshot must retain the original sequence through compaction diagnostics; entries_before_compaction={}",
                            internal_entries_before_compaction
                        );
                        assert_eq!(
                            Some(1),
                            snapshot_three_sequence_number_hint,
                            "third snapshot must retain the original sequence through compaction diagnostics; entries_before_compaction={}",
                            internal_entries_before_compaction
                        );

                        assert_eq!("v2", current_before_compaction_value);
                        assert_eq!("v1", snapshot_one_before_compaction_value);
                        assert_eq!("v1", snapshot_two_before_compaction_value);
                        assert_eq!("v1", snapshot_three_before_compaction_value);

                        assert_eq!(
                            "[ v2, v1 ]",
                            internal_entries_before_compaction,
                            "internal iterator must expose both versions before compaction"
                        );

                        assert_eq!("v2", current_after_compaction_value);

                        assert_eq!(
                            "[ v2, v1 ]",
                            internal_entries_after_compaction,
                            "internal iterator must preserve both versions across compaction when identical snapshots remain live; snapshot_two_after_compaction_value={} snapshot_three_after_compaction_value={}",
                            snapshot_two_after_compaction_value,
                            snapshot_three_after_compaction_value
                        );

                        assert_eq!(
                            "v1",
                            snapshot_two_after_compaction_value,
                            "second identical snapshot must continue to observe the original value after compaction; entries_after_compaction={}",
                            internal_entries_after_compaction
                        );

                        assert_eq!(
                            "v1",
                            snapshot_three_after_compaction_value,
                            "third identical snapshot must continue to observe the original value after compaction; entries_after_compaction={}",
                            internal_entries_after_compaction
                        );
                    }
                }

                i += 1;
            }

            trace!(
                target: BITCOINLEVELDBT_SNAPSHOT_GET_SNAPSHOT_TRACE_TARGET,
                label = "db_test_get_identical_snapshots.body.exit",
                phase = "exit"
            );
        };

        dbtest_fixture_run_across_option_configurations(&mut body);
    }
}
