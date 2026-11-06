#!/usr/bin/env python3
"""Bounded issue178 diagnosis: explicit small probe and original full workload."""
import argparse
import datetime as dt
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import stat
import subprocess
import sys
import threading
import time
import uuid

WORKFLOW = "leveldb-issue178-check001"
LOCK_WORKFLOW = "leveldb-resource-tests001"
CONTROLLER_PATH = "contrib/test/issue178_resource_guard.py"
CONTROLLER_SHA256 = "b823684a678ee02ea7390ebf31ab92c458ff3cd4eaa64d79908a12c420fbb9cc"
SHELL_SHA256 = "8f316c4f007426302e26962e13def8c7d5bc0efa85d5e1549c55e85ba0d7c651"
CRATE = "bitcoinleveldbt-issue178"
UNIT_TARGET = "bitcoinleveldbt_issue178"
CASE_SPECS = [("issue178_bounded_diagnostic", "diagnostic"), ("issue178_test", "stress")]
BUILD = ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", CRATE, "--lib", "--no-run", "--message-format=json"]
RUSTC_VV = """rustc 1.97.0-nightly (9eb3be26b 2026-05-18)
binary: rustc
commit-hash: 9eb3be26b46eccea1de7448ea9cc3c1d20bb1a35
commit-date: 2026-05-18
host: x86_64-unknown-linux-gnu
release: 1.97.0-nightly
LLVM version: 22.1.4"""
CARGO_VERSION = "cargo 1.97.0-nightly (4d1f98451 2026-05-15)"
GIT_SELECTION = {"GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR", "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_PREFIX", "GIT_NAMESPACE"}
MIB = 1024 * 1024
CASE_BUDGETS = {
    "diagnostic": dict(memory=512*MIB, high=384*MIB, tasks=64, seconds=30, file=16*MIB, output=MIB, data=128*MIB, entries=4096),
    "stress": dict(memory=2048*MIB, high=1536*MIB, tasks=64, seconds=600, file=256*MIB, output=MIB, data=512*MIB, entries=4096),
}
PROGRESS_PREFIX = b"ISSUE178_PROGRESS "
PROGRESS_LIMIT = 64 * 1024
PROGRESS_RECORD_LIMIT = 256


def encoded(value):
    return (json.dumps(value, indent=2, sort_keys=True) + "\n").encode()


def atomic_json(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(encoded(value)); stream.flush(); os.fsync(stream.fileno())
        os.replace(temporary, path)
    finally:
        if temporary.exists():
            temporary.unlink()


def fingerprint(path):
    path = Path(path)
    info = path.lstat()
    if not stat.S_ISREG(info.st_mode):
        raise RuntimeError("Expected an ordinary file: " + str(path))
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(65536), b""):
            digest.update(chunk)
    return {"kind": "file", "mode": stat.S_IMODE(info.st_mode), "size": info.st_size, "sha256": digest.hexdigest()}


def ordinary_path(path, root):
    path, root = Path(path), Path(root)
    if not path.is_relative_to(root):
        raise RuntimeError("Expected a path inside the selected repository: " + str(path))
    for part in [path, *path.parents]:
        if part == root:
            break
        if part.is_symlink():
            raise RuntimeError("Expected an ordinary repository path: " + str(path))
    return path


def repository_dirs(repo):
    """Read standard Git worktree metadata without spawning an unguarded child."""
    dotgit = repo / ".git"
    if dotgit.is_symlink():
        raise RuntimeError("The selected .git path is a symlink.")
    if dotgit.is_dir():
        gitdir = dotgit.resolve()
    elif dotgit.is_file() and dotgit.stat().st_size < 16384:
        text = dotgit.read_text().strip()
        if not text.startswith("gitdir: "):
            raise RuntimeError("Unsupported Git worktree metadata.")
        gitdir = (repo / text[8:]).resolve()
    else:
        raise RuntimeError("--repo must name a Git checkout root.")
    common_file = gitdir / "commondir"
    if common_file.exists():
        if common_file.is_symlink() or common_file.stat().st_size >= 16384:
            raise RuntimeError("Unsupported common Git directory metadata.")
        common = (gitdir / common_file.read_text().strip()).resolve()
    else:
        common = gitdir
    if not (gitdir / "HEAD").is_file() or not (common / "objects").is_dir():
        raise RuntimeError("Incomplete Git checkout metadata.")
    return gitdir, common


def checked_environment():
    env = dict(os.environ)
    cleared = sorted(key for key in GIT_SELECTION if env.pop(key, None) is not None)
    bad = [key for key, value in env.items() if value and (key in {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_TARGET", "CARGO_BUILD_RUSTFLAGS", "CARGO_BUILD_BUILD_DIR", "CARGO_BUILD_RUSTC", "CARGO_BUILD_RUSTC_WRAPPER", "RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER"} or key.startswith("CARGO_PROFILE_") or (key.startswith("CARGO_TARGET_") and key.endswith(("_RUSTFLAGS", "_RUSTC_WRAPPER"))))]
    if bad:
        raise RuntimeError("Unexpected Rust/build overrides require review: " + repr(sorted(bad)))
    env["GIT_OPTIONAL_LOCKS"] = "0"
    return env, cleared


def load_controller(repo):
    path = ordinary_path(repo / CONTROLLER_PATH, repo)
    if fingerprint(path)["sha256"] != CONTROLLER_SHA256:
        raise RuntimeError("The reviewed resource controller differs from its accepted SHA256.")
    if fingerprint(ordinary_path(repo / "shell.nix", repo))["sha256"] != SHELL_SHA256:
        raise RuntimeError("The selected shell.nix differs from the accepted pinned native environment.")
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("issue178_resource_guard001", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module, path


def capture_inputs(repo, output):
    """Internal probe. Its Git children and hashing run inside the probe service."""
    env, _ = checked_environment()
    def git(*args):
        result = subprocess.run(["git", "-c", "core.fsmonitor=false", *args], cwd=repo, env=env, capture_output=True, timeout=5)
        if result.returncode or len(result.stdout) > 4*1024*1024 or len(result.stderr) > 65536:
            raise RuntimeError("Guarded Git metadata query failed or exceeded its output allowance.")
        return result.stdout
    if Path(os.fsdecode(git("rev-parse", "--show-toplevel")).strip()).resolve() != repo:
        raise RuntimeError("--repo must identify the checkout root.")
    gitdir, common = repository_dirs(repo)
    names = {os.fsdecode(name) for name in git("ls-files", "-z", "--cached", "--others", "--exclude-standard").split(b"\0") if name}
    if len(names) > 100000:
        raise RuntimeError("Input inventory exceeds the finite metadata allowance.")
    selected = {name for name in names if name.endswith((".rs", ".nix")) or Path(name).name in {"Cargo.toml", "Cargo.lock", "rust-toolchain", "rust-toolchain.toml"} or ".cargo" in Path(name).parts}
    selected.update({CONTROLLER_PATH, "contrib/test/leveldb-issue178.py", "shell.nix"})
    inputs = {}
    total = 0
    for name in sorted(selected):
        path = ordinary_path(repo / name, repo)
        if path.stat().st_size > 16*1024*1024:
            raise RuntimeError("A build input exceeds the finite inventory allowance: " + name)
        total += path.stat().st_size
        if total > 256*1024*1024:
            raise RuntimeError("Build input inventory exceeds 256 MiB.")
        inputs[name] = fingerprint(path)
    configurations = {}
    homes = [Path(env.get("CARGO_HOME", Path.home() / ".cargo"))]
    homes.extend(parent / ".cargo" for parent in [repo, *repo.parents])
    for home in homes:
        for name in ["config", "config.toml"]:
            path = home / name
            if path.exists() or path.is_symlink():
                configurations[str(path)] = fingerprint(path)
    result = {"head": git("rev-parse", "HEAD").decode().strip(), "inputs": inputs, "cargo_configurations": configurations,
              "index": fingerprint(gitdir / "index") if (gitdir / "index").exists() else None,
              "git_directory": str(gitdir), "common_directory": str(common)}
    atomic_json(output, result)
    print("ISSUE178_INPUTS_RECORDED", flush=True)


def _load_json(path):
    path = Path(path)
    if path.is_symlink() or not path.is_file() or path.stat().st_size > 8*1024*1024:
        raise RuntimeError("Expected capped ordinary JSON evidence: " + str(path))
    return json.loads(path.read_text())


def log_text(result):
    path = Path(result["log"])
    if path.is_symlink() or not path.is_file() or path.stat().st_size > result["budget"]["output"]:
        raise RuntimeError("Guard log is not an ordinary file within its output allowance.")
    return path.read_bytes().decode(errors="replace")


def compact(result):
    return {key: value for key, value in result.items() if key not in {"tail", "command"}} | {"tail": result.get("tail", "")[-2400:]}


def require_run(result, allowed_exits=(0,)):
    if result.get("outcome") != "exited" or result.get("verified") is not True or result.get("exit_code") not in allowed_exits:
        raise RuntimeError("Resource-controlled phase failed: " + str(result.get("label")) + ": "
                           + str(result.get("outcome")) + ", exit=" + str(result.get("exit_code")))


def configure_budget(controller, profile):
    """The parent and internal guard child install the same named envelope."""
    if profile not in CASE_BUDGETS:
        raise RuntimeError("Unknown issue178 execution profile: " + str(profile))
    controller.BUDGETS["case"] = dict(CASE_BUDGETS[profile])


def validate_target(target, common):
    """Generated Rust lives in common Git metadata, outside every source tree."""
    root = common / "codex-repairs"
    if not target.is_relative_to(root) or target == root:
        raise RuntimeError("The issue178 cache must be below COMMON_GIT_DIR/codex-repairs.")
    for path in [target, *target.parents]:
        if path == common:
            break
        if path.is_symlink():
            raise RuntimeError("The issue178 cache path contains a symlink.")
        if path.exists() and not path.is_dir():
            raise RuntimeError("The issue178 cache path contains a non-directory.")
    target.mkdir(parents=True, exist_ok=True)


def artifact_from(result, target_dir, repo):
    found = None
    for line in log_text(result).splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        if not isinstance(item, dict):
            continue
        target = item.get("target", {})
        if item.get("reason") != "compiler-artifact" or target.get("name") != UNIT_TARGET or not item.get("executable"):
            continue
        if ("lib" not in target.get("kind", []) or item.get("profile", {}).get("test") is not True
                or Path(target.get("src_path", "")).resolve() != (repo / CRATE / "src/lib.rs").resolve()):
            raise RuntimeError("Cargo returned an unexpected issue178 test artifact.")
        path = Path(item["executable"])
        if not path.is_absolute() or not path.is_relative_to(target_dir):
            raise RuntimeError("The issue178 executable is outside the selected cache.")
        for part in [path, *path.parents]:
            if part == target_dir:
                break
            if part.is_symlink():
                raise RuntimeError("The issue178 executable path contains a symlink.")
        record = {"path": str(path), "fingerprint": fingerprint(path)}
        if not record["fingerprint"]["mode"] & 0o111:
            raise RuntimeError("The issue178 artifact is not executable.")
        if found is not None and found != record:
            raise RuntimeError("Cargo identified conflicting issue178 test executables.")
        found = record
    if found is None:
        raise RuntimeError("Capped Cargo output did not identify the issue178 unit-test executable.")
    return found


def bound_names(text, selected_names):
    listed = [match[1] for line in text.splitlines() if (match := re.match(r"^(.+): test$", line))]
    if len(listed) != len(set(listed)):
        raise RuntimeError("Duplicated issue178 test metadata.")
    bound = {}
    for name in selected_names:
        matches = [full for full in listed if full.rsplit("::", 1)[-1] == name]
        if len(matches) != 1:
            raise RuntimeError("Required issue178 test metadata is missing or ambiguous: " + name)
        bound[name] = matches[0]
    return listed, bound


def completion_summary(text, name):
    rows = [(match[1], *map(int, match.groups()[1:])) for line in text.splitlines()
            if (match := re.match(r"^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;", line))]
    marker = "ISSUE178_CONTRACT_OK case=" + name
    if rows != [("ok", 1, 0, 0)] or text.splitlines().count(marker) != 1:
        raise RuntimeError("The issue178 case lacks one passing test and its exact completion marker: " + name)
    return {"passed": 1, "failed": 0, "ignored": 0, "marker": marker}


def execute_test(path, name, progress_path):
    """Stream output unchanged; retain only capped, explicit progress records."""
    progress_path = Path(progress_path)
    fd = os.open(progress_path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    child = None
    pending = bytearray()
    written = records = 0
    try:
        child = subprocess.Popen([str(path), "--exact", name, "--nocapture", "--test-threads=1"],
                                 stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, bufsize=0)
        while True:
            raw = os.read(child.stdout.fileno(), 65536)
            if not raw:
                break
            view = memoryview(raw)
            while view:
                view = view[os.write(1, view):]
            pending.extend(raw)
            while b"\n" in pending:
                line, _, tail = pending.partition(b"\n")
                pending = bytearray(tail)
                if line.startswith(PROGRESS_PREFIX):
                    if len(line) > 1024 or records >= PROGRESS_RECORD_LIMIT or written + len(line) + 1 > PROGRESS_LIMIT:
                        raise RuntimeError("Issue178 progress records exceeded their explicit allowance.")
                    record = bytes(line) + b"\n"
                    os.write(fd, record)
                    written += len(record)
                    records += 1
            if len(pending) > 65536:
                # Preserve stdout forwarding, but never accumulate an unbounded line.
                pending.clear()
        return_code = child.wait()
        return return_code if return_code >= 0 else 128 - return_code
    finally:
        os.close(fd)
        if child:
            if child.poll() is None:
                child.terminate()
                try:
                    child.wait(timeout=0.5)
                except subprocess.TimeoutExpired:
                    child.kill()
                    child.wait(timeout=0.5)
            child.stdout.close()


def progress_records(path, complete=True):
    path = Path(path)
    if not path.exists():
        return []
    if path.is_symlink() or not path.is_file() or path.stat().st_size > PROGRESS_LIMIT:
        raise RuntimeError("Expected an ordinary capped issue178 progress file.")
    content = path.read_bytes()
    if content and not content.endswith(b"\n"):
        if complete:
            raise RuntimeError("The final issue178 progress record is incomplete.")
        content = content.rpartition(b"\n")[0]
    lines = content.splitlines()
    if len(lines) > PROGRESS_RECORD_LIMIT or any(not line.startswith(PROGRESS_PREFIX) or len(line) > 1024 for line in lines):
        raise RuntimeError("Invalid issue178 progress records.")
    return [line.decode(errors="replace") for line in lines]


class ProgressDisplay:
    def __init__(self, path):
        self.path = Path(path)
        self.stop = threading.Event()
        self.thread = threading.Thread(target=self._run, daemon=True)

    def _run(self):
        shown = 0
        while not self.stop.wait(0.1):
            try:
                rows = progress_records(self.path, complete=False)
            except (OSError, RuntimeError):
                continue
            for row in rows[shown:]:
                print(row, flush=True)
            shown = len(rows)

    def __enter__(self):
        self.thread.start()
        return self

    def __exit__(self, *_):
        self.stop.set()
        self.thread.join(timeout=1)


def assess_case(result, name, text):
    record = {"passed": 0, "failed": 0, "ignored": 0, "guard_outcome": result.get("outcome"),
              "exit_code": result.get("exit_code"), "last_progress": text.splitlines()[-16:]}
    if result.get("verified") is not True:
        return "failed", record | {"reason": "resource controls were not verified"}
    if result.get("outcome") == "exited":
        if result.get("exit_code") == 0:
            return "passed", record | completion_summary(text, name)
        return "failed", record | {"failed": 1, "reason": "native test process exited unsuccessfully"}
    if result.get("outcome") in {"memory_limit", "data_limit", "output_limit", "disk_free_limit", "parent_timeout", "service_timeout", "interrupted"}:
        return "inconclusive", record | {"reason": "bounded observation ended before the case completed"}
    return "failed", record | {"reason": "guard or execution setup failed"}


def verify(repo, target_dir, directory, before_phase=None, selected_names=None):
    """Run fresh cases; a limit stop is diagnostic evidence, never a repair pass."""
    repo, directory = map(lambda p: Path(p).expanduser().resolve(), (repo, directory))
    target_dir = Path(os.path.abspath(Path(target_dir).expanduser()))
    directory.mkdir(parents=True, exist_ok=False, mode=0o700)
    report_path = directory / "VERIFICATION_RETURN.json"
    specs = list(CASE_SPECS)
    if selected_names is not None:
        selected_names = list(selected_names)
        if len(selected_names) != len(set(selected_names)) or set(selected_names) - dict(CASE_SPECS).keys():
            raise RuntimeError("Unknown or duplicated issue178 case selection.")
        specs = [(name, profile) for name, profile in specs if name in selected_names]
    report = {"workflow": WORKFLOW, "outcome": "failed", "phase": "preflight", "repository": str(repo),
              "target_directory": str(target_dir), "run_directory": str(directory), "workspace_suite": "not run",
              "full_coverage_claimed": False, "validation_reused": False, "build_command": BUILD,
              "selected_cases": [name for name, _ in specs], "readouts": {}, "cases": {},
              "original_case_completed": False, "test_count": 0, "inputs_preserved": False,
              "original_workload": {"keys_per_range": 1100000, "put_operations": 2200000,
                                    "delete_operations": 1100000, "expected_live_keys": 1100000,
                                    "compaction_endpoints": "original key1(0) through key1(NUM_KEYS-1)"},
              "case_budgets": CASE_BUDGETS, "logging_policy": {"RUST_LOG": "off", "progress": "explicit stderr records"}}
    lock = guard = initial = None
    controller = None
    def snapshot(label):
        output = directory / (label + ".json")
        result = phase(label, [sys.executable, str(Path(__file__).resolve()), "--capture-inputs", str(repo), str(output)],
                       "probe", extra={"RUSTFLAGS": ""})
        if "ISSUE178_INPUTS_RECORDED" not in log_text(result):
            raise RuntimeError("Issue178 input snapshot completion evidence is absent.")
        return _load_json(output)
    def final_snapshot():
        if initial is None:
            return
        previous_phase = report["phase"]
        final = snapshot("inputs-after")
        report["phase"] = previous_phase
        report["inputs_after_sha256"] = hashlib.sha256(encoded(final)).hexdigest()
        report["inputs_preserved"] = final == initial
        if final != initial:
            raise RuntimeError("Repository/build inputs or Git index changed during issue178 verification.")
    try:
        if not specs:
            raise RuntimeError("No issue178 cases were selected.")
        controller, controller_path = load_controller(repo)
        env, report["cleared_git_selection_variables"] = checked_environment()
        _, common = repository_dirs(repo)
        metadata = common / "codex-repairs" / LOCK_WORKFLOW
        metadata.mkdir(parents=True, exist_ok=True)
        lock_path = metadata / "run.lock"
        if lock_path.is_symlink():
            raise RuntimeError("Verification lock is a symlink.")
        lock = lock_path.open("a+")
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        validate_target(target_dir, common)
        report.update(controller_sha256=CONTROLLER_SHA256, shell_sha256=SHELL_SHA256)
        guard = controller.Guard(directory / "guard", Path(__file__).resolve(), env)
        runtime_env = {"CARGO_TARGET_DIR": str(target_dir), "CARGO_BUILD_JOBS": "1", "NUM_JOBS": "1",
                       "CARGO_NET_OFFLINE": "true", "RUST_BACKTRACE": "1", "RUSTFLAGS": "-Awarnings",
                       "CARGO_NET_GIT_FETCH_WITH_CLI": "true", "RUST_LOG": "off", "BITCOIN_RS_ISSUE178_PROFILE": "diagnostic"}
        def phase(label, command, kind="case", extra=None, tmpdir=None, check=True, profile="diagnostic"):
            if before_phase:
                before_phase(label)
            configure_budget(controller, profile)
            report["phase"] = label
            atomic_json(report_path, report)
            print("Protected issue178 phase: " + label, flush=True)
            supplied = dict(runtime_env, **(extra or {}))
            supplied["BITCOIN_RS_ISSUE178_PROFILE"] = profile
            result = guard.run(label, command, repo, supplied, kind=kind, tmpdir=tmpdir)
            report["readouts"][label] = compact(result)
            atomic_json(report_path, report)
            if check:
                require_run(result)
            return result
        preflight = phase("preflight", [sys.executable, "-c", "print('GUARD_PREFLIGHT_OK')"], "probe")
        if "GUARD_PREFLIGHT_OK" not in log_text(preflight):
            raise RuntimeError("Guard preflight success marker is absent.")
        initial = snapshot("inputs-before")
        report["head"] = initial["head"]
        report["inputs_before_sha256"] = hashlib.sha256(encoded(initial)).hexdigest()
        nix = ["nix-shell", "--option", "max-jobs", "0", "--option", "builders", "", "--option", "substitute", "false",
               str(repo / "shell.nix"), "--run"]
        native = phase("native-tools", nix + ["rustc -Vv && cargo -V"], "build")
        if RUSTC_VV not in log_text(native) or CARGO_VERSION not in log_text(native).splitlines():
            raise RuntimeError("Rust/Cargo differs from the accepted native toolchain.")
        built = phase("compile-tests", nix + [shlex.join(BUILD)], "build")
        artifact = artifact_from(built, target_dir, repo)
        report["artifact"] = artifact
        path = Path(artifact["path"])
        if fingerprint(path) != artifact["fingerprint"]:
            raise RuntimeError("Compiled issue178 executable changed before metadata inspection.")
        listed = phase("list-issue178", [str(path), "--list"])
        inventory, names = bound_names(log_text(listed), [name for name, _ in specs])
        report["protected_test_list"] = inventory
        report["selected_test_bindings"] = names
        overall = "passed"
        for name, profile in specs:
            if fingerprint(path) != artifact["fingerprint"]:
                raise RuntimeError("Compiled issue178 executable changed before case execution.")
            progress_path = directory / (name + ".progress.log")
            command = [sys.executable, str(Path(__file__).resolve()), "--execute-test", str(path), names[name], str(progress_path)]
            with ProgressDisplay(progress_path):
                result = phase(name, command, tmpdir=directory / ("case-data-" + name), check=False, profile=profile)
            outcome, record = assess_case(result, name, log_text(result))
            rows = progress_records(progress_path)
            record.update(outcome=outcome, executed_name=names[name], target=UNIT_TARGET, profile=profile,
                          progress=rows, last_phase=rows[-1] if rows else None)
            report["cases"][name] = record
            if outcome == "passed":
                report["test_count"] += 1
                if name == "issue178_test":
                    report["original_case_completed"] = True
            else:
                overall = outcome
                report["reason"] = record["reason"]
                break
        if fingerprint(path) != artifact["fingerprint"]:
            raise RuntimeError("Compiled issue178 executable changed during verification.")
        final_snapshot()
        report.update(outcome=overall, phase="complete" if overall == "passed" else report["phase"],
                      passed=sum(case["passed"] for case in report["cases"].values()))
    except BaseException as error:
        failed_phase = report["phase"]
        report.update(outcome="failed", error=type(error).__name__ + ": " + str(error))
        if initial is not None and "inputs_after_sha256" not in report:
            try:
                final_snapshot()
            except BaseException as followup:
                report["input_recheck_error"] = type(followup).__name__ + ": " + str(followup)
        report["phase"] = failed_phase
    finally:
        atomic_json(report_path, report)
        if lock:
            lock.close()
    return report


def main():
    if len(sys.argv) == 3 and sys.argv[1] == "--guard-child":
        plan = _load_json(sys.argv[2])
        controller, _ = load_controller(Path(plan["cwd"]).resolve())
        configure_budget(controller, plan["env"].get("BITCOIN_RS_ISSUE178_PROFILE", "diagnostic"))
        return controller.guard_child(sys.argv[2])
    if len(sys.argv) == 4 and sys.argv[1] == "--capture-inputs":
        capture_inputs(Path(sys.argv[2]).resolve(), Path(sys.argv[3])); return 0
    if len(sys.argv) == 5 and sys.argv[1] == "--execute-test":
        return execute_test(Path(sys.argv[2]), sys.argv[3], Path(sys.argv[4]))
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--target-dir", type=Path, help="cache below COMMON_GIT_DIR/codex-repairs")
    parser.add_argument("--diagnostic-only", action="store_true", help="run the small diagnosis only; never claims original-case completion")
    args = parser.parse_args()
    directory = None
    try:
        repo = args.repo.expanduser().resolve()
        _, common = repository_dirs(repo)
        root = common / "codex-repairs" / WORKFLOW
        target = args.target_dir if args.target_dir else root / "cargo-target"
        directory = root / "runs" / (dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])
        report = verify(repo, target, directory, selected_names=["issue178_bounded_diagnostic"] if args.diagnostic_only else None)
    except BaseException as error:
        report = {"workflow": WORKFLOW, "outcome": "failed", "error": type(error).__name__ + ": " + str(error)}
    print("BEGIN_ISSUE178_CHECK_RETURN"); print(json.dumps(report, indent=2)); print("END_ISSUE178_CHECK_RETURN")
    if directory:
        print("Return file: " + str(directory / "VERIFICATION_RETURN.json"))
    return 0 if report.get("outcome") == "passed" else 1


if __name__ == "__main__":
    sys.exit(main())
