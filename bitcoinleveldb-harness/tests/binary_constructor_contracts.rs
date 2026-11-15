use bitcoinleveldb_harness::Harness;
use bitcoinleveldb_rand::Random;
use bitcoinleveldbt_dbtest::{TestArgs, TestType};

fn exercise(ty: TestType) {
    for reverse in [false, true] {
        let mut harness = Harness::default();
        harness.init(&TestArgs { ty, reverse_compare: reverse, restart_interval: 1 });
        let keys = [vec![], vec![0], vec![0, 0xff], vec![0x80], vec![0xfe], vec![0xff],
            "�".as_bytes().to_vec(), b"a->b".to_vec(), b"a".to_vec()];
        let payload: Vec<u8> = (0u8..=255).collect();
        for (index, key) in keys.iter().enumerate() {
            let value = [payload.as_slice(), &[index as u8]].concat();
            harness.add_bytes(key, &value);
        }
        harness.add_bytes(b"empty-value", &[]);
        harness.add_bytes(b"a", b"b->c"); harness.add_bytes(b"a->b", b"c");
        let mut random = Random::new(301);
        harness.test_bytes(&mut random);
        // Re-finalization must replace old backing state after iterators die.
        harness.add_bytes(&[0xff, 0], &[0x80, 0, 0xff]);
        harness.add_bytes(&[], &[]);
        harness.test_bytes(std::ptr::null_mut());
        // Keep the legacy text surface usable with the same comparator policy.
        harness.add(&String::from("ascii"), &String::from("value"));
        harness.test(&mut random);
    }
}

#[test]
fn binary_block_constructor_handles_both_orders_and_refinalization() { exercise(TestType::BLOCK_TEST); }
#[test]
fn binary_table_constructor_handles_both_orders_and_refinalization() { exercise(TestType::TABLE_TEST); }
#[test]
fn binary_memtable_constructor_handles_both_orders_and_refinalization() { exercise(TestType::MEMTABLE_TEST); }
#[test]
fn binary_database_constructor_handles_both_orders_and_refinalization() { exercise(TestType::DB_TEST); }
