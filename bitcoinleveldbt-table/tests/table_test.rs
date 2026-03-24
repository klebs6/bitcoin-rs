// ---------------- [ File: bitcoinleveldbt-table/tests/table_test.rs ]
use bitcoinleveldbt_util::*;
use traced_test::*;
use tracing_setup::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_options::*;
use bitcoinleveldb_rand::*;
use bitcoinleveldb_tableconstructor::*;
use bitcoinleveldb_comparator::*;

pub struct TableTest {}

#[traced_test]
fn table_test_approximate_offset_of_plain() {
    let mut c = TableConstructor::new(Box::new(BytewiseComparatorImpl::default()));

    c.base_mut().add(&"k01".to_string(), &Slice::from("hello"));
    c.base_mut().add(&"k02".to_string(), &Slice::from("hello2"));

    let v3 = "x".repeat(10000);
    let v4 = "x".repeat(200000);
    let v5 = "x".repeat(300000);
    let v7 = "x".repeat(100000);

    c.base_mut().add(&"k03".to_string(), &Slice::from(&v3));
    c.base_mut().add(&"k04".to_string(), &Slice::from(&v4));
    c.base_mut().add(&"k05".to_string(), &Slice::from(&v5));
    c.base_mut().add(&"k06".to_string(), &Slice::from("hello3"));
    c.base_mut().add(&"k07".to_string(), &Slice::from(&v7));

    let mut options = Options::default();
    options.set_block_size(1024);
    options.set_compression(CompressionType::None);

    let data = c.base().data().clone();
    let s = c.finish_impl(&options, &data);
    assert!(s.is_ok());

    assert!(between(c.approximate_offset_of(&Slice::from("abc")), 0, 0));
    assert!(between(c.approximate_offset_of(&Slice::from("k01")), 0, 0));
    assert!(between(c.approximate_offset_of(&Slice::from("k01a")), 0, 0));
    assert!(between(c.approximate_offset_of(&Slice::from("k02")), 0, 0));
    assert!(between(c.approximate_offset_of(&Slice::from("k03")), 0, 0));
    assert!(between(c.approximate_offset_of(&Slice::from("k04")), 10000, 11000));
    assert!(between(c.approximate_offset_of(&Slice::from("k04a")), 210000, 211000));
    assert!(between(c.approximate_offset_of(&Slice::from("k05")), 210000, 211000));
    assert!(between(c.approximate_offset_of(&Slice::from("k06")), 510000, 511000));
    assert!(between(c.approximate_offset_of(&Slice::from("k07")), 510000, 511000));
    assert!(between(c.approximate_offset_of(&Slice::from("xyz")), 610000, 612000));
}

pub fn snappy_compression_supported() -> bool {
    cfg!(feature = "leveldb_snappy")
}

#[traced_test]
fn table_test_approximate_offset_of_compressed() {
    if !snappy_compression_supported() {
        eprintln!("skipping compression tests");
        return;
    }

    let mut rnd = Random::new(301);
    let mut c = TableConstructor::new(Box::new(BytewiseComparatorImpl::default()));
    let mut tmp = String::new();

    c.base_mut().add(&"k01".to_string(), &Slice::from("hello"));

    let k02 = bitcoinleveldbt_util::compressible_string(
        (&mut rnd) as *mut Random,
        0.25,
        10000,
        &mut tmp as *mut String,
    ).to_string();

    c.base_mut().add(&"k02".to_string(), &Slice::from(&k02));
    c.base_mut().add(&"k03".to_string(), &Slice::from("hello3"));

    let k04 = bitcoinleveldbt_util::compressible_string(
        (&mut rnd) as *mut Random,
        0.25,
        10000,
        &mut tmp as *mut String,
    ).to_string();

    c.base_mut().add(&"k04".to_string(), &Slice::from(&k04));

    let mut options = Options::default();
    options.set_block_size(1024);
    options.set_compression(CompressionType::Snappy);

    let data = c.base().data().clone();
    let s = c.finish_impl(&options, &data);
    assert!(s.is_ok());

    // Expected upper and lower bounds of space used by compressible strings.
    static K_SLOP: u64 = 1000; // Compressor effectiveness varies.
    let expected: u64 = 2500;  // 10000 * compression ratio (0.25)
    let min_z: u64 = expected - K_SLOP;
    let max_z: u64 = expected + K_SLOP;

    assert!(between(c.approximate_offset_of(&Slice::from("abc")), 0, K_SLOP));
    assert!(between(c.approximate_offset_of(&Slice::from("k01")), 0, K_SLOP));
    assert!(between(c.approximate_offset_of(&Slice::from("k02")), 0, K_SLOP));

    // Have now emitted a large compressible string, so adjust expected offset.
    assert!(between(c.approximate_offset_of(&Slice::from("k03")), min_z, max_z));
    assert!(between(c.approximate_offset_of(&Slice::from("k04")), min_z, max_z));

    // Have now emitted two large compressible strings, so adjust expected offset.
    assert!(between(c.approximate_offset_of(&Slice::from("xyz")), 2 * min_z, 2 * max_z));
}
