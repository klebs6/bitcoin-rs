#!/usr/bin/env python3
"""Harness repair031: preserve all fourteen original workloads, add real resource admission and quiet logging, qualify each exact case under finite controls on an isolated candidate; no master update."""
import argparse
import datetime as dt
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import signal
import stat
import subprocess
import sys
import types
import uuid
MIB027 = 1024 * 1024
WORKFLOW = 'leveldb-harness-controlled031'
MASTER_BASE = '2a5093e69dd91dc6d770e7e8ed1d201751f9ad5c'
MASTER_TREE = '061d6ba025e0d2803bf4add4029aebc978d1fbfd'
BASE = 'fceccea344550ced58044a34dce9e1c677c54bb9'
BASE_TREE = '3f15b24845aac80af16dac7c7c840f68cadf9abe'
BRANCH = 'repair/leveldb-harness-controlled031'
SUFFIX = '-leveldb-harness-controlled031'
REPLAY_PATH = 'contrib/repair/bitcoinleveldb-harness-controlled031.py'
DOCUMENT_PATH = 'doc/testing/leveldb-harness-controlled031.md'
RECEIPT_PATH = 'doc/repair/leveldb-harness-controlled031.json'
COMMIT_MESSAGE = b'test: control LevelDB harness execution and qualify original cases\n\nRetain every workload and oracle, require real finite controls before\nfixture work, keep heavy cases ignored by default and verify all sixteen\nselected cases with fixed seed, quiet logging and private temporary roots.\n'


SOURCE_PAYLOADS = {'bitcoinleveldb-harness/tests/harness.rs': b'// ---------------- [ File: bitcoinleveldb-harness/tests/harness.rs ]\nuse bitcoin_imports::*;\n\n#[path = "support/execution_controls.rs"]\nmod execution_controls;\nuse bitcoinleveldbt_dbtest::{TestArgs, TestType};\nuse bitcoinleveldb_harness::Harness;\nuse bitcoinleveldb_iterator::{LevelDBIterator, MockStubIterator};\nuse bitcoinleveldb_rand::Random;\nuse bitcoinleveldb_slice::*;\nuse bitcoinleveldb_dbimplinner::*;\nuse bitcoinleveldb_comparator::*;\nuse bitcoinleveldb_block::*;\nuse bitcoinleveldb_blockcontents::*;\nuse bitcoinleveldb_iteratorinner::*;\n\n/// Stable LevelDB constant used by the upstream harness.\n///\n/// Invariant: LevelDB uses 7 levels by default (`config::kNumLevels == 7` in C++).\nconst BITCOINLEVELDB_HARNESS_TEST_K_NUM_LEVELS: i32 = 7;\n\nfn c_atoi_decimal_i32(bytes: &[u8]) -> i32 {\n    let mut i: usize = 0;\n\n    while i < bytes.len() {\n        let b = bytes[i];\n        if b == b\' \' || b == b\'\\t\' || b == b\'\\n\' || b == b\'\\r\' || b == 0x0b || b == 0x0c {\n            i += 1;\n        } else {\n            break;\n        }\n    }\n\n    let mut neg: bool = false;\n    if i < bytes.len() {\n        if bytes[i] == b\'-\' {\n            neg = true;\n            i += 1;\n        } else if bytes[i] == b\'+\' {\n            i += 1;\n        }\n    }\n\n    let mut acc: i64 = 0;\n    while i < bytes.len() {\n        let b = bytes[i];\n        if b >= b\'0\' && b <= b\'9\' {\n            acc = acc.saturating_mul(10).saturating_add((b - b\'0\') as i64);\n            i += 1;\n        } else {\n            break;\n        }\n    }\n\n    if neg {\n        acc = -acc;\n    }\n\n    if acc > i32::MAX as i64 {\n        i32::MAX\n    } else if acc < i32::MIN as i64 {\n        i32::MIN\n    } else {\n        acc as i32\n    }\n}\n\nfn leveldb_test_random_seed() -> u32 {\n    if let Ok(v) = std::env::var("TEST_RANDOM_SEED") {\n        let seed_i32: i32 = c_atoi_decimal_i32(v.as_bytes());\n        return seed_i32 as u32;\n    }\n\n    let now = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {\n        Ok(d) => d,\n        Err(_e) => std::time::Duration::from_secs(0),\n    };\n\n    now.as_secs() as u32\n}\n\nfn leveldb_test_random_string_into(rnd: &mut Random, len: u32, dst: &mut String) -> Slice {\n    let n: usize = len as usize;\n\n    dst.clear();\n    dst.reserve(n);\n\n    let mut i: usize = 0;\n    while i < n {\n        let b: u8 = b\' \' + (rnd.uniform(95) as u8);\n        dst.push(b as char);\n        i += 1;\n    }\n\n    Slice::from(dst.as_bytes())\n}\n\nfn leveldb_test_random_key(rnd: &mut Random, len: u32) -> String {\n    let mut dst: String = String::new();\n    let _ = leveldb_test_random_string_into(rnd, len, &mut dst);\n    dst\n}\n\nfn harness_test_arg_matrix() -> Vec<TestArgs> {\n    vec![\n        TestArgs {\n            ty: TestType::TABLE_TEST,\n            reverse_compare: false,\n            restart_interval: 16,\n        },\n        TestArgs {\n            ty: TestType::TABLE_TEST,\n            reverse_compare: false,\n            restart_interval: 1,\n        },\n        TestArgs {\n            ty: TestType::TABLE_TEST,\n            reverse_compare: false,\n            restart_interval: 1024,\n        },\n        TestArgs {\n            ty: TestType::TABLE_TEST,\n            reverse_compare: true,\n            restart_interval: 16,\n        },\n        TestArgs {\n            ty: TestType::TABLE_TEST,\n            reverse_compare: true,\n            restart_interval: 1,\n        },\n        TestArgs {\n            ty: TestType::TABLE_TEST,\n            reverse_compare: true,\n            restart_interval: 1024,\n        },\n        TestArgs {\n            ty: TestType::BLOCK_TEST,\n            reverse_compare: false,\n            restart_interval: 16,\n        },\n        TestArgs {\n            ty: TestType::BLOCK_TEST,\n            reverse_compare: false,\n            restart_interval: 1,\n        },\n        TestArgs {\n            ty: TestType::BLOCK_TEST,\n            reverse_compare: false,\n            restart_interval: 1024,\n        },\n        TestArgs {\n            ty: TestType::BLOCK_TEST,\n            reverse_compare: true,\n            restart_interval: 16,\n        },\n        TestArgs {\n            ty: TestType::BLOCK_TEST,\n            reverse_compare: true,\n            restart_interval: 1,\n        },\n        TestArgs {\n            ty: TestType::BLOCK_TEST,\n            reverse_compare: true,\n            restart_interval: 1024,\n        },\n        // Restart interval does not matter for memtables\n        TestArgs {\n            ty: TestType::MEMTABLE_TEST,\n            reverse_compare: false,\n            restart_interval: 16,\n        },\n        TestArgs {\n            ty: TestType::MEMTABLE_TEST,\n            reverse_compare: true,\n            restart_interval: 16,\n        },\n        // Do not bother with restart interval variations for DB\n        TestArgs {\n            ty: TestType::DB_TEST,\n            reverse_compare: false,\n            restart_interval: 16,\n        },\n        TestArgs {\n            ty: TestType::DB_TEST,\n            reverse_compare: true,\n            restart_interval: 16,\n        },\n    ]\n}\n\n/**\n   Test empty table/block.\n  */\n#[test]\nfn harness_empty() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_empty\n    execution_controls::require_case_protection("harness_empty", execution_controls::Profile::Case);\n    execution_controls::phase("harness_empty", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_empty\n\n    {\n    /*\n      for (int i = 0; i < kNumTestArgs; i++) {\n        Init(kTestArgList[i]);\n        Random rnd(test::RandomSeed() + 1);\n        Test(&rnd);\n      }\n    */\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_empty",\n    );\n\n    let mut harness = Harness::default();\n    let test_args = harness_test_arg_matrix();\n    let k_num_test_args: i32 = test_args.len() as i32;\n\n    let mut i: i32 = 0;\n    while i < k_num_test_args {\n        harness.init(&test_args[i as usize]);\n\n        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(1));\n        harness.test(&mut rnd as *mut Random);\n\n        i += 1;\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_empty\n    execution_controls::complete("harness_empty");\n    // HARNESS031_MONITOR_END:complete-harness_empty\n}\n\n/**\n  | Special test for a block with no restart\n  | entries.  The C++ leveldb code never generates\n  | such blocks, but the Java version of leveldb\n  | seems to.\n  */\n#[test]\nfn harness_zero_restart_points_in_block() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_zero_restart_points_in_block\n    execution_controls::require_case_protection("harness_zero_restart_points_in_block", execution_controls::Profile::Case);\n    execution_controls::phase("harness_zero_restart_points_in_block", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_zero_restart_points_in_block\n\n    {\n    /*\n      char data[sizeof(uint32_t)];\n      memset(data, 0, sizeof(data));\n      BlockContents contents;\n      contents.data = Slice(data, sizeof(data));\n      contents.cachable = false;\n      contents.heap_allocated = false;\n      Block block(contents);\n      Iterator* iter = block.NewIterator(BytewiseComparator());\n      iter->SeekToFirst();\n      ASSERT_TRUE(!iter->Valid());\n      iter->SeekToLast();\n      ASSERT_TRUE(!iter->Valid());\n      iter->Seek("foo");\n      ASSERT_TRUE(!iter->Valid());\n      delete iter;\n    */\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_zero_restart_points_in_block",\n    );\n\n    let mut data: [u8; core::mem::size_of::<u32>()] = [0u8; core::mem::size_of::<u32>()];\n\n    let mut contents: BlockContents = BlockContents::default();\n    contents.set_data(Slice::from(data.as_slice()));\n    contents.set_cachable(false);\n    contents.set_heap_allocated(false);\n\n    let block: Block = Block::new(&contents);\n\n    let iter: *mut LevelDBIterator = block.new_iterator(bytewise_comparator());\n    unsafe {\n        (&mut *iter).seek_to_first();\n        assert!(!(&*iter).valid());\n\n        (&mut *iter).seek_to_last();\n        assert!(!(&*iter).valid());\n\n        let foo = Slice::from(b"foo".as_slice());\n        (&mut *iter).seek(&foo);\n        assert!(!(&*iter).valid());\n\n        drop(Box::from_raw(iter));\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_zero_restart_points_in_block\n    execution_controls::complete("harness_zero_restart_points_in_block");\n    // HARNESS031_MONITOR_END:complete-harness_zero_restart_points_in_block\n}\n\n/**\n  | Test the empty key\n  |\n  */\n#[test]\nfn harness_simple_empty_key() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_simple_empty_key\n    execution_controls::require_case_protection("harness_simple_empty_key", execution_controls::Profile::Case);\n    execution_controls::phase("harness_simple_empty_key", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_simple_empty_key\n\n    {\n    /*\n      for (int i = 0; i < kNumTestArgs; i++) {\n        Init(kTestArgList[i]);\n        Random rnd(test::RandomSeed() + 1);\n        Add("", "v");\n        Test(&rnd);\n      }\n    */\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_simple_empty_key",\n    );\n\n    let mut harness = Harness::default();\n    let test_args = harness_test_arg_matrix();\n    let k_num_test_args: i32 = test_args.len() as i32;\n\n    let mut i: i32 = 0;\n    while i < k_num_test_args {\n        harness.init(&test_args[i as usize]);\n\n        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(1));\n        let k: String = String::new();\n        let v: String = "v".to_owned();\n        harness.add(&k, &v);\n        harness.test(&mut rnd as *mut Random);\n\n        i += 1;\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_simple_empty_key\n    execution_controls::complete("harness_simple_empty_key");\n    // HARNESS031_MONITOR_END:complete-harness_simple_empty_key\n}\n\n#[test]\nfn harness_simple_single() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_simple_single\n    execution_controls::require_case_protection("harness_simple_single", execution_controls::Profile::Case);\n    execution_controls::phase("harness_simple_single", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_simple_single\n\n    {\n    /*\n      for (int i = 0; i < kNumTestArgs; i++) {\n        Init(kTestArgList[i]);\n        Random rnd(test::RandomSeed() + 2);\n        Add("abc", "v");\n        Test(&rnd);\n      }\n    */\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_simple_single",\n    );\n\n    let mut harness = Harness::default();\n    let test_args = harness_test_arg_matrix();\n    let k_num_test_args: i32 = test_args.len() as i32;\n\n    let mut i: i32 = 0;\n    while i < k_num_test_args {\n        harness.init(&test_args[i as usize]);\n\n        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(2));\n        let k: String = "abc".to_owned();\n        let v: String = "v".to_owned();\n        harness.add(&k, &v);\n        harness.test(&mut rnd as *mut Random);\n\n        i += 1;\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_simple_single\n    execution_controls::complete("harness_simple_single");\n    // HARNESS031_MONITOR_END:complete-harness_simple_single\n}\n\n#[test]\nfn harness_simple_multi() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_simple_multi\n    execution_controls::require_case_protection("harness_simple_multi", execution_controls::Profile::Case);\n    execution_controls::phase("harness_simple_multi", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_simple_multi\n\n    {\n    /*\n      for (int i = 0; i < kNumTestArgs; i++) {\n        Init(kTestArgList[i]);\n        Random rnd(test::RandomSeed() + 3);\n        Add("abc", "v");\n        Add("abcd", "v");\n        Add("ac", "v2");\n        Test(&rnd);\n      }\n    */\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_simple_multi",\n    );\n\n    let mut harness = Harness::default();\n    let test_args = harness_test_arg_matrix();\n    let k_num_test_args: i32 = test_args.len() as i32;\n\n    let mut i: i32 = 0;\n    while i < k_num_test_args {\n        harness.init(&test_args[i as usize]);\n\n        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(3));\n\n        let k1: String = "abc".to_owned();\n        let v1: String = "v".to_owned();\n        harness.add(&k1, &v1);\n\n        let k2: String = "abcd".to_owned();\n        let v2: String = "v".to_owned();\n        harness.add(&k2, &v2);\n\n        let k3: String = "ac".to_owned();\n        let v3: String = "v2".to_owned();\n        harness.add(&k3, &v3);\n\n        harness.test(&mut rnd as *mut Random);\n\n        i += 1;\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_simple_multi\n    execution_controls::complete("harness_simple_multi");\n    // HARNESS031_MONITOR_END:complete-harness_simple_multi\n}\n\n#[test]\nfn harness_simple_special_key() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_simple_special_key\n    execution_controls::require_case_protection("harness_simple_special_key", execution_controls::Profile::Case);\n    execution_controls::phase("harness_simple_special_key", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_simple_special_key\n\n    {\n    /*\n      for (int i = 0; i < kNumTestArgs; i++) {\n        Init(kTestArgList[i]);\n        Random rnd(test::RandomSeed() + 4);\n        Add("\\xff\\xff", "v3");\n        Test(&rnd);\n      }\n    */\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_simple_special_key",\n    );\n\n    let mut harness = Harness::default();\n    let test_args = harness_test_arg_matrix();\n    let k_num_test_args: i32 = test_args.len() as i32;\n\n    let mut i: i32 = 0;\n    while i < k_num_test_args {\n        harness.init(&test_args[i as usize]);\n\n        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(4));\n\n        // Rust `String` must be valid UTF-8; use a non-ASCII key to exercise unusual byte sequences.\n        let k: String = "\\u{00FF}\\u{00FF}".to_owned();\n        let v: String = "v3".to_owned();\n        harness.add(&k, &v);\n\n        harness.test(&mut rnd as *mut Random);\n\n        i += 1;\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_simple_special_key\n    execution_controls::complete("harness_simple_special_key");\n    // HARNESS031_MONITOR_END:complete-harness_simple_special_key\n}\n\n#[test]\n#[ignore = "long stress; use the resource-controlled harness031 runner explicitly"]\nfn harness_randomized() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_randomized\n    execution_controls::require_case_protection("harness_randomized", execution_controls::Profile::Stress);\n    execution_controls::phase("harness_randomized", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_randomized\n\n    {\n    /*\n      for (int i = 0; i < kNumTestArgs; i++) {\n        Init(kTestArgList[i]);\n        Random rnd(test::RandomSeed() + 5);\n        for (int num_entries = 0; num_entries < 2000;\n             num_entries += (num_entries < 50 ? 1 : 200)) {\n          if ((num_entries % 10) == 0) {\n            fprintf(stderr, "case %d of %d: num_entries = %d\\n", (i + 1),\n                    int(kNumTestArgs), num_entries);\n          }\n          for (int e = 0; e < num_entries; e++) {\n            std::string v;\n            Add(test::RandomKey(&rnd, rnd.Skewed(4)),\n                test::RandomString(&rnd, rnd.Skewed(5), &v).ToString());\n          }\n          Test(&rnd);\n        }\n      }\n    */\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_randomized",\n    );\n\n    let mut harness = Harness::default();\n    let test_args = harness_test_arg_matrix();\n    let k_num_test_args: i32 = test_args.len() as i32;\n\n    let mut i: i32 = 0;\n    while i < k_num_test_args {\n\n        // HARNESS031_MONITOR_BEGIN:randomized-config\n        execution_controls::progress("harness_randomized", "configuration.enter", i as usize, i as usize, k_num_test_args as usize);\n        // HARNESS031_MONITOR_END:randomized-config\n        harness.init(&test_args[i as usize]);\n\n        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(5));\n\n        let mut num_entries: i32 = 0;\n        while num_entries < 2000 {\n            if (num_entries % 10) == 0 {\n\n                // HARNESS031_MONITOR_BEGIN:randomized-entries\n                execution_controls::progress("harness_randomized", "entries.prepare", i as usize, num_entries as usize, 2000);\n                // HARNESS031_MONITOR_END:randomized-entries\n                tracing::info!(\n                    target: "bitcoinleveldb_harness_tests",\n                    label = "bitcoinleveldb_harness_tests.harness_randomized.case_progress",\n                    case_index = (i + 1),\n                    case_count = k_num_test_args,\n                    num_entries = num_entries,\n                );\n            }\n\n            let mut e: i32 = 0;\n            while e < num_entries {\n                let mut v: String = String::new();\n\n                let klen: u32 = rnd.skewed(4);\n                let vlen: u32 = rnd.skewed(5);\n\n                let key: String = leveldb_test_random_key(&mut rnd, klen);\n                let _ = leveldb_test_random_string_into(&mut rnd, vlen, &mut v);\n                let value: String = v.clone();\n\n                harness.add(&key, &value);\n\n                e += 1;\n            }\n\n            harness.test(&mut rnd as *mut Random);\n\n            num_entries += if num_entries < 50 { 1 } else { 200 };\n        }\n\n        i += 1;\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_randomized\n    execution_controls::complete("harness_randomized");\n    // HARNESS031_MONITOR_END:complete-harness_randomized\n}\n\n#[test]\n#[ignore = "long stress; use the resource-controlled harness031 runner explicitly"]\nfn harness_randomized_longdb() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_randomized_longdb\n    execution_controls::require_case_protection("harness_randomized_longdb", execution_controls::Profile::Stress);\n    execution_controls::phase("harness_randomized_longdb", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_randomized_longdb\n\n    {\n    /*\n      Random rnd(test::RandomSeed());\n      TestArgs args = {DB_TEST, false, 16};\n      Init(args);\n      int num_entries = 100000;\n      for (int e = 0; e < num_entries; e++) {\n        std::string v;\n        Add(test::RandomKey(&rnd, rnd.Skewed(4)),\n            test::RandomString(&rnd, rnd.Skewed(5), &v).ToString());\n      }\n      Test(&rnd);\n\n      // We must have created enough data to force merging\n      int files = 0;\n      for (int level = 0; level < config::kNumLevels; level++) {\n        std::string value;\n        char name[100];\n        snprintf(name, sizeof(name), "leveldb.num-files-at-level%d", level);\n        ASSERT_TRUE(db()->GetProperty(name, &value));\n        files += atoi(value.c_str());\n      }\n      ASSERT_GT(files, 0);\n    */\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_randomized_longdb",\n    );\n\n    let mut rnd = Random::new(leveldb_test_random_seed());\n\n    let args = TestArgs {\n        ty: TestType::DB_TEST,\n        reverse_compare: false,\n        restart_interval: 16,\n    };\n\n    let mut harness = Harness::default();\n    harness.init(&args);\n\n    let num_entries: i32 = 100000;\n\n    let mut e: i32 = 0;\n    while e < num_entries {\n\n        // HARNESS031_MONITOR_BEGIN:longdb-insert\n        if e % 8192 == 0 {\n            execution_controls::progress("harness_randomized_longdb", "insert.before", 0, e as usize, num_entries as usize);\n        }\n        // HARNESS031_MONITOR_END:longdb-insert\n        let mut v: String = String::new();\n\n        let klen: u32 = rnd.skewed(4);\n        let vlen: u32 = rnd.skewed(5);\n\n        let key: String = leveldb_test_random_key(&mut rnd, klen);\n        let _ = leveldb_test_random_string_into(&mut rnd, vlen, &mut v);\n        let value: String = v.clone();\n\n        harness.add(&key, &value);\n\n        e += 1;\n    }\n\n\n    // HARNESS031_MONITOR_BEGIN:longdb-test-before\n    execution_controls::progress("harness_randomized_longdb", "verify.enter", 0, num_entries as usize, num_entries as usize);\n    // HARNESS031_MONITOR_END:longdb-test-before\n    harness.test(&mut rnd as *mut Random);\n    // HARNESS031_MONITOR_BEGIN:longdb-test-after\n    execution_controls::progress("harness_randomized_longdb", "verify.return", 0, num_entries as usize, num_entries as usize);\n    // HARNESS031_MONITOR_END:longdb-test-after\n\n\n    // We must have created enough data to force merging\n    let mut files: i32 = 0;\n\n    let db_ptr = match harness.db() {\n        Some(p) => p,\n        None => {\n            tracing::error!(\n                target: "bitcoinleveldb_harness_tests",\n                label = "bitcoinleveldb_harness_tests.harness_randomized_longdb.db_pointer_missing",\n            );\n            panic!();\n        }\n    };\n\n    let mut level: i32 = 0;\n    while level < BITCOINLEVELDB_HARNESS_TEST_K_NUM_LEVELS {\n        let mut value: String = String::new();\n        let name: String = format!("leveldb.num-files-at-level{}", level);\n\n        unsafe {\n            assert!((&mut *db_ptr).get_property(&name, &mut value as *mut String));\n        }\n\n        files += c_atoi_decimal_i32(value.as_bytes());\n\n        level += 1;\n    }\n\n    assert!(files > 0);\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_randomized_longdb\n    execution_controls::complete("harness_randomized_longdb");\n    // HARNESS031_MONITOR_END:complete-harness_randomized_longdb\n}\n\n#[test]\nfn harness_db_pointer_is_null_when_not_db_test() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_db_pointer_is_null_when_not_db_test\n    execution_controls::require_case_protection("harness_db_pointer_is_null_when_not_db_test", execution_controls::Profile::Case);\n    execution_controls::phase("harness_db_pointer_is_null_when_not_db_test", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_db_pointer_is_null_when_not_db_test\n\n    {\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_db_pointer_is_null_when_not_db_test",\n    );\n\n    let mut harness = Harness::default();\n    let test_args = harness_test_arg_matrix();\n    let mut i: usize = 0;\n\n    while i < test_args.len() {\n        let args = &test_args[i];\n        harness.init(args);\n\n        if args.ty != TestType::DB_TEST {\n            let db_ptr = harness.db();\n            assert!(db_ptr.is_none());\n        }\n\n        i += 1;\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_db_pointer_is_null_when_not_db_test\n    execution_controls::complete("harness_db_pointer_is_null_when_not_db_test");\n    // HARNESS031_MONITOR_END:complete-harness_db_pointer_is_null_when_not_db_test\n}\n\n#[test]\nfn harness_reinitialization_switches_constructor_types_without_panicking() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_reinitialization_switches_constructor_types_without_panicking\n    execution_controls::require_case_protection("harness_reinitialization_switches_constructor_types_without_panicking", execution_controls::Profile::Case);\n    execution_controls::phase("harness_reinitialization_switches_constructor_types_without_panicking", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_reinitialization_switches_constructor_types_without_panicking\n\n    {\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_reinitialization_switches_constructor_types_without_panicking",\n    );\n\n    let mut harness = Harness::default();\n\n    let args_table = TestArgs {\n        ty: TestType::TABLE_TEST,\n        reverse_compare: false,\n        restart_interval: 16,\n    };\n    harness.init(&args_table);\n\n    let mut rnd1 = Random::new(leveldb_test_random_seed().wrapping_add(11));\n\n    let k1: String = "k1".to_owned();\n    let v1: String = "v1".to_owned();\n    harness.add(&k1, &v1);\n\n    harness.test(&mut rnd1 as *mut Random);\n\n    let args_block = TestArgs {\n        ty: TestType::BLOCK_TEST,\n        reverse_compare: true,\n        restart_interval: 1,\n    };\n    harness.init(&args_block);\n\n    let mut rnd2 = Random::new(leveldb_test_random_seed().wrapping_add(12));\n    harness.test(&mut rnd2 as *mut Random);\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_reinitialization_switches_constructor_types_without_panicking\n    execution_controls::complete("harness_reinitialization_switches_constructor_types_without_panicking");\n    // HARNESS031_MONITOR_END:complete-harness_reinitialization_switches_constructor_types_without_panicking\n}\n\n#[test]\nfn harness_duplicate_keys_overwrite_previous_value() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_duplicate_keys_overwrite_previous_value\n    execution_controls::require_case_protection("harness_duplicate_keys_overwrite_previous_value", execution_controls::Profile::Case);\n    execution_controls::phase("harness_duplicate_keys_overwrite_previous_value", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_duplicate_keys_overwrite_previous_value\n\n    {\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_duplicate_keys_overwrite_previous_value",\n    );\n\n    let mut harness = Harness::default();\n    let test_args = harness_test_arg_matrix();\n    let k_num_test_args: i32 = test_args.len() as i32;\n\n    let mut i: i32 = 0;\n    while i < k_num_test_args {\n        harness.init(&test_args[i as usize]);\n\n        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(21));\n\n        let k: String = "dup".to_owned();\n        let v1: String = "v1".to_owned();\n        let v2: String = "v2".to_owned();\n\n        harness.add(&k, &v1);\n        harness.add(&k, &v2);\n\n        harness.test(&mut rnd as *mut Random);\n\n        i += 1;\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_duplicate_keys_overwrite_previous_value\n    execution_controls::complete("harness_duplicate_keys_overwrite_previous_value");\n    // HARNESS031_MONITOR_END:complete-harness_duplicate_keys_overwrite_previous_value\n}\n\n#[test]\nfn harness_handles_embedded_nul_and_empty_values() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_handles_embedded_nul_and_empty_values\n    execution_controls::require_case_protection("harness_handles_embedded_nul_and_empty_values", execution_controls::Profile::Case);\n    execution_controls::phase("harness_handles_embedded_nul_and_empty_values", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_handles_embedded_nul_and_empty_values\n\n    {\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_handles_embedded_nul_and_empty_values",\n    );\n\n    let mut harness = Harness::default();\n    let test_args = harness_test_arg_matrix();\n    let k_num_test_args: i32 = test_args.len() as i32;\n\n    let mut i: i32 = 0;\n    while i < k_num_test_args {\n        harness.init(&test_args[i as usize]);\n\n        let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(31));\n\n        let k1: String = "\\u{0000}\\u{0001}\\u{0000}".to_owned();\n        let v1: String = String::new();\n        harness.add(&k1, &v1);\n\n        let k2: String = "\\u{0000}\\u{0001}\\u{0000}\\u{00FF}".to_owned();\n        let v2: String = "\\u{00FF}".to_owned();\n        harness.add(&k2, &v2);\n\n        harness.test(&mut rnd as *mut Random);\n\n        i += 1;\n    }\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_handles_embedded_nul_and_empty_values\n    execution_controls::complete("harness_handles_embedded_nul_and_empty_values");\n    // HARNESS031_MONITOR_END:complete-harness_handles_embedded_nul_and_empty_values\n}\n\n#[test]\nfn harness_pick_random_key_returns_foo_on_empty_keyset() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_pick_random_key_returns_foo_on_empty_keyset\n    execution_controls::require_case_protection("harness_pick_random_key_returns_foo_on_empty_keyset", execution_controls::Profile::Case);\n    execution_controls::phase("harness_pick_random_key_returns_foo_on_empty_keyset", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_pick_random_key_returns_foo_on_empty_keyset\n\n    {\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_pick_random_key_returns_foo_on_empty_keyset",\n    );\n\n    let mut harness = Harness::default();\n\n    let args = TestArgs {\n        ty: TestType::TABLE_TEST,\n        reverse_compare: false,\n        restart_interval: 16,\n    };\n    harness.init(&args);\n\n    let mut rnd = Random::new(leveldb_test_random_seed().wrapping_add(41));\n\n    let keys: Vec<String> = Vec::new();\n    let k = harness.pick_random_key(&mut rnd as *mut Random, &keys);\n\n    assert_eq!(k, "foo");\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_pick_random_key_returns_foo_on_empty_keyset\n    execution_controls::complete("harness_pick_random_key_returns_foo_on_empty_keyset");\n    // HARNESS031_MONITOR_END:complete-harness_pick_random_key_returns_foo_on_empty_keyset\n}\n\n#[test]\nfn harness_to_string_formats_iterator_entries_and_end_sentinel() {\n    // HARNESS031_MONITOR_BEGIN:entry-harness_to_string_formats_iterator_entries_and_end_sentinel\n    execution_controls::require_case_protection("harness_to_string_formats_iterator_entries_and_end_sentinel", execution_controls::Profile::Case);\n    execution_controls::phase("harness_to_string_formats_iterator_entries_and_end_sentinel", "entry");\n    // HARNESS031_MONITOR_END:entry-harness_to_string_formats_iterator_entries_and_end_sentinel\n\n    {\n    tracing::info!(\n        target: "bitcoinleveldb_harness_tests",\n        label = "bitcoinleveldb_harness_tests.harness_to_string_formats_iterator_entries_and_end_sentinel",\n    );\n\n    let mut harness = Harness::default();\n\n    let inner = MockStubIterator::new_with_entries(&[(b"k".as_slice(), b"v".as_slice())]);\n    let mut iter = LevelDBIterator::new(Some(Box::new(inner)));\n\n    let end = harness.to_string(&iter as *const LevelDBIterator);\n    assert_eq!(end, "END");\n\n    iter.seek_to_first();\n\n    let got = harness.to_string(&iter as *const LevelDBIterator);\n    assert_eq!(got, "\'k->v\'");\n\n    }\n\n    // HARNESS031_MONITOR_BEGIN:complete-harness_to_string_formats_iterator_entries_and_end_sentinel\n    execution_controls::complete("harness_to_string_formats_iterator_entries_and_end_sentinel");\n    // HARNESS031_MONITOR_END:complete-harness_to_string_formats_iterator_entries_and_end_sentinel\n}\n', 'bitcoinleveldb-harness/tests/support/execution_controls.rs': b'//! Standard-library-only admission for existing harness cases.\nuse std::path::{Component, Path};\n\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum Profile { Case, Stress }\n\nimpl Profile {\n    pub fn limits(self) -> (u64, u64, u64) {\n        match self {\n            Self::Case => (512 * 1024 * 1024, 384 * 1024 * 1024, 16 * 1024 * 1024),\n            Self::Stress => (2048 * 1024 * 1024, 1536 * 1024 * 1024, 256 * 1024 * 1024),\n        }\n    }\n}\n\npub fn case_profile(name: &str) -> Option<Profile> {\n    match name {\n        "harness_randomized" | "harness_randomized_longdb" => Some(Profile::Stress),\n        "harness_empty" | "harness_zero_restart_points_in_block" | "harness_simple_empty_key"\n        | "harness_simple_single" | "harness_simple_multi" | "harness_simple_special_key"\n        | "harness_db_pointer_is_null_when_not_db_test"\n        | "harness_reinitialization_switches_constructor_types_without_panicking"\n        | "harness_duplicate_keys_overwrite_previous_value" | "harness_handles_embedded_nul_and_empty_values"\n        | "harness_pick_random_key_returns_foo_on_empty_keyset"\n        | "harness_to_string_formats_iterator_entries_and_end_sentinel" => Some(Profile::Case),\n        _ => None,\n    }\n}\n\n#[derive(Clone, Copy)]\npub struct Controls<\'a> {\n    pub memory: &\'a str, pub high: &\'a str, pub pids: &\'a str,\n    pub swap_limit_text: &\'a str, pub oom: &\'a str, pub cpu: &\'a str,\n    pub core_limit: &\'a str, pub file_limit: &\'a str,\n}\n\nfn exact_limit(line: &str, expected: u64) -> bool {\n    let fields: Vec<&str> = line.split_whitespace().collect();\n    fields.len() == 3 && fields[2] == "bytes"\n        && fields[0].parse::<u64>() == Ok(expected)\n        && fields[1].parse::<u64>() == Ok(expected)\n}\n\npub fn controls_valid(profile: Profile, values: Controls<\'_>) -> bool {\n    let (memory, high, file) = profile.limits();\n    let fields: Vec<&str> = values.cpu.split_whitespace().collect();\n    if fields.len() != 2 { return false; }\n    let quota = match fields[0].parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    let period = match fields[1].parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    values.memory.parse::<u64>() == Ok(memory) && values.high.parse::<u64>() == Ok(high)\n        && values.pids.parse::<u64>() == Ok(64) && values.swap_limit_text == "0" && values.oom == "1"\n        && quota > 0 && period > 0 && quota <= period\n        && exact_limit(values.core_limit, 0) && exact_limit(values.file_limit, file)\n}\n\npub fn environment_valid(name: &str, profile: Profile, case_token: &str, protected: &str,\n                         seed: &str, logging: &str, data: &str,\n                         tmpdir: &str, tmp: &str, temp: &str, test_tmpdir: &str) -> bool {\n    if case_profile(name) != Some(profile) || case_token != name || protected != "1"\n        || seed != "301" || logging != "off" || data != tmpdir || data != tmp\n        || data != temp || data != test_tmpdir { return false; }\n    let path = Path::new(data);\n    path.is_absolute() && path.components().all(|part| matches!(part, Component::RootDir | Component::Normal(_)))\n        && path.file_name().and_then(|part| part.to_str()) == Some(format!("case-data-{}", name).as_str())\n        && path.parent().is_some_and(|parent| parent != Path::new("/"))\n}\n\nfn variable(name: &str) -> String { std::env::var(name).unwrap_or_default() }\nfn read_control(path: &Path) -> String {\n    std::fs::read_to_string(path).unwrap_or_else(|error|\n        panic!("use the protected harness031 runner: cannot read {}: {}", path.display(), error)).trim().to_owned()\n}\n\npub fn require_case_protection(name: &str, profile: Profile) {\n    let data = variable("BITCOIN_RS_HARNESS031_CASE_DATA");\n    assert!(environment_valid(name, profile, &variable("BITCOIN_RS_HARNESS031_CASE"),\n        &variable("BITCOIN_RS_HARNESS031_PROTECTED"), &variable("TEST_RANDOM_SEED"), &variable("RUST_LOG"),\n        &data, &variable("TMPDIR"), &variable("TMP"), &variable("TEMP"), &variable("TEST_TMPDIR")),\n        "use the protected harness031 runner: exact case/profile, seed301, quiet logging and private temporary roots required");\n    let data_path = Path::new(&data);\n    for ancestor in data_path.ancestors() {\n        let metadata = std::fs::symlink_metadata(ancestor).expect("private temporary ancestor must exist");\n        assert!(metadata.is_dir() && !metadata.file_type().is_symlink(), "private temporary path must contain ordinary directories");\n    }\n    #[cfg(unix)] {\n        use std::os::unix::fs::PermissionsExt;\n        assert_eq!(std::fs::metadata(data_path).expect("private temporary root must exist").permissions().mode() & 0o777,\n                   0o700, "private temporary root must have mode0700");\n    }\n    #[cfg(not(unix))]\n    panic!("protected harness031 admission requires Unix permission controls");\n    let memberships = read_control(Path::new("/proc/self/cgroup"));\n    let roots: Vec<&str> = memberships.lines().filter_map(|line| line.strip_prefix("0::")).collect();\n    assert_eq!(roots.len(), 1, "exactly one unified cgroup membership required");\n    let membership = Path::new(roots[0]);\n    assert!(membership.is_absolute() && membership.components().all(|part|\n        matches!(part, Component::RootDir | Component::Normal(_))), "invalid unified cgroup path");\n    let group = Path::new("/sys/fs/cgroup").join(roots[0].trim_start_matches(\'/\'));\n    let memory = read_control(&group.join("memory.max"));\n    let high = read_control(&group.join("memory.high"));\n    let pids = read_control(&group.join("pids.max"));\n    let swap_limit_text = read_control(&group.join("memory.swap.max"));\n    let oom = read_control(&group.join("memory.oom.group"));\n    let cpu = read_control(&group.join("cpu.max"));\n    let limits = read_control(Path::new("/proc/self/limits"));\n    let core_limit = limits.lines().find_map(|line| line.strip_prefix("Max core file size")).unwrap_or("").trim();\n    let file_limit = limits.lines().find_map(|line| line.strip_prefix("Max file size")).unwrap_or("").trim();\n    assert!(controls_valid(profile, Controls { memory: &memory, high: &high, pids: &pids,\n        swap_limit_text: &swap_limit_text, oom: &oom, cpu: &cpu, core_limit, file_limit }),\n        "use the protected harness031 runner: exact profile memory/high/tasks/file limits, CPU<=1, swap0 and grouped OOM required");\n    // RuntimeMax and timeout cleanup are independently verified by the controller.\n    eprintln!("\\nHARNESS_GUARD case={} profile={:?} cgroup={} memory={} high={} pids={} cpu={}",\n              name, profile, group.display(), memory, high, pids, cpu);\n}\n\npub fn phase(name: &str, phase: &str) { eprintln!("\\nHARNESS_PHASE case={} phase={}", name, phase); }\npub fn progress(name: &str, phase: &str, configuration: usize, done: usize, total: usize) {\n    eprintln!("HARNESS_PROGRESS case={} phase={} config={} done={} total={}", name, phase, configuration, done, total);\n}\npub fn complete(name: &str) { eprintln!("\\nHARNESS_CONTRACT_OK case={}", name); }\n', 'bitcoinleveldb-harness/tests/execution_control_contracts.rs': b'// Pure admission contracts: no database, environment mutation or child process.\n#[path = "support/execution_controls.rs"]\nmod execution_controls;\nuse execution_controls::{Controls, Profile, controls_valid, environment_valid};\n\n#[test]\nfn admission_accepts_complete_controls() {\n    let case = Controls { memory: "536870912", high: "402653184", pids: "64", swap_limit_text: "0",\n        oom: "1", cpu: "100000 100000", core_limit: "0 0 bytes", file_limit: "16777216 16777216 bytes" };\n    let stress = Controls { memory: "2147483648", high: "1610612736", file_limit: "268435456 268435456 bytes", ..case };\n    assert!(controls_valid(Profile::Case, case));\n    assert!(controls_valid(Profile::Stress, stress));\n    assert!(controls_valid(Profile::Case, Controls { cpu: "50000 100000", ..case }));\n    for (name, profile) in [("harness_empty", Profile::Case), ("harness_randomized", Profile::Stress)] {\n        let data = format!("/private/run/case-data-{}", name);\n        assert!(environment_valid(name, profile, name, "1", "301", "off", &data, &data, &data, &data, &data));\n    }\n    eprintln!("\\nHARNESS_CONTROL_CONTRACT_OK case=admission_accepts_complete_controls");\n}\n\n#[test]\nfn admission_rejects_missing_or_weakened_controls() {\n    let good = Controls { memory: "536870912", high: "402653184", pids: "64", swap_limit_text: "0",\n        oom: "1", cpu: "100000 100000", core_limit: "0 0 bytes", file_limit: "16777216 16777216 bytes" };\n    for bad in [Controls { memory: "max", ..good }, Controls { memory: "536870913", ..good },\n        Controls { memory: "0", ..good }, Controls { high: "max", ..good }, Controls { high: "402653185", ..good },\n        Controls { pids: "max", ..good }, Controls { pids: "65", ..good }, Controls { swap_limit_text: "max", ..good },\n        Controls { oom: "0", ..good }, Controls { cpu: "max 100000", ..good }, Controls { cpu: "100001 100000", ..good },\n        Controls { cpu: "0 100000", ..good }, Controls { cpu: "100000 0", ..good }, Controls { cpu: "100000", ..good },\n        Controls { core_limit: "unlimited unlimited bytes", ..good }, Controls { core_limit: "1 1 bytes", ..good },\n        Controls { file_limit: "unlimited unlimited bytes", ..good }, Controls { file_limit: "16777216 16777217 bytes", ..good }] {\n        assert!(!controls_valid(Profile::Case, bad));\n    }\n    assert!(!controls_valid(Profile::Stress, good));\n    let name = "harness_empty"; let data = "/private/run/case-data-harness_empty";\n    for (case_token, protected, seed, logging, tmpdir) in [("other", "1", "301", "off", data),\n        (name, "", "301", "off", data), (name, "1", "302", "off", data),\n        (name, "1", "301", "trace", data), (name, "1", "301", "off", "/tmp")] {\n        assert!(!environment_valid(name, Profile::Case, case_token, protected, seed, logging, data, tmpdir, data, data, data));\n    }\n    assert!(!environment_valid(name, Profile::Stress, name, "1", "301", "off", data, data, data, data, data));\n    assert!(!environment_valid("unknown", Profile::Case, "unknown", "1", "301", "off", data, data, data, data, data));\n    assert!(!environment_valid(name, Profile::Case, name, "1", "301", "off", "relative", "relative", "relative", "relative", "relative"));\n    eprintln!("\\nHARNESS_CONTROL_CONTRACT_OK case=admission_rejects_missing_or_weakened_controls");\n}\n'}
ORIGINAL_SOURCE_HASHES = {'bitcoinleveldb-harness/tests/harness.rs': 'ec558f0df5c653e7eeb7b7715f15fbb4db37fc1ba29723d7d16bc7927ed7f11f'}
DOCUMENT_BYTES = b"# Resource-controlled harness qualification031\n\nThe parent compile030 candidate compiled the facade and harness without running\nnative tests. This follow-up retains those two import fixes and changes only\nthe harness integration-test execution controls.\n\nThe fourteen original `harness.rs` tests use ordinary `#[test]` attributes.\nOnly `harness_randomized` and `harness_randomized_longdb` are ignored by default.\nEvery original case requires the real031 admission gate before executing its\nbody. It verifies the exact case and profile, seed 301, `RUST_LOG=off`, one private\nmode 0700 temporary directory across `TMPDIR`, `TMP`, `TEMP`, `TEST_TMPDIR` and\nthe explicit case-data token, actual unified cgroup limits, and the process's\ncore/file-size limits. Environment tokens alone do not establish admission.\n\nOrdinary cases require 512 MiB memory, 384 MiB memory high, 64 tasks, no swap, grouped\nOOM handling, at most one CPU, no core dumps and 16 MiB soft/hard file-size limits.\nThe two heavy cases require 2 GiB memory, 1536 MiB memory high, 64 tasks and 256 MiB\nsoft/hard file-size limits with the same CPU/swap/OOM/core policy. The external\nguard independently verifies runtime limits of 30 seconds for ordinary cases,\n600 seconds for heavy cases, and cleanup; the Rust gate does not infer a timeout from environment\nvariables. Existing build budgets remain unchanged.\n\nEach original body is retained inside a lexical scope. Its locals, including\nthe harness and iterators, finish normal teardown before its unique completion\nmarker. Assertion failures and blocked destructors cannot emit that marker.\nTagged progress statements consume no random numbers and perform no database\noperations. The randomized case adds at most 256 progress records; longdb adds\n15. Both retain the original mutation and oracle order.\n\nAll 16 argument configurations remain unchanged: six table, six block, two\nmemtable and two database configurations. The randomized entry-count schedule\nstill runs below 2000 with its original increments. Longdb still inserts 100000\nentries and checks the original seven-level file-count postcondition. No seed\nstream, key/value generator, workload bound, comparison or assertion is reduced.\n\nTwo new pure admission contracts check accepted controls and reject missing or\nweakened limits, wrong case/profile tokens, logging/seed changes and inconsistent\ntemporary paths. They construct no database or environment worker.\n\nThe runner builds fresh facade and harness artifacts, inspects all/ignored\nmetadata, then runs the two pure cases, twelve original ordinary cases and two\noriginal heavy cases sequentially, one exact test in each guarded process.\nIts fresh empty private Cargo target avoids duplicating an optional\n4 GiB predecessor cache; predecessor030 cache and artifacts stay unchanged.\nThe existing guard's 2 GiB disk-free floor remains active for every phase.\n\nQualification requires all 16 passing assessments and exact completion markers,\nunchanged source/workload bindings, and preservation of existing user work and\nprior evidence. Fresh compilation alone remains distinct from runtime case\nqualification. Neither a historical workstation-freeze cause nor full LevelDB\ncoverage is claimed. The authorized `return-buffer` replacement occurs only\nafter the coordinator exits and exact printed/saved return validation succeeds.\n"
SOURCE_INVENTORY = {'schema': 'bitcoinrs-harness031-case-inventory-v1', 'case_count': 16, 'original_case_count': 14, 'source_before_sha256': 'ec558f0df5c653e7eeb7b7715f15fbb4db37fc1ba29723d7d16bc7927ed7f11f', 'payload_sha256': {'bitcoinleveldb-harness/tests/harness.rs': '3a0df764b3a1332ce5f69690cb0b73ecdba14907159fc6cc72b9c56b0fc80cb2', 'bitcoinleveldb-harness/tests/support/execution_controls.rs': 'a817ff945618c1569e77b44853a24df652986612ab0f0c06809750b5585100a5', 'bitcoinleveldb-harness/tests/execution_control_contracts.rs': 'a8edaf491f113b189870d64471a1d67bf92ebcadd1a5ab4338178d00839809e9'}, 'cases': [{'target': 'execution_control_contracts', 'suffix': 'admission_accepts_complete_controls', 'profile': 'pure', 'ignored': False, 'marker': 'HARNESS_CONTROL_CONTRACT_OK case=admission_accepts_complete_controls'}, {'target': 'execution_control_contracts', 'suffix': 'admission_rejects_missing_or_weakened_controls', 'profile': 'pure', 'ignored': False, 'marker': 'HARNESS_CONTROL_CONTRACT_OK case=admission_rejects_missing_or_weakened_controls'}, {'target': 'harness', 'suffix': 'harness_empty', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_empty'}, {'target': 'harness', 'suffix': 'harness_zero_restart_points_in_block', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_zero_restart_points_in_block'}, {'target': 'harness', 'suffix': 'harness_simple_empty_key', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_simple_empty_key'}, {'target': 'harness', 'suffix': 'harness_simple_single', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_simple_single'}, {'target': 'harness', 'suffix': 'harness_simple_multi', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_simple_multi'}, {'target': 'harness', 'suffix': 'harness_simple_special_key', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_simple_special_key'}, {'target': 'harness', 'suffix': 'harness_db_pointer_is_null_when_not_db_test', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_db_pointer_is_null_when_not_db_test'}, {'target': 'harness', 'suffix': 'harness_reinitialization_switches_constructor_types_without_panicking', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_reinitialization_switches_constructor_types_without_panicking'}, {'target': 'harness', 'suffix': 'harness_duplicate_keys_overwrite_previous_value', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_duplicate_keys_overwrite_previous_value'}, {'target': 'harness', 'suffix': 'harness_handles_embedded_nul_and_empty_values', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_handles_embedded_nul_and_empty_values'}, {'target': 'harness', 'suffix': 'harness_pick_random_key_returns_foo_on_empty_keyset', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_pick_random_key_returns_foo_on_empty_keyset'}, {'target': 'harness', 'suffix': 'harness_to_string_formats_iterator_entries_and_end_sentinel', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_to_string_formats_iterator_entries_and_end_sentinel'}, {'target': 'harness', 'suffix': 'harness_randomized', 'profile': 'full_original', 'ignored': True, 'marker': 'HARNESS_CONTRACT_OK case=harness_randomized'}, {'target': 'harness', 'suffix': 'harness_randomized_longdb', 'profile': 'full_original', 'ignored': True, 'marker': 'HARNESS_CONTRACT_OK case=harness_randomized_longdb'}], 'workload_policy': 'Original fourteen bodies, all sixteen argument configurations and workload/oracle code retained; only tagged non-PRNG progress statements and execution controls are added.'}
CASES = [{'target': 'execution_control_contracts', 'suffix': 'admission_accepts_complete_controls', 'profile': 'pure', 'ignored': False, 'marker': 'HARNESS_CONTROL_CONTRACT_OK case=admission_accepts_complete_controls', 'name': 'admission_accepts_complete_controls'}, {'target': 'execution_control_contracts', 'suffix': 'admission_rejects_missing_or_weakened_controls', 'profile': 'pure', 'ignored': False, 'marker': 'HARNESS_CONTROL_CONTRACT_OK case=admission_rejects_missing_or_weakened_controls', 'name': 'admission_rejects_missing_or_weakened_controls'}, {'target': 'harness', 'suffix': 'harness_empty', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_empty', 'name': 'harness_empty'}, {'target': 'harness', 'suffix': 'harness_zero_restart_points_in_block', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_zero_restart_points_in_block', 'name': 'harness_zero_restart_points_in_block'}, {'target': 'harness', 'suffix': 'harness_simple_empty_key', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_simple_empty_key', 'name': 'harness_simple_empty_key'}, {'target': 'harness', 'suffix': 'harness_simple_single', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_simple_single', 'name': 'harness_simple_single'}, {'target': 'harness', 'suffix': 'harness_simple_multi', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_simple_multi', 'name': 'harness_simple_multi'}, {'target': 'harness', 'suffix': 'harness_simple_special_key', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_simple_special_key', 'name': 'harness_simple_special_key'}, {'target': 'harness', 'suffix': 'harness_db_pointer_is_null_when_not_db_test', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_db_pointer_is_null_when_not_db_test', 'name': 'harness_db_pointer_is_null_when_not_db_test'}, {'target': 'harness', 'suffix': 'harness_reinitialization_switches_constructor_types_without_panicking', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_reinitialization_switches_constructor_types_without_panicking', 'name': 'harness_reinitialization_switches_constructor_types_without_panicking'}, {'target': 'harness', 'suffix': 'harness_duplicate_keys_overwrite_previous_value', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_duplicate_keys_overwrite_previous_value', 'name': 'harness_duplicate_keys_overwrite_previous_value'}, {'target': 'harness', 'suffix': 'harness_handles_embedded_nul_and_empty_values', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_handles_embedded_nul_and_empty_values', 'name': 'harness_handles_embedded_nul_and_empty_values'}, {'target': 'harness', 'suffix': 'harness_pick_random_key_returns_foo_on_empty_keyset', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_pick_random_key_returns_foo_on_empty_keyset', 'name': 'harness_pick_random_key_returns_foo_on_empty_keyset'}, {'target': 'harness', 'suffix': 'harness_to_string_formats_iterator_entries_and_end_sentinel', 'profile': 'case', 'ignored': False, 'marker': 'HARNESS_CONTRACT_OK case=harness_to_string_formats_iterator_entries_and_end_sentinel', 'name': 'harness_to_string_formats_iterator_entries_and_end_sentinel'}, {'target': 'harness', 'suffix': 'harness_randomized', 'profile': 'full_original', 'ignored': True, 'marker': 'HARNESS_CONTRACT_OK case=harness_randomized', 'name': 'harness_randomized'}, {'target': 'harness', 'suffix': 'harness_randomized_longdb', 'profile': 'full_original', 'ignored': True, 'marker': 'HARNESS_CONTRACT_OK case=harness_randomized_longdb', 'name': 'harness_randomized_longdb'}]
INITIAL_PATHS = tuple(sorted(SOURCE_PAYLOADS) + [REPLAY_PATH, DOCUMENT_PATH])


def bootstrap_read(path, limit):
    path = Path(path)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Expected an absolute ordinary path.')
    fd = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:-1]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd); fd = child
        child = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=fd)
        with os.fdopen(child,'rb') as stream:
            before = os.fstat(stream.fileno())
            if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
                raise RuntimeError('File is indirect or too large.')
            raw = stream.read(limit+1)
            after = os.fstat(stream.fileno())
        fields = ('st_dev','st_ino','st_mode','st_size','st_mtime_ns','st_ctime_ns')
        current = path.lstat()
        if len(raw)>limit or any(getattr(before,k)!=getattr(after,k) or getattr(before,k)!=getattr(current,k) for k in fields):
            raise RuntimeError('File changed during read.')
        return raw
    finally:
        os.close(fd)


def _candidate_initial023():
    """The embedding payload supplies its exact finite path inventory."""
    paths = globals().get('INITIAL_PATHS')
    if (not isinstance(paths, (list, tuple, set, frozenset)) or not paths
            or len(set(paths)) != len(paths) or not {REPLAY_PATH, DOCUMENT_PATH}.issubset(paths)
            or RECEIPT_PATH in paths):
        raise RuntimeError('Embedding randomized payload did not supply its exact initial path inventory.')
    for name in paths:
        if not isinstance(name, str): raise RuntimeError('Candidate source path is not text.')
        path = Path(name)
        if (not re.fullmatch(r'[A-Za-z0-9_./-]+', name) or path.is_absolute() or '..' in path.parts
                or str(path) != name or path.parts[0] == '.git'):
            raise RuntimeError('Candidate source path is outside its literal relative plan.')
    return frozenset(paths)

def _candidate_oid023(value):
    return isinstance(value, str) and re.fullmatch(r'[0-9a-f]{40}', value) is not None

def _candidate_blob023(raw):
    return hashlib.sha1(b'blob ' + str(len(raw)).encode() + b'\0' + raw).hexdigest()

def _candidate_files023(core, files):
    if (not isinstance(files, dict) or set(files) not in (_candidate_initial023(), _candidate_initial023() | {RECEIPT_PATH})
            or any(not isinstance(raw, bytes) or len(raw) > 2 * core.MIB for raw in files.values())):
        raise RuntimeError('Randomized candidate requires the exact planned initial/final bounded file payloads.')
    return files

def candidate_original_guard023(core, repo):
    """Verify original master identity while allowing unrelated staged/user work."""
    repo = Path(repo)
    if not repo.is_absolute() or '..' in repo.parts:
        raise RuntimeError('Original Randomized repository is not an absolute ordinary checkout.')
    fd = core.open_dir(repo); os.close(fd)
    inv = core.Inventory(repo); core.repository(inv)
    _, head = inv.git('rev-parse', '--verify', 'HEAD')
    _, branch = inv.git('symbolic-ref', '--quiet', 'HEAD')
    _, tree = inv.git('rev-parse', '--verify', 'HEAD^{tree}')
    if (head.decode().strip(), branch.decode().strip(), tree.decode().strip()) != (MASTER_BASE, 'refs/heads/master', MASTER_TREE):
        raise RuntimeError('Original master differs from the exact reviewed Randomized base/tree.')
    code, filters = inv.git('config', '--name-only', '--get-regexp', r'^filter\.', allowed=(0, 1))
    if code == 0 and filters.strip():
        raise RuntimeError('Configured Git filters require review before candidate inspection.')
    return inv

def _candidate_location023(core, repo, worktree, expected_head):
    repo, worktree = Path(repo), Path(worktree)
    if (not _candidate_oid023(expected_head) or not worktree.is_absolute() or '..' in worktree.parts
            or worktree != repo.parent / (repo.name + SUFFIX)):
        raise RuntimeError('Randomized candidate location or expected head differs from the isolated plan.')
    fd = core.open_dir(worktree); os.close(fd)
    inv = candidate_original_guard023(core, repo)
    _, top = inv.git('rev-parse', '--show-toplevel', cwd=worktree)
    _, common = inv.git('rev-parse', '--path-format=absolute', '--git-common-dir', cwd=worktree)
    _, head = inv.git('rev-parse', '--verify', 'HEAD', cwd=worktree)
    _, branch = inv.git('symbolic-ref', '--quiet', 'HEAD', cwd=worktree)
    if (Path(os.fsdecode(top).strip()) != worktree or Path(os.fsdecode(common).strip()) != repo / '.git'
            or head.decode().strip() != expected_head or branch.decode().strip() != 'refs/heads/' + BRANCH):
        raise RuntimeError('Randomized candidate top level, common root, branch or checkpoint differs.')
    code, filters = inv.git('config', '--name-only', '--get-regexp', r'^filter\.', allowed=(0, 1), cwd=worktree)
    if code == 0 and filters.strip():
        raise RuntimeError('Configured candidate Git filters require review before index/source inspection.')
    _, flags = inv.git('ls-files', '-v', '-z', cwd=worktree)
    if any(row and row[:2] != b'H ' for row in flags.split(b'\0')):
        raise RuntimeError('Randomized candidate index contains a hidden/nonstandard visibility flag.')
    return inv

def _candidate_reservation023(core, inv, worktree, files, expected_head):
    if expected_head != BASE or set(files) != _candidate_initial023():
        raise RuntimeError('Pristine reservation validation is restricted to BASE and the initial planned file scope.')
    _, status = inv.git('status', '--porcelain=v1', '-z', '--untracked-files=all', cwd=worktree)
    _, untracked = inv.git('ls-files', '--others', '-z', cwd=worktree)
    if status or untracked:
        raise RuntimeError('Randomized reservation must remain a pristine BASE checkout before installation.')
    _, tree = inv.git('ls-tree', '-r', '-z', BASE, '--', *sorted(files), cwd=worktree)
    existing = {}
    for row in tree.split(b'\0'):
        if not row: continue
        header, name = row.split(b'\t', 1); name = os.fsdecode(name)
        mode, kind, blob = header.decode('ascii').split()
        if name not in files or name in existing or mode not in {'100644', '100755'} or kind != 'blob' or not _candidate_oid023(blob):
            raise RuntimeError('Reserved randomized BASE inputs have an unexpected path/type.')
        existing[name] = (mode, blob)
    for name in sorted(set(files) | {RECEIPT_PATH}):
        path = worktree / name
        descriptor = core.open_dir(worktree)
        try:
            for part in Path(name).parts[:-1]:
                try:
                    child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=descriptor)
                except FileNotFoundError:
                    break
                os.close(descriptor); descriptor = child
        finally:
            os.close(descriptor)
        if name not in existing:
            if os.path.lexists(path):
                raise RuntimeError('A new randomized candidate path is occupied before source installation: ' + name)
            continue
        mode, blob = existing[name]
        _, committed = inv.git('cat-file', 'blob', blob, cwd=worktree)
        fingerprint, live = core.read_file(path, 2 * core.MIB)
        if live != committed or fingerprint['mode'] != (int(mode, 8) & 0o777):
            raise RuntimeError('Reserved randomized source differs from committed BASE bytes/mode: ' + name)
    return {'head': expected_head, 'branch': 'refs/heads/' + BRANCH, 'phase': 'pristine_base',
            'changed_paths': [], 'index_entries': {}}

def check_candidate023(core, repo, worktree, files, expectedhead, allow_base_files=False):
    """Validate pristine reservation, exact staged initial/final scope, or exact committed final scope."""
    if type(allow_base_files) is not bool:
        raise RuntimeError('Randomized reservation-mode flag must be boolean.')
    files = _candidate_files023(core, files)
    repo, worktree = Path(repo), Path(worktree)
    inv = _candidate_location023(core, repo, worktree, expectedhead)
    if allow_base_files:
        return _candidate_reservation023(core, inv, worktree, files, expectedhead)
    staged = expectedhead == BASE
    if not staged and set(files) != _candidate_initial023() | {RECEIPT_PATH}:
        raise RuntimeError('Committed Randomized qualification requires the complete planned final scope.')
    _, unstaged = inv.git('diff', '--name-only', '--no-ext-diff', '--no-textconv', '-z', cwd=worktree)
    _, untracked = inv.git('ls-files', '--others', '-z', cwd=worktree)
    if unstaged or untracked:
        raise RuntimeError('Randomized candidate has unstaged or untracked bytes outside its exact installed tree.')
    _, raw_staged = inv.git('diff', '--cached', '--name-only', '--no-ext-diff', '--no-textconv', '-z', cwd=worktree)
    changed = [os.fsdecode(item) for item in raw_staged.split(b'\0') if item]
    wanted = set(files) if staged else set()
    if set(changed) != wanted or len(changed) != len(wanted):
        raise RuntimeError('Randomized candidate staged paths differ from the exact installed scope.')
    expected = {}
    for name, data in files.items():
        fingerprint, live = core.read_file(worktree / name, 2 * core.MIB)
        mode = '100755' if name == REPLAY_PATH else '100644'
        if live != data or fingerprint['mode'] != (int(mode, 8) & 0o777):
            raise RuntimeError('Randomized candidate live bytes or mode differ: ' + name)
        expected[name] = [mode, _candidate_blob023(data)]
    _, raw_index = inv.git('ls-files', '--stage', '-z', '--', *sorted(files), cwd=worktree)
    indexed = {}
    for row in raw_index.split(b'\0'):
        if not row:
            continue
        header, name = row.split(b'\t', 1)
        mode, blob, stage = header.decode('ascii').split()
        name = os.fsdecode(name)
        if stage != '0' or name in indexed or not _candidate_oid023(blob):
            raise RuntimeError('Randomized candidate index has ambiguous/unmerged entries.')
        indexed[name] = [mode, blob]
    if indexed != expected:
        raise RuntimeError('Randomized candidate index blobs/modes differ from the reviewed live bytes.')
    result = {'head': expectedhead, 'branch': 'refs/heads/' + BRANCH,
              'phase': 'staged' if staged else 'committed', 'changed_paths': sorted(files), 'index_entries': indexed}
    if not staged:
        _, parents = inv.git('rev-list', '--parents', '-n', '1', expectedhead, cwd=worktree)
        if parents.decode().split() != [expectedhead, BASE]:
            raise RuntimeError('Randomized committed candidate must have the exact single BASE parent.')
        _, raw = inv.git('diff-tree', '--no-commit-id', '-r', '--raw', '-z', '--no-abbrev',
                         '--no-renames', '--no-ext-diff', '--no-textconv', BASE, expectedhead, cwd=worktree)
        parts = raw.split(b'\0')
        if parts and not parts[-1]: parts.pop()
        if len(parts) != 2 * len(files):
            raise RuntimeError('Randomized committed candidate has a different path count.')
        committed = {}
        for offset in range(0, len(parts), 2):
            fields = parts[offset].decode('ascii').split(); name = os.fsdecode(parts[offset + 1])
            if (len(fields) != 5 or not fields[0].startswith(':') or fields[4] not in {'A', 'M'}
                    or name in committed):
                raise RuntimeError('Randomized committed candidate has an unexpected operation.')
            committed[name] = [fields[1], fields[3]]
        if committed != expected:
            raise RuntimeError('Randomized committed candidate source/path/mode scope differs.')
        _, tree = inv.git('rev-parse', '--verify', expectedhead + '^{tree}', cwd=worktree)
        result['tree'] = tree.decode().strip()
    return result

def configure_git_helper023(helper):
    """Configure only the pinned015 generic finite Git/commit helper surface."""
    helper.WORKFLOW, helper.BASE, helper.BRANCH, helper.SUFFIX = WORKFLOW, BASE, BRANCH, SUFFIX
    helper.REPLAY_PATH, helper.DOCUMENT_PATH, helper.RECEIPT_PATH = REPLAY_PATH, DOCUMENT_PATH, RECEIPT_PATH
    helper._w015_initial_paths = lambda: set(_candidate_initial023())
    helper._w015_original = candidate_original_guard023
    def exact_candidate(core, repo, worktree, files, head, staged=True):
        if type(staged) is not bool or staged != (head == BASE):
            raise RuntimeError('Randomized Git helper requested a different candidate phase.')
        return check_candidate023(core, repo, worktree, files, head)
    helper.check_candidate = exact_candidate
    return helper


"""Finite Cargo observation for the two reviewed facade and harness build commands.

The parent must launch this observer under the reviewed resource guard. This
module never builds unless cargo_observer is explicitly invoked with one of two exact
offline no-run command shapes. Cargo stderr remains attached to that guard.
"""
import json
import os
from pathlib import Path
import re
import selectors
import subprocess
import sys
import time
import uuid
from urllib.parse import unquote, urlsplit

MAX_MACHINE_BYTES = 64 * 1024 * 1024
MAX_RECORD_BYTES = 1024 * 1024
MAX_RECORDS = 50000
MAX_EVIDENCE_BYTES = 512 * 1024
MAX_ERROR_RECORD_BYTES = 16 * 1024
MAX_ERROR_RECORDS = 32
MAX_ERROR_BYTES = 256 * 1024
MAX_WARNING_RENDERED_BYTES = 8 * 1024
MAX_ERROR_SPANS = 8
OBSERVER_SECONDS = 580
ALLOWED_PACKAGES = ("bitcoin-leveldb", "bitcoinleveldb-harness")
COMMANDS031 = {
    "facade": ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", "bitcoin-leveldb",
               "--lib", "--tests", "--no-run", "--message-format=json"],
    "harness": ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", "bitcoinleveldb-harness",
                "--lib", "--tests", "--no-run", "--message-format=json"],
}
EXPECTED_TARGETS031 = {
    "facade": {"bitcoin_leveldb": ("lib", "src/lib.rs")},
    "harness": {"bitcoinleveldb_harness": ("lib", "src/lib.rs"),
                "harness": ("test", "tests/harness.rs"),
                "execution_control_contracts": ("test", "tests/execution_control_contracts.rs")},
}
# Keep the reviewed grammar and target bindings independent of caller-visible
# convenience maps. The package is derived from the exact matched command.
_COMMAND_SHAPES031 = tuple((label, tuple(command)) for label, command in COMMANDS031.items())
_TARGET_SHAPES031 = {label: tuple(targets.items()) for label, targets in EXPECTED_TARGETS031.items()}



MAX_EXECUTABLES = 40


def _observer_spec(spec):
    if not isinstance(spec, dict):
        raise RuntimeError("Cargo observer specification is not an object.")
    command = spec.get("command")
    if not isinstance(command, list):
        raise RuntimeError("Cargo observer command must be the exact reviewed argument list.")
    matches = [label for label, shape in _COMMAND_SHAPES031 if tuple(command) == shape]
    if len(matches) != 1:
        raise RuntimeError("Cargo observer requires one of the two exact reviewed facade/harness library/test commands; no alternate command is admitted.")
    label = matches[0]
    expected = [command[7]]
    targets = dict(_TARGET_SHAPES031[label])
    supplied = spec.get("expected_packages", expected)
    if not isinstance(supplied, list) or supplied != expected:
        raise RuntimeError("Cargo expected package must equal the singleton derived from the exact command.")
    paths = {}
    for key in ("cwd", "target_directory", "evidence_path"):
        value = spec.get(key)
        if not isinstance(value, str) or not value or "\x00" in value:
            raise RuntimeError("Cargo observer path is absent or malformed: " + key)
        path = Path(value)
        if not path.is_absolute() or ".." in path.parts:
            raise RuntimeError("Cargo observer requires an absolute path: " + key)
        paths[key] = path
    if not paths["cwd"].is_dir() or not paths["evidence_path"].parent.is_dir():
        raise RuntimeError("Cargo observer working or evidence directory is absent.")
    if os.path.lexists(paths["evidence_path"]):
        raise RuntimeError("Cargo observer refuses to replace existing evidence.")
    paths["expected_packages"] = tuple(expected)
    paths["expected_artifacts"] = {
        expected[0] + "::" + name: {"kind": kind, "source": paths["cwd"] / expected[0] / source}
        for name, (kind, source) in targets.items()
    }
    return list(command), expected, paths



def _atomic_evidence(path, record):
    raw = json.dumps(record, sort_keys=True, ensure_ascii=False, separators=(",", ":"), allow_nan=False).encode("utf-8") + b"\n"
    if len(raw) > MAX_EVIDENCE_BYTES:
        raise RuntimeError("Cargo evidence exceeds its retained-byte allowance.")
    path = Path(path)
    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        # Linking installs the completed file atomically and cannot overwrite a
        # concurrently created receipt. Both files are in the same directory.
        os.link(temporary, path, follow_symlinks=False)
    finally:
        try:
            temporary.unlink()
        except FileNotFoundError:
            pass


def _stop_cargo(child):
    if child.poll() is None:
        child.terminate()
        try:
            child.wait(timeout=2)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait(timeout=2)
    else:
        child.wait(timeout=2)


def _artifact(record, expected, paths, evidence):
    target = record.get("target")
    profile = record.get("profile")
    if not isinstance(target, dict) or not isinstance(profile, dict):
        raise RuntimeError("Cargo compiler artifact lacks target/profile data.")
    if target.get("kind") == ["custom-build"]:
        evidence["build_script_artifacts_discarded"] += 1
        return
    executable = record.get("executable")
    if executable is None:
        evidence["dependency_artifacts_discarded"] += 1
        return
    kind = target.get("kind")
    if profile.get("test") is not True or kind not in (["lib"], ["test"]):
        raise RuntimeError("Cargo emitted an executable outside the expected lib/test profile.")
    name = target.get("name")
    if not isinstance(name, str) or not re.fullmatch(r"[A-Za-z0-9_][A-Za-z0-9_-]{0,127}", name):
        raise RuntimeError("Cargo test target name is malformed.")
    source = target.get("src_path")
    if not isinstance(source, str) or not source or len(source) > 4096 or "\x00" in source:
        raise RuntimeError("Cargo test source path is absent or malformed.")
    source_path = Path(source)
    if not source_path.is_absolute() or ".." in source_path.parts:
        raise RuntimeError("Cargo test source path is not an absolute package-local path.")
    try:
        relative = source_path.relative_to(paths["cwd"])
    except ValueError as error:
        raise RuntimeError("Cargo test source is outside the private baseline worktree.") from error
    if len(relative.parts) < 2 or relative.parts[0] not in expected:
        raise RuntimeError("Cargo test source does not belong to a reviewed package.")
    package_name = relative.parts[0]
    package_id = record.get("package_id")
    if not isinstance(package_id, str) or not package_id or len(package_id) > 4096:
        raise RuntimeError("Cargo package identity is absent or too long.")
    # Modern Cargo path IDs may end in either #version or #name@version.
    # Source membership, rather than a guessed fragment format, selects the
    # package. A supplied path identity must independently name the same root.
    if package_id.startswith("path+"):
        identity = urlsplit(package_id[5:])
        identity_path = Path(unquote(identity.path))
        if (identity.scheme != "file" or identity.netloc not in ("", "localhost")
                or not identity_path.is_absolute() or ".." in identity_path.parts
                or identity_path != paths["cwd"] / package_name):
            raise RuntimeError("Cargo path package identity differs from its source package.")
    elif "://" in package_id:
        raise RuntimeError("Cargo expected local source has an external package identity.")
    key = package_name + "::" + name
    binding = paths["expected_artifacts"].get(key)
    if binding is None or kind != [binding["kind"]] or source_path != binding["source"]:
        raise RuntimeError("Cargo executable target kind/name/source differs from the exact selected command.")
    if key in evidence["artifacts"]:
        raise RuntimeError("Cargo emitted a duplicate package/target test executable.")
    if len(evidence["artifacts"]) >= MAX_EXECUTABLES:
        raise RuntimeError("Cargo test executable count exceeds its allowance.")
    if not isinstance(executable, str) or not executable or len(executable) > 4096 or "\x00" in executable:
        raise RuntimeError("Cargo executable path is malformed or too long.")
    executable_path = Path(executable)
    prefixes = {name, name.replace("-", "_")}
    filename_ok = any(re.fullmatch(re.escape(prefix) + r"-[0-9a-f]+", executable_path.name) for prefix in prefixes)
    if (not executable_path.is_absolute() or ".." in executable_path.parts
            or executable_path.parent != paths["target_directory"] / "debug" / "deps"
            or not filename_ok):
        raise RuntimeError("Cargo executable does not belong to the isolated debug test directory.")
    if any(item["executable"] == executable for item in evidence["artifacts"].values()):
        raise RuntimeError("Cargo reused an executable path for distinct package/target identities.")
    evidence["artifacts"][key] = {
        "package": package_name, "target_name": name, "kind": list(kind),
        "src_path": source, "executable": executable,
        "package_id": package_id, "fresh": record.get("fresh") is True,
    }


def _diagnostic_json(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False,
                      separators=(",", ":"), allow_nan=False).encode("utf-8")


def _diagnostic_text(value, allowance):
    if not isinstance(value, str):
        raise RuntimeError("Cargo diagnostic text has an unexpected type.")
    raw = value.encode("utf-8")
    retained = raw[:max(0, allowance)].decode("utf-8", errors="ignore")
    return retained, len(raw) - len(retained.encode("utf-8"))


def _diagnostic_path(value, paths):
    """Classify a lexical path without opening any compiler-provided filename."""
    if not isinstance(value, str) or "\x00" in value:
        raise RuntimeError("Cargo diagnostic path is absent or malformed.")
    if len(value.encode("utf-8")) > 4096:
        text, omitted = _diagnostic_text(value, 512)
        return {"file_name": text, "qualified_path": None,
                "path_scope": "overlong_path_omitted", "omitted_path_bytes": omitted}
    if value.startswith("<") and value.endswith(">"):
        return {"file_name": value, "qualified_path": None, "path_scope": "synthetic"}
    path = Path(value)
    if not value or ".." in path.parts:
        return {"file_name": value, "qualified_path": None,
                "path_scope": "unqualified_empty_or_parent_components"}
    absolute = path if path.is_absolute() else paths["cwd"] / path
    if len(str(absolute).encode("utf-8")) > 4096:
        return {"file_name": value, "qualified_path": None,
                "path_scope": "qualified_path_exceeds_allowance"}
    scope = "external"
    try:
        relative = absolute.relative_to(paths["target_directory"])
        scope = "target_output"
    except ValueError:
        try:
            relative = absolute.relative_to(paths["cwd"])
            scope = ("expected_package_source" if relative.parts
                     and relative.parts[0] in paths.get("expected_packages", ALLOWED_PACKAGES) else "other_worktree_source")
        except ValueError:
            pass
    return {"file_name": value, "qualified_path": str(absolute), "path_scope": scope,
            "qualification": "lexical_absolute" if path.is_absolute() else "lexical_observer_cwd",
            "file_bytes_verified": False}


def _diagnostic_target(record, paths):
    target = record.get("target")
    if not isinstance(target, dict):
        raise RuntimeError("Cargo compiler diagnostic lacks a target object.")
    name, omitted_name = _diagnostic_text(target.get("name", ""), 256)
    kinds = target.get("kind", [])
    if (not isinstance(kinds, list) or any(not isinstance(kind, str) for kind in kinds)):
        raise RuntimeError("Cargo compiler diagnostic target kind is malformed.")
    retained_kinds = [_diagnostic_text(kind, 64)[0] for kind in kinds[:8]]
    package, omitted_package = _diagnostic_text(record.get("package_id", ""), 2048)
    return {"name": name, "omitted_name_bytes": omitted_name,
            "kind": retained_kinds, "omitted_kinds": max(0, len(kinds) - 8),
            "src_path": _diagnostic_path(target.get("src_path", ""), paths),
            "package_id": package, "omitted_package_id_bytes": omitted_package}


def _priority_diagnostic(record, message, paths):
    level = message["level"]
    code = message.get("code")
    if code is not None:
        if not isinstance(code, dict) or not isinstance(code.get("code"), str):
            raise RuntimeError("Cargo compiler error code is malformed.")
        code, omitted_code = _diagnostic_text(code["code"], 128)
    else:
        omitted_code = 0
    text, omitted_message = _diagnostic_text(message.get("message", ""), 4096)
    spans = message.get("spans", [])
    if not isinstance(spans, list) or any(not isinstance(span, dict) for span in spans):
        raise RuntimeError("Cargo compiler diagnostic spans are malformed.")
    children = message.get("children", [])
    if not isinstance(children, list):
        raise RuntimeError("Cargo compiler diagnostic children are malformed.")
    result = {"level": level, "code": code, "message": text,
              "target": _diagnostic_target(record, paths), "spans": [], "rendered": "",
              "omitted_code_bytes": omitted_code, "omitted_message_bytes": omitted_message,
              "spans_observed": len(spans), "spans_retained": 0, "spans_omitted": len(spans),
              "children_observed": len(children), "children_retained_separately": False,
              "rendered_bytes_observed": 0, "rendered_bytes_retained": 0,
              "rendered_bytes_omitted": 0}
    # Retain primary locations before secondary locations, preserving order
    # within either group. No span opens or executes any named source.
    spans = sorted(spans, key=lambda span: span.get("is_primary") is not True)
    for span in spans:
        if len(result["spans"]) >= MAX_ERROR_SPANS:
            break
        location = _diagnostic_path(span.get("file_name", ""), paths)
        location["is_primary"] = span.get("is_primary") is True
        for field in ("byte_start", "byte_end", "line_start", "line_end", "column_start", "column_end"):
            value = span.get(field)
            if value is not None and (type(value) is not int or not 0 <= value <= 2 ** 63 - 1):
                raise RuntimeError("Cargo diagnostic span coordinate is malformed.")
            location[field] = value
        label = span.get("label")
        if label is not None:
            location["label"], location["omitted_label_bytes"] = _diagnostic_text(label, 512)
        candidate = dict(result, spans=result["spans"] + [location],
                         spans_retained=len(result["spans"]) + 1,
                         spans_omitted=len(spans) - len(result["spans"]) - 1)
        # Reserve a few bytes for subsequent decimal omission-count growth.
        if len(_diagnostic_json(candidate)) <= MAX_ERROR_RECORD_BYTES - 64:
            result = candidate
    rendered = message.get("rendered") or ""
    raw_rendered = rendered.encode("utf-8")
    result["rendered_bytes_observed"] = len(raw_rendered)
    # The exact encoded record, including JSON escaping and omission fields,
    # determines admission. Warning output never consumes this allowance.
    allowance = min(len(raw_rendered), MAX_ERROR_RECORD_BYTES)
    while True:
        snippet, omitted = _diagnostic_text(rendered, allowance)
        result.update(rendered=snippet, rendered_bytes_retained=len(snippet.encode("utf-8")),
                      rendered_bytes_omitted=omitted)
        excess = len(_diagnostic_json(result)) - MAX_ERROR_RECORD_BYTES
        if excess <= 0:
            return result
        if allowance == 0:
            return None
        allowance = max(0, allowance - max(excess, 64))


def _compiler_diagnostic(record, paths, evidence):
    message = record.get("message")
    if not isinstance(message, dict):
        raise RuntimeError("Cargo compiler message is not an object.")
    level = message.get("level")
    if level not in {"error", "warning", "note", "help", "failure-note", "ice"}:
        raise RuntimeError("Cargo compiler diagnostic level is absent or unknown.")
    rendered = message.get("rendered")
    if rendered is not None and not isinstance(rendered, str):
        raise RuntimeError("Cargo rendered diagnostic is not text.")
    evidence["diagnostic_records"] += 1
    evidence["diagnostic_levels"][level] = evidence["diagnostic_levels"].get(level, 0) + 1
    raw = (rendered or "").encode("utf-8")
    if level in {"error", "failure-note", "ice"}:
        evidence["priority_records_observed"] += 1
        candidate = _priority_diagnostic(record, message, paths)
        encoded = _diagnostic_json(candidate) if candidate is not None else b""
        reason = ("record_byte_allowance" if candidate is None else
                  "record_count_allowance" if evidence["priority_records_retained"] >= MAX_ERROR_RECORDS else
                  "total_byte_allowance" if evidence["priority_bytes_retained"] + len(encoded) > MAX_ERROR_BYTES else None)
        if reason is not None:
            evidence["priority_records_omitted"] += 1
            counts = evidence["priority_omission_reasons"]
            counts[reason] = counts.get(reason, 0) + 1
            snippet = b""
        else:
            evidence["priority_diagnostics"].append(candidate)
            evidence["priority_records_retained"] += 1
            evidence["priority_bytes_retained"] += len(encoded)
            snippet = candidate["rendered"].encode("utf-8")
            evidence["priority_rendered_bytes"] += len(snippet)
    else:
        allowance = max(0, MAX_WARNING_RENDERED_BYTES - evidence["warning_rendered_bytes"])
        snippet = _diagnostic_text(rendered or "", allowance)[0].encode("utf-8")
        evidence["warning_rendered_bytes"] += len(snippet)
    if snippet:
        sys.stderr.write(snippet.decode("utf-8"))
        sys.stderr.flush()
    evidence["rendered_bytes"] += len(snippet)
    evidence["rendered_bytes_omitted"] += len(raw) - len(snippet)


def _record(raw, expected, paths, evidence):
    if len(raw) > MAX_RECORD_BYTES:
        raise RuntimeError("Cargo machine record exceeds its per-record allowance.")
    if not raw.strip():
        return
    evidence["records"] += 1
    if evidence["records"] > MAX_RECORDS:
        raise RuntimeError("Cargo machine record count exceeds its allowance.")
    try:
        record = json.loads(raw.decode("utf-8"))
    except (ValueError, UnicodeError) as error:
        raise RuntimeError("Cargo stdout contains malformed machine JSON.") from error
    if not isinstance(record, dict):
        raise RuntimeError("Cargo machine record is not an object.")
    reason = record.get("reason")
    if reason == "compiler-artifact":
        _artifact(record, expected, paths, evidence)
    elif reason == "build-script-executed":
        evidence["build_script_records_discarded"] += 1
    elif reason == "compiler-message":
        _compiler_diagnostic(record, paths, evidence)
    elif reason == "build-finished":
        if evidence["build_finished"] is not None or type(record.get("success")) is not bool:
            raise RuntimeError("Cargo build-finished record is duplicate or malformed.")
        evidence["build_finished"] = record["success"]
    elif reason == "future-incompat-report":
        evidence["other_records_discarded"] += 1
    else:
        raise RuntimeError("Cargo emitted an unknown machine-record reason.")


def cargo_observer(spec):
    """Run only the fixed offline compilation command and retain small evidence.

    Return 0 only for a complete successful target inventory; return 101 for a
    valid Cargo101 diagnostic failure, preserving any valid partial artifacts.
    On malformed output or an output allowance violation, stop Cargo promptly.
    """
    evidence = {
        "schema": "bitcoinrs-harness031-cargo-observer-v1", "command": None,
        "cwd": None, "target_directory": None, "expected_packages": [],
        "cargo_exit_code": None, "observer_exit_code": 2, "compile_outcome": None, "error": None,
        "artifacts": {}, "machine_bytes": 0, "records": 0,
        "dependency_artifacts_discarded": 0, "build_script_records_discarded": 0,
        "build_script_artifacts_discarded": 0,
        "diagnostic_records": 0, "other_records_discarded": 0,
        "rendered_bytes": 0, "rendered_bytes_omitted": 0,
        "diagnostic_levels": {}, "priority_diagnostics": [],
        "priority_records_observed": 0, "priority_records_retained": 0,
        "priority_records_omitted": 0, "priority_bytes_retained": 0,
        "priority_omission_reasons": {}, "warning_rendered_bytes": 0,
        "priority_rendered_bytes": 0,
        "build_finished": None,
    }
    child, paths = None, None
    try:
        command, expected, paths = _observer_spec(spec)
        evidence.update(command=command, cwd=str(paths["cwd"]),
                        target_directory=str(paths["target_directory"]), expected_packages=expected)
        env = dict(os.environ)
        env.update(CARGO_TARGET_DIR=str(paths["target_directory"]), CARGO_BUILD_JOBS="1", CARGO_INCREMENTAL="0")
        child = subprocess.Popen(command, cwd=paths["cwd"], env=env,
                                 stdin=subprocess.DEVNULL, stdout=subprocess.PIPE)
        os.set_blocking(child.stdout.fileno(), False)
        buffer = bytearray()
        deadline = time.monotonic() + OBSERVER_SECONDS
        with selectors.DefaultSelector() as selector:
            selector.register(child.stdout, selectors.EVENT_READ)
            while selector.get_map():
                if time.monotonic() >= deadline:
                    raise RuntimeError("Cargo observer reached its wall-clock allowance.")
                for key, _ in selector.select(0.1):
                    try:
                        chunk = os.read(key.fileobj.fileno(), 65536)
                    except BlockingIOError:
                        continue
                    if not chunk:
                        selector.unregister(key.fileobj)
                        if buffer:
                            _record(bytes(buffer), expected, paths, evidence)
                            buffer.clear()
                        continue
                    evidence["machine_bytes"] += len(chunk)
                    if evidence["machine_bytes"] > MAX_MACHINE_BYTES:
                        raise RuntimeError("Cargo machine stdout exceeds its total-byte allowance.")
                    buffer.extend(chunk)
                    while b"\n" in buffer:
                        newline = buffer.index(b"\n")
                        _record(bytes(buffer[:newline]), expected, paths, evidence)
                        del buffer[:newline + 1]
                    if len(buffer) > MAX_RECORD_BYTES:
                        raise RuntimeError("Cargo machine record exceeds its per-record allowance.")
        remaining = max(0.1, deadline - time.monotonic())
        evidence["cargo_exit_code"] = child.wait(timeout=remaining)
        if evidence["cargo_exit_code"] == 101:
            # A fully consumed, valid Cargo stream may end before build-finished
            # when Cargo itself fails. Keep its diagnostic failure distinct from
            # observer parsing, framing, target or resource-evidence failures.
            if evidence["build_finished"] is True:
                raise RuntimeError("Cargo reported a successful build despite its failed exit101.")
            evidence.update(observer_exit_code=101, compile_outcome="failed")
        elif evidence["cargo_exit_code"] != 0:
            raise RuntimeError("Cargo exited with an unreviewed unsuccessful status.")
        else:
            if evidence["build_finished"] is not True:
                raise RuntimeError("Cargo did not report a successful completed build.")
            # rustc may label explanatory output (for example --explain E0133
            # following an unsafe-op warning) as failure-note even on success.
            # Keep that output prioritized, but use error/ICE plus Cargo's actual
            # process/build-finished result to decide whether compilation passed.
            if (evidence["diagnostic_levels"].get("error", 0)
                    or evidence["diagnostic_levels"].get("ice", 0)):
                raise RuntimeError("Cargo reported error diagnostics despite a successful process/build result.")
            if set(evidence["artifacts"]) != set(paths["expected_artifacts"]):
                raise RuntimeError("Cargo did not emit the exact complete executable inventory for its selected command.")
            evidence.update(observer_exit_code=0, compile_outcome="passed")
    except Exception as error:
        evidence["error"] = (type(error).__name__ + ": " + str(error))[:2048]
        if child is not None:
            try:
                _stop_cargo(child)
            except Exception as stop_error:
                evidence["cleanup_error"] = str(stop_error)[:512]
            evidence["cargo_exit_code"] = child.poll()
    finally:
        if child is not None and child.stdout is not None:
            child.stdout.close()
    if paths is None:
        sys.stderr.write("HARNESS031_CARGO_OBSERVER_REJECTED " + (evidence["error"] or "invalid specification") + "\n")
        return 2
    try:
        _atomic_evidence(paths["evidence_path"], evidence)
    except Exception as error:
        sys.stderr.write("HARNESS031_CARGO_EVIDENCE_ERROR " + str(error)[:1024] + "\n")
        return 3
    print("HARNESS031_CARGO_OBSERVER records=" + str(evidence["records"])
          + " executables=" + str(len(evidence["artifacts"]))
          + " native_exit=" + str(evidence["cargo_exit_code"])
          + " observer_exit=" + str(evidence["observer_exit_code"]), flush=True)
    return evidence["observer_exit_code"]


"""Read-only binding of the held compile030 candidate; no native reuse."""
import hashlib
import json
import os
from pathlib import Path


PARENT_WORKFLOW031 = 'leveldb-facade-harness-compile030'
PARENT_RUN031 = '20261108T192119Z-18da1f5b'
PARENT_HEAD031 = 'fceccea344550ced58044a34dce9e1c677c54bb9'
PARENT_TREE031 = '3f15b24845aac80af16dac7c7c840f68cadf9abe'
ORIGINAL_MASTER031 = '2a5093e69dd91dc6d770e7e8ed1d201751f9ad5c'
ORIGINAL_TREE031 = '061d6ba025e0d2803bf4add4029aebc978d1fbfd'
PARENT_REPORT_BYTES031 = 1842786
PARENT_REPORT_SHA031 = '1fbfcd28cca57f6c1fecf8743452c49f818b1899712dde2e40f17906bf178867'
PARENT_CONTROLLER_BYTES031 = 114500
PARENT_CONTROLLER_SHA031 = 'f8f46facc717b96247594c715fe0699891b6431b6d4779e8febf02155c80c95b'
HARNESS_SOURCE031 = 'bitcoinleveldb-harness/tests/harness.rs'
HARNESS_SOURCE_SHA031 = 'ec558f0df5c653e7eeb7b7715f15fbb4db37fc1ba29723d7d16bc7927ed7f11f'
HARNESS_SOURCE_BLOB031 = '54b7fd3b817505bdf2bc7f48de3585bb8f53ef49'
HARNESS_SOURCE_BYTES031 = 20993
MIB_PRIOR031 = 1024 * 1024


def _equal031(core, left, right):
    return core.encoded(left) == core.encoded(right)


def baseline031(parent030module, core, repo):
    """Bind030's exact five-path commit and compilation-only evidence.

    The original checkout remains on master2a5093, while031 will be based on
    the separately held030 candidate. Independent030 runtime objects retain
    their own globals; the caller's031 core/helper configuration is not changed.
    """
    repo = Path(repo)
    if not repo.is_absolute() or '..' in repo.parts:
        raise RuntimeError('Harness031 prior repository must be an absolute ordinary path.')
    own = repo / '.git/codex-repairs' / PARENT_WORKFLOW031
    run = own / 'runs' / PARENT_RUN031
    report_path, controller_path = run / 'COMPILE_REPAIR_RETURN.json', run / 'compilation-controller030.py'
    worktree = repo.parent / (repo.name + '-leveldb-facade-harness-compile030')
    bindings = {}

    def read(path, maximum, *, size=None, sha=None, mode=None, content=True):
        fp, raw = core.read_file(path, maximum, content=content)
        if (fp.get('kind') != 'file' or (size is not None and fp.get('size') != size)
                or (sha is not None and fp.get('sha256') != sha)
                or (mode is not None and fp.get('mode') != mode)):
            raise RuntimeError('Harness031 retained030 bytes or mode differ: ' + str(path))
        bindings[str(path)] = fp
        return fp, raw

    report_fp, raw = read(report_path, 3 * MIB_PRIOR031, size=PARENT_REPORT_BYTES031,
                          sha=PARENT_REPORT_SHA031, mode=0o600)
    report = json.loads(raw)
    controller_fp, controller_raw = read(controller_path, 2 * MIB_PRIOR031,
        size=PARENT_CONTROLLER_BYTES031, sha=PARENT_CONTROLLER_SHA031, mode=0o700)
    required = {'workflow': PARENT_WORKFLOW031, 'controller_revision': 1,
        'outcome': 'compile_repair_qualified', 'phase': 'complete', 'compile_qualified': True,
        'repair_qualified': False, 'diagnostic_only': True, 'runtime_validation_claimed': False,
        'validation_reused': False, 'native_tests_started': False, 'debuggers_started': False,
        'test_count': 0, 'full_coverage_claimed': False, 'master_updated': False,
        'other_refs_updated': False, 'builds_started': True, 'crate_builds_started': True,
        'original_preserved': True, 'base': ORIGINAL_MASTER031, 'base_tree': ORIGINAL_TREE031,
        'candidate_head': PARENT_HEAD031, 'candidate_tree': PARENT_TREE031,
        'repository': str(repo), 'worktree': str(worktree), 'run_directory': str(run),
        'return_file': str(report_path), 'controller_sha256': PARENT_CONTROLLER_SHA031}
    if (any(type(report.get(k)) is not type(v) or report.get(k) != v for k, v in required.items())
            or not _equal031(core, report.get('intake_before'), report.get('intake_after'))
            or report.get('preservation_before_sha256') != report.get('preservation_after_sha256')):
        raise RuntimeError('Harness031 held030 report identity or preservation differs.')
    if (Path(parent030module.__file__) != controller_path
            or parent030module.WORKFLOW != PARENT_WORKFLOW031
            or parent030module.BASE != ORIGINAL_MASTER031 or parent030module.BASE_TREE != ORIGINAL_TREE031):
        raise RuntimeError('Harness031 parent module is not the pinned030 controller.')

    outer_identity = tuple(getattr(core, key, None) for key in ('WORKFLOW', 'BRANCH', 'WORKTREE_SUFFIX'))
    baseline028, _, finalizer, parent_core, helper = parent030module.runtime030(repo)
    if (parent_core is core or tuple(getattr(core, key, None) for key in
            ('WORKFLOW', 'BRANCH', 'WORKTREE_SUFFIX')) != outer_identity):
        raise RuntimeError('Harness031 parent runtime must remain independent of caller globals.')
    underlying = parent030module.baseline030(baseline028, parent_core, repo)
    if not _equal031(core, underlying, report['intake_before']):
        raise RuntimeError('Harness031 retained028/current-source binding differs from030 qualification.')
    files = parent030module.files030(controller_raw)
    if ({name: parent_core.digest(value) for name, value in files.items()} != report.get('source_plan')
            or len(files) != 4):
        raise RuntimeError('Harness031 parent030 initial payload identity differs.')
    receipt = report.get('receipt')
    if (not isinstance(receipt, dict) or receipt.get('compile_qualified') is not True
            or receipt.get('repair_qualified') is not False or receipt.get('runtime_validation_claimed') is not False
            or receipt.get('native_tests_started') is not False or type(receipt.get('test_count')) is not int
            or receipt['test_count'] != 0 or receipt.get('full_coverage_claimed') is not False
            or not _equal031(core, receipt.get('compilations'), report.get('compilations'))
            or receipt.get('source_plan') != report['source_plan']
            or receipt.get('guard_sha256') != report.get('guard_sha256')):
        raise RuntimeError('Harness031 parent receipt is not its exact compilation-only claim.')
    files[parent030module.RECEIPT_PATH] = parent_core.encoded(receipt)
    if report.get('changed_paths') != sorted(files):
        raise RuntimeError('Harness031 parent candidate differs from its exact five-path scope.')
    intent = {'tree': PARENT_TREE031, 'parents': [ORIGINAL_MASTER031],
              'identity': report.get('configured_identity'),
              'message_sha256': parent_core.digest(parent030module.COMMIT_MESSAGE)}
    state_path = own / 'state.json'
    state_fp, state_raw = read(state_path, MIB_PRIOR031, mode=0o600)
    state = json.loads(state_raw)
    expected_state = {'workflow': PARENT_WORKFLOW031, 'base': ORIGINAL_MASTER031,
        'branch': parent030module.BRANCH, 'worktree': str(worktree), 'run_directory': str(run),
        'phase': 'complete', 'source_plan': report['source_plan'],
        'compilations': {'facade': 'compiled', 'harness': 'compiled'}, 'receipt': receipt,
        'commit_intent': intent, 'created_commit': PARENT_HEAD031, 'head': PARENT_HEAD031,
        'report': str(report_path), 'report_fingerprint': report_fp}
    if (not _equal031(core, state, expected_state) or state_raw != parent_core.encoded(expected_state)):
        raise RuntimeError('Harness031 completed030 journal/commit intent/receipt differs.')
    if helper.verify_candidate_commit(finalizer, parent_core, repo, PARENT_HEAD031, intent, files) != PARENT_HEAD031:
        raise RuntimeError('Harness031 held030 candidate commit did not verify.')
    candidate = parent030module.check_candidate023(parent_core, repo, worktree, files, PARENT_HEAD031)
    if not _equal031(core, candidate, report.get('candidate_after')) or candidate.get('tree') != PARENT_TREE031:
        raise RuntimeError('Harness031 parent candidate index/live/tree binding differs.')

    artifacts, cargo_bindings = {}, {}
    if set(report.get('compilations', {})) != {'facade', 'harness'}:
        raise RuntimeError('Harness031 parent030 requires both separate compilations.')
    target = own / 'cargo-target'
    if report.get('target_directory') != str(target):
        raise RuntimeError('Harness031 parent compilation cache identity differs.')
    for label in ('facade', 'harness'):
        summary, evidence_path = report['compilations'][label], run / ('cargo-' + label + '.json')
        evidence_fp, evidence_raw = read(evidence_path, MIB_PRIOR031, mode=0o600)
        evidence = json.loads(evidence_raw)
        spec_path = run / ('cargo-' + label + '.spec.json')
        spec_fp, spec_raw = read(spec_path, MIB_PRIOR031, mode=0o600)
        spec = json.loads(spec_raw)
        expected_spec = {'command': parent030module.COMMANDS027[label], 'cwd': str(worktree),
                        'target_directory': str(target), 'evidence_path': str(evidence_path)}
        if (spec != expected_spec or summary.get('evidence_file') != str(evidence_path)
                or summary.get('evidence_fingerprint') != evidence_fp
                or not _equal031(core, evidence, report.get('cargo_evidence', {}).get(label))):
            raise RuntimeError('Harness031 parent external Cargo evidence/specification differs: ' + label)
        row = report.get('readouts', {}).get('compile-' + label, {})
        actual_summary = parent030module.qualify_compile030(parent_core, label, row, evidence, spec, target,
            guard_command=row.get('command'), run_directory=run)
        actual_summary.update(evidence_file=str(evidence_path), evidence_fingerprint=evidence_fp)
        if not _equal031(core, actual_summary, summary):
            raise RuntimeError('Harness031 parent fresh compile evidence does not reverify: ' + label)
        for name, item in summary['artifacts'].items():
            if name in artifacts: raise RuntimeError('Harness031 parent artifact inventory repeats a target.')
            fp, _ = read(Path(item['path']), 256 * MIB_PRIOR031, content=False)
            if fp != item['fingerprint']:
                raise RuntimeError('Harness031 parent compiled artifact differs: ' + name)
            artifacts[name] = item
        cargo_bindings[label] = {'path': str(evidence_path), 'fingerprint': evidence_fp,
                                 'specification': str(spec_path), 'specification_fingerprint': spec_fp}
    if len(artifacts) != 3:
        raise RuntimeError('Harness031 parent must retain exactly three compiled artifacts.')

    selected = underlying['source_capture']['files'].get(HARNESS_SOURCE031, {})
    expected_fp = {'kind': 'file', 'mode': 0o644, 'size': HARNESS_SOURCE_BYTES031, 'sha256': HARNESS_SOURCE_SHA031}
    if (selected.get('committed_sha256') != HARNESS_SOURCE_SHA031
            or selected.get('live_fingerprint') != expected_fp
            or selected.get('live_matches_committed') is not True
            or selected.get('live_mode_matches_committed') is not True
            or selected.get('text', '').encode() != parent_core.read_file(repo / HARNESS_SOURCE031, MIB_PRIOR031)[1]):
        raise RuntimeError('Harness031 selected original source is not the captured committed030 input.')
    inv = parent030module.candidate_original_guard023(parent_core, repo)
    expected_tree_row = ('100644 blob ' + HARNESS_SOURCE_BLOB031 + '\t' + HARNESS_SOURCE031 + '\0').encode()
    for commit in (ORIGINAL_MASTER031, PARENT_HEAD031):
        _, row = inv.git('ls-tree', '-r', '-z', commit, '--', HARNESS_SOURCE031)
        if row != expected_tree_row:
            raise RuntimeError('Harness031 selected source differs between original master and held parent.')
    _, committed = inv.git('cat-file', 'blob', HARNESS_SOURCE_BLOB031)
    if (len(committed) != HARNESS_SOURCE_BYTES031 or hashlib.sha256(committed).hexdigest() != HARNESS_SOURCE_SHA031
            or committed != selected['text'].encode()):
        raise RuntimeError('Harness031 selected committed source bytes differ.')
    master_fp, master_raw = read(repo / HARNESS_SOURCE031, MIB_PRIOR031, size=HARNESS_SOURCE_BYTES031,
                                 sha=HARNESS_SOURCE_SHA031, mode=0o644)
    parent_fp, parent_raw = read(worktree / HARNESS_SOURCE031, MIB_PRIOR031, size=HARNESS_SOURCE_BYTES031,
                                 sha=HARNESS_SOURCE_SHA031, mode=0o644)
    if master_raw != committed or parent_raw != committed:
        raise RuntimeError('Harness031 selected live source bytes differ from both committed blobs.')
    return {'original_master': ORIGINAL_MASTER031, 'original_tree': ORIGINAL_TREE031,
        'parent_head': PARENT_HEAD031, 'parent_tree': PARENT_TREE031,
        'report': str(report_path), 'report_fingerprint': report_fp,
        'controller': str(controller_path), 'controller_fingerprint': controller_fp,
        'journal': str(state_path), 'journal_fingerprint': state_fp,
        'artifact_bindings': artifacts, 'cargo_bindings': cargo_bindings,
        'selected_source': {'path': HARNESS_SOURCE031, 'sha256': HARNESS_SOURCE_SHA031,
            'blob_at_master': HARNESS_SOURCE_BLOB031, 'blob_at_parent': HARNESS_SOURCE_BLOB031,
            'original_fingerprint': master_fp, 'parent_fingerprint': parent_fp},
        'underlying_baseline': {'sha256': parent_core.digest(parent_core.encoded(underlying)),
            'report': underlying['report'], 'report_fingerprint': underlying['report_fingerprint'],
            'input_count': underlying['source_capture']['input_count'],
            'source_count': underlying['source_capture']['source_count']},
        'retained_bindings': bindings, 'prior_native_qualification_reused': False,
        'classification': 'held_compile030_candidate_and_preserved_sources_only'}


"""Read-only qualification of fresh compile031 no-run evidence and artifacts."""
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import stat
from urllib.parse import unquote, urlsplit


_Q031_WORKFLOW = 'leveldb-harness-controlled031'
_Q031_SUFFIX = '-leveldb-harness-controlled031'
_Q031_SCHEMA = 'bitcoinrs-harness031-cargo-observer-v1'
_Q031_COMMANDS = {
    'facade': ('cargo', 'test', '--offline', '--locked', '--jobs', '1', '-p', 'bitcoin-leveldb',
               '--lib', '--tests', '--no-run', '--message-format=json'),
    'harness': ('cargo', 'test', '--offline', '--locked', '--jobs', '1', '-p', 'bitcoinleveldb-harness',
                '--lib', '--tests', '--no-run', '--message-format=json')}
_Q031_TARGETS = {
    'facade': {'bitcoin_leveldb': ('lib', 'src/lib.rs')},
    'harness': {'bitcoinleveldb_harness': ('lib', 'src/lib.rs'), 'harness': ('test', 'tests/harness.rs'),
                'execution_control_contracts': ('test', 'tests/execution_control_contracts.rs')}}


def _q031_path(value, description):
    if (not isinstance(value, (str, Path)) or not str(value) or len(str(value)) > 4096
            or '\x00' in str(value)):
        raise RuntimeError('Compile031 ' + description + ' is not a bounded literal path.')
    path = Path(value)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Compile031 ' + description + ' is not an absolute ordinary path.')
    return path


def _q031_context(core, label, spec, target, guard_command, run_directory):
    if label not in _Q031_COMMANDS or not isinstance(spec, dict):
        raise RuntimeError('Compile031 requires one of its two reviewed package specifications.')
    run = _q031_path(run_directory, 'run directory')
    target = _q031_path(target, 'target directory')
    own = run.parent.parent
    if (not re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', run.name)
            or run.parent.name != 'runs' or own.name != _Q031_WORKFLOW
            or own.parent.name != 'codex-repairs' or own.parent.parent.name != '.git'
            or target != own / 'cargo-target'):
        raise RuntimeError('Compile031 evidence or cache is outside its independent fresh workflow.')
    repo = own.parent.parent.parent
    worktree = repo.parent / (repo.name + _Q031_SUFFIX)
    expected = {'command': list(_Q031_COMMANDS[label]), 'cwd': str(worktree),
                'target_directory': str(target), 'evidence_path': str(run / ('cargo-' + label + '.json'))}
    if spec != expected:
        raise RuntimeError('Compile031 observer specification differs from its exact fresh no-run plan.')
    for path in (run, worktree, target):
        descriptor = core.open_dir(path)
        os.close(descriptor)
    if (not isinstance(guard_command, list) or len(guard_command) != 13
            or any(not isinstance(value, str) or '\x00' in value or len(value) > 65536
                   for value in guard_command)
            or not Path(guard_command[0]).is_absolute() or Path(guard_command[0]).name != 'nix-shell'
            or guard_command[1:12] != ['--option', 'max-jobs', '0', '--option', 'builders', '',
                                       '--option', 'substitute', 'false', str(worktree / 'shell.nix'), '--run']):
        raise RuntimeError('Compile031 outer guard command differs from its offline Nix compilation plan.')
    try:
        inner = shlex.split(guard_command[-1])
    except ValueError as error:
        raise RuntimeError('Compile031 observer shell argument is malformed.') from error
    if (len(inner) != 7 or not Path(inner[0]).is_absolute() or Path(inner[0]).name != 'python3'
            or inner[1:] != ['-I', '-S', str(run / 'compilation-controller031.py'), '--cargo-observer',
                             str(repo), str(run / ('cargo-' + label + '.spec.json'))]):
        raise RuntimeError('Compile031 outer command is not bound to this fresh observer/script specification.')
    return repo, worktree, target


def _q031_package_id(value, package_root):
    if not isinstance(value, str) or not value or len(value) > 4096 or '\x00' in value:
        raise RuntimeError('Compile031 artifact package identity is absent or malformed.')
    if value.startswith('path+'):
        parsed = urlsplit(value[5:])
        path = Path(unquote(parsed.path))
        if (parsed.scheme != 'file' or parsed.netloc not in ('', 'localhost')
                or not path.is_absolute() or '..' in path.parts or path != package_root):
            raise RuntimeError('Compile031 artifact path package identity differs from its exact source root.')
    elif '://' in value:
        raise RuntimeError('Compile031 local artifact has an external package identity.')


def _q031_fingerprint(value, maximum):
    return (isinstance(value, dict) and value.get('kind') == 'file'
            and type(value.get('mode')) is int and 0 <= value['mode'] <= 0o777
            and value['mode'] & 0o111 != 0
            and type(value.get('size')) is int and 0 < value['size'] <= maximum
            and isinstance(value.get('sha256'), str)
            and re.fullmatch(r'[0-9a-f]{64}', value['sha256']) is not None)


def qualify_compile031(core, label, row, evidence, spec, target, *, guard_command, run_directory):
    """Require one freshly invoked031 build and bind its complete executable inventory.

    This routine only reads files. It never executes an artifact or reuses a
    preceding baseline failure as qualification. The caller invokes it for both
    labels after each newly executed guarded compilation.
    """
    _, worktree, target = _q031_context(core, label, spec, target, guard_command, run_directory)
    budget = core.load_guard().BUDGETS['build']
    if (not isinstance(row, dict) or row.get('verified') is not True or row.get('outcome') != 'exited'
            or type(row.get('exit_code')) is not int or row['exit_code'] != 0
            or row.get('kind') != 'build' or row.get('budget') != budget
            or any(type(value) is not int for value in row.get('budget', {}).values())
            or row.get('command') != guard_command or row.get('cwd') != str(worktree)):
        raise RuntimeError('Compile031 lacks its exact verified zero-exit finite guard command/cwd/budget.')
    core.require_guard(row)
    package = _Q031_COMMANDS[label][7]
    expected_keys = {package + '::' + name for name in _Q031_TARGETS[label]}
    if (not isinstance(evidence, dict) or evidence.get('schema') != _Q031_SCHEMA
            or evidence.get('command') != spec['command'] or evidence.get('cwd') != spec['cwd']
            or evidence.get('target_directory') != spec['target_directory']
            or evidence.get('expected_packages') != [package]
            or type(evidence.get('cargo_exit_code')) is not int or evidence['cargo_exit_code'] != 0
            or type(evidence.get('observer_exit_code')) is not int or evidence['observer_exit_code'] != 0
            or evidence.get('build_finished') is not True or evidence.get('compile_outcome') != 'passed'
            or 'error' not in evidence or evidence['error'] is not None
            or not isinstance(evidence.get('artifacts'), dict) or set(evidence['artifacts']) != expected_keys):
        raise RuntimeError('Compile031 requires exact successful observer evidence and every selected artifact.')
    levels = evidence.get('diagnostic_levels')
    if (not isinstance(levels, dict) or not set(levels).issubset({'error', 'warning', 'note', 'help', 'failure-note', 'ice'})
            or any(type(value) is not int or not 0 <= value <= 50000 for value in levels.values())
            or any(levels.get(level, 0) for level in ('error', 'ice'))):
        raise RuntimeError('Compile031 successful evidence contains malformed or failing compiler diagnostics.')
    priority = {name: evidence.get(name) for name in
                ('priority_records_observed', 'priority_records_retained', 'priority_records_omitted')}
    if (any(type(value) is not int or not 0 <= value <= 50000 for value in priority.values())
            or priority['priority_records_observed'] != priority['priority_records_retained'] + priority['priority_records_omitted']
            or priority['priority_records_observed'] != sum(levels.get(level, 0) for level in ('error', 'failure-note', 'ice'))
            or priority['priority_records_omitted'] != 0):
        raise RuntimeError('Compile031 priority diagnostic retention is incomplete or malformed.')
    artifacts, paths = {}, set()
    maximum = 256 * core.MIB
    for name, (kind, relative_source) in _Q031_TARGETS[label].items():
        key = package + '::' + name
        record = evidence['artifacts'][key]
        if not isinstance(record, dict):
            raise RuntimeError('Compile031 artifact record is not an object.')
        path = _q031_path(record.get('executable'), 'artifact executable')
        if (path.parent != target / 'debug/deps' or path in paths
                or re.fullmatch(re.escape(name) + r'-[0-9a-f]+', path.name) is None
                or record.get('package') != package or record.get('target_name') != name
                or record.get('kind') != [kind]
                or record.get('src_path') != str(worktree / package / relative_source)):
            raise RuntimeError('Compile031 artifact package/target/source/private executable binding differs.')
        _q031_package_id(record.get('package_id'), worktree / package)
        fingerprint, _ = core.read_file(path, maximum, content=False)
        if not _q031_fingerprint(fingerprint, maximum):
            raise RuntimeError('Compile031 artifact lacks ordinary executable mode/size/SHA256 evidence.')
        paths.add(path)
        artifacts[key] = {'path': str(path), 'fingerprint': fingerprint}
    for item in artifacts.values():
        if core.read_file(Path(item['path']), maximum, content=False)[0] != item['fingerprint']:
            raise RuntimeError('Compile031 artifact changed during its read-only qualification.')
    return {'schema': 'bitcoinrs-compile031-qualification-v1', 'outcome': 'compiled',
            'cargo_exit_code': 0, 'observer_exit_code': 0,
            'artifacts': artifacts, 'expected_artifacts': sorted(expected_keys),
            'native_cases_executed': 0, 'validation_reused': False,
            'diagnostic_levels': dict(levels), **priority, 'repair_qualified': False,
            'scope': 'fresh successful offline no-run compilation only; no native-case qualification'}


def _q031_case_table():
    pure = ('admission_accepts_complete_controls', 'admission_rejects_missing_or_weakened_controls')
    ordinary = ('harness_empty', 'harness_zero_restart_points_in_block', 'harness_simple_empty_key',
        'harness_simple_single', 'harness_simple_multi', 'harness_simple_special_key',
        'harness_db_pointer_is_null_when_not_db_test',
        'harness_reinitialization_switches_constructor_types_without_panicking',
        'harness_duplicate_keys_overwrite_previous_value', 'harness_handles_embedded_nul_and_empty_values',
        'harness_pick_random_key_returns_foo_on_empty_keyset',
        'harness_to_string_formats_iterator_entries_and_end_sentinel')
    heavy = ('harness_randomized', 'harness_randomized_longdb')
    return [dict(target=target, suffix=name, name=name, profile=profile, ignored=ignored,
                 marker=prefix + ' case=' + name)
        for names, target, profile, ignored, prefix in (
            (pure, 'execution_control_contracts', 'pure', False, 'HARNESS_CONTROL_CONTRACT_OK'),
            (ordinary, 'harness', 'case', False, 'HARNESS_CONTRACT_OK'),
            (heavy, 'harness', 'full_original', True, 'HARNESS_CONTRACT_OK'))
        for name in names]


def _q031_case_item(item):
    table = _q031_case_table()
    keys = ('target', 'suffix', 'profile', 'ignored', 'marker')
    declared = globals().get('CASES')
    if (not isinstance(declared, (list, tuple)) or len(declared) != 16
            or any(not isinstance(value, dict) or type(value.get('ignored')) is not bool for value in declared)
            or [{key: value.get(key) for key in keys} for value in declared]
               != [{key: value[key] for key in keys} for value in table]):
        raise RuntimeError('Harness031 reviewed sixteen-case table differs from its exact source bindings.')
    selected = [value for value in table if isinstance(item, dict) and value['suffix'] == item.get('suffix')]
    if (len(selected) != 1 or type(item.get('ignored')) is not bool
            or any(item.get(key) != value for key, value in selected[0].items())):
        raise RuntimeError('Harness031 selected case target/name/profile/ignore/marker binding differs.')
    return selected[0]


def _q031_kernel(probe, budget):
    kernel = probe.get('kernel') if isinstance(probe, dict) else None
    if not isinstance(probe, dict) or probe.get('ok') is not True or not isinstance(kernel, dict):
        raise RuntimeError('Harness031 native probe lacks verified kernel admission.')
    expected = {'memory.max': str(budget['memory']), 'memory.high': str(budget['high']),
                'memory.swap.max': '0', 'memory.oom.group': '1', 'pids.max': str(budget['tasks'])}
    if any(kernel.get(key) != value for key, value in expected.items()):
        raise RuntimeError('Harness031 native kernel memory/swap/task/OOM controls differ.')
    cpu = kernel.get('cpu.max')
    match = re.fullmatch(r'([0-9]{1,20}) ([0-9]{1,20})', cpu) if isinstance(cpu, str) else None
    if match is None or not 0 < int(match.group(1)) <= int(match.group(2)):
        raise RuntimeError('Harness031 native kernel CPU quota is absent or exceeds one CPU.')
    for key, expected_value in (('rlimit_core', [0, 0]), ('rlimit_fsize', [budget['file'], budget['file']])):
        value = kernel.get(key)
        if not isinstance(value, list) or any(type(item) is not int for item in value) or value != expected_value:
            raise RuntimeError('Harness031 native core/file-size controls differ.')
    headroom = kernel.get('headroom')
    required = budget['memory'] + 256 * 1024 * 1024
    if (not isinstance(headroom, dict) or type(headroom.get('required_bytes')) is not int
            or headroom['required_bytes'] != required or type(headroom.get('mem_available_bytes')) is not int
            or headroom['mem_available_bytes'] < required):
        raise RuntimeError('Harness031 native probe lacks its declared memory-admission margin.')


def case_guard_binding031(core, report, item, row):
    """Bind one exact passing case to current artifact/log and guard file bytes."""
    item = _q031_case_item(item)
    if not isinstance(report, dict) or not isinstance(row, dict):
        raise RuntimeError('Harness031 native report or guard result is malformed.')
    run = _q031_path(report.get('run_directory'), 'native run directory')
    own = run.parent.parent
    if (not re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', run.name) or run.parent.name != 'runs'
            or own.name != _Q031_WORKFLOW or own.parent.name != 'codex-repairs' or own.parent.parent.name != '.git'):
        raise RuntimeError('Harness031 native case belongs to another workflow/run.')
    repo = own.parent.parent.parent
    worktree = repo.parent / (repo.name + _Q031_SUFFIX)
    target = own / 'cargo-target'
    if report.get('worktree') != str(worktree) or report.get('target_directory') != str(target):
        raise RuntimeError('Harness031 native worktree or private artifact cache differs.')
    artifacts = report.get('artifacts')
    expected = {(_Q031_COMMANDS[label][7] + '::' + name): (name, kind, source)
                for label, values in _Q031_TARGETS.items() for name, (kind, source) in values.items()}
    if not isinstance(artifacts, dict) or set(artifacts) != set(expected):
        raise RuntimeError('Harness031 native case requires all four compiled artifact bindings.')
    for key, (name, _, _) in expected.items():
        entry = artifacts[key]
        if not isinstance(entry, dict):
            raise RuntimeError('Harness031 compiled artifact binding is malformed.')
        path = _q031_path(entry.get('path'), 'native artifact')
        if (path.parent != target / 'debug/deps' or not re.fullmatch(re.escape(name) + r'-[0-9a-f]+', path.name)
                or not _q031_fingerprint(entry.get('fingerprint'), 256 * core.MIB)
                or core.read_file(path, 256 * core.MIB, content=False)[0] != entry['fingerprint']):
            raise RuntimeError('Harness031 compiled artifact fingerprint/private identity changed.')
    selected = artifacts['bitcoinleveldb-harness::' + item['target']]
    data = run / ('case-data-' + item['suffix'])
    kind = 'stress' if item['profile'] == 'full_original' else 'case'
    budget = core.load_guard().BUDGETS[kind]
    command = [selected['path'], '--exact', item['name'], '--nocapture', '--test-threads=1']
    if item['ignored']:
        command.append('--ignored')
    expected_env = {'BITCOIN_RS_HARNESS031_PROTECTED': '1', 'BITCOIN_RS_HARNESS031_CASE': item['name'],
                    'BITCOIN_RS_HARNESS031_CASE_DATA': str(data), 'TEST_RANDOM_SEED': '301', 'RUST_LOG': 'off',
                    **{key: str(data) for key in ('TMPDIR', 'TMP', 'TEMP', 'TEST_TMPDIR')}}
    log_path = _q031_path(row.get('log'), 'native output log')
    group = log_path.parent
    if (log_path.name != 'output.log' or group.parent != run / 'guard'
            or not re.fullmatch(r'[A-Za-z0-9_.-]{1,128}', group.name)
            or row.get('cwd') != str(worktree) or row.get('command') != command
            or row.get('kind') != kind or row.get('budget') != budget
            or row.get('data_directory') != str(data) or row.get('verified') is not True
            or row.get('outcome') != 'exited' or type(row.get('exit_code')) is not int or row['exit_code'] != 0):
        raise RuntimeError('Harness031 exact native command, controls, data path or successful result differs.')
    descriptor = core.open_dir(data)
    try:
        info = os.fstat(descriptor)
        if stat.S_IMODE(info.st_mode) != 0o700 or info.st_uid != os.getuid():
            raise RuntimeError('Harness031 native case data directory is not privately owned mode0700.')
    finally:
        os.close(descriptor)
    values, fingerprints = {}, {}
    for name in ('output.log', 'plan.json', 'probe.json', 'permit', 'result.json'):
        limit = budget['output'] if name == 'output.log' else core.MIB
        fingerprint, raw = core.read_file(group / name, limit)
        fingerprints[str(group / name)] = fingerprint
        values[name] = raw if name in {'output.log', 'permit'} else json.loads(raw)
    plan, probe = values['plan.json'], values['probe.json']
    if (not isinstance(plan, dict) or not isinstance(probe, dict) or plan.get('kind') != kind
            or plan.get('command') != command or plan.get('cwd') != str(worktree)
            or not isinstance(plan.get('env'), dict) or any(plan['env'].get(key) != value for key, value in expected_env.items())
            or plan.get('probe') != str(group / 'probe.json') or plan.get('permit') != str(group / 'permit')
            or not isinstance(plan.get('nonce'), str) or not re.fullmatch(r'[0-9a-f]{32}', plan['nonce'])
            or values['permit'] != plan['nonce'].encode()
            or json.dumps(probe, sort_keys=True, allow_nan=False) != json.dumps(row.get('probe'), sort_keys=True, allow_nan=False)
            or json.dumps(values['result.json'], sort_keys=True, allow_nan=False) != json.dumps(row, sort_keys=True, allow_nan=False)
            or type(row.get('output_bytes')) is not int
            or row['output_bytes'] != len(values['output.log']) or row['output_bytes'] > budget['output']):
        raise RuntimeError('Harness031 plan/environment/nonce/admission/result/log bytes contradict the exact native record.')
    _q031_kernel(probe, budget)
    assessed = core.assess_case(row, values['output.log'].decode('utf-8', errors='replace'), item['name'], item['marker'])
    if (assessed.get('outcome') != 'passed' or assessed.get('expected_name') != item['name']
            or assessed.get('marker') != item['marker']
            or any(type(assessed.get(key)) is not int or assessed[key] != value
                   for key, value in (('passed', 1), ('failed', 0), ('ignored', 0)))):
        raise RuntimeError('Harness031 native log does not prove exactly one passing selected nonignored case and marker.')
    for path, fingerprint in fingerprints.items():
        limit = budget['output'] if Path(path).name == 'output.log' else core.MIB
        if core.read_file(Path(path), limit, content=False)[0] != fingerprint:
            raise RuntimeError('Harness031 guard or output bytes changed during native qualification.')
    if core.read_file(Path(selected['path']), 256 * core.MIB, content=False)[0] != selected['fingerprint']:
        raise RuntimeError('Harness031 selected executable changed during native qualification.')
    return {'label': item['suffix'], 'kind': kind, 'required_environment': expected_env,
            'guard_files': fingerprints, 'assessment': assessed,
            'selected_artifact': {'key': 'bitcoinleveldb-harness::' + item['target'], **selected}}


"""Exact case selection, command and environment grammar for harness031."""

def source_review_notes031():
    return {'execution_policy': 'Plain test attributes, true resource admission before all fourteen original bodies, two heavy cases ignored by default, seed301/quiet logging/private per-case temporary roots.',
        'body_preservation': 'Inverse source projection retains every original helper, operation stream, argument matrix, workload size and assertion; added progress only reads existing counters.',
        'completion': 'Original body locals and fixture teardown leave an inner lexical scope before the completion marker is emitted.',
        'workloads': {'ordinary_cases': 12, 'full_original_cases': 2, 'argument_configurations': 16,
            'randomized_sizes_per_configuration': 60, 'randomized_add_calls': 171600,
            'randomized_harness_test_calls': 960, 'longdb_add_calls': 100000},
        'oracle_limits': 'Existing iterator checks do not assert iterator status, and manual raw iterator cleanup is not made panic-safe by this change. No full coverage or historical lockup cause is claimed.',
        'storage': 'Fresh empty Cargo target; no optional cache duplication; pinned guard keeps its existing 2 GiB free-space floor.'}

def case_command031(executable, item):
    command = [str(executable), '--exact', item['name'], '--nocapture', '--test-threads=1']
    if item['ignored']: command.append('--ignored')
    return command


def case_environment031(item, data):
    return {'BITCOIN_RS_HARNESS031_PROTECTED': '1', 'BITCOIN_RS_HARNESS031_CASE': item['name'],
        'BITCOIN_RS_HARNESS031_CASE_DATA': str(data), 'TEST_RANDOM_SEED': '301', 'RUST_LOG': 'off',
        'TEST_TMPDIR': str(data), 'TMPDIR': str(data), 'TMP': str(data), 'TEMP': str(data)}


def selected_cases031(lists):
    expected_keys = {'bitcoin-leveldb::bitcoin_leveldb', 'bitcoinleveldb-harness::bitcoinleveldb_harness',
        'bitcoinleveldb-harness::harness', 'bitcoinleveldb-harness::execution_control_contracts'}
    if (not isinstance(lists, dict) or set(lists) != {'all', 'ignored'}
            or any(set(lists[kind]) != expected_keys for kind in lists)):
        raise RuntimeError('Harness031 native test-list artifact inventory differs.')
    if len(CASES) != 16 or len({item['name'] for item in CASES}) != 16:
        raise RuntimeError('Harness031 requires its exact two control plus fourteen original cases.')
    for key in sorted(expected_keys):
        target = key.split('::', 1)[1]
        selected = [item for item in CASES if item['target'] == target]
        expected = sorted(item['name'] for item in selected)
        ignored = sorted(item['name'] for item in selected if item['ignored'])
        for kind, names in (('all', expected), ('ignored', ignored)):
            observed = lists[kind][key]
            if not isinstance(observed, list) or sorted(observed) != names or len(observed) != len(set(observed)):
                raise RuntimeError('Harness031 exact all/ignored test inventory differs: ' + key)
    if ([item['profile'] for item in CASES[:2]] != ['pure', 'pure']
            or any(item['profile'] != 'case' for item in CASES[2:14])
            or [item['suffix'] for item in CASES[-2:]] != ['harness_randomized', 'harness_randomized_longdb']
            or any(item['profile'] != 'full_original' or not item['ignored'] for item in CASES[-2:])):
        raise RuntimeError('Harness031 qualification ordering or full-workload inventory differs.')
    return [dict(item) for item in CASES]


def flatten_artifacts031(compilations):
    if set(compilations) != {'facade', 'harness'} or any(v.get('outcome') != 'compiled' for v in compilations.values()):
        raise RuntimeError('Harness031 requires both fresh successful no-run compilations before native execution.')
    artifacts = {}
    for value in compilations.values():
        for key, item in value['artifacts'].items():
            if key in artifacts: raise RuntimeError('Harness031 compiled artifact inventory repeats a target.')
            artifacts[key] = item
    if set(artifacts) != {'bitcoin-leveldb::bitcoin_leveldb', 'bitcoinleveldb-harness::bitcoinleveldb_harness',
            'bitcoinleveldb-harness::harness', 'bitcoinleveldb-harness::execution_control_contracts'}:
        raise RuntimeError('Harness031 compiled artifact inventory lacks an exact selected target.')
    return artifacts


def check_artifacts031(core, artifacts):
    for item in artifacts.values():
        if core.read_file(Path(item['path']), 256 * MIB027, content=False)[0] != item['fingerprint']:
            raise RuntimeError('Harness031 compiled native artifact changed during qualification.')


def load031(name, path, size, sha):
    raw = bootstrap_read(path, 2 * MIB027)
    if len(raw) != size or hashlib.sha256(raw).hexdigest() != sha:
        raise RuntimeError('Reviewed retained runtime differs: ' + path.name)
    module = types.ModuleType(name); module.__file__ = str(path); sys.modules[name] = module
    exec(compile(raw, str(path), 'exec'), module.__dict__)
    return module

def runtime031(repo):
    parent = load031('harness031_parent030',
        repo / '.git/codex-repairs/leveldb-facade-harness-compile030/runs/20261108T192119Z-18da1f5b/compilation-controller030.py',
        114500, 'f8f46facc717b96247594c715fe0699891b6431b6d4779e8febf02155c80c95b')
    _, previous, finalizer, core, helper = parent.runtime030(repo)
    configure_git_helper023(helper)
    core.WORKFLOW, core.BRANCH, core.WORKTREE_SUFFIX = WORKFLOW, BRANCH, SUFFIX
    return parent, previous, finalizer, core, helper


def files031(raw):
    if not isinstance(raw, bytes) or len(raw) > 2 * MIB027:
        raise RuntimeError('Compile repair controller bytes exceed their finite allowance.')
    files = dict(SOURCE_PAYLOADS)
    files.update({REPLAY_PATH: raw, DOCUMENT_PATH: DOCUMENT_BYTES})
    if set(files) != set(INITIAL_PATHS): raise RuntimeError('Compile repair file plan differs.')
    return files

def owned031(core, repo, path, names):
    path = Path(path)
    if (not path.is_absolute() or '..' in path.parts or path.name not in names
            or path.parent.parent != repo / '.git/codex-repairs' / WORKFLOW / 'runs'
            or re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', path.parent.name) is None):
        raise RuntimeError('Compile repair worker path differs from its finite owned output scope.')
    fd = core.open_dir(path.parent); os.close(fd)

def inputs_worker031(repo, output):
    baseline, _, _, core, _ = runtime031(repo)
    owned031(core, repo, output, {'inputs-before.json', 'inputs-after.json'})
    core.exclusive(output, core.encoded(baseline031(baseline, core, repo)))
    print('COMPILE031_SOURCE_AND_PRIOR_EVIDENCE_REVERIFIED', flush=True)
    return 0

def identity_worker031(repo, spec_path, output):
    _, _, finalizer, core, _ = runtime031(repo)
    owned031(core, repo, spec_path, {'identity-input.json'}); owned031(core, repo, output, {'identity.json'})
    if spec_path.parent != output.parent: raise RuntimeError('Identity paths belong to different compile repair runs.')
    spec = json.loads(core.read_file(spec_path, MIB027)[1]); inv = candidate_original_guard023(core, repo)
    env = dict(inv.env)
    for key in finalizer.IDENTITY_ENV: env.pop(key, None)
    if not isinstance(spec, dict): raise RuntimeError('Configured identity input is malformed.')
    for key, value in spec.items():
        if key not in finalizer.IDENTITY_ENV or not isinstance(value, str) or '\0' in value:
            raise RuntimeError('Configured identity override differs from the finite plan.')
        env[key] = value
    result = {}
    for role in ('AUTHOR', 'COMMITTER'):
        p = subprocess.run(inv.prefix + ['-c', 'user.useConfigOnly=true', 'var', 'GIT_' + role + '_IDENT'],
            cwd=repo, env=env, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10, check=False)
        if p.returncode or len(p.stdout) > 4096 or len(p.stderr) > 8192:
            raise RuntimeError('Configured Git identity is unavailable; no identity was guessed.')
        result[role.lower()] = finalizer.parse_ident(p.stdout.decode().rstrip('\n'))
    core.exclusive(output, core.encoded(result)); print('COMPILE031_CONFIGURED_IDENTITIES_RECORDED', flush=True)
    return 0

def install_worker031(repo):
    baseline, _, _, core, helper = runtime031(repo); intake = baseline031(baseline, core, repo)
    worktree = repo.parent / (repo.name + SUFFIX)
    raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB027); files = files031(raw)
    check_candidate023(core, repo, worktree, files, BASE, allow_base_files=True)
    for name, expected_sha in ORIGINAL_SOURCE_HASHES.items():
        fp, source = core.read_file(worktree / name, MIB027)
        if fp['sha256'] != expected_sha:
            raise RuntimeError('Compile replacement source differs from its captured committed input: ' + name)
    for name, content in sorted(files.items()):
        destination = worktree / name; destination.parent.mkdir(parents=True, exist_ok=True)
        fd = core.open_dir(destination.parent); os.close(fd)
        mode = 0o755 if name == REPLAY_PATH else 0o644
        if name in ORIGINAL_SOURCE_HASHES:
            temporary = destination.with_name(destination.name + '.compile031-' + uuid.uuid4().hex)
            helper.candidate_exclusive015(core, temporary, content, mode); os.replace(temporary, destination)
        else:
            if os.path.lexists(destination): raise RuntimeError('New compile repair path is occupied: ' + name)
            helper.candidate_exclusive015(core, destination, content, mode)
    print('COMPILE031_EXACT_SOURCE_INSTALLED', flush=True)
    return 0

def observer_worker031(repo, spec_path):
    _, _, _, core, _ = runtime031(repo)
    owned031(core, repo, spec_path, {'cargo-' + name + '.spec.json' for name in COMMANDS031})
    worktree = repo.parent / (repo.name + SUFFIX)
    raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB027)
    check_candidate023(core, repo, worktree, files031(raw), BASE)
    label = spec_path.name[len('cargo-'):-len('.spec.json')]
    spec = json.loads(core.read_file(spec_path, MIB027)[1])
    expected = {'command': COMMANDS031[label], 'cwd': str(worktree),
        'target_directory': str(repo / '.git/codex-repairs' / WORKFLOW / 'cargo-target'),
        'evidence_path': str(spec_path.parent / ('cargo-' + label + '.json'))}
    if spec != expected: raise RuntimeError('Fresh compile031 Cargo observer specification differs.')
    return cargo_observer(spec)

def git_worker031(repo, spec):
    _, _, _, core, helper = runtime031(repo)
    previous = os.umask(0o022)
    try: return helper.run_git_operation(core, repo, spec)
    finally: os.umask(previous)

def artifact_check031(core, compilations):
    for label in ('facade', 'harness'):
        for item in compilations[label]['artifacts'].values():
            if core.read_file(Path(item['path']), 256 * MIB027, content=False)[0] != item['fingerprint']:
                raise RuntimeError('Fresh compile031 artifact changed before publication.')


class AdmissionDeferred031(RuntimeError):
    """Handled guard headroom refusal before reservation."""


def build_admission031(core, row, command, repo):
    budget = core.load_guard().BUDGETS['build']
    required = budget['memory'] + 256 * MIB027
    if (row.get('kind') != 'build' or row.get('budget') != budget
            or row.get('command') != command or row.get('cwd') != str(repo)
            or type(row.get('exit_code')) is not int):
        raise RuntimeError('Build admission does not match the unchanged finite probe plan.')
    if row.get('verified') is True:
        if row['exit_code'] != 0:
            raise RuntimeError('Verified build admission must have native integer zero status.')
        core.require_guard(row)
        headroom = row.get('probe', {}).get('kernel', {}).get('headroom', {})
        available = headroom.get('mem_available_bytes')
        if (row.get('probe', {}).get('ok') is not True or headroom.get('required_bytes') != required
                or type(available) is not int or available < required):
            raise RuntimeError('Verified build admission lacks its declared host-memory margin.')
        return {'admitted': True, 'required_bytes': required, 'available_bytes_observed': available,
                'budget_unchanged': True, 'interpretation': 'Admission at this probe; every later guarded phase rechecks controls and headroom.'}
    message = row.get('probe', {}).get('error')
    pattern = (r"RuntimeError: Host memory headroom is below the declared admission margin: "
               r"\{'required_bytes': ([0-9]{1,20}), 'mem_available_bytes': ([0-9]{1,20}), 'ancestors': \[\]\}")
    match = re.fullmatch(pattern, message) if isinstance(message, str) else None
    if (row['exit_code'] not in {0, 78} or row.get('verified') is not False or row.get('outcome') != 'controls_unavailable'
            or row.get('probe', {}).get('ok') is not False or match is None
            or int(match.group(1)) != required or int(match.group(2)) >= required):
        raise RuntimeError('Build admission failed for a control/protocol condition requiring separate review.')
    available = int(match.group(2))
    return {'admitted': False, 'required_bytes': required, 'available_bytes_observed': available,
            'deficit_bytes_observed': required - available, 'budget_unchanged': True,
            'interpretation': 'Guard refused before the probe command could execute; no cause of memory pressure is inferred.'}


"""Post-exit transport for the controlled harness repair031.

Only the separately authorized original return-buffer is replaced after the
coordinator has exited and printed/saved return JSON identities agree.
"""
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import stat
import subprocess
import sys
import threading
import uuid


_TRANSPORT_MAX_BYTES = 8 * 1024 * 1024


_TRANSPORT_WORKFLOW = "leveldb-harness-controlled031"


_TRANSPORT_BEGIN = b"BEGIN_HARNESS_REPAIR_RETURN"


_TRANSPORT_END = b"END_HARNESS_REPAIR_RETURN"

_TRANSPORT_ARTIFACTS031 = {
    "facade": {"bitcoin-leveldb::bitcoin_leveldb"},
    "harness": {"bitcoinleveldb-harness::bitcoinleveldb_harness", "bitcoinleveldb-harness::harness",
                "bitcoinleveldb-harness::execution_control_contracts"},
}
_TRANSPORT_PATHS031 = {
    "bitcoinleveldb-harness/tests/harness.rs", "bitcoinleveldb-harness/tests/support/execution_controls.rs",
    "bitcoinleveldb-harness/tests/execution_control_contracts.rs",
    "contrib/repair/bitcoinleveldb-harness-controlled031.py",
    "doc/repair/leveldb-harness-controlled031.json", "doc/testing/leveldb-harness-controlled031.md",
}

_TRANSPORT_FULL_CASES031 = {"harness_randomized", "harness_randomized_longdb"}
_TRANSPORT_HARNESS_CASES031 = {
    "harness_empty", "harness_zero_restart_points_in_block", "harness_simple_empty_key", "harness_simple_single",
    "harness_simple_multi", "harness_simple_special_key", "harness_randomized", "harness_randomized_longdb",
    "harness_db_pointer_is_null_when_not_db_test", "harness_reinitialization_switches_constructor_types_without_panicking",
    "harness_duplicate_keys_overwrite_previous_value", "harness_handles_embedded_nul_and_empty_values",
    "harness_pick_random_key_returns_foo_on_empty_keyset", "harness_to_string_formats_iterator_entries_and_end_sentinel",
}
_TRANSPORT_CONTROL_CASES031 = {"admission_accepts_complete_controls", "admission_rejects_missing_or_weakened_controls"}


def _transport_cases031(report):
    wanted = _TRANSPORT_CONTROL_CASES031 | _TRANSPORT_HARNESS_CASES031
    cases = report.get("cases")
    if not isinstance(cases, list) or len(cases) != 16 or any(not isinstance(case, dict) for case in cases):
        raise RuntimeError("Harness qualification lacks the exact sixteen native case assessments.")
    names = [case.get("label") for case in cases]
    if (any(not isinstance(name, str) for name in names) or len(set(names)) != 16 or set(names) != wanted
            or set(names[:2]) != _TRANSPORT_CONTROL_CASES031 or set(names[-2:]) != _TRANSPORT_FULL_CASES031):
        raise RuntimeError("Harness native case inventory or protected execution order differs.")
    for case in cases:
        name = case["label"]; pure = name in _TRANSPORT_CONTROL_CASES031
        target = "execution_control_contracts" if pure else "harness"
        profile = "pure" if pure else "full_original" if name in _TRANSPORT_FULL_CASES031 else "case"
        marker = ("HARNESS_CONTROL_CONTRACT_OK case=" if pure else "HARNESS_CONTRACT_OK case=") + name
        if (case.get("outcome") != "passed" or case.get("expected_name") != name or case.get("target") != target
                or case.get("profile") != profile or case.get("marker") != marker
                or case.get("error") is not None or not isinstance(case.get("log"), str) or marker not in case["log"]
                or case.get("ignored_by_default") is not (name in _TRANSPORT_FULL_CASES031)
                or any(type(case.get(key)) is not int or case[key] != value
                       for key, value in (("passed", 1), ("failed", 0), ("ignored", 0)))):
            raise RuntimeError("A required harness case is not an exact successful native assessment.")
    completed = report.get("completed_full_cases")
    if (not isinstance(completed, list) or len(completed) != 2 or set(completed) != _TRANSPORT_FULL_CASES031
            or report.get("original_full_cases_completed") is not True or report.get("full_harness_stress") != "passed"
            or not isinstance(report.get("case_guards"), dict) or set(report["case_guards"]) != wanted):
        raise RuntimeError("Harness original heavy cases or all sixteen native guard bindings are incomplete.")
    expected = {key: value for group in report["compilations"].values() for key, value in group["artifacts"].items()}
    if report.get("artifacts") != expected:
        raise RuntimeError("Harness flattened executable bindings differ from the four freshly compiled artifacts.")


def _transport_compilations031(report, repo):
    compilations = report.get("compilations")
    if not isinstance(compilations, dict) or set(compilations) != set(_TRANSPORT_ARTIFACTS031):
        raise RuntimeError("Compile-repair result lacks both separate crate compilations.")
    successes = 0
    target = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW / "cargo-target"
    if report.get("target_directory") != str(target):
        raise RuntimeError("Compile-repair artifacts do not identify the owned private cache.")
    for label, expected in _TRANSPORT_ARTIFACTS031.items():
        value = compilations[label]
        if not isinstance(value, dict): raise RuntimeError("Crate compilation summary is malformed.")
        code = value.get("cargo_exit_code")
        levels = value.get("diagnostic_levels")
        artifacts = value.get("artifacts")
        if (type(code) is not int or code not in {0, 101}
                or type(value.get("observer_exit_code")) is not int or value["observer_exit_code"] != code
                or value.get("expected_artifacts") != sorted(expected)
                or type(value.get("native_cases_executed")) is not int or value["native_cases_executed"] != 0
                or value.get("repair_qualified") is not False
                or type(value.get("priority_records_omitted")) is not int or value["priority_records_omitted"] != 0
                or not isinstance(levels, dict) or any(type(count) is not int or count < 0 for count in levels.values())
                or not isinstance(artifacts, dict) or not set(artifacts).issubset(expected)):
            raise RuntimeError("Crate compilation status, diagnostics or expected artifact inventory differs.")
        if code == 0:
            if (value.get("outcome") != "compiled" or set(artifacts) != expected
                    or any(levels.get(level, 0) != 0 for level in ("error", "ice"))):
                raise RuntimeError("Qualified compile repair lacks a complete successful crate artifact inventory.")
            successes += 1
        elif value.get("outcome") != "compiler_failure_observed":
            raise RuntimeError("Failed compiler status is not classified as compiler_failure_observed.")
        for key, artifact in artifacts.items():
            name = key.split("::", 1)[1]
            if not isinstance(artifact, dict) or set(artifact) != {"path", "fingerprint"}:
                raise RuntimeError("Compile artifact entry is malformed.")
            path = Path(artifact.get("path", "")); fp = artifact.get("fingerprint")
            if (path.parent != target / "debug/deps" or not re.fullmatch(re.escape(name) + r"-[0-9a-f]+", path.name)
                    or not isinstance(fp, dict) or set(fp) != {"kind", "mode", "size", "sha256"}
                    or fp.get("kind") != "file" or type(fp.get("mode")) is not int
                    or not 0 <= fp["mode"] <= 0o777 or not fp["mode"] & 0o111
                    or type(fp.get("size")) is not int or not 0 < fp["size"] <= 256 * 1024 * 1024
                    or not isinstance(fp.get("sha256"), str) or not re.fullmatch(r"[0-9a-f]{64}", fp["sha256"])):
                raise RuntimeError("Compile artifact path or ordinary executable fingerprint differs.")
    return successes


class _TransportReplacementError(RuntimeError):
    buffer_replaced = True


def _transport_open_directory(path):
    path = Path(path)
    if not path.is_absolute() or ".." in path.parts:
        raise RuntimeError("Transport requires an absolute ordinary directory.")
    fd = os.open("/", os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd)
            fd = child
        return fd
    except BaseException:
        os.close(fd)
        raise


def _transport_stamp(info):
    return tuple(getattr(info, key) for key in
                 ("st_dev", "st_ino", "st_mode", "st_uid", "st_size", "st_mtime_ns", "st_ctime_ns"))


def _transport_read_at(directory_fd, name, limit, own=False):
    fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=directory_fd)
    try:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
            raise RuntimeError("Transport input is not a bounded ordinary file: " + name)
        if own and before.st_uid != os.getuid():
            raise RuntimeError("Transport return-buffer is not owned by the current user.")
        pieces, total, digest = [], 0, hashlib.sha256()
        while True:
            chunk = os.read(fd, min(65536, limit + 1 - total))
            if not chunk:
                break
            total += len(chunk)
            if total > limit:
                raise RuntimeError("Transport input grew beyond its byte allowance.")
            pieces.append(chunk)
            digest.update(chunk)
        after = os.fstat(fd)
        current = os.stat(name, dir_fd=directory_fd, follow_symlinks=False)
        if (total != before.st_size or _transport_stamp(before) != _transport_stamp(after)
                or _transport_stamp(before) != _transport_stamp(current)):
            raise RuntimeError("Transport input changed during its ordinary-file read.")
        fp = {"kind": "file", "mode": stat.S_IMODE(before.st_mode), "size": total,
              "sha256": digest.hexdigest()}
        return b"".join(pieces), fp, _transport_stamp(before)
    finally:
        os.close(fd)


def _transport_read_file(path, limit):
    path = Path(path)
    parent = _transport_open_directory(path.parent)
    try:
        return _transport_read_at(parent, path.name, limit)
    finally:
        os.close(parent)


def _transport_emit(raw):
    view = memoryview(raw)
    while view:
        try:
            written = os.write(1, view)
        except InterruptedError:
            continue
        if written <= 0:
            raise RuntimeError("Terminal output did not accept captured bytes.")
        view = view[written:]


def _transport_unique_pairs(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise RuntimeError("Compile-repair transport JSON contains duplicate fields.")
        result[key] = value
    return result


def _transport_report(capture, repo, child_status):
    lines = capture.splitlines()
    begins = [index for index, line in enumerate(lines) if line == _TRANSPORT_BEGIN]
    ends = [index for index, line in enumerate(lines) if line == _TRANSPORT_END]
    if len(begins) != 1 or len(ends) != 1 or begins[0] >= ends[0]:
        raise RuntimeError("Completed output lacks one unambiguous controlled harness repair return JSON block.")
    report = json.loads(b"\n".join(lines[begins[0] + 1:ends[0]]).decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if (not isinstance(report, dict) or report.get("workflow") != _TRANSPORT_WORKFLOW
            or report.get("repository") != str(repo) or not isinstance(report.get("outcome"), str)):
        raise RuntimeError("Printed controlled harness repair return does not identify this coordinator/repository.")
    value = report.get("return_file")
    if not isinstance(value, str) or not value or "\x00" in value:
        raise RuntimeError("Controlled harness repair return-file path is absent or malformed.")
    path = Path(value)
    root = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW / "runs"
    if not path.is_absolute() or ".." in path.parts:
        raise RuntimeError("Controlled harness repair return-file path is not ordinary and absolute.")
    try:
        relative = path.relative_to(root)
    except ValueError as error:
        raise RuntimeError("Controlled harness repair return file is outside the owned workflow.") from error
    if len(relative.parts) != 2 or relative.name != "HARNESS_REPAIR_RETURN.json":
        raise RuntimeError("Controlled harness repair return-file layout differs from the owned run layout.")
    if "run_directory" in report and report["run_directory"] != str(path.parent):
        raise RuntimeError("Compile-repair run directory differs from its return file.")
    raw, fp, _ = _transport_read_file(path, _TRANSPORT_MAX_BYTES)
    retained = json.loads(raw.decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if retained != report:
        raise RuntimeError("Printed controlled harness repair return differs from its retained JSON file.")
    successful = report["outcome"] in {"qualified_candidate", "admission_deferred"}
    if successful:
        if report.get("original_preserved") is not True:
            raise RuntimeError("Completed controlled harness repair return lacks verified original preservation.")
        if (report.get("master_updated") is not False or report.get("other_refs_updated") is not False
                or report.get("full_coverage_claimed") is not False):
            raise RuntimeError("Harness result claims promotion, other ref mutation or full coverage.")
        if report["outcome"] == "admission_deferred":
            no_changes = ("builds_started", "crate_builds_started", "source_edits_started", "worktree_created",
                          "own_journal_created", "cache_copy_started", "native_tests_started", "repair_qualified")
            if (report.get("phase") != "build-admission" or report.get("compilations") != {}
                    or report.get("cases", []) != [] or report.get("diagnostic_only") is not True
                    or report.get("case_guards", {}) != {} or report.get("artifacts", {}) != {}
                    or report.get("completed_full_cases", []) != [] or report.get("original_full_cases_completed", False) is not False
                    or ("compile_qualified" in report and report["compile_qualified"] is not False)
                    or type(report.get("test_count")) is not int or report["test_count"] != 0
                    or any(report.get(key) is not None for key in ("candidate_head", "candidate_tree", "created_commit"))
                    or any(report.get(field) is not False for field in no_changes)):
                raise RuntimeError("Deferred harness result does not prove admission before any build, native case or mutation.")
        else:
            changed = report.get("changed_paths")
            if (report.get("phase") != "complete" or report.get("repair_qualified") is not True
                    or report.get("diagnostic_only") is not False or report.get("native_tests_started") is not True
                    or type(report.get("test_count")) is not int or report["test_count"] != 16
                    or type(report.get("expected_test_count")) is not int or report["expected_test_count"] != 16
                    or report.get("builds_started") is not True or report.get("crate_builds_started") is not True
                    or not isinstance(changed, list) or len(changed) != len(_TRANSPORT_PATHS031)
                    or set(changed) != _TRANSPORT_PATHS031
                    or any(not isinstance(report.get(key), str) or not re.fullmatch(r"[0-9a-f]{40}", report[key])
                           for key in ("candidate_head", "candidate_tree"))
                    or _transport_compilations031(report, repo) != 2):
                raise RuntimeError("Harness qualification lacks all controlled native cases, successful builds or the committed scope.")
            _transport_cases031(report)
    if (child_status == 0) != successful:
        raise RuntimeError("Coordinator exit status disagrees with its recorded outcome.")
    return report, path, raw, fp


def _transport_existing_state(repo_fd):
    try:
        raw, fp, stamp = _transport_read_at(repo_fd, "return-buffer", _TRANSPORT_MAX_BYTES, own=True)
        return fp, stamp
    except FileNotFoundError:
        return {"kind": "absent"}, None


def _transport_replace_buffer(repo, payload):
    if len(payload) > _TRANSPORT_MAX_BYTES:
        raise RuntimeError("Complete output plus return JSON exceeds the 8 MiB return-buffer allowance.")
    repo_fd = _transport_open_directory(repo)
    temporary = ".return-buffer-transport-" + uuid.uuid4().hex + ".tmp"
    made = False
    replacement_done = False
    try:
        before, stamp = _transport_existing_state(repo_fd)
        mode = before["mode"] if before["kind"] == "file" else 0o600
        fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=repo_fd)
        made = True
        with os.fdopen(fd, "wb") as stream:
            os.fchmod(stream.fileno(), mode)
            stream.write(payload)
            stream.flush()
            os.fsync(stream.fileno())
        try:
            current = os.stat("return-buffer", dir_fd=repo_fd, follow_symlinks=False)
        except FileNotFoundError:
            current = None
        if (stamp is None and current is not None) or (stamp is not None and (current is None or _transport_stamp(current) != stamp)):
            raise RuntimeError("Return-buffer changed before its authorized atomic replacement.")
        # Replacing the directory entry never follows a target symlink and
        # never writes through a source hard link to another pathname.
        os.replace(temporary, "return-buffer", src_dir_fd=repo_fd, dst_dir_fd=repo_fd)
        replacement_done = True
        made = False
        os.fsync(repo_fd)
        written, after, _ = _transport_read_at(repo_fd, "return-buffer", _TRANSPORT_MAX_BYTES, own=True)
        if written != payload or after["mode"] != mode:
            raise RuntimeError("Published return-buffer does not match the complete authorized payload.")
        return before, after
    except BaseException as error:
        if replacement_done:
            raise _TransportReplacementError("Return-buffer was replaced, but final durability/readback verification failed: " + str(error)) from error
        raise
    finally:
        if made:
            try:
                os.unlink(temporary, dir_fd=repo_fd)
            except FileNotFoundError:
                pass
        os.close(repo_fd)


def _transport_write_receipt(directory, value):
    raw = (json.dumps(value, sort_keys=True, indent=2) + "\n").encode()
    if len(raw) > 65536:
        raise RuntimeError("Transport receipt exceeds its finite allowance.")
    parent = _transport_open_directory(directory)
    try:
        fd = os.open("TRANSPORT_RETURN.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=parent)
        with os.fdopen(fd, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        os.fsync(parent)
    finally:
        os.close(parent)


def _transport_lock(repo):
    workflow = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW
    parent = _transport_open_directory(workflow)
    try:
        fd = os.open("run.lock", os.O_RDWR | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=parent)
    finally:
        os.close(parent)
    try:
        info = os.fstat(fd)
        if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid():
            raise RuntimeError("Existing compile-repair transport lock is nonordinary or foreign-owned.")
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return fd
    except BaseException:
        os.close(fd)
        raise


def run_with_transport(script_path, repo, apply):
    """Run one coordinator child, mirror every byte, then replace one buffer."""
    child = None
    child_status = None
    capture = bytearray()
    overflow = False
    mirror_error = None
    report = return_path = return_fp = None
    buffer_written = False
    buffer_replaced = False
    lock_fd = None
    previous_handlers = {}
    total_output = 0
    try:
        if type(apply) is not bool:
            raise RuntimeError("Transport apply/preview selection is malformed.")
        repo = Path(os.path.abspath(Path(repo).expanduser()))
        script_path = Path(os.path.abspath(Path(script_path).expanduser()))
        if any(char in str(repo) for char in ("\x00", "\n", "\r")):
            raise RuntimeError("Transport repository path framing differs.")
        fd = _transport_open_directory(repo)
        os.close(fd)
        _transport_read_file(script_path, 2 * 1024 * 1024)
        command = [sys.executable, "-I", "-S", str(script_path), "--coordinator", str(repo), "apply" if apply else "preview"]
        child = subprocess.Popen(command, cwd=repo, stdin=subprocess.DEVNULL,
                                 stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                 bufsize=0, start_new_session=True)
        if threading.current_thread() is threading.main_thread():
            def forward(signum, frame):
                if child.poll() is None:
                    try:
                        child.send_signal(signum)
                    except ProcessLookupError:
                        pass
            for signum in (signal.SIGINT, signal.SIGTERM):
                previous_handlers[signum] = signal.signal(signum, forward)
        while True:
            try:
                chunk = os.read(child.stdout.fileno(), 65536)
            except InterruptedError:
                continue
            if not chunk:
                break
            total_output += len(chunk)
            if mirror_error is None:
                try:
                    _transport_emit(chunk)
                except OSError as error:
                    mirror_error = str(error)
            room = _TRANSPORT_MAX_BYTES - len(capture)
            if len(chunk) > room:
                overflow = True
            if room > 0:
                capture.extend(chunk[:room])
        child_status = child.wait()
        if overflow:
            raise RuntimeError("Complete coordinator output exceeds the 8 MiB capture allowance; prior return-buffer was retained.")
        report, return_path, return_raw, return_fp = _transport_report(bytes(capture), repo, child_status)
        if mirror_error is not None:
            raise RuntimeError("Terminal mirroring failed: " + mirror_error)
        confirmation = ("RETURN_BUFFER_UPDATED_AFTER_COORDINATOR_EXIT " + str(repo / "return-buffer") + "\n").encode()
        payload = (bytes(capture) + b"\nBEGIN_HARNESS_REPAIR_RETURN_FILE_JSON\n" + return_raw
                   + (b"" if return_raw.endswith(b"\n") else b"\n")
                   + b"END_HARNESS_REPAIR_RETURN_FILE_JSON\n" + confirmation)
        if len(payload) > _TRANSPORT_MAX_BYTES:
            raise RuntimeError("Complete coordinator output plus exact return file exceeds the 8 MiB payload allowance; prior return-buffer was retained.")
        # No lock is held during the coordinator. Reacquire its existing own
        # workflow lock afterward to avoid overlapping a new snapshot/run.
        lock_fd = _transport_lock(repo)
        before, after = _transport_replace_buffer(repo, payload)
        buffer_written = True
        buffer_replaced = True
        transport = {"schema": "bitcoinrs-facade-harness-compile031-buffer-transport-v1", "outcome": "written",
                     "coordinator_exit_code": child_status, "coordinator_outcome": report["outcome"],
                     "coordinator_return_file": str(return_path), "coordinator_return_fingerprint": return_fp,
                     "target": str(repo / "return-buffer"), "captured_output_bytes": total_output,
                     "payload_bytes": len(payload), "payload_sha256": hashlib.sha256(payload).hexdigest(),
                     "full_output_captured": True, "authorized_overwrite_after_coordinator": True,
                     "coordinator_preservation_verified": (report.get("original_preserved") is True),
                     "preservation_scope": "Buffer publication occurs after coordinator exit and return-file validation; preservation results belong to the coordinator report.",
                     "buffer_before": before, "buffer_after": after}
        _transport_write_receipt(return_path.parent, transport)
        _transport_emit(confirmation)
        return 0 if child_status == 0 else 1
    except BaseException as error:
        buffer_replaced = buffer_replaced or getattr(error, "buffer_replaced", False)
        if child is not None and child_status is None:
            # Transport failures must not terminate/restart an in-flight
            # evidence capture. Finish draining its bounded coordinator lifecycle.
            try:
                while True:
                    chunk = os.read(child.stdout.fileno(), 65536)
                    if not chunk:
                        break
                    try:
                        _transport_emit(chunk)
                    except OSError:
                        pass
                child_status = child.wait()
            except BaseException as cleanup_error:
                sys.stderr.write("Transport could not finish observing the coordinator: " + str(cleanup_error)[:512] + "\n")
        actual = "coordinator_exit=" + str(child_status)
        if report is not None:
            actual += " coordinator_outcome=" + report["outcome"]
        suffix = ("; return-buffer was already written and verified" if buffer_written
                  else "; return-buffer was already replaced; final verification is incomplete" if buffer_replaced
                  else "; prior return-buffer was retained")
        sys.stderr.write("Return-buffer transport failed: " + type(error).__name__ + ": " + str(error)[:2048] + "; " + actual + suffix + ". No coordinator retry was started.\n")
        sys.stderr.flush()
        return 1
    finally:
        for signum, handler in previous_handlers.items():
            signal.signal(signum, handler)
        if child is not None and child.stdout is not None:
            child.stdout.close()
        if lock_fd is not None:
            os.close(lock_fd)


def run031(repo):
    report = {'workflow': WORKFLOW, 'controller_revision': 1, 'outcome': 'failed', 'phase': 'preflight',
        'diagnostic_only': False, 'compile_qualified': False, 'repair_qualified': False,
        'master_updated': False, 'other_refs_updated': False, 'source_edits_started': False,
        'worktree_created': False, 'own_journal_created': False, 'cache_copy_started': False, 'target_created': False,
        'builds_started': False, 'crate_builds_started': False, 'native_tests_started': False,
        'debuggers_started': False, 'test_count': 0, 'full_coverage_claimed': False,
        'validation_reused': False, 'runtime_validation_claimed': False, 'expected_test_count': 16,
        'cases': [], 'case_guards': {}, 'completed_full_cases': [], 'original_full_cases_completed': False,
        'full_harness_stress': 'not rerun', 'artifacts': {},
        'base': BASE, 'base_tree': BASE_TREE, 'branch': BRANCH,
        'targets': ['bitcoin-leveldb', 'bitcoinleveldb-harness'], 'compilations': {}, 'readouts': {},
        'source_review': source_review_notes031(),
        'scope': 'Controlled harness tests on the qualified030 parent: two pure control contracts and all fourteen original harness cases, including both original heavy workloads. One exact guarded process per case; original master and the held compile candidate remain preserved.',
        'postrun_transport': 'Replace authorized return-buffer only after coordinator exit and exact printed/saved report validation.'}
    held, core, baseline, directory, worktree, before, lineage = [], None, None, None, None, None, None
    state, state_path, files, candidate_head, installed = None, None, None, BASE, False
    try:
        repo = Path(os.path.abspath(Path(repo).expanduser())); report['repository'] = str(repo)
        baseline, previous, finalizer, core, helper = runtime031(repo); previous.checked_environment023(core)
        common = core.repository(core.Inventory(repo)); held = core.acquire_locks(common)
        own = common / 'codex-repairs' / WORKFLOW; own.mkdir(mode=0o700, exist_ok=True)
        fd = core.open_dir(own); os.close(fd)
        lock = os.open(own / 'run.lock', os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600)
        held.append((lock, own / 'run.lock'))
        if not stat.S_ISREG(os.fstat(lock).st_mode): raise RuntimeError('Compile repair workflow lock is not ordinary.')
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        candidate_original_guard023(core, repo); worktree = repo.parent / (repo.name + SUFFIX)
        before = core.normalized_original(core.protected_snapshot(core, repo), worktree)
        lineage = baseline031(baseline, core, repo)
        report.update(intake_before=lineage, preservation_before_sha256=core.digest(core.encoded(before)),
                      locks_held=[str(path) for _, path in held], worktree=str(worktree))
        runs = own / 'runs'; runs.mkdir(mode=0o700, exist_ok=True); fd = core.open_dir(runs); os.close(fd)
        directory = runs / (dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ') + '-' + uuid.uuid4().hex[:8])
        directory.mkdir(mode=0o700)
        script = directory / 'compilation-controller031.py'; raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB027)
        core.exclusive(script, raw, 0o700); files = files031(raw)
        report.update(run_directory=str(directory), controller_sha256=core.digest(raw), guard_sha256=core.GUARD_SHA,
                      source_plan={name: core.digest(value) for name, value in files.items()})
        state_path = own / 'state.json'
        guard = core.load_guard().Guard(directory / 'guard', script,
                                       dict(core.Inventory(repo).env, BITCOIN_RS_COMPILE031_REPO=str(repo)))
        def preserve():
            candidate_original_guard023(core, repo)
            if (core.normalized_original(core.protected_snapshot(core, repo), worktree) != before
                    or baseline031(baseline, core, repo) != lineage):
                raise RuntimeError('Protected original, prior compilation evidence or read input source changed.')
        def phase(label, command, kind='source', cwd=None, env=None, compiler=False):
            preserve(); report['phase'] = label
            print('Protected harness031 phase: ' + label, flush=True)
            row = guard.run(label, command, cwd or repo, env or {}, kind=kind)
            report['readouts'][label] = row
            if not compiler: core.require_guard(row)
            return row
        def git(label, args, cwd, extra=None, tag=None):
            tag = tag or label; spec = directory / (tag + '.git.spec.json'); output = directory / (tag + '.git.json')
            value = {'label': label, 'args': args, 'cwd': str(cwd), 'output': str(output)}
            value.update(extra or {}); core.exclusive(spec, core.encoded(value))
            phase('git-' + tag, [sys.executable, '-I', '-S', str(script), '--git', str(repo), str(spec)], 'build', cwd)
            evidence = json.loads(core.read_file(output, 2 * MIB027)[1]); report.setdefault('git_evidence', {})[tag] = evidence
            if evidence.get('exit_code') != 0 or evidence.get('args') != args:
                raise RuntimeError('Exact isolated compile repair Git evidence differs.')
            return evidence['stdout'].strip()
        phase('preflight', [sys.executable, '-I', '-S', '-c', "print('GUARD_PREFLIGHT_OK')"], 'probe')
        inputs = directory / 'inputs-before.json'
        phase('inputs-before', [sys.executable, '-I', '-S', str(script), '--inputs', str(repo), str(inputs)])
        if json.loads(core.read_file(inputs, 2 * MIB027)[1]) != lineage:
            raise RuntimeError('Fresh input proof differs from the retained028 source/compiler binding.')
        if os.path.lexists(state_path):
            raise RuntimeError('An earlier compile031 reservation is retained; automatic recompilation is blocked.')
        inv = candidate_original_guard023(core, repo)
        _, existing = inv.git('for-each-ref', '--format=%(refname)', 'refs/heads/' + BRANCH)
        if existing.strip() or os.path.lexists(worktree) or os.path.lexists(own / 'cargo-target'):
            raise RuntimeError('Compile repair branch, checkout or private target is occupied; retained without reuse.')
        admission_command = [sys.executable, '-I', '-S', '-c', "print('COMPILE031_BUILD_ADMISSION_OK')"]
        admission = phase('build-admission', admission_command, 'build', compiler=True)
        report['build_admission'] = build_admission031(core, admission, admission_command, repo)
        if report['build_admission']['admitted'] is False:
            _, existing = inv.git('for-each-ref', '--format=%(refname)', 'refs/heads/' + BRANCH)
            if existing.strip() or any(os.path.lexists(p) for p in (state_path, worktree, own / 'cargo-target')):
                raise RuntimeError('Deferred compile admission cannot reserve candidate or cache state.')
            raise AdmissionDeferred031()
        state = {'workflow': WORKFLOW, 'base': BASE, 'branch': BRANCH, 'worktree': str(worktree),
            'run_directory': str(directory), 'phase': 'reserved', 'source_plan': report['source_plan']}
        core.atomic_json(state_path, state); report['own_journal_created'] = True
        identity_spec, identity_out = directory / 'identity-input.json', directory / 'identity.json'
        core.exclusive(identity_spec, core.encoded({key: os.environ[key] for key in finalizer.IDENTITY_ENV if key in os.environ}))
        phase('read-configured-identity', [sys.executable, '-I', '-S', str(script), '--identity', str(repo), str(identity_spec), str(identity_out)])
        ident = json.loads(core.read_file(identity_out, MIB027)[1]); report['configured_identity'] = ident
        identity_env = {'GIT_' + role.upper() + '_' + field.upper(): ident[role][field.lower()]
                        for role in ('author', 'committer') for field in ('NAME', 'EMAIL')}
        git('worktree', ['worktree', 'add', '-b', BRANCH, str(worktree), BASE], repo)
        report['worktree_created'] = True; report['new_ref_created'] = 'refs/heads/' + BRANCH
        report['source_edits_started'] = True
        phase('install-source', [sys.executable, '-I', '-S', str(script), '--install', str(repo)], 'source', worktree)
        git('add', ['add', '--', *sorted(files)], worktree)
        check_candidate023(core, repo, worktree, files, BASE); installed = True
        state['phase'] = 'installed'; core.atomic_json(state_path, state)
        target = own / 'cargo-target'; report['target_directory'] = str(target)
        target.mkdir(mode=0o700); report['target_created'] = True
        fd = core.open_dir(target); os.close(fd)
        report['cache_policy'] = 'Fresh empty private target; no optional cache copy and no prior cache mutation.'
        nix_shell = shutil.which('nix-shell', path=core.Inventory(repo).env.get('PATH'))
        if nix_shell is None:
            raise RuntimeError('Offline Nix shell entry point is unavailable; no alternate build was attempted.')
        # Preserve the dispatch entry name: resolving the Nix symlink changes argv[0].
        nix_shell = os.path.abspath(nix_shell)
        for label in ('facade', 'harness'):
            preserve(); check_candidate023(core, repo, worktree, files, BASE)
            spec_path = directory / ('cargo-' + label + '.spec.json'); evidence_path = directory / ('cargo-' + label + '.json')
            spec = {'command': COMMANDS031[label], 'cwd': str(worktree), 'target_directory': str(target),
                    'evidence_path': str(evidence_path)}
            core.exclusive(spec_path, core.encoded(spec)); state.update(phase='compiling', compilation_pending=label)
            core.atomic_json(state_path, state)
            command = [nix_shell, '--option', 'max-jobs', '0', '--option', 'builders', '', '--option', 'substitute', 'false',
                str(worktree / 'shell.nix'), '--run',
                shlex.join([sys.executable, '-I', '-S', str(script), '--cargo-observer', str(repo), str(spec_path)])]
            report.update(builds_started=True, crate_builds_started=True)
            row = phase('compile-' + label, command, 'build', worktree,
                {'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '1', 'CARGO_INCREMENTAL': '0', 'RUST_LOG': 'off'}, compiler=True)
            if not evidence_path.exists(): raise RuntimeError('Fresh Cargo observer did not publish bounded evidence.')
            evidence = json.loads(core.read_file(evidence_path, MIB027)[1]); report.setdefault('cargo_evidence', {})[label] = evidence
            summary = qualify_compile031(core, label, row, evidence, spec, target,
                guard_command=command, run_directory=directory)
            summary['evidence_file'] = str(evidence_path)
            summary['evidence_fingerprint'] = core.read_file(evidence_path, MIB027, content=False)[0]
            report['compilations'][label] = summary
            state.pop('compilation_pending'); state.setdefault('compilations', {})[label] = summary['outcome']
            state['phase'] = 'compiled-' + label; core.atomic_json(state_path, state)
            check_candidate023(core, repo, worktree, files, BASE)
        report['compile_qualified'] = True
        report['artifacts'] = flatten_artifacts031(report['compilations'])
        lists = {'all': {}, 'ignored': {}}
        for key in sorted(report['artifacts']):
            executable = report['artifacts'][key]['path']; target_name = key.split('::', 1)[1]
            for selection, arguments in (('all', ['--list']), ('ignored', ['--list', '--ignored'])):
                check_candidate023(core, repo, worktree, files, BASE)
                check_artifacts031(core, report['artifacts'])
                state['native_pending'] = {'kind': 'listing', 'target': key, 'selection': selection}
                core.atomic_json(state_path, state); report['native_tests_started'] = True
                row = phase('list-' + target_name + '-' + selection, [executable, *arguments], 'case', worktree, {'RUST_LOG': 'off'})
                text = core.read_file(Path(row['log']), MIB027)[1].decode(errors='replace')
                lists[selection][key] = [line[:-6] for line in text.splitlines() if line.endswith(': test')]
                state.pop('native_pending'); core.atomic_json(state_path, state)
        report['test_lists'] = lists; selected = selected_cases031(lists)
        for item in selected:
            preserve(); check_candidate023(core, repo, worktree, files, BASE)
            check_artifacts031(core, report['artifacts'])
            data = directory / ('case-data-' + item['suffix']); data.mkdir(mode=0o700)
            state['native_pending'] = {'run': str(directory), 'case': item['name'], 'profile': item['profile']}
            core.atomic_json(state_path, state)
            report.update(phase=item['suffix'], native_tests_started=True)
            print('Protected harness031 phase: ' + item['suffix'], flush=True)
            executable = report['artifacts']['bitcoinleveldb-harness::' + item['target']]['path']
            command = case_command031(executable, item); env = case_environment031(item, data)
            kind = 'stress' if item['profile'] == 'full_original' else 'case'
            row = guard.run(item['suffix'], command, worktree, env, kind=kind, tmpdir=data)
            report['readouts'][item['suffix']] = row
            text = core.read_file(Path(row['log']), MIB027)[1].decode(errors='replace') if row.get('log') else ''
            assessed = core.assess_case(row, text, item['name'], item['marker'])
            report['cases'].append(dict(assessed, label=item['suffix'], target=item['target'], profile=item['profile'],
                ignored_by_default=item['ignored'], log=text))
            report['test_count'] = len(report['cases'])
            if assessed.get('outcome') != 'passed':
                state['native_failure'] = dict(state['native_pending']); core.atomic_json(state_path, state)
                raise RuntimeError('Harness031 case did not pass; its pending/failure evidence is retained and automatic retry is blocked.')
            report['case_guards'][item['suffix']] = case_guard_binding031(core, report, item, row)
            if item['profile'] == 'full_original': report['completed_full_cases'].append(item['suffix'])
            state.pop('native_pending'); core.atomic_json(state_path, state)
        helper.check_native_journal015(state)
        if report['test_count'] != 16 or report['completed_full_cases'] != ['harness_randomized', 'harness_randomized_longdb']:
            raise RuntimeError('Harness031 original and control case inventory is incomplete.')
        report.update(original_full_cases_completed=True, full_harness_stress='passed')
        inputs = directory / 'inputs-after.json'
        phase('inputs-after', [sys.executable, '-I', '-S', str(script), '--inputs', str(repo), str(inputs)])
        if json.loads(core.read_file(inputs, 2 * MIB027)[1]) != lineage:
            raise RuntimeError('Original source or held030 compilation evidence changed during harness qualification.')
        preserve(); check_artifacts031(core, report['artifacts'])
        receipt = {'workflow': WORKFLOW, 'controller_revision': 1, 'base': BASE, 'base_tree': BASE_TREE,
            'original_master': MASTER_BASE, 'original_master_tree': MASTER_TREE,
            'repair_qualified': True, 'runtime_validation_claimed': True,
            'source_plan': report['source_plan'], 'source_review': report['source_review'],
            'compilations': report['compilations'], 'commands': COMMANDS031, 'artifacts': report['artifacts'],
            'cases': [{k: v for k, v in case.items() if k != 'log'} for case in report['cases']],
            'case_guards': report['case_guards'], 'test_lists': report['test_lists'],
            'guard_sha256': core.GUARD_SHA, 'run_directory': str(directory), 'test_count': 16,
            'native_tests_started': True, 'full_coverage_claimed': False,
            'original_full_cases_completed': True, 'completed_full_cases': report['completed_full_cases'],
            'full_harness_stress': 'passed', 'prior_report': lineage['report'],
            'prior_report_fingerprint': lineage['report_fingerprint']}
        destination = worktree / RECEIPT_PATH; destination.parent.mkdir(parents=True, exist_ok=True)
        fd = core.open_dir(destination.parent); os.close(fd)
        helper.candidate_exclusive015(core, destination, core.encoded(receipt), 0o644)
        files[RECEIPT_PATH] = core.encoded(receipt)
        git('add', ['add', '--', *sorted(files)], worktree, tag='add-final')
        check_candidate023(core, repo, worktree, files, BASE)
        tree = git('tree', ['write-tree'], worktree)
        if not _candidate_oid023(tree): raise RuntimeError('Harness031 candidate tree identity is absent.')
        message = directory / 'commit-message.txt'; core.exclusive(message, COMMIT_MESSAGE)
        intent = {'tree': tree, 'parents': [BASE], 'identity': ident, 'message_sha256': core.digest(COMMIT_MESSAGE)}
        state.update(phase='commit-intent', receipt=receipt, commit_intent=intent); core.atomic_json(state_path, state)
        head = git('commit-tree', ['commit-tree', tree, '-p', BASE, '-F', str(message)], worktree,
            {'tree': tree, 'identity_env': identity_env})
        if not _candidate_oid023(head): raise RuntimeError('Harness031 candidate commit identity is absent.')
        state['created_commit'] = head; core.atomic_json(state_path, state)
        git('head', ['update-ref', 'refs/heads/' + BRANCH, head, BASE], worktree, {'head': head})
        candidate_head = head
        helper.verify_candidate_commit(finalizer, core, repo, head, intent, files)
        preserve(); check_artifacts031(core, report['artifacts']); state['head'] = head
        report.update(outcome='qualified_candidate', phase='complete', repair_qualified=True,
            runtime_validation_claimed=True, candidate_head=head, candidate_tree=tree, receipt=receipt, changed_paths=sorted(files))
    except BaseException as error:
        if isinstance(error, AdmissionDeferred031):
            report.update(outcome='admission_deferred', phase='build-admission', diagnostic_only=True,
                diagnostic_result='Unchanged build admission refused before candidate/cache reservation.')
        else:
            report.update(outcome='failed', repair_qualified=False, error=type(error).__name__ + ': ' + str(error))
    finally:
        if before is not None:
            try:
                report['intake_after'] = baseline031(baseline, core, repo)
                after = core.normalized_original(core.protected_snapshot(core, repo), worktree)
                report['preservation_after_sha256'] = core.digest(core.encoded(after))
                report['original_preserved'] = after == before and report['intake_after'] == lineage
                if report['original_preserved'] is not True: raise RuntimeError('Original user/evidence/source state was not preserved.')
                if installed:
                    report['candidate_after'] = check_candidate023(core, repo, worktree, files, candidate_head)
                if set(report['compilations']) == {'facade', 'harness'}:
                    artifact_check031(core, report['compilations'])
            except BaseException as error:
                report.update(outcome='failed', repair_qualified=False, original_preserved=False, preservation_error=str(error))
        if directory is not None:
            report['return_file'] = str(directory / 'HARNESS_REPAIR_RETURN.json')
            if len(core.encoded(report)) > 3 * MIB027:
                report.update(outcome='failed', repair_qualified=False, publication_error='Compile repair return exceeds its 3 MiB allowance.')
                report.pop('intake_before', None); report.pop('intake_after', None)
            try:
                core.atomic_json(Path(report['return_file']), report)
                if state is not None and report['outcome'] == 'qualified_candidate':
                    state.update(phase='complete',
                        report=report['return_file'], report_fingerprint=core.read_file(Path(report['return_file']), 3 * MIB027, content=False)[0])
                    core.atomic_json(state_path, state)
            except BaseException as error:
                report.update(outcome='failed', repair_qualified=False, publication_error=str(error))
                core.atomic_json(Path(report['return_file']), report)
        for fd, _ in held: os.close(fd)
    return report


def main031():
    sys.dont_write_bytecode = True; resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    if len(sys.argv) == 3 and sys.argv[1] == '--guard-child':
        plan = json.loads(bootstrap_read(Path(sys.argv[2]), MIB027))
        _, _, _, core, _ = runtime031(Path(plan['env']['BITCOIN_RS_COMPILE031_REPO']))
        return core.load_guard().guard_child(Path(sys.argv[2]))
    if len(sys.argv) == 4 and sys.argv[1] == '--inputs':
        return inputs_worker031(Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 5 and sys.argv[1] == '--identity':
        return identity_worker031(Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]))
    if len(sys.argv) == 4 and sys.argv[1] == '--git':
        return git_worker031(Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 3 and sys.argv[1] == '--install': return install_worker031(Path(sys.argv[2]))
    if len(sys.argv) == 4 and sys.argv[1] == '--cargo-observer':
        return observer_worker031(Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 4 and sys.argv[1] == '--coordinator' and sys.argv[3] == 'apply':
        resource.setrlimit(resource.RLIMIT_AS, (768 * MIB027,) * 2)
        signal.signal(signal.SIGALRM, lambda *_: (_ for _ in ()).throw(RuntimeError('Compile031 coordinator deadline reached.')))
        signal.alarm(3600)
        report = run031(Path(sys.argv[2]))
        print('BEGIN_HARNESS_REPAIR_RETURN', flush=True); print(json.dumps(report, indent=2), flush=True)
        print('END_HARNESS_REPAIR_RETURN', flush=True)
        return 0 if report['outcome'] in {'qualified_candidate', 'admission_deferred'} else 1
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=Path.home() / 'bitcoin-rs')
    args = parser.parse_args()
    return run_with_transport(Path(__file__).absolute(), Path(os.path.abspath(args.repo.expanduser())), True)


if __name__ == '__main__': raise SystemExit(main031())
