
use bitcoinleveldb_comparator::bytewise_comparator;
use bitcoinleveldb_key::{InternalKeyComparator, LookupKey, ValueType};
use bitcoinleveldb_memtable::MemTable;
use bitcoinleveldb_slice::Slice;
use bitcoinleveldb_status::Status;

fn table() -> MemTable {
    let comparator = unsafe { InternalKeyComparator::new(bytewise_comparator()) };
    MemTable::new(&comparator)
}

#[test]
fn binary_point_reads_preserve_values_versions_deletions_and_missing_outputs() {
    let mut table = table();
    let key = [0x00, 0xff, 0x80];
    let old: Vec<u8> = (0u8..=255).collect();
    let new: Vec<u8> = old.iter().rev().copied().collect();
    table.add(2, ValueType::TypeValue, &Slice::from(&key[..]), &Slice::from(old.as_slice()));
    table.add(9, ValueType::TypeValue, &Slice::from(&key[..]), &Slice::from(new.as_slice()));
    table.add(10, ValueType::TypeDeletion, &Slice::from(&key[..]), &Slice::default());
    for (sequence, expected) in [(2, old.as_slice()), (8, old.as_slice()), (9, new.as_slice())] {
        let lookup = LookupKey::new(&Slice::from(&key[..]), sequence);
        let mut out = vec![0x7f]; let mut status = Status::ok();
        assert!(table.get_bytes(&lookup, &mut out, &mut status));
        assert!(status.is_ok()); assert_eq!(out.as_slice(), expected);
    }
    let sentinel = vec![0xaa, 0x00, 0xff];
    let mut out = sentinel.clone(); let mut status = Status::ok();
    assert!(table.get_bytes(&LookupKey::new(&Slice::from(&key[..]), 10), &mut out, &mut status));
    assert!(status.is_not_found()); assert_eq!(out, sentinel);
    status = Status::ok();
    assert!(!table.get_bytes(&LookupKey::new(&Slice::from(&b"missing"[..]), 10), &mut out, &mut status));
    assert!(status.is_ok()); assert_eq!(out, sentinel);
}

#[test]
fn checked_memtable_text_adapter_preserves_unicode_status_and_legacy_tombstones() {
    let mut table = table();
    let key = Slice::from("text");
    table.add(1, ValueType::TypeValue, &key, &Slice::from("é\0水"));
    let mut out = String::from("sentinel");
    let mut status = Status::io_error(&Slice::from("existing caller status"), None);
    let before_status = status.to_string();
    assert!(table.get(&LookupKey::new(&key, 1), &mut out, &mut status));
    assert_eq!(out, "é\0水"); assert_eq!(status.to_string(), before_status);
    table.add(2, ValueType::TypeValue, &key, &Slice::from(&[0xff, 0x00, 0x80][..]));
    assert!(table.get(&LookupKey::new(&key, 2), &mut out, &mut status));
    assert!(status.is_corruption()); assert_eq!(out, "é\0水");
    table.add(3, ValueType::TypeValue, &key, &Slice::default());
    status = Status::ok();
    assert!(table.get(&LookupKey::new(&key, 3), &mut out, &mut status));
    assert!(status.is_ok()); assert!(out.is_empty());
    out = String::from("must clear");
    table.add(4, ValueType::TypeDeletion, &key, &Slice::default());
    assert!(table.get(&LookupKey::new(&key, 4), &mut out, &mut status));
    assert!(status.is_not_found()); assert!(out.is_empty());
}
