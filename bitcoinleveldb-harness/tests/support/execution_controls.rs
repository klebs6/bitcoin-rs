//! Standard-library-only admission for existing harness cases.
use std::path::{Component, Path};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile { Case, Stress }

impl Profile {
    pub fn limits(self) -> (u64, u64, u64) {
        match self {
            Self::Case => (512 * 1024 * 1024, 384 * 1024 * 1024, 16 * 1024 * 1024),
            Self::Stress => (2048 * 1024 * 1024, 1536 * 1024 * 1024, 256 * 1024 * 1024),
        }
    }
}

pub fn case_profile(name: &str) -> Option<Profile> {
    match name {
        "harness_randomized" | "harness_randomized_longdb" => Some(Profile::Stress),
        "harness_empty" | "harness_zero_restart_points_in_block" | "harness_simple_empty_key"
        | "harness_simple_single" | "harness_simple_multi" | "harness_simple_special_key"
        | "harness_db_pointer_is_null_when_not_db_test"
        | "harness_reinitialization_switches_constructor_types_without_panicking"
        | "harness_duplicate_keys_overwrite_previous_value" | "harness_handles_embedded_nul_and_empty_values"
        | "harness_pick_random_key_returns_foo_on_empty_keyset"
        | "harness_to_string_formats_iterator_entries_and_end_sentinel" => Some(Profile::Case),
        _ => None,
    }
}

#[derive(Clone, Copy)]
pub struct Controls<'a> {
    pub memory: &'a str, pub high: &'a str, pub pids: &'a str,
    pub swap_limit_text: &'a str, pub oom: &'a str, pub cpu: &'a str,
    pub core_limit: &'a str, pub file_limit: &'a str,
}

fn exact_limit(line: &str, expected: u64) -> bool {
    let fields: Vec<&str> = line.split_whitespace().collect();
    fields.len() == 3 && fields[2] == "bytes"
        && fields[0].parse::<u64>() == Ok(expected)
        && fields[1].parse::<u64>() == Ok(expected)
}

pub fn controls_valid(profile: Profile, values: Controls<'_>) -> bool {
    let (memory, high, file) = profile.limits();
    let fields: Vec<&str> = values.cpu.split_whitespace().collect();
    if fields.len() != 2 { return false; }
    let quota = match fields[0].parse::<u64>() { Ok(value) => value, Err(_) => return false };
    let period = match fields[1].parse::<u64>() { Ok(value) => value, Err(_) => return false };
    values.memory.parse::<u64>() == Ok(memory) && values.high.parse::<u64>() == Ok(high)
        && values.pids.parse::<u64>() == Ok(64) && values.swap_limit_text == "0" && values.oom == "1"
        && quota > 0 && period > 0 && quota <= period
        && exact_limit(values.core_limit, 0) && exact_limit(values.file_limit, file)
}

pub fn environment_valid(name: &str, profile: Profile, case_token: &str, protected: &str,
                         seed: &str, logging: &str, data: &str,
                         tmpdir: &str, tmp: &str, temp: &str, test_tmpdir: &str) -> bool {
    if case_profile(name) != Some(profile) || case_token != name || protected != "1"
        || seed != "301" || logging != "off" || data != tmpdir || data != tmp
        || data != temp || data != test_tmpdir { return false; }
    let path = Path::new(data);
    path.is_absolute() && path.components().all(|part| matches!(part, Component::RootDir | Component::Normal(_)))
        && path.file_name().and_then(|part| part.to_str()) == Some(format!("case-data-{}", name).as_str())
        && path.parent().is_some_and(|parent| parent != Path::new("/"))
}

fn variable(name: &str) -> String { std::env::var(name).unwrap_or_default() }
fn read_control(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error|
        panic!("use the protected harness031 runner: cannot read {}: {}", path.display(), error)).trim().to_owned()
}

pub fn require_case_protection(name: &str, profile: Profile) {
    let data = variable("BITCOIN_RS_HARNESS031_CASE_DATA");
    assert!(environment_valid(name, profile, &variable("BITCOIN_RS_HARNESS031_CASE"),
        &variable("BITCOIN_RS_HARNESS031_PROTECTED"), &variable("TEST_RANDOM_SEED"), &variable("RUST_LOG"),
        &data, &variable("TMPDIR"), &variable("TMP"), &variable("TEMP"), &variable("TEST_TMPDIR")),
        "use the protected harness031 runner: exact case/profile, seed301, quiet logging and private temporary roots required");
    let data_path = Path::new(&data);
    for ancestor in data_path.ancestors() {
        let metadata = std::fs::symlink_metadata(ancestor).expect("private temporary ancestor must exist");
        assert!(metadata.is_dir() && !metadata.file_type().is_symlink(), "private temporary path must contain ordinary directories");
    }
    #[cfg(unix)] {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(std::fs::metadata(data_path).expect("private temporary root must exist").permissions().mode() & 0o777,
                   0o700, "private temporary root must have mode0700");
    }
    #[cfg(not(unix))]
    panic!("protected harness031 admission requires Unix permission controls");
    let memberships = read_control(Path::new("/proc/self/cgroup"));
    let roots: Vec<&str> = memberships.lines().filter_map(|line| line.strip_prefix("0::")).collect();
    assert_eq!(roots.len(), 1, "exactly one unified cgroup membership required");
    let membership = Path::new(roots[0]);
    assert!(membership.is_absolute() && membership.components().all(|part|
        matches!(part, Component::RootDir | Component::Normal(_))), "invalid unified cgroup path");
    let group = Path::new("/sys/fs/cgroup").join(roots[0].trim_start_matches('/'));
    let memory = read_control(&group.join("memory.max"));
    let high = read_control(&group.join("memory.high"));
    let pids = read_control(&group.join("pids.max"));
    let swap_limit_text = read_control(&group.join("memory.swap.max"));
    let oom = read_control(&group.join("memory.oom.group"));
    let cpu = read_control(&group.join("cpu.max"));
    let limits = read_control(Path::new("/proc/self/limits"));
    let core_limit = limits.lines().find_map(|line| line.strip_prefix("Max core file size")).unwrap_or("").trim();
    let file_limit = limits.lines().find_map(|line| line.strip_prefix("Max file size")).unwrap_or("").trim();
    assert!(controls_valid(profile, Controls { memory: &memory, high: &high, pids: &pids,
        swap_limit_text: &swap_limit_text, oom: &oom, cpu: &cpu, core_limit, file_limit }),
        "use the protected harness031 runner: exact profile memory/high/tasks/file limits, CPU<=1, swap0 and grouped OOM required");
    // RuntimeMax and timeout cleanup are independently verified by the controller.
    eprintln!("\nHARNESS_GUARD case={} profile={:?} cgroup={} memory={} high={} pids={} cpu={}",
              name, profile, group.display(), memory, high, pids, cpu);
}

pub fn phase(name: &str, phase: &str) { eprintln!("\nHARNESS_PHASE case={} phase={}", name, phase); }
pub fn progress(name: &str, phase: &str, configuration: usize, done: usize, total: usize) {
    eprintln!("HARNESS_PROGRESS case={} phase={} config={} done={} total={}", name, phase, configuration, done, total);
}
pub fn complete(name: &str) { eprintln!("\nHARNESS_CONTRACT_OK case={}", name); }
