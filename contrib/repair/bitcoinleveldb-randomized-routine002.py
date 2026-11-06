#!/usr/bin/env python3
"""Install a repeatable, resource-controlled randomized verification command."""

ROUTINE_SOURCE = b'#!/usr/bin/env python3\n"""Fresh, resource-controlled verification of the finite randomized contracts."""\nimport argparse\nimport datetime as dt\nimport fcntl\nimport hashlib\nimport importlib.util\nimport json\nimport os\nfrom pathlib import Path\nimport re\nimport shlex\nimport stat\nimport subprocess\nimport sys\nimport uuid\n\nWORKFLOW = "leveldb-randomized-check002"\nCONTROLLER_PATH = "contrib/repair/bitcoinleveldb-randomized-bounded001.py"\nCONTROLLER_SHA256 = "bd8fbea7164f83d2b5ae80c45bf831a2da7c5b456ca10753fac09e09d4d88fc4"\nSHELL_SHA256 = "8f316c4f007426302e26962e13def8c7d5bc0efa85d5e1549c55e85ba0d7c651"\nCRATE = "bitcoinleveldbt-randomized"\nSTRESS = {"randomized": "db_test_randomized", "bisect": "db_test_bisect_randomized"}\nCASES = [("bounded_default", 0), ("bounded_reuse", 1), ("bounded_filter", 2), ("bounded_uncompressed", 3)]\nBUILD = ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", CRATE, "--no-run", "--message-format=json"]\nRUSTC_VV = """rustc 1.97.0-nightly (9eb3be26b 2026-05-18)\nbinary: rustc\ncommit-hash: 9eb3be26b46eccea1de7448ea9cc3c1d20bb1a35\ncommit-date: 2026-05-18\nhost: x86_64-unknown-linux-gnu\nrelease: 1.97.0-nightly\nLLVM version: 22.1.4"""\nCARGO_VERSION = "cargo 1.97.0-nightly (4d1f98451 2026-05-15)"\nGIT_SELECTION = {"GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR", "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_PREFIX", "GIT_NAMESPACE"}\n\n\ndef encoded(value):\n    return (json.dumps(value, indent=2, sort_keys=True) + "\\n").encode()\n\n\ndef atomic_json(path, value):\n    path = Path(path)\n    path.parent.mkdir(parents=True, exist_ok=True)\n    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)\n    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)\n    try:\n        with os.fdopen(fd, "wb") as stream:\n            stream.write(encoded(value)); stream.flush(); os.fsync(stream.fileno())\n        os.replace(temporary, path)\n    finally:\n        if temporary.exists():\n            temporary.unlink()\n\n\ndef fingerprint(path):\n    path = Path(path)\n    info = path.lstat()\n    if not stat.S_ISREG(info.st_mode):\n        raise RuntimeError("Expected an ordinary file: " + str(path))\n    digest = hashlib.sha256()\n    with path.open("rb") as stream:\n        for chunk in iter(lambda: stream.read(65536), b""):\n            digest.update(chunk)\n    return {"kind": "file", "mode": stat.S_IMODE(info.st_mode), "size": info.st_size, "sha256": digest.hexdigest()}\n\n\ndef ordinary_path(path, root):\n    path, root = Path(path), Path(root)\n    if not path.is_relative_to(root):\n        raise RuntimeError("Expected a path inside the selected repository: " + str(path))\n    for part in [path, *path.parents]:\n        if part == root:\n            break\n        if part.is_symlink():\n            raise RuntimeError("Expected an ordinary repository path: " + str(path))\n    return path\n\n\ndef repository_dirs(repo):\n    """Read standard Git worktree metadata without spawning an unguarded child."""\n    dotgit = repo / ".git"\n    if dotgit.is_symlink():\n        raise RuntimeError("The selected .git path is a symlink.")\n    if dotgit.is_dir():\n        gitdir = dotgit.resolve()\n    elif dotgit.is_file() and dotgit.stat().st_size < 16384:\n        text = dotgit.read_text().strip()\n        if not text.startswith("gitdir: "):\n            raise RuntimeError("Unsupported Git worktree metadata.")\n        gitdir = (repo / text[8:]).resolve()\n    else:\n        raise RuntimeError("--repo must name a Git checkout root.")\n    common_file = gitdir / "commondir"\n    if common_file.exists():\n        if common_file.is_symlink() or common_file.stat().st_size >= 16384:\n            raise RuntimeError("Unsupported common Git directory metadata.")\n        common = (gitdir / common_file.read_text().strip()).resolve()\n    else:\n        common = gitdir\n    if not (gitdir / "HEAD").is_file() or not (common / "objects").is_dir():\n        raise RuntimeError("Incomplete Git checkout metadata.")\n    return gitdir, common\n\n\ndef checked_environment():\n    env = dict(os.environ)\n    cleared = sorted(key for key in GIT_SELECTION if env.pop(key, None) is not None)\n    bad = [key for key, value in env.items() if value and (key in {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_TARGET", "CARGO_BUILD_RUSTFLAGS", "CARGO_BUILD_RUSTC", "CARGO_BUILD_RUSTC_WRAPPER", "RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER"} or key.startswith("CARGO_PROFILE_") or (key.startswith("CARGO_TARGET_") and key.endswith(("_RUSTFLAGS", "_RUSTC_WRAPPER"))))]\n    if bad:\n        raise RuntimeError("Unexpected Rust/build overrides require review: " + repr(sorted(bad)))\n    env["GIT_OPTIONAL_LOCKS"] = "0"\n    return env, cleared\n\n\ndef load_controller(repo):\n    path = ordinary_path(repo / CONTROLLER_PATH, repo)\n    if fingerprint(path)["sha256"] != CONTROLLER_SHA256:\n        raise RuntimeError("The reviewed resource controller differs from its accepted SHA256.")\n    if fingerprint(ordinary_path(repo / "shell.nix", repo))["sha256"] != SHELL_SHA256:\n        raise RuntimeError("The selected shell.nix differs from the accepted pinned native environment.")\n    sys.dont_write_bytecode = True\n    spec = importlib.util.spec_from_file_location("randomized_bounded_controller001", path)\n    module = importlib.util.module_from_spec(spec)\n    spec.loader.exec_module(module)\n    return module, path\n\n\ndef capture_inputs(repo, output):\n    """Internal probe. Its Git children and hashing run inside the probe service."""\n    env, _ = checked_environment()\n    def git(*args):\n        result = subprocess.run(["git", "-c", "core.fsmonitor=false", *args], cwd=repo, env=env, capture_output=True, timeout=5)\n        if result.returncode or len(result.stdout) > 4*1024*1024 or len(result.stderr) > 65536:\n            raise RuntimeError("Guarded Git metadata query failed or exceeded its output allowance.")\n        return result.stdout\n    if Path(os.fsdecode(git("rev-parse", "--show-toplevel")).strip()).resolve() != repo:\n        raise RuntimeError("--repo must identify the checkout root.")\n    gitdir, common = repository_dirs(repo)\n    names = {os.fsdecode(name) for name in git("ls-files", "-z", "--cached", "--others", "--exclude-standard").split(b"\\0") if name}\n    if len(names) > 100000:\n        raise RuntimeError("Input inventory exceeds the finite metadata allowance.")\n    selected = {name for name in names if name.endswith((".rs", ".nix")) or Path(name).name in {"Cargo.toml", "Cargo.lock", "rust-toolchain", "rust-toolchain.toml"} or ".cargo" in Path(name).parts}\n    selected.update({CONTROLLER_PATH, "contrib/test/leveldb-randomized.py", "shell.nix"})\n    inputs = {}\n    total = 0\n    for name in sorted(selected):\n        path = ordinary_path(repo / name, repo)\n        if path.stat().st_size > 16*1024*1024:\n            raise RuntimeError("A build input exceeds the finite inventory allowance: " + name)\n        total += path.stat().st_size\n        if total > 256*1024*1024:\n            raise RuntimeError("Build input inventory exceeds 256 MiB.")\n        inputs[name] = fingerprint(path)\n    configurations = {}\n    homes = [Path(env.get("CARGO_HOME", Path.home() / ".cargo"))]\n    homes.extend(parent / ".cargo" for parent in [repo, *repo.parents])\n    for home in homes:\n        for name in ["config", "config.toml"]:\n            path = home / name\n            if path.exists() or path.is_symlink():\n                configurations[str(path)] = fingerprint(path)\n    result = {"head": git("rev-parse", "HEAD").decode().strip(), "inputs": inputs, "cargo_configurations": configurations,\n              "index": fingerprint(gitdir / "index") if (gitdir / "index").exists() else None,\n              "git_directory": str(gitdir), "common_directory": str(common)}\n    atomic_json(output, result)\n    print("RANDOMIZED_INPUTS_RECORDED", flush=True)\n\n\ndef _load_json(path):\n    path = Path(path)\n    if path.is_symlink() or not path.is_file() or path.stat().st_size > 8*1024*1024:\n        raise RuntimeError("Expected capped ordinary JSON evidence: " + str(path))\n    return json.loads(path.read_text())\n\n\n\ndef artifacts_from(controller, result, target_dir, repo):\n    found = {}\n    expected = set(STRESS) | {"bounded_contracts"}\n    for line in controller.log_text(result).splitlines():\n        try:\n            item = json.loads(line)\n        except ValueError:\n            continue\n        if not isinstance(item, dict):\n            continue\n        target = item.get("target", {})\n        name = target.get("name")\n        if item.get("reason") != "compiler-artifact" or name not in expected or not item.get("executable"):\n            continue\n        if "test" not in target.get("kind", []) or item.get("profile", {}).get("test") is not True or Path(target.get("src_path", "")).resolve() != (repo / CRATE / "tests" / (name + ".rs")).resolve():\n            raise RuntimeError("Cargo returned an unexpected integration-test artifact.")\n        path = Path(item["executable"])\n        if not path.is_absolute() or path.is_symlink() or not path.resolve().is_relative_to(target_dir) or not path.is_file() or not path.stat().st_mode & 0o111:\n            raise RuntimeError("Cargo test executable must be an ordinary executable inside the selected target cache.")\n        for part in [path, *path.parents]:\n            if part == target_dir:\n                break\n            if part.is_symlink():\n                raise RuntimeError("Cargo test executable path contains a symlink.")\n        record = {"path": str(path), "fingerprint": fingerprint(path)}\n        if name in found and found[name] != record:\n            raise RuntimeError("Cargo returned different executables for one target.")\n        found[name] = record\n    if set(found) != expected:\n        raise RuntimeError("Capped Cargo output did not identify all three expected integration-test executables.")\n    return found\n\n\ndef verify(repo, target_dir, directory, before_phase=None):\n    """Always rerun; failed evidence remains in directory/VERIFICATION_RETURN.json."""\n    repo, target_dir, directory = map(lambda p: Path(p).expanduser().resolve(), (repo, target_dir, directory))\n    directory.mkdir(parents=True, exist_ok=False, mode=0o700)\n    report_path = directory / "VERIFICATION_RETURN.json"\n    report = {"workflow": WORKFLOW, "outcome": "failed", "phase": "preflight", "repository": str(repo), "target_directory": str(target_dir), "run_directory": str(directory), "full_stress": "not rerun", "full_coverage_claimed": False, "validation_reused": False, "build_command": BUILD, "readouts": {}, "cases": {}}\n    lock = None\n    try:\n        controller, controller_path = load_controller(repo)\n        env, report["cleared_git_selection_variables"] = checked_environment()\n        _, common = repository_dirs(repo)\n        metadata = common / "codex-repairs" / WORKFLOW\n        metadata.mkdir(parents=True, exist_ok=True)\n        lock_path = metadata / "run.lock"\n        if lock_path.is_symlink():\n            raise RuntimeError("Verification lock is a symlink.")\n        lock = lock_path.open("a+")\n        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)\n        target_dir.mkdir(parents=True, exist_ok=True)\n        report.update(budgets=controller.BUDGETS, controller_sha256=CONTROLLER_SHA256, shell_sha256=SHELL_SHA256)\n        guard = controller.Guard(directory / "guard", controller_path, env)\n        runtime_env = {"CARGO_TARGET_DIR": str(target_dir), "CARGO_BUILD_JOBS": "1", "CARGO_NET_OFFLINE": "true", "RUST_BACKTRACE": "1", "RUSTFLAGS": "-Awarnings", "CARGO_NET_GIT_FETCH_WITH_CLI": "true"}\n        def phase(label, command, kind="case", extra=None, tmpdir=None):\n            if before_phase:\n                before_phase(label)\n            report["phase"] = label\n            atomic_json(report_path, report)\n            print("Protected randomized phase: " + label, flush=True)\n            result = guard.run(label, command, repo, dict(runtime_env, **(extra or {})), kind=kind, tmpdir=tmpdir)\n            report["readouts"][label] = controller.compact(result)\n            atomic_json(report_path, report)\n            controller.require_run(result)\n            return result\n        preflight = phase("preflight", [sys.executable, "-c", "print(\'GUARD_PREFLIGHT_OK\')"], "probe")\n        if "GUARD_PREFLIGHT_OK" not in controller.log_text(preflight):\n            raise RuntimeError("Guard preflight success marker is absent.")\n        def snapshot(label):\n            output = directory / (label + ".json")\n            result = phase(label, [sys.executable, str(Path(__file__).resolve()), "--capture-inputs", str(repo), str(output)], "probe", extra={"RUSTFLAGS": ""})\n            if "RANDOMIZED_INPUTS_RECORDED" not in controller.log_text(result):\n                raise RuntimeError("Input snapshot completion evidence is absent.")\n            return _load_json(output)\n        initial = snapshot("inputs-before")\n        report["head"] = initial["head"]\n        report["inputs_before_sha256"] = hashlib.sha256(encoded(initial)).hexdigest()\n        nix = ["nix-shell", "--option", "max-jobs", "0", "--option", "builders", "", "--option", "substitute", "false", str(repo / "shell.nix"), "--run"]\n        native = phase("native-tools", nix + ["rustc -Vv && cargo -V"], "build")\n        if RUSTC_VV not in controller.log_text(native) or CARGO_VERSION not in controller.log_text(native).splitlines():\n            raise RuntimeError("Rust/Cargo differs from the accepted native toolchain.")\n        built = phase("compile-tests", nix + [shlex.join(BUILD)], "build")\n        artifacts = artifacts_from(controller, built, target_dir, repo)\n        report["artifacts"] = artifacts\n        listings = {}\n        for target, expected in [(name, {function}) for name, function in STRESS.items()] + [("bounded_contracts", {name for name, _ in CASES})]:\n            path = Path(artifacts[target]["path"])\n            if fingerprint(path) != artifacts[target]["fingerprint"]:\n                raise RuntimeError("Compiled test executable changed before metadata inspection.")\n            result = phase("list-" + target, [str(path), "--list"] + (["--ignored"] if target in STRESS else []))\n            names = {match[1] for line in controller.log_text(result).splitlines() if (match := re.match(r"^(.+): test$", line))}\n            if names != expected:\n                raise RuntimeError("Required ordinary/ignored test metadata differs: " + target)\n            listings[target] = sorted(names)\n        report["protected_test_lists"] = listings\n        for name, ordinal in CASES:\n            path = Path(artifacts["bounded_contracts"]["path"])\n            if fingerprint(path) != artifacts["bounded_contracts"]["fingerprint"]:\n                raise RuntimeError("Bounded executable changed before case execution.")\n            result = phase(name, [str(path), "--exact", name, "--nocapture", "--test-threads=1"], extra={"BITCOIN_RS_RANDOMIZED_GUARD": "v1", "TEST_RANDOM_SEED": "301"}, tmpdir=directory / ("case-data-" + name))\n            report["cases"][name] = controller.case_summary(controller.log_text(result), ordinal)\n            atomic_json(report_path, report)\n        if fingerprint(Path(artifacts["bounded_contracts"]["path"])) != artifacts["bounded_contracts"]["fingerprint"]:\n            raise RuntimeError("Bounded executable changed during verification.")\n        final = snapshot("inputs-after")\n        report["inputs_after_sha256"] = hashlib.sha256(encoded(final)).hexdigest()\n        if final != initial:\n            raise RuntimeError("Repository/build inputs or Git index changed during verification; results were not accepted.")\n        report.update(outcome="passed", phase="complete", bounded_passed=4, stress_ignored=2, inputs_preserved=True)\n        atomic_json(report_path, report)\n        return report\n    except BaseException as error:\n        report.update(outcome="failed", error=type(error).__name__ + ": " + str(error))\n        atomic_json(report_path, report)\n        raise\n    finally:\n        if lock:\n            lock.close()\n\n\ndef main():\n    if len(sys.argv) == 4 and sys.argv[1] == "--capture-inputs":\n        capture_inputs(Path(sys.argv[2]).resolve(), Path(sys.argv[3])); return 0\n    parser = argparse.ArgumentParser(description=__doc__)\n    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[2], help="checkout to verify; never implicitly switched")\n    parser.add_argument("--target-dir", type=Path, help="shared cache, default COMMON_GIT_DIR/../target")\n    args = parser.parse_args()\n    repo = args.repo.expanduser().resolve()\n    report = None\n    directory = None\n    try:\n        _, common = repository_dirs(repo)\n        target = (args.target_dir.expanduser().resolve() if args.target_dir else common.parent / "target")\n        directory = common / "codex-repairs" / WORKFLOW / "runs" / (dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])\n        report = verify(repo, target, directory)\n    except BaseException as error:\n        report = _load_json(directory / "VERIFICATION_RETURN.json") if directory and (directory / "VERIFICATION_RETURN.json").is_file() else {"workflow": WORKFLOW, "outcome": "failed", "error": type(error).__name__ + ": " + str(error)}\n    print("BEGIN_RANDOMIZED_CHECK_RETURN"); print(json.dumps(report, indent=2)); print("END_RANDOMIZED_CHECK_RETURN")\n    if directory:\n        print("Return file: " + str(directory / "VERIFICATION_RETURN.json"))\n    return 0 if report.get("outcome") == "passed" else 1\n\n\nif __name__ == "__main__":\n    sys.exit(main())\n'

DOCUMENT_SOURCE = b'# Resource-controlled randomized regression checks\n\nRun the finite randomized contracts through the repository command:\n\n```sh\nnix shell --offline --option max-jobs 0 --option builders \'\' \\\n  nixpkgs#python3 --command python3 \\\n  contrib/test/leveldb-randomized.py\n```\n\nThe command verifies the checkout containing the script. Before this branch is\nmerged, run it from the repair worktree, or select that checkout explicitly:\n\n```sh\nnix shell --offline --option max-jobs 0 --option builders \'\' \\\n  nixpkgs#python3 --command python3 \\\n  "$HOME/bitcoin-rs-randomized-bounded-repair/contrib/test/leveldb-randomized.py" \\\n  --repo "$HOME/bitcoin-rs-randomized-bounded-repair"\n```\n\n`--repo` selects the checkout to verify; it never silently switches branches or\nredirects verification to another worktree. The default Cargo cache is the main\nrepository\'s `target` directory, shared by linked worktrees. `--target-dir PATH`\nselects another cache. Every invocation compiles as needed, inspects the test\nlists, and executes all four cases again. Previous receipts are not accepted as\nsubstitutes for a fresh run.\n\nThe command leaves source files, branches, commits, tags, and the Git index\nunchanged. It uses a nonblocking lock shared across verification of worktrees in\nthe same repository. A concurrent invocation fails without launching the build.\n\n## Scope and evidence\n\nThe cases are `bounded_default`, `bounded_reuse`, `bounded_filter`, and\n`bounded_uncompressed`. Each runs 34 logical steps, including writes, overwrites,\ndeletes, batches, iterator comparisons, saved snapshots, forced flushing,\nreopening, and cleanup. Results are compared against an independent BTreeMap\noracle and the ModelDB. The names identify fixture configuration ordinals;\npassing `bounded_filter` alone does not prove an active Bloom-filter path.\n\nThe command compiles the two existing long stress tests and verifies that both\nare ignored. It never executes either stress workload and accepts no argument\nfor running them. Successful finite checks establish their declared contracts;\nfull stress execution and full coverage remain unmeasured.\n\nThe report prints between `BEGIN_RANDOMIZED_CHECK_RETURN` and\n`END_RANDOMIZED_CHECK_RETURN`. Its path is printed afterward. Reports and capped\nphase logs are stored under the common Git directory in\n`codex-repairs/leveldb-randomized-check002/runs/`. Failed phases retain their\nreports and logs. The selected input inventory is captured in guarded probes before and after\nthe run. It covers tracked and nonignored untracked Rust/Cargo/Nix files,\nCargo configurations, HEAD, and the index. A difference prevents acceptance.\nIt does not inventory every possible included data asset or ignored source\nfile, and it does not create an immutable filesystem snapshot.\n\n## Environment and containment\n\nThe runner imports the SHA256-verified resource controller committed by the\nbounded repair. It requires the accepted pinned `shell.nix` and the measured\nRust/Cargo versions. No original installer journal is needed for routine use.\n\nCargo runs with `--offline --locked --jobs 1` and `CARGO_NET_OFFLINE=true`.\nNix shell evaluation uses the cached pinned environment with local build jobs\nzero, an empty remote-builder list, and substitution disabled. Missing cached\nrequirements cause failure; the runner does not fall back to an unrestricted\nbuild or change system configuration. These controls are not a general network\nsandbox for arbitrary build scripts.\n\nEvery build/test child is admitted through a systemd user service after the\nparent and child verify its actual controls. If the user manager or required\ncgroup-v2 controls are unavailable, execution is refused.\n\n| Phase | Memory hard / soft | Deadline | Tasks | Output cap |\n| --- | --- | --- | --- | --- |\n| Build/native tools | 4 GiB / 3 GiB | 600 s | 128 | 2 MiB |\n| Individual case/metadata | 512 MiB / 384 MiB | 30 s | 64 | 1 MiB |\n| Preflight/input probe | 128 MiB / 96 MiB | 10 s | 32 | 1 MiB |\n\nAll phases are limited to one CPU in aggregate, have swap disabled, and terminate\nthe service\'s entire cgroup on failure. Core dumps are disabled. File-size,\ntest-data, disk-headroom, output, and external wall-clock checks remain active.\nSoft memory-pressure events can occur during a successful build; they are not\nreported as assertion failures. A resource-limit termination is a failed run.\n\nA bare `cargo test`, or the user\'s existing Makefile, does not invoke this\ncontroller. The new bounded cases refuse execution without their required\ncgroup controls. Routine verification should therefore use the command above.\n\n## Retained stress and replay limitations\n\nThe previous-boot journal records global OOM kills of bisect processes, including\none at about 27 GiB of anonymous resident memory. The exact locked tracing source\nalso contains an uncapped vector of formatted event strings. These are separate\nverified findings: the source establishes a retention hazard, while no heap\nprofile attributes the entire observed process memory to those strings.\n\nThe main stress test initializes one PRNG outside its configuration loop. The\nbisect test initializes a fresh PRNG for each configuration. Its full scan and\nreduced prefixes share its own `execute_prefix` implementation, but this does\nnot establish faithful replay of the main test\'s later configuration streams.\n\nA common seed also does not establish that a database failure involving\nbackground work is repeatable or monotone in prefix length. Changing generator\nsharing or replay semantics requires a separate behavior change and focused\nchecks; this command does not claim to repair or qualify the long-run reducer.\n'

"""Install the routine guarded verifier at the accepted bounded checkpoint.

Assembly provides ROUTINE_SOURCE and DOCUMENT_SOURCE as exact byte payloads.
The preceding repair and native-environment controllers are imported by hash.
"""
import argparse
import datetime as dt
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import traceback
import uuid

BASE = "3b9184753f212821a972b5720ce1b501b84b2f26"
ORIGIN_BASE = "a8beeac26124749a69d0ad6f7a999891f8be73f1"
BRANCH = "repair/leveldb-randomized-bounded"
WORKFLOW = "leveldb-randomized-routine002"
PREVIOUS_PATH = "contrib/repair/bitcoinleveldb-randomized-bounded001.py"
PREVIOUS_SHA = "bd8fbea7164f83d2b5ae80c45bf831a2da7c5b456ca10753fac09e09d4d88fc4"
ROUTINE_PATH = "contrib/test/leveldb-randomized.py"
DOCUMENT_PATH = "doc/testing/leveldb-randomized.md"
REPLAY_PATH = "contrib/repair/bitcoinleveldb-randomized-routine002.py"
RECEIPT_PATH = "doc/repair/leveldb-randomized-routine002.json"
PATHS = {ROUTINE_PATH, DOCUMENT_PATH, REPLAY_PATH, RECEIPT_PATH}
EXECUTABLES = {ROUTINE_PATH, REPLAY_PATH}


def checked_import(path, digest, name):
    path = Path(path)
    if path.is_symlink() or hashlib.sha256(path.read_bytes()).hexdigest() != digest:
        raise RuntimeError("Reviewed controller differs: " + str(path))
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def worker_guard(runner, final, head, files, pending, initial=False):
    runner.guard_operations()
    if runner.git("symbolic-ref", "--quiet", "--short", "HEAD") != BRANCH or runner.git("rev-parse", "HEAD") != head:
        raise RuntimeError("Repair branch advanced beyond this checkpoint; all work retained.")
    dirty = final.names(runner, "diff", "--no-renames", "--name-only") | final.names(runner, "diff", "--cached", "--no-renames", "--name-only")
    untracked = final.names(runner, "ls-files", "--others", "--exclude-standard")
    if (dirty | untracked) - (PATHS if pending else set()):
        raise RuntimeError("Repair worktree contains unrelated changes; all work retained.")
    configs = {".cargo/config", ".cargo/config.toml"}
    tracked = final.names(runner, "ls-files", "--", *sorted(configs))
    if any((runner.repo / name).exists() or (runner.repo / name).is_symlink() for name in configs - tracked):
        raise RuntimeError("Untracked or ignored Cargo configuration requires review.")
    for name, expected in files.items():
        path = runner.repo / name
        for part in [path, *path.parents]:
            if part == runner.repo:
                break
            if part.is_symlink() or (part != path and part.exists() and not part.is_dir()):
                raise RuntimeError("Owned path has a symlink or non-directory ancestor: " + name)
        current = path.read_bytes() if path.exists() else None
        indexed = final.blob(runner, "", name, False)
        allowed_working = {expected, None} if initial and pending else {expected}
        allowed_index = {expected, None} if pending else {expected}
        if current not in allowed_working or indexed not in allowed_index:
            raise RuntimeError("Unexpected owned file/index content retained: " + name)
        if path.exists() and path.stat().st_mode & 0o7777 != (0o755 if name in EXECUTABLES else 0o644):
            raise RuntimeError("Unexpected owned-file permissions retained: " + name)


def own_commit(runner, final, head, files):
    return (runner.git("show", "-s", "--format=%P", head).split() == [BASE]
            and final.changed(runner, BASE, head) == PATHS
            and all(final.blob(runner, head, name, False) == data
                    and runner.git("ls-tree", head, "--", name).startswith(("100755" if name in EXECUTABLES else "100644") + " blob ")
                    for name, data in files.items()))


def validate_verification(result):
    expected = {name for name in ("bounded_default", "bounded_reuse", "bounded_filter", "bounded_uncompressed")}
    if result.get("outcome") != "passed" or set(result.get("cases", {})) != expected:
        raise RuntimeError("Routine verification did not establish all four passing bounded cases.")
    if result.get("full_stress") != "not rerun" or result.get("full_coverage_claimed") is not False:
        raise RuntimeError("Routine verification scope differs from this checkpoint.")
    for row in result["cases"].values():
        if (row.get("passed"), row.get("failed"), row.get("ignored")) != (1, 0, 0):
            raise RuntimeError("Routine verification has an incomplete bounded case.")


def run_install(origin, script_path):
    report = {"workflow": WORKFLOW, "outcome": "failed", "phase": "preflight", "base": BASE,
              "branch": BRANCH, "full_stress": "not rerun", "full_coverage_claimed": False,
              "merge": "not performed at this checkpoint", "tag": "not performed at this checkpoint"}
    runner = final = module = original_guard = directory = None
    locks = []
    failure = None
    try:
        worktree = origin.parent / (origin.name + "-randomized-bounded-repair")
        if worktree.is_symlink():
            raise RuntimeError("Recorded repair worktree path is a symlink.")
        previous = checked_import(worktree / PREVIOUS_PATH, PREVIOUS_SHA, "accepted_bounded001")
        dev = previous.trusted_devshell(origin)
        final = dev.trusted_finalizer(origin)
        if Path(final.bootstrap_git(origin, "rev-parse", "--show-toplevel")).resolve() != origin:
            raise RuntimeError("--repo must identify the original repository root.")
        if final.bootstrap_git(origin, "symbolic-ref", "--quiet", "--short", "HEAD") != "master" or final.bootstrap_git(origin, "rev-parse", "HEAD") != ORIGIN_BASE:
            raise RuntimeError("Original master must remain at the reviewed native-environment checkpoint.")
        common = Path(final.bootstrap_git(origin, "rev-parse", "--path-format=absolute", "--git-common-dir"))
        if (Path(final.bootstrap_git(worktree, "rev-parse", "--path-format=absolute", "--git-common-dir")).resolve() != common.resolve()
                or ("worktree " + str(worktree) + "\n") not in final.bootstrap_git(origin, "worktree", "list", "--porcelain")):
            raise RuntimeError("Bounded worktree is not registered to this original repository.")
        metadata = common / "codex-repairs" / WORKFLOW
        metadata.mkdir(parents=True, exist_ok=True)
        for path in (common / "codex-repairs" / final.REPAIR / "run.lock",
                     common / "codex-repairs" / dev.WORKFLOW / "run.lock",
                     common / "codex-repairs" / previous.WORKFLOW / "run.lock", metadata / "run.lock"):
            if path.is_symlink():
                raise RuntimeError("Workflow lock path is a symlink.")
            lock = path.open("a+"); locks.append(lock)
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        module, runner = final.trusted_coverage(origin).load_repair_runner(worktree, origin)
        runner.env["GIT_OPTIONAL_LOCKS"] = "0"
        directory = metadata / "runs" / (dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])
        directory.mkdir(parents=True)
        runner.directory, runner.log, runner.report = directory, (directory / "run.log").open("w", buffering=1), report
        runner.repo = worktree
        report.update(origin_repository=str(origin), worktree=str(worktree), run_directory=str(directory))
        final.environment_guard(runner)
        if runner.git("show", "-s", "--format=%P", BASE).split() != [ORIGIN_BASE]:
            raise RuntimeError("Accepted bounded checkpoint ancestry differs.")
        previous_files = {name: final.blob(runner, BASE, name) for name in previous.PATHS}
        if (previous_files[PREVIOUS_PATH] != (worktree / PREVIOUS_PATH).read_bytes()
                or previous_files[previous.BOUNDED_PATH] != previous.RUST_TEST_SOURCE
                or not previous.own_commit(runner, final, BASE, previous_files)):
            raise RuntimeError("Accepted bounded checkpoint differs from its reviewed five-file scope.")
        for name, function in previous.STRESS.items():
            path = previous.CRATE + "/tests/" + name + ".rs"
            if previous_files[path] != previous.stress_replacement(final.blob(runner, ORIGIN_BASE, path), function):
                raise RuntimeError("Accepted ignored stress source differs: " + path)
        if (worktree / "shell.nix").read_bytes() != final.blob(runner, ORIGIN_BASE, "shell.nix"):
            raise RuntimeError("Native development shell differs from the accepted checkpoint.")
        files = {ROUTINE_PATH: ROUTINE_SOURCE, DOCUMENT_PATH: DOCUMENT_SOURCE, REPLAY_PATH: script_path.read_bytes()}
        if any(final.blob(runner, BASE, name, False) is not None for name in PATHS):
            raise RuntimeError("A new routine-checkpoint path already exists in its reviewed base.")
        identity = {"workflow": WORKFLOW, "base": BASE, "branch": BRANCH,
                    "file_hashes": {name: final.sha(data) for name, data in files.items()}}
        state_path = metadata / "state.json"
        if state_path.is_symlink():
            raise RuntimeError("Routine checkpoint state is a symlink.")
        state = json.loads(state_path.read_text()) if state_path.exists() else dict(identity, phase="pending")
        if any(state.get(key) != value for key, value in identity.items()) or state.get("blocked_checkpoint"):
            raise RuntimeError("Routine checkpoint journal identity changed; all work retained.")
        files[RECEIPT_PATH] = final.encoded(state["receipt"]) if state.get("receipt") is not None else None
        before = final.userdata(runner, PATHS)
        tag_oid = runner.git("rev-parse", "refs/tags/" + final.TAG)
        original_index = final.fingerprint(Path(final.bootstrap_git(origin, "rev-parse", "--path-format=absolute", "--git-path", "index")))
        report["userdata_before_sha256"] = final.sha(final.encoded(before))
        inputs = final.integration_inputs(runner, ORIGIN_BASE)
        def original_guard():
            saved = runner.repo
            runner.repo = origin
            try:
                runner.guard_operations()
                if (runner.git("symbolic-ref", "--quiet", "--short", "HEAD") != "master"
                        or runner.git("rev-parse", "HEAD") != ORIGIN_BASE
                        or final.userdata(runner, PATHS) != before
                        or final.fingerprint(Path(runner.git("rev-parse", "--path-format=absolute", "--git-path", "index"))) != original_index):
                    raise RuntimeError("Original branch/HEAD/index/user-owned state changed; all work retained.")
                if (final.names(runner, "diff", "--no-renames", "--name-only", ORIGIN_BASE)
                        | final.names(runner, "diff", "--cached", "--no-renames", "--name-only", ORIGIN_BASE)) & inputs.keys():
                    raise RuntimeError("Original Rust/build inputs changed.")
                if runner.git("rev-parse", "refs/tags/" + final.TAG) != tag_oid:
                    raise RuntimeError("Previous snapshot repair tag changed.")
            finally:
                runner.repo = saved
        original_guard()
        checkpoint = state.get("checkpoint")
        head = runner.git("rev-parse", "HEAD")
        if head != BASE:
            if state.get("receipt") is None or not own_commit(runner, final, head, files) or checkpoint not in (None, head):
                raise RuntimeError("Repair branch contains an unowned checkpoint; all work retained.")
            validate_verification(state["receipt"]["verification"])
            previous.require_run(state.get("git_commit", {}))
            checkpoint = state["checkpoint"] = head
        elif checkpoint is not None:
            raise RuntimeError("Recorded routine checkpoint no longer matches the branch.")
        worker_guard(runner, final, checkpoint or BASE, files, not checkpoint, initial=not checkpoint)
        if not checkpoint:
            for name, data in files.items():
                if data is not None:
                    module.atomic_write(worktree / name, data, 0o755 if name in EXECUTABLES else 0o644)
        runner.save_state(state_path, state)
        routine = checked_import(worktree / ROUTINE_PATH, final.sha(ROUTINE_SOURCE), "routine_randomized002")
        def before_phase(label):
            report["phase"] = label
            runner.say("Routine phase: " + label)
            original_guard()
            worker_guard(runner, final, checkpoint or BASE, files, not checkpoint)
        report["phase"] = "verification"
        try:
            verification = routine.verify(worktree, origin / "target", directory / "verification", before_phase=before_phase)
        except BaseException:
            evidence = directory / "verification" / "VERIFICATION_RETURN.json"
            if evidence.is_file() and evidence.stat().st_size <= 2 * 1024 * 1024:
                report["verification"] = json.loads(evidence.read_text())
            raise
        report["verification"] = verification
        validate_verification(verification)
        original_guard()
        worker_guard(runner, final, checkpoint or BASE, files, not checkpoint)
        if not checkpoint:
            receipt = {"workflow": WORKFLOW, "base": BASE, "file_hashes": identity["file_hashes"],
                       "verification": verification, "full_stress": "not rerun", "full_coverage_claimed": False,
                       "replay_status": "legacy cross-configuration stream differs from bisector; not repaired here"}
            state.update(receipt=receipt, phase="validated")
            runner.save_state(state_path, state)
            files[RECEIPT_PATH] = final.encoded(receipt)
            module.atomic_write(worktree / RECEIPT_PATH, files[RECEIPT_PATH], 0o644)
            for name in sorted(PATHS):
                runner.git("add", "--chmod=" + ("+x" if name in EXECUTABLES else "-x"), "--", name)
            original_guard(); worker_guard(runner, final, BASE, files, True)
            message = directory / "commit-message.txt"
            message.write_text("test: add repeatable guarded randomized verification\n\n"
                               "Add a repository command that freshly builds and runs the four bounded cases under verified resource controls.\n"
                               "Document confirmed OOM evidence, uncapped tracing retention, and the unresolved cross-configuration replay mismatch.\n"
                               "Keep original stress tests opt-in; no full stress or coverage claim.\n\n"
                               "Repair-Workflow: " + WORKFLOW + "\nBase: " + BASE + "\n")
            report["phase"] = "git-commit"
            guard = previous.Guard(directory / "git-guard", worktree / PREVIOUS_PATH, runner.env)
            preflight = guard.preflight(); previous.require_run(preflight)
            report["git_preflight"] = previous.compact(preflight)
            if "GUARD_PREFLIGHT_OK" not in previous.log_text(preflight):
                raise RuntimeError("Git guard preflight completion evidence is absent.")
            original_guard(); worker_guard(runner, final, BASE, files, True)
            committed = guard.run("git-commit", ["git", "commit", "--file", str(message)], worktree, {}, kind="build")
            report["git_commit"] = previous.compact(committed)
            state["git_commit"] = report["git_commit"]
            runner.save_state(state_path, state)
            checkpoint = runner.git("rev-parse", "HEAD")
            if checkpoint != BASE and not own_commit(runner, final, checkpoint, files):
                state["blocked_checkpoint"] = checkpoint
                runner.save_state(state_path, state)
                raise RuntimeError("Commit exceeded its exact four-file scope; retained for review.")
            previous.require_run(committed)
            if checkpoint == BASE:
                raise RuntimeError("Guarded Git command did not create the expected checkpoint.")
        original_guard(); worker_guard(runner, final, checkpoint, files, False)
        state.update(checkpoint=checkpoint, phase="complete"); runner.save_state(state_path, state)
        report.update(outcome="passed", phase="complete", checkpoint=checkpoint, original_preserved=True,
                      userdata_after_sha256=final.sha(final.encoded(final.userdata(runner, PATHS))),
                      routine_command="python3 " + str(worktree / ROUTINE_PATH) + " --repo " + str(worktree))
    except KeyboardInterrupt:
        failure = "Interrupted; owned files and logs retained. No unrestricted retry was attempted."
    except Exception as error:
        failure = type(error).__name__ + ": " + str(error)
        report["exception_traceback"] = traceback.format_exc().splitlines()
    finally:
        if failure:
            report.update(outcome="failed", error=failure)
        if runner is not None:
            try:
                if original_guard is not None:
                    original_guard(); report["original_preserved"] = True
                report.update(original_head=final.bootstrap_git(origin, "rev-parse", "HEAD"),
                              original_status=final.bootstrap_git(origin, "status", "--short"))
            except Exception as error:
                report.update(outcome="failed", preservation_error=str(error), original_preserved=False)
            if directory is not None:
                report["return_file"] = str(directory / "ROUTINE_RETURN.txt")
                try:
                    module.atomic_write(Path(report["return_file"]), final.encoded(report), 0o600)
                except Exception as error:
                    report.update(outcome="failed", finalization_error=str(error))
            if runner.log:
                runner.log.close()
        for lock in reversed(locks):
            lock.close()
        print("BEGIN_ROUTINE_RETURN"); print(json.dumps(report, indent=2)); print("END_ROUTINE_RETURN")
        if report.get("return_file"):
            print("Return file: " + report["return_file"])
    return 0 if report["outcome"] == "passed" else 1


def main():
    parser = argparse.ArgumentParser(description="Install and freshly verify the routine guarded randomized command.")
    parser.add_argument("--repo", type=Path, default=Path.home() / "bitcoin-rs")
    args = parser.parse_args()
    return run_install(args.repo.expanduser().resolve(), Path(__file__).resolve())


if __name__ == "__main__":
    sys.exit(main())
