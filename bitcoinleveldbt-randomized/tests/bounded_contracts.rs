// Bounded model/database contract checks. The original stress tests remain separate.
// Seed 301 defines this small tape; this is not a replay of the 40,000-step run.
use bitcoinleveldb_batch::*;
use bitcoinleveldb_dbinterface::*;
use bitcoinleveldb_iteratorinner::*;
use bitcoinleveldb_modeldb::*;
use bitcoinleveldb_options::*;
use bitcoinleveldb_rand::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_snapshot::*;
use bitcoinleveldbt_dbtest::*;
use bitcoinleveldbt_snapshot::*;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

const KEY_LIMIT: usize = 8;
const SEEDED_STEPS: usize = 24;
const MEMORY_LIMIT: u64 = 512 * 1024 * 1024;

fn read_control(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(value) => value.trim().to_owned(),
        Err(error) => panic!("use the protected repair runner: cannot read {}: {}", path.display(), error),
    }
}

fn control_number(value: &str, name: &str) -> u64 {
    match value.parse::<u64>() {
        Ok(number) => number,
        Err(_) => panic!("use the protected repair runner: {} is not a finite limit", name),
    }
}

fn verify_protection() {
    assert!(matches!(std::env::var("BITCOIN_RS_RANDOMIZED_GUARD"), Ok(value) if value == "v1"),
            "use the protected repair runner; this test requires verified cgroup limits");
    let memberships = read_control(Path::new("/proc/self/cgroup"));
    let unified = match memberships.lines().find_map(|line| line.strip_prefix("0::")) {
        Some(value) => value,
        None => panic!("use the protected repair runner: unified cgroup membership missing"),
    };
    assert!(Path::new(unified).is_absolute(), "invalid unified cgroup path");
    assert!(Path::new(unified).components().all(|part| matches!(part, Component::RootDir | Component::Normal(_))),
            "invalid unified cgroup path components");
    let group = Path::new("/sys/fs/cgroup").join(unified.trim_start_matches('/'));
    let memory = control_number(&read_control(&group.join("memory.max")), "memory.max");
    let pids = control_number(&read_control(&group.join("pids.max")), "pids.max");
    assert!(memory > 0 && memory <= MEMORY_LIMIT, "use the protected repair runner: memory.max exceeds 512 MiB");
    assert!(pids > 0 && pids <= 64, "use the protected repair runner: pids.max exceeds 64");
    assert_eq!(read_control(&group.join("memory.swap.max")), "0", "use the protected repair runner: swap must be disabled");
    assert_eq!(read_control(&group.join("memory.oom.group")), "1", "use the protected repair runner: grouped OOM handling required");
    let cpu = read_control(&group.join("cpu.max"));
    let fields: Vec<&str> = cpu.split_whitespace().collect();
    assert_eq!(fields.len(), 2, "invalid cpu.max");
    let quota = control_number(fields[0], "cpu.max quota");
    let period = control_number(fields[1], "cpu.max period");
    assert!(quota > 0 && period > 0 && quota <= period,
            "use the protected repair runner: CPU quota must be at most one CPU");
    eprintln!("BOUNDED_RANDOMIZED_GUARD cgroup={} memory={} pids={} cpu={}/{}", group.display(), memory, pids, quota, period);
}

#[derive(Default)]
struct Counts {
    puts: usize,
    deletes: usize,
    batches: usize,
    empty_batches: usize,
    repeated_key_batches: usize,
    snapshots: usize,
    releases: usize,
    snapshot_after_mutation_checks: usize,
    reopens: usize,
    flushes: usize,
}

enum Update<'a> {
    Put(&'a str, &'a str),
    Delete(&'a str),
}

struct Case {
    fixture: DBTest,
    model: ModelDB,
    model_snapshot: Option<Box<dyn Snapshot>>,
    db_snapshot: Option<Box<dyn Snapshot>>,
    expected: BTreeMap<String, String>,
    saved_expected: Option<BTreeMap<String, String>>,
    config: usize,
    step: usize,
    counts: Counts,
}

impl Drop for Case {
    fn drop(&mut self) {
        // SnapshotImpl owns a node still linked into the database's snapshot list.
        // Release it before DBTest::drop, including when an assertion unwinds.
        self.release_inner();
    }
}

fn collect_rows<D: DBNewIterator + ?Sized>(db: &mut D, snapshot: Option<&dyn Snapshot>, is_model: bool) -> Vec<(String, String)> {
    let mut options = ReadOptions::default();
    options.set_snapshot(snapshot.map(|value| {
        if is_model {
            snapshot_read_arc_from_snapshot_ref(value)
        } else {
            dbtest_snapshot_read_arc_from_snapshot_ref(value)
        }
    }));
    let pointer = db.new_iterator(&options);
    assert!(!pointer.is_null(), "new_iterator returned null");
    let mut iterator = unsafe { Box::from_raw(pointer) };
    iterator.seek_to_first();
    let mut rows: Vec<(String, String)> = Vec::new();
    while iterator.valid() {
        assert!(rows.len() < KEY_LIMIT, "iterator exceeded the finite key universe");
        let key_slice = iterator.key();
        let value_slice = iterator.value();
        assert!(key_slice.as_bytes().len() <= 200, "unexpected oversized iterator key");
        assert!(value_slice.as_bytes().len() <= 64, "unexpected oversized iterator value");
        let key = key_slice.to_string();
        let value = value_slice.to_string();
        if let Some((previous, _)) = rows.last() {
            assert!(previous.as_bytes() < key.as_bytes(), "iterator did not advance in strict key order");
        }
        rows.push((key, value));
        iterator.next();
    }
    assert!(iterator.status().is_ok(), "iterator ended with an error");
    rows
}

impl Case {
    fn new(config: usize) -> Self {
        verify_protection(); // No database, environment worker, or iterator exists before this succeeds.
        eprintln!("BOUNDED_RANDOMIZED_PHASE config={} step=0 phase=construct.before", config);
        let mut fixture = DBTest::default();
        for _ in 0..config {
            assert!(fixture.change_options(), "requested option configuration unavailable");
        }
        let options = fixture.current_options();
        eprintln!("BOUNDED_RANDOMIZED_OPTIONS config={}", config);
        let model = ModelDB::new(&options);
        let case = Self { fixture, model, model_snapshot: None, db_snapshot: None,
                          expected: BTreeMap::new(), saved_expected: None,
                          config, step: 0, counts: Counts::default() };
        case.mark("construct.after");
        case
    }

    fn mark(&self, phase: &str) {
        eprintln!("BOUNDED_RANDOMIZED_PHASE config={} step={} phase={}", self.config, self.step, phase);
    }

    fn check(&mut self) {
        let expected: Vec<_> = self.expected.iter().map(|(key, value)| (key.clone(), value.clone())).collect();
        self.mark("compare.live.model.before");
        let model_rows = collect_rows(&mut self.model, None, true);
        self.mark("compare.live.database.before");
        let pointer = self.fixture.dbfull();
        assert!(!pointer.is_null(), "database missing during comparison");
        let database_rows = unsafe { collect_rows(&mut *pointer, None, false) };
        assert_eq!(model_rows, expected, "model differs from the independent expected map");
        assert_eq!(database_rows, expected, "database differs from the independent expected map");
        if let Some(saved) = self.saved_expected.as_ref() {
            let expected_saved: Vec<_> = saved.iter().map(|(key, value)| (key.clone(), value.clone())).collect();
            assert!(self.model_snapshot.is_some() && self.db_snapshot.is_some(), "saved snapshot pair is incomplete");
            self.mark("compare.saved.model.before");
            let model_saved = collect_rows(&mut self.model, self.model_snapshot.as_deref(), true);
            self.mark("compare.saved.database.before");
            let database_saved = unsafe { collect_rows(&mut *pointer, self.db_snapshot.as_deref(), false) };
            assert_eq!(model_saved, expected_saved, "model snapshot changed after acquisition");
            assert_eq!(database_saved, expected_saved, "database snapshot changed after acquisition");
            if saved != &self.expected {
                self.counts.snapshot_after_mutation_checks += 1;
            }
        }
        self.mark("compare.after");
    }

    fn put(&mut self, key: &str, value: &str) {
        self.mark("put.before");
        let options = WriteOptions::default();
        assert!(self.model.put(&options, &Slice::from_str(key), &Slice::from_str(value)).is_ok(), "model put failed");
        assert!(self.fixture.put(&key.to_owned(), &value.to_owned()).is_ok(), "database put failed");
        self.expected.insert(key.to_owned(), value.to_owned());
        self.counts.puts += 1;
        self.step += 1;
        self.mark("put.after");
        self.check();
    }

    fn delete(&mut self, key: &str) {
        self.mark("delete.before");
        let options = WriteOptions::default();
        assert!(self.model.delete(&options, &Slice::from_str(key)).is_ok(), "model delete failed");
        assert!(self.fixture.delete(&key.to_owned()).is_ok(), "database delete failed");
        self.expected.remove(key);
        self.counts.deletes += 1;
        self.step += 1;
        self.mark("delete.after");
        self.check();
    }

    fn batch(&mut self, operations: &[Update<'_>]) {
        self.mark("batch.before");
        let mut batch = WriteBatch::default();
        let mut distinct = BTreeSet::new();
        let mut repeated = false;
        for operation in operations {
            let key = match operation {
                Update::Put(key, value) => { batch.put(&Slice::from_str(key), &Slice::from_str(value)); *key }
                Update::Delete(key) => { batch.delete(&Slice::from_str(key)); *key }
            };
            repeated |= !distinct.insert(key);
        }
        let options = WriteOptions::default();
        assert!(self.model.write(&options, &mut batch as *mut WriteBatch).is_ok(), "model batch failed");
        let pointer = self.fixture.dbfull();
        assert!(!pointer.is_null(), "database missing during batch");
        assert!(unsafe { (&mut *pointer).write(&options, &mut batch as *mut WriteBatch) }.is_ok(), "database batch failed");
        for operation in operations {
            match operation {
                Update::Put(key, value) => { self.expected.insert((*key).to_owned(), (*value).to_owned()); }
                Update::Delete(key) => { self.expected.remove(*key); }
            }
        }
        self.counts.batches += 1;
        self.counts.empty_batches += usize::from(operations.is_empty());
        self.counts.repeated_key_batches += usize::from(repeated);
        self.step += 1;
        self.mark("batch.after");
        self.check();
    }

    fn save(&mut self) {
        assert!(self.model_snapshot.is_none() && self.db_snapshot.is_none(), "release the previous snapshot pair first");
        self.mark("snapshot.acquire.before");
        self.model_snapshot = Some(self.model.get_snapshot());
        let pointer = self.fixture.dbfull();
        assert!(!pointer.is_null(), "database missing during snapshot acquisition");
        self.db_snapshot = Some(unsafe { (&mut *pointer).get_snapshot() });
        self.saved_expected = Some(self.expected.clone());
        self.counts.snapshots += 1;
        self.mark("snapshot.acquire.after");
        self.check();
    }

    fn release_inner(&mut self) {
        if self.model_snapshot.is_some() || self.db_snapshot.is_some() {
            self.mark("snapshot.release.before");
        }
        if let Some(snapshot) = self.model_snapshot.take() {
            self.model.release_snapshot(snapshot);
        }
        if let Some(snapshot) = self.db_snapshot.take() {
            let pointer = self.fixture.dbfull();
            if pointer.is_null() {
                // Do not free a potentially linked node through ordinary Box drop.
                // Terminate this contained process instead of dereferencing a missing DB.
                eprintln!("BOUNDED_RANDOMIZED_FATAL snapshot cleanup has no live database");
                std::process::abort();
            }
            unsafe { (&mut *pointer).release_snapshot(snapshot); }
        }
        self.saved_expected = None;
    }

    fn release(&mut self) {
        assert!(self.model_snapshot.is_some() && self.db_snapshot.is_some(), "release requires a complete snapshot pair");
        self.release_inner();
        self.counts.releases += 1;
        self.mark("snapshot.release.after");
    }

    fn reopen(&mut self) {
        assert!(self.model_snapshot.is_none() && self.db_snapshot.is_none(), "snapshots must be released before reopen");
        self.mark("reopen.before");
        self.fixture.reopen(None);
        self.counts.reopens += 1;
        self.mark("reopen.after");
        self.check();
    }

    fn flush(&mut self) {
        assert!(self.db_snapshot.is_some(), "flush case must retain a saved snapshot");
        self.mark("flush.before");
        assert!(dbtest_fixture_test_compact_memtable_status(&mut self.fixture).is_ok(), "forced memtable flush failed");
        self.counts.flushes += 1;
        self.mark("flush.after");
        self.check();
    }
}

fn run_case(config: usize) {
    let keys = ["".to_owned(), "a".to_owned(), "\0".to_owned(), "batch".to_owned(),
                "c".to_owned(), "d".to_owned(), "x".repeat(200), "κλειδί".to_owned()];
    assert_eq!(keys.len(), KEY_LIMIT);
    let mut case = Case::new(config);
    case.put("a", "old");
    case.put(&keys[6], "long-old");
    case.save();
    case.put("a", "new");
    case.put("", "");
    case.put("\0", "second");
    case.put(&keys[7], "unicode-key");
    case.delete(&keys[6]);
    case.delete("never-inserted");
    case.batch(&[]);
    case.batch(&[Update::Put("a", "temporary"), Update::Delete("a"),
                 Update::Put("a", "final"), Update::Put("batch", "batch-value")]);
    case.flush();
    case.release();
    case.reopen();
    case.save();
    let mut random = Random::new(301);
    for index in 0..SEEDED_STEPS {
        let key_index = random.uniform(KEY_LIMIT as i32) as usize;
        let other_index = random.uniform(KEY_LIMIT as i32) as usize;
        let value = format!("seed301-{}-{}", index, random.uniform(1000));
        match random.uniform(3) {
            0 => case.put(&keys[key_index], &value),
            1 => case.delete(&keys[key_index]),
            _ => case.batch(&[Update::Put(&keys[key_index], "intermediate"),
                             Update::Delete(&keys[other_index]), Update::Put(&keys[key_index], &value)]),
        }
        if index == 11 {
            case.release();
            case.reopen();
            case.save();
        }
    }
    case.check();
    case.release();
    assert!(case.counts.puts > 0 && case.counts.deletes > 0 && case.counts.batches > 0);
    assert_eq!(case.counts.empty_batches, 1);
    assert!(case.counts.repeated_key_batches > 0 && case.counts.snapshot_after_mutation_checks > 0);
    assert_eq!(case.counts.snapshots, 3);
    assert_eq!(case.counts.releases, 3);
    assert_eq!(case.counts.reopens, 2);
    assert_eq!(case.counts.flushes, 1);
    assert_eq!(case.step, 10 + SEEDED_STEPS);
    let steps = case.step;
    case.mark("cleanup.before");
    drop(case); // A blocked destructor is a timed-out case, never a successful marker.
    eprintln!("BOUNDED_RANDOMIZED_OK config={} steps={}", config, steps);
}

#[test]
fn bounded_default() { run_case(0); }
#[test]
fn bounded_reuse() { run_case(1); }
#[test]
fn bounded_filter() { run_case(2); }
#[test]
fn bounded_uncompressed() { run_case(3); }
