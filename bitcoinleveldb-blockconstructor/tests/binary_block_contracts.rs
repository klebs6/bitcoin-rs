use bitcoin_imports::KVMap;
use bitcoinleveldb_block::Block;
use bitcoinleveldb_blockbuilder::BlockBuilder;
use bitcoinleveldb_blockconstructor::BlockConstructor;
use bitcoinleveldb_blockcontents::BlockContents;
use bitcoinleveldb_comparator::{BytewiseComparatorImpl, bytewise_comparator};
use bitcoinleveldb_iterator::LevelDBIterator;
use bitcoinleveldb_iteratorinner::{
    LevelDBIteratorKey, LevelDBIteratorNext, LevelDBIteratorPrev,
    LevelDBIteratorSeek, LevelDBIteratorSeekToFirst, LevelDBIteratorSeekToLast,
    LevelDBIteratorStatus, LevelDBIteratorValid, LevelDBIteratorValue,
};
use bitcoinleveldb_options::Options;
use bitcoinleveldb_slice::Slice;

#[test]
fn constructor_owns_non_utf8_block_metadata_and_rebuilds_after_iterator_drop() {
    let mut constructor = BlockConstructor::new(Box::new(BytewiseComparatorImpl::default()));
    let options = Options::default();
    let first_key = "a".repeat(128);
    let second_key = format!("{}b", "a".repeat(127));
    let first_value = "v".repeat(129);
    let second_value = "w".repeat(128);
    let mut input = KVMap::default();
    input.insert(first_key.clone(), first_value.clone());
    input.insert(second_key.clone(), second_value.clone());
    assert!(constructor.finish_impl(&options, &input).is_ok());
    let mut expected = vec![0x00, 0x80, 0x01, 0x81, 0x01];
    expected.extend_from_slice(first_key.as_bytes());
    expected.extend_from_slice(first_value.as_bytes());
    expected.extend_from_slice(&[0x7f, 0x01, 0x80, 0x01, b'b']);
    expected.extend_from_slice(second_value.as_bytes());
    expected.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]);
    assert_eq!(constructor.data_bytes(), expected.as_slice());
    assert!(core::str::from_utf8(constructor.data_bytes()).is_err());
    unsafe {
        let block = &*constructor.block_ptr();
        assert_eq!(block.data_ptr(), constructor.data_bytes().as_ptr());
        assert_eq!(block.size(), expected.len());
        assert!(!block.is_owned());
    }
    let mut iter: Box<LevelDBIterator> = unsafe { Box::from_raw(constructor.new_iterator()) };
    iter.seek_to_first();
    assert!(iter.valid());
    assert_eq!(iter.key().as_bytes(), first_key.as_bytes());
    assert_eq!(iter.value().as_bytes(), first_value.as_bytes());
    iter.next();
    assert!(iter.valid());
    assert_eq!(iter.key().as_bytes(), second_key.as_bytes());
    assert_eq!(iter.value().as_bytes(), second_value.as_bytes());
    iter.next();
    assert!(!iter.valid());
    assert!(iter.status().is_ok());
    drop(iter);
    assert!(constructor.finish_impl(&options, &KVMap::default()).is_ok());
    assert_eq!(constructor.data_bytes(), &[0, 0, 0, 0, 1, 0, 0, 0]);
}

#[test]
fn binary_builder_block_iteration_seek_and_direction_changes_preserve_exact_bytes() {
    let mut options = Box::new(Options::default());
    options.set_block_restart_interval(2);
    let mut builder = BlockBuilder::new(&*options);
    let expected = vec![
        (vec![0x00, 0xff], vec![0xff; 128]),
        (vec![0x00, 0xff, 0x80], vec![0x80; 129]),
        (vec![0x80, 0x00], vec![0xc3, 0x28, 0x00]),
    ];
    for (key, value) in &expected {
        builder.add(&Slice::from(key.as_slice()), &Slice::from(value.as_slice()));
    }
    assert_eq!(builder.restarts_slice(), &[0, 268]);
    let bytes = builder.finish().as_bytes().to_vec();
    assert_eq!(&bytes[bytes.len()-12..], &[0, 0, 0, 0, 0x0c, 0x01, 0, 0, 2, 0, 0, 0]);
    drop(builder);
    drop(options);
    let contents = BlockContents::new(Slice::from(bytes.as_slice()), false, false);
    let block = Block::new(&contents);
    let raw = block.new_iterator(bytewise_comparator());
    assert!(!raw.is_null());
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
    iter.seek(&Slice::from(&[0x00, 0xff, 0x7f][..]));
    assert!(iter.valid());
    assert_eq!(iter.key().as_bytes(), expected[1].0.as_slice());
    assert_eq!(iter.value().as_bytes(), expected[1].1.as_slice());
    iter.next();
    assert_eq!(iter.key().as_bytes(), expected[2].0.as_slice());
    iter.prev();
    assert_eq!(iter.key().as_bytes(), expected[1].0.as_slice());
    iter.seek(&Slice::from(&[0xff][..]));
    assert!(!iter.valid());
    assert!(iter.status().is_ok());
    drop(iter);
}
