// ---------------- [ File: bitcoinleveldbt-randomized/tests/randomized.rs ]
use traced_test::*;
use tracing_setup::*;
use bitcoinleveldb_rand::*;
use bitcoinleveldbt_dbtest::*;
use bitcoinleveldbt_util::*;
use bitcoinleveldbt_snapshot::*;
use bitcoinleveldb_snapshot::*;
use bitcoinleveldb_options::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_batch::*;
use bitcoinleveldb_dbinterface::*;
use bitcoinleveldb_modeldb::*;

#[path = "support/randomized_operations.rs"]
mod randomized_operations;
use randomized_operations::{BatchMutation, Mutation, OperationGenerator, LEGACY_STEPS_PER_CONFIGURATION};

#[test]
#[ignore = "long stress; use a resource-controlled runner explicitly"]
fn db_test_randomized() {
    let mut operations = OperationGenerator::new(bitcoinleveldbt_util::random_seed() as u32);

    let mut body = |dbtest: &mut DBTest| {
        operations.begin_configuration();
        let opts = dbtest.current_options();
        let mut model = ModelDB::new(&opts);
        const N: i32 = LEGACY_STEPS_PER_CONFIGURATION as i32;

        let mut model_snap: Option<Box<dyn Snapshot>> = None;
        let mut db_snap: Option<Box<dyn Snapshot>> = None;

        let mut step: i32 = 0;
        while step < N {
            if step % 100 == 0 {
                eprintln!("Step {} of {}", step, N);
            }

            // TODO(sanjay): Test Get() works
            let operation = operations.next_operation();
            match operation.mutation {
                Mutation::Put { key: k, value: v } => {
                    let write_options = WriteOptions::default();
                    let ks = Slice::from(&k);
                    let vs = Slice::from(&v);
                    assert!(model.put(&write_options, &ks, &vs).is_ok());
                    assert!(dbtest.put(&k, &v).is_ok());
                }
                Mutation::Delete { key: k } => {
                    let write_options = WriteOptions::default();
                    let ks = Slice::from(&k);
                    assert!(model.delete(&write_options, &ks).is_ok());
                    assert!(dbtest.delete(&k).is_ok());
                }
                Mutation::Batch(updates) => {
                    let mut b = WriteBatch::default();
                    for update in updates {
                        match update {
                            BatchMutation::Put { key, value } => {
                                b.put(&Slice::from(&key), &Slice::from(&value));
                            }
                            BatchMutation::Delete { key } => {
                                b.delete(&Slice::from(&key));
                            }
                        }
                    }
                    let write_options = WriteOptions::default();
                    assert!(model.write(&write_options, (&mut b) as *mut WriteBatch).is_ok());
                    assert!(unsafe {
                        (*dbtest.dbfull()).write(&write_options, (&mut b) as *mut WriteBatch)
                    }.is_ok());
                }
            }

            if (step % 100) == 0 {
                let model_ptr: *mut dyn DB = (&mut model as *mut ModelDB) as *mut dyn DB;
                let db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;

                assert!(compare_iterators(step, model_ptr, db_ptr, None, None));
                assert!(compare_iterators(
                    step,
                    model_ptr,
                    db_ptr,
                    model_snap.as_deref(),
                    db_snap.as_deref(),
                ));

                // Save a snapshot from each DB this time that we'll use next
                // time we compare things, to make sure the current state is
                // preserved with the snapshot
                if let Some(snapshot) = model_snap.take() {
                    model.release_snapshot(snapshot);
                }
                if let Some(snapshot) = db_snap.take() {
                    unsafe {
                        (*dbtest.dbfull()).release_snapshot(snapshot);
                    }
                }

                dbtest.reopen(None);

                let reopened_db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;
                assert!(compare_iterators(step, model_ptr, reopened_db_ptr, None, None));

                model_snap = Some(model.get_snapshot());
                db_snap = Some(unsafe { (*dbtest.dbfull()).get_snapshot() });
            }

            step += 1;
        }

        if let Some(snapshot) = model_snap.take() {
            model.release_snapshot(snapshot);
        }
        if let Some(snapshot) = db_snap.take() {
            unsafe {
                (*dbtest.dbfull()).release_snapshot(snapshot);
            }
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

