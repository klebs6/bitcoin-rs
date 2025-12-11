// ---------------- [ File: bitcoinleveldb-comparator/tests/comparator.rs ]
use bitcoinleveldb_comparator::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn test_compare_basic() {
    let cmp = BytewiseComparatorImpl::default();
    let s1 = Slice::from_ptr_len(b"abc".as_ptr(), 3);
    let s2 = Slice::from_ptr_len(b"abd".as_ptr(), 3);
    let s3 = Slice::from_ptr_len(b"abc".as_ptr(), 3);

    assert!(cmp.compare(&s1, &s2) < 0, "abc < abd");
    assert!(cmp.compare(&s2, &s1) > 0, "abd > abc");
    assert_eq!(cmp.compare(&s1, &s3), 0, "abc == abc");
}

#[traced_test]
fn test_find_shortest_separator() {
    let cmp = BytewiseComparatorImpl::default();
    let limit = b"abcxyz".to_vec();

    // If prefix matches entirely, do nothing
    let mut start1 = b"abc".to_vec();
    cmp.find_shortest_separator(&mut start1, &limit);
    assert_eq!(start1, b"abc");

    // Diverge at index=2
    let mut start2 = b"abaxxx".to_vec(); // 'a' < 'c'
    cmp.find_shortest_separator(&mut start2, &limit);
    // Expect "abb" => 'a'(0x61), 'b'(0x62), second char is 'a'(0x61) < 'c'(0x63),
    // so 0x61 + 1=0x62 => 'b'
    // => [0x61, 0x62, 0x62] -> "abb"
    assert_eq!(start2, b"abb");
}

#[traced_test]
fn test_find_short_successor() {
    let cmp = BytewiseComparatorImpl::default();
    
    // Key #1: "abz" => first non-0xFF is 'a' -> 'b' (0x61 -> 0x62), 
    // truncate after i=0 => "b".
    let mut key1 = b"abz".to_vec();
    cmp.find_short_successor(&mut key1);
    assert_eq!(key1, b"b", "Expect the official LevelDB left-to-right behavior");

    // Key #2: entire key is 0xFF => do nothing
    let mut key2 = vec![0xFF, 0xFF, 0xFF];
    cmp.find_short_successor(&mut key2);
    assert_eq!(key2, vec![0xFF, 0xFF, 0xFF]);

    // Key #3: [ 'a', 0xFF, ... ] => first is 'a', not 0xFF => increment to 'b', truncate => "b"
    let mut key3 = vec![b'a', 0xFF, b'x', b'y', b'z'];
    cmp.find_short_successor(&mut key3);
    assert_eq!(key3, b"b", "Same logic as above");
}

#[traced_test]
fn test_bytewise_comparator_singleton() {
    let ptr1 = bytewise_comparator();
    assert!(!ptr1.is_null());
    let ptr2 = bytewise_comparator();
    assert_eq!(ptr1, ptr2, "Singleton pointer must not change");
}
