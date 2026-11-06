"""Register only the dedicated comparator build cache in local Git metadata."""
import hashlib
import os
from pathlib import Path
import stat
import uuid


def _cache_write(path, data, mode):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, mode)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(data); stream.flush(); os.fsync(stream.fileno())
        os.chmod(temporary, mode)
        os.replace(temporary, path)
    finally:
        if temporary.exists():
            temporary.unlink()


def prepare_comparator_cache(owner, common, cache, evidence, git_call):
    """Preserve all existing rules and prove the one owned cache is ignored.

    git_call returns a CompletedProcess, uses owner as cwd, and does not raise
    for a nonzero status. No tracked paths or source/index bytes are changed.
    """
    owner, common, cache, evidence = map(Path, (owner, common, cache, evidence))
    if cache != owner / "target-comparator":
        if cache.is_relative_to(owner):
            raise RuntimeError("Use target-comparator or an external path for the comparator cache.")
        return {"cache": str(cache), "changed": False, "reason": "external cache"}
    if cache.is_symlink() or (cache.exists() and not cache.is_dir()):
        raise RuntimeError("Comparator cache must be an ordinary directory.")
    tracked = git_call("ls-files", "-z", "--", "target-comparator")
    if tracked.returncode or tracked.stdout:
        raise RuntimeError("Comparator cache contains tracked paths or could not be inspected.")
    exclude = common / "info" / "exclude"
    if exclude.parent.is_symlink() or exclude.is_symlink():
        raise RuntimeError("Git cache exclusion metadata is a symlink; retained.")
    if exclude.exists() and (not exclude.is_file() or exclude.stat().st_size > 1024 * 1024):
        raise RuntimeError("Git cache exclusion metadata is not a capped ordinary file.")
    before = exclude.read_bytes() if exclude.exists() else None
    mode = stat.S_IMODE(exclude.stat().st_mode) if before is not None else 0o644
    ignored = git_call("check-ignore", "--quiet", "--no-index", "--", "target-comparator/")
    if ignored.returncode not in (0, 1):
        raise RuntimeError("Could not verify comparator cache exclusion.")
    result = {"cache": str(cache), "exclude": str(exclude), "changed": False,
              "rule": "/target-comparator/", "mode": mode,
              "before_sha256": hashlib.sha256(before).hexdigest() if before is not None else None}
    if ignored.returncode == 0:
        result["after_sha256"] = result["before_sha256"]
        return result
    evidence.mkdir(parents=True, exist_ok=True)
    backup = evidence / "git-info-exclude.before"
    if backup.is_symlink() or backup.exists():
        raise RuntimeError("Cache exclusion backup path is already occupied.")
    _cache_write(backup, before if before is not None else b"", 0o600)
    after = (before or b"")
    if after and not after.endswith(b"\n"):
        after += b"\n"
    after += b"/target-comparator/\n"
    if (exclude.read_bytes() if exclude.exists() else None) != before:
        raise RuntimeError("Git exclusion rules changed concurrently; retained.")
    _cache_write(exclude, after, mode)
    checked = git_call("check-ignore", "--quiet", "--no-index", "--", "target-comparator/")
    if checked.returncode != 0:
        raise RuntimeError("Exact comparator cache rule did not take effect; backup retained.")
    result.update(changed=True, backup=str(backup), before_existed=before is not None,
                  after_sha256=hashlib.sha256(after).hexdigest())
    return result

#!/usr/bin/env python3
"""Fresh, resource-controlled comparator regression checks."""
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
import uuid

WORKFLOW = "leveldb-comparator-check001"
LOCK_WORKFLOW = "leveldb-resource-tests001"
CONTROLLER_PATH = "contrib/test/resource_guard.py"
CONTROLLER_SHA256 = "7de80b8c219e9ae62473aa4e7a9727578c992fd6472c67599f4e87fe7d0b61db"
SHELL_SHA256 = "8f316c4f007426302e26962e13def8c7d5bc0efa85d5e1549c55e85ba0d7c651"
CRATE = "bitcoinleveldb-comparator"
UNIT_TARGET = "bitcoinleveldb_comparator"
TARGETS = {UNIT_TARGET: ("lib", "src/lib.rs"),
           "comparator": ("test", "tests/comparator.rs"),
           "mock_bytewise_contracts": ("test", "tests/mock_bytewise_contracts.rs")}
UNIT_NAMES = ["borrowed_slice_comparator_adapter_box_returns_none_for_null_pointer",
              "borrowed_slice_comparator_adapter_box_forwards_full_slice_comparator_surface"]
CONTRACT_CASES = {name: "COMPARATOR_CONTRACT_OK case=" + name for name in (
    "mock_bytewise_pointer_contract", "corrupt_mock_bytewise_pointer_contract")}
TESTS = {UNIT_TARGET: UNIT_NAMES,
         "comparator": ["test_compare_basic", "test_find_shortest_separator", "test_find_short_successor", "test_bytewise_comparator_singleton"],
         "mock_bytewise_contracts": list(CONTRACT_CASES)}
CASES = [(target, name) for target, names in TESTS.items() for name in names]
FAILURE_FILES = {"mock_bytewise_pointer_contract": "mock_comparator.rs",
                 "corrupt_mock_bytewise_pointer_contract": "mock_corrupt_comparator.rs"}
BUILD = ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", CRATE, "--lib", "--tests", "--no-run", "--message-format=json"]
RUSTC_VV = """rustc 1.97.0-nightly (9eb3be26b 2026-05-18)
binary: rustc
commit-hash: 9eb3be26b46eccea1de7448ea9cc3c1d20bb1a35
commit-date: 2026-05-18
host: x86_64-unknown-linux-gnu
release: 1.97.0-nightly
LLVM version: 22.1.4"""
CARGO_VERSION = "cargo 1.97.0-nightly (4d1f98451 2026-05-15)"
GIT_SELECTION = {"GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR", "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_PREFIX", "GIT_NAMESPACE"}


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
    spec = importlib.util.spec_from_file_location("comparator_resource_guard001", path)
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
    selected.update({CONTROLLER_PATH, "contrib/test/leveldb-comparator.py", "shell.nix"})
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
    print("COMPARATOR_INPUTS_RECORDED", flush=True)


def _load_json(path):
    path = Path(path)
    if path.is_symlink() or not path.is_file() or path.stat().st_size > 8*1024*1024:
        raise RuntimeError("Expected capped ordinary JSON evidence: " + str(path))
    return json.loads(path.read_text())



def artifacts_from(controller, result, target_dir, repo):
    found = {}
    expected = set(TARGETS)
    for line in log_text(result).splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        if not isinstance(item, dict):
            continue
        target = item.get("target", {})
        name = target.get("name")
        if item.get("reason") != "compiler-artifact" or name not in expected or not item.get("executable"):
            continue
        required_kind, source = TARGETS[name]
        if required_kind not in target.get("kind", []) or item.get("profile", {}).get("test") is not True or Path(target.get("src_path", "")).resolve() != (repo / CRATE / source).resolve():
            raise RuntimeError("Cargo returned an unexpected unit/integration-test artifact.")
        path = Path(item["executable"])
        if not path.is_absolute() or path.is_symlink() or not path.resolve().is_relative_to(target_dir) or not path.is_file() or not path.stat().st_mode & 0o111:
            raise RuntimeError("Cargo test executable must be an ordinary executable inside the selected target cache.")
        for part in [path, *path.parents]:
            if part == target_dir:
                break
            if part.is_symlink():
                raise RuntimeError("Cargo test executable path contains a symlink.")
        record = {"path": str(path), "fingerprint": fingerprint(path)}
        if name in found and found[name] != record:
            raise RuntimeError("Cargo returned different executables for one target.")
        found[name] = record
    if set(found) != expected:
        raise RuntimeError("Capped Cargo output did not identify all three expected test executables.")
    return found


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


def bound_names(target, text):
    listed = [match[1] for line in text.splitlines() if (match := re.match(r"^(.+): test$", line))]
    result = {}
    for full_name in listed:
        logical = full_name.rsplit("::", 1)[-1] if target == UNIT_TARGET else full_name
        if logical in result or logical not in TESTS[target]:
            raise RuntimeError("Unexpected or duplicated test metadata: " + target + ": " + full_name)
        result[logical] = full_name
    if set(result) != set(TESTS[target]):
        raise RuntimeError("Required test metadata differs: " + target)
    return result


def test_summary(text, name, expected_failure=False):
    rows = [(match[1], *map(int, match.groups()[1:])) for line in text.splitlines()
            if (match := re.match(r"^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;", line))]
    marker = CONTRACT_CASES.get(name)
    if expected_failure:
        if (rows != [("FAILED", 0, 1, 0)] or "not yet implemented" not in text
                or FAILURE_FILES[name] not in text or marker in text):
            raise RuntimeError("Expected stub failure was not established for " + name)
        return {"passed": 0, "failed": 1, "ignored": 0, "expected_failure": True,
                "failure_kind": "unimplemented comparator accessor", "last_progress": text.splitlines()[-16:]}
    if rows != [("ok", 1, 0, 0)] or (marker is not None and text.splitlines().count(marker) != 1):
        raise RuntimeError("Test did not produce one passing result and its required completion marker: " + name)
    return {"passed": 1, "failed": 0, "ignored": 0, "expected_failure": False,
            "marker": marker, "last_progress": text.splitlines()[-12:]}


def verify(repo, target_dir, directory, before_phase=None, expected_failures=frozenset()):
    """Always execute fresh tests; expected stub failures are installer-only."""
    repo, target_dir, directory = map(lambda p: Path(p).expanduser().resolve(), (repo, target_dir, directory))
    directory.mkdir(parents=True, exist_ok=False, mode=0o700)
    report_path = directory / "VERIFICATION_RETURN.json"
    expected_failures = set(expected_failures)
    report = {"workflow": WORKFLOW, "outcome": "failed", "phase": "preflight", "repository": str(repo),
              "target_directory": str(target_dir), "run_directory": str(directory), "workspace_suite": "not run",
              "full_coverage_claimed": False, "validation_reused": False, "build_command": BUILD,
              "expected_failures": sorted(expected_failures), "readouts": {}, "cases": {}}
    lock = None
    try:
        if expected_failures - CONTRACT_CASES.keys():
            raise RuntimeError("Only the two new comparator contracts may be expected stub failures.")
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
        def cache_git(*args):
            return subprocess.run(["git", "-c", "core.fsmonitor=false", *args], cwd=common.parent,
                                  env=env, capture_output=True, timeout=5)
        report["cache_exclusion"] = prepare_comparator_cache(common.parent, common, target_dir,
                                                              directory / "cache-exclusion", cache_git)
        target_dir.mkdir(parents=True, exist_ok=True)
        report.update(budgets=controller.BUDGETS, controller_sha256=CONTROLLER_SHA256, shell_sha256=SHELL_SHA256)
        guard = controller.Guard(directory / "guard", controller_path, env)
        runtime_env = {"CARGO_TARGET_DIR": str(target_dir), "CARGO_BUILD_JOBS": "1", "CARGO_NET_OFFLINE": "true",
                       "RUST_BACKTRACE": "1", "RUSTFLAGS": "-Awarnings", "CARGO_NET_GIT_FETCH_WITH_CLI": "true"}
        def phase(label, command, kind="case", extra=None, tmpdir=None, allowed_exits=(0,)):
            if before_phase:
                before_phase(label)
            report["phase"] = label
            atomic_json(report_path, report)
            print("Protected comparator phase: " + label, flush=True)
            result = guard.run(label, command, repo, dict(runtime_env, **(extra or {})), kind=kind, tmpdir=tmpdir)
            report["readouts"][label] = compact(result)
            atomic_json(report_path, report)
            require_run(result, allowed_exits)
            return result
        preflight = phase("preflight", [sys.executable, "-c", "print('GUARD_PREFLIGHT_OK')"], "probe")
        if "GUARD_PREFLIGHT_OK" not in log_text(preflight):
            raise RuntimeError("Guard preflight success marker is absent.")
        def snapshot(label):
            output = directory / (label + ".json")
            result = phase(label, [sys.executable, str(Path(__file__).resolve()), "--capture-inputs", str(repo), str(output)],
                           "probe", extra={"RUSTFLAGS": ""})
            if "COMPARATOR_INPUTS_RECORDED" not in log_text(result):
                raise RuntimeError("Input snapshot completion evidence is absent.")
            return _load_json(output)
        initial = snapshot("inputs-before")
        report["head"] = initial["head"]
        report["inputs_before_sha256"] = hashlib.sha256(encoded(initial)).hexdigest()
        nix = ["nix-shell", "--option", "max-jobs", "0", "--option", "builders", "", "--option", "substitute", "false",
               str(repo / "shell.nix"), "--run"]
        native = phase("native-tools", nix + ["rustc -Vv && cargo -V"], "build")
        if RUSTC_VV not in log_text(native) or CARGO_VERSION not in log_text(native).splitlines():
            raise RuntimeError("Rust/Cargo differs from the accepted native toolchain.")
        built = phase("compile-tests", nix + [shlex.join(BUILD)], "build")
        artifacts = artifacts_from(controller, built, target_dir, repo)
        report["artifacts"] = artifacts
        listings, actual_names = {}, {}
        for target in TARGETS:
            path = Path(artifacts[target]["path"])
            if fingerprint(path) != artifacts[target]["fingerprint"]:
                raise RuntimeError("Compiled test executable changed before metadata inspection.")
            result = phase("list-" + target, [str(path), "--list"])
            bound = bound_names(target, log_text(result))
            actual_names.update(bound)
            listings[target] = sorted(bound.values())
        report["protected_test_lists"] = listings
        for target, name in CASES:
            path = Path(artifacts[target]["path"])
            if fingerprint(path) != artifacts[target]["fingerprint"]:
                raise RuntimeError("Compiled test executable changed before case execution.")
            expected_failure = name in expected_failures
            result = phase(name, [str(path), "--exact", actual_names[name], "--nocapture", "--test-threads=1"],
                           tmpdir=directory / ("case-data-" + name), allowed_exits=(101,) if expected_failure else (0,))
            report["cases"][name] = test_summary(log_text(result), name, expected_failure)
            report["cases"][name].update(target=target, executed_name=actual_names[name])
            atomic_json(report_path, report)
        for target, record in artifacts.items():
            if fingerprint(Path(record["path"])) != record["fingerprint"]:
                raise RuntimeError("Compiled test executable changed during verification: " + target)
        final = snapshot("inputs-after")
        report["inputs_after_sha256"] = hashlib.sha256(encoded(final)).hexdigest()
        if final != initial:
            raise RuntimeError("Repository/build inputs or Git index changed during verification; results were not accepted.")
        report.update(outcome="baseline_confirmed" if expected_failures else "passed", phase="complete",
                      passed=sum(row["passed"] for row in report["cases"].values()),
                      expected_failed=len(expected_failures), test_count=len(CASES), inputs_preserved=True)
        atomic_json(report_path, report)
        return report
    except BaseException as error:
        report.update(outcome="failed", error=type(error).__name__ + ": " + str(error))
        atomic_json(report_path, report)
        raise
    finally:
        if lock:
            lock.close()


def main():
    if len(sys.argv) == 4 and sys.argv[1] == "--capture-inputs":
        capture_inputs(Path(sys.argv[2]).resolve(), Path(sys.argv[3])); return 0
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[2], help="checkout to verify; never implicitly switched")
    parser.add_argument("--target-dir", type=Path, help="dedicated cache; default COMMON_GIT_DIR/../target-comparator")
    args = parser.parse_args()
    repo = args.repo.expanduser().resolve()
    directory = None
    try:
        _, common = repository_dirs(repo)
        target = args.target_dir.expanduser().resolve() if args.target_dir else common.parent / "target-comparator"
        directory = common / "codex-repairs" / WORKFLOW / "runs" / (dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])
        report = verify(repo, target, directory)
    except BaseException as error:
        report = _load_json(directory / "VERIFICATION_RETURN.json") if directory and (directory / "VERIFICATION_RETURN.json").is_file() else {"workflow": WORKFLOW, "outcome": "failed", "error": type(error).__name__ + ": " + str(error)}
    print("BEGIN_COMPARATOR_CHECK_RETURN"); print(json.dumps(report, indent=2)); print("END_COMPARATOR_CHECK_RETURN")
    if directory:
        print("Return file: " + str(directory / "VERIFICATION_RETURN.json"))
    return 0 if report.get("outcome") == "passed" else 1


if __name__ == "__main__":
    sys.exit(main())
