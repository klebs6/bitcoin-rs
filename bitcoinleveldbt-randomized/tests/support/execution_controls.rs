//! Admission and bounded evidence for the two original ignored stress tests.
//! The operation generator, mutation order and historical comparison cadence
//! remain separate and unchanged. This helper constructs no database itself.

use std::path::{Component, Path};
use bitcoinleveldb_dbinterface::{DB, DBGetSnapshot, DBReleaseSnapshot};
use bitcoinleveldb_snapshot::Snapshot;

pub fn stress_limits_valid(memory: &str, pids: &str, swap_limit_text: &str, oom: &str, cpu: &str) -> bool {
    let memory = match memory.parse::<u64>() { Ok(value) => value, Err(_) => return false };
    let pids = match pids.parse::<u64>() { Ok(value) => value, Err(_) => return false };
    let fields: Vec<&str> = cpu.split_whitespace().collect();
    if fields.len() != 2 { return false; }
    let quota = match fields[0].parse::<u64>() { Ok(value) => value, Err(_) => return false };
    let period = match fields[1].parse::<u64>() { Ok(value) => value, Err(_) => return false };
    memory > 0 && memory <= 2 * 1024 * 1024 * 1024
        && pids > 0 && pids <= 64 && swap_limit_text == "0" && oom == "1"
        && quota > 0 && period > 0 && quota <= period
}

pub fn stress_policy_valid(logging: &str, seed: &str, data: &str,
                           test_tmpdir: &str, tmpdir: &str, tmp: &str, temp: &str,
                           case: &str) -> bool {
    if logging != "off" || seed != "301"
        || !matches!(case, "db_test_randomized" | "db_test_bisect_randomized")
        || data != test_tmpdir || data != tmpdir || data != tmp || data != temp {
        return false;
    }
    let path = Path::new(data);
    path.is_absolute()
        && path.components().all(|part| matches!(part, Component::RootDir | Component::Normal(_)))
        && path.file_name().and_then(|name| name.to_str()) == Some(format!("case-data-{}", case).as_str())
        && path.parent().is_some_and(|parent| parent != Path::new("/"))
}

fn read_control(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(value) => value.trim().to_owned(),
        Err(error) => panic!("use the protected randomized runner: cannot read {}: {}", path.display(), error),
    }
}

fn variable(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}

pub fn require_stress_protection(case: &str) {
    // This must precede DBTest construction, including fixture-driven creation.
    assert_eq!(variable("BITCOIN_RS_RANDOMIZED_STRESS_GUARD"), "v1",
               "use the protected randomized runner; a verified stress service is required");
    let data = variable("BITCOIN_RS_RANDOMIZED_CASE_DATA");
    assert!(stress_policy_valid(&variable("RUST_LOG"), &variable("TEST_RANDOM_SEED"), &data,
                                &variable("TEST_TMPDIR"), &variable("TMPDIR"),
                                &variable("TMP"), &variable("TEMP"), case),
            "use the protected randomized runner: seed 301, quiet logging and one private data root are required");
    let data_path = Path::new(&data);
    for component in data_path.ancestors() {
        let metadata = std::fs::symlink_metadata(component).expect("private data ancestor must exist");
        assert!(metadata.is_dir() && !metadata.file_type().is_symlink(),
                "private data path must contain only ordinary directories");
    }
    #[cfg(unix)] {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(std::fs::metadata(data_path).expect("private data root must exist").permissions().mode() & 0o777,
                   0o700, "private data root must have mode 0700");
    }
    #[cfg(not(unix))]
    panic!("protected randomized stress admission requires Unix directory permissions");

    let memberships = read_control(Path::new("/proc/self/cgroup"));
    let roots: Vec<&str> = memberships.lines().filter_map(|line| line.strip_prefix("0::")).collect();
    assert_eq!(roots.len(), 1, "exactly one unified cgroup membership is required");
    let membership = Path::new(roots[0]);
    assert!(membership.is_absolute() && membership.components().all(|part|
            matches!(part, Component::RootDir | Component::Normal(_))),
            "invalid unified cgroup path");
    let group = Path::new("/sys/fs/cgroup").join(roots[0].trim_start_matches('/'));
    let memory = read_control(&group.join("memory.max"));
    let pids = read_control(&group.join("pids.max"));
    let swap_limit_text = read_control(&group.join("memory.swap.max"));
    let oom = read_control(&group.join("memory.oom.group"));
    let cpu = read_control(&group.join("cpu.max"));
    assert!(stress_limits_valid(&memory, &pids, &swap_limit_text, &oom, &cpu),
            "use the protected randomized runner: memory<=2GiB,pids<=64,swap=0,grouped OOM and CPU<=1 required");
    eprintln!("\nRANDOMIZED_STRESS_GUARD case={} cgroup={} memory={} pids={} cpu={}",
              case, group.display(), memory, pids, cpu);
}

pub fn progress(case: &str, config: usize, done: usize, total: usize, phase: &str) {
    eprintln!("RANDOMIZED_STRESS_PROGRESS case={} config={} phase={} done={} total={}",
              case, config, phase, done, total);
}

pub fn complete(case: &str, configurations: usize, steps: usize) {
    eprintln!("\nRANDOMIZED_STRESS_OK case={} configurations={} steps={} seed=301",
              case, configurations, steps);
}

/// A held database snapshot must be unlinked through its owning live database.
///
/// The fixture and its heap database must remain alive and stable while this
/// lease is nonempty. Declare the lease after the fixture/model, empty it before
/// reopen, and acquire again from the new database pointer only after reopen.
/// Existing explicit release paths use take(); Drop covers assertion unwinding.
pub struct DatabaseSnapshotLease {
    owner: Option<*mut dyn DB>,
    snapshot: Option<Box<dyn Snapshot>>,
}

impl DatabaseSnapshotLease {
    pub fn new() -> Self { Self { owner: None, snapshot: None } }

    pub fn as_deref(&self) -> Option<&dyn Snapshot> { self.snapshot.as_deref() }

    pub fn take(&mut self) -> Option<Box<dyn Snapshot>> {
        self.owner = None;
        self.snapshot.take()
    }

    /// Safety: owner is a live, stable database until this lease is emptied.
    pub unsafe fn acquire(&mut self, owner: *mut dyn DB) {
        // Check before GetSnapshot: a newly linked snapshot must not be dropped
        // as the argument to an assertion or a rejected insertion.
        assert!(self.owner.is_none() && self.snapshot.is_none(), "release the previous snapshot before acquire");
        assert!(!owner.is_null(), "snapshot acquisition requires a live database");
        let snapshot = unsafe { DBGetSnapshot::get_snapshot(&mut *owner) };
        self.owner = Some(owner);
        self.snapshot = Some(snapshot);
    }
}

impl Drop for DatabaseSnapshotLease {
    fn drop(&mut self) {
        if let Some(snapshot) = self.snapshot.take() {
            match self.owner.take() {
                Some(owner) => unsafe { DBReleaseSnapshot::release_snapshot(&mut *owner, snapshot); },
                None => {
                    // A broken owner invariant must not ordinarily drop a node
                    // that could still be linked into a database snapshot list.
                    std::mem::forget(snapshot);
                    eprintln!("RANDOMIZED_STRESS_FATAL snapshot lease has no live owner");
                    std::process::abort();
                }
            }
        }
    }
}
