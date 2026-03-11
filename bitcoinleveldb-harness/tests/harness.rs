// ---------------- [ File: bitcoinleveldb-harness/tests/harness.rs ]
use bitcoin_imports::*;
use bitcoinleveldb_dbtest::{TestArgs, TestType};
use bitcoinleveldb_harness::Harness;
use bitcoinleveldb_iterator::{LevelDBIterator, MockStubIterator};
use bitcoinleveldb_rand::Random;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_dbimplinner::*;
use bitcoinleveldb_comparator::*;
use bitcoinleveldb_block::*;
use bitcoinleveldb_blockcontents::*;
use bitcoinleveldb_iteratorinner::*;

/// Stable LevelDB constant used by the upstream harness.
///
/// Invariant: LevelDB uses 7 levels by default (`config::kNumLevels == 7` in C++).
const BITCOINLEVELDB_HARNESS_TEST_K_NUM_LEVELS: i32 = 7;

fn c_atoi_decimal_i32(bytes: &[u8]) -> i32 {
    let mut i: usize = 0;

    while i < bytes.len() {
        let b = bytes[i];
        if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 0x0b || b == 0x0c {
            i += 1;
        } else {
            break;
        }
    }

    let mut neg: bool = false;
    if i < bytes.len() {
        if bytes[i] == b'-' {
            neg = true;
            i += 1;
        } else if bytes[i] == b'+' {
            i += 1;
        }
    }

    let mut acc: i64 = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b >= b'0' && b <= b'9' {
            acc = acc.saturating_mul(10).saturating_add((b - b'0') as i64);
            i += 1;
        } else {
            break;
        }
    }

    if neg {
        acc = -acc;
    }

    if acc > i32::MAX as i64 {
        i32::MAX
    } else if acc < i32::MIN as i64 {
        i32::MIN
    } else {
        acc as i32
    }
}

fn leveldb_test_random_seed() -> u32 {
    if let Ok(v) = std::env::var("TEST_RANDOM_SEED") {
        let seed_i32: i32 = c_atoi_decimal_i32(v.as_bytes());
        return seed_i32 as u32;
    }

    let now = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d,
        Err(_e) => std::time::Duration::from_secs(0),
    };

    now.as_secs() as u32
}

fn leveldb_test_random_string_into(rnd: &mut Random, len: u32, dst: &mut String) -> Slice {
    let n: usize = len as usize;

    dst.clear();
    dst.reserve(n);

    let mut i: usize = 0;
    while i < n {
        let b: u8 = b' ' + (rnd.uniform(95) as u8);
        dst.push(b as char);
        i += 1;
    }

    Slice::from(dst.as_bytes())
}

fn leveldb_test_random_key(rnd: &mut Random, len: u32) -> String {
    let mut dst: String = String::new();
    let _ = leveldb_test_random_string_into(rnd, len, &mut dst);
    dst
}

fn harness_test_arg_matrix() -> Vec<TestArgs> {
    vec![
        TestArgs {
            ty: TestType::TABLE_TEST,
            reverse_compare: false,
            restart_interval: 16,
        },
        TestArgs {
            ty: TestType::TABLE_TEST,
            reverse_compare: false,
            restart_interval: 1,
        },
        TestArgs {
            ty: TestType::TABLE_TEST,
            reverse_compare: false,
            restart_interval: 1024,
        },
        TestArgs {
            ty: TestType::TABLE_TEST,
            reverse_compare: true,
            restart_interval: 16,
        },
        TestArgs {
            ty: TestType::TABLE_TEST,
            reverse_compare: true,
            restart_interval: 1,
        },
        TestArgs {
            ty: TestType::TABLE_TEST,
            reverse_compare: true,
            restart_interval: 1024,
        },
        TestArgs {
            ty: TestType::BLOCK_TEST,
            reverse_compare: false,
            restart_interval: 16,
        },
        TestArgs {
            ty: TestType::BLOCK_TEST,
            reverse_compare: false,
            restart_interval: 1,
        },
        TestArgs {
            ty: TestType::BLOCK_TEST,
            reverse_compare: false,
            restart_interval: 1024,
        },
        TestArgs {
            ty: TestType::BLOCK_TEST,
            reverse_compare: true,
            restart_interval: 16,
        },
        TestArgs {
            ty: TestType::BLOCK_TEST,
            reverse_compare: true,
            restart_interval: 1,
        },
        TestArgs {
            ty: TestType::BLOCK_TEST,
            reverse_compare: true,
            restart_interval: 1024,
        },
        // Restart interval does not matter for memtables
        TestArgs {
            ty: TestType::MEMTABLE_TEST,
            reverse_compare: false,
            restart_interval: 16,
        },
        TestArgs {
            ty: TestType::MEMTABLE_TEST,
            reverse_compare: true,
            restart_interval: 16,
        },
        // Do not bother with restart interval variations for DB
        TestArgs {
            ty: TestType::DB_TEST,
            reverse_compare: false,
            restart_interval: 16,
        },
        TestArgs {
            ty: TestType::DB_TEST,
            reverse_compare: true,
            restart_interval: 16,
        },
    ]
}

/**
   Test empty table/block.
  */
#[traced_test]
fn harness_empty() {
    /*
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 1);
        Test(&rnd);
      }
    */
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_empty",
    );

    let mut harness = Harness::default();
    let test_args = harness_test_arg_matrix();
    let k_num_test_args: i32 = test_args.len() as i32;

    let mut i: i32 = 0;
    while i < k_num_test_args {
        harness.init(&test_args[i as usize]);

        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(1));
        harness.test(&mut rnd as *mut Random);

        i += 1;
    }
}

/**
  | Special test for a block with no restart
  | entries.  The C++ leveldb code never generates
  | such blocks, but the Java version of leveldb
  | seems to.
  */
#[traced_test]
fn harness_zero_restart_points_in_block() {
    /*
      char data[sizeof(uint32_t)];
      memset(data, 0, sizeof(data));
      BlockContents contents;
      contents.data = Slice(data, sizeof(data));
      contents.cachable = false;
      contents.heap_allocated = false;
      Block block(contents);
      Iterator* iter = block.NewIterator(BytewiseComparator());
      iter->SeekToFirst();
      ASSERT_TRUE(!iter->Valid());
      iter->SeekToLast();
      ASSERT_TRUE(!iter->Valid());
      iter->Seek("foo");
      ASSERT_TRUE(!iter->Valid());
      delete iter;
    */
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_zero_restart_points_in_block",
    );

    let mut data: [u8; core::mem::size_of::<u32>()] = [0u8; core::mem::size_of::<u32>()];

    let mut contents: BlockContents = BlockContents::default();
    contents.set_data(Slice::from(data.as_slice()));
    contents.set_cachable(false);
    contents.set_heap_allocated(false);

    let block: Block = Block::new(&contents);

    let iter: *mut LevelDBIterator = block.new_iterator(bytewise_comparator());
    unsafe {
        (&mut *iter).seek_to_first();
        assert!(!(&*iter).valid());

        (&mut *iter).seek_to_last();
        assert!(!(&*iter).valid());

        let foo = Slice::from(b"foo".as_slice());
        (&mut *iter).seek(&foo);
        assert!(!(&*iter).valid());

        drop(Box::from_raw(iter));
    }
}

/**
  | Test the empty key
  |
  */
#[traced_test]
fn harness_simple_empty_key() {
    /*
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 1);
        Add("", "v");
        Test(&rnd);
      }
    */
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_simple_empty_key",
    );

    let mut harness = Harness::default();
    let test_args = harness_test_arg_matrix();
    let k_num_test_args: i32 = test_args.len() as i32;

    let mut i: i32 = 0;
    while i < k_num_test_args {
        harness.init(&test_args[i as usize]);

        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(1));
        let k: String = String::new();
        let v: String = "v".to_owned();
        harness.add(&k, &v);
        harness.test(&mut rnd as *mut Random);

        i += 1;
    }
}

#[traced_test]
fn harness_simple_single() {
    /*
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 2);
        Add("abc", "v");
        Test(&rnd);
      }
    */
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_simple_single",
    );

    let mut harness = Harness::default();
    let test_args = harness_test_arg_matrix();
    let k_num_test_args: i32 = test_args.len() as i32;

    let mut i: i32 = 0;
    while i < k_num_test_args {
        harness.init(&test_args[i as usize]);

        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(2));
        let k: String = "abc".to_owned();
        let v: String = "v".to_owned();
        harness.add(&k, &v);
        harness.test(&mut rnd as *mut Random);

        i += 1;
    }
}

#[traced_test]
fn harness_simple_multi() {
    /*
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 3);
        Add("abc", "v");
        Add("abcd", "v");
        Add("ac", "v2");
        Test(&rnd);
      }
    */
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_simple_multi",
    );

    let mut harness = Harness::default();
    let test_args = harness_test_arg_matrix();
    let k_num_test_args: i32 = test_args.len() as i32;

    let mut i: i32 = 0;
    while i < k_num_test_args {
        harness.init(&test_args[i as usize]);

        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(3));

        let k1: String = "abc".to_owned();
        let v1: String = "v".to_owned();
        harness.add(&k1, &v1);

        let k2: String = "abcd".to_owned();
        let v2: String = "v".to_owned();
        harness.add(&k2, &v2);

        let k3: String = "ac".to_owned();
        let v3: String = "v2".to_owned();
        harness.add(&k3, &v3);

        harness.test(&mut rnd as *mut Random);

        i += 1;
    }
}

#[traced_test]
fn harness_simple_special_key() {
    /*
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 4);
        Add("\xff\xff", "v3");
        Test(&rnd);
      }
    */
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_simple_special_key",
    );

    let mut harness = Harness::default();
    let test_args = harness_test_arg_matrix();
    let k_num_test_args: i32 = test_args.len() as i32;

    let mut i: i32 = 0;
    while i < k_num_test_args {
        harness.init(&test_args[i as usize]);

        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(4));

        // Rust `String` must be valid UTF-8; use a non-ASCII key to exercise unusual byte sequences.
        let k: String = "\u{00FF}\u{00FF}".to_owned();
        let v: String = "v3".to_owned();
        harness.add(&k, &v);

        harness.test(&mut rnd as *mut Random);

        i += 1;
    }
}

#[traced_test]
fn harness_randomized() {
    /*
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 5);
        for (int num_entries = 0; num_entries < 2000;
             num_entries += (num_entries < 50 ? 1 : 200)) {
          if ((num_entries % 10) == 0) {
            fprintf(stderr, "case %d of %d: num_entries = %d\n", (i + 1),
                    int(kNumTestArgs), num_entries);
          }
          for (int e = 0; e < num_entries; e++) {
            std::string v;
            Add(test::RandomKey(&rnd, rnd.Skewed(4)),
                test::RandomString(&rnd, rnd.Skewed(5), &v).ToString());
          }
          Test(&rnd);
        }
      }
    */
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_randomized",
    );

    let mut harness = Harness::default();
    let test_args = harness_test_arg_matrix();
    let k_num_test_args: i32 = test_args.len() as i32;

    let mut i: i32 = 0;
    while i < k_num_test_args {
        harness.init(&test_args[i as usize]);

        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(5));

        let mut num_entries: i32 = 0;
        while num_entries < 2000 {
            if (num_entries % 10) == 0 {
                tracing::info!(
                    target: "bitcoinleveldb_harness_tests",
                    label = "bitcoinleveldb_harness_tests.harness_randomized.case_progress",
                    case_index = (i + 1),
                    case_count = k_num_test_args,
                    num_entries = num_entries,
                );
            }

            let mut e: i32 = 0;
            while e < num_entries {
                let mut v: String = String::new();

                let klen: u32 = rnd.skewed(4);
                let vlen: u32 = rnd.skewed(5);

                let key: String = leveldb_test_random_key(&mut rnd, klen);
                let _ = leveldb_test_random_string_into(&mut rnd, vlen, &mut v);
                let value: String = v.clone();

                harness.add(&key, &value);

                e += 1;
            }

            harness.test(&mut rnd as *mut Random);

            num_entries += if num_entries < 50 { 1 } else { 200 };
        }

        i += 1;
    }
}

#[traced_test]
fn harness_randomized_longdb() {
    /*
      Random rnd(test::RandomSeed());
      TestArgs args = {DB_TEST, false, 16};
      Init(args);
      int num_entries = 100000;
      for (int e = 0; e < num_entries; e++) {
        std::string v;
        Add(test::RandomKey(&rnd, rnd.Skewed(4)),
            test::RandomString(&rnd, rnd.Skewed(5), &v).ToString());
      }
      Test(&rnd);

      // We must have created enough data to force merging
      int files = 0;
      for (int level = 0; level < config::kNumLevels; level++) {
        std::string value;
        char name[100];
        snprintf(name, sizeof(name), "leveldb.num-files-at-level%d", level);
        ASSERT_TRUE(db()->GetProperty(name, &value));
        files += atoi(value.c_str());
      }
      ASSERT_GT(files, 0);
    */
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_randomized_longdb",
    );

    let mut rnd = Random::new(leveldb_test_random_seed());

    let args = TestArgs {
        ty: TestType::DB_TEST,
        reverse_compare: false,
        restart_interval: 16,
    };

    let mut harness = Harness::default();
    harness.init(&args);

    let num_entries: i32 = 100000;

    let mut e: i32 = 0;
    while e < num_entries {
        let mut v: String = String::new();

        let klen: u32 = rnd.skewed(4);
        let vlen: u32 = rnd.skewed(5);

        let key: String = leveldb_test_random_key(&mut rnd, klen);
        let _ = leveldb_test_random_string_into(&mut rnd, vlen, &mut v);
        let value: String = v.clone();

        harness.add(&key, &value);

        e += 1;
    }

    harness.test(&mut rnd as *mut Random);

    // We must have created enough data to force merging
    let mut files: i32 = 0;

    let db_ptr = match harness.db() {
        Some(p) => p,
        None => {
            tracing::error!(
                target: "bitcoinleveldb_harness_tests",
                label = "bitcoinleveldb_harness_tests.harness_randomized_longdb.db_pointer_missing",
            );
            panic!();
        }
    };

    let mut level: i32 = 0;
    while level < BITCOINLEVELDB_HARNESS_TEST_K_NUM_LEVELS {
        let mut value: String = String::new();
        let name: String = format!("leveldb.num-files-at-level{}", level);

        unsafe {
            assert!((&mut *db_ptr).get_property(&name, &mut value as *mut String));
        }

        files += c_atoi_decimal_i32(value.as_bytes());

        level += 1;
    }

    assert!(files > 0);
}

#[traced_test]
fn harness_db_pointer_is_null_when_not_db_test() {
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_db_pointer_is_null_when_not_db_test",
    );

    let mut harness = Harness::default();
    let test_args = harness_test_arg_matrix();
    let mut i: usize = 0;

    while i < test_args.len() {
        let args = &test_args[i];
        harness.init(args);

        if args.ty != TestType::DB_TEST {
            let db_ptr = harness.db();
            assert!(db_ptr.is_none());
        }

        i += 1;
    }
}

#[traced_test]
fn harness_reinitialization_switches_constructor_types_without_panicking() {
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_reinitialization_switches_constructor_types_without_panicking",
    );

    let mut harness = Harness::default();

    let args_table = TestArgs {
        ty: TestType::TABLE_TEST,
        reverse_compare: false,
        restart_interval: 16,
    };
    harness.init(&args_table);

    let mut rnd1 = Random::new(leveldb_test_random_seed().wrapping_add(11));

    let k1: String = "k1".to_owned();
    let v1: String = "v1".to_owned();
    harness.add(&k1, &v1);

    harness.test(&mut rnd1 as *mut Random);

    let args_block = TestArgs {
        ty: TestType::BLOCK_TEST,
        reverse_compare: true,
        restart_interval: 1,
    };
    harness.init(&args_block);

    let mut rnd2 = Random::new(leveldb_test_random_seed().wrapping_add(12));
    harness.test(&mut rnd2 as *mut Random);
}

#[traced_test]
fn harness_duplicate_keys_overwrite_previous_value() {
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_duplicate_keys_overwrite_previous_value",
    );

    let mut harness = Harness::default();
    let test_args = harness_test_arg_matrix();
    let k_num_test_args: i32 = test_args.len() as i32;

    let mut i: i32 = 0;
    while i < k_num_test_args {
        harness.init(&test_args[i as usize]);

        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(21));

        let k: String = "dup".to_owned();
        let v1: String = "v1".to_owned();
        let v2: String = "v2".to_owned();

        harness.add(&k, &v1);
        harness.add(&k, &v2);

        harness.test(&mut rnd as *mut Random);

        i += 1;
    }
}

#[traced_test]
fn harness_handles_embedded_nul_and_empty_values() {
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_handles_embedded_nul_and_empty_values",
    );

    let mut harness = Harness::default();
    let test_args = harness_test_arg_matrix();
    let k_num_test_args: i32 = test_args.len() as i32;

    let mut i: i32 = 0;
    while i < k_num_test_args {
        harness.init(&test_args[i as usize]);

        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(31));

        let k1: String = "\u{0000}\u{0001}\u{0000}".to_owned();
        let v1: String = String::new();
        harness.add(&k1, &v1);

        let k2: String = "\u{0000}\u{0001}\u{0000}\u{00FF}".to_owned();
        let v2: String = "\u{00FF}".to_owned();
        harness.add(&k2, &v2);

        harness.test(&mut rnd as *mut Random);

        i += 1;
    }
}

#[traced_test]
fn harness_pick_random_key_returns_foo_on_empty_keyset() {
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_pick_random_key_returns_foo_on_empty_keyset",
    );

    let mut harness = Harness::default();

    let args = TestArgs {
        ty: TestType::TABLE_TEST,
        reverse_compare: false,
        restart_interval: 16,
    };
    harness.init(&args);

    let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(41));

    let keys: Vec<String> = Vec::new();
    let k = harness.pick_random_key(&mut rnd as *mut Random, &keys);

    assert_eq!(k, "foo");
}

#[traced_test]
fn harness_to_string_formats_iterator_entries_and_end_sentinel() {
    tracing::info!(
        target: "bitcoinleveldb_harness_tests",
        label = "bitcoinleveldb_harness_tests.harness_to_string_formats_iterator_entries_and_end_sentinel",
    );

    let mut harness = Harness::default();

    let inner = MockStubIterator::new_with_entries(&[(b"k".as_slice(), b"v".as_slice())]);
    let mut iter = LevelDBIterator::new(Some(Box::new(inner)));

    let end = harness.to_string(&iter as *const LevelDBIterator);
    assert_eq!(end, "END");

    iter.seek_to_first();

    let got = harness.to_string(&iter as *const LevelDBIterator);
    assert_eq!(got, "'k->v'");
}
