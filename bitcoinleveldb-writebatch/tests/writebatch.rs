// ---------------- [ File: bitcoinleveldb-writebatch/tests/writebatch.rs ]
use bitcoin_imports::*;
use bitcoinleveldb_batch::{write_batch_internal, WriteBatch};
use bitcoinleveldb_comparator::bytewise_comparator;
use bitcoinleveldb_iterator::LevelDBIterator;
use bitcoinleveldb_iteratorinner::*;
use bitcoinleveldb_key::{extract_user_key, InternalKeyComparator};
use bitcoinleveldb_memtable::MemTable;
use bitcoinleveldb_slice::Slice;

#[test]
fn mem_table_test_simple() {
    info!("mem_table_test_simple: starting");

    unsafe {
        let user_cmp_ptr = bytewise_comparator();
        let icmp = InternalKeyComparator::new(user_cmp_ptr);
        let memtable_raw = Box::into_raw(Box::new(MemTable::new(&icmp)));
        (*memtable_raw).ref_();

        let entries: &[(&[u8], &[u8])] = &[
            (b"k1", b"v1"),
            (b"k2", b"v2"),
            (b"k3", b"v3"),
            (b"largekey", b"vlarge"),
        ];
        let mut batch = WriteBatch::new();
        write_batch_internal::set_sequence(&mut batch, 100);
        for &(key, value) in entries {
            batch.put(&Slice::from(key), &Slice::from(value));
        }
        assert_eq!(write_batch_internal::count(&batch), entries.len() as i32);

        let status = write_batch_internal::insert_into(&batch, memtable_raw);
        assert!(status.is_ok(), "batch insertion failed: {:?}", status);

        let iter_raw = (*memtable_raw).new_iterator();
        assert!(!iter_raw.is_null(), "memtable must return an iterator");
        let mut iter_box: Box<LevelDBIterator> = Box::from_raw(iter_raw);
        iter_box.seek_to_first();
        let mut observed: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        while iter_box.valid() {
            let internal_key = iter_box.key();
            let user_key = extract_user_key(&internal_key);
            let value = iter_box.value();
            observed.push((user_key.as_bytes().to_vec(), value.as_bytes().to_vec()));
            iter_box.next();
        }
        assert!(iter_box.status().is_ok(), "memtable iteration must finish successfully");
        let expected: Vec<(Vec<u8>, Vec<u8>)> = entries.iter()
            .map(|(key, value)| (key.to_vec(), value.to_vec()))
            .collect();
        assert_eq!(observed, expected, "all inserted keys and values must round trip in order");

        // The iterator borrows the memtable's storage. Release it before the
        // final unref can destroy the memtable.
        drop(iter_box);
        (*memtable_raw).unref();
    }

    info!("mem_table_test_simple: completed");
}
