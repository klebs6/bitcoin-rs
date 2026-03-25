// ---------------- [ File: bitcoinleveldbt-snapshot/tests/clue_flush.rs ]
use bitcoinleveldbt_snapshot::*;
use bitcoinleveldbt_dbtest::*;
use bitcoinleveldbt_util::*;
use bitcoinleveldb_dbinterface::*;
use bitcoinleveldb_options::*;
use bitcoinleveldb_modeldb::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_dbimpl::*;
use bitcoinleveldb_iterator::*;
use bitcoinleveldb_iteratorinner::*;
use bitcoinleveldb_snapshot::*;
use traced_test::*;
use tracing_setup::*;

#[cfg(test)]
mod clue_flush_tests {
    use super::*;

    #[traced_test]
    fn db_test_snapshot_clue_internal_history_dump_retains_both_versions_after_single_key_overwrite_and_forced_memtable_compaction() {
        let mut body =
            |dbtest: &mut DBTest, key_owned: &String, _snapshot: &dyn Snapshot| {
                let key_slice = Slice::from(key_owned);

                // If this fails, stop chasing snapshot-dispatch or comparator clues:
                // the older version did not survive into the flushed history surface at all.
                assert_eq!(
                    dbtest.all_entries_for(&key_slice),
                    "[ v2, v1 ]",
                    "an outstanding snapshot must keep both overwrite versions visible in the internal history surface after a forced memtable compaction",
                );
            };

        bitcoinleveldbt_snapshot_clue_run_single_key_overwrite_snapshot_then_flushed_memtable_case(
            &mut body,
        );
    }

    #[traced_test]
    fn db_test_snapshot_clue_iterator_surface_preserves_pre_flush_value_after_single_key_overwrite_and_forced_memtable_compaction() {
        let mut body =
            |dbtest: &mut DBTest, key_owned: &String, snapshot: &dyn Snapshot| {
                let live_rows =
                    bitcoinleveldbt_snapshot_clue_collect_db_rows_from_optional_snapshot_ref(
                        dbtest,
                        None,
                    );

                let snapshot_rows =
                    bitcoinleveldbt_snapshot_clue_collect_db_rows_from_optional_snapshot_ref(
                        dbtest,
                        Some(snapshot),
                    );

                // If the live iterator view is wrong, the failure is broader than snapshots.
                assert_eq!(
                    live_rows,
                    vec![(key_owned.clone(), String::from("v2"))],
                    "the live iterator surface must observe the post-overwrite value after the memtable has been flushed",
                );

                // If this fails while the history dump passes, the iterator/table read surface
                // is selecting the wrong version from flushed state.
                assert_eq!(
                    snapshot_rows,
                    vec![(key_owned.clone(), String::from("v1"))],
                    "the snapshot iterator surface must preserve the pre-overwrite value after the memtable has been flushed",
                );
            };

        bitcoinleveldbt_snapshot_clue_run_single_key_overwrite_snapshot_then_flushed_memtable_case(
            &mut body,
        );
    }

    #[traced_test]
    fn db_test_snapshot_clue_point_lookup_preserves_pre_flush_value_after_single_key_overwrite_and_forced_memtable_compaction() {
        let mut body =
            |dbtest: &mut DBTest, key_owned: &String, snapshot: &dyn Snapshot| {
                // If live point lookup is wrong, the failure is not snapshot-specific.
                assert_eq!(
                    "v2",
                    dbtest.get(key_owned, None),
                    "the live point-lookup surface must observe the post-overwrite value after the memtable has been flushed",
                );

                // If the history dump and iterator surface both pass while this fails,
                // the bug is localized to point lookup over flushed table state.
                assert_eq!(
                    "v1",
                    dbtest.get(key_owned, Some(snapshot)),
                    "the snapshot point-lookup surface must preserve the pre-overwrite value after the memtable has been flushed",
                );
            };

        bitcoinleveldbt_snapshot_clue_run_single_key_overwrite_snapshot_then_flushed_memtable_case(
            &mut body,
        );
    }
}
