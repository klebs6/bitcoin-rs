#!/usr/bin/env python3
"""Install and qualify an isolated resource-controlled Issue320 test candidate. Master is never updated."""
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
import signal
import stat
import subprocess
import sys
import types
import uuid
MIB020=1024*1024
CONTROLLER_REVISION=2
ARTIFACT_KEY='bitcoinleveldbt-issue320::bitcoinleveldbt_issue320'
SOURCE_BEFORE_SHA='29a8594d990cebc044926ce878e584d6af7b4b524a5aa9a132cbaf747cd78e71'


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


SOURCE_BYTES=b'// ---------------- [ File: bitcoinleveldbt-issue320/src/bitcoinleveldbt_issue320.rs ]\ncrate::ix!();\n\n// Execution profiles change only these four workload bounds. The original\n// profile retains seed zero, the 1 KiB generator, flags and PRNG/C-API ordering.\n#[derive(Clone, Copy)]\nstruct Issue320ExecutionProfile {\n    case_name: &\'static str,\n    model_slots: usize,\n    snapshot_slots: usize,\n    target_size: u32,\n    steps: u32,\n}\n\nconst ISSUE320_FULL_PROFILE: Issue320ExecutionProfile = Issue320ExecutionProfile {\n    case_name: "issue320_test", model_slots: 10000, snapshot_slots: 100,\n    target_size: 10000, steps: 200000,\n};\nconst ISSUE320_BOUNDED_PROFILE: Issue320ExecutionProfile = Issue320ExecutionProfile {\n    case_name: "issue320_bounded_diagnostic", model_slots: 64, snapshot_slots: 8,\n    target_size: 32, steps: 256,\n};\n\nfn issue320_guard_limits_valid(memory: &str, pids: &str, swap_limit_text: &str, oom: &str, cpu: &str) -> bool {\n    let memory = match memory.parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    let pids = match pids.parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    let fields: Vec<&str> = cpu.split_whitespace().collect();\n    if fields.len() != 2 { return false; }\n    let quota = match fields[0].parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    let period = match fields[1].parse::<u64>() { Ok(value) => value, Err(_) => return false };\n    memory > 0 && memory <= 2 * 1024 * 1024 * 1024\n        && pids > 0 && pids <= 64 && swap_limit_text == "0" && oom == "1"\n        && quota > 0 && period > 0 && quota <= period\n}\n\nfn issue320_read_control(path: &std::path::Path) -> String {\n    match std::fs::read_to_string(path) {\n        Ok(value) => value.trim().to_owned(),\n        Err(error) => panic!("use the protected Issue320 runner: cannot read {}: {}", path.display(), error),\n    }\n}\n\nfn issue320_require_protection() {\n    assert!(matches!(std::env::var("BITCOIN_RS_ISSUE320_GUARD"), Ok(value) if value == "v1"),\n            "use the protected Issue320 runner; this case requires verified cgroup limits");\n    let memberships = issue320_read_control(std::path::Path::new("/proc/self/cgroup"));\n    let unified = match memberships.lines().find_map(|line| line.strip_prefix("0::")) {\n        Some(value) => value,\n        None => panic!("use the protected Issue320 runner: unified cgroup membership missing"),\n    };\n    let membership_path = std::path::Path::new(unified);\n    assert!(membership_path.is_absolute(), "invalid unified cgroup path");\n    assert!(membership_path.components().all(|part| matches!(part,\n            std::path::Component::RootDir | std::path::Component::Normal(_))),\n            "invalid unified cgroup path components");\n    let group = std::path::Path::new("/sys/fs/cgroup").join(unified.trim_start_matches(\'/\'));\n    let memory = issue320_read_control(&group.join("memory.max"));\n    let pids = issue320_read_control(&group.join("pids.max"));\n    let swap_limit_text = issue320_read_control(&group.join("memory.swap.max"));\n    let oom = issue320_read_control(&group.join("memory.oom.group"));\n    let cpu = issue320_read_control(&group.join("cpu.max"));\n    assert!(issue320_guard_limits_valid(&memory, &pids, &swap_limit_text, &oom, &cpu),\n            "use the protected Issue320 runner: finite memory<=2GiB,pids<=64,swap=0,grouped OOM and CPU<=1 required");\n    eprintln!("ISSUE320_GUARD cgroup={} memory={} pids={} cpu={}", group.display(), memory, pids, cpu);\n}\n\nfn issue320_progress(profile: Issue320ExecutionProfile, phase: &str, done: u32) {\n    eprintln!("ISSUE320_PROGRESS case={} phase={} done={} total={}", profile.case_name, phase, done, profile.steps);\n}\n\nfn issue320_progress_step(profile: Issue320ExecutionProfile, done: u32) -> bool {\n    let interval = if profile.steps <= 256 { 32 } else { 1000 };\n    done == 1 || done == profile.steps || done % interval == 0\n}\n\n#[test]\nfn issue320_execution_profile_contract() {\n    assert_eq!((ISSUE320_FULL_PROFILE.model_slots, ISSUE320_FULL_PROFILE.snapshot_slots,\n                ISSUE320_FULL_PROFILE.target_size, ISSUE320_FULL_PROFILE.steps), (10000, 100, 10000, 200000));\n    assert_eq!((ISSUE320_BOUNDED_PROFILE.model_slots, ISSUE320_BOUNDED_PROFILE.snapshot_slots,\n                ISSUE320_BOUNDED_PROFILE.target_size, ISSUE320_BOUNDED_PROFILE.steps), (64, 8, 32, 256));\n    assert!(ISSUE320_BOUNDED_PROFILE.target_size < ISSUE320_BOUNDED_PROFILE.model_slots as u32);\n    assert_eq!(ISSUE320_FULL_PROFILE.case_name, "issue320_test");\n    assert_eq!(ISSUE320_BOUNDED_PROFILE.case_name, "issue320_bounded_diagnostic");\n}\n\n#[test]\nfn issue320_guard_predicate_rejects_unbounded_and_invalid_controls() {\n    assert!(issue320_guard_limits_valid("2147483648", "64", "0", "1", "100000 100000"));\n    assert!(issue320_guard_limits_valid("536870912", "32", "0", "1", "50000 100000"));\n    for controls in [\n        ("max", "64", "0", "1", "100000 100000"),\n        ("0", "64", "0", "1", "100000 100000"),\n        ("2147483649", "64", "0", "1", "100000 100000"),\n        ("536870912", "max", "0", "1", "100000 100000"),\n        ("536870912", "65", "0", "1", "100000 100000"),\n        ("536870912", "0", "0", "1", "100000 100000"),\n        ("536870912", "64", "max", "1", "100000 100000"),\n        ("536870912", "64", "1", "1", "100000 100000"),\n        ("536870912", "64", "0", "0", "100000 100000"),\n        ("536870912", "64", "0", "1", "max 100000"),\n        ("536870912", "64", "0", "1", "100001 100000"),\n        ("536870912", "64", "0", "1", "0 100000"),\n        ("536870912", "64", "0", "1", "100000 0"),\n        ("536870912", "64", "0", "1", "100000"),\n    ] {\n        assert!(!issue320_guard_limits_valid(controls.0, controls.1, controls.2, controls.3, controls.4));\n    }\n}\n\n\n//-------------------------------------------[.cpp/bitcoin/src/leveldb/issues/issue320_test.cc]\n\n/**\n  | Creates a random number in the range\n  | of [0, max).\n  |\n  */\nfn generate_random_number(max: i32) -> i32 {\n    /*\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "generate_random_number_entry",\n        max = max\n    );\n    */\n\n    let out = if max <= 0 {\n        0i32\n    } else {\n        unsafe { libc::rand() % max }\n    };\n\n    /*\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "generate_random_number_exit",\n        result = out\n    );\n    */\n\n    out\n}\n\nfn create_random_string(index: i32) -> String {\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "create_random_string_entry",\n        index = index\n    );\n\n    const BITCOINLEVELDB_TEST_ISSUE320_TEST_RANDOM_STRING_LEN: usize = 1024usize;\n    let mut bytes = [0u8; BITCOINLEVELDB_TEST_ISSUE320_TEST_RANDOM_STRING_LEN];\n\n    let mut i: usize = 0usize;\n    while i < 8usize {\n        let nibble = ((index >> (4 * (i as i32))) & 0x0f) as u8;\n        bytes[i] = b\'a\' + nibble;\n        i += 1usize;\n    }\n\n    while i < bytes.len() {\n        let rnd = generate_random_number(26);\n        let ch = match u8::try_from(rnd) {\n            Ok(v) => v,\n            Err(_) => 0u8,\n        };\n        bytes[i] = b\'a\' + ch;\n        i += 1usize;\n    }\n\n    let out = match String::from_utf8(bytes.to_vec()) {\n        Ok(s) => s,\n        Err(e) => {\n            let recovered = e.into_bytes();\n            String::from_utf8_lossy(recovered.as_slice()).into_owned()\n        }\n    };\n\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "create_random_string_exit",\n        len = out.len()\n    );\n\n    out\n}\n\nstruct Issue320 {}\n\nfn bitcoinleveldb_test__issue320_test_rs__free_err_if_non_null(err: *mut u8) {\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "free_err_if_non_null_entry",\n        err_is_null = err.is_null()\n    );\n\n    if !err.is_null() {\n        leveldb_free(err as *mut c_void);\n    }\n\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "free_err_if_non_null_exit"\n    );\n}\n\nfn bitcoinleveldb_test__issue320_test_rs__get_value_string_or_panic(\n    db:       *mut LevelDB,\n    options:  *const LevelDBReadOptions,\n    key:      &String,\n) -> String {\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "get_value_string_or_panic_entry",\n        db_is_null = db.is_null(),\n        options_is_null = options.is_null(),\n        key_len = key.len()\n    );\n\n    let mut err: *mut u8 = core::ptr::null_mut();\n    let mut val_len: usize = 0usize;\n\n    let val = leveldb_get(\n        db,\n        options,\n        key.as_bytes().as_ptr(),\n        key.len(),\n        (&mut val_len) as *mut usize,\n        (&mut err) as *mut *mut u8,\n    );\n\n    assert!(err.is_null());\n    assert!(!val.is_null());\n\n    let out = {\n        let bytes = unsafe { core::slice::from_raw_parts(val as *const u8, val_len) };\n        String::from_utf8_lossy(bytes).into_owned()\n    };\n\n    leveldb_free(val as *mut c_void);\n\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "get_value_string_or_panic_exit",\n        value_len = out.len()\n    );\n\n    out\n}\n\n#[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\nfn issue320_test() {\n    issue320_require_protection();\n    issue320_run_profile(ISSUE320_FULL_PROFILE);\n}\n\n#[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\nfn issue320_bounded_diagnostic() {\n    issue320_require_protection();\n    issue320_run_profile(ISSUE320_BOUNDED_PROFILE);\n}\n\nfn issue320_run_profile(profile: Issue320ExecutionProfile) {\n    // ISSUE320020_MONITOR_BEGIN:entry\n    issue320_progress(profile, "entry", 0);\n    // ISSUE320020_MONITOR_END:entry\n\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "issue320_test_entry"\n    );\n\n    unsafe {\n        libc::srand(0u32);\n\n        let delete_before_put = false;\n        let keep_snapshots = true;\n\n        let mut test_map: Vec<Option<(String, String)>> = vec![None; profile.model_slots];\n        let mut snapshots: Vec<*const LevelDBSnapshot> =\n            vec![core::ptr::null(); profile.snapshot_slots];\n\n        let dbpath = unique_db_path("/leveldb_issue320_test");\n\n        let mut dbname = dbpath.into_bytes();\n        dbname.push(0u8);\n\n        let options: *mut LevelDBOptions = leveldb_options_create();\n        let roptions: *mut LevelDBReadOptions = leveldb_readoptions_create();\n        let woptions: *mut LevelDBWriteOptions = leveldb_writeoptions_create();\n\n        assert!(!options.is_null());\n        assert!(!roptions.is_null());\n        assert!(!woptions.is_null());\n\n        leveldb_options_set_create_if_missing(options, 1u8);\n\n\n        // ISSUE320020_MONITOR_BEGIN:initial-destroy-before\n        issue320_progress(profile, "initial_destroy.before", 0);\n        // ISSUE320020_MONITOR_END:initial-destroy-before\n        let mut destroy_err: *mut u8 = core::ptr::null_mut();\n        leveldb_destroy_db(\n            options,\n            dbname.as_ptr(),\n            (&mut destroy_err) as *mut *mut u8,\n        );\n        bitcoinleveldb_test__issue320_test_rs__free_err_if_non_null(destroy_err);\n        // ISSUE320020_MONITOR_BEGIN:initial-destroy-after\n        issue320_progress(profile, "initial_destroy.after", 0);\n        // ISSUE320020_MONITOR_END:initial-destroy-after\n\n\n        let mut err: *mut u8 = core::ptr::null_mut();\n\n        // ISSUE320020_MONITOR_BEGIN:open-before\n        issue320_progress(profile, "open.before", 0);\n        // ISSUE320020_MONITOR_END:open-before\n        let db: *mut LevelDB = leveldb_open(\n            options,\n            dbname.as_ptr(),\n            (&mut err) as *mut *mut u8,\n        );\n        assert!(err.is_null());\n        assert!(!db.is_null());\n        // ISSUE320020_MONITOR_BEGIN:open-after\n        issue320_progress(profile, "open.after", 0);\n        // ISSUE320020_MONITOR_END:open-after\n\n\n        let target_size: u32 = profile.target_size;\n        let mut num_items: u32 = 0u32;\n        let mut count: u32 = 0u32;\n        // ISSUE320020_MONITOR_BEGIN:counters\n        let mut inserted = 0u32;\n        let mut overwritten = 0u32;\n        let mut deleted = 0u32;\n        let mut snapshots_created = 0u32;\n        let mut snapshots_rotated = 0u32;\n        // ISSUE320020_MONITOR_END:counters\n\n\n        while count < profile.steps {\n            count += 1u32;\n\n            // ISSUE320020_MONITOR_BEGIN:progress\n            if issue320_progress_step(profile, count) {\n                issue320_progress(profile, "iteration", count);\n            }\n            // ISSUE320020_MONITOR_END:progress\n\n\n            let index = generate_random_number(test_map.len() as i32) as usize;\n            let batch: *mut LevelDBWriteBatch = leveldb_writebatch_create();\n            assert!(!batch.is_null());\n\n            match test_map[index].take() {\n                None => {\n                    num_items += 1u32;\n                    // ISSUE320020_MONITOR_BEGIN:insert-count\n                    inserted += 1;\n                    // ISSUE320020_MONITOR_END:insert-count\n\n\n                    let key = create_random_string(index as i32);\n                    let value = create_random_string(index as i32);\n\n                    leveldb_writebatch_put(\n                        batch,\n                        key.as_bytes().as_ptr(),\n                        key.len(),\n                        value.as_bytes().as_ptr(),\n                        value.len(),\n                    );\n\n                    test_map[index] = Some((key, value));\n                }\n                Some((key, mut value)) => {\n                    let old_value =\n                        bitcoinleveldb_test__issue320_test_rs__get_value_string_or_panic(\n                            db,\n                            roptions,\n                            &key,\n                        );\n\n                    if old_value != value {\n                        eprintln!("ERROR incorrect value returned by Get");\n                        eprintln!("  count={}", count);\n                        eprintln!("  old value={}", old_value);\n                        eprintln!("  test_map[index]->second={}", value);\n                        eprintln!("  test_map[index]->first={}", key);\n                        eprintln!("  index={}", index);\n                        assert_eq!(old_value, value);\n                    }\n\n                    if num_items >= target_size && generate_random_number(100) > 30 {\n                        leveldb_writebatch_delete(\n                            batch,\n                            key.as_bytes().as_ptr(),\n                            key.len(),\n                        );\n                        num_items -= 1u32;\n                        // ISSUE320020_MONITOR_BEGIN:delete-count\n                        deleted += 1;\n                        // ISSUE320020_MONITOR_END:delete-count\n\n                        test_map[index] = None;\n                    } else {\n\n                        // ISSUE320020_MONITOR_BEGIN:overwrite-count\n                        overwritten += 1;\n                        // ISSUE320020_MONITOR_END:overwrite-count\n                        value = create_random_string(index as i32);\n                        if delete_before_put {\n                            leveldb_writebatch_delete(\n                                batch,\n                                key.as_bytes().as_ptr(),\n                                key.len(),\n                            );\n                        }\n                        leveldb_writebatch_put(\n                            batch,\n                            key.as_bytes().as_ptr(),\n                            key.len(),\n                            value.as_bytes().as_ptr(),\n                            value.len(),\n                        );\n                        test_map[index] = Some((key, value));\n                    }\n                }\n            }\n\n\n            // ISSUE320020_MONITOR_BEGIN:write-before\n            if issue320_progress_step(profile, count) {\n                issue320_progress(profile, "write.before", count);\n            }\n            // ISSUE320020_MONITOR_END:write-before\n            leveldb_write(\n                db,\n                woptions,\n                batch,\n                (&mut err) as *mut *mut u8,\n            );\n            assert!(err.is_null());\n            leveldb_writebatch_destroy(batch);\n            // ISSUE320020_MONITOR_BEGIN:write-after\n            if issue320_progress_step(profile, count) {\n                issue320_progress(profile, "write.after", count);\n            }\n            // ISSUE320020_MONITOR_END:write-after\n\n\n            if keep_snapshots && generate_random_number(10) == 0 {\n                let i = generate_random_number(snapshots.len() as i32) as usize;\n                if !snapshots[i].is_null() {\n\n                    // ISSUE320020_MONITOR_BEGIN:snapshot-rotation-count\n                    snapshots_rotated += 1;\n                    // ISSUE320020_MONITOR_END:snapshot-rotation-count\n                    leveldb_release_snapshot(db, snapshots[i]);\n                }\n                snapshots[i] = leveldb_create_snapshot(db);\n                // ISSUE320020_MONITOR_BEGIN:snapshot-count\n                snapshots_created += 1;\n                // ISSUE320020_MONITOR_END:snapshot-count\n\n            }\n        }\n\n\n        // ISSUE320020_MONITOR_BEGIN:release-before\n        issue320_progress(profile, "release.before", count);\n        // ISSUE320020_MONITOR_END:release-before\n        for snapshot in snapshots.iter() {\n            if !snapshot.is_null() {\n                leveldb_release_snapshot(db, *snapshot);\n            }\n        }\n\n\n        // ISSUE320020_MONITOR_BEGIN:release-after-close-before\n        issue320_progress(profile, "release.after", count);\n        issue320_progress(profile, "close.before", count);\n        // ISSUE320020_MONITOR_END:release-after-close-before\n        leveldb_close(db);\n        // ISSUE320020_MONITOR_BEGIN:close-after\n        issue320_progress(profile, "close.after", count);\n        // ISSUE320020_MONITOR_END:close-after\n\n\n\n        // ISSUE320020_MONITOR_BEGIN:cleanup-before\n        issue320_progress(profile, "cleanup.before", count);\n        // ISSUE320020_MONITOR_END:cleanup-before\n        let mut cleanup_err: *mut u8 = core::ptr::null_mut();\n        leveldb_destroy_db(\n            options,\n            dbname.as_ptr(),\n            (&mut cleanup_err) as *mut *mut u8,\n        );\n        bitcoinleveldb_test__issue320_test_rs__free_err_if_non_null(cleanup_err);\n\n        leveldb_readoptions_destroy(roptions);\n        leveldb_writeoptions_destroy(woptions);\n        leveldb_options_destroy(options);\n        // ISSUE320020_MONITOR_BEGIN:cleanup-after-counts\n        issue320_progress(profile, "cleanup.after", count);\n        eprintln!("ISSUE320_COUNTS case={} steps={} inserted={} overwritten={} deleted={} snapshots={} rotations={}",\n                  profile.case_name, count, inserted, overwritten, deleted, snapshots_created, snapshots_rotated);\n        // ISSUE320020_MONITOR_END:cleanup-after-counts\n\n    }\n\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "issue320_test_exit"\n    );\n\n    // ISSUE320020_MONITOR_BEGIN:completed\n    eprintln!("ISSUE320_CASE_OK case={}", profile.case_name);\n    // ISSUE320020_MONITOR_END:completed\n}\n\nfn issuesissue320_test_main(\n    _argc: i32,\n    _argv: *mut *mut u8,\n) -> i32 {\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "issuesissue320_test_main_entry"\n    );\n\n    let rc = run_all_tests();\n\n    trace!(\n        target: "bitcoinleveldbt_issue320::issue320_test",\n        event = "issuesissue320_test_main_exit",\n        result = rc\n    );\n\n    rc\n}\n\n#[cfg(test)]\nmod issue320_snapshot_probe_tests {\n    use super::*;\n\n    const ISSUE320_PROBE_KEY_LEN: usize = 1024;\n    const ISSUE320_PROBE_VALUE_LEN: usize = 1024;\n    const ISSUE320_PROBE_FILLER_COUNT: usize = 12;\n    const ISSUE320_PROBE_HISTORY_LIMIT: usize = 24;\n    const ISSUE320_PROBE_MAX_MANUAL_LEVEL: i32 = 1;\n\n    fn issue320_probe_blob(tag: u8, a: usize, b: usize, len: usize) -> String {\n        let mut out = format!("{}:{:08x}:{:08x}:", tag as char, a as u32, b as u32);\n\n        let mut x: u64 =\n            0x9e37_79b9_7f4a_7c15u64\n                ^ ((tag as u64) << 56)\n                ^ ((a as u64) << 24)\n                ^ (b as u64);\n\n        while out.len() < len {\n            x = x\n                .wrapping_mul(6364136223846793005u64)\n                .wrapping_add(1442695040888963407u64);\n\n            let ch: u8 = b\'a\' + (((x >> 33) % 26) as u8);\n            out.push(ch as char);\n        }\n\n        out.truncate(len);\n        out\n    }\n\n    fn issue320_probe_hot_key() -> String {\n        issue320_probe_blob(b\'k\', 0, 0, ISSUE320_PROBE_KEY_LEN)\n    }\n\n    fn issue320_probe_value(version: usize) -> String {\n        issue320_probe_blob(b\'v\', version, 0, ISSUE320_PROBE_VALUE_LEN)\n    }\n\n    fn issue320_probe_filler_key(wave: usize, idx: usize) -> String {\n        issue320_probe_blob(b\'f\', wave, idx, ISSUE320_PROBE_KEY_LEN)\n    }\n\n    fn issue320_probe_filler_value(wave: usize, idx: usize) -> String {\n        issue320_probe_blob(b\'g\', wave, idx, ISSUE320_PROBE_VALUE_LEN)\n    }\n\n    fn issue320_probe_sig_bytes(bytes: &[u8]) -> String {\n        let head_len: usize = std::cmp::min(24, bytes.len());\n        let tail_len: usize = std::cmp::min(24, bytes.len());\n\n        let head = String::from_utf8_lossy(&bytes[..head_len]);\n        let tail = String::from_utf8_lossy(&bytes[bytes.len().saturating_sub(tail_len)..]);\n\n        format!("len={} head=\'{}\' tail=\'{}\'", bytes.len(), head, tail)\n    }\n\n    fn issue320_probe_sig_str(s: &str) -> String {\n        issue320_probe_sig_bytes(s.as_bytes())\n    }\n\n    fn issue320_probe_push_history(history: &mut Vec<String>, line: String) {\n        history.push(line);\n        if history.len() > ISSUE320_PROBE_HISTORY_LIMIT {\n            history.remove(0);\n        }\n    }\n\n    fn issue320_probe_history_text(history: &[String]) -> String {\n        if history.is_empty() {\n            "<empty>".to_string()\n        } else {\n            history.join("\\n    ")\n        }\n    }\n\n    fn issue320_probe_options() -> Options {\n        let env = PosixEnv::shared();\n        let mut options = Options::with_env(env);\n\n        assert!(options.env().is_some());\n\n        options.set_create_if_missing(true);\n        options.set_error_if_exists(false);\n        options.set_paranoid_checks(true);\n\n        // Keep the workload small but still force memtable flushes / compaction work.\n        options.set_write_buffer_size(32 * 1024);\n        options.set_max_file_size(16 * 1024);\n\n        options\n    }\n\n    fn issue320_probe_open_db(options: &Options, dbname: &String) -> *mut dyn DB {\n        let mut opener = DBImpl::new(options, dbname);\n\n        let mut out_db: *mut dyn DB =\n            core::ptr::null_mut::<DBImpl>() as *mut dyn DB;\n\n        let status = <DBImpl as DBOpen>::open(\n            &mut opener,\n            options,\n            dbname,\n            &mut out_db as *mut *mut dyn DB,\n        );\n\n        assert!(\n            status.is_ok(),\n            "DBOpen::open failed for {}: {}",\n            dbname,\n            status.to_string()\n        );\n        assert!(\n            !out_db.is_null(),\n            "DBOpen::open returned null DB pointer for {}",\n            dbname\n        );\n\n        out_db\n    }\n\n    unsafe fn issue320_probe_dbimpl_ptr(db_ptr: *mut dyn DB) -> *mut DBImpl {\n        let data: *mut () = db_ptr as *mut ();\n        data as *mut DBImpl\n    }\n\n    fn issue320_probe_close_db(db_ptr: *mut dyn DB) {\n        assert!(!db_ptr.is_null());\n        unsafe {\n            drop(Box::from_raw(db_ptr));\n        }\n    }\n\n    fn issue320_probe_assert_bg_error_ok(db: &DBImpl, stage: &str, history: &[String]) {\n        assert!(\n            db.bg_error.is_ok(),\n            "{}: bg_error={}\\n  history:\\n    {}",\n            stage,\n            db.bg_error.to_string(),\n            issue320_probe_history_text(history),\n        );\n    }\n\n    fn issue320_probe_write_step(\n        db: &mut DBImpl,\n        hot_key: &str,\n        hot_value: &str,\n        wave: usize,\n        history: &mut Vec<String>,\n    ) {\n        let mut batch = WriteBatch::default();\n\n        let hot_key_slice = Slice::from_str(hot_key);\n        let hot_value_slice = Slice::from_str(hot_value);\n        batch.put(&hot_key_slice, &hot_value_slice);\n\n        for idx in 0..ISSUE320_PROBE_FILLER_COUNT {\n            let filler_key = issue320_probe_filler_key(wave, idx);\n            let filler_value = issue320_probe_filler_value(wave, idx);\n\n            let filler_key_slice = Slice::from_str(&filler_key);\n            let filler_value_slice = Slice::from_str(&filler_value);\n\n            batch.put(&filler_key_slice, &filler_value_slice);\n        }\n\n        let status = <DBImpl as DBWrite>::write(\n            db,\n            &WriteOptions::default(),\n            &mut batch as *mut WriteBatch,\n        );\n\n        assert!(\n            status.is_ok(),\n            "wave {} write failed: {}\\n  history:\\n    {}",\n            wave,\n            status.to_string(),\n            issue320_probe_history_text(history),\n        );\n\n        issue320_probe_push_history(\n            history,\n            format!(\n                "write wave={} hot_value={}",\n                wave,\n                issue320_probe_sig_str(hot_value)\n            ),\n        );\n    }\n\n    fn issue320_probe_flush_only(\n        db: &mut DBImpl,\n        stage: &str,\n        history: &mut Vec<String>,\n    ) {\n        let status = db.test_compact_mem_table();\n\n        assert!(\n            status.is_ok(),\n            "{}: test_compact_mem_table failed: {}\\n  history:\\n    {}",\n            stage,\n            status.to_string(),\n            issue320_probe_history_text(history),\n        );\n\n        issue320_probe_assert_bg_error_ok(db, stage, history);\n\n        issue320_probe_push_history(\n            history,\n            format!("flush {}", stage),\n        );\n    }\n\n    fn issue320_probe_flush_and_manual_compact(\n        db: &mut DBImpl,\n        stage: &str,\n        history: &mut Vec<String>,\n    ) {\n        issue320_probe_flush_only(db, stage, history);\n\n        for level in 0..=ISSUE320_PROBE_MAX_MANUAL_LEVEL {\n            db.test_compact_range(level, core::ptr::null(), core::ptr::null());\n\n            issue320_probe_assert_bg_error_ok(\n                db,\n                &format!("{} level {}", stage, level),\n                history,\n            );\n\n            issue320_probe_push_history(\n                history,\n                format!("manual_compact stage={} level={}", stage, level),\n            );\n        }\n    }\n\n    fn issue320_probe_latest_get(\n        db: &mut DBImpl,\n        key: &str,\n        history: &[String],\n    ) -> String {\n        let mut value = String::new();\n\n        let status = <DBImpl as DBGet>::get(\n            db,\n            &ReadOptions::default(),\n            &Slice::from_str(key),\n            &mut value,\n        );\n\n        assert!(\n            status.is_ok(),\n            "get failed for key(len={}): {}\\n  history:\\n    {}",\n            key.len(),\n            status.to_string(),\n            issue320_probe_history_text(history),\n        );\n\n        value\n    }\n\n    fn issue320_probe_collect_versions_for_user_key(\n        db: &mut DBImpl,\n        user_key: &str,\n        history: &[String],\n    ) -> Vec<String> {\n        let mut out: Vec<String> = Vec::new();\n\n        let iter = db.test_new_internal_iterator();\n        assert!(\n            !iter.is_null(),\n            "test_new_internal_iterator returned null\\n  history:\\n    {}",\n            issue320_probe_history_text(history),\n        );\n\n        unsafe {\n            (*iter).seek_to_first();\n\n            while (*iter).valid() {\n                let internal_key: Slice = (*iter).key();\n                let value: Slice = (*iter).value();\n\n                let mut parsed: ParsedInternalKey = Default::default();\n\n                if parse_internal_key(&internal_key, &mut parsed) {\n                    if parsed.user_key().as_bytes() == user_key.as_bytes() {\n                        out.push(format!(\n                            "seq={} ty={:?} value={}",\n                            *parsed.sequence(),\n                            *parsed.ty(),\n                            issue320_probe_sig_bytes(value.as_bytes()),\n                        ));\n                    }\n                }\n\n                (*iter).next();\n            }\n\n            let status = (*iter).status();\n            assert!(\n                status.is_ok(),\n                "internal iterator status failed: {}\\n  history:\\n    {}",\n                status.to_string(),\n                issue320_probe_history_text(history),\n            );\n\n            drop(Box::from_raw(iter));\n        }\n\n        out\n    }\n\n    fn issue320_probe_fail_latest_mismatch(\n        stage: &str,\n        expected: &str,\n        actual: &str,\n        versions: &[String],\n        history: &[String],\n    ) -> ! {\n        let versions_text = if versions.is_empty() {\n            "<none>".to_string()\n        } else {\n            versions.join("\\n    ")\n        };\n\n        panic!(\n            "{}:\\n  expected_latest={}\\n  actual_latest={}\\n  internal_versions:\\n    {}\\n  history:\\n    {}",\n            stage,\n            issue320_probe_sig_str(expected),\n            issue320_probe_sig_str(actual),\n            versions_text,\n            issue320_probe_history_text(history),\n        );\n    }\n\n    fn issue320_probe_assert_latest(\n        db: &mut DBImpl,\n        key: &str,\n        expected: &str,\n        stage: &str,\n        history: &mut Vec<String>,\n    ) {\n        let actual = issue320_probe_latest_get(db, key, history);\n\n        issue320_probe_push_history(\n            history,\n            format!(\n                "latest_get stage={} actual={}",\n                stage,\n                issue320_probe_sig_str(&actual),\n            ),\n        );\n\n        if actual != expected {\n            let versions = issue320_probe_collect_versions_for_user_key(\n                db,\n                key,\n                history,\n            );\n\n            issue320_probe_fail_latest_mismatch(\n                stage,\n                expected,\n                &actual,\n                &versions,\n                history,\n            );\n        }\n    }\n\n    fn issue320_probe_db_path(test_name: &str) -> String {\n        let tmp = TempDir::new().unwrap();\n        tmp.path().join(test_name).to_string_lossy().to_string()\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_probe_single_key_without_snapshot_keeps_only_newest_version_after_manual_compaction() {\n        issue320_require_protection();\n        let dbname = issue320_probe_db_path(\n            "issue320_probe_single_key_without_snapshot",\n        );\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = issue320_probe_options();\n        let db_ptr = issue320_probe_open_db(&options, &dbname);\n\n        {\n            let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(db_ptr) };\n            assert!(!dbimpl_ptr.is_null());\n\n            let db = unsafe { &mut *dbimpl_ptr };\n            db.clear_background_error_for_test();\n\n            let key = issue320_probe_hot_key();\n            let v0 = issue320_probe_value(0);\n            let v1 = issue320_probe_value(1);\n\n            let mut history: Vec<String> = Vec::new();\n\n            issue320_probe_write_step(db, &key, &v0, 0, &mut history);\n            issue320_probe_flush_and_manual_compact(db, "after-v0", &mut history);\n            issue320_probe_assert_latest(db, &key, &v0, "after-v0", &mut history);\n\n            issue320_probe_write_step(db, &key, &v1, 1, &mut history);\n            issue320_probe_flush_and_manual_compact(db, "after-v1", &mut history);\n            issue320_probe_assert_latest(db, &key, &v1, "after-v1", &mut history);\n\n            let versions = issue320_probe_collect_versions_for_user_key(\n                db,\n                &key,\n                &history,\n            );\n\n            assert!(\n                versions.len() == 1,\n                "expected exactly one retained version without snapshots\\n  internal_versions:\\n    {}\\n  history:\\n    {}",\n                if versions.is_empty() {\n                    "<none>".to_string()\n                } else {\n                    versions.join("\\n    ")\n                },\n                issue320_probe_history_text(&history),\n            );\n        }\n\n        issue320_probe_close_db(db_ptr);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_probe_single_key_with_live_snapshot_survives_flush_without_manual_compaction() {\n        issue320_require_protection();\n        let dbname = issue320_probe_db_path(\n            "issue320_probe_single_key_live_snapshot_flush_only",\n        );\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = issue320_probe_options();\n        let db_ptr = issue320_probe_open_db(&options, &dbname);\n\n        {\n            let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(db_ptr) };\n            assert!(!dbimpl_ptr.is_null());\n\n            let db = unsafe { &mut *dbimpl_ptr };\n            db.clear_background_error_for_test();\n\n            let key = issue320_probe_hot_key();\n            let v0 = issue320_probe_value(0);\n            let v1 = issue320_probe_value(1);\n\n            let mut history: Vec<String> = Vec::new();\n\n            issue320_probe_write_step(db, &key, &v0, 0, &mut history);\n            issue320_probe_flush_only(db, "after-v0", &mut history);\n            issue320_probe_assert_latest(db, &key, &v0, "after-v0", &mut history);\n\n            let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);\n            issue320_probe_push_history(\n                &mut history,\n                "acquired live snapshot".to_string(),\n            );\n\n            issue320_probe_write_step(db, &key, &v1, 1, &mut history);\n            issue320_probe_flush_only(db, "after-v1", &mut history);\n            issue320_probe_assert_latest(\n                db,\n                &key,\n                &v1,\n                "after-v1-with-live-snapshot-flush-only",\n                &mut history,\n            );\n\n            <DBImpl as DBReleaseSnapshot>::release_snapshot(db, snapshot);\n            issue320_probe_push_history(\n                &mut history,\n                "released live snapshot".to_string(),\n            );\n        }\n\n        issue320_probe_close_db(db_ptr);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_probe_single_key_with_live_snapshot_manual_compaction_reports_retained_versions_on_failure() {\n        issue320_require_protection();\n        let dbname = issue320_probe_db_path(\n            "issue320_probe_single_key_live_snapshot_manual_compaction",\n        );\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = issue320_probe_options();\n        let db_ptr = issue320_probe_open_db(&options, &dbname);\n\n        {\n            let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(db_ptr) };\n            assert!(!dbimpl_ptr.is_null());\n\n            let db = unsafe { &mut *dbimpl_ptr };\n            db.clear_background_error_for_test();\n\n            let key = issue320_probe_hot_key();\n            let v0 = issue320_probe_value(0);\n            let v1 = issue320_probe_value(1);\n\n            let mut history: Vec<String> = Vec::new();\n\n            issue320_probe_write_step(db, &key, &v0, 0, &mut history);\n            issue320_probe_flush_and_manual_compact(db, "after-v0", &mut history);\n            issue320_probe_assert_latest(db, &key, &v0, "after-v0", &mut history);\n\n            let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);\n            issue320_probe_push_history(\n                &mut history,\n                "acquired live snapshot".to_string(),\n            );\n\n            issue320_probe_write_step(db, &key, &v1, 1, &mut history);\n            issue320_probe_flush_and_manual_compact(db, "after-v1", &mut history);\n            issue320_probe_assert_latest(\n                db,\n                &key,\n                &v1,\n                "after-v1-with-live-snapshot-manual-compaction",\n                &mut history,\n            );\n\n            let versions = issue320_probe_collect_versions_for_user_key(\n                db,\n                &key,\n                &history,\n            );\n\n            assert!(\n                versions.len() >= 2,\n                "expected at least two retained versions with live snapshot\\n  internal_versions:\\n    {}\\n  history:\\n    {}",\n                if versions.is_empty() {\n                    "<none>".to_string()\n                } else {\n                    versions.join("\\n    ")\n                },\n                issue320_probe_history_text(&history),\n            );\n\n            <DBImpl as DBReleaseSnapshot>::release_snapshot(db, snapshot);\n            issue320_probe_push_history(\n                &mut history,\n                "released live snapshot".to_string(),\n            );\n        }\n\n        issue320_probe_close_db(db_ptr);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_probe_single_key_with_live_snapshot_reopen_reports_before_and_after_state_compactly() {\n        issue320_require_protection();\n        let dbname = issue320_probe_db_path(\n            "issue320_probe_single_key_live_snapshot_reopen",\n        );\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = issue320_probe_options();\n\n        let key = issue320_probe_hot_key();\n        let v0 = issue320_probe_value(0);\n        let v1 = issue320_probe_value(1);\n\n        let mut history: Vec<String> = Vec::new();\n\n        let (before_close_latest, before_close_versions): (String, Vec<String>) = {\n            let db_ptr = issue320_probe_open_db(&options, &dbname);\n\n            let result = {\n                let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(db_ptr) };\n                assert!(!dbimpl_ptr.is_null());\n\n                let db = unsafe { &mut *dbimpl_ptr };\n                db.clear_background_error_for_test();\n\n                issue320_probe_write_step(db, &key, &v0, 0, &mut history);\n                issue320_probe_flush_and_manual_compact(db, "after-v0", &mut history);\n                issue320_probe_assert_latest(db, &key, &v0, "after-v0", &mut history);\n\n                let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);\n                issue320_probe_push_history(\n                    &mut history,\n                    "acquired live snapshot".to_string(),\n                );\n\n                issue320_probe_write_step(db, &key, &v1, 1, &mut history);\n                issue320_probe_flush_and_manual_compact(db, "after-v1", &mut history);\n\n                let latest = issue320_probe_latest_get(db, &key, &history);\n                issue320_probe_push_history(\n                    &mut history,\n                    format!(\n                        "before-close latest={}",\n                        issue320_probe_sig_str(&latest),\n                    ),\n                );\n\n                let versions = issue320_probe_collect_versions_for_user_key(\n                    db,\n                    &key,\n                    &history,\n                );\n\n                <DBImpl as DBReleaseSnapshot>::release_snapshot(db, snapshot);\n                issue320_probe_push_history(\n                    &mut history,\n                    "released live snapshot before close".to_string(),\n                );\n\n                (latest, versions)\n            };\n\n            issue320_probe_close_db(db_ptr);\n            result\n        };\n\n        let reopened_db_ptr = issue320_probe_open_db(&options, &dbname);\n\n        {\n            let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(reopened_db_ptr) };\n            assert!(!dbimpl_ptr.is_null());\n\n            let db = unsafe { &mut *dbimpl_ptr };\n            db.clear_background_error_for_test();\n\n            let after_reopen_latest = issue320_probe_latest_get(db, &key, &history);\n            let after_reopen_versions = issue320_probe_collect_versions_for_user_key(\n                db,\n                &key,\n                &history,\n            );\n\n            if before_close_latest != v1 || after_reopen_latest != v1 {\n                panic!(\n                    "live-snapshot reopen probe failed:\\n  expected_latest={}\\n  before_close_latest={}\\n  after_reopen_latest={}\\n  before_close_versions:\\n    {}\\n  after_reopen_versions:\\n    {}\\n  history:\\n    {}",\n                    issue320_probe_sig_str(&v1),\n                    issue320_probe_sig_str(&before_close_latest),\n                    issue320_probe_sig_str(&after_reopen_latest),\n                    if before_close_versions.is_empty() {\n                        "<none>".to_string()\n                    } else {\n                        before_close_versions.join("\\n    ")\n                    },\n                    if after_reopen_versions.is_empty() {\n                        "<none>".to_string()\n                    } else {\n                        after_reopen_versions.join("\\n    ")\n                    },\n                    issue320_probe_history_text(&history),\n                );\n            }\n        }\n\n        issue320_probe_close_db(reopened_db_ptr);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n}\n\n#[cfg(test)]\nmod issue320_two_key_reducer_tests {\n    use super::*;\n\n    #[derive(Clone, Copy)]\n    struct ReducerConfig {\n        name: &\'static str,\n        waves: usize,\n        snapshot_waves: &\'static [usize],\n        reopen: bool,\n        wal_tail_before_reopen: bool,\n    }\n\n    struct HeldSnapshot {\n        label: String,\n        expected_by_key: BTreeMap<String, String>,\n        live: Option<Box<dyn bitcoinleveldb_snapshot::Snapshot>>,\n    }\n\n    #[derive(Clone)]\n    struct InternalVersionRow {\n        seq: u64,\n        ty: &\'static str,\n        value: String,\n    }\n\n    fn reducer_options() -> Options {\n        let env = PosixEnv::shared();\n        let mut options = Options::with_env(env);\n\n        if options.env().is_none() {\n            panic!("issue320 reducer: Options::with_env(env) returned env=None");\n        }\n\n        options.set_create_if_missing(true);\n        options.set_error_if_exists(false);\n        options\n    }\n\n    fn open_db(dbname: &String, options: &Options) -> (*mut dyn DB, *mut DBImpl) {\n        let mut dispatcher = DBImpl::new(options, dbname);\n\n        let mut out_db: *mut dyn DB =\n            core::ptr::null_mut::<DBImpl>() as *mut dyn DB;\n\n        let st = <DBImpl as DBOpen>::open(\n            &mut dispatcher,\n            options,\n            dbname,\n            &mut out_db as *mut *mut dyn DB,\n        );\n\n        assert!(\n            st.is_ok(),\n            "issue320 reducer: open failed for {}: {}",\n            dbname,\n            st.to_string()\n        );\n        assert!(\n            !out_db.is_null(),\n            "issue320 reducer: open returned null db pointer for {}",\n            dbname\n        );\n\n        let dbimpl_ptr = unsafe { db_ptr_to_dbimpl_mut(out_db) };\n        assert!(\n            !dbimpl_ptr.is_null(),\n            "issue320 reducer: downcast to DBImpl produced null for {}",\n            dbname\n        );\n\n        (out_db, dbimpl_ptr)\n    }\n\n    unsafe fn db_ptr_to_dbimpl_mut(db_ptr: *mut dyn DB) -> *mut DBImpl {\n        let data: *mut () = db_ptr as *mut ();\n        data as *mut DBImpl\n    }\n\n    fn write_wave(\n        db: &mut DBImpl,\n        wave_label: &str,\n        keys: &[&str],\n        latest_expected: &mut BTreeMap<String, String>,\n        history: &mut Vec<String>,\n    ) {\n        let mut batch = WriteBatch::default();\n\n        for key in keys {\n            let value = format!("{}::{}", key, wave_label);\n            let k = Slice::from_str(key);\n            let v = Slice::from_str(&value);\n\n            batch.put(&k, &v);\n            latest_expected.insert((*key).to_string(), value.clone());\n            history.push(format!("write {}={}", key, value));\n        }\n\n        let st = <DBImpl as DBWrite>::write(db, &WriteOptions::default(), &mut batch);\n        assert!(\n            st.is_ok(),\n            "issue320 reducer: write failed at {}: {}",\n            wave_label,\n            st.to_string()\n        );\n    }\n\n    fn force_full_compaction(\n        db: &mut DBImpl,\n        history: &mut Vec<String>,\n    ) {\n        history.push("compact_range(all)".to_string());\n        <DBImpl as DBCompactRange>::compact_range(\n            db,\n            core::ptr::null(),\n            core::ptr::null(),\n        );\n    }\n\n    fn acquire_snapshot(\n        db: &mut DBImpl,\n        label: String,\n        latest_expected: &BTreeMap<String, String>,\n        history: &mut Vec<String>,\n    ) -> HeldSnapshot {\n        history.push(format!("acquire_snapshot {}", label));\n\n        let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);\n\n        HeldSnapshot {\n            label,\n            expected_by_key: latest_expected.clone(),\n            live: Some(snapshot),\n        }\n    }\n\n    fn release_snapshots(\n        db: &mut DBImpl,\n        held: &mut [HeldSnapshot],\n        history: &mut Vec<String>,\n    ) {\n        for snapshot in held.iter_mut() {\n            if let Some(live) = snapshot.live.take() {\n                history.push(format!("release_snapshot {}", snapshot.label));\n                <DBImpl as DBReleaseSnapshot>::release_snapshot(db, live);\n            }\n        }\n    }\n\n    fn get_latest_value(\n        db: &mut DBImpl,\n        key: &str,\n    ) -> String {\n        let mut out = String::new();\n        let key_slice = Slice::from_str(key);\n        let st = <DBImpl as DBGet>::get(\n            db,\n            &ReadOptions::default(),\n            &key_slice,\n            &mut out,\n        );\n\n        if st.is_ok() {\n            out\n        } else {\n            format!("<status:{}>", st.to_string())\n        }\n    }\n\n    fn collect_internal_versions_for_keys(\n        db: &mut DBImpl,\n        keys: &[&str],\n    ) -> BTreeMap<String, Vec<InternalVersionRow>> {\n        let wanted: BTreeSet<String> = keys.iter().map(|k| (*k).to_string()).collect();\n\n        let mut rows: BTreeMap<String, Vec<InternalVersionRow>> = BTreeMap::new();\n        for key in wanted.iter() {\n            rows.insert(key.clone(), Vec::new());\n        }\n\n        let mut latest_snapshot: u64 = 0;\n        let mut seed: u32 = 0;\n\n        let it = db.new_internal_iterator(\n            &ReadOptions::default(),\n            &mut latest_snapshot as *mut u64,\n            &mut seed as *mut u32,\n        );\n\n        assert!(\n            !it.is_null(),\n            "issue320 reducer: new_internal_iterator returned null"\n        );\n\n        unsafe {\n            (*it).seek_to_first();\n\n            while (*it).valid() {\n                let internal_key = (*it).key();\n                let value = (*it).value();\n\n                let mut parsed: ParsedInternalKey = Default::default();\n\n                if parse_internal_key(&internal_key, &mut parsed) {\n                    let user_key =\n                        String::from_utf8_lossy(parsed.user_key().as_bytes()).into_owned();\n\n                    if wanted.contains(&user_key) {\n                        let ty = match *parsed.ty() {\n                            ValueType::TypeValue => "TypeValue",\n                            ValueType::TypeDeletion => "TypeDeletion",\n                        };\n\n                        let value_string =\n                            String::from_utf8_lossy(value.as_bytes()).into_owned();\n\n                        rows.get_mut(&user_key)\n                            .unwrap()\n                            .push(InternalVersionRow {\n                                seq: *parsed.sequence(),\n                                ty,\n                                value: value_string,\n                            });\n                    }\n                }\n\n                (*it).next();\n            }\n\n            let st = (*it).status();\n            assert!(\n                st.is_ok(),\n                "issue320 reducer: internal iterator status was non-OK: {}",\n                st.to_string()\n            );\n\n            drop(Box::from_raw(it));\n        }\n\n        for versions in rows.values_mut() {\n            versions.sort_by(|a, b| b.seq.cmp(&a.seq));\n        }\n\n        rows\n    }\n\n    fn retained_value_set(\n        rows: &[InternalVersionRow],\n    ) -> BTreeSet<String> {\n        rows.iter()\n            .filter(|row| row.ty == "TypeValue")\n            .map(|row| row.value.clone())\n            .collect()\n    }\n\n    fn render_failure(\n        phase: &str,\n        cfg: ReducerConfig,\n        keys: &[&str],\n        latest_expected: &BTreeMap<String, String>,\n        snapshots: &[HeldSnapshot],\n        internal: &BTreeMap<String, Vec<InternalVersionRow>>,\n        latest_actual_get: &BTreeMap<String, String>,\n        history: &[String],\n        mismatches: &[String],\n    ) -> String {\n        let mut out = String::new();\n\n        out.push_str(&format!(\n            "issue320 reducer mismatch\\nphase={}\\nconfig: name={} waves={} snapshot_waves={:?} reopen={} wal_tail_before_reopen={}\\n",\n            phase,\n            cfg.name,\n            cfg.waves,\n            cfg.snapshot_waves,\n            cfg.reopen,\n            cfg.wal_tail_before_reopen,\n        ));\n\n        out.push_str("mismatches:\\n");\n        for m in mismatches {\n            out.push_str("  ");\n            out.push_str(m);\n            out.push(\'\\n\');\n        }\n\n        out.push_str("latest:\\n");\n        for key in keys {\n            let expected = latest_expected.get(*key).cloned().unwrap_or_else(|| "<missing>".to_string());\n            let actual = latest_actual_get.get(*key).cloned().unwrap_or_else(|| "<missing>".to_string());\n            out.push_str(&format!("  {} expected={} actual={}\\n", key, expected, actual));\n        }\n\n        out.push_str("snapshot_requirements:\\n");\n        for snapshot in snapshots {\n            out.push_str(&format!("  {}:\\n", snapshot.label));\n            for key in keys {\n                let expected = snapshot.expected_by_key\n                    .get(*key)\n                    .cloned()\n                    .unwrap_or_else(|| "<missing>".to_string());\n                out.push_str(&format!("    {} -> {}\\n", key, expected));\n            }\n        }\n\n        out.push_str("internal_versions:\\n");\n        for key in keys {\n            out.push_str(&format!("  {}:\\n", key));\n            match internal.get(*key) {\n                Some(rows) if !rows.is_empty() => {\n                    for row in rows {\n                        out.push_str(&format!(\n                            "    seq={} ty={} value={}\\n",\n                            row.seq,\n                            row.ty,\n                            row.value\n                        ));\n                    }\n                }\n                _ => {\n                    out.push_str("    <none>\\n");\n                }\n            }\n        }\n\n        out.push_str("history:\\n");\n        for item in history {\n            out.push_str("  ");\n            out.push_str(item);\n            out.push(\'\\n\');\n        }\n\n        out\n    }\n\n    fn assert_model_matches(\n        db: &mut DBImpl,\n        phase: &str,\n        cfg: ReducerConfig,\n        keys: &[&str],\n        latest_expected: &BTreeMap<String, String>,\n        snapshots: &[HeldSnapshot],\n        history: &[String],\n    ) {\n        let internal = collect_internal_versions_for_keys(db, keys);\n\n        let mut latest_actual_get: BTreeMap<String, String> = BTreeMap::new();\n        let mut mismatches: Vec<String> = Vec::new();\n\n        for key in keys {\n            let actual = get_latest_value(db, key);\n            latest_actual_get.insert((*key).to_string(), actual.clone());\n\n            let expected_latest = latest_expected\n                .get(*key)\n                .cloned()\n                .unwrap_or_else(|| "<missing>".to_string());\n\n            if actual != expected_latest {\n                mismatches.push(format!(\n                    "latest mismatch: key={} expected={} actual={}",\n                    key,\n                    expected_latest,\n                    actual,\n                ));\n            }\n\n            let retained = retained_value_set(\n                internal.get(*key).map(|v| v.as_slice()).unwrap_or(&[])\n            );\n\n            if !retained.contains(&expected_latest) {\n                mismatches.push(format!(\n                    "latest missing from internal versions: key={} expected_latest={} retained={:?}",\n                    key,\n                    expected_latest,\n                    retained,\n                ));\n            }\n\n            for snapshot in snapshots {\n                let expected_snapshot_value = snapshot.expected_by_key\n                    .get(*key)\n                    .cloned()\n                    .unwrap_or_else(|| "<missing>".to_string());\n\n                if !retained.contains(&expected_snapshot_value) {\n                    mismatches.push(format!(\n                        "snapshot-required version missing: snapshot={} key={} expected_value={} retained={:?}",\n                        snapshot.label,\n                        key,\n                        expected_snapshot_value,\n                        retained,\n                    ));\n                }\n            }\n        }\n\n        if !mismatches.is_empty() {\n            panic!(\n                "{}",\n                render_failure(\n                    phase,\n                    cfg,\n                    keys,\n                    latest_expected,\n                    snapshots,\n                    &internal,\n                    &latest_actual_get,\n                    history,\n                    &mismatches,\n                )\n            );\n        }\n    }\n\n    fn run_reducer(cfg: ReducerConfig) {\n        let tmp = TempDir::new().unwrap();\n        let dbname = tmp\n            .path()\n            .join(cfg.name)\n            .to_string_lossy()\n            .into_owned();\n\n        std::fs::create_dir_all(&dbname).unwrap();\n\n        let options = reducer_options();\n\n        let keys = ["k0", "k1"];\n        let mut latest_expected: BTreeMap<String, String> = BTreeMap::new();\n        let mut history: Vec<String> = Vec::new();\n        let mut held_snapshots: Vec<HeldSnapshot> = Vec::new();\n\n        let (db_ptr, dbimpl_ptr) = open_db(&dbname, &options);\n        let db = unsafe { &mut *dbimpl_ptr };\n\n        for wave in 0..cfg.waves {\n            let wave_label = format!("wave{}", wave);\n            write_wave(\n                db,\n                &wave_label,\n                &keys,\n                &mut latest_expected,\n                &mut history,\n            );\n\n            if cfg.snapshot_waves.contains(&wave) {\n                let snapshot_label = format!("snapshot_after_{}", wave_label);\n                held_snapshots.push(acquire_snapshot(\n                    db,\n                    snapshot_label,\n                    &latest_expected,\n                    &mut history,\n                ));\n            }\n\n            force_full_compaction(db, &mut history);\n        }\n\n        if cfg.wal_tail_before_reopen {\n            write_wave(\n                db,\n                "tail",\n                &keys,\n                &mut latest_expected,\n                &mut history,\n            );\n            history.push("leave_tail_in_wal_without_compaction".to_string());\n        }\n\n        if cfg.reopen {\n            release_snapshots(db, &mut held_snapshots, &mut history);\n\n            unsafe {\n                drop(Box::from_raw(db_ptr));\n            }\n\n            history.push("close_for_reopen".to_string());\n\n            let (reopened_ptr, reopened_dbimpl_ptr) = open_db(&dbname, &options);\n            let reopened = unsafe { &mut *reopened_dbimpl_ptr };\n\n            history.push("reopen_complete".to_string());\n\n            assert_model_matches(\n                reopened,\n                "after_reopen",\n                cfg,\n                &keys,\n                &latest_expected,\n                &held_snapshots,\n                &history,\n            );\n\n            unsafe {\n                drop(Box::from_raw(reopened_ptr));\n            }\n        } else {\n            assert_model_matches(\n                db,\n                "after_forced_compactions",\n                cfg,\n                &keys,\n                &latest_expected,\n                &held_snapshots,\n                &history,\n            );\n\n            release_snapshots(db, &mut held_snapshots, &mut history);\n\n            unsafe {\n                drop(Box::from_raw(db_ptr));\n            }\n        }\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_reducer_two_keys_one_snapshot_matches_model_after_forced_compactions() {\n        issue320_require_protection();\n        run_reducer(ReducerConfig {\n            name: "issue320_reducer_two_keys_one_snapshot_after_compactions",\n            waves: 3,\n            snapshot_waves: &[0],\n            reopen: false,\n            wal_tail_before_reopen: false,\n        });\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_reducer_two_keys_one_snapshot_matches_model_after_reopen_with_wal_tail() {\n        issue320_require_protection();\n        run_reducer(ReducerConfig {\n            name: "issue320_reducer_two_keys_one_snapshot_after_reopen_with_wal_tail",\n            waves: 3,\n            snapshot_waves: &[0],\n            reopen: true,\n            wal_tail_before_reopen: true,\n        });\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_reducer_two_keys_two_snapshots_matches_model_after_forced_compactions() {\n        issue320_require_protection();\n        run_reducer(ReducerConfig {\n            name: "issue320_reducer_two_keys_two_snapshots_after_compactions",\n            waves: 3,\n            snapshot_waves: &[0, 1],\n            reopen: false,\n            wal_tail_before_reopen: false,\n        });\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_reducer_two_keys_two_snapshots_matches_model_after_reopen_with_wal_tail() {\n        issue320_require_protection();\n        run_reducer(ReducerConfig {\n            name: "issue320_reducer_two_keys_two_snapshots_after_reopen_with_wal_tail",\n            waves: 3,\n            snapshot_waves: &[0, 1],\n            reopen: true,\n            wal_tail_before_reopen: true,\n        });\n    }\n}\n\n#[cfg(test)]\nmod issue320_fanout_reducer_tests {\n    use super::*;\n\n    #[derive(Clone, Copy)]\n    struct Issue320FanoutCase {\n        label: &\'static str,\n        key_count: usize,\n        wave_count: usize,\n        snapshot_waves: &\'static [usize],\n    }\n\n    #[derive(Clone, Debug)]\n    struct Issue320InternalVersionRow {\n        seq: u64,\n        kind: &\'static str,\n        value: String,\n    }\n\n    unsafe fn db_ptr_to_dbimpl_mut(db_ptr: *mut dyn DB) -> *mut DBImpl {\n        let data: *mut () = db_ptr as *mut ();\n        data as *mut DBImpl\n    }\n\n    fn build_options_for_issue320_fanout_reducer() -> Options {\n        let env = PosixEnv::shared();\n        let mut options = Options::with_env(env);\n\n        assert!(\n            options.env().is_some(),\n            "issue320 fanout reducer requires Options::env to be present"\n        );\n\n        options.set_create_if_missing(true);\n        options.set_error_if_exists(false);\n        options\n    }\n\n    fn open_db_for_issue320_fanout_reducer(dbname: &String) -> (*mut dyn DB, *mut DBImpl) {\n        let options = build_options_for_issue320_fanout_reducer();\n        let mut dispatcher = DBImpl::new(&options, dbname);\n\n        let mut out_db: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;\n\n        let st = <DBImpl as DBOpen>::open(\n            &mut dispatcher,\n            &options,\n            dbname,\n            &mut out_db as *mut *mut dyn DB,\n        );\n\n        assert!(\n            st.is_ok(),\n            "DBOpen::open failed for {}: {}",\n            dbname,\n            st.to_string()\n        );\n        assert!(\n            !out_db.is_null(),\n            "DBOpen::open returned OK but db pointer was null for {}",\n            dbname\n        );\n\n        let dbimpl_ptr = unsafe { db_ptr_to_dbimpl_mut(out_db) };\n        assert!(\n            !dbimpl_ptr.is_null(),\n            "downcast from *mut dyn DB to *mut DBImpl returned null for {}",\n            dbname\n        );\n\n        (out_db, dbimpl_ptr)\n    }\n\n    fn drop_open_db(db_ptr: *mut dyn DB) {\n        assert!(!db_ptr.is_null(), "attempted to drop a null DB pointer");\n        unsafe {\n            drop(Box::from_raw(db_ptr));\n        }\n    }\n\n    fn make_issue320_keys(key_count: usize) -> Vec<String> {\n        (0..key_count)\n            .map(|i| format!("k{:04}", i))\n            .collect()\n    }\n\n    fn issue320_value_for_wave(wave: usize, key: &str) -> String {\n        format!("wave{:02}-{}", wave, key)\n    }\n\n    fn issue320_kind_for_value_type(ty: ValueType) -> &\'static str {\n        match ty {\n            ValueType::TypeValue => "value",\n            ValueType::TypeDeletion => "deletion",\n            _ => "other",\n        }\n    }\n\n    fn issue320_apply_wave(\n        dbimpl: &mut DBImpl,\n        keys: &[String],\n        model: &mut BTreeMap<String, String>,\n        wave: usize,\n        context: &str,\n    ) {\n        let mut batch = WriteBatch::default();\n\n        for key in keys.iter() {\n            let value = issue320_value_for_wave(wave, key);\n            batch.put(&Slice::from_str(key), &Slice::from_str(&value));\n            model.insert(key.clone(), value);\n        }\n\n        let st = <DBImpl as DBWrite>::write(dbimpl, &WriteOptions::default(), &mut batch);\n\n        assert!(\n            st.is_ok(),\n            "{}: write failed at wave {}: {}",\n            context,\n            wave,\n            st.to_string()\n        );\n    }\n\n    fn issue320_flush_memtable(dbimpl: &mut DBImpl, context: &str) {\n        let st = dbimpl.test_compact_mem_table();\n        assert!(\n            st.is_ok(),\n            "{}: test_compact_mem_table failed: {}",\n            context,\n            st.to_string()\n        );\n    }\n\n    fn issue320_force_compact_all_levels(dbimpl: &mut DBImpl, context: &str) {\n        issue320_flush_memtable(dbimpl, context);\n\n        for level in 0..((bitcoinleveldb_cfg::NUM_LEVELS as i32) - 1) {\n            dbimpl.test_compact_range(level, core::ptr::null(), core::ptr::null());\n        }\n    }\n\n    fn issue320_take_snapshot(dbimpl: &mut DBImpl) -> Box<dyn Snapshot> {\n        <DBImpl as DBGetSnapshot>::get_snapshot(dbimpl)\n    }\n\n    fn issue320_release_snapshots(dbimpl: &mut DBImpl, snapshots: &mut Vec<Box<dyn Snapshot>>) {\n        while let Some(snapshot) = snapshots.pop() {\n            <DBImpl as DBReleaseSnapshot>::release_snapshot(dbimpl, snapshot);\n        }\n    }\n\n    fn issue320_read_current_values(\n        dbimpl: &mut DBImpl,\n        keys: &[String],\n    ) -> BTreeMap<String, String> {\n        let mut out = BTreeMap::new();\n\n        for key in keys.iter() {\n            let mut value = String::new();\n            let st = <DBImpl as DBGet>::get(\n                dbimpl,\n                &ReadOptions::default(),\n                &Slice::from_str(key),\n                &mut value,\n            );\n\n            if st.is_ok() {\n                out.insert(key.clone(), value);\n            } else {\n                out.insert(key.clone(), format!("<{}>", st.to_string()));\n            }\n        }\n\n        out\n    }\n\n    fn issue320_choose_watch_keys(keys: &[String], mismatched_keys: &[String]) -> Vec<String> {\n        let mut set = BTreeSet::new();\n\n        if !keys.is_empty() {\n            set.insert(keys[0].clone());\n            set.insert(keys[keys.len() / 2].clone());\n            set.insert(keys[keys.len() - 1].clone());\n        }\n\n        for key in mismatched_keys.iter().take(4) {\n            set.insert(key.clone());\n        }\n\n        set.into_iter().collect()\n    }\n\n    fn issue320_scan_internal_versions_for_keys(\n        dbimpl: &mut DBImpl,\n        watch_keys: &[String],\n    ) -> BTreeMap<String, Vec<Issue320InternalVersionRow>> {\n        let mut out: BTreeMap<String, Vec<Issue320InternalVersionRow>> = BTreeMap::new();\n        let watch_set: BTreeSet<String> = watch_keys.iter().cloned().collect();\n\n        for key in watch_keys.iter() {\n            out.entry(key.clone()).or_default();\n        }\n\n        let iter = dbimpl.test_new_internal_iterator();\n        assert!(\n            !iter.is_null(),\n            "test_new_internal_iterator returned null while collecting internal versions"\n        );\n\n        let mut parsed = ParsedInternalKey::default();\n\n        unsafe {\n            (*iter).seek_to_first();\n        }\n\n        while unsafe { (*iter).valid() } {\n            let internal_key = unsafe { (*iter).key() };\n\n            if parse_internal_key(&internal_key, &mut parsed) {\n                let user_key = parsed.user_key().to_string();\n\n                if watch_set.contains(&user_key) {\n                    let value = unsafe { (*iter).value() }.to_string();\n\n                    out.entry(user_key).or_default().push(Issue320InternalVersionRow {\n                        seq: *parsed.sequence(),\n                        kind: issue320_kind_for_value_type(*parsed.ty()),\n                        value,\n                    });\n                }\n            }\n\n            unsafe {\n                (*iter).next();\n            }\n        }\n\n        let st = unsafe { (*iter).status() };\n        unsafe {\n            drop(Box::from_raw(iter));\n        }\n\n        assert!(\n            st.is_ok(),\n            "internal iterator status not OK while collecting internal versions: {}",\n            st.to_string()\n        );\n\n        out\n    }\n\n    fn issue320_sstable_summary(dbimpl: &mut DBImpl) -> String {\n        let mut summary = String::new();\n\n        let ok = <DBImpl as DBGetProperty>::get_property(\n            dbimpl,\n            "leveldb.sstables",\n            &mut summary,\n        );\n\n        if ok {\n            summary\n        } else {\n            String::from("<leveldb.sstables unavailable>")\n        }\n    }\n\n    fn issue320_render_internal_versions(\n        versions: &BTreeMap<String, Vec<Issue320InternalVersionRow>>,\n    ) -> String {\n        let mut out = String::new();\n\n        for (key, rows) in versions.iter() {\n            out.push_str(key);\n            out.push_str(" => ");\n\n            if rows.is_empty() {\n                out.push_str("[]");\n            } else {\n                out.push(\'[\');\n                for (idx, row) in rows.iter().enumerate() {\n                    if idx != 0 {\n                        out.push_str(" | ");\n                    }\n\n                    out.push_str(&format!(\n                        "{}:{}:{}",\n                        row.seq,\n                        row.kind,\n                        row.value\n                    ));\n                }\n                out.push(\']\');\n            }\n\n            out.push(\'\\n\');\n        }\n\n        out\n    }\n\n    fn issue320_assert_matches_model_or_panic(\n        context: &str,\n        dbimpl: &mut DBImpl,\n        keys: &[String],\n        expected: &BTreeMap<String, String>,\n        pre_close_log: Option<(u64, u64)>,\n    ) {\n        let actual = issue320_read_current_values(dbimpl, keys);\n\n        let mut mismatched_keys = Vec::new();\n        let mut mismatch_lines = Vec::new();\n\n        for key in keys.iter() {\n            let expected_value = expected\n                .get(key)\n                .cloned()\n                .unwrap_or_else(|| String::from("<missing-from-model>"));\n\n            let actual_value = actual\n                .get(key)\n                .cloned()\n                .unwrap_or_else(|| String::from("<missing-from-db>"));\n\n            if expected_value != actual_value {\n                mismatched_keys.push(key.clone());\n                mismatch_lines.push(format!(\n                    "{} expected=\'{}\' actual=\'{}\'",\n                    key,\n                    expected_value,\n                    actual_value\n                ));\n            }\n        }\n\n        if mismatch_lines.is_empty() {\n            return;\n        }\n\n        let watch_keys = issue320_choose_watch_keys(keys, &mismatched_keys);\n        let internal_versions = issue320_scan_internal_versions_for_keys(dbimpl, &watch_keys);\n        let sstable_summary = issue320_sstable_summary(dbimpl);\n\n        let mut message = String::new();\n\n        message.push_str(context);\n        message.push(\'\\n\');\n\n        if let Some((log_number, log_size)) = pre_close_log {\n            message.push_str(&format!(\n                "pre_close_log: number={} size={}\\n",\n                log_number,\n                log_size\n            ));\n        }\n\n        message.push_str(&format!(\n            "mismatch_count={} key_count={}\\n",\n            mismatch_lines.len(),\n            keys.len()\n        ));\n\n        message.push_str("mismatches:\\n");\n        for line in mismatch_lines.iter().take(12) {\n            message.push_str("  ");\n            message.push_str(line);\n            message.push(\'\\n\');\n        }\n\n        message.push_str("watch_internal_versions:\\n");\n        message.push_str(&issue320_render_internal_versions(&internal_versions));\n\n        message.push_str("sstables:\\n");\n        message.push_str(&sstable_summary);\n\n        panic!("{}", message);\n    }\n\n    fn issue320_verify_nonempty_current_wal(dbimpl: &mut DBImpl) -> (u64, u64) {\n        let log_number = dbimpl.logfile_number;\n        let log_path = log_file_name(&dbimpl.dbname, log_number);\n\n        let log_size = std::fs::metadata(&log_path)\n            .map(|m| m.len())\n            .unwrap_or(0);\n\n        assert!(\n            log_size > 0,\n            "expected non-empty current WAL before close, but {} has size {}",\n            log_path,\n            log_size\n        );\n\n        (log_number, log_size)\n    }\n\n    fn run_issue320_flush_only_fanout_case_after_forced_compactions(case: Issue320FanoutCase) {\n        let tempdir = TempDir::new().unwrap();\n        let dbname = tempdir\n            .path()\n            .join(case.label)\n            .to_string_lossy()\n            .to_string();\n\n        std::fs::create_dir_all(&dbname).unwrap();\n\n        let (db_ptr, dbimpl_ptr) = open_db_for_issue320_fanout_reducer(&dbname);\n        let dbimpl = unsafe { &mut *dbimpl_ptr };\n\n        let keys = make_issue320_keys(case.key_count);\n        let mut model = BTreeMap::<String, String>::new();\n        let mut snapshots: Vec<Box<dyn Snapshot>> = Vec::new();\n\n        for wave in 0..case.wave_count {\n            issue320_apply_wave(\n                dbimpl,\n                &keys,\n                &mut model,\n                wave,\n                case.label,\n            );\n\n            issue320_flush_memtable(\n                dbimpl,\n                &format!("{}:after_wave_flush_{}", case.label, wave),\n            );\n\n            if case.snapshot_waves.contains(&wave) {\n                snapshots.push(issue320_take_snapshot(dbimpl));\n            }\n\n            issue320_assert_matches_model_or_panic(\n                &format!("{}:after_flush_wave_{}", case.label, wave),\n                dbimpl,\n                &keys,\n                &model,\n                None,\n            );\n        }\n\n        issue320_force_compact_all_levels(\n            dbimpl,\n            &format!("{}:forced_compactions", case.label),\n        );\n\n        issue320_assert_matches_model_or_panic(\n            &format!("{}:after_forced_compactions", case.label),\n            dbimpl,\n            &keys,\n            &model,\n            None,\n        );\n\n        issue320_release_snapshots(dbimpl, &mut snapshots);\n        drop_open_db(db_ptr);\n    }\n\n    fn run_issue320_reopen_case_with_verified_nonempty_wal(case: Issue320FanoutCase) {\n        let tempdir = TempDir::new().unwrap();\n        let dbname = tempdir\n            .path()\n            .join(case.label)\n            .to_string_lossy()\n            .to_string();\n\n        std::fs::create_dir_all(&dbname).unwrap();\n\n        let (db_ptr, dbimpl_ptr) = open_db_for_issue320_fanout_reducer(&dbname);\n        let dbimpl = unsafe { &mut *dbimpl_ptr };\n\n        let keys = make_issue320_keys(case.key_count);\n        let mut model = BTreeMap::<String, String>::new();\n        let mut snapshots: Vec<Box<dyn Snapshot>> = Vec::new();\n\n        for wave in 0..case.wave_count {\n            issue320_apply_wave(\n                dbimpl,\n                &keys,\n                &mut model,\n                wave,\n                case.label,\n            );\n\n            issue320_flush_memtable(\n                dbimpl,\n                &format!("{}:after_wave_flush_{}", case.label, wave),\n            );\n\n            if case.snapshot_waves.contains(&wave) {\n                snapshots.push(issue320_take_snapshot(dbimpl));\n            }\n\n            issue320_assert_matches_model_or_panic(\n                &format!("{}:after_flush_wave_{}", case.label, wave),\n                dbimpl,\n                &keys,\n                &model,\n                None,\n            );\n        }\n\n        issue320_force_compact_all_levels(\n            dbimpl,\n            &format!("{}:forced_compactions_before_tail", case.label),\n        );\n\n        issue320_assert_matches_model_or_panic(\n            &format!("{}:after_forced_compactions_before_tail", case.label),\n            dbimpl,\n            &keys,\n            &model,\n            None,\n        );\n\n        let tail_wave = case.wave_count;\n        issue320_apply_wave(\n            dbimpl,\n            &keys,\n            &mut model,\n            tail_wave,\n            case.label,\n        );\n\n        issue320_assert_matches_model_or_panic(\n            &format!("{}:after_tail_write_before_close", case.label),\n            dbimpl,\n            &keys,\n            &model,\n            None,\n        );\n\n        let pre_close_log = issue320_verify_nonempty_current_wal(dbimpl);\n\n        issue320_release_snapshots(dbimpl, &mut snapshots);\n        drop_open_db(db_ptr);\n\n        let (reopened_db_ptr, reopened_dbimpl_ptr) = open_db_for_issue320_fanout_reducer(&dbname);\n        let reopened_dbimpl = unsafe { &mut *reopened_dbimpl_ptr };\n\n        issue320_assert_matches_model_or_panic(\n            &format!("{}:after_reopen_with_verified_nonempty_wal", case.label),\n            reopened_dbimpl,\n            &keys,\n            &model,\n            Some(pre_close_log),\n        );\n\n        drop_open_db(reopened_db_ptr);\n    }\n\n    const ISSUE320_FANOUT_8_KEYS_1_SNAPSHOT: Issue320FanoutCase = Issue320FanoutCase {\n        label: "issue320_flush_only_fanout_8_keys_1_snapshot",\n        key_count: 8,\n        wave_count: 6,\n        snapshot_waves: &[0],\n    };\n\n    const ISSUE320_FANOUT_16_KEYS_1_SNAPSHOT: Issue320FanoutCase = Issue320FanoutCase {\n        label: "issue320_flush_only_fanout_16_keys_1_snapshot",\n        key_count: 16,\n        wave_count: 6,\n        snapshot_waves: &[0],\n    };\n\n    const ISSUE320_FANOUT_32_KEYS_2_SNAPSHOTS: Issue320FanoutCase = Issue320FanoutCase {\n        label: "issue320_flush_only_fanout_32_keys_2_snapshots",\n        key_count: 32,\n        wave_count: 6,\n        snapshot_waves: &[0, 2],\n    };\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_forced_compactions() {\n        issue320_require_protection();\n        run_issue320_flush_only_fanout_case_after_forced_compactions(\n            ISSUE320_FANOUT_8_KEYS_1_SNAPSHOT,\n        );\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_forced_compactions() {\n        issue320_require_protection();\n        run_issue320_flush_only_fanout_case_after_forced_compactions(\n            ISSUE320_FANOUT_16_KEYS_1_SNAPSHOT,\n        );\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_forced_compactions() {\n        issue320_require_protection();\n        run_issue320_flush_only_fanout_case_after_forced_compactions(\n            ISSUE320_FANOUT_32_KEYS_2_SNAPSHOTS,\n        );\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal() {\n        issue320_require_protection();\n        run_issue320_reopen_case_with_verified_nonempty_wal(\n            ISSUE320_FANOUT_8_KEYS_1_SNAPSHOT,\n        );\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal() {\n        issue320_require_protection();\n        run_issue320_reopen_case_with_verified_nonempty_wal(\n            ISSUE320_FANOUT_16_KEYS_1_SNAPSHOT,\n        );\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_reopen_with_verified_nonempty_wal() {\n        issue320_require_protection();\n        run_issue320_reopen_case_with_verified_nonempty_wal(\n            ISSUE320_FANOUT_32_KEYS_2_SNAPSHOTS,\n        );\n    }\n}\n#[cfg(test)]\nmod issue320_isolation_tests {\n    use super::*;\n\n    const ISSUE320_KEY_LEN: usize = 1024;\n    const ISSUE320_VALUE_LEN: usize = 1024;\n    const ISSUE320_HOT_KEY_COUNT: usize = 8;\n    const ISSUE320_FILLER_PER_WAVE: usize = 16;\n    const ISSUE320_WAVES: usize = 24;\n    const ISSUE320_SNAPSHOT_SLOTS: usize = 8;\n\n    fn issue320_blob(tag: u8, a: usize, b: usize, len: usize) -> String {\n        let mut out = format!("{}:{:08x}:{:08x}:", tag as char, a as u32, b as u32);\n\n        let mut x: u64 =\n            0x9e37_79b9_7f4a_7c15u64\n                ^ ((tag as u64) << 56)\n                ^ ((a as u64) << 24)\n                ^ (b as u64);\n\n        while out.len() < len {\n            x = x\n                .wrapping_mul(6364136223846793005u64)\n                .wrapping_add(1442695040888963407u64);\n\n            let ch: u8 = b\'a\' + (((x >> 33) % 26) as u8);\n            out.push(ch as char);\n        }\n\n        out.truncate(len);\n        out\n    }\n\n    fn issue320_hot_key(index: usize) -> String {\n        issue320_blob(b\'k\', index, 0, ISSUE320_KEY_LEN)\n    }\n\n    fn issue320_filler_key(wave: usize, index: usize) -> String {\n        issue320_blob(b\'f\', wave, index, ISSUE320_KEY_LEN)\n    }\n\n    fn issue320_value(wave: usize, index: usize) -> String {\n        issue320_blob(b\'v\', wave, index, ISSUE320_VALUE_LEN)\n    }\n\n    fn issue320_options() -> Options {\n        let env = PosixEnv::shared();\n        let mut options = Options::with_env(env);\n\n        assert!(options.env().is_some());\n\n        options.set_create_if_missing(true);\n        options.set_error_if_exists(false);\n        options.set_paranoid_checks(true);\n\n        // Keep files and flushes small so the test forces lots of compaction work\n        // without needing the huge randomized runtime of upstream issue320_test.\n        options.set_write_buffer_size(32 * 1024);\n        options.set_max_file_size(16 * 1024);\n\n        options\n    }\n\n    fn issue320_open_db(options: &Options, dbname: &String) -> *mut dyn DB {\n        let mut opener = DBImpl::new(options, dbname);\n\n        let mut out_db: *mut dyn DB =\n            core::ptr::null_mut::<DBImpl>() as *mut dyn DB;\n\n        let status = <DBImpl as DBOpen>::open(\n            &mut opener,\n            options,\n            dbname,\n            &mut out_db as *mut *mut dyn DB,\n        );\n\n        assert!(\n            status.is_ok(),\n            "DBOpen::open failed for {}: {}",\n            dbname,\n            status.to_string()\n        );\n        assert!(\n            !out_db.is_null(),\n            "DBOpen::open returned null DB pointer for {}",\n            dbname\n        );\n\n        out_db\n    }\n\n    unsafe fn issue320_db_ptr_to_dbimpl_mut(db_ptr: *mut dyn DB) -> *mut DBImpl {\n        let data: *mut () = db_ptr as *mut ();\n        data as *mut DBImpl\n    }\n\n    fn issue320_close_db(db_ptr: *mut dyn DB) {\n        assert!(!db_ptr.is_null());\n        unsafe {\n            drop(Box::from_raw(db_ptr));\n        }\n    }\n\n    fn issue320_assert_bg_error_ok(db: &DBImpl, label: &str) {\n        assert!(\n            db.bg_error.is_ok(),\n            "{}: bg_error={}",\n            label,\n            db.bg_error.to_string()\n        );\n    }\n\n    fn issue320_force_flush_and_compact(db: &mut DBImpl, wave: usize) {\n        let flush_status = db.test_compact_mem_table();\n        assert!(\n            flush_status.is_ok(),\n            "wave {}: test_compact_mem_table failed: {}",\n            wave,\n            flush_status.to_string()\n        );\n\n        let max_manual_level: i32 = std::cmp::min(4, (NUM_LEVELS as i32) - 1);\n\n        for level in 0..max_manual_level {\n            tracing::info!(\n                wave,\n                level,\n                "issue320 isolation: forcing manual compaction"\n            );\n\n            db.test_compact_range(level, core::ptr::null(), core::ptr::null());\n\n            issue320_assert_bg_error_ok(\n                db,\n                &format!("wave {} level {} after test_compact_range", wave, level),\n            );\n        }\n    }\n\n    fn issue320_get_value_or_panic(db: &mut DBImpl, key: &str) -> String {\n        let mut value = String::new();\n\n        let status = <DBImpl as DBGet>::get(\n            db,\n            &ReadOptions::default(),\n            &Slice::from_str(key),\n            &mut value,\n        );\n\n        assert!(\n            status.is_ok(),\n            "Get failed for key(len={}): {}",\n            key.len(),\n            status.to_string()\n        );\n\n        value\n    }\n\n    fn issue320_assert_current_model(\n        db: &mut DBImpl,\n        keys: &[String],\n        expected: &[String],\n        label: &str,\n    ) {\n        assert_eq!(keys.len(), expected.len());\n\n        for (idx, (key, expected_value)) in keys.iter().zip(expected.iter()).enumerate() {\n            let actual = issue320_get_value_or_panic(db, key);\n\n            assert_eq!(\n                actual,\n                *expected_value,\n                "{}: mismatch at hot-key index {}",\n                label,\n                idx\n            );\n        }\n    }\n\n    fn issue320_run_overwrite_only_workload(\n        db: &mut DBImpl,\n        keep_snapshots: bool,\n    ) -> Vec<(String, String)> {\n        let hot_keys: Vec<String> = (0..ISSUE320_HOT_KEY_COUNT)\n            .map(issue320_hot_key)\n            .collect();\n\n        let mut expected: Vec<String> = vec![String::new(); ISSUE320_HOT_KEY_COUNT];\n\n        let mut snapshots: Vec<Option<Box<dyn Snapshot>>> =\n            (0..ISSUE320_SNAPSHOT_SLOTS).map(|_| None).collect();\n\n        for wave in 0..ISSUE320_WAVES {\n            tracing::info!(\n                wave,\n                keep_snapshots,\n                "issue320 isolation: starting deterministic wave"\n            );\n\n            let mut batch = WriteBatch::default();\n\n            for hot_idx in 0..ISSUE320_HOT_KEY_COUNT {\n                let value = issue320_value(wave, hot_idx);\n\n                let key_slice = Slice::from_str(&hot_keys[hot_idx]);\n                let value_slice = Slice::from_str(&value);\n\n                batch.put(&key_slice, &value_slice);\n                expected[hot_idx] = value;\n            }\n\n            for filler_idx in 0..ISSUE320_FILLER_PER_WAVE {\n                let filler_key = issue320_filler_key(wave, filler_idx);\n                let filler_value = issue320_value(wave, ISSUE320_HOT_KEY_COUNT + filler_idx);\n\n                let key_slice = Slice::from_str(&filler_key);\n                let value_slice = Slice::from_str(&filler_value);\n\n                batch.put(&key_slice, &value_slice);\n            }\n\n            let write_status = <DBImpl as DBWrite>::write(\n                db,\n                &WriteOptions::default(),\n                &mut batch as *mut WriteBatch,\n            );\n\n            assert!(\n                write_status.is_ok(),\n                "wave {}: write failed: {}",\n                wave,\n                write_status.to_string()\n            );\n\n            if keep_snapshots && (wave % 3 == 0) {\n                let slot: usize = (wave / 3) % ISSUE320_SNAPSHOT_SLOTS;\n\n                if let Some(old_snapshot) = snapshots[slot].take() {\n                    <DBImpl as DBReleaseSnapshot>::release_snapshot(db, old_snapshot);\n                }\n\n                let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);\n                snapshots[slot] = Some(snapshot);\n            }\n\n            issue320_force_flush_and_compact(db, wave);\n\n            if (wave % 4 == 0) || (wave + 1 == ISSUE320_WAVES) {\n                issue320_assert_current_model(\n                    db,\n                    &hot_keys,\n                    &expected,\n                    &format!("post-wave-{}", wave),\n                );\n            }\n        }\n\n        for snapshot in snapshots.into_iter().flatten() {\n            <DBImpl as DBReleaseSnapshot>::release_snapshot(db, snapshot);\n        }\n\n        hot_keys.into_iter().zip(expected.into_iter()).collect()\n    }\n\n    fn issue320_verify_pairs(db: &mut DBImpl, expected: &[(String, String)], label: &str) {\n        for (idx, (key, value)) in expected.iter().enumerate() {\n            let actual = issue320_get_value_or_panic(db, key);\n\n            assert_eq!(\n                actual,\n                *value,\n                "{}: mismatch at pair index {}",\n                label,\n                idx\n            );\n        }\n    }\n\n    fn issue320_test_db_path(test_name: &str) -> String {\n        let tmp = TempDir::new().unwrap();\n        tmp.path()\n            .join(test_name)\n            .to_string_lossy()\n            .to_string()\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_control_overwrite_only_without_snapshots_matches_model_after_forced_compactions() {\n        issue320_require_protection();\n        let dbname = issue320_test_db_path(\n            "leveldb_issue320_isolation_control_without_snapshots",\n        );\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = issue320_options();\n        let db_ptr = issue320_open_db(&options, &dbname);\n\n        {\n            let dbimpl_ptr = unsafe { issue320_db_ptr_to_dbimpl_mut(db_ptr) };\n            assert!(!dbimpl_ptr.is_null());\n\n            let db = unsafe { &mut *dbimpl_ptr };\n            db.clear_background_error_for_test();\n\n            let expected = issue320_run_overwrite_only_workload(db, false);\n\n            issue320_assert_bg_error_ok(db, "control test final bg_error");\n            issue320_verify_pairs(db, &expected, "control test final verify");\n        }\n\n        issue320_close_db(db_ptr);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_overwrite_only_with_live_snapshots_matches_model_after_forced_compactions() {\n        issue320_require_protection();\n        let dbname = issue320_test_db_path(\n            "leveldb_issue320_isolation_with_live_snapshots",\n        );\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = issue320_options();\n        let db_ptr = issue320_open_db(&options, &dbname);\n\n        {\n            let dbimpl_ptr = unsafe { issue320_db_ptr_to_dbimpl_mut(db_ptr) };\n            assert!(!dbimpl_ptr.is_null());\n\n            let db = unsafe { &mut *dbimpl_ptr };\n            db.clear_background_error_for_test();\n\n            let expected = issue320_run_overwrite_only_workload(db, true);\n\n            issue320_assert_bg_error_ok(db, "snapshot test final bg_error");\n            issue320_verify_pairs(db, &expected, "snapshot test final verify");\n        }\n\n        issue320_close_db(db_ptr);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n\n    #[test]\n#[ignore = "resource-controlled database case; use contrib/repair/bitcoinleveldb-issue320-controlled020.py"]\n    fn issue320_overwrite_only_with_live_snapshots_matches_model_after_reopen() {\n        issue320_require_protection();\n        let dbname = issue320_test_db_path(\n            "leveldb_issue320_isolation_with_live_snapshots_reopen",\n        );\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = issue320_options();\n\n        let expected: Vec<(String, String)> = {\n            let db_ptr = issue320_open_db(&options, &dbname);\n\n            let result = {\n                let dbimpl_ptr = unsafe { issue320_db_ptr_to_dbimpl_mut(db_ptr) };\n                assert!(!dbimpl_ptr.is_null());\n\n                let db = unsafe { &mut *dbimpl_ptr };\n                db.clear_background_error_for_test();\n\n                let expected = issue320_run_overwrite_only_workload(db, true);\n\n                issue320_assert_bg_error_ok(db, "pre-reopen final bg_error");\n                issue320_verify_pairs(db, &expected, "pre-reopen final verify");\n\n                expected\n            };\n\n            issue320_close_db(db_ptr);\n            result\n        };\n\n        let reopened_db_ptr = issue320_open_db(&options, &dbname);\n\n        {\n            let dbimpl_ptr = unsafe { issue320_db_ptr_to_dbimpl_mut(reopened_db_ptr) };\n            assert!(!dbimpl_ptr.is_null());\n\n            let db = unsafe { &mut *dbimpl_ptr };\n            db.clear_background_error_for_test();\n\n            issue320_assert_bg_error_ok(db, "post-reopen initial bg_error");\n            issue320_verify_pairs(db, &expected, "post-reopen verify");\n        }\n\n        issue320_close_db(reopened_db_ptr);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n}\n'
DOCUMENT_BYTES=b"# Controlled Issue320 tests\n\nThis change controls how `bitcoinleveldbt-issue320` database tests are launched. It does not identify the cause of the reported host freeze or change a production database or C API implementation.\n\nAll 18 existing database tests remain present, retain their assertions, and become explicitly ignored by an ordinary Cargo test run. They use ordinary `#[test]` attributes rather than automatically installing a traced-test subscriber. A new ignored `issue320_bounded_diagnostic` exercises a smaller execution profile through the original C API workload body. Every database test checks its protection before creating a database, selecting a temporary database path, or drawing random values.\n\nTwo ordinary tests, `issue320_execution_profile_contract` and `issue320_guard_predicate_rejects_unbounded_and_invalid_controls`, check profile constants and the pure limit predicate. They do not open a database, read cgroup files, or install a tracing subscriber. An ordinary Cargo run therefore does not execute any of the 19 database cases; success of those two contracts is not database-workload coverage.\n\n## Workload profiles\n\n| Parameter | Original `issue320_test` | `issue320_bounded_diagnostic` |\n| --- | ---: | ---: |\n| Loop iterations | 200,000 | 256 |\n| Model slots | 10,000 | 64 |\n| Snapshot slots | 100 | 8 |\n| Deletion target size | 10,000 | 32 |\n| C random seed | 0 | 0 |\n| Generated key and value size | 1,024 bytes | 1,024 bytes |\n| Delete before replacement put | false | false |\n| Retain and rotate snapshots | true | true |\n\nThe original profile preserves its workload bounds, random-number calls, C API call order, status/value assertions, and successful snapshot-release, close, database-destroy and option-destroy sequence. Added counters and progress records do not consume random numbers or perform database operations. The 17 other existing database cases retain their bodies apart from an initial protection check and their test attributes.\n\nThe bounded diagnostic is a scaled workload, not a replay of the first 256 operations of the original profile. Different model and snapshot indices and a different deletion threshold can change branches and later random-number consumption. Passing it does not establish that the 200,000-iteration profile passes.\n\n## Protection and execution\n\nUse the controlled runner in `contrib/repair/bitcoinleveldb-issue320-controlled020.py`. Database cases are selected individually with exact names; they must not be launched as an unrestricted group of ignored tests.\n\nEach database case requires `BITCOIN_RS_ISSUE320_GUARD=v1` and verifies its actual cgroup v2 membership and these kernel controls:\n\n| Control | Accepted value |\n| --- | --- |\n| `memory.max` | finite, positive, at most 2 GiB |\n| `pids.max` | finite, positive, at most 64 |\n| `memory.swap.max` | 0 |\n| `memory.oom.group` | 1 |\n| `cpu.max` | positive finite quota and period; quota at most period |\n\nThe environment marker alone is insufficient. The Rust check inspects existing limits; it does not create them. The runner must also establish and verify wall-clock, output, per-file and aggregate test-data controls, execute one case per contained process, and terminate the entire case process group when a limit is reached. Aggregate data monitoring is a monitored allowance rather than a filesystem quota. The run receipt records the selected limits and actual guard outcomes.\n\nThe shared C API workload emits `ISSUE320_PROGRESS` around opening, selected iterations and writes, snapshot release, close and cleanup. It emits `ISSUE320_COUNTS` and `ISSUE320_CASE_OK` only after its original cleanup calls return. The original database-destroy error handling is retained, so that marker alone does not prove successful removal of every test file. A case is accepted only when its required evidence accompanies a normal successful test exit; a partial progress record is not a pass. The existing diagnostics retain their existing assertions and failure paths.\n\n## Evidence limits\n\nThe repair runner works in its isolated candidate checkout and records exact source, executable, case-selection and preservation bindings. Its receipt states which cases actually ran and whether the original full workload ran. Test enumeration, pure contracts, the bounded workload and existing small diagnostics are distinct evidence and must not be reported as full original-workload coverage.\n\nA resource stop, assertion failure, panic or abnormal exit does not qualify a repair and does not establish the freeze's cause. This change adds no debugger, sanitizer, race detector, allocator intervention, privilege escalation, package installation, or deliberately invalid-memory input. The randomized, `bitcoin-leveldb`, and harness crates require their own review and verification.\n\n## Guard binding names\n\nThe guard uses `swap_limit_text` for the swap-limit text to avoid the broad `bitcoin-imports` reexports of pointer and memory `swap` functions. This binding change does not alter the required `memory.swap.max=0` kernel control or any workload. Revision002 retains the compiler-only first attempt and refreshes only the exact candidate source, replay and documentation; the existing private cache is reused, and every native case still requires fresh qualification.\n\nCase receipts keep `ignored` as the numeric libtest outcome (zero for an executed passing case); `ignored_by_default` separately records whether explicit selection was required.\n"
CASES=[{'ignored': False, 'marker': None, 'profile': 'pure_contract', 'suffix': 'issue320_execution_profile_contract'}, {'ignored': False, 'marker': None, 'profile': 'pure_contract', 'suffix': 'issue320_guard_predicate_rejects_unbounded_and_invalid_controls'}, {'ignored': True, 'marker': 'ISSUE320_CASE_OK case=issue320_test', 'profile': 'full_original', 'suffix': 'issue320_test'}, {'ignored': True, 'marker': 'ISSUE320_CASE_OK case=issue320_bounded_diagnostic', 'profile': 'bounded_diagnostic', 'suffix': 'issue320_bounded_diagnostic'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_probe_single_key_without_snapshot_keeps_only_newest_version_after_manual_compaction'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_probe_single_key_with_live_snapshot_survives_flush_without_manual_compaction'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_probe_single_key_with_live_snapshot_manual_compaction_reports_retained_versions_on_failure'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_probe_single_key_with_live_snapshot_reopen_reports_before_and_after_state_compactly'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_reducer_two_keys_one_snapshot_matches_model_after_forced_compactions'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_reducer_two_keys_one_snapshot_matches_model_after_reopen_with_wal_tail'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_reducer_two_keys_two_snapshots_matches_model_after_forced_compactions'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_reducer_two_keys_two_snapshots_matches_model_after_reopen_with_wal_tail'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_forced_compactions'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_forced_compactions'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_forced_compactions'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_reopen_with_verified_nonempty_wal'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_control_overwrite_only_without_snapshots_matches_model_after_forced_compactions'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_overwrite_only_with_live_snapshots_matches_model_after_forced_compactions'}, {'ignored': True, 'marker': None, 'profile': 'existing_database_case', 'suffix': 'issue320_overwrite_only_with_live_snapshots_matches_model_after_reopen'}]


"""Exact source/index qualification for the isolated Issue320020 candidate.

These functions perform only bounded reads and read-only Git metadata queries.
The optional adapter configures the previously pinned finite Git worker; it does
not invoke it, install source, reserve a journal or read an identity by itself.
"""
import hashlib
import os
from pathlib import Path
import re


WORKFLOW = 'leveldb-issue320-controlled020'
BASE = '9845b3664d48d58ddcdd2dc9e7fa3c11783d3f85'
BASE_TREE = '1a43624b2061b4e13f4d30853fca2921344a9312'
BRANCH = 'repair/leveldb-issue320-controlled020'
SUFFIX = '-leveldb-issue320-controlled020'
SOURCE_PATH = 'bitcoinleveldbt-issue320/src/bitcoinleveldbt_issue320.rs'
REPLAY_PATH = 'contrib/repair/bitcoinleveldb-issue320-controlled020.py'
DOCUMENT_PATH = 'doc/testing/leveldb-issue320-controlled020.md'
RECEIPT_PATH = 'doc/repair/leveldb-issue320-controlled020.json'
INITIAL_PATHS = frozenset((SOURCE_PATH, REPLAY_PATH, DOCUMENT_PATH))


def _candidate_oid020(value):
    return isinstance(value, str) and re.fullmatch(r'[0-9a-f]{40}', value) is not None


def _candidate_blob020(raw):
    return hashlib.sha1(b'blob ' + str(len(raw)).encode() + b'\0' + raw).hexdigest()


def _candidate_files020(core, files):
    if (not isinstance(files, dict) or set(files) not in (INITIAL_PATHS, INITIAL_PATHS | {RECEIPT_PATH})
            or any(not isinstance(raw, bytes) or len(raw) > 2 * core.MIB for raw in files.values())):
        raise RuntimeError('Issue320 candidate requires the exact three/four bounded file payloads.')
    return files


def candidate_original_guard020(core, repo):
    """Verify original master identity while allowing unrelated staged/user work."""
    repo = Path(repo)
    if not repo.is_absolute() or '..' in repo.parts:
        raise RuntimeError('Original Issue320 repository is not an absolute ordinary checkout.')
    fd = core.open_dir(repo); os.close(fd)
    inv = core.Inventory(repo); core.repository(inv)
    _, head = inv.git('rev-parse', '--verify', 'HEAD')
    _, branch = inv.git('symbolic-ref', '--quiet', 'HEAD')
    _, tree = inv.git('rev-parse', '--verify', 'HEAD^{tree}')
    if (head.decode().strip(), branch.decode().strip(), tree.decode().strip()) != (BASE, 'refs/heads/master', BASE_TREE):
        raise RuntimeError('Original master differs from the exact reviewed Issue320 base/tree.')
    code, filters = inv.git('config', '--name-only', '--get-regexp', r'^filter\.', allowed=(0, 1))
    if code == 0 and filters.strip():
        raise RuntimeError('Configured Git filters require review before candidate inspection.')
    return inv


def _candidate_location020(core, repo, worktree, expected_head):
    repo, worktree = Path(repo), Path(worktree)
    if (not _candidate_oid020(expected_head) or not worktree.is_absolute() or '..' in worktree.parts
            or worktree != repo.parent / (repo.name + SUFFIX)):
        raise RuntimeError('Issue320 candidate location or expected head differs from the isolated plan.')
    fd = core.open_dir(worktree); os.close(fd)
    inv = candidate_original_guard020(core, repo)
    _, top = inv.git('rev-parse', '--show-toplevel', cwd=worktree)
    _, common = inv.git('rev-parse', '--path-format=absolute', '--git-common-dir', cwd=worktree)
    _, head = inv.git('rev-parse', '--verify', 'HEAD', cwd=worktree)
    _, branch = inv.git('symbolic-ref', '--quiet', 'HEAD', cwd=worktree)
    if (Path(os.fsdecode(top).strip()) != worktree or Path(os.fsdecode(common).strip()) != repo / '.git'
            or head.decode().strip() != expected_head or branch.decode().strip() != 'refs/heads/' + BRANCH):
        raise RuntimeError('Issue320 candidate top level, common root, branch or checkpoint differs.')
    code, filters = inv.git('config', '--name-only', '--get-regexp', r'^filter\.', allowed=(0, 1), cwd=worktree)
    if code == 0 and filters.strip():
        raise RuntimeError('Configured candidate Git filters require review before index/source inspection.')
    _, flags = inv.git('ls-files', '-v', '-z', cwd=worktree)
    if any(row and row[:2] != b'H ' for row in flags.split(b'\0')):
        raise RuntimeError('Issue320 candidate index contains a hidden/nonstandard visibility flag.')
    return inv


def _candidate_reservation020(core, inv, worktree, files, expected_head):
    if expected_head != BASE or set(files) != INITIAL_PATHS:
        raise RuntimeError('Pristine reservation validation is restricted to BASE and the initial three-file plan.')
    _, status = inv.git('status', '--porcelain=v1', '-z', '--untracked-files=all', cwd=worktree)
    _, untracked = inv.git('ls-files', '--others', '-z', cwd=worktree)
    if status or untracked:
        raise RuntimeError('Issue320 reservation must remain a pristine BASE checkout before installation.')
    _, base_source = inv.git('cat-file', 'blob', BASE + ':' + SOURCE_PATH, cwd=worktree)
    fingerprint, live = core.read_file(worktree / SOURCE_PATH, 2 * core.MIB)
    if live != base_source or fingerprint['mode'] != 0o644:
        raise RuntimeError('Reserved Issue320 source differs from its committed BASE bytes/mode.')
    for name in (REPLAY_PATH, DOCUMENT_PATH, RECEIPT_PATH):
        if os.path.lexists(worktree / name):
            raise RuntimeError('An Issue320 generated path is occupied before source installation: ' + name)
    return {'head': expected_head, 'branch': 'refs/heads/' + BRANCH, 'phase': 'pristine_base',
            'changed_paths': [], 'index_entries': {}}


def check_candidate020(core, repo, worktree, files, expectedhead, allow_base_files=False):
    """Validate pristine reservation, exact staged3/4, or exact committed4."""
    if type(allow_base_files) is not bool:
        raise RuntimeError('Issue320 reservation-mode flag must be boolean.')
    files = _candidate_files020(core, files)
    repo, worktree = Path(repo), Path(worktree)
    inv = _candidate_location020(core, repo, worktree, expectedhead)
    if allow_base_files:
        return _candidate_reservation020(core, inv, worktree, files, expectedhead)
    staged = expectedhead == BASE
    if not staged and set(files) != INITIAL_PATHS | {RECEIPT_PATH}:
        raise RuntimeError('Committed Issue320 qualification requires the complete four-file scope.')
    _, unstaged = inv.git('diff', '--name-only', '--no-ext-diff', '--no-textconv', '-z', cwd=worktree)
    _, untracked = inv.git('ls-files', '--others', '-z', cwd=worktree)
    if unstaged or untracked:
        raise RuntimeError('Issue320 candidate has unstaged or untracked bytes outside its exact installed tree.')
    _, raw_staged = inv.git('diff', '--cached', '--name-only', '--no-ext-diff', '--no-textconv', '-z', cwd=worktree)
    changed = [os.fsdecode(item) for item in raw_staged.split(b'\0') if item]
    wanted = set(files) if staged else set()
    if set(changed) != wanted or len(changed) != len(wanted):
        raise RuntimeError('Issue320 candidate staged paths differ from the exact installed scope.')
    expected = {}
    for name, data in files.items():
        fingerprint, live = core.read_file(worktree / name, 2 * core.MIB)
        mode = '100755' if name == REPLAY_PATH else '100644'
        if live != data or fingerprint['mode'] != (int(mode, 8) & 0o777):
            raise RuntimeError('Issue320 candidate live bytes or mode differ: ' + name)
        expected[name] = [mode, _candidate_blob020(data)]
    _, raw_index = inv.git('ls-files', '--stage', '-z', '--', *sorted(files), cwd=worktree)
    indexed = {}
    for row in raw_index.split(b'\0'):
        if not row:
            continue
        header, name = row.split(b'\t', 1)
        mode, blob, stage = header.decode('ascii').split()
        name = os.fsdecode(name)
        if stage != '0' or name in indexed or not _candidate_oid020(blob):
            raise RuntimeError('Issue320 candidate index has ambiguous/unmerged entries.')
        indexed[name] = [mode, blob]
    if indexed != expected:
        raise RuntimeError('Issue320 candidate index blobs/modes differ from the reviewed live bytes.')
    result = {'head': expectedhead, 'branch': 'refs/heads/' + BRANCH,
              'phase': 'staged' if staged else 'committed', 'changed_paths': sorted(files), 'index_entries': indexed}
    if not staged:
        _, parents = inv.git('rev-list', '--parents', '-n', '1', expectedhead, cwd=worktree)
        if parents.decode().split() != [expectedhead, BASE]:
            raise RuntimeError('Issue320 committed candidate must have the exact single BASE parent.')
        _, raw = inv.git('diff-tree', '--no-commit-id', '-r', '--raw', '-z', '--no-abbrev',
                         '--no-renames', '--no-ext-diff', '--no-textconv', BASE, expectedhead, cwd=worktree)
        parts = raw.split(b'\0')
        if parts and not parts[-1]: parts.pop()
        if len(parts) != 2 * len(files):
            raise RuntimeError('Issue320 committed candidate has a different path count.')
        committed = {}
        for offset in range(0, len(parts), 2):
            fields = parts[offset].decode('ascii').split(); name = os.fsdecode(parts[offset + 1])
            if (len(fields) != 5 or not fields[0].startswith(':') or fields[4] not in {'A', 'M'}
                    or name in committed):
                raise RuntimeError('Issue320 committed candidate has an unexpected operation.')
            committed[name] = [fields[1], fields[3]]
        if committed != expected:
            raise RuntimeError('Issue320 committed candidate source/path/mode scope differs.')
        _, tree = inv.git('rev-parse', '--verify', expectedhead + '^{tree}', cwd=worktree)
        result['tree'] = tree.decode().strip()
    return result


def configure_git_helper020(helper):
    """Configure only the pinned015 generic finite Git/commit helper surface."""
    helper.WORKFLOW, helper.BASE, helper.BRANCH, helper.SUFFIX = WORKFLOW, BASE, BRANCH, SUFFIX
    helper.REPLAY_PATH, helper.DOCUMENT_PATH, helper.RECEIPT_PATH = REPLAY_PATH, DOCUMENT_PATH, RECEIPT_PATH
    helper._w015_initial_paths = lambda: set(INITIAL_PATHS)
    helper._w015_original = candidate_original_guard020
    def exact_candidate(core, repo, worktree, files, head, staged=True):
        if type(staged) is not bool or staged != (head == BASE):
            raise RuntimeError('Issue320 Git helper requested a different candidate phase.')
        return check_candidate020(core, repo, worktree, files, head)
    helper.check_candidate = exact_candidate
    return helper


"""Finite Cargo observation for the controlled Issue320 library-test build.

The parent must launch this observer under the reviewed resource guard. This
module never builds unless cargo_observer is explicitly invoked with its one exact
offline no-run command shape. Cargo stderr remains attached to that guard.
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
ALLOWED_PACKAGES = ("bitcoinleveldbt-issue320",)
ISSUE320_COMMAND = ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", "bitcoinleveldbt-issue320", "--lib", "--no-run", "--message-format=json"]
_ISSUE320_COMMAND_SHAPE = tuple(ISSUE320_COMMAND)
ISSUE320_TARGETS = {"bitcoinleveldbt_issue320": ("lib", "src/lib.rs")}


MAX_EXECUTABLES = 40


def _observer_spec(spec):
    if not isinstance(spec, dict):
        raise RuntimeError("Cargo observer specification is not an object.")
    command = spec.get("command")
    if not isinstance(command, list):
        raise RuntimeError("Cargo observer command must be the exact reviewed argument list.")
    if tuple(command) != _ISSUE320_COMMAND_SHAPE:
        raise RuntimeError("Cargo observer requires the one exact reviewed Issue320 library-test command; no alternate target is admitted.")
    expected = ["bitcoinleveldbt-issue320"]
    targets = ISSUE320_TARGETS
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
        raise RuntimeError("Cargo test source is outside the private Issue320 worktree.") from error
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

    Return 0 only when the exact command succeeded and emitted its complete selected target inventory.
    On malformed output or an output allowance violation, stop Cargo promptly.
    """
    evidence = {
        "schema": "bitcoinrs-issue320020-cargo-observer-v1", "command": None,
        "cwd": None, "target_directory": None, "expected_packages": [],
        "cargo_exit_code": None, "observer_exit_code": 2, "error": None,
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
        if evidence["cargo_exit_code"] != 0:
            raise RuntimeError("Cargo exited unsuccessfully.")
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
        evidence["observer_exit_code"] = 0
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
        sys.stderr.write("ISSUE320020_CARGO_OBSERVER_REJECTED " + (evidence["error"] or "invalid specification") + "\n")
        return 2
    try:
        _atomic_evidence(paths["evidence_path"], evidence)
    except Exception as error:
        sys.stderr.write("ISSUE320020_CARGO_EVIDENCE_ERROR " + str(error)[:1024] + "\n")
        return 3
    print("ISSUE320020_CARGO_OBSERVER records=" + str(evidence["records"])
          + " executables=" + str(len(evidence["artifacts"]))
          + " native_exit=" + str(evidence["cargo_exit_code"])
          + " observer_exit=" + str(evidence["observer_exit_code"]), flush=True)
    return evidence["observer_exit_code"]


"""Post-exit transport for the controlled Issue320 repair020.

Only the separately authorized original return-buffer is replaced after the
coordinator has exited and printed/saved return JSON identities agree.
"""
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import stat
import subprocess
import sys
import threading
import uuid


_TRANSPORT_MAX_BYTES = 8 * 1024 * 1024


_TRANSPORT_WORKFLOW = "leveldb-issue320-controlled020"


_TRANSPORT_BEGIN = b"BEGIN_ISSUE320_REPAIR_RETURN"


_TRANSPORT_END = b"END_ISSUE320_REPAIR_RETURN"


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
            raise RuntimeError("Intake transport JSON contains duplicate fields.")
        result[key] = value
    return result


def _transport_report(capture, repo, child_status):
    lines = capture.splitlines()
    begins = [index for index, line in enumerate(lines) if line == _TRANSPORT_BEGIN]
    ends = [index for index, line in enumerate(lines) if line == _TRANSPORT_END]
    if len(begins) != 1 or len(ends) != 1 or begins[0] >= ends[0]:
        raise RuntimeError("Completed output lacks one unambiguous Issue320 repair return JSON block.")
    report = json.loads(b"\n".join(lines[begins[0] + 1:ends[0]]).decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if (not isinstance(report, dict) or report.get("workflow") != _TRANSPORT_WORKFLOW
            or report.get("repository") != str(repo) or not isinstance(report.get("outcome"), str)):
        raise RuntimeError("Printed Issue320 repair return does not identify this coordinator/repository.")
    value = report.get("return_file")
    if not isinstance(value, str) or not value or "\x00" in value:
        raise RuntimeError("Issue320 repair return-file path is absent or malformed.")
    path = Path(value)
    root = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW / "runs"
    if not path.is_absolute() or ".." in path.parts:
        raise RuntimeError("Issue320 repair return-file path is not ordinary and absolute.")
    try:
        relative = path.relative_to(root)
    except ValueError as error:
        raise RuntimeError("Issue320 repair return file is outside the owned workflow.") from error
    if len(relative.parts) != 2 or relative.name != "ISSUE320_REPAIR_RETURN.json":
        raise RuntimeError("Issue320 repair return-file layout differs from the owned run layout.")
    if "run_directory" in report and report["run_directory"] != str(path.parent):
        raise RuntimeError("Intake run directory differs from its return file.")
    raw, fp, _ = _transport_read_file(path, _TRANSPORT_MAX_BYTES)
    retained = json.loads(raw.decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if retained != report:
        raise RuntimeError("Printed Issue320 repair return differs from its retained JSON file.")
    successful = report["outcome"] in {"qualified_candidate", "already_qualified"}
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
            raise RuntimeError("Existing Issue320 repair transport lock is nonordinary or foreign-owned.")
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
        payload = (bytes(capture) + b"\nBEGIN_ISSUE320_REPAIR_RETURN_FILE_JSON\n" + return_raw
                   + (b"" if return_raw.endswith(b"\n") else b"\n")
                   + b"END_ISSUE320_REPAIR_RETURN_FILE_JSON\n" + confirmation)
        if len(payload) > _TRANSPORT_MAX_BYTES:
            raise RuntimeError("Complete coordinator output plus exact return file exceeds the 8 MiB payload allowance; prior return-buffer was retained.")
        # No lock is held during the coordinator. Reacquire its existing own
        # workflow lock afterward to avoid overlapping a new snapshot/run.
        lock_fd = _transport_lock(repo)
        before, after = _transport_replace_buffer(repo, payload)
        buffer_written = True
        buffer_replaced = True
        transport = {"schema": "bitcoinrs-issue320-controlled020-buffer-transport-v1", "outcome": "written",
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
            # repair. Finish draining its bounded coordinator lifecycle.
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


def load_checked020(name, path, expected):
    raw = bootstrap_read(path, 2 * MIB020)
    if hashlib.sha256(raw).hexdigest() != expected:
        raise RuntimeError('Retained reviewed controller differs: ' + path.name)
    module = types.ModuleType(name); module.__file__ = str(path); sys.modules[name] = module
    exec(compile(raw, str(path), 'exec'), module.__dict__)
    return module


def runtime020(repo):
    retained = repo / '.git/codex-repairs'
    helper = load_checked020('issue320020_git015', retained / 'leveldb-writablefile-shared015/runs/20261108T035429Z-804585ba/writable-file-controller015.py',
                             'd2ab9c4583284d91eec10db22914e5779e0400b1ba0324df95088a9f59bbfe1a')
    finalizer, core = helper.modules015(repo)
    configure_git_helper020(helper)
    core.WORKFLOW, core.BRANCH, core.WORKTREE_SUFFIX = WORKFLOW, BRANCH, SUFFIX
    intake = load_checked020('issue320020_source019', retained / 'leveldb-final-crates-intake019/runs/20261108T062738Z-9a59dd6e/final-crates-intake-controller019.py',
                             '52a8fa39d34049bc41e399d6ebfe702a5a8dbf04420af6123d9a38b76c6cd9e6')
    return finalizer, core, helper, intake


def checked_environment020(core):
    core.checked_environment()
    bad = []
    exact = {'CARGO_BUILD_TARGET', 'CARGO_BUILD_RUSTFLAGS', 'CARGO_BUILD_RUSTC', 'CARGO_BUILD_RUSTC_WRAPPER',
             'RUSTC', 'RUSTDOC', 'RUSTUP_TOOLCHAIN'}
    for key, value in os.environ.items():
        if value and (key in exact or key.startswith('CARGO_PROFILE_') or
                      (key.startswith('CARGO_TARGET_') and key.endswith(('_RUSTFLAGS', '_RUSTC_WRAPPER', '_RUNNER')))):
            bad.append(key)
    if bad:
        raise RuntimeError('Build overrides require review: ' + repr(sorted(bad)))


def intake_binding020(core, intake, repo, full=False):
    candidate_original_guard020(core, repo)
    directory = repo / '.git/codex-repairs/leveldb-final-crates-intake019/runs/20261108T062738Z-9a59dd6e'
    path = directory / 'FINAL_CRATES_INTAKE_RETURN.json'
    fp, raw = core.read_file(path, 2 * MIB020)
    if fp['size'] != 815818 or fp['sha256'] != '4bab014b2fdd2d98b5ab1d74e4d7ff85b882cf41ee3c083950bab42ddcf209ec':
        raise RuntimeError('Retained successful four-crate source intake differs.')
    report = json.loads(raw)
    expected = {'workflow': 'leveldb-final-crates-intake019', 'controller_revision': 2, 'outcome': 'captured',
                'phase': 'complete', 'source_capture_complete': True, 'original_preserved': True,
                'base': BASE, 'builds_started': False, 'native_tests_started': False, 'repair_qualified': False}
    if any(report.get(k) != v for k, v in expected.items()):
        raise RuntimeError('Retained intake is not the complete reviewed source-only capture.')
    source = report['source_capture']
    if (source['files'][SOURCE_PATH]['live_fingerprint']['sha256'] != SOURCE_BEFORE_SHA
            or source.get('source_omissions') != [] or source.get('input_count') != 106):
        raise RuntimeError('Retained Issue320 source binding or intake inventory differs.')
    if full:
        proof = intake.verify_sources019(core, repo, source)
        if proof.get('inputs_preserved') is not True or proof.get('fingerprint_count') != 106:
            raise RuntimeError('Current original source inputs differ from the reviewed intake.')
    return {'report': str(path), 'fingerprint': fp, 'source_before_sha256': SOURCE_BEFORE_SHA,
            'input_count': 106, 'qualification': 'selected_source_preservation_only'}


def files020(raw):
    return {SOURCE_PATH: SOURCE_BYTES, REPLAY_PATH: raw, DOCUMENT_PATH: DOCUMENT_BYTES}


def owned_worker020(core, repo, output):
    own = repo / '.git/codex-repairs' / WORKFLOW / 'runs'
    if (not output.is_absolute() or '..' in output.parts or output.parent.parent != own
            or not re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', output.parent.name)):
        raise RuntimeError('Issue320 worker output is outside its owned run.')
    fd = core.open_dir(output.parent); os.close(fd)


def inputs_worker020(repo, output):
    _, core, _, intake = runtime020(repo)
    owned_worker020(core, repo, output)
    core.atomic_json(output, intake_binding020(core, intake, repo, full=True))
    print('ISSUE320020_SOURCE_INPUTS_REVERIFIED', flush=True)
    return 0


def identity_worker020(repo, spec_path, output):
    finalizer, core, _, _ = runtime020(repo)
    owned_worker020(core, repo, output)
    if spec_path.parent != output.parent:
        raise RuntimeError('Configured identity input is outside its owned run.')
    spec = json.loads(core.read_file(spec_path, MIB020)[1])
    if not isinstance(spec, dict): raise RuntimeError('Configured identity input is malformed.')
    inv = candidate_original_guard020(core, repo); env = dict(inv.env)
    for key in finalizer.IDENTITY_ENV: env.pop(key, None)
    for key, value in spec.items():
        if key not in finalizer.IDENTITY_ENV or not isinstance(value, str) or '\0' in value:
            raise RuntimeError('Unexpected configured identity environment override.')
        env[key] = value
    result = {}
    for role in ('AUTHOR', 'COMMITTER'):
        command = inv.prefix + ['-c', 'user.useConfigOnly=true', 'var', 'GIT_' + role + '_IDENT']
        p = subprocess.run(command, cwd=repo, env=env, stdin=subprocess.DEVNULL,
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10, check=False)
        if p.returncode or len(p.stdout) > 4096 or len(p.stderr) > 8192:
            raise RuntimeError('Configured Git identity is unavailable; no identity was guessed.')
        result[role.lower()] = finalizer.parse_ident(p.stdout.decode().rstrip('\n'))
    core.atomic_json(output, result); print('ISSUE320020_CONFIGURED_IDENTITIES_RECORDED', flush=True)
    return 0


def install_worker020(repo):
    _, core, helper, intake = runtime020(repo)
    intake_binding020(core, intake, repo, full=True)
    worktree = repo.parent / (repo.name + SUFFIX)
    raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB020); files = files020(raw)
    check_candidate020(core, repo, worktree, files, BASE, allow_base_files=True)
    if core.read_file(worktree / SOURCE_PATH, MIB020)[0]['sha256'] != SOURCE_BEFORE_SHA:
        raise RuntimeError('Pristine Issue320 source differs from the captured original.')
    for name, content in sorted(files.items()):
        destination = worktree / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        fd = core.open_dir(destination.parent); os.close(fd)
        mode = 0o755 if name == REPLAY_PATH else 0o644
        if name == SOURCE_PATH:
            temporary = destination.with_name(destination.name + '.issue320020-' + uuid.uuid4().hex)
            helper.candidate_exclusive015(core, temporary, content, mode)
            os.replace(temporary, destination)
        else:
            if os.path.lexists(destination): raise RuntimeError('New candidate path is occupied: ' + name)
            helper.candidate_exclusive015(core, destination, content, mode)
    print('ISSUE320020_SOURCE_INSTALLED', flush=True)
    return 0


def seed_worker020(repo, source, target, output):
    _, core, helper, _ = runtime020(repo)
    owned_worker020(core, repo, output)
    if (source != repo / '.git/codex-repairs/leveldb-writablefile-shared017/cargo-target'
            or target != repo / '.git/codex-repairs' / WORKFLOW / 'cargo-target'):
        raise RuntimeError('Cache seed is outside the exact independent-copy plan.')
    return helper.copy_cache_worker_revision002(core, source, target, output)


def artifact_check020(core, artifacts):
    if set(artifacts) != {ARTIFACT_KEY}:
        raise RuntimeError('Issue320 qualification requires its exact one library-test executable.')
    for value in artifacts.values():
        if core.read_file(Path(value['path']), 256 * MIB020, content=False)[0] != value['fingerprint']:
            raise RuntimeError('Tested Issue320 executable changed.')


def choose_cases020(all_names, ignored_names):
    if len(all_names) != len(set(all_names)) or len(ignored_names) != len(set(ignored_names)):
        raise RuntimeError('Repeated names in Issue320 executable listing.')
    chosen = []
    for item in CASES:
        matches = [name for name in all_names if name == item['suffix'] or name.endswith('::' + item['suffix'])]
        if len(matches) != 1: raise RuntimeError('Selected Issue320 case absent or ambiguous: ' + item['suffix'])
        chosen.append(dict(item, name=matches[0]))
    if set(all_names) != {item['name'] for item in chosen} or len(chosen) != 21:
        raise RuntimeError('Compiled Issue320 test inventory differs from the reviewed twenty-one cases.')
    if set(ignored_names) != {item['name'] for item in chosen if item['ignored']}:
        raise RuntimeError('Compiled default/ignored Issue320 execution policy differs.')
    priority = {'pure_contract': 0, 'bounded_diagnostic': 1, 'existing_database_case': 2, 'full_original': 3}
    return sorted(chosen, key=lambda item: (priority[item['profile']], item['suffix']))


def case_command020(artifact, item):
    command = [artifact, '--exact', item['name']]
    if item['ignored']: command.append('--ignored')
    return command + ['--nocapture', '--test-threads=1']


def receipt020(core, report, files):
    return {'workflow': WORKFLOW, 'base': BASE, 'source_plan': {k: core.digest(v) for k, v in files.items()},
            'controller_revision': CONTROLLER_REVISION,
            'controller_transition': report['controller_transition_before'],
            'intake_binding': report['intake_before'], 'execution_policy': 'nineteen explicitly guarded database cases; two default pure contracts',
            'original_profile': {'seed': 0, 'model_slots': 10000, 'snapshot_slots': 100, 'target_size': 10000,
                                 'steps': 200000, 'key_bytes': 1024, 'value_bytes': 1024},
            'bounded_profile': {'model_slots': 64, 'snapshot_slots': 8, 'target_size': 32, 'steps': 256,
                                'classification': 'scaled workload, not a literal full-profile prefix'},
            'cases': [{k: v for k, v in case.items() if k != 'log'} for case in report['cases']],
            'artifacts': report['artifacts'], 'build_command': ISSUE320_COMMAND, 'test_count': 21,
            'run_directory': report['run_directory'], 'original_full_case_completed': True,
            'historical_freeze_cause_established': False, 'full_coverage_claimed': False,
            'full_randomized_stress': 'not rerun'}


def reuse020(finalizer, core, helper, repo, state, files):
    helper.check_native_journal015(state)
    if state.get('phase') != 'complete' or not state.get('qualified_report'):
        raise RuntimeError('An incomplete Issue320 attempt is retained; automatic rerunning is blocked.')
    path = Path(state['qualified_report']); owned_worker020(core, repo, path)
    if path.name != 'ISSUE320_REPAIR_RETURN.json': raise RuntimeError('Qualified return layout differs.')
    fp, raw = core.read_file(path, 4 * MIB020)
    if fp != state.get('qualified_report_fingerprint'): raise RuntimeError('Qualified Issue320 return changed.')
    prior = json.loads(raw); receipt = state['receipt']
    if (prior.get('outcome') != 'qualified_candidate' or prior.get('repair_qualified') is not True
            or prior.get('controller_revision') != CONTROLLER_REVISION
            or receipt.get('controller_revision') != CONTROLLER_REVISION
            or prior.get('controller_transition_before') != state.get('controller_transition')
            or receipt.get('controller_transition') != state.get('controller_transition')
            or prior.get('original_preserved') is not True or prior.get('original_full_case_completed') is not True
            or prior.get('test_count') != 21 or len(prior.get('cases', [])) != 21 or prior.get('receipt') != receipt
            or prior.get('artifacts') != receipt.get('artifacts')
            or prior.get('build_command') != ISSUE320_COMMAND or receipt.get('build_command') != ISSUE320_COMMAND
            or receipt.get('test_count') != 21 or receipt.get('original_full_case_completed') is not True
            or [{k: v for k, v in c.items() if k != 'log'} for c in prior['cases']] != receipt.get('cases')):
        raise RuntimeError('Retained Issue320 qualification is incomplete or contradictory.')
    complete = dict(files); complete[RECEIPT_PATH] = core.encoded(receipt)
    helper.verify_candidate_commit(finalizer, core, repo, state['head'], state['commit_intent'], complete)
    artifact_check020(core, prior['artifacts'])
    selected = choose_cases020(prior['test_lists']['all'], prior['test_lists']['ignored'])
    if {item['suffix'] for item in selected} != {item['label'] for item in prior['cases']}:
        raise RuntimeError('Retained selected case inventory differs.')
    by_label = {item['suffix']: item for item in selected}
    budgets = core.load_guard().BUDGETS
    for case in prior['cases']:
        item = by_label[case['label']]; row = prior['readouts'][case['label']]
        if (case.get('outcome') != 'passed' or case.get('profile') != item['profile']
                or type(case.get('ignored')) is not int or case['ignored'] != 0
                or case.get('ignored_by_default') is not item['ignored'] or case.get('expected_name') != item['name']
                or case.get('marker') != item['marker']):
            raise RuntimeError('Retained Issue320 case classification contradicts its selected inventory.')
        if row.get('command') != case_command020(prior['artifacts'][ARTIFACT_KEY]['path'], item):
            raise RuntimeError('Retained native command is not bound to its exact tested case/executable.')
        kind = 'stress' if item['profile'] == 'full_original' else 'case'
        if (row.get('kind') != kind or row.get('budget') != budgets[kind]
                or row.get('data_directory') != str(path.parent / ('case-data-' + case['label']))):
            raise RuntimeError('Retained native controls or monitored data root differ from the selected profile.')
        log_path = Path(row.get('log', ''))
        if log_path.name != 'output.log' or log_path.parent.parent.parent != path.parent:
            raise RuntimeError('Retained case log is outside its owned guard run.')
        text = core.read_file(log_path, MIB020)[1].decode(errors='replace')
        if text != case['log'] or core.assess_case(row, text, item['name'], item['marker']).get('outcome') != 'passed':
            raise RuntimeError('Retained exact Issue320 case evidence does not verify.')
    return prior


"""Narrow, archived transition from the exact pre-native Issue320 compiler stop."""
import hashlib
import json
import os
from pathlib import Path
import re
import sys
import types
import uuid

R1_RUN = '20261108T065134Z-b76f99ff'
R1_REPORT_SHA = 'f081969a83eab00f754be70968b565dfbc882b92d0e044df0f002b0f0ffaa9b3'
R1_REPORT_BYTES = 140011
R1_CONTROLLER_SHA = 'b37bc0748c4d827e9640c26a990e728f7a3fa649cc32ed961e0706229804834f'
R1_CONTROLLER_BYTES = 189508
R1_STATE_SHA = 'e237eb07c3731cceaf431a14fd653723781a393915f27c046e603a853968b36f'
R1_STATE_BYTES = 786
R2_RUST_SHA = '267f1aa5a7860161191dc32adb2dcf2dd4228ca0c476ff75443f00f241006937'
R2_RUST_BYTES = 90855
TRANSITION_REVIEW_ID020 = 'issue320020-r1-pre-native-swap-binding-collision'


def classify_compiler_stop020(report):
    if (report.get('workflow') != WORKFLOW or report.get('outcome') != 'failed'
            or report.get('phase') != 'compile-issue320-tests' or report.get('repair_qualified') is not False
            or report.get('native_tests_started') is not False or report.get('test_count') != 0
            or report.get('cases') != [] or report.get('master_updated') is not False
            or report.get('other_refs_updated') is not False or report.get('original_preserved') is not True
            or report.get('original_full_case_completed') is not False
            or report.get('validation_reused') is not False
            or report.get('intake_before') != report.get('intake_after')
            or report.get('preservation_before_sha256') != report.get('preservation_after_sha256')):
        raise RuntimeError('Prior Issue320 evidence is not the exact preserved pre-native compiler stop.')
    labels = {'preflight', 'inputs-before', 'read-configured-identity', 'git-worktree',
              'install-source', 'git-add', 'seed-private-cache', 'compile-issue320-tests'}
    if set(report.get('readouts', {})) != labels:
        raise RuntimeError('Prior Issue320 phase inventory differs from the reviewed eight phases.')
    for label, row in report['readouts'].items():
        expected = 2 if label == 'compile-issue320-tests' else 0
        if row.get('verified') is not True or row.get('outcome') != 'exited' or row.get('exit_code') != expected:
            raise RuntimeError('Prior Issue320 includes an unreviewed control/process failure.')
    cargo = report.get('cargo_evidence', {})
    levels = cargo.get('diagnostic_levels', {})
    diagnostics = cargo.get('priority_diagnostics', [])
    errors = [item for item in diagnostics if item.get('level') == 'error']
    notes = [item for item in diagnostics if item.get('level') == 'failure-note']
    if (cargo.get('command') != ISSUE320_COMMAND or cargo.get('cargo_exit_code') != 101
            or cargo.get('observer_exit_code') != 2 or cargo.get('build_finished') is not False
            or cargo.get('artifacts') != {} or cargo.get('error') != 'RuntimeError: Cargo exited unsuccessfully.'
            or levels.get('error') != 2 or levels.get('failure-note') != 2 or levels.get('ice', 0)
            or set(levels) - {'error', 'warning', 'failure-note'}
            or cargo.get('priority_records_omitted') != 0 or len(diagnostics) != 4
            or len(errors) != 2 or len(notes) != 2):
        raise RuntimeError('Prior Issue320 compilation is not the complete reviewed two-error failure.')
    if {item.get('target', {}).get('name') for item in notes} != {'bitcoinleveldb_logwriter', 'bitcoinleveldb_dbimplinner'}:
        raise RuntimeError('Prior explanatory note targets differ.')
    for item in notes:
        if (item.get('code') is not None or item.get('message') != 'For more information about this error, try `rustc --explain E0133`.'
                or item.get('spans') != []):
            raise RuntimeError('Prior explanatory note differs.')
    lines = set()
    for item in errors:
        spans = item.get('spans', [])
        if (item.get('code') != 'ambiguous_glob_imports' or item.get('message') != '`swap` is ambiguous'
                or item.get('target', {}).get('name') != 'bitcoinleveldbt_issue320'
                or len(spans) != 1 or spans[0].get('file_name') != SOURCE_PATH
                or spans[0].get('is_primary') is not True or item.get('spans_omitted') != 0):
            raise RuntimeError('Prior primary compiler diagnostic differs from the two swap binding errors.')
        lines.add(spans[0].get('line_start'))
    if lines != {24, 59}:
        raise RuntimeError('Prior swap binding error locations differ.')
    return {'review_id': TRANSITION_REVIEW_ID020,
            'classification': 'introduced_guard_local_binding_collides_with_glob_imported_swap',
            'native_tests_executed': 0, 'test_validation_reused': False, 'all_twenty_one_cases_must_run': True,
            'rust_change': 'rename exactly four guard binding/reference tokens to swap_limit_text'}


def expected_previous_state020(core, report):
    return {'identity': {'workflow': WORKFLOW, 'base': BASE, 'branch': BRANCH,
                         'worktree': report['worktree'], 'file_hashes': report['source_plan'],
                         'intake_sha256': core.digest(core.encoded(report['intake_before']))}, 'phase': 'installed'}


def _exact_renamed_source020(original):
    result = original
    for old, new in (
        (b'pids: &str, swap: &str, oom:', b'pids: &str, swap_limit_text: &str, oom:'),
        (b'&& pids <= 64 && swap == "0" && oom', b'&& pids <= 64 && swap_limit_text == "0" && oom'),
        (b'let swap = issue320_read_control', b'let swap_limit_text = issue320_read_control'),
        (b'(&memory, &pids, &swap, &oom, &cpu)', b'(&memory, &pids, &swap_limit_text, &oom, &cpu)'),
    ):
        if result.count(old) != 1:
            raise RuntimeError('Reviewed Rust rename anchor is absent or ambiguous.')
        result = result.replace(old, new)
    if len(result) != R2_RUST_BYTES or hashlib.sha256(result).hexdigest() != R2_RUST_SHA:
        raise RuntimeError('Reviewed four-token Rust rename differs from the pinned revision002 payload.')
    return result


def frozen_previous020(core, repo):
    directory = repo / '.git/codex-repairs' / WORKFLOW / 'runs' / R1_RUN
    report_path, controller_path = directory / 'ISSUE320_REPAIR_RETURN.json', directory / 'issue320-controller020.py'
    report_fp, report_raw = core.read_file(report_path, 2 * MIB020)
    controller_fp, controller_raw = core.read_file(controller_path, 2 * MIB020)
    if (report_fp['sha256'] != R1_REPORT_SHA or report_fp['size'] != R1_REPORT_BYTES
            or controller_fp['sha256'] != R1_CONTROLLER_SHA or controller_fp['size'] != R1_CONTROLLER_BYTES):
        raise RuntimeError('Frozen original Issue320 report/controller bytes differ.')
    report = json.loads(report_raw); proof = classify_compiler_stop020(report)
    worktree = repo.parent / (repo.name + SUFFIX)
    target = repo / '.git/codex-repairs' / WORKFLOW / 'cargo-target'
    if (report.get('repository') != str(repo) or report.get('worktree') != str(worktree)
            or report.get('run_directory') != str(directory) or report.get('target_directory') != str(target)
            or report.get('base') != BASE or report.get('branch') != BRANCH
            or report.get('controller_sha256') != R1_CONTROLLER_SHA
            or report['cargo_evidence'].get('cwd') != str(worktree)
            or report['cargo_evidence'].get('target_directory') != str(target)):
        raise RuntimeError('Frozen original Issue320 path/base bindings differ.')
    old = types.ModuleType('issue320020_frozen_revision001'); old.__file__ = str(controller_path)
    sys.modules[old.__name__] = old
    exec(compile(controller_raw, str(controller_path), 'exec'), old.__dict__)
    old_files = old.files020(controller_raw)
    if (set(old_files) != set(INITIAL_PATHS) or old.CASES != CASES or old.ISSUE320_COMMAND != ISSUE320_COMMAND
            or report.get('source_plan') != {name: core.digest(raw) for name, raw in old_files.items()}
            or _exact_renamed_source020(old_files[SOURCE_PATH]) != SOURCE_BYTES):
        raise RuntimeError('Revision002 changes more than the reviewed Rust rename and replay/document files.')
    expected_state_raw = core.encoded(expected_previous_state020(core, report))
    if len(expected_state_raw) != R1_STATE_BYTES or core.digest(expected_state_raw) != R1_STATE_SHA:
        raise RuntimeError('Derived original installed journal differs from its exact reviewed binding.')
    retained = {str(report_path): report_fp, str(controller_path): controller_fp}
    raw_bindings = {'previous-report.json': report_raw, 'previous-controller.py': controller_raw}
    for label, row in report['readouts'].items():
        path = Path(row['log'])
        if path.name != 'output.log' or path.parent.parent != directory / 'guard':
            raise RuntimeError('A prior guard log is outside its retained run.')
        fingerprint, raw = core.read_file(path, MIB020)
        if raw != row.get('tail', '').encode() or len(raw) != row.get('output_bytes'):
            raise RuntimeError('Retained prior full guard log bytes differ: ' + label)
        retained[str(path)] = fingerprint; raw_bindings['previous-logs/' + label + '.log'] = raw
    cargo_path = directory / 'cargo-artifacts.json'
    fingerprint, raw = core.read_file(cargo_path, MIB020)
    if json.loads(raw) != report['cargo_evidence']:
        raise RuntimeError('Retained prior Cargo diagnostics differ from the frozen return.')
    retained[str(cargo_path)] = fingerprint; raw_bindings['previous-cargo-artifacts.json'] = raw
    spec_path = directory / 'cargo.spec.json'
    fingerprint, raw = core.read_file(spec_path, MIB020)
    if json.loads(raw) != {'command': ISSUE320_COMMAND, 'cwd': str(worktree),
                           'target_directory': str(target), 'evidence_path': str(cargo_path)}:
        raise RuntimeError('Retained prior compiler specification differs.')
    retained[str(spec_path)] = fingerprint; raw_bindings['previous-cargo.spec.json'] = raw
    previous_sources = {}
    for name, raw in old_files.items():
        previous_sources[name] = {'kind': 'file', 'mode': 0o755 if name == REPLAY_PATH else 0o644,
                                  'size': len(raw), 'sha256': core.digest(raw)}
        raw_bindings['previous-candidate-sources/' + name] = raw
    proof.update(report_path=str(report_path), report_fingerprint=report_fp,
                 controller_path=str(controller_path), controller_fingerprint=controller_fp,
                 original_source_plan=report['source_plan'], previous_candidate_sources=previous_sources,
                 retained_bindings=retained,
                 previous_state_fingerprint={'kind': 'file', 'mode': 0o600,
                                             'size': R1_STATE_BYTES, 'sha256': R1_STATE_SHA})
    return proof, report, old_files, raw_bindings


def review_transition020(core, helper, repo, state_path):
    repo, state_path = Path(repo), Path(state_path)
    if state_path != repo / '.git/codex-repairs' / WORKFLOW / 'state.json':
        raise RuntimeError('Original Issue320 journal path differs.')
    proof, report, files, _ = frozen_previous020(core, repo)
    fingerprint, raw = core.read_file(state_path, 2 * MIB020)
    if (fingerprint != proof['previous_state_fingerprint']
            or raw != core.encoded(expected_previous_state020(core, report))):
        raise RuntimeError('Original journal is not the exact installed, pre-native state; no failure was cleared.')
    helper.check_candidate(core, repo, repo.parent / (repo.name + SUFFIX), files, BASE)
    return proof, raw


def _archive_directory020(core, repo, directory):
    if (directory.parent != repo / '.git/codex-repairs' / WORKFLOW / 'runs'
            or not re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', directory.name) or directory.name == R1_RUN):
        raise RuntimeError('Compiler review archive is outside a new owned run.')
    descriptor = core.open_dir(directory); os.close(descriptor)


def archive_transition020(core, repo, proof, old_state_raw, directory):
    repo, directory = Path(repo), Path(directory)
    _archive_directory020(core, repo, directory)
    expected, report, files, raw_bindings = frozen_previous020(core, repo)
    if (proof != expected or old_state_raw != core.encoded(expected_previous_state020(core, report))
            or core.read_file(repo / '.git/codex-repairs' / WORKFLOW / 'state.json', 2 * MIB020)[1] != old_state_raw):
        raise RuntimeError('Reviewed original journal/evidence changed before exclusive archival.')
    for name, expected_raw in files.items():
        fingerprint, raw = core.read_file(repo.parent / (repo.name + SUFFIX) / name, 2 * MIB020)
        if raw != expected_raw or fingerprint != proof['previous_candidate_sources'][name]:
            raise RuntimeError('Original candidate bytes/mode changed before archive: ' + name)
    raw_bindings['previous-state-before-compiler-review.json'] = old_state_raw
    archive = {}
    for name, raw in sorted(raw_bindings.items()):
        path = directory / name; path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
        descriptor = core.open_dir(path.parent); os.close(descriptor)
        core.exclusive(path, raw, 0o600)
        fingerprint, observed = core.read_file(path, 2 * MIB020)
        if observed != raw or fingerprint['mode'] != 0o600:
            raise RuntimeError('Exclusive compiler review archive bytes/mode differ.')
        archive[name] = {'path': str(path), 'fingerprint': fingerprint}
    transition = dict(proof, previous_state_path=str(directory / 'previous-state-before-compiler-review.json'),
                      archive_files=archive)
    verify_transition_archive020(core, repo, transition)
    return transition


def verify_transition_archive020(core, repo, transition):
    repo = Path(repo)
    expected, report, _, raw_bindings = frozen_previous020(core, repo)
    if not isinstance(transition, dict): raise RuntimeError('Compiler review transition is absent.')
    state_path = Path(transition.get('previous_state_path', '')); directory = state_path.parent
    _archive_directory020(core, repo, directory)
    if state_path.name != 'previous-state-before-compiler-review.json':
        raise RuntimeError('Previous journal archive filename differs.')
    raw_bindings['previous-state-before-compiler-review.json'] = core.encoded(expected_previous_state020(core, report))
    archive = transition.get('archive_files')
    if not isinstance(archive, dict) or set(archive) != set(raw_bindings):
        raise RuntimeError('Compiler review archive inventory differs.')
    for name, expected_raw in raw_bindings.items():
        path = directory / name
        fingerprint, raw = core.read_file(path, 2 * MIB020)
        if archive[name] != {'path': str(path), 'fingerprint': fingerprint} or raw != expected_raw or fingerprint['mode'] != 0o600:
            raise RuntimeError('Retained compiler review archive changed: ' + name)
    expected.update(previous_state_path=str(state_path), archive_files=archive)
    if transition != expected:
        raise RuntimeError('Compiler review proof differs from its exact frozen stop/archive.')
    return True


def refresh_controller020(core, helper, repo):
    repo = Path(repo)
    state_path = repo / '.git/codex-repairs' / WORKFLOW / 'state.json'
    state = json.loads(core.read_file(state_path, 4 * MIB020)[1])
    verify_transition_archive020(core, repo, state.get('controller_transition'))
    if (state.get('phase') != 'controller-refresh-pending' or 'native_pending' in state or 'native_failure' in state
            or set(state) != {'identity', 'phase', 'controller_transition'}):
        raise RuntimeError('Compiler refresh is not the exact pending, never-native transition state.')
    raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB020); files = files020(raw)
    if set(files) != set(INITIAL_PATHS):
        raise RuntimeError('Compiler refresh is restricted to the exact three planned source/replay/document files.')
    _, report, old_files, _ = frozen_previous020(core, repo)
    identity = {'workflow': WORKFLOW, 'base': BASE, 'branch': BRANCH,
                'worktree': str(repo.parent / (repo.name + SUFFIX)),
                'file_hashes': {name: core.digest(data) for name, data in files.items()},
                'intake_sha256': core.digest(core.encoded(report['intake_before'])), 'controller_revision': 2}
    if state.get('identity') != identity or files[SOURCE_PATH] != _exact_renamed_source020(old_files[SOURCE_PATH]):
        raise RuntimeError('Compiler refresh identity/payload differs from the exact revision002 plan.')
    worktree = repo.parent / (repo.name + SUFFIX)
    helper.check_candidate(core, repo, worktree, old_files, BASE)
    for name, data in sorted(files.items()):
        path = worktree / name; temporary = path.with_name(path.name + '.r2-' + uuid.uuid4().hex)
        helper.candidate_exclusive015(core, temporary, data, 0o755 if name == REPLAY_PATH else 0o644)
        os.replace(temporary, path)
    for name, expected in files.items():
        fingerprint, observed = core.read_file(worktree / name, 2 * MIB020)
        if observed != expected or fingerprint['mode'] != (0o755 if name == REPLAY_PATH else 0o644):
            raise RuntimeError('Refreshed candidate bytes/mode differ.')
    print('ISSUE320020_REVISION002_RENAME_INSTALLED', flush=True)
    return 0


def run020(repo):
    report = {'workflow': WORKFLOW, 'controller_revision': CONTROLLER_REVISION, 'outcome': 'failed', 'phase': 'preflight', 'repair_qualified': False,
              'base': BASE, 'branch': BRANCH, 'master_updated': False, 'other_refs_updated': False,
              'builds_started': False, 'native_tests_started': False, 'debuggers_started': False,
              'validation_reused': False, 'test_count': 0, 'expected_test_count': 21,
              'cases': [], 'readouts': {}, 'build_command': ISSUE320_COMMAND,
              'historical_freeze_cause_established': False, 'original_full_case_completed': False,
              'full_coverage_claimed': False, 'full_randomized_stress': 'not rerun',
              'other_active_targets': ['bitcoinleveldbt-randomized', 'bitcoin-leveldb', 'bitcoinleveldb-harness'],
              'other_active_targets_executed': False,
              'scope': 'Isolated Issue320 execution-control repair: two pure contracts, one scaled C-API workload, seventeen existing database diagnostics and the unchanged full original profile, each exact case in a separate guarded process.',
              'postrun_transport': 'Replace the authorized return-buffer only after coordinator exit.',
              'preservation_scope': 'Original user work, refs, indices, prior workflow journals and read intake through coordinator exit; own candidate branch/worktree/journal/private cache/reports may change. Subsequent authorized buffer replacement is separate.'}
    held, core, initial, worktree, directory, state, state_path, lineage, transition = [], None, None, None, None, None, None, None, None
    success = False
    try:
        repo = Path(os.path.abspath(Path(repo).expanduser())); report['repository'] = str(repo)
        finalizer, core, helper, intake = runtime020(repo)
        checked_environment020(core)
        common = core.repository(core.Inventory(repo)); held = core.acquire_locks(common)
        own = common / 'codex-repairs' / WORKFLOW; own.mkdir(mode=0o700, exist_ok=True)
        fd = core.open_dir(own); os.close(fd)
        lock = os.open(own / 'run.lock', os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600)
        held.append((lock, own / 'run.lock'))
        if not stat.S_ISREG(os.fstat(lock).st_mode): raise RuntimeError('Issue320 workflow lock is not ordinary.')
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        candidate_original_guard020(core, repo)
        worktree = repo.parent / (repo.name + SUFFIX)
        initial = core.normalized_original(core.protected_snapshot(core, repo), worktree)
        lineage = intake_binding020(core, intake, repo)
        report.update(preservation_before_sha256=core.digest(core.encoded(initial)), intake_before=lineage,
                      locks_held=[str(path) for _, path in held], worktree=str(worktree))
        runs = own / 'runs'; runs.mkdir(mode=0o700, exist_ok=True)
        fd = core.open_dir(runs); os.close(fd)
        directory = runs / (dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ') + '-' + uuid.uuid4().hex[:8])
        directory.mkdir(mode=0o700)
        script = directory / 'issue320-controller020.py'; raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB020)
        core.exclusive(script, raw, 0o700); files = files020(raw)
        report.update(run_directory=str(directory), controller_sha256=core.digest(raw), guard_sha256=core.GUARD_SHA,
                      source_plan={name: core.digest(value) for name, value in files.items()})
        state_path = own / 'state.json'
        identity = {'workflow': WORKFLOW, 'base': BASE, 'branch': BRANCH, 'worktree': str(worktree),
                    'file_hashes': report['source_plan'], 'intake_sha256': core.digest(core.encoded(lineage)),
                    'controller_revision': CONTROLLER_REVISION}
        guard_module = core.load_guard()
        guard = guard_module.Guard(directory / 'guard', script,
                                   dict(core.Inventory(repo).env, BITCOIN_RS_ISSUE320020_REPO=str(repo)))
        def preserve():
            candidate_original_guard020(core, repo)
            if core.normalized_original(core.protected_snapshot(core, repo), worktree) != initial:
                raise RuntimeError('Protected original or predecessor state changed.')
            if intake_binding020(core, intake, repo) != lineage:
                raise RuntimeError('Read source-intake evidence changed.')
            if transition is not None:
                verify_transition_archive020(core, repo, transition)
        def phase(label, command, kind='source', cwd=None, env=None):
            preserve(); report['phase'] = label
            print('Protected Issue320 repair phase: ' + label, flush=True)
            row = guard.run(label, command, cwd or repo, env or {}, kind=kind)
            report['readouts'][label] = row; core.require_guard(row)
            return row
        def git(label, args, cwd, extra=None, tag=None):
            tag = tag or label
            spec, output = directory / (tag + '.git.spec.json'), directory / (tag + '.git.json')
            value = {'label': label, 'args': args, 'cwd': str(cwd), 'output': str(output)}
            value.update(extra or {}); core.exclusive(spec, core.encoded(value))
            phase('git-' + tag, [sys.executable, '-I', '-S', str(script), '--git', str(repo), str(spec)], 'build', cwd)
            evidence = json.loads(core.read_file(output, 4 * MIB020)[1]); report.setdefault('git_evidence', {})[tag] = evidence
            if evidence.get('exit_code') != 0 or evidence.get('args') != args:
                raise RuntimeError('Exact isolated Git operation evidence differs.')
            return evidence['stdout'].strip()
        phase('preflight', [sys.executable, '-I', '-S', '-c', "print('GUARD_PREFLIGHT_OK')"], 'probe')
        before_inputs = directory / 'inputs-before.json'
        phase('inputs-before', [sys.executable, '-I', '-S', str(script), '--inputs', str(repo), str(before_inputs)])
        if json.loads(core.read_file(before_inputs, MIB020)[1]) != lineage:
            raise RuntimeError('Initial current source proof differs from retained intake binding.')
        if not os.path.lexists(state_path):
            raise RuntimeError('Revision002 requires the exact retained compiler-only first attempt; no new candidate was created.')
        state = json.loads(core.read_file(state_path, 4 * MIB020)[1])
        if state.get('identity') == identity:
            transition = state.get('controller_transition')
            verify_transition_archive020(core, repo, transition)
            report['controller_transition_before'] = transition
            prior = reuse020(finalizer, core, helper, repo, state, files)
            report.update(outcome='already_qualified', phase='complete', repair_qualified=True, validation_reused=True,
                          test_count=21, cases=prior['cases'], artifacts=prior['artifacts'],
                          candidate_head=state['head'], candidate_tree=state['commit_intent']['tree'],
                          qualified_report=state['qualified_report'], original_full_case_completed=True,
                          test_lists=prior['test_lists'], receipt=prior['receipt'])
            report['qualified_readouts'] = prior['readouts']
            success = True
        else:
            reviewed, previous_state_raw = review_transition020(core, helper, repo, state_path)
            transition = archive_transition020(core, repo, reviewed, previous_state_raw, directory)
            verify_transition_archive020(core, repo, transition)
            report['controller_transition_before'] = transition
            state = {'identity': identity, 'phase': 'controller-refresh-pending', 'controller_transition': transition}
            core.atomic_json(state_path, state)
            identity_spec, identity_out = directory / 'identity-input.json', directory / 'identity.json'
            core.exclusive(identity_spec, core.encoded({key: os.environ[key] for key in finalizer.IDENTITY_ENV if key in os.environ}))
            phase('read-configured-identity', [sys.executable, '-I', '-S', str(script), '--identity', str(repo), str(identity_spec), str(identity_out)])
            ident = json.loads(core.read_file(identity_out, MIB020)[1]); report['configured_identity'] = ident
            identity_env = {'GIT_' + role.upper() + '_' + field.upper(): ident[role][field.lower()]
                            for role in ('author', 'committer') for field in ('NAME', 'EMAIL')}
            phase('refresh-controller', [sys.executable, '-I', '-S', str(script), '--refresh-controller', str(repo)], 'source', worktree)
            git('add', ['add', '--', *sorted(files)], worktree)
            check_candidate020(core, repo, worktree, files, BASE)
            state['phase'] = 'installed'; core.atomic_json(state_path, state)
            target = own / 'cargo-target'
            fd = core.open_dir(target); os.close(fd)
            report.update(target_directory=str(target), existing_private_cache_reused=True, cache_copy_started=False)
            spec, evidence = directory / 'cargo.spec.json', directory / 'cargo-artifacts.json'
            core.exclusive(spec, core.encoded({'command': ISSUE320_COMMAND, 'cwd': str(worktree),
                                              'target_directory': str(target), 'evidence_path': str(evidence)}))
            nix = ['nix-shell', '--option', 'max-jobs', '0', '--option', 'builders', '', '--option', 'substitute', 'false',
                   str(worktree / 'shell.nix'), '--run', shlex.join([sys.executable, '-I', '-S', str(script), '--cargo-observer', str(spec)])]
            report['builds_started'] = True
            try:
                phase('compile-issue320-tests', nix, 'build', worktree,
                      {'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '1', 'CARGO_INCREMENTAL': '0'})
            finally:
                if evidence.exists(): report['cargo_evidence'] = json.loads(core.read_file(evidence, MIB020)[1])
            check_candidate020(core, repo, worktree, files, BASE)
            compiled = report['cargo_evidence']['artifacts']
            if set(compiled) != {ARTIFACT_KEY}: raise RuntimeError('Exact selected test executable was not compiled.')
            executable = Path(compiled[ARTIFACT_KEY]['executable'])
            artifacts = {ARTIFACT_KEY: {'path': str(executable), 'fingerprint': core.read_file(executable, 256 * MIB020, content=False)[0]}}
            report['artifacts'] = artifacts
            lists = {}
            for label, arguments in (('all', ['--list']), ('ignored', ['--list', '--ignored'])):
                row = phase('list-' + label, [str(executable), *arguments], 'case', worktree, {'RUST_LOG': 'off'})
                text = core.read_file(Path(row['log']), MIB020)[1].decode(errors='replace')
                lists[label] = [line[:-6] for line in text.splitlines() if line.endswith(': test')]
            report['test_lists'] = lists
            selected = choose_cases020(lists['all'], lists['ignored'])
            for item in selected:
                preserve(); check_candidate020(core, repo, worktree, files, BASE); artifact_check020(core, artifacts)
                label = item['suffix']; data = directory / ('case-data-' + label)
                data.mkdir(mode=0o700)
                state['native_pending'] = {'run': str(directory), 'case': item['name'], 'profile': item['profile']}
                core.atomic_json(state_path, state)
                report.update(phase=label, native_tests_started=True)
                print('Protected Issue320 repair phase: ' + label, flush=True)
                command = case_command020(str(executable), item)
                env = {'RUST_LOG': 'off', 'RUST_BACKTRACE': '0', 'TEST_RANDOM_SEED': '0',
                       'BITCOIN_RS_ISSUE320_GUARD': 'v1', 'TMPDIR': str(data), 'TMP': str(data), 'TEMP': str(data)}
                kind = 'stress' if item['profile'] == 'full_original' else 'case'
                row = guard.run(label, command, worktree, env, kind=kind, tmpdir=data)
                report['readouts'][label] = row
                text = core.read_file(Path(row['log']), MIB020)[1].decode(errors='replace') if row.get('log') else ''
                assessed = core.assess_case(row, text, item['name'], item['marker'])
                report['cases'].append(dict(assessed, label=label, profile=item['profile'], ignored_by_default=item['ignored'], log=text))
                report['test_count'] = len(report['cases'])
                if assessed.get('outcome') != 'passed':
                    state['native_failure'] = dict(state['native_pending']); core.atomic_json(state_path, state)
                    raise RuntimeError('Selected Issue320 case failed; exact evidence is retained and automatic qualification is blocked.')
                if item['profile'] == 'full_original': report['original_full_case_completed'] = True
                state.pop('native_pending'); core.atomic_json(state_path, state)
            helper.check_native_journal015(state)
            if report['test_count'] != 21 or report['original_full_case_completed'] is not True:
                raise RuntimeError('Selected Issue320 qualification is incomplete.')
            after_inputs = directory / 'inputs-after.json'
            phase('inputs-after', [sys.executable, '-I', '-S', str(script), '--inputs', str(repo), str(after_inputs)])
            if json.loads(core.read_file(after_inputs, MIB020)[1]) != lineage:
                raise RuntimeError('Original selected source inputs changed during candidate testing.')
            preserve(); check_candidate020(core, repo, worktree, files, BASE); artifact_check020(core, artifacts)
            receipt = receipt020(core, report, files); receipt_path = worktree / RECEIPT_PATH
            receipt_path.parent.mkdir(parents=True, exist_ok=True); fd = core.open_dir(receipt_path.parent); os.close(fd)
            helper.candidate_exclusive015(core, receipt_path, core.encoded(receipt), 0o644); files[RECEIPT_PATH] = core.encoded(receipt)
            git('add', ['add', '--', *sorted(files)], worktree, tag='add-final')
            check_candidate020(core, repo, worktree, files, BASE)
            tree = git('tree', ['write-tree'], worktree)
            if not _candidate_oid020(tree): raise RuntimeError('Candidate tree identity is absent.')
            message_bytes = b'test: control Issue320 database workloads and preserve full regression\n\nRequire verified resource controls for database cases, keep tracing quiet,\nand distinguish the scaled C-API diagnostic from the original workload.\n'
            message = directory / 'commit-message.txt'; core.exclusive(message, message_bytes)
            intent = {'tree': tree, 'parents': [BASE], 'identity': ident, 'message_sha256': core.digest(message_bytes)}
            state.update(phase='commit-intent', receipt=receipt, commit_intent=intent); core.atomic_json(state_path, state)
            head = git('commit-tree', ['commit-tree', tree, '-p', BASE, '-F', str(message)], worktree,
                       {'tree': tree, 'identity_env': identity_env})
            if not _candidate_oid020(head): raise RuntimeError('Candidate commit identity is absent.')
            state['created_commit'] = head; core.atomic_json(state_path, state)
            git('head', ['update-ref', 'refs/heads/' + BRANCH, head, BASE], worktree, {'head': head})
            helper.verify_candidate_commit(finalizer, core, repo, head, intent, files)
            artifact_check020(core, artifacts); preserve()
            state['head'] = head
            report.update(outcome='qualified_candidate', phase='complete', repair_qualified=True,
                          candidate_head=head, candidate_tree=tree, receipt=receipt, changed_paths=sorted(files))
            success = True
    except BaseException as error:
        report.update(outcome='failed', repair_qualified=False, error=type(error).__name__ + ': ' + str(error))
    finally:
        if initial is not None:
            try:
                after = core.normalized_original(core.protected_snapshot(core, repo), worktree)
                report['preservation_after_sha256'] = core.digest(core.encoded(after))
                report['original_preserved'] = after == initial
                report['intake_after'] = intake_binding020(core, intake, repo)
                if after != initial or report['intake_after'] != lineage:
                    raise RuntimeError('Protected original or read source-intake state changed.')
                if transition is not None:
                    report['controller_transition_reverified'] = verify_transition_archive020(core, repo, transition)
            except BaseException as error:
                success = False; report.update(outcome='failed', repair_qualified=False, preservation_error=str(error))
        if directory is not None:
            report['return_file'] = str(directory / 'ISSUE320_REPAIR_RETURN.json')
            try:
                core.atomic_json(Path(report['return_file']), report)
                if success and report['outcome'] == 'qualified_candidate':
                    state.update(phase='complete', qualified_report=report['return_file'],
                                 qualified_report_fingerprint=core.read_file(Path(report['return_file']), 4 * MIB020)[0])
                    core.atomic_json(state_path, state)
            except BaseException as error:
                report.update(outcome='failed', repair_qualified=False, publication_error=str(error))
                core.atomic_json(Path(report['return_file']), report)
        for fd, _ in held: os.close(fd)
    return report


def main020():
    sys.dont_write_bytecode = True; resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    if len(sys.argv) == 3 and sys.argv[1] == '--guard-child':
        plan = json.loads(bootstrap_read(Path(sys.argv[2]), MIB020))
        _, core, _, _ = runtime020(Path(plan['env']['BITCOIN_RS_ISSUE320020_REPO']))
        return core.load_guard().guard_child(Path(sys.argv[2]))
    if len(sys.argv) == 4 and sys.argv[1] == '--inputs':
        return inputs_worker020(Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 5 and sys.argv[1] == '--identity':
        return identity_worker020(Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]))
    if len(sys.argv) == 4 and sys.argv[1] == '--git':
        repo = Path(sys.argv[2]); _, core, helper, _ = runtime020(repo)
        return helper.run_git_operation(core, repo, Path(sys.argv[3]))
    if len(sys.argv) == 3 and sys.argv[1] == '--refresh-controller':
        repo = Path(sys.argv[2]); _, core, helper, _ = runtime020(repo)
        return refresh_controller020(core, helper, repo)
    if len(sys.argv) == 6 and sys.argv[1] == '--copy-cache':
        return seed_worker020(Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]), Path(sys.argv[5]))
    if len(sys.argv) == 3 and sys.argv[1] == '--cargo-observer':
        return cargo_observer(json.loads(bootstrap_read(Path(sys.argv[2]), MIB020)))
    if len(sys.argv) == 4 and sys.argv[1] == '--coordinator' and sys.argv[3] == 'apply':
        resource.setrlimit(resource.RLIMIT_AS, (768 * MIB020,) * 2)
        signal.signal(signal.SIGALRM, lambda *_: (_ for _ in ()).throw(RuntimeError('Issue320 coordinator deadline reached.')))
        signal.alarm(3600)
        report = run020(Path(sys.argv[2]))
        print('BEGIN_ISSUE320_REPAIR_RETURN', flush=True); print(json.dumps(report, indent=2), flush=True)
        print('END_ISSUE320_REPAIR_RETURN', flush=True)
        return 0 if report['outcome'] in {'qualified_candidate', 'already_qualified'} else 1
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=Path.home() / 'bitcoin-rs')
    args = parser.parse_args()
    return run_with_transport(Path(__file__).absolute(), Path(os.path.abspath(args.repo.expanduser())), True)


if __name__ == '__main__': raise SystemExit(main020())
