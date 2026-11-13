use bitcoinleveldb_comparator::bytewise_comparator;
use bitcoinleveldb_iterator::LevelDBIterator;
use bitcoinleveldb_iteratorinner::{
    LevelDBIteratorKey, LevelDBIteratorNext, LevelDBIteratorPrev,
    LevelDBIteratorSeek, LevelDBIteratorSeekToFirst, LevelDBIteratorSeekToLast,
    LevelDBIteratorStatus, LevelDBIteratorValid, LevelDBIteratorValue,
};
use bitcoinleveldb_key::{InternalKeyComparator, MAX_SEQUENCE_NUMBER, ValueType};
use bitcoinleveldb_memtable::{encode_key, MemTable};
use bitcoinleveldb_slice::Slice;

#[test]
fn byte_seek_scratch_preserves_literal_varint_boundaries_and_payloads() {
    let cases: &[(usize, &[u8])] = &[
        (0, &[0x00]),
        (1, &[0x01]),
        (127, &[0x7f]),
        (128, &[0x80, 0x01]),
        (16383, &[0xff, 0x7f]),
        (16384, &[0x80, 0x80, 0x01]),
    ];
    let mut scratch = Vec::new();
    for &(length, prefix) in cases {
        let payload: Vec<u8> = (0..length).map(|i| (i % 256) as u8).collect();
        let pointer = encode_key(&mut scratch, &Slice::from(payload.as_slice()));
        assert_eq!(pointer, scratch.as_ptr());
        assert_eq!(&scratch[..prefix.len()], prefix);
        assert_eq!(&scratch[prefix.len()..], payload.as_slice());
        assert_eq!(scratch.len(), prefix.len() + length);
    }
}

#[test]
fn byte_seek_scratch_copies_a_view_of_its_previous_allocation() {
    let mut scratch = vec![0xaa, 0xff, 0x80, 0x00, 0xbb];
    let target = Slice::from(&scratch[1..4]);
    let pointer = encode_key(&mut scratch, &target);
    assert_eq!(pointer, scratch.as_ptr());
    assert_eq!(scratch, vec![0x03, 0xff, 0x80, 0x00]);
}

// This oracle uses the format's fixed little-endian tag directly, not the
// production append/parse helpers being exercised by the table.
fn wire_key(user: &[u8], sequence: u64, ty: ValueType) -> Vec<u8> {
    let mut result = user.to_vec();
    result.extend_from_slice(&((sequence << 8) | ty as u64).to_le_bytes());
    result
}

#[test]
fn binary_memtable_iteration_and_seek_preserve_keys_values_and_record_order() {
    let comparator = unsafe { InternalKeyComparator::new(bytewise_comparator()) };
    let mut table = MemTable::new(&comparator);
    let a = [0x00, 0xff];
    let b = [0x80, 0x00];
    let c = [0xff, 0x00];
    let a_value = [0x00, 0xff];
    let b_value = [0xff, 0x80];
    let c_value = [0xfe, 0x00];
    table.add(MAX_SEQUENCE_NUMBER, ValueType::TypeValue,
        &Slice::from(&c[..]), &Slice::from(&c_value[..]));
    table.add(9, ValueType::TypeValue, &Slice::from(&b[..]), &Slice::from(&b_value[..]));
    table.add(2, ValueType::TypeValue, &Slice::from(&a[..]), &Slice::from(&a_value[..]));
    table.add(7, ValueType::TypeDeletion, &Slice::from(&b[..]), &Slice::from(&[][..]));

    let expected = vec![
        (wire_key(&a, 2, ValueType::TypeValue), a_value.to_vec()),
        (wire_key(&b, 9, ValueType::TypeValue), b_value.to_vec()),
        (wire_key(&b, 7, ValueType::TypeDeletion), vec![]),
        (wire_key(&c, MAX_SEQUENCE_NUMBER, ValueType::TypeValue), c_value.to_vec()),
    ];
    let raw = table.new_iterator();
    assert!(!raw.is_null());
    // Iterator ownership is reclaimed before the table's borrowed storage.
    let mut iter: Box<LevelDBIterator> = unsafe { Box::from_raw(raw) };
    let mut forward = Vec::new();
    iter.seek_to_first();
    while iter.valid() {
        forward.push((iter.key().as_bytes().to_vec(), iter.value().as_bytes().to_vec()));
        iter.next();
    }
    assert_eq!(forward, expected);
    let mut reverse = Vec::new();
    iter.seek_to_last();
    while iter.valid() {
        reverse.push((iter.key().as_bytes().to_vec(), iter.value().as_bytes().to_vec()));
        iter.prev();
    }
    assert_eq!(reverse, expected.iter().rev().cloned().collect::<Vec<_>>());

    let target = wire_key(&b, 8, ValueType::TypeValue);
    iter.seek(&Slice::from(target.as_slice()));
    assert!(iter.valid());
    assert_eq!(iter.key().as_bytes(), expected[2].0.as_slice());
    assert!(iter.value().as_bytes().is_empty());
    iter.next();
    assert_eq!(iter.key().as_bytes(), expected[3].0.as_slice());

    let between = wire_key(&[0x90], MAX_SEQUENCE_NUMBER, ValueType::TypeValue);
    iter.seek(&Slice::from(between.as_slice()));
    assert!(iter.valid());
    assert_eq!(iter.key().as_bytes(), expected[3].0.as_slice());
    assert_eq!(iter.value().as_bytes(), c_value.as_slice());
    assert!(iter.status().is_ok());
    drop(iter);
}
