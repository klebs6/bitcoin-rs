// ---------------- [ File: bitcoinleveldb-writebatch/tests/writebatch.rs ]
use bitcoinleveldb_writebatch::*;
use bitcoinleveldb_memtable::*;

#[traced_test]
fn mem_table_test_simple() {

    info!("mem_table_test_simple: starting");

    unsafe {
        // --------------------------------------------------------------------
        // InternalKeyComparator cmp(BytewiseComparator());
        // --------------------------------------------------------------------
        let user_cmp_ptr = bytewise_comparator();
        let icmp = InternalKeyComparator::new(user_cmp_ptr);

        // --------------------------------------------------------------------
        // MemTable* memtable = new MemTable(cmp);
        // memtable->Ref();
        // --------------------------------------------------------------------
        let memtable_box = Box::new(MemTable::new(&icmp));
        let memtable_raw: *mut MemTable = Box::into_raw(memtable_box);
        (*memtable_raw).ref_();
        info!("mem_table_test_simple: created memtable @ {:?}", memtable_raw);

        // --------------------------------------------------------------------
        // WriteBatch batch;
        // WriteBatchInternal::SetSequence(&batch, 100);
        // --------------------------------------------------------------------
        let mut batch = WriteBatch::default();
        WriteBatchInternal::set_sequence(&mut batch, 100);

        // --------------------------------------------------------------------
        // batch.Put("k1","v1");
        // batch.Put("k2","v2");
        // batch.Put("k3","v3");
        // batch.Put("largekey","vlarge");
        // --------------------------------------------------------------------
        batch.put("k1".as_bytes(), "v1".as_bytes());
        batch.put("k2".as_bytes(), "v2".as_bytes());
        batch.put("k3".as_bytes(), "v3".as_bytes());
        batch.put("largekey".as_bytes(), "vlarge".as_bytes());

        // --------------------------------------------------------------------
        // ASSERT_TRUE(WriteBatchInternal::InsertInto(&batch, memtable).ok());
        // --------------------------------------------------------------------
        let status = WriteBatchInternal::insert_into(&batch, memtable_raw);
        assert!(
            status.is_ok(),
            "WriteBatchInternal::InsertInto returned non-OK: {:?}",
            status
        );

        // --------------------------------------------------------------------
        // Iterator* iter = memtable->NewIterator();
        // iter->SeekToFirst();
        // while (iter->Valid()) {
        //   fprintf(stderr, "key_: '%s' -> '%s'\n", iter->key().ToString().c_str(),
        //           iter->value().ToString().c_str());
        //   iter->Next();
        // }
        // delete iter;
        // --------------------------------------------------------------------
        let iter_raw = (*memtable_raw).new_iterator();
        let mut iter_box: Box<LevelDBIterator> = Box::from_raw(iter_raw);

        iter_box.seek_to_first();

        while iter_box.valid() {
            let key_slice   = iter_box.key();
            let value_slice = iter_box.value();

            eprintln!(
                "key_: '{}' -> '{}'",
                key_slice.to_string(),
                value_slice.to_string()
            );

            iter_box.next();
        }

        // iterator Box dropped automatically here (equivalent of delete iter)

        // --------------------------------------------------------------------
        // memtable->Unref();
        // --------------------------------------------------------------------
        info!("mem_table_test_simple: calling Unref() on memtable");
        (*memtable_raw).unref();

        // DO NOT USE memtable_raw after this point —
        // Unref may delete the MemTable, exactly like C++.
    }

    info!("mem_table_test_simple: completed");
}
