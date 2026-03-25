// ---------------- [ File: bitcoinleveldbt-snapshot/src/run_single_key_overwrite.rs ]
crate::ix!();

pub fn bitcoinleveldbt_snapshot_clue_run_single_key_overwrite_snapshot_then_flushed_memtable_case<Body>(
    body: &mut Body,
)
where
    Body: FnMut(&mut DBTest, &String, &dyn Snapshot),
{
    let mut run_case = |dbtest: &mut DBTest| {
        let key_owned = String::from("snapshot-clue-key");
        let first_value_owned = String::from("v1");
        let second_value_owned = String::from("v2");

        assert!(dbtest.put(&key_owned, &first_value_owned).is_ok());

        let snapshot = unsafe { (*dbtest.dbfull()).get_snapshot() };

        assert!(dbtest.put(&key_owned, &second_value_owned).is_ok());

        // This compaction boundary is the one that matters for the current bug:
        // before it, reads can be satisfied from the memtable; after it, reads
        // must still preserve the old snapshot across flushed table state.
        assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());

        let snapshot_ref: &dyn Snapshot = unsafe { &*snapshot };
        body(dbtest, &key_owned, snapshot_ref);

        unsafe {
            (*dbtest.dbfull()).release_snapshot(snapshot);
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut run_case);
}
