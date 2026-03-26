crate::ix!();

#[traced_test]
fn db_test_probe_single_key_history_survives_two_memtable_flushes() {
    let mut dbtest = DBTest::default();

    let foo = "foo".to_string();
    let v1 = "v1".to_string();
    let v2 = "v2".to_string();

    assert!(dbtest.put(&foo, &v1).is_ok());
    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

    assert!(dbtest.put(&foo, &v2).is_ok());

    let foo_slice = Slice::from(&foo);
    assert_eq!("[ v2, v1 ]", dbtest.all_entries_for(&foo_slice));

    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

    // This is the key probe: once both versions live behind the table path,
    // internal seek/order must still expose the newest then the older value.
    assert_eq!("[ v2, v1 ]", dbtest.all_entries_for(&foo_slice));
    assert_eq!("v2", dbtest.get(&foo, None));
}

#[traced_test]
fn db_test_probe_delete_marker_history_survives_flush() {
    let mut dbtest = DBTest::default();

    let foo = "foo".to_string();
    let v1 = "v1".to_string();

    assert!(dbtest.put(&foo, &v1).is_ok());
    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

    assert!(dbtest.delete(&foo).is_ok());

    let foo_slice = Slice::from(&foo);
    assert_eq!("[ DEL, v1 ]", dbtest.all_entries_for(&foo_slice));
    assert_eq!("NOT_FOUND", dbtest.get(&foo, None));

    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());

    // Once the tombstone is table-backed, order and visibility must remain intact.
    assert_eq!("[ DEL, v1 ]", dbtest.all_entries_for(&foo_slice));
    assert_eq!("NOT_FOUND", dbtest.get(&foo, None));
}

#[traced_test]
fn db_test_probe_custom_comparator_still_works_after_memtable_flush() {
    let mut dbtest = DBTest::default();

    let mut options = dbtest.current_options();
    options.set_create_if_missing(true);
    options.set_error_if_exists(false);
    options.set_comparator(Arc::new(DBTestBracketedIntegerComparator::default()));
    options.set_filter_policy(Arc::new(NullFilterPolicy::default()));
    dbtest.destroy_and_reopen(Some(&mut options));

    let k10 = "[10]".to_string();
    let k20_hex = "[0x14]".to_string();
    let ten = "ten".to_string();
    let twenty = "twenty".to_string();

    assert!(dbtest.put(&k10, &ten).is_ok());
    assert!(dbtest.put(&k20_hex, &twenty).is_ok());

    let q10_hex = "[0xa]".to_string();
    let q20 = "[20]".to_string();

    // Control: memtable path
    assert_eq!("ten", dbtest.get(&q10_hex, None));
    assert_eq!("twenty", dbtest.get(&q20, None));

    // Probe: table path after flush
    assert!(unsafe { (*dbtest.dbfull()).test_compact_mem_table() }.is_ok());
    assert_eq!("ten", dbtest.get(&q10_hex, None));
    assert_eq!("twenty", dbtest.get(&q20, None));

    // Probe again across reopen/recovery
    dbtest.reopen(Some(&mut options));
    assert_eq!("ten", dbtest.get(&q10_hex, None));
    assert_eq!("twenty", dbtest.get(&q20, None));
}

