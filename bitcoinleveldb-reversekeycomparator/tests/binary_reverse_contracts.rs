use bitcoinleveldb_comparator::*;
use bitcoinleveldb_reversekeycomparator::*;
use bitcoinleveldb_slice::Slice;
use bitcoin_imports::Named;

fn order(cmp: &ReverseKeyComparator, a: &[u8], b: &[u8]) -> i32 {
    cmp.compare(&Slice::from(a), &Slice::from(b)).signum()
}

#[test]
fn reverse_handles_empty_default_slice() {
    assert!(reverse(&Slice::default()).is_empty());
}

#[test]
fn reverse_preserves_every_byte_and_is_an_involution() {
    let source: Vec<u8> = (0u8..=255).collect();
    let reversed = reverse(&Slice::from(source.as_slice()));
    assert_eq!(reversed.first(), Some(&0xff));
    assert_eq!(reversed.last(), Some(&0));
    assert_eq!(reverse(&Slice::from(reversed.as_slice())), source);

    let literal = [0, 0xfd, 0xfe, 0xff, 0xe2, 0x82, 0xac];
    assert_eq!(reverse(&Slice::from(&literal[..])), vec![0xac, 0x82, 0xe2, 0xff, 0xfe, 0xfd, 0]);
}

#[test]
fn comparison_distinguishes_all_single_byte_keys() {
    let cmp = ReverseKeyComparator;
    for n in 0u16..255 {
        assert_eq!(order(&cmp, &[n as u8], &[(n+1) as u8]), -1);
    }
    assert_eq!(order(&cmp, &[0xfd], &[0xfe]), -1);
    assert_eq!(order(&cmp, &[0xfe], &[0xff]), -1);
}

#[test]
fn binary_order_matches_independent_slice_order_and_is_transitive() {
    let cmp = ReverseKeyComparator;
    let keys: Vec<Vec<u8>> = vec![
        vec![], vec![0], vec![0, 0xff], vec![0xff, 0], vec![0xfd], vec![0xfe], vec![0xff],
        b"ab".to_vec(), b"ac".to_vec(), "€".as_bytes().to_vec(), "�".as_bytes().to_vec(),
    ];
    for a in &keys {
        for b in &keys {
            // Standard slice ordering is the reference, independent of LevelDB's
            // bytewise comparator and its return-value magnitude.
            let ra: Vec<u8> = a.iter().rev().copied().collect();
            let rb: Vec<u8> = b.iter().rev().copied().collect();
            let expected = match ra.as_slice().cmp(rb.as_slice()) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            let actual = order(&cmp, a, b);
            assert_eq!(actual, expected);
            assert_eq!(actual, -order(&cmp, b, a));
            assert_eq!(actual==0, a==b);
            for c in &keys {
                if actual<0 && order(&cmp, b, c)<0 {
                    assert!(order(&cmp, a, c)<0);
                }
            }
        }
    }
}

#[test]
fn separator_preserves_binary_order_bounds_and_literal_bytes() {
    let cmp = ReverseKeyComparator;
    let original = vec![0xfd, 0x10, 0xfe];
    let limit = vec![0xff, 0x30, 0xfe];
    let mut start = original.clone();
    cmp.find_shortest_separator(&mut start, &limit);
    assert_eq!(start, vec![0x11, 0xfe]);
    assert!(order(&cmp, &original, &start)<=0);
    assert!(order(&cmp, &start, &limit)<0);
}

#[test]
fn separator_keeps_prefix_and_adjacent_high_byte_boundaries() {
    let cmp = ReverseKeyComparator;
    for (mut start, limit) in [
        (vec![0xff], vec![0, 0xff]),
        (vec![0, 0xfe], vec![0, 0xff]),
        (vec![], vec![0]),
    ] {
        let original = start.clone();
        assert!(order(&cmp, &start, &limit)<0);
        cmp.find_shortest_separator(&mut start, &limit);
        assert_eq!(start, original);
    }
}

#[test]
fn successor_preserves_binary_order_and_maximal_boundaries() {
    let cmp = ReverseKeyComparator;
    let original = vec![0xfd, 0x10, 0xfe];
    let mut key = original.clone();
    cmp.find_short_successor(&mut key);
    assert_eq!(key, vec![0xff]);
    assert!(order(&cmp, &original, &key)<0);
    for mut key in [vec![], vec![0xff], vec![0xff, 0xff]] {
        let original = key.clone();
        cmp.find_short_successor(&mut key);
        assert_eq!(key, original);
    }
}

#[test]
fn bytewise_increment_appends_one_nul_without_text_conversion() {
    let original = vec![0xfd, 0xfe, 0xff, 0, 0xe2, 0x82, 0xac];
    let mut key = original.clone();
    increment(Box::new(BytewiseComparatorImpl::default()), &mut key);
    assert_eq!(key, vec![0xfd, 0xfe, 0xff, 0, 0xe2, 0x82, 0xac, 0]);
    assert!(original.as_slice()<key.as_slice());
}

#[test]
fn reverse_increment_prepends_one_nul_without_text_conversion() {
    let original = vec![0xfd, 0xfe, 0xff, 0, 0xe2, 0x82, 0xac];
    let mut key = original.clone();
    increment(Box::new(ReverseKeyComparator), &mut key);
    assert_eq!(key, vec![0, 0xfd, 0xfe, 0xff, 0, 0xe2, 0x82, 0xac]);
    assert!(order(&ReverseKeyComparator, &original, &key)<0);
    let mut empty = Vec::<u8>::new();
    increment(Box::new(ReverseKeyComparator), &mut empty);
    assert_eq!(empty, vec![0]);
}

#[test]
fn byte_ordering_has_a_distinct_persistent_comparator_identity() {
    let cmp = ReverseKeyComparator;
    assert_eq!(cmp.name().as_ref(), "bitcoinrs.ReverseBytewiseComparator.BytesV1");
    assert_ne!(cmp.name().as_ref(), "leveldb.ReverseBytewiseComparator");
}
