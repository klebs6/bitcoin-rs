// ---------------- [ File: bitcoinleveldbt-dbtest/src/fixture.rs ]
crate::ix!();

use std::sync::atomic::{AtomicU64, Ordering};

static DBTEST_TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Invariant: produces a deterministic-per-process unique temporary database pathname by
/// appending `suffix` plus a monotone instance id to the test harness temporary directory.
///
/// Precondition: `suffix` is a byte-stable path suffix.
/// Postcondition: the returned pathname begins with `bitcoinleveldbt_util::tmp_dir()` and is
/// distinct from prior calls in this process.
pub fn dbtest_fixture_tmp_dbname_with_suffix(suffix: &str) -> String {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_tmp_dbname_with_suffix.entry",
        suffix_len = suffix.len()
    );

    let id = DBTEST_TMP_COUNTER.fetch_add(1, Ordering::Relaxed);

    let mut dbname = bitcoinleveldbt_util::tmp_dir();
    dbname.push_str(suffix);
    dbname.push('.');
    dbname.push_str(&std::process::id().to_string());
    dbname.push('.');
    dbname.push_str(&id.to_string());

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_tmp_dbname_with_suffix.exit",
        dbname_len = dbname.len(),
        id = id
    );

    dbname
}

/// Invariant: preserves the `DB::Open` out-parameter contract by returning `Some(ptr)` iff the
/// underlying open status is ok, and `None` otherwise.
///
/// Precondition: `options` is initialized and `dbname` names the target database path.
/// Postcondition: the returned `Status` is exactly the one produced by `DBImpl::open`.
pub fn dbtest_fixture_open_db_pointer_with_options(
    dbname:  &String,
    options: &Options,
) -> (Status, Option<*mut dyn DB>) {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_open_db_pointer_with_options.entry",
        dbname_len = dbname.len()
    );

    let mut opener = DBImpl::new(options, dbname);
    let mut db_slot: MaybeUninit<*mut dyn DB> = MaybeUninit::uninit();
    let s = opener.open(options, dbname, db_slot.as_mut_ptr());

    let db_ptr = match s.is_ok() {
        true => Some(unsafe { db_slot.assume_init() }),
        false => None,
    };

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_open_db_pointer_with_options.exit",
        ok = s.is_ok(),
        has_db_ptr = db_ptr.is_some()
    );

    (s, db_ptr)
}

/// Invariant: releases ownership of an open raw DB pointer exactly once when present, and is a
/// no-op when the pointer is absent.
///
/// Precondition: any present pointer was returned by a successful `DBImpl::open`.
/// Postcondition: the allocation behind a present pointer has been dropped.
pub fn dbtest_fixture_drop_open_db_pointer(
    db_ptr: Option<*mut dyn DB>,
) {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_drop_open_db_pointer.entry",
        has_db_ptr = db_ptr.is_some()
    );

    match db_ptr {
        Some(ptr) => unsafe {
            drop(Box::from_raw(ptr));
        },
        None => {}
    }

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_drop_open_db_pointer.exit"
    );
}

/// Invariant: forwards owned key/value byte sequences to the DB harness without reinterpretation.
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: returns exactly the `Status` produced by `DBTest::put`.
pub fn dbtest_fixture_put_owned_string_pair(
    dbtest: &mut DBTest,
    key_owned: &String,
    value_owned: &String,
) -> Status {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_put_owned_string_pair.entry",
        key_len = key_owned.len(),
        value_len = value_owned.len()
    );

    let s = dbtest.put(key_owned, value_owned);

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_put_owned_string_pair.exit",
        ok = s.is_ok()
    );

    s
}

/// Invariant: forwards an owned key byte sequence to the DB harness without reinterpretation.
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: returns exactly the string produced by `DBTest::get(..., None)`.
pub fn dbtest_fixture_get_owned_string_key(
    dbtest: &mut DBTest,
    key_owned: &String,
) -> String {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_get_owned_string_key.entry",
        key_len = key_owned.len()
    );

    let out = dbtest.get(key_owned, None);

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_get_owned_string_key.exit",
        out_len = out.len()
    );

    out
}

/// Invariant: computes an approximate size over the exact byte bounds supplied by the caller.
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: returns exactly the value produced by `DBTest::size`.
pub fn dbtest_fixture_size_owned_string_bounds(
    dbtest: &mut DBTest,
    start_owned: &String,
    limit_owned: &String,
) -> u64 {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_size_owned_string_bounds.entry",
        start_len = start_owned.len(),
        limit_len = limit_owned.len()
    );

    let start_slice = Slice::from(start_owned);
    let limit_slice = Slice::from(limit_owned);
    let out = dbtest.size(start_slice, limit_slice);

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_size_owned_string_bounds.exit",
        out = out
    );

    out
}

/// Invariant: computes an approximate size over literal byte bounds without reinterpretation.
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: returns exactly the value produced by `DBTest::size`.
pub fn dbtest_fixture_size_literal_string_bounds(
    dbtest: &mut DBTest,
    start_literal: &str,
    limit_literal: &str,
) -> u64 {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_size_literal_string_bounds.entry",
        start_len = start_literal.len(),
        limit_len = limit_literal.len()
    );

    let start_owned = dbtest_fixture_owned_string(start_literal);
    let limit_owned = dbtest_fixture_owned_string(limit_literal);
    let out = dbtest_fixture_size_owned_string_bounds(dbtest, &start_owned, &limit_owned);

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_size_literal_string_bounds.exit",
        out = out
    );

    out
}

/// Invariant: forwards the memtable-compaction trigger exactly once and preserves the returned
/// `Status` without translation.
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: returns exactly the `Status` produced by `DBImpl::test_compact_mem_table`.
pub fn dbtest_fixture_test_compact_memtable_status(
    dbtest: &mut DBTest,
) -> Status {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_test_compact_memtable_status.entry"
    );

    let s = unsafe { (*dbtest.dbfull()).test_compact_mem_table() };

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_test_compact_memtable_status.exit",
        ok = s.is_ok()
    );

    s
}

/// Invariant: forwards optional owned bounds to `DBImpl::test_compact_range` exactly as pointers
/// to stable local `Slice` values for the duration of the call.
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: the underlying implementation observes either null bounds or the exact
/// byte bounds supplied by the caller.
pub fn dbtest_fixture_test_compact_range_optional_owned_bounds(
    dbtest: &mut DBTest,
    level: i32,
    start_owned: Option<&String>,
    limit_owned: Option<&String>,
) {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_test_compact_range_optional_owned_bounds.entry",
        level = level,
        has_start = start_owned.is_some(),
        has_limit = limit_owned.is_some()
    );

    let start_slice = match start_owned {
        Some(s) => Some(Slice::from(s)),
        None => None,
    };
    let limit_slice = match limit_owned {
        Some(s) => Some(Slice::from(s)),
        None => None,
    };

    let start_ptr: *const Slice = match start_slice.as_ref() {
        Some(s) => s as *const Slice,
        None => null::<Slice>(),
    };
    let limit_ptr: *const Slice = match limit_slice.as_ref() {
        Some(s) => s as *const Slice,
        None => null::<Slice>(),
    };

    unsafe {
        (*dbtest.dbfull()).test_compact_range(level, start_ptr, limit_ptr);
    }

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_test_compact_range_optional_owned_bounds.exit",
        level = level
    );
}
/// Invariant: returns an owned `String` whose byte content is exactly the provided literal.
///
/// Precondition: none.
/// Postcondition: the returned string compares byte-for-byte equal to `literal`.
pub fn dbtest_fixture_owned_string(literal: &str) -> String {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_owned_string.entry",
        literal_len = literal.len()
    );

    let out = literal.to_string();

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_owned_string.exit",
        out_len = out.len()
    );

    out
}

/// Invariant: forwards literal bytes to the DB test harness without reinterpretation.
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: returns exactly the `Status` produced by `DBTest::put`.
pub fn dbtest_fixture_put_literal(
    dbtest: &mut DBTest,
    key_literal: &str,
    value_literal: &str,
) -> Status {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_put_literal.entry",
        key_len = key_literal.len(),
        value_len = value_literal.len()
    );

    let key_owned = dbtest_fixture_owned_string(key_literal);
    let value_owned = dbtest_fixture_owned_string(value_literal);
    let s = dbtest.put(&key_owned, &value_owned);

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_put_literal.exit",
        ok = s.is_ok()
    );

    s
}

/// Invariant: forwards literal bytes to the DB test harness without reinterpretation.
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: returns exactly the `Status` produced by `DBTest::delete`.
pub fn dbtest_fixture_delete_literal(
    dbtest: &mut DBTest,
    key_literal: &str,
) -> Status {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_delete_literal.entry",
        key_len = key_literal.len()
    );

    let key_owned = dbtest_fixture_owned_string(key_literal);
    let s = dbtest.delete(&key_owned);

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_delete_literal.exit",
        ok = s.is_ok()
    );

    s
}

/// Invariant: forwards literal bytes to the DB test harness without reinterpretation and
/// observes the current head state (no snapshot).
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: returns exactly the string produced by `DBTest::get(..., None)`.
pub fn dbtest_fixture_get_literal(
    dbtest: &mut DBTest,
    key_literal: &str,
) -> String {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_get_literal.entry",
        key_len = key_literal.len()
    );

    let key_owned = dbtest_fixture_owned_string(key_literal);
    let out = dbtest.get(&key_owned, None);

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_get_literal.exit",
        out_len = out.len()
    );

    out
}

/// Invariant: materializes `Slice` views whose lifetimes remain bounded by the owned strings
/// held for the duration of the call.
///
/// Precondition: `dbtest` owns an open DB instance.
/// Postcondition: invokes `DBTest::compact` on exactly the specified byte ranges.
pub fn dbtest_fixture_compact_literal_range(
    dbtest: &mut DBTest,
    start_literal: &str,
    limit_literal: &str,
) {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_compact_literal_range.entry",
        start_len = start_literal.len(),
        limit_len = limit_literal.len()
    );

    let start_owned = dbtest_fixture_owned_string(start_literal);
    let limit_owned = dbtest_fixture_owned_string(limit_literal);
    let start_slice = Slice::from(&start_owned);
    let limit_slice = Slice::from(&limit_owned);

    dbtest.compact(&start_slice, &limit_slice);

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_compact_literal_range.exit"
    );
}

/// Invariant: executes the body at least once and advances option configurations in the same
/// order as `do { ... } while (ChangeOptions());`.
///
/// Precondition: `body` is deterministic with respect to the mutable `DBTest` state supplied.
/// Postcondition: `body` has been invoked once for every reachable option configuration.
pub fn dbtest_fixture_run_across_option_configurations<F>(body: &mut F)
where
    F: FnMut(&mut DBTest),
{
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_run_across_option_configurations.entry"
    );

    let mut dbtest = DBTest::default();
    let mut rounds: i32 = 0;

    loop {
        body(&mut dbtest);
        rounds += 1;

        if !dbtest.change_options() {
            break;
        }
    }

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::tests",
        label = "dbtest_fixture_run_across_option_configurations.exit",
        rounds
    );
}

#[traced_test]
fn db_test_empty() {
    let mut body = |dbtest: &mut DBTest| {
        assert!(!dbtest.dbfull().is_null());
        assert_eq!("NOT_FOUND", dbtest_fixture_get_literal(dbtest, "foo"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_empty_key() {
    let mut body = |dbtest: &mut DBTest| {
        let s1 = dbtest_fixture_put_literal(dbtest, "", "v1");
        assert!(s1.is_ok());
        assert_eq!("v1", dbtest_fixture_get_literal(dbtest, ""));

        let s2 = dbtest_fixture_put_literal(dbtest, "", "v2");
        assert!(s2.is_ok());
        assert_eq!("v2", dbtest_fixture_get_literal(dbtest, ""));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_empty_value() {
    let mut body = |dbtest: &mut DBTest| {
        let s1 = dbtest_fixture_put_literal(dbtest, "key", "v1");
        assert!(s1.is_ok());
        assert_eq!("v1", dbtest_fixture_get_literal(dbtest, "key"));

        let s2 = dbtest_fixture_put_literal(dbtest, "key", "");
        assert!(s2.is_ok());
        assert_eq!("", dbtest_fixture_get_literal(dbtest, "key"));

        let s3 = dbtest_fixture_put_literal(dbtest, "key", "v2");
        assert!(s3.is_ok());
        assert_eq!("v2", dbtest_fixture_get_literal(dbtest, "key"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_read_write() {
    let mut body = |dbtest: &mut DBTest| {
        let s1 = dbtest_fixture_put_literal(dbtest, "foo", "v1");
        assert!(s1.is_ok());
        assert_eq!("v1", dbtest_fixture_get_literal(dbtest, "foo"));

        let s2 = dbtest_fixture_put_literal(dbtest, "bar", "v2");
        assert!(s2.is_ok());

        let s3 = dbtest_fixture_put_literal(dbtest, "foo", "v3");
        assert!(s3.is_ok());

        assert_eq!("v3", dbtest_fixture_get_literal(dbtest, "foo"));
        assert_eq!("v2", dbtest_fixture_get_literal(dbtest, "bar"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_put_delete_get() {
    let mut body = |dbtest: &mut DBTest| {
        let s1 = dbtest_fixture_put_literal(dbtest, "foo", "v1");
        assert!(s1.is_ok());
        assert_eq!("v1", dbtest_fixture_get_literal(dbtest, "foo"));

        let s2 = dbtest_fixture_put_literal(dbtest, "foo", "v2");
        assert!(s2.is_ok());
        assert_eq!("v2", dbtest_fixture_get_literal(dbtest, "foo"));

        let s3 = dbtest_fixture_delete_literal(dbtest, "foo");
        assert!(s3.is_ok());
        assert_eq!("NOT_FOUND", dbtest_fixture_get_literal(dbtest, "foo"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_get_from_versions() {
    let mut body = |dbtest: &mut DBTest| {
        let s = dbtest_fixture_put_literal(dbtest, "foo", "v1");
        assert!(s.is_ok());

        let _ = unsafe { (*dbtest.dbfull()).test_compact_mem_table() };

        assert_eq!("v1", dbtest_fixture_get_literal(dbtest, "foo"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_get_mem_usage() {
    let mut body = |dbtest: &mut DBTest| {
        let s = dbtest_fixture_put_literal(dbtest, "foo", "v1");
        assert!(s.is_ok());

        let property_name = dbtest_fixture_owned_string("leveldb.approximate-memory-usage");
        let mut value = String::new();
        let ok = unsafe { (*dbtest.dbfull()).get_property(&property_name, &mut value as *mut String) };
        assert!(ok);

        let mem_usage = match value.parse::<i32>() {
            Ok(parsed) => parsed,
            Err(_) => {
                assert!(false);
                0
            }
        };

        assert!(mem_usage > 0);
        assert!(mem_usage < 5 * 1024 * 1024);
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_get_level_0ordering() {
    let mut body = |dbtest: &mut DBTest| {
        let s1 = dbtest_fixture_put_literal(dbtest, "bar", "b");
        assert!(s1.is_ok());

        let s2 = dbtest_fixture_put_literal(dbtest, "foo", "v1");
        assert!(s2.is_ok());

        let _ = unsafe { (*dbtest.dbfull()).test_compact_mem_table() };

        let s3 = dbtest_fixture_put_literal(dbtest, "foo", "v2");
        assert!(s3.is_ok());

        let _ = unsafe { (*dbtest.dbfull()).test_compact_mem_table() };

        assert_eq!("v2", dbtest_fixture_get_literal(dbtest, "foo"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_get_ordered_by_levels() {
    let mut body = |dbtest: &mut DBTest| {
        let s1 = dbtest_fixture_put_literal(dbtest, "foo", "v1");
        assert!(s1.is_ok());

        dbtest_fixture_compact_literal_range(dbtest, "a", "z");
        assert_eq!("v1", dbtest_fixture_get_literal(dbtest, "foo"));

        let s2 = dbtest_fixture_put_literal(dbtest, "foo", "v2");
        assert!(s2.is_ok());
        assert_eq!("v2", dbtest_fixture_get_literal(dbtest, "foo"));

        let _ = unsafe { (*dbtest.dbfull()).test_compact_mem_table() };
        assert_eq!("v2", dbtest_fixture_get_literal(dbtest, "foo"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_get_picks_correct_file() {
    let mut body = |dbtest: &mut DBTest| {
        // Arrange to have multiple files in a non-level-0 level.
        let s1 = dbtest_fixture_put_literal(dbtest, "a", "va");
        assert!(s1.is_ok());
        dbtest_fixture_compact_literal_range(dbtest, "a", "b");

        let s2 = dbtest_fixture_put_literal(dbtest, "x", "vx");
        assert!(s2.is_ok());
        dbtest_fixture_compact_literal_range(dbtest, "x", "y");

        let s3 = dbtest_fixture_put_literal(dbtest, "f", "vf");
        assert!(s3.is_ok());
        dbtest_fixture_compact_literal_range(dbtest, "f", "g");

        assert_eq!("va", dbtest_fixture_get_literal(dbtest, "a"));
        assert_eq!("vf", dbtest_fixture_get_literal(dbtest, "f"));
        assert_eq!("vx", dbtest_fixture_get_literal(dbtest, "x"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_iter_empty() {
    let mut dbtest = DBTest::default();
    let iter = unsafe { (*dbtest.dbfull()).new_iterator(&ReadOptions::default()) };

    unsafe {
        (*iter).seek_to_first();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_last();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    let foo_owned = dbtest_fixture_owned_string("foo");
    let foo_slice = Slice::from(&foo_owned);
    unsafe {
        (*iter).seek(&foo_slice);
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        drop(Box::from_raw(iter));
    }
}

#[traced_test]
fn db_test_iter_single() {
    let mut dbtest = DBTest::default();
    let s = dbtest_fixture_put_literal(&mut dbtest, "a", "va");
    assert!(s.is_ok());

    let iter = unsafe { (*dbtest.dbfull()).new_iterator(&ReadOptions::default()) };

    unsafe {
        (*iter).seek_to_first();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_first();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_last();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_last();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    let empty_owned = dbtest_fixture_owned_string("");
    let empty_slice = Slice::from(&empty_owned);
    unsafe {
        (*iter).seek(&empty_slice);
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    let a_owned = dbtest_fixture_owned_string("a");
    let a_slice = Slice::from(&a_owned);
    unsafe {
        (*iter).seek(&a_slice);
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    let b_owned = dbtest_fixture_owned_string("b");
    let b_slice = Slice::from(&b_owned);
    unsafe {
        (*iter).seek(&b_slice);
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        drop(Box::from_raw(iter));
    }
}

#[traced_test]
fn db_test_iter_multi() {
    let mut dbtest = DBTest::default();
    assert!(dbtest_fixture_put_literal(&mut dbtest, "a", "va").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "b", "vb").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "c", "vc").is_ok());

    let iter = unsafe { (*dbtest.dbfull()).new_iterator(&ReadOptions::default()) };

    unsafe {
        (*iter).seek_to_first();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "b->vb");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "c->vc");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_first();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_last();
    }
    assert_eq!(dbtest.iter_status(iter), "c->vc");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "b->vb");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_last();
    }
    assert_eq!(dbtest.iter_status(iter), "c->vc");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    let empty_owned = dbtest_fixture_owned_string("");
    let empty_slice = Slice::from(&empty_owned);
    unsafe {
        (*iter).seek(&empty_slice);
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    let a_owned = dbtest_fixture_owned_string("a");
    let a_slice = Slice::from(&a_owned);
    unsafe {
        (*iter).seek(&a_slice);
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    let ax_owned = dbtest_fixture_owned_string("ax");
    let ax_slice = Slice::from(&ax_owned);
    unsafe {
        (*iter).seek(&ax_slice);
    }
    assert_eq!(dbtest.iter_status(iter), "b->vb");

    let b_owned = dbtest_fixture_owned_string("b");
    let b_slice = Slice::from(&b_owned);
    unsafe {
        (*iter).seek(&b_slice);
    }
    assert_eq!(dbtest.iter_status(iter), "b->vb");

    let z_owned = dbtest_fixture_owned_string("z");
    let z_slice = Slice::from(&z_owned);
    unsafe {
        (*iter).seek(&z_slice);
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_last();
        (*iter).prev();
        (*iter).prev();
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "b->vb");

    unsafe {
        (*iter).seek_to_first();
        (*iter).next();
        (*iter).next();
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "b->vb");

    assert!(dbtest_fixture_put_literal(&mut dbtest, "a", "va2").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "a2", "va3").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "b", "vb2").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "c", "vc2").is_ok());
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "b").is_ok());

    unsafe {
        (*iter).seek_to_first();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "b->vb");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "c->vc");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_last();
    }
    assert_eq!(dbtest.iter_status(iter), "c->vc");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "b->vb");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        drop(Box::from_raw(iter));
    }
}

#[traced_test]
fn db_test_iter_small_and_large_mix() {
    let mut dbtest = DBTest::default();

    let b_large = "b".repeat(100000);
    let d_large = "d".repeat(100000);
    let e_large = "e".repeat(100000);

    assert!(dbtest_fixture_put_literal(&mut dbtest, "a", "va").is_ok());

    let b_key = dbtest_fixture_owned_string("b");
    assert!(dbtest.put(&b_key, &b_large).is_ok());

    assert!(dbtest_fixture_put_literal(&mut dbtest, "c", "vc").is_ok());

    let d_key = dbtest_fixture_owned_string("d");
    assert!(dbtest.put(&d_key, &d_large).is_ok());

    let e_key = dbtest_fixture_owned_string("e");
    assert!(dbtest.put(&e_key, &e_large).is_ok());

    let iter = unsafe { (*dbtest.dbfull()).new_iterator(&ReadOptions::default()) };

    unsafe {
        (*iter).seek_to_first();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), format!("b->{}", b_large));

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "c->vc");

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), format!("d->{}", d_large));

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), format!("e->{}", e_large));

    unsafe {
        (*iter).next();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        (*iter).seek_to_last();
    }
    assert_eq!(dbtest.iter_status(iter), format!("e->{}", e_large));

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), format!("d->{}", d_large));

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "c->vc");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), format!("b->{}", b_large));

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "a->va");

    unsafe {
        (*iter).prev();
    }
    assert_eq!(dbtest.iter_status(iter), "(invalid)");

    unsafe {
        drop(Box::from_raw(iter));
    }
}

#[traced_test]
fn db_test_recover() {
    let mut body = |dbtest: &mut DBTest| {
        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v1").is_ok());
        assert!(dbtest_fixture_put_literal(dbtest, "baz", "v5").is_ok());

        dbtest.reopen(None);
        assert_eq!("v1", dbtest_fixture_get_literal(dbtest, "foo"));

        assert_eq!("v1", dbtest_fixture_get_literal(dbtest, "foo"));
        assert_eq!("v5", dbtest_fixture_get_literal(dbtest, "baz"));

        assert!(dbtest_fixture_put_literal(dbtest, "bar", "v2").is_ok());
        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v3").is_ok());

        dbtest.reopen(None);
        assert_eq!("v3", dbtest_fixture_get_literal(dbtest, "foo"));

        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v4").is_ok());
        assert_eq!("v4", dbtest_fixture_get_literal(dbtest, "foo"));
        assert_eq!("v2", dbtest_fixture_get_literal(dbtest, "bar"));
        assert_eq!("v5", dbtest_fixture_get_literal(dbtest, "baz"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_recovery_with_empty_log() {
    let mut body = |dbtest: &mut DBTest| {
        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v1").is_ok());
        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v2").is_ok());

        dbtest.reopen(None);
        dbtest.reopen(None);

        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v3").is_ok());

        dbtest.reopen(None);
        assert_eq!("v3", dbtest_fixture_get_literal(dbtest, "foo"));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_minor_compactions_happen() {
    let mut dbtest = DBTest::default();
    let mut options = dbtest.current_options();
    options.set_write_buffer_size(10000);
    dbtest.reopen(Some(&mut options));

    let n: i32 = 500;
    let starting_num_tables = dbtest.total_table_files();

    let value_suffix = "v".repeat(1000);
    let mut i: i32 = 0;
    while i < n {
        let k = crate::key(i);
        let v = format!("{}{}", k, value_suffix);
        let s = dbtest.put(&k, &v);
        assert!(s.is_ok());
        i += 1;
    }

    let ending_num_tables = dbtest.total_table_files();
    assert!(ending_num_tables > starting_num_tables);

    let mut j: i32 = 0;
    while j < n {
        let k = crate::key(j);
        let expected = format!("{}{}", k, value_suffix);
        assert_eq!(expected, dbtest.get(&k, None));
        j += 1;
    }

    dbtest.reopen(None);

    let mut k_index: i32 = 0;
    while k_index < n {
        let k = crate::key(k_index);
        let expected = format!("{}{}", k, value_suffix);
        assert_eq!(expected, dbtest.get(&k, None));
        k_index += 1;
    }
}

#[traced_test]
fn db_test_iterator_pins_ref() {
    let mut dbtest = DBTest::default();
    assert!(dbtest_fixture_put_literal(&mut dbtest, "foo", "hello").is_ok());

      // Get iterator that will yield the current contents of the DB.
    let iter = unsafe { (*dbtest.dbfull()).new_iterator(&ReadOptions::default()) };

    // Write to force compactions
    assert!(dbtest_fixture_put_literal(&mut dbtest, "foo", "newvalue1").is_ok());

    let value_suffix = "v".repeat(100000);
    let mut i: i32 = 0;
    while i < 100 {
        let k = crate::key(i);
        let v = format!("{}{}", k, value_suffix);
        let s = dbtest.put(&k, &v);
        assert!(s.is_ok());
        i += 1;
    }

    assert!(dbtest_fixture_put_literal(&mut dbtest, "foo", "newvalue2").is_ok());

    unsafe {
        (*iter).seek_to_first();
    }
    assert!(unsafe { (*iter).valid() });
    assert_eq!("foo", unsafe { (*iter).key().to_string() });
    assert_eq!("hello", unsafe { (*iter).value().to_string() });

    unsafe {
        (*iter).next();
    }
    assert!(!unsafe { (*iter).valid() });

    unsafe {
        drop(Box::from_raw(iter));
    }
}

#[traced_test]
fn db_test_files_deleted_after_compaction() {
    let mut dbtest = DBTest::default();

    assert!(dbtest_fixture_put_literal(&mut dbtest, "foo", "v2").is_ok());
    dbtest_fixture_compact_literal_range(&mut dbtest, "a", "z");

    let num_files = dbtest.count_files();

    let mut i: i32 = 0;
    while i < 10 {
        assert!(dbtest_fixture_put_literal(&mut dbtest, "foo", "v2").is_ok());
        dbtest_fixture_compact_literal_range(&mut dbtest, "a", "z");
        i += 1;
    }

    assert_eq!(dbtest.count_files(), num_files);
}

#[traced_test]
fn db_test_get_from_immutable_layer() {
    let mut body = |dbtest: &mut DBTest| {
        let mut options = dbtest.current_options();
        options.set_write_buffer_size(100000); // Small write buffer
        dbtest.reopen(Some(&mut options));

        let foo = "foo".to_string();
        let v1 = "v1".to_string();
        assert!(dbtest.put(&foo, &v1).is_ok());
        assert_eq!("v1", dbtest.get(&foo, None));

        // Block sync calls.
        unsafe {
            (*dbtest.special_env())
                .delay_data_sync()
                .store(true, atomic::Ordering::Release);
        }

        let k1 = "k1".to_string();
        let k2 = "k2".to_string();
        let big_x = "x".repeat(100000);
        let big_y = "y".repeat(100000);

        assert!(dbtest.put(&k1, &big_x).is_ok()); // Fill memtable.
        assert!(dbtest.put(&k2, &big_y).is_ok()); // Trigger compaction.
        assert_eq!("v1", dbtest.get(&foo, None));

        // Release sync calls.
        unsafe {
            (*dbtest.special_env())
                .delay_data_sync()
                .store(false, atomic::Ordering::Release);
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_get_encounters_empty_level() {
    let mut body = |dbtest: &mut DBTest| {
        // Arrange for the following to happen:
        //   * sstable A in level 0
        //   * nothing in level 1
        //   * sstable B in level 2
        // Then do enough Get() calls to arrange for an automatic compaction
        // of sstable A.  A bug would cause the compaction to be marked as
        // occurring at level 1 (instead of the correct level 0).

        // Step 1: First place sstables in levels 0 and 2
        let mut compaction_count: i32 = 0;

        while dbtest.num_table_files_at_level(0) == 0 || dbtest.num_table_files_at_level(2) == 0 {
            assert!(compaction_count <= 100);
            compaction_count += 1;

            assert!(dbtest_fixture_put_literal(dbtest, "a", "begin").is_ok());
            assert!(dbtest_fixture_put_literal(dbtest, "z", "end").is_ok());
            assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());
        }

        // Step 2: clear level 1 if necessary.
        dbtest_fixture_test_compact_range_optional_owned_bounds(dbtest, 1, None, None);

        assert_eq!(1, dbtest.num_table_files_at_level(0));
        assert_eq!(0, dbtest.num_table_files_at_level(1));
        assert_eq!(1, dbtest.num_table_files_at_level(2));

        // Step 3: read a bunch of times
        let mut i: i32 = 0;
        while i < 1000 {
            assert_eq!("NOT_FOUND", dbtest_fixture_get_literal(dbtest, "missing"));
            i += 1;
        }

        // Step 4: Wait for compaction to finish
        delay_milliseconds(1000);

        assert_eq!(0, dbtest.num_table_files_at_level(0));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_iter_multi_with_delete() {
    let mut body = |dbtest: &mut DBTest| {
        assert!(dbtest_fixture_put_literal(dbtest, "a", "va").is_ok());
        assert!(dbtest_fixture_put_literal(dbtest, "b", "vb").is_ok());
        assert!(dbtest_fixture_put_literal(dbtest, "c", "vc").is_ok());
        assert!(dbtest_fixture_delete_literal(dbtest, "b").is_ok());
        assert_eq!("NOT_FOUND", dbtest_fixture_get_literal(dbtest, "b"));

        let iter = unsafe { (*dbtest.dbfull()).new_iterator(&ReadOptions::default()) };

        let c_owned = dbtest_fixture_owned_string("c");
        let c_slice = Slice::from(&c_owned);
        unsafe {
            (*iter).seek(&c_slice);
        }
        assert_eq!(dbtest.iter_status(iter), "c->vc");

        unsafe {
            (*iter).prev();
        }
        assert_eq!(dbtest.iter_status(iter), "a->va");

        unsafe {
            drop(Box::from_raw(iter));
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

/**
  | Check that writes done during a memtable
  | compaction are recovered if the database
  | is shutdown during the memtable compaction.
  |
  */
#[traced_test]
fn db_test_recover_during_memtable_compaction() {
    let mut body = |dbtest: &mut DBTest| {
        let mut options = dbtest.current_options();
        options.set_write_buffer_size(1000000);
        dbtest.reopen(Some(&mut options));

        // Trigger a long memtable compaction and reopen the database during it
        let foo = "foo".to_string();
        let bar = "bar".to_string();
        let big1_key = "big1".to_string();
        let big2_key = "big2".to_string();

        let v1 = "v1".to_string();
        let v2 = "v2".to_string();
        let big1_value = "x".repeat(10000000);
        let big2_value = "y".repeat(1000);

        assert!(dbtest.put(&foo, &v1).is_ok());               // Goes to 1st log file
        assert!(dbtest.put(&big1_key, &big1_value).is_ok());  // Fills memtable
        assert!(dbtest.put(&big2_key, &big2_value).is_ok());  // Triggers compaction
        assert!(dbtest.put(&bar, &v2).is_ok());               // Goes to new log file

        dbtest.reopen(Some(&mut options));
        assert_eq!("v1", dbtest.get(&foo, None));
        assert_eq!("v2", dbtest.get(&bar, None));
        assert_eq!(big1_value, dbtest.get(&big1_key, None));
        assert_eq!(big2_value, dbtest.get(&big2_key, None));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_recover_with_large_log() {
    let mut dbtest = DBTest::default();

    {
        let mut options = dbtest.current_options();
        dbtest.reopen(Some(&mut options));

        let big1_key = dbtest_fixture_owned_string("big1");
        let big2_key = dbtest_fixture_owned_string("big2");
        let small3_key = dbtest_fixture_owned_string("small3");
        let small4_key = dbtest_fixture_owned_string("small4");

        let big1_value = "1".repeat(200000);
        let big2_value = "2".repeat(200000);
        let small3_value = "3".repeat(10);
        let small4_value = "4".repeat(10);

        assert!(dbtest_fixture_put_owned_string_pair(&mut dbtest, &big1_key, &big1_value).is_ok());
        assert!(dbtest_fixture_put_owned_string_pair(&mut dbtest, &big2_key, &big2_value).is_ok());
        assert!(dbtest_fixture_put_owned_string_pair(&mut dbtest, &small3_key, &small3_value).is_ok());
        assert!(dbtest_fixture_put_owned_string_pair(&mut dbtest, &small4_key, &small4_value).is_ok());

        assert_eq!(0, dbtest.num_table_files_at_level(0));
    }

    // Make sure that if we re-open with a small write buffer size that
    // we flush table files in the middle of a large log file.
    let mut options = dbtest.current_options();
    options.set_write_buffer_size(100000);
    dbtest.reopen(Some(&mut options));

    let big1_key = dbtest_fixture_owned_string("big1");
    let big2_key = dbtest_fixture_owned_string("big2");
    let small3_key = dbtest_fixture_owned_string("small3");
    let small4_key = dbtest_fixture_owned_string("small4");

    let big1_value = "1".repeat(200000);
    let big2_value = "2".repeat(200000);
    let small3_value = "3".repeat(10);
    let small4_value = "4".repeat(10);

    assert_eq!(3, dbtest.num_table_files_at_level(0));
    assert_eq!(big1_value, dbtest_fixture_get_owned_string_key(&mut dbtest, &big1_key));
    assert_eq!(big2_value, dbtest_fixture_get_owned_string_key(&mut dbtest, &big2_key));
    assert_eq!(small3_value, dbtest_fixture_get_owned_string_key(&mut dbtest, &small3_key));
    assert_eq!(small4_value, dbtest_fixture_get_owned_string_key(&mut dbtest, &small4_key));
    assert!(dbtest.num_table_files_at_level(0) > 1);
}

#[traced_test]
fn db_test_compactions_generate_multiple_files() {
    let mut dbtest = DBTest::default();
    let mut options = dbtest.current_options();
    options.set_write_buffer_size(100000000);
    dbtest.reopen(Some(&mut options));

    let mut rnd = Random::new(301);
    let mut values: Vec<String> = Vec::new();

    // Write 8MB (80 values, each 100K)
    assert_eq!(0, dbtest.num_table_files_at_level(0));

    let mut i: i32 = 0;
    while i < 80 {
        let value = dbtest_random_string((&mut rnd) as *mut Random, 100000);
        let key_owned = crate::key(i);
        let s = dbtest.put(&key_owned, &value);
        assert!(s.is_ok());
        values.push(value);
        i += 1;
    }

    // Reopening moves updates to level-0
    dbtest.reopen(Some(&mut options));
    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 0, None, None);

    assert_eq!(0, dbtest.num_table_files_at_level(0));
    assert!(dbtest.num_table_files_at_level(1) > 1);

    let mut j: i32 = 0;
    while j < 80 {
        let key_owned = crate::key(j);
        assert_eq!(values[j as usize], dbtest_fixture_get_owned_string_key(&mut dbtest, &key_owned));
        j += 1;
    }
}

#[traced_test]
fn db_test_repeated_writes_to_same_key() {
    let mut dbtest = DBTest::default();
    let mut options = dbtest.current_options();
    options.set_write_buffer_size(100000); // Small write buffer
    dbtest.reopen(Some(&mut options));

    // We must have at most one file per level except for level-0,
    // which may have up to kL0_StopWritesTrigger files.
    let k_max_files: i32 =
        (bitcoinleveldb_cfg::NUM_LEVELS as i32) + (bitcoinleveldb_cfg::L0_STOP_WRITES_TRIGGER as i32);

    let mut rnd = Random::new(301);
    let value = dbtest_random_string(
        (&mut rnd) as *mut Random,
        ((*options.write_buffer_size()) * 2) as i32,
    );
    let key = "key".to_string();

    let mut i: i32 = 0;
    while i < 5 * k_max_files {
        assert!(dbtest.put(&key, &value).is_ok());
        assert!(dbtest.total_table_files() <= k_max_files);
        eprintln!("after {}: {} files", i + 1, dbtest.total_table_files());
        i += 1;
    }
}

#[traced_test]
fn db_test_sparse_merge() {
    let mut dbtest = DBTest::default();
    let mut options = dbtest.current_options();
    options.set_compression(CompressionType::None);
    dbtest.reopen(Some(&mut options));

    let smallest = dbtest_fixture_owned_string("A");
    let largest = dbtest_fixture_owned_string("Z");
    dbtest.fill_levels(&smallest, &largest);

    // Suppose there is:
    //    small amount of data with prefix A
    //    large amount of data with prefix B
    //    small amount of data with prefix C
    // and that recent updates have made small changes to all three prefixes.
    // Check that we do not do a compaction that merges all of B in one shot.
    let value = "x".repeat(1000);

    assert!(dbtest_fixture_put_literal(&mut dbtest, "A", "va").is_ok());

    // Write approximately 100MB of "B" values
    let mut i: i32 = 0;
    while i < 100000 {
        let key_owned = format!("B{:010}", i);
        assert!(dbtest.put(&key_owned, &value).is_ok());
        i += 1;
    }

    assert!(dbtest_fixture_put_literal(&mut dbtest, "C", "vc").is_ok());
    assert!(dbtest_fixture_test_compact_memtable_status(&mut dbtest).is_ok());
    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 0, None, None);

    // Make sparse update
    assert!(dbtest_fixture_put_literal(&mut dbtest, "A", "va2").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "B100", "bvalue2").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "C", "vc2").is_ok());
    assert!(dbtest_fixture_test_compact_memtable_status(&mut dbtest).is_ok());

    // Compactions should not cause us to create a situation where
    // a file overlaps too much data at the next level.
    assert!(unsafe { (*dbtest.dbfull()).test_max_next_level_overlapping_bytes() } <= 20_i64 * 1048576_i64);

    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 0, None, None);
    assert!(unsafe { (*dbtest.dbfull()).test_max_next_level_overlapping_bytes() } <= 20_i64 * 1048576_i64);

    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 1, None, None);
    assert!(unsafe { (*dbtest.dbfull()).test_max_next_level_overlapping_bytes() } <= 20_i64 * 1048576_i64);
}

#[traced_test]
fn db_test_approximate_sizes() {
    let mut body = |dbtest: &mut DBTest| {
        let mut options = dbtest.current_options();
        options.set_write_buffer_size(100000000);
        options.set_compression(CompressionType::None);

        dbtest.destroy_and_reopen(None);

        assert!(between(
            dbtest_fixture_size_literal_string_bounds(dbtest, "", "xyz"),
            0,
            0
        ));

        dbtest.reopen(Some(&mut options));

        assert!(between(
            dbtest_fixture_size_literal_string_bounds(dbtest, "", "xyz"),
            0,
            0
        ));

        // Write 8MB (80 values, each 100K)
        assert_eq!(0, dbtest.num_table_files_at_level(0));

        const N: i32 = 80;
        const S1: u64 = 100000;
        const S2: u64 = 105000; // Allow some expansion from metadata

        let mut rnd = Random::new(301);
        let mut i: i32 = 0;
        while i < N {
            let key_owned = crate::key(i);
            let value_owned = dbtest_random_string((&mut rnd) as *mut Random, S1 as i32);
            assert!(dbtest.put(&key_owned, &value_owned).is_ok());
            i += 1;
        }

        let empty_owned = dbtest_fixture_owned_string("");
        let key50_owned = crate::key(50);

        // 0 because GetApproximateSizes() does not account for memtable space
        assert!(between(
            dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key50_owned),
            0,
            0
        ));

        if *options.reuse_logs() {
            // Recovery will reuse memtable, and GetApproximateSizes() does not
            // account for memtable usage;
            dbtest.reopen(Some(&mut options));
            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key50_owned),
                0,
                0
            ));
            return;
        }

        // Check sizes across recovery by reopening a few times
        let mut run: i32 = 0;
        while run < 3 {
            dbtest.reopen(Some(&mut options));

            let mut compact_start: i32 = 0;
            while compact_start < N {
                let mut inner_i: i32 = 0;
                while inner_i < N {
                    let key_i_owned = crate::key(inner_i);
                    let key_i_plus_10_owned = crate::key(inner_i + 10);
                    let key_i_suffix_owned = format!("{}.suffix", key_i_owned);

                    assert!(between(
                        dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key_i_owned),
                        S1 * (inner_i as u64),
                        S2 * (inner_i as u64)
                    ));

                    assert!(between(
                        dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key_i_suffix_owned),
                        S1 * ((inner_i + 1) as u64),
                        S2 * ((inner_i + 1) as u64)
                    ));

                    assert!(between(
                        dbtest_fixture_size_owned_string_bounds(dbtest, &key_i_owned, &key_i_plus_10_owned),
                        S1 * 10_u64,
                        S2 * 10_u64
                    ));

                    inner_i += 10;
                }

                let key50_suffix_owned = format!("{}.suffix", key50_owned);
                assert!(between(
                    dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key50_owned),
                    S1 * 50_u64,
                    S2 * 50_u64
                ));
                assert!(between(
                    dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key50_suffix_owned),
                    S1 * 50_u64,
                    S2 * 50_u64
                ));

                let cstart_owned = crate::key(compact_start);
                let cend_owned = crate::key(compact_start + 9);
                dbtest_fixture_test_compact_range_optional_owned_bounds(
                    dbtest,
                    0,
                    Some(&cstart_owned),
                    Some(&cend_owned),
                );

                compact_start += 10;
            }

            assert_eq!(0, dbtest.num_table_files_at_level(0));
            assert!(dbtest.num_table_files_at_level(1) > 0);

            run += 1;
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_approximate_sizes_mix_of_small_and_large() {
    let mut body = |dbtest: &mut DBTest| {
        let mut options = dbtest.current_options();
        options.set_compression(CompressionType::None);
        dbtest.reopen(Some(&mut options));

        let mut rnd = Random::new(301);

        let big1 = dbtest_random_string((&mut rnd) as *mut Random, 100000);

        let key0 = crate::key(0);
        let key1 = crate::key(1);
        let key2 = crate::key(2);
        let key3 = crate::key(3);
        let key4 = crate::key(4);
        let key5 = crate::key(5);
        let key6 = crate::key(6);
        let key7 = crate::key(7);
        let key8 = crate::key(8);

        let val0 = dbtest_random_string((&mut rnd) as *mut Random, 10000);
        let val1 = dbtest_random_string((&mut rnd) as *mut Random, 10000);
        let val3 = dbtest_random_string((&mut rnd) as *mut Random, 10000);
        let val5 = dbtest_random_string((&mut rnd) as *mut Random, 10000);
        let val6 = dbtest_random_string((&mut rnd) as *mut Random, 300000);
        let val7 = dbtest_random_string((&mut rnd) as *mut Random, 10000);

        assert!(dbtest.put(&key0, &val0).is_ok());
        assert!(dbtest.put(&key1, &val1).is_ok());
        assert!(dbtest.put(&key2, &big1).is_ok());
        assert!(dbtest.put(&key3, &val3).is_ok());
        assert!(dbtest.put(&key4, &big1).is_ok());
        assert!(dbtest.put(&key5, &val5).is_ok());
        assert!(dbtest.put(&key6, &val6).is_ok());
        assert!(dbtest.put(&key7, &val7).is_ok());

        if *options.reuse_logs() {
            assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());
        }

        let empty_owned = dbtest_fixture_owned_string("");

        let mut run: i32 = 0;
        while run < 3 {
            dbtest.reopen(Some(&mut options));

            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key0),
                0,
                0
            ));
            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key1),
                10000,
                11000
            ));
            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key2),
                20000,
                21000
            ));
            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key3),
                120000,
                121000
            ));
            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key4),
                130000,
                131000
            ));
            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key5),
                230000,
                231000
            ));
            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key6),
                240000,
                241000
            ));
            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key7),
                540000,
                541000
            ));
            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &empty_owned, &key8),
                550000,
                560000
            ));

            assert!(between(
                dbtest_fixture_size_owned_string_bounds(dbtest, &key3, &key5),
                110000,
                111000
            ));

            dbtest_fixture_test_compact_range_optional_owned_bounds(dbtest, 0, None, None);

            run += 1;
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_deletion_markers1() {
    let mut dbtest = DBTest::default();

    let foo = "foo".to_string();
    let v1 = "v1".to_string();
    assert!(dbtest.put(&foo, &v1).is_ok());
    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

    let last: i32 = bitcoinleveldb_cfg::MAX_MEM_COMPACT_LEVEL as i32;
    assert_eq!(1, dbtest.num_table_files_at_level(last)); // foo => v1 is now in last level

    // Place a table at level last-1 to prevent merging with preceding mutation
    let a = "a".to_string();
    let z = "z".to_string();
    let begin = "begin".to_string();
    let end = "end".to_string();

    assert!(dbtest.put(&a, &begin).is_ok());
    assert!(dbtest.put(&z, &end).is_ok());
    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

    assert_eq!(1, dbtest.num_table_files_at_level(last));
    assert_eq!(1, dbtest.num_table_files_at_level(last - 1));

    let v2 = "v2".to_string();
    assert!(dbtest.delete(&foo).is_ok());
    assert!(dbtest.put(&foo, &v2).is_ok());

    let foo_slice = Slice::from(&foo);
    assert_eq!("[ v2, DEL, v1 ]", dbtest.all_entries_for(&foo_slice));

    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok()); // Moves to level last-2
    assert_eq!("[ v2, DEL, v1 ]", dbtest.all_entries_for(&foo_slice));

    let z_slice = Slice::from(&z);
    unsafe {
        (*dbtest.dbfull()).test_compact_range(last - 2, null::<Slice>(), &z_slice as *const Slice);
    }

    // DEL eliminated, but v1 remains because we aren't compacting that level
    // (DEL can be eliminated because v2 hides v1).
    assert_eq!("[ v2, v1 ]", dbtest.all_entries_for(&foo_slice));

    unsafe {
        (*dbtest.dbfull()).test_compact_range(last - 1, null::<Slice>(), null::<Slice>());
    }

    // Merging last-1 w/ last, so we are the base level for "foo", so
    // DEL is removed.  (as is v1).
    assert_eq!("[ v2 ]", dbtest.all_entries_for(&foo_slice));
}

#[traced_test]
fn db_test_deletion_markers2() {
    let mut dbtest = DBTest::default();

    let foo = "foo".to_string();
    let v1 = "v1".to_string();
    assert!(dbtest.put(&foo, &v1).is_ok());
    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

    let last: i32 = bitcoinleveldb_cfg::MAX_MEM_COMPACT_LEVEL as i32;
    assert_eq!(1, dbtest.num_table_files_at_level(last)); // foo => v1 is now in last level

    // Place a table at level last-1 to prevent merging with preceding mutation
    let a = "a".to_string();
    let z = "z".to_string();
    let begin = "begin".to_string();
    let end = "end".to_string();

    assert!(dbtest.put(&a, &begin).is_ok());
    assert!(dbtest.put(&z, &end).is_ok());
    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

    assert_eq!(1, dbtest.num_table_files_at_level(last));
    assert_eq!(1, dbtest.num_table_files_at_level(last - 1));

    assert!(dbtest.delete(&foo).is_ok());

    let foo_slice = Slice::from(&foo);
    assert_eq!("[ DEL, v1 ]", dbtest.all_entries_for(&foo_slice));

    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok()); // Moves to level last-2
    assert_eq!("[ DEL, v1 ]", dbtest.all_entries_for(&foo_slice));

    unsafe {
        (*dbtest.dbfull()).test_compact_range(last - 2, null::<Slice>(), null::<Slice>());
    }

    // DEL kept: "last" file overlaps
    assert_eq!("[ DEL, v1 ]", dbtest.all_entries_for(&foo_slice));

    unsafe {
        (*dbtest.dbfull()).test_compact_range(last - 1, null::<Slice>(), null::<Slice>());
    }

    // Merging last-1 w/ last, so we are the base level for "foo", so
    // DEL is removed.  (as is v1).
    assert_eq!("[ ]", dbtest.all_entries_for(&foo_slice));
}

#[traced_test]
fn db_test_overlap_in_level0() {
    let mut body = |dbtest: &mut DBTest| {
        assert_eq!(2_i32, bitcoinleveldb_cfg::MAX_MEM_COMPACT_LEVEL as i32); // Fix test to match config

        // Fill levels 1 and 2 to disable the pushing of new memtables to levels > 0.
        let k100 = "100".to_string();
        let k999 = "999".to_string();
        let v100 = "v100".to_string();
        let v999 = "v999".to_string();

        assert!(dbtest.put(&k100, &v100).is_ok());
        assert!(dbtest.put(&k999, &v999).is_ok());
        assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

        assert!(dbtest.delete(&k100).is_ok());
        assert!(dbtest.delete(&k999).is_ok());
        assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

        assert_eq!("0,1,1", dbtest.files_per_level());

        // Make files spanning the following ranges in level-0:
        //  files[0]  200 .. 900
        //  files[1]  300 .. 500
        // Note that files are sorted by smallest key.
        let k200 = "200".to_string();
        let k300 = "300".to_string();
        let k500 = "500".to_string();
        let k600 = "600".to_string();
        let k900 = "900".to_string();

        let v200 = "v200".to_string();
        let v300 = "v300".to_string();
        let v500 = "v500".to_string();
        let v600 = "v600".to_string();
        let v900 = "v900".to_string();

        assert!(dbtest.put(&k300, &v300).is_ok());
        assert!(dbtest.put(&k500, &v500).is_ok());
        assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

        assert!(dbtest.put(&k200, &v200).is_ok());
        assert!(dbtest.put(&k600, &v600).is_ok());
        assert!(dbtest.put(&k900, &v900).is_ok());
        assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

        assert_eq!("2,1,1", dbtest.files_per_level());

        // Compact away the placeholder files we created initially
        unsafe {
            (*dbtest.dbfull()).test_compact_range(1, null::<Slice>(), null::<Slice>());
            (*dbtest.dbfull()).test_compact_range(2, null::<Slice>(), null::<Slice>());
        }
        assert_eq!("2", dbtest.files_per_level());

        // Do a memtable compaction.  Before bug-fix, the compaction would
        // not detect the overlap with level-0 files and would incorrectly place
        // the deletion in a deeper level.
        assert!(dbtest.delete(&k600).is_ok());
        assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());
        assert_eq!("3", dbtest.files_per_level());
        assert_eq!("NOT_FOUND", dbtest.get(&k600, None));
    };

    let mut dbtest = DBTest::default();
    loop {
        body(&mut dbtest);
        if !dbtest.change_options() {
            break;
        }
    }
}

#[traced_test]
fn db_test_l0_compaction_bug_issue44_a() {
    let mut dbtest = DBTest::default();

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "b", "v").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "b").is_ok());
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "a").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "a").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "a", "v").is_ok());

    dbtest.reopen(None);
    dbtest.reopen(None);
    assert_eq!("(a->v)", dbtest.contents());

    // Wait for compaction to finish
    delay_milliseconds(1000);
    assert_eq!("(a->v)", dbtest.contents());
}

#[traced_test]
fn db_test_l0_compaction_bug_issue44_b() {
    let mut dbtest = DBTest::default();

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "e").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "c", "cv").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    // Wait for compaction to finish
    delay_milliseconds(1000);

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "d", "dv").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "d").is_ok());
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "b").is_ok());

    dbtest.reopen(None);
    assert_eq!("(->)(c->cv)", dbtest.contents());

    // Wait for compaction to finish
    delay_milliseconds(1000);
    assert_eq!("(->)(c->cv)", dbtest.contents());
}

#[traced_test]
fn db_test_fflush_issue474() {
    static K_NUM: i32 = 100000;

    let mut dbtest = DBTest::default();
    let mut rnd = Random::new(bitcoinleveldbt_util::random_seed() as u32);

    let mut i: i32 = 0;
    while i < K_NUM {
        unsafe {
            fflush(null_mut());
        }

        let key_owned = dbtest_random_key((&mut rnd) as *mut Random);
        let value_owned = dbtest_random_string((&mut rnd) as *mut Random, 100);
        assert!(dbtest.put(&key_owned, &value_owned).is_ok());

        i += 1;
    }
}

#[traced_test]
fn db_test_comparator_check() {
    let mut dbtest = DBTest::default();

    let mut new_options = dbtest.current_options();
    new_options.set_comparator(Arc::new(DBTestComparatorNameMismatchProbe::default()));

    let s = dbtest.try_reopen((&mut new_options) as *mut Options);
    assert!(!s.is_ok());
    assert!(s.to_string().find("comparator").is_some());
}

#[traced_test]
fn db_test_custom_comparator() {
    let mut dbtest = DBTest::default();

    let mut new_options = dbtest.current_options();
    new_options.set_create_if_missing(true);
    new_options.set_comparator(Arc::new(DBTestBracketedIntegerComparator::default()));
    new_options.set_filter_policy(Arc::new(NullFilterPolicy::default())); // Cannot use bloom filters
    new_options.set_write_buffer_size(1000); // Compact more often
    dbtest.destroy_and_reopen(Some(&mut new_options));

    let key_10 = "[10]".to_string();
    let key_14_hex = "[0x14]".to_string();
    let value_ten = "ten".to_string();
    let value_twenty = "twenty".to_string();

    assert!(dbtest.put(&key_10, &value_ten).is_ok());
    assert!(dbtest.put(&key_14_hex, &value_twenty).is_ok());

    let mut i: i32 = 0;
    while i < 2 {
        let lookup_10 = "[10]".to_string();
        let lookup_10_hex = "[0xa]".to_string();
        let lookup_20 = "[20]".to_string();
        let lookup_20_hex = "[0x14]".to_string();
        let lookup_15 = "[15]".to_string();
        let lookup_15_hex = "[0xf]".to_string();

        assert_eq!("ten", dbtest.get(&lookup_10, None));
        assert_eq!("ten", dbtest.get(&lookup_10_hex, None));
        assert_eq!("twenty", dbtest.get(&lookup_20, None));
        assert_eq!("twenty", dbtest.get(&lookup_20_hex, None));
        assert_eq!("NOT_FOUND", dbtest.get(&lookup_15, None));
        assert_eq!("NOT_FOUND", dbtest.get(&lookup_15_hex, None));

        let compact_start = "[0]".to_string();
        let compact_limit = "[9999]".to_string();
        let compact_start_slice = Slice::from(&compact_start);
        let compact_limit_slice = Slice::from(&compact_limit);
        dbtest.compact(&compact_start_slice, &compact_limit_slice);

        i += 1;
    }

    let mut run: i32 = 0;
    while run < 2 {
        let mut j: i32 = 0;
        while j < 1000 {
            let key_buf = format!("[{}]", j * 10);
            assert!(dbtest.put(&key_buf, &key_buf).is_ok());
            j += 1;
        }

        let compact_start = "[0]".to_string();
        let compact_limit = "[1000000]".to_string();
        let compact_start_slice = Slice::from(&compact_start);
        let compact_limit_slice = Slice::from(&compact_limit);
        dbtest.compact(&compact_start_slice, &compact_limit_slice);

        run += 1;
    }
}

#[traced_test]
fn db_test_manual_compaction() {
    let mut dbtest = DBTest::default();

    assert_eq!(
        2_i32,
        bitcoinleveldb_cfg::MAX_MEM_COMPACT_LEVEL as i32
    ); // Need to update this test to match kMaxMemCompactLevel

    let p = "p".to_string();
    let q = "q".to_string();
    dbtest.make_tables(3, &p, &q);
    assert_eq!("1,1,1", dbtest.files_per_level());

    // Compaction range falls before files
    let empty = "".to_string();
    let c = "c".to_string();
    let empty_slice = Slice::from(&empty);
    let c_slice = Slice::from(&c);
    dbtest.compact(&empty_slice, &c_slice);
    assert_eq!("1,1,1", dbtest.files_per_level());

    // Compaction range falls after files
    let r = "r".to_string();
    let z = "z".to_string();
    let r_slice = Slice::from(&r);
    let z_slice = Slice::from(&z);
    dbtest.compact(&r_slice, &z_slice);
    assert_eq!("1,1,1", dbtest.files_per_level());

    // Compaction range overlaps files
    let p1 = "p1".to_string();
    let p9 = "p9".to_string();
    let p1_slice = Slice::from(&p1);
    let p9_slice = Slice::from(&p9);
    dbtest.compact(&p1_slice, &p9_slice);
    assert_eq!("0,0,1", dbtest.files_per_level());

    // Populate a different range
    let c_key = "c".to_string();
    let e_key = "e".to_string();
    dbtest.make_tables(3, &c_key, &e_key);
    assert_eq!("1,1,2", dbtest.files_per_level());

    // Compact just the new range
    let b = "b".to_string();
    let f = "f".to_string();
    let b_slice = Slice::from(&b);
    let f_slice = Slice::from(&f);
    dbtest.compact(&b_slice, &f_slice);
    assert_eq!("0,0,2", dbtest.files_per_level());

    // Compact all
    let a = "a".to_string();
    let z_all = "z".to_string();
    dbtest.make_tables(1, &a, &z_all);
    assert_eq!("0,1,2", dbtest.files_per_level());

    unsafe {
        (*dbtest.dbfull()).compact_range(null::<Slice>(), null::<Slice>());
    }

    assert_eq!("0,0,1", dbtest.files_per_level());
}

#[traced_test]
fn db_test_open_options() {
    let mut dbname = dbtest_fixture_tmp_dbname_with_suffix("/db_options_test");
    let _ = destroy_db(&dbname, &Options::default());

    let mut open_db = |options: &Options| -> (Status, Option<*mut dyn DB>) {
        let mut opener = DBImpl::new(options, &dbname);
        let mut slot: MaybeUninit<*mut dyn DB> = MaybeUninit::uninit();
        let s = opener.open(options, &dbname, slot.as_mut_ptr());
        let db_ptr = match s.is_ok() {
            true => Some(unsafe { slot.assume_init() }),
            false => None,
        };
        (s, db_ptr)
    };

    // Does not exist, and create_if_missing == false: error
    let mut opts = Options::default();
    opts.set_create_if_missing(false);

    let (s_missing, db_missing) = open_db(&opts);
    assert!(s_missing.to_string().find("does not exist").is_some());
    assert!(db_missing.is_none());

    // Does not exist, and create_if_missing == true: OK
    opts.set_create_if_missing(true);
    let (s_create, db_created) = open_db(&opts);
    assert!(s_create.is_ok());
    assert!(db_created.is_some());

    match db_created {
        Some(ptr) => unsafe { drop(Box::from_raw(ptr)); },
        None => {}
    }

    // Does exist, and error_if_exists == true: error
    opts.set_create_if_missing(false);
    opts.set_error_if_exists(true);
    let (s_exists_error, db_exists_error) = open_db(&opts);
    assert!(s_exists_error.to_string().find("exists").is_some());
    assert!(db_exists_error.is_none());

    // Does exist, and error_if_exists == false: OK
    opts.set_create_if_missing(true);
    opts.set_error_if_exists(false);
    let (s_reopen, db_reopen) = open_db(&opts);
    assert!(s_reopen.is_ok());
    assert!(db_reopen.is_some());

    match db_reopen {
        Some(ptr) => unsafe { drop(Box::from_raw(ptr)); },
        None => {}
    }
}

#[traced_test]
fn db_test_destroy_empty_dir() {
    let mut dbname = dbtest_fixture_tmp_dbname_with_suffix("/db_empty_dir");

    let env: Rc<RefCell<TestEnv>> =
        Rc::new(RefCell::new(TestEnv::new(PosixEnv::shared())));

    let _ = env.borrow_mut().delete_dir(&dbname);
    assert!(!env.borrow_mut().file_exists(&dbname));

    let env_for_options: Rc<RefCell<dyn Env>> = env.clone();
    let opts = Options::with_env(env_for_options);

    assert!(env.borrow_mut().create_dir(&dbname).is_ok());
    assert!(env.borrow_mut().file_exists(&dbname));

    let mut children: Vec<String> = Vec::new();
    assert!(env.borrow_mut().get_children(&dbname, &mut children as *mut Vec<String>).is_ok());

    // The stock Env's do not filter out '.' and '..' special files.
    assert_eq!(2, children.len());

    assert!(destroy_db(&dbname, &opts).is_ok());
    assert!(!env.borrow_mut().file_exists(&dbname));

    // Should also be destroyed if Env is filtering out dot files.
    env.borrow_mut().set_ignore_dot_files(true);

    assert!(env.borrow_mut().create_dir(&dbname).is_ok());
    assert!(env.borrow_mut().file_exists(&dbname));

    children.clear();
    assert!(env.borrow_mut().get_children(&dbname, &mut children as *mut Vec<String>).is_ok());
    assert_eq!(0, children.len());

    assert!(destroy_db(&dbname, &opts).is_ok());
    assert!(!env.borrow_mut().file_exists(&dbname));
}

#[traced_test]
fn db_test_destroy_opendb() {
    let mut dbname = dbtest_fixture_tmp_dbname_with_suffix("/open_db_dir");

    let env = PosixEnv::shared();
    let _ = env.borrow_mut().delete_dir(&dbname);
    assert!(!env.borrow_mut().file_exists(&dbname));

    let mut opts = Options::default();
    opts.set_create_if_missing(true);

    let mut opener = DBImpl::new(&opts, &dbname);
    let mut slot: MaybeUninit<*mut dyn DB> = MaybeUninit::uninit();
    let s_open = opener.open(&opts, &dbname, slot.as_mut_ptr());
    assert!(s_open.is_ok());

    let db_ptr = unsafe { slot.assume_init() };

    // Must fail to destroy an open db.
    assert!(env.borrow_mut().file_exists(&dbname));
    let destroy_while_open = destroy_db(&dbname, &Options::default());
    assert!(!destroy_while_open.is_ok());
    assert!(env.borrow_mut().file_exists(&dbname));

    unsafe {
        drop(Box::from_raw(db_ptr));
    }

    // Should succeed destroying a closed db.
    let destroy_closed = destroy_db(&dbname, &Options::default());
    assert!(destroy_closed.is_ok());
    assert!(!env.borrow_mut().file_exists(&dbname));
}

#[traced_test]
fn db_test_locking() {
    let mut dbname = dbtest_fixture_tmp_dbname_with_suffix("/locking_test");
    let _ = destroy_db(&dbname, &Options::default());

    let mut opts = Options::default();
    opts.set_create_if_missing(true);

    let mut opener1 = DBImpl::new(&opts, &dbname);
    let mut slot1: MaybeUninit<*mut dyn DB> = MaybeUninit::uninit();
    let s_first_open = opener1.open(&opts, &dbname, slot1.as_mut_ptr());
    assert!(s_first_open.is_ok());

    let db_ptr1 = unsafe { slot1.assume_init() };

    let mut opener2 = DBImpl::new(&opts, &dbname);
    let mut slot2: MaybeUninit<*mut dyn DB> = MaybeUninit::uninit();
    let s_second_open = opener2.open(&opts, &dbname, slot2.as_mut_ptr());
    assert!(!s_second_open.is_ok());

    unsafe {
        drop(Box::from_raw(db_ptr1));
    }
}

/**
  | Check that number of files does not grow
  | when we are out of space
  |
  */
#[traced_test]
fn db_test_no_space() {
    let mut dbtest = DBTest::default();
    let mut options = dbtest.current_options();
    dbtest.reopen(Some(&mut options));

    let foo = "foo".to_string();
    let v1 = "v1".to_string();
    let a = "a".to_string();
    let z = "z".to_string();

    assert!(dbtest.put(&foo, &v1).is_ok());
    assert_eq!("v1", dbtest.get(&foo, None));

    let a_slice = Slice::from(&a);
    let z_slice = Slice::from(&z);
    dbtest.compact(&a_slice, &z_slice);

    let num_files = dbtest.count_files();

    // Force out-of-space errors.
    unsafe {
        (*dbtest.special_env())
            .no_space()
            .store(true, atomic::Ordering::Release);
    }

    let mut i: i32 = 0;
    while i < 10 {
        let mut level: i32 = 0;
        while level < (bitcoinleveldb_cfg::NUM_LEVELS as i32) - 1 {
            unsafe {
                (*dbtest.dbfull()).test_compact_range(level, null::<Slice>(), null::<Slice>());
            }
            level += 1;
        }
        i += 1;
    }

    unsafe {
        (*dbtest.special_env())
            .no_space()
            .store(false, atomic::Ordering::Release);
    }

    assert!(dbtest.count_files() < num_files + 3);
}

#[traced_test]
fn db_test_non_writable_file_system() {
    let mut dbtest = DBTest::default();
    let mut options = dbtest.current_options();
    options.set_write_buffer_size(1000);
    dbtest.reopen(Some(&mut options));

    let foo = "foo".to_string();
    let v1 = "v1".to_string();
    assert!(dbtest.put(&foo, &v1).is_ok());

    // Force errors for new files.
    unsafe {
        (*dbtest.special_env())
            .non_writable()
            .store(true, atomic::Ordering::Release);
    }

    let big = "x".repeat(100000);
    let mut errors: i32 = 0;
    let mut i: i32 = 0;
    while i < 20 {
        eprintln!("iter {}; errors {}", i, errors);
        if !dbtest.put(&foo, &big).is_ok() {
            errors += 1;
            delay_milliseconds(100);
        }
        i += 1;
    }

    assert!(errors > 0);

    unsafe {
        (*dbtest.special_env())
            .non_writable()
            .store(false, atomic::Ordering::Release);
    }
}

#[traced_test]
fn db_test_write_sync_error() {
    // Check that log sync errors cause the DB to disallow future writes.
    let mut dbtest = DBTest::default();
    let mut options = dbtest.current_options();
    dbtest.reopen(Some(&mut options));

    // (a) Cause log sync calls to fail
    unsafe {
        (*dbtest.special_env())
            .data_sync_error()
            .store(true, atomic::Ordering::Release);
    }

    let k1 = "k1".to_string();
    let k2 = "k2".to_string();
    let k3 = "k3".to_string();
    let v1 = "v1".to_string();
    let v2 = "v2".to_string();
    let v3 = "v3".to_string();

    let k1s = Slice::from(&k1);
    let k2s = Slice::from(&k2);
    let k3s = Slice::from(&k3);
    let v1s = Slice::from(&v1);
    let v2s = Slice::from(&v2);
    let v3s = Slice::from(&v3);

    // (b) Normal write should succeed
    let mut w = WriteOptions::default();
    assert!(unsafe { (*dbtest.dbfull()).put(&w, &k1s, &v1s) }.is_ok());
    assert_eq!("v1", dbtest.get(&k1, None));

    // (c) Do a sync write; should fail
    w.set_sync(true);
    assert!(!unsafe { (*dbtest.dbfull()).put(&w, &k2s, &v2s) }.is_ok());
    assert_eq!("v1", dbtest.get(&k1, None));
    assert_eq!("NOT_FOUND", dbtest.get(&k2, None));

    // (d) make sync behave normally
    unsafe {
        (*dbtest.special_env())
            .data_sync_error()
            .store(false, atomic::Ordering::Release);
    }

    // (e) Do a non-sync write; should fail
    w.set_sync(false);
    assert!(!unsafe { (*dbtest.dbfull()).put(&w, &k3s, &v3s) }.is_ok());
    assert_eq!("v1", dbtest.get(&k1, None));
    assert_eq!("NOT_FOUND", dbtest.get(&k2, None));
    assert_eq!("NOT_FOUND", dbtest.get(&k3, None));
}

#[traced_test]
fn db_test_manifest_write_error() {
    // Test for the following problem:
    // (a) Compaction produces file F
    // (b) Log record containing F is written to MANIFEST file, but Sync() fails
    // (c) GC deletes F
    // (d) After reopening DB, reads fail since deleted F is named in log record

    let mut dbtest = DBTest::default();
    let last = bitcoinleveldb_cfg::MAX_MEM_COMPACT_LEVEL as i32;

    // We iterate twice.  In the second iteration, everything is the
    // same except the log record never makes it to the MANIFEST file.
    let mut iter: i32 = 0;
    while iter < 2 {
        unsafe {
            (*dbtest.special_env())
                .manifest_sync_error()
                .store(false, atomic::Ordering::Release);
            (*dbtest.special_env())
                .manifest_write_error()
                .store(false, atomic::Ordering::Release);
        }

        // Insert foo=>bar mapping
        let mut options = dbtest.current_options();
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);
        dbtest.destroy_and_reopen(Some(&mut options));

        let foo = "foo".to_string();
        let bar = "bar".to_string();

        assert!(dbtest.put(&foo, &bar).is_ok());
        assert_eq!("bar", dbtest.get(&foo, None));

        // Memtable compaction (will succeed)
        assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());
        assert_eq!("bar", dbtest.get(&foo, None));
        assert_eq!(1, dbtest.num_table_files_at_level(last)); // foo=>bar is now in last level

        // Merging compaction (will fail)
        match iter {
            0 => unsafe {
                (*dbtest.special_env())
                    .manifest_sync_error()
                    .store(true, atomic::Ordering::Release);
            },
            1 => unsafe {
                (*dbtest.special_env())
                    .manifest_write_error()
                    .store(true, atomic::Ordering::Release);
            },
            _ => {
                assert!(false);
            }
        }

        unsafe {
            (*dbtest.dbfull()).test_compact_range(last, null::<Slice>(), null::<Slice>());
        }
        assert_eq!("bar", dbtest.get(&foo, None));

        // Recovery: should not lose data
        unsafe {
            (*dbtest.special_env())
                .manifest_sync_error()
                .store(false, atomic::Ordering::Release);
            (*dbtest.special_env())
                .manifest_write_error()
                .store(false, atomic::Ordering::Release);
        }

        dbtest.reopen(Some(&mut options));
        assert_eq!("bar", dbtest.get(&foo, None));

        iter += 1;
    }
}

#[traced_test]
fn db_test_missing_sst_file() {
    let mut dbtest = DBTest::default();

    assert!(dbtest_fixture_put_literal(&mut dbtest, "foo", "bar").is_ok());
    assert_eq!("bar", dbtest_fixture_get_literal(&mut dbtest, "foo"));

    // Dump the memtable to disk.
    assert!(dbtest_fixture_test_compact_memtable_status(&mut dbtest).is_ok());
    assert_eq!("bar", dbtest_fixture_get_literal(&mut dbtest, "foo"));

    dbtest.close();
    assert!(dbtest.delete_an_sst_file());

    let mut options = dbtest.current_options();
    options.set_paranoid_checks(true);

    let s = dbtest.try_reopen((&mut options) as *mut Options);
    assert!(!s.is_ok());
    assert!(s.to_string().find("issing").is_some());
}

#[traced_test]
fn db_test_still_readsst() {
    let mut dbtest = DBTest::default();

    assert!(dbtest_fixture_put_literal(&mut dbtest, "foo", "bar").is_ok());
    assert_eq!("bar", dbtest_fixture_get_literal(&mut dbtest, "foo"));

    assert!(dbtest_fixture_test_compact_memtable_status(&mut dbtest).is_ok());
    assert_eq!("bar", dbtest_fixture_get_literal(&mut dbtest, "foo"));

    dbtest.close();
    assert!(dbtest.rename_ldb_tosst() > 0);

    let mut options = dbtest.current_options();
    options.set_paranoid_checks(true);

    let s = dbtest.try_reopen((&mut options) as *mut Options);
    assert!(s.is_ok());
    assert_eq!("bar", dbtest_fixture_get_literal(&mut dbtest, "foo"));
}

#[traced_test]
fn db_test_bloom_filter() {
    let mut dbtest = DBTest::default();

    unsafe {
        (*dbtest.special_env()).set_count_random_reads(true);
    }

    let mut options = dbtest.current_options();
    options.set_block_cache(new_lru_cache(0)); // Prevent cache hits

    let bloom_filter_policy: Arc<dyn FilterPolicy> =
        Arc::from(new_bloom_filter_policy(10));
    options.set_filter_policy(bloom_filter_policy);

    dbtest.reopen(Some(&mut options));

    // Populate multiple layers
    const N: i32 = 10000;
    let mut i: i32 = 0;
    while i < N {
        let key_owned = crate::key(i);
        assert!(dbtest.put(&key_owned, &key_owned).is_ok());
        i += 1;
    }

    let a = "a".to_string();
    let z = "z".to_string();
    let a_slice = Slice::from(&a);
    let z_slice = Slice::from(&z);
    dbtest.compact(&a_slice, &z_slice);

    let mut j: i32 = 0;
    while j < N {
        let key_owned = crate::key(j);
        assert!(dbtest.put(&key_owned, &key_owned).is_ok());
        j += 100;
    }

    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

    // Prevent auto compactions triggered by seeks
    unsafe {
        (*dbtest.special_env())
            .delay_data_sync()
            .store(true, atomic::Ordering::Release);
    }

    // Lookup present keys.  Should rarely read from small sstable.
    unsafe {
        (*dbtest.special_env()).random_read_counter().reset();
    }

    let mut present_i: i32 = 0;
    while present_i < N {
        let key_owned = crate::key(present_i);
        assert_eq!(key_owned, dbtest.get(&key_owned, None));
        present_i += 1;
    }

    let reads_present = unsafe { (*dbtest.special_env()).random_read_counter().read() };
    eprintln!("{} present => {} reads", N, reads_present);
    assert!(reads_present >= N);
    assert!(reads_present <= N + 2 * N / 100);

    // Lookup present keys.  Should rarely read from either sstable.
    unsafe {
        (*dbtest.special_env()).random_read_counter().reset();
    }

    let mut missing_i: i32 = 0;
    while missing_i < N {
        let missing_key = format!("{}.missing", crate::key(missing_i));
        assert_eq!("NOT_FOUND", dbtest.get(&missing_key, None));
        missing_i += 1;
    }

    let reads_missing = unsafe { (*dbtest.special_env()).random_read_counter().read() };
    eprintln!("{} missing => {} reads", N, reads_missing);
    assert!(reads_missing <= 3 * N / 100);

    unsafe {
        (*dbtest.special_env())
            .delay_data_sync()
            .store(false, atomic::Ordering::Release);
    }

    dbtest.close();

    let block_cache_ptr = *options.block_cache();
    if !block_cache_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(block_cache_ptr));
        }
        options.set_block_cache(null_mut::<Cache>());
    }
}
