// ---------------- [ File: bitcoinleveldb-dbtest/src/compare_iterators.rs ]
crate::ix!();

/// Let `K` be the set of byte strings (keys) with total order `≺` induced by `Slice::compare`,
/// and let `V` be the set of byte strings (values). An iterator execution yields a finite
/// sequence `S = [(k₁,v₁), …, (kₙ,vₙ)] ∈ (K×V)*` produced by the deterministic procedure:
/// `SeekToFirst; while Valid: emit (key,value); Next`.
///
/// For two DB instances plus optional snapshot references, this function returns `true` iff
/// the produced sequences are identical (pairwise in order) and their termination `Valid`
/// flags agree.
///
pub fn compare_iterators(
    step: i32,
    model: *mut dyn DB,
    db: *mut dyn DB,
    model_snap: *const dyn Snapshot,
    db_snap: *const dyn Snapshot,
) -> bool {
    let model_ptr_usize: usize = model as *const () as usize;
    let db_ptr_usize: usize = db as *const () as usize;
    let model_snap_ptr_usize: usize = model_snap as *const () as usize;
    let db_snap_ptr_usize: usize = db_snap as *const () as usize;

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "compare_iterators.entry",
        step,
        model_ptr_usize,
        db_ptr_usize,
        model_snap_ptr_usize,
        db_snap_ptr_usize
    );

    let mut options = ReadOptions::default();

    // options.snapshot = model_snap;
    let model_snapshot_opt: Option<Arc<dyn Snapshot>> = if model_snap.is_null() {
        None
    } else {
        unsafe {
            Arc::increment_strong_count(model_snap);
            Some(Arc::from_raw(model_snap))
        }
    };
    options.set_snapshot(model_snapshot_opt);

    let miter: *mut LevelDBIterator = unsafe { (&mut *model).new_iterator(&options) };

    // options.snapshot = db_snap;
    let db_snapshot_opt: Option<Arc<dyn Snapshot>> = if db_snap.is_null() {
        None
    } else {
        unsafe {
            Arc::increment_strong_count(db_snap);
            Some(Arc::from_raw(db_snap))
        }
    };
    options.set_snapshot(db_snapshot_opt);

    let dbiter: *mut LevelDBIterator = unsafe { (&mut *db).new_iterator(&options) };

    let mut ok: bool = true;
    let mut count: i32 = 0;

    unsafe {
        (&mut *miter).seek_to_first();
        (&mut *dbiter).seek_to_first();

        while ok && (*miter).valid() && (*dbiter).valid() {
            count += 1;

            if (*miter).key().compare(&(*dbiter).key()) != 0 {
                eprintln!(
                    "step {}: Key mismatch: '{}' vs. '{}'",
                    step,
                    escape_string(&(*miter).key()),  // defined elsewhere in codebase
                    escape_string(&(*dbiter).key())  // defined elsewhere in codebase
                );
                ok = false;
                break;
            }

            if (*miter).value().compare(&(*dbiter).value()) != 0 {
                eprintln!(
                    "step {}: Value mismatch for key '{}': '{}' vs. '{}'",
                    step,
                    escape_string(&(*miter).key()),    // defined elsewhere in codebase
                    escape_string(&(*miter).value()),  // defined elsewhere in codebase
                    escape_string(&(*miter).value())   // NOTE: mirrors C++ (prints miter value twice)
                );
                ok = false;
            }

            (*miter).next();
            (*dbiter).next();
        }

        if ok {
            if (*miter).valid() != (*dbiter).valid() {
                eprintln!(
                    "step {}: Mismatch at end of iterators: {} vs. {}",
                    step,
                    (*miter).valid() as i32,
                    (*dbiter).valid() as i32
                );
                ok = false;
            }
        }

        eprintln!("{} entries compared: ok={}", count, ok as i32);

        drop(Box::from_raw(miter));
        drop(Box::from_raw(dbiter));
    }

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "compare_iterators.exit",
        step,
        ok,
        count
    );

    ok
}
