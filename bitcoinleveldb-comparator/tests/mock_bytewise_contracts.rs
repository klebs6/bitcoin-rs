// Focused contracts for the mocks' canonical bytewise-comparator capability.
use bitcoin_imports::Named;
use bitcoinleveldb_comparator::*;
use bitcoinleveldb_slice::Slice;

fn assert_canonical_bytewise(pointer: *const dyn SliceComparator) {
    let canonical = bytewise_comparator();
    assert!(!pointer.is_null(), "the delegated comparator must be non-null");
    assert!(!canonical.is_null());
    assert!(core::ptr::addr_eq(pointer, canonical),
            "the capability must refer to the canonical singleton object");
    // The preceding address check and the public lifetime contract establish
    // the expected singleton before any dereference of the returned pointer.
    let comparator = unsafe { &*pointer };
    assert_eq!(comparator.name().as_ref(), "leveldb.BytewiseComparator");
    let keys: [&[u8]; 10] = [
        b"", b"\0", b"\0\0", b"a", b"a\0", b"\x7f", b"\x80",
        b"\xfe", b"\xff", b"\xff\0",
    ];
    for left in &keys {
        for right in &keys {
            let a = Slice::from(*left);
            let b = Slice::from(*right);
            assert_eq!(comparator.compare(&a, &b).cmp(&0), left.cmp(right),
                       "canonical byte ordering differs for {:?} and {:?}", left, right);
        }
    }

    for (start, limit, expected) in [
        (b"".to_vec(), b"a".to_vec(), b"".to_vec()),
        (b"a".to_vec(), b"aa".to_vec(), b"a".to_vec()),
        (b"a\0tail".to_vec(), b"a\x02".to_vec(), b"a\x01".to_vec()),
        (vec![0xfe, 0], vec![0xff], vec![0xfe, 0]),
        (vec![0xff, 0], vec![0xff, 1], vec![0xff, 0]),
    ] {
        assert!(start < limit, "separator fixture must satisfy its precondition");
        let mut shortened = start.clone();
        comparator.find_shortest_separator(&mut shortened, &limit);
        assert_eq!(shortened, expected);
        assert!(start <= shortened && shortened < limit);
        assert!(shortened.len() <= start.len());
    }
    for (original, expected) in [
        (vec![], vec![]),
        (vec![0xff, 0xff], vec![0xff, 0xff]),
        (vec![0xfe, 0xff], vec![0xff]),
        (vec![0, 0xff], vec![1]),
    ] {
        let mut successor = original.clone();
        comparator.find_short_successor(&mut successor);
        assert_eq!(successor, expected);
        assert!(successor >= original);
        assert!(successor.len() <= original.len());
    }
}

fn assert_mock_forwarding<M: SliceComparator + 'static>(mock: &M, corrupt: bool) {
    let underlying = mock as *const M as *const dyn SliceComparator;
    // Keep the concrete mock alive until the adapter is dropped at function
    // exit, satisfying the adapter's documented non-owning lifetime condition.
    let adapter = borrowed_slice_comparator_adapter_box(underlying)
        .expect("a live non-null mock must be adaptable");
    assert_eq!(adapter.name().as_ref(), "dummy-comparator");
    let a = Slice::from(&b"a"[..]);
    let b = Slice::from(&b"b"[..]);
    if corrupt {
        assert_eq!(mock.compare(&a, &b), 0);
        assert_eq!(adapter.compare(&a, &b), 0,
                   "requesting bytewise capability must not replace the corrupt mock");
    } else {
        assert!(mock.compare(&a, &b) < 0);
        assert!(adapter.compare(&a, &b) < 0);
    }
    let mut separator = b"foo1".to_vec();
    adapter.find_shortest_separator(&mut separator, b"foo9");
    assert_eq!(separator, b"foo1", "the mock's no-op shortener must remain a no-op");
    let mut successor = b"foo".to_vec();
    adapter.find_short_successor(&mut successor);
    assert_eq!(successor, b"foo", "the adapter must preserve the mock's no-op successor");
    assert_canonical_bytewise(adapter.bytewise_comparator());
    if corrupt {
        assert_eq!(adapter.compare(&a, &b), 0,
                   "querying the capability must leave adapter dispatch unchanged");
    }
}

#[test]
fn mock_bytewise_pointer_contract() {
    let pointer = {
        let mock = MockComparator::default();
        assert_mock_forwarding(&mock, false);
        mock.bytewise_comparator()
    };
    assert_canonical_bytewise(pointer);
    for _ in 0..8 {
        assert_canonical_bytewise(MockComparator::default().bytewise_comparator());
    }
    eprintln!("\nCOMPARATOR_CONTRACT_OK case=mock_bytewise_pointer_contract");
}

#[test]
fn corrupt_mock_bytewise_pointer_contract() {
    let pointer = {
        let mock = MockCorruptComparator::default();
        assert_mock_forwarding(&mock, true);
        mock.bytewise_comparator()
    };
    assert_canonical_bytewise(pointer);
    for _ in 0..8 {
        assert_canonical_bytewise(MockCorruptComparator::default().bytewise_comparator());
    }
    eprintln!("\nCOMPARATOR_CONTRACT_OK case=corrupt_mock_bytewise_pointer_contract");
}
