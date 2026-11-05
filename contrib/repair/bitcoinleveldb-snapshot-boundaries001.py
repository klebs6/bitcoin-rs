#!/usr/bin/env python3
"""Add snapshot dispatch boundary tests, commit them, then remeasure coverage."""
import argparse
import datetime as dt
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import traceback
import uuid

BASE = "3fd7819012a56f795b5b950bd4fc52a29f7fe469"
REPAIR = "snapshot-diagnostic-dispatch001"
WORKFLOW = "snapshot-dispatch-boundaries001"
REVISION = "002"
PREVIOUS_SCRIPT_SHA256 = "9c27cd42e6bfafc6a2290deac19decaffee0ef7d0bad4ff8335bf26dab9814dd"
PREVIOUS_TEST_SHA256 = "bb515ac68ef8e61b308778f00e9c3a8b3d252b07b301a17bff0824e897730df2"
BRANCH = "repair/leveldb-snapshot-diagnostic-dispatch"
COVERAGE_RUNNER = "contrib/repair/bitcoinleveldb-snapshot-coverage001.py"
COVERAGE_RUNNER_SHA256 = "47cae6f1df4354bfda2da86a5294009d08628830d33e81b4efcdef6737dd960e"
REPAIR_RUNNER = "contrib/repair/bitcoinleveldb-snapshot-dispatch001.py"
REPAIR_RUNNER_SHA256 = "454cb0e7ee58985af5a0fbecc2234dad280999b4270c5dc7caacde88d3fe9979"
TEST_PATH = "bitcoinleveldbt-snapshot/tests/snapshot_dispatch_boundary_contracts.rs"
REPLAY_PATH = "contrib/repair/bitcoinleveldb-snapshot-boundaries001.py"
REPORT_PATH = "doc/repair/snapshot-diagnostic-dispatch001-coverage.json"
TEST_PATHS = [TEST_PATH, REPLAY_PATH]
ROOTS = ["bitcoinleveldb-modeldb/src", "bitcoinleveldbt-snapshot/src"]
FOCUSED = ["cargo", "test", "--locked", "-p", "bitcoinleveldbt-snapshot",
           "--test", "snapshot_dispatch_boundary_contracts", "--", "--test-threads=1"]
SUITE = ["cargo", "test", "--locked", "-p", "bitcoinleveldb-modeldb", "-p",
         "bitcoinleveldbt-snapshot", "--", "--test-threads=1"]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def json_bytes(value):
    return (json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n").encode()


def git_env():
    env = dict(os.environ)
    for key in ("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR",
                "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_PREFIX", "GIT_NAMESPACE"):
        env.pop(key, None)
    env["GIT_OPTIONAL_LOCKS"] = "0"
    return env


def bootstrap_git(repo, *args):
    result = subprocess.run(["git", *args], cwd=repo, env=git_env(), capture_output=True)
    if result.returncode:
        raise RuntimeError(result.stderr.decode(errors="replace"))
    return result.stdout.decode().strip()


def trusted_coverage(worktree):
    path = worktree / COVERAGE_RUNNER
    if digest(path.read_bytes()) != COVERAGE_RUNNER_SHA256:
        raise RuntimeError("The retained coverage runner differs from the accepted Coverage001.")
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("trusted_snapshot_coverage001", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def safe_path(root, name):
    path = root / name
    if not path.resolve().is_relative_to(root.resolve()):
        raise RuntimeError("Workflow path leaves the worktree: " + name)
    for parent in [path, *path.parents]:
        if parent == root:
            break
        if parent.is_symlink():
            raise RuntimeError("Workflow path contains a symlink: " + str(parent))
    return path


def blob(runner, revision, name, required=True):
    result = runner.capture(["git", "show", revision + ":" + name], check=False)
    if result.returncode:
        if required:
            raise RuntimeError("Required Git blob missing: " + revision + ":" + name)
        return None
    return result.stdout


def exact_commit(runner, commit, parent, files):
    return (runner.git("show", "-s", "--format=%P", commit).split() == [parent]
            and runner.names("diff", "--name-only", parent, commit) == set(files)
            and all(blob(runner, commit, name, False) == value for name, value in files.items()))


def guard(runner, head, allowed, choices, manifest):
    runner.guard_git(allowed, head)
    untracked = runner.names("ls-files", "--others", "--exclude-standard")
    if untracked - set(allowed):
        raise RuntimeError("Unexpected untracked files retained: " + repr(sorted(untracked - set(allowed))))
    actual = {name: digest((runner.repo / name).read_bytes()) for name in manifest}
    if actual != manifest:
        raise RuntimeError("Production source changed; no boundary/coverage checkpoint was accepted.")
    inventory = {name for name in runner.names("ls-files", "--", *ROOTS) if name.endswith(".rs")}
    if inventory != set(manifest):
        raise RuntimeError("The declared production source inventory changed.")
    for name, accepted in choices.items():
        path = safe_path(runner.repo, name)
        if path.exists() and path.read_bytes() not in accepted:
            raise RuntimeError("Unexpected workflow content was retained: " + name)
        staged = blob(runner, "", name, False)
        if staged is not None and staged not in accepted:
            raise RuntimeError("Unexpected staged workflow content was retained: " + name)
    if not runner.origin_preserved(runner.origin_snapshot()):
        raise RuntimeError("Original checkout changed; no changes were discarded.")


def commit_files(runner, module, state, state_path, parent, files, message, manifest, baseline_report):
    choices = {name: {data} for name, data in files.items()}
    if REPORT_PATH in files:
        choices[REPORT_PATH].add(baseline_report)
    guard(runner, parent, list(files), choices, manifest)
    for name, data in files.items():
        module.atomic_write(safe_path(runner.repo, name), data, 0o755 if name == REPLAY_PATH else 0o644)
    runner.git("diff", "--check")
    runner.git("add", "--", *files)
    guard(runner, parent, list(files), {name: {data} for name, data in files.items()}, manifest)
    message_path = runner.directory / ("commit-" + state["phase"] + ".txt")
    message_path.write_text(message)
    status = runner.stream(["git", "commit", "--only", "--file", str(message_path), "--", *files])
    if status:
        raise RuntimeError("Commit failed; validation and generated files retained for retry.")
    checkpoint = runner.git("rev-parse", "HEAD")
    if not exact_commit(runner, checkpoint, parent, files):
        state["blocked_checkpoint"] = checkpoint
        runner.save_state(state_path, state)
        raise RuntimeError("Commit parent, paths or blobs differ from the validated checkpoint.")
    for name, data in files.items():
        if safe_path(runner.repo, name).read_bytes() != data:
            state["blocked_checkpoint"] = checkpoint
            runner.save_state(state_path, state)
            raise RuntimeError("A commit hook changed validated workflow content.")
    guard(runner, checkpoint, [], {name: {data} for name, data in files.items()}, manifest)
    return checkpoint


def count_delta(before, after):
    result = {}
    for metric in ("lines", "functions", "regions", "branches"):
        left, right = before["metrics"][metric], after["metrics"][metric]
        result[metric] = {key: right[key] - left[key] for key in ("count", "covered", "uncovered")}
        result[metric]["percent_points"] = (right["percent"] - left["percent"]
                                            if right["percent"] is not None and left["percent"] is not None else None)
    return result


def upgrade_failed_import_state(runner, module, metadata, state_path, state, identity, files):
    """Correct only the recognized uncommitted macro-import failure."""
    allowed_scripts = {PREVIOUS_SCRIPT_SHA256, identity["script_sha256"]}
    allowed_tests = {PREVIOUS_TEST_SHA256, identity["test_sha256"]}
    if (state.get("script_sha256") not in allowed_scripts
            or state.get("test_sha256") not in allowed_tests
            or state.get("normal_validated") or state.get("test_checkpoint")
            or state.get("coverage_validated") or state.get("coverage_checkpoint")
            or state.get("blocked_checkpoint")):
        raise RuntimeError("Boundary journal is not the known uncommitted import failure.")
    for key, value in identity.items():
        if key not in {"script_sha256", "test_sha256", "script_revision"} and state.get(key) != value:
            raise RuntimeError("Failed boundary journal source/baseline differs from this revision.")
    if state.get("phase") != "tests_pending":
        raise RuntimeError("Boundary workflow advanced beyond the recognized pre-test checkpoint.")
    previous_results = state.get("normal_tests", {})
    if (set(previous_results) - {"focused"}
            or ("focused" in previous_results and previous_results["focused"].get("exit_code") != 101)):
        raise RuntimeError("Boundary validation results differ from the retained failed compile.")
    if runner.git("rev-parse", "HEAD") != BASE:
        raise RuntimeError("Boundary branch advanced; import fix was not applied.")
    if runner.names("diff", "--name-only") or runner.names("diff", "--cached", "--name-only"):
        raise RuntimeError("Tracked/staged worktree edits were preserved; import migration stopped.")
    if runner.names("ls-files", "--others", "--exclude-standard") != set(TEST_PATHS):
        raise RuntimeError("Unexpected untracked files were preserved; import migration stopped.")
    actual_test = safe_path(runner.repo, TEST_PATH).read_bytes()
    actual_script = safe_path(runner.repo, REPLAY_PATH).read_bytes()
    if digest(actual_test) not in allowed_tests or digest(actual_script) not in allowed_scripts:
        raise RuntimeError("Retained boundary test/replay differs from the recognized predecessor or successor.")
    choices = {TEST_PATH: {actual_test, files[TEST_PATH]}, REPLAY_PATH: {actual_script, files[REPLAY_PATH]}}
    guard(runner, BASE, TEST_PATHS, choices, identity["source_manifest"])
    backups = [
        (metadata / "boundary-state-before-revision002.json", state_path.read_bytes(), "state"),
        (metadata / "boundary-tests-before-revision002.rs", actual_test, "test"),
        (metadata / "boundary-replay-before-revision002.py", actual_script, "script"),
    ]
    # Predecessor backups precede all writes, so interrupted upgrades can roll forward.
    for path, current, kind in backups:
        if path.exists():
            saved = path.read_bytes()
        else:
            if kind == "state":
                prior = json.loads(current)
                valid = (prior.get("script_sha256") == PREVIOUS_SCRIPT_SHA256
                         and prior.get("test_sha256") == PREVIOUS_TEST_SHA256)
            else:
                valid = digest(current) == (PREVIOUS_TEST_SHA256 if kind == "test" else PREVIOUS_SCRIPT_SHA256)
            if not valid:
                raise RuntimeError("Partial import upgrade lacks its predecessor backup; files retained.")
            module.atomic_write(path, current, 0o600)
            saved = current
        if kind == "state":
            prior = json.loads(saved)
            valid = (prior.get("workflow") == WORKFLOW and prior.get("baseline") == BASE
                     and prior.get("script_sha256") == PREVIOUS_SCRIPT_SHA256
                     and prior.get("test_sha256") == PREVIOUS_TEST_SHA256
                     and prior.get("source_manifest") == identity["source_manifest"])
        else:
            valid = digest(saved) == (PREVIOUS_TEST_SHA256 if kind == "test" else PREVIOUS_SCRIPT_SHA256)
        if not valid:
            raise RuntimeError("Existing predecessor backup differs from the known failed state.")
    module.atomic_write(safe_path(runner.repo, TEST_PATH), files[TEST_PATH], 0o644)
    module.atomic_write(safe_path(runner.repo, REPLAY_PATH), files[REPLAY_PATH], 0o755)
    state.update(identity, phase="tests_pending", normal_tests={}, normal_validated=False)
    runner.save_state(state_path, state)
    runner.report["journal_upgrade"] = {
        "from_script_sha256": PREVIOUS_SCRIPT_SHA256,
        "to_script_sha256": identity["script_sha256"],
        "test_change": "add the existing tracing_setup glob import; test bodies unchanged",
        "backups": [str(item[0]) for item in backups],
    }
    runner.say("Resuming the failed boundary compile with the tracing_setup import corrected.")


def run_boundaries(origin, script_path):
    runner = module = coverage = lock = None
    report = {"workflow": WORKFLOW, "script_revision": REVISION, "outcome": "failed", "phase": "preflight",
              "baseline": BASE, "tests": {}, "merge": "pending", "tag": "pending"}
    failure = None
    try:
        if Path(bootstrap_git(origin, "rev-parse", "--show-toplevel")).resolve() != origin:
            raise RuntimeError("--repo must identify the original checkout root.")
        common = Path(bootstrap_git(origin, "rev-parse", "--path-format=absolute", "--git-common-dir"))
        metadata = common / "codex-repairs" / REPAIR
        lock = (metadata / "run.lock").open("a+")
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        layout = json.loads((metadata / "worktree.json").read_text())
        repair_state = json.loads((metadata / "state.json").read_text())
        old_state = json.loads((metadata / "coverage-state001.json").read_text())
        worktree = Path(layout["path"])
        if (layout.get("repair") != REPAIR or layout.get("origin") != str(origin)
                or layout.get("branch") != BRANCH or layout.get("revision") != "004"
                or layout.get("script_sha256") != REPAIR_RUNNER_SHA256
                or repair_state.get("script_sha256") != REPAIR_RUNNER_SHA256
                or layout.get("baseline") != repair_state.get("baseline")
                or repair_state.get("blocked_checkpoint") or old_state.get("blocked_checkpoint")):
            raise RuntimeError("Existing repair journals differ from the accepted checkpoint.")
        registration = [dict(line.split(" ", 1) for line in block.splitlines() if " " in line)
                        for block in bootstrap_git(origin, "worktree", "list", "--porcelain").split("\n\n")]
        if (not any(item.get("worktree") == str(worktree) and item.get("branch") == "refs/heads/" + BRANCH
                    for item in registration) or not worktree.is_dir() or worktree.is_symlink()):
            raise RuntimeError("Accepted repair worktree registration changed.")
        coverage = trusted_coverage(worktree)
        if (old_state.get("workflow") != "snapshot-dispatch-coverage001"
                or old_state.get("script_sha256") != COVERAGE_RUNNER_SHA256
                or old_state.get("checkpoint") != BASE or old_state.get("validated") is not True
                or old_state.get("tested_commit") != coverage.EXPECTED_COMMIT
                or repair_state.get("checkpoint") != coverage.EXPECTED_COMMIT):
            raise RuntimeError("Coverage/repair journals do not identify the accepted baseline.")
        module, runner = coverage.load_repair_runner(worktree, origin)
        runner.repo = worktree
        stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
        directory = metadata / "boundary-runs" / (stamp + "-" + uuid.uuid4().hex[:8])
        directory.mkdir(parents=True)
        runner.directory = directory
        runner.log = (directory / "run.log").open("w", buffering=1)
        runner.report = report
        report.update(origin_repository=str(origin), worktree=str(worktree), run_directory=str(directory),
                      origin_before=runner.origin_snapshot(), script_sha256=digest(script_path.read_bytes()))
        for name, expected in ((COVERAGE_RUNNER, COVERAGE_RUNNER_SHA256), (REPAIR_RUNNER, REPAIR_RUNNER_SHA256)):
            if digest(blob(runner, BASE, name)) != expected or digest((worktree / name).read_bytes()) != expected:
                raise RuntimeError("Trusted runner differs from its accepted committed blob: " + name)
        baseline_report = blob(runner, BASE, REPORT_PATH)
        baseline_measurement = json.loads(baseline_report)
        if (baseline_measurement != old_state.get("measurement")
                or baseline_measurement.get("raw_json_sha256") != old_state.get("raw_sha256")):
            raise RuntimeError("Baseline report differs from the accepted coverage state.")
        old_raw_path = Path(old_state["raw_path"])
        if digest(old_raw_path.read_bytes()) != old_state["raw_sha256"]:
            raise RuntimeError("Retained baseline raw coverage failed its hash check.")
        tracked = sorted(name for name in runner.names("ls-files", "--", *ROOTS) if name.endswith(".rs"))
        manifest = {name: digest((worktree / name).read_bytes()) for name in tracked}
        if manifest != old_state["source_manifest"]:
            raise RuntimeError("Production source differs from the measured coverage baseline.")
        baseline_views = reconstruct_source_gaps(json.loads(old_raw_path.read_bytes()), worktree, tracked)
        report.update(metrics_before=baseline_measurement["coverage"]["metrics"],
                      baseline_source_views=baseline_views, source_manifest=manifest)
        state_path = metadata / "boundary-tests001-state.json"
        state = json.loads(state_path.read_text()) if state_path.exists() else None
        files = {TEST_PATH: TEST_BYTES, REPLAY_PATH: script_path.read_bytes()}
        identity = {"workflow": WORKFLOW, "baseline": BASE, "script_sha256": digest(files[REPLAY_PATH]),
                    "script_revision": REVISION,
                    "test_sha256": digest(TEST_BYTES), "source_manifest": manifest,
                    "baseline_report_sha256": digest(baseline_report), "baseline_raw_sha256": old_state["raw_sha256"]}
        if state:
            if (state.get("script_sha256") == PREVIOUS_SCRIPT_SHA256
                    or state.get("test_sha256") == PREVIOUS_TEST_SHA256):
                upgrade_failed_import_state(runner, module, metadata, state_path, state, identity, files)
            if any(state.get(key) != value for key, value in identity.items()):
                raise RuntimeError("Boundary journal differs from the accepted script/source/baseline.")
            if state.get("blocked_checkpoint"):
                raise RuntimeError("Altered boundary checkpoint needs inspection: " + state["blocked_checkpoint"])
        else:
            if any(blob(runner, BASE, name, False) is not None for name in files):
                raise RuntimeError("A new boundary-test path already exists in the baseline.")
            state = dict(identity, phase="tests_pending", normal_tests={})
            runner.save_state(state_path, state)
        report["tests"] = state["normal_tests"]
        report["normal_validation_reused"] = bool(state.get("normal_validated"))
        report["coverage_measurement_reused"] = bool(state.get("coverage_validated"))
        current = runner.git("rev-parse", "HEAD")
        if not state.get("test_checkpoint") and current != BASE:
            if not state.get("normal_validated") or not exact_commit(runner, current, BASE, files):
                raise RuntimeError("Branch advanced beyond the known boundary baseline.")
            state.update(test_checkpoint=current, phase="test_checkpoint")
            runner.save_state(state_path, state)
        test_checkpoint = state.get("test_checkpoint")
        if test_checkpoint and not exact_commit(runner, test_checkpoint, BASE, files):
            raise RuntimeError("Recorded test checkpoint parent, paths or blobs changed.")
        if not test_checkpoint:
            guard(runner, BASE, TEST_PATHS, {name: {value} for name, value in files.items()}, manifest)
            if not state.get("normal_validated"):
                for name, data in files.items():
                    module.atomic_write(safe_path(worktree, name), data, 0o755 if name == REPLAY_PATH else 0o644)
                nixpkgs = module.normalize_nixpkgs(repair_state["nixpkgs"])
                shell = ('with import ' + json.dumps(nixpkgs["outPath"]) + ' {}; mkShell { '
                         'nativeBuildInputs = [ pkg-config ]; buildInputs = [ libevent openssl clang ]; '
                         'LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib"; }')
                prefix = ["nix", "develop", "--impure", "--expr", shell, "--command", "env",
                          "GIT_CONFIG_COUNT=1", "GIT_CONFIG_KEY_0=url.https://github.com/.insteadOf",
                          "GIT_CONFIG_VALUE_0=git@github.com:", "CARGO_NET_GIT_FETCH_WITH_CLI=true",
                          "CARGO_BUILD_JOBS=4", "RUST_BACKTRACE=1", "CARGO_TARGET_DIR=" + str(origin / "target")]
                for label, command in (("focused", FOCUSED), ("crate_suites", SUITE)):
                    report["phase"] = label
                    status = runner.stream(prefix + command)
                    state["normal_tests"][label] = {"command": command, "exit_code": status}
                    runner.save_state(state_path, state)
                    if status:
                        raise RuntimeError(label + " failed; edits retained, no test checkpoint committed.")
                    guard(runner, BASE, TEST_PATHS, {name: {value} for name, value in files.items()}, manifest)
                state.update(normal_validated=True, phase="normal_validated")
                runner.save_state(state_path, state)
            report["phase"] = "test_checkpoint"
            message = ("leveldb: exercise snapshot dispatch boundary contracts\n\n"
                       "Add integration tests for unsupported dispatch, sequence and clone boundaries.\n"
                       "Preserve production sources and record the replay procedure.\n\n"
                       "Boundary-Workflow: " + WORKFLOW + "\nBaseline: " + BASE + "\n"
                       "Validation: PASS " + " ".join(FOCUSED) + "\n"
                       "Validation: PASS " + " ".join(SUITE) + "\n")
            test_checkpoint = commit_files(runner, module, state, state_path, BASE, files, message, manifest, baseline_report)
            state.update(test_checkpoint=test_checkpoint, phase="test_checkpoint")
            runner.save_state(state_path, state)
        report.update(test_checkpoint=test_checkpoint, tested_commit=test_checkpoint, tests=state["normal_tests"])
        current = runner.git("rev-parse", "HEAD")
        report_bytes = json_bytes(state["measurement"]) if state.get("coverage_validated") else None
        if current not in {test_checkpoint, state.get("coverage_checkpoint")}:
            if not report_bytes or not exact_commit(runner, current, test_checkpoint, {REPORT_PATH: report_bytes}):
                raise RuntimeError("Branch advanced beyond its known test/coverage checkpoints.")
            state.update(coverage_checkpoint=current, phase="coverage_checkpoint")
            runner.save_state(state_path, state)
        if state.get("coverage_checkpoint") and not exact_commit(
                runner, state["coverage_checkpoint"], test_checkpoint, {REPORT_PATH: report_bytes}):
            raise RuntimeError("Recorded coverage checkpoint content changed.")
        if not state.get("coverage_validated"):
            guard(runner, test_checkpoint, [], {}, manifest)
            report["phase"] = "coverage_tools"
            nixpkgs = module.normalize_nixpkgs(repair_state["nixpkgs"])
            tools = coverage.prepare_coverage_tools(runner, nixpkgs["outPath"], directory)
            guard(runner, test_checkpoint, [], {}, manifest)
            target = origin / "target" / "snapshot-dispatch-coverage001"
            if target.is_symlink() or target.resolve() == (origin / "target").resolve():
                raise RuntimeError("Coverage target must remain distinct from the ordinary target.")
            raw_path = directory / "llvm-coverage-full.json"
            command = coverage.coverage_command(tools, target.resolve(), raw_path.resolve())
            report.update(phase="instrumented_tests", coverage_command=command, coverage_target=str(target))
            status = runner.stream(command)
            report["instrumented_test_exit_code"] = status
            if status:
                raise RuntimeError("Instrumented tests failed; the ordinary test checkpoint is retained.")
            guard(runner, test_checkpoint, [], {}, manifest)
            raw = json.loads(raw_path.read_bytes())
            summary = coverage.summarize_coverage(raw, worktree, tracked)
            views = reconstruct_source_gaps(raw, worktree, tracked)
            measured = {"workflow": WORKFLOW, "tested_commit": test_checkpoint, "source_manifest": manifest,
                        "nixpkgs": nixpkgs, "compiler": tools["compiler"], "tool_versions": tools["tool_versions"],
                        "branch_probe_passed": tools["branch_supported"], "instrumented_test_exit_code": status,
                        "coverage": summary, "source_views": views, "raw_json_sha256": digest(raw_path.read_bytes()),
                        "baseline": {"coverage_checkpoint": BASE, "tested_commit": baseline_measurement["tested_commit"],
                                     "raw_json_sha256": old_state["raw_sha256"], "coverage": baseline_measurement["coverage"],
                                     "source_views": baseline_views},
                        "coverage_count_delta": count_delta(baseline_measurement["coverage"], summary),
                        "merge": "pending", "tag": "pending"}
            state.update(coverage_validated=True, phase="coverage_validated", raw_path=str(raw_path),
                         raw_sha256=digest(raw_path.read_bytes()), measurement=measured,
                         report_sha256=digest(json_bytes(measured)))
            runner.save_state(state_path, state)
        raw_path = Path(state["raw_path"])
        if digest(raw_path.read_bytes()) != state["raw_sha256"]:
            raise RuntimeError("Retained boundary coverage JSON failed its validated hash check.")
        measured = state["measurement"]
        if (measured.get("tested_commit") != test_checkpoint or measured.get("source_manifest") != manifest
                or measured.get("raw_json_sha256") != state["raw_sha256"]
                or coverage.summarize_coverage(json.loads(raw_path.read_bytes()), worktree, tracked) != measured["coverage"]
                or reconstruct_source_gaps(json.loads(raw_path.read_bytes()), worktree, tracked) != measured["source_views"]):
            raise RuntimeError("Retained coverage summary/source views differ from the verified raw export.")
        report_bytes = json_bytes(measured)
        if digest(report_bytes) != state["report_sha256"]:
            raise RuntimeError("Retained coverage report differs from its validated bytes.")
        report.update(measurement=measured, metrics_before=baseline_measurement["coverage"]["metrics"],
                      metrics_after=measured["coverage"]["metrics"], coverage_count_delta=measured["coverage_count_delta"],
                      source_views=measured["source_views"], baseline_source_views=baseline_views,
                      instrumented_test_exit_code=measured["instrumented_test_exit_code"], raw_coverage_json=str(raw_path))
        if not state.get("coverage_checkpoint"):
            report["phase"] = "coverage_checkpoint"
            state["phase"] = "coverage_validated"
            message = ("leveldb: record boundary-test coverage measurement\n\n"
                       "Record measured counts, unmapped sources, source views and deltas against the prior baseline.\n"
                       "Keep production sources unchanged; merge/tag qualification remains pending.\n\n"
                       "Boundary-Workflow: " + WORKFLOW + "\nTested-Commit: " + test_checkpoint + "\n"
                       "Raw-JSON-SHA256: " + state["raw_sha256"] + "\n")
            checkpoint = commit_files(runner, module, state, state_path, test_checkpoint,
                                      {REPORT_PATH: report_bytes}, message, manifest, baseline_report)
            state.update(coverage_checkpoint=checkpoint, phase="coverage_checkpoint")
            runner.save_state(state_path, state)
        guard(runner, state["coverage_checkpoint"], [], {REPORT_PATH: {report_bytes}}, manifest)
        report.update(outcome="passed", phase="complete", coverage_checkpoint=state["coverage_checkpoint"])
    except KeyboardInterrupt:
        failure = "Interrupted; checkpoints, source and validation evidence retained."
    except Exception as error:
        failure = type(error).__name__ + ": " + str(error)
        report["exception_traceback"] = traceback.format_exc().splitlines()
    finally:
        if failure:
            report.update(outcome="failed", error=failure)
        if runner and runner.log:
            try:
                if failure:
                    runner.say("STOP: " + failure)
                report["origin_after"] = runner.origin_snapshot()
                report["origin_tracked_state_preserved"] = runner.origin_preserved(report["origin_after"])
                if not report["origin_tracked_state_preserved"]:
                    report.update(outcome="failed", error="Original checkout changed; no work discarded.")
                report.update(final_head=runner.git("rev-parse", "HEAD"), git_status=runner.git("status", "--short"),
                              return_file=str(runner.directory / "BOUNDARY_RETURN.txt"))
                if failure:
                    report["failure_tail"] = (runner.directory / "run.log").read_text().splitlines()[-80:]
                module.atomic_write(Path(report["return_file"]), json_bytes(report))
            except Exception as error:
                report.update(outcome="failed", finalization_error=str(error))
            runner.log.close()
        if lock:
            lock.close()
        print("BEGIN_BOUNDARY_RETURN")
        print(json.dumps(report, ensure_ascii=False, indent=2))
        print("END_BOUNDARY_RETURN")
        if report.get("return_file"):
            print("Return file: " + report["return_file"])
    return 0 if report["outcome"] == "passed" else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.home() / "bitcoin-rs")
    args = parser.parse_args()
    return run_boundaries(args.repo.expanduser().resolve(), Path(__file__).resolve())


TEST_BYTES = b'use bitcoinleveldb_dbinterface::{DB, DBDelete, DBGetSnapshot, DBPut, DBReleaseSnapshot};\nuse bitcoinleveldb_key::{SequenceNumber, MAX_SEQUENCE_NUMBER};\nuse bitcoinleveldb_modeldb::{\n    ModelDB,\n    dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref,\n    dbtest_snapshot_sequence_from_snapshot_ref,\n    snapshot_read_arc_from_snapshot_ref,\n};\nuse bitcoinleveldb_options::{Options, WriteOptions};\nuse bitcoinleveldb_slice::Slice;\nuse bitcoinleveldb_snapshot::{Snapshot, SnapshotDispatchConcreteImplementationKind, SnapshotImpl};\nuse bitcoinleveldbt_dbtest::{\n    DBTest,\n    dbtest_fixture_put_owned_string_pair,\n    dbtest_fixture_run_across_option_configurations,\n};\nuse bitcoinleveldbt_snapshot::{\n    compare_iterators,\n    dbtest_read_options_from_snapshot_ref,\n    dbtest_snapshot_read_arc_from_snapshot_ptr,\n    dbtest_snapshot_read_arc_from_snapshot_ref,\n    snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot,\n};\nuse std::sync::Arc;\nuse traced_test::traced_test;\nuse tracing_setup::*;\n\nfn put_model_rows(db: &mut ModelDB, rows: &[(&str, &str)]) {\n    let options = WriteOptions::default();\n    for &(key, value) in rows {\n        let status = DBPut::put(db, &options, &Slice::from(key), &Slice::from(value));\n        assert!(status.is_ok(), "model put must succeed: {}", status.to_string());\n    }\n}\n\nfn require_snapshot_arc(value: Option<Arc<dyn Snapshot>>) -> Arc<dyn Snapshot> {\n    match value {\n        Some(snapshot) => snapshot,\n        None => panic!("a supported non-null snapshot must produce a read owner"),\n    }\n}\n\n#[traced_test]\nfn snapshot_pointer_bridge_accepts_typed_null_without_dereferencing_it() {\n    let null_concrete: *const SnapshotImpl = core::ptr::null();\n    let null_snapshot: *const dyn Snapshot = null_concrete;\n\n    assert!(dbtest_snapshot_read_arc_from_snapshot_ptr(null_snapshot).is_none());\n}\n\n#[traced_test]\nfn snapshot_pointer_bridge_preserves_sequence_after_the_source_scope_ends() {\n    for sequence in [0_u64, 23_u64, MAX_SEQUENCE_NUMBER] {\n        let retained = {\n            let source = SnapshotImpl::new(sequence);\n            let pointer: *const dyn Snapshot = &source;\n            require_snapshot_arc(dbtest_snapshot_read_arc_from_snapshot_ptr(pointer))\n        };\n\n        assert_eq!(\n            retained.snapshot_runtime_implementation_kind(),\n            SnapshotDispatchConcreteImplementationKind::SnapshotImpl\n        );\n        assert_eq!(\n            retained.snapshot_sequence_number_for_read_reconstruction(),\n            Some(sequence)\n        );\n    }\n}\n\n#[traced_test]\nfn model_snapshot_pointer_bridge_retains_contents_after_release_and_live_mutation() {\n    let options = Options::default();\n    let mut db = ModelDB::new(&options);\n    put_model_rows(&mut db, &[("a", "captured-a"), ("b", "captured-b")]);\n\n    let original: Box<dyn Snapshot> = DBGetSnapshot::get_snapshot(&mut db);\n    let pointer: *const dyn Snapshot = &*original;\n    let retained = require_snapshot_arc(dbtest_snapshot_read_arc_from_snapshot_ptr(pointer));\n    DBReleaseSnapshot::release_snapshot(&mut db, original);\n\n    put_model_rows(&mut db, &[("a", "live-a"), ("c", "live-c")]);\n    let delete_status = DBDelete::delete(\n        &mut db,\n        &WriteOptions::default(),\n        &Slice::from("b"),\n    );\n    assert!(delete_status.is_ok(), "live delete must succeed: {}", delete_status.to_string());\n\n    assert_eq!(\n        retained.snapshot_runtime_implementation_kind(),\n        SnapshotDispatchConcreteImplementationKind::ModelSnapshot\n    );\n    assert_eq!(retained.snapshot_sequence_number_for_read_reconstruction(), None);\n\n    let captured = snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(\n        &mut db,\n        Some(retained),\n    );\n    let live = snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(&mut db, None);\n\n    assert_eq!(\n        captured,\n        vec![\n            (String::from("a"), String::from("captured-a")),\n            (String::from("b"), String::from("captured-b")),\n        ]\n    );\n    assert_eq!(\n        live,\n        vec![\n            (String::from("a"), String::from("live-a")),\n            (String::from("c"), String::from("live-c")),\n        ]\n    );\n}\n\n#[repr(C)]\nstruct ForwardingSnapshot {\n    marker: [u8; 64],\n    inner: SnapshotImpl,\n}\n\nimpl Snapshot for ForwardingSnapshot {\n    fn snapshot_runtime_implementation_kind(&self) -> SnapshotDispatchConcreteImplementationKind {\n        self.inner.snapshot_runtime_implementation_kind()\n    }\n\n    fn snapshot_sequence_number_for_read_reconstruction(&self) -> Option<SequenceNumber> {\n        self.inner.snapshot_sequence_number_for_read_reconstruction()\n    }\n\n    fn snapshot_read_arc_clone(&self) -> Option<Arc<dyn Snapshot>> {\n        self.inner.snapshot_read_arc_clone()\n    }\n}\n\n#[traced_test]\nfn forwarding_snapshot_observation_uses_semantic_methods_without_a_layout_cast() {\n    let snapshot = ForwardingSnapshot {\n        marker: [0xa5; 64],\n        inner: SnapshotImpl::new(41),\n    };\n    let outer_address = &snapshot as *const ForwardingSnapshot as usize;\n    let inner_address = &snapshot.inner as *const SnapshotImpl as usize;\n    assert_ne!(outer_address, inner_address);\n\n    let observation =\n        dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(&snapshot);\n\n    assert_eq!(*observation.snapshot_data_ptr_value(), outer_address);\n    assert_eq!(\n        *observation.implementation_kind(),\n        SnapshotDispatchConcreteImplementationKind::SnapshotImpl\n    );\n    assert_eq!(*observation.snapshot_sequence_number_hint(), Some(41));\n}\n\nstruct UnsupportedSnapshot;\n\nimpl Snapshot for UnsupportedSnapshot {\n    fn snapshot_runtime_implementation_kind(&self) -> SnapshotDispatchConcreteImplementationKind {\n        SnapshotDispatchConcreteImplementationKind::Unsupported\n    }\n\n    fn snapshot_sequence_number_for_read_reconstruction(&self) -> Option<SequenceNumber> {\n        None\n    }\n\n    fn snapshot_read_arc_clone(&self) -> Option<Arc<dyn Snapshot>> {\n        None\n    }\n}\n\nfn assert_rejection_message<Operation, Value>(expected: &str, operation: Operation)\nwhere\n    Operation: FnOnce() -> Value + std::panic::UnwindSafe,\n{\n    // Each caller captures only a shared reference to an immutable capability\n    // fixture. Rust checks the closure\'s unwind-safety requirement directly.\n    let failure = std::panic::catch_unwind(operation);\n    let payload = match failure {\n        Err(payload) => payload,\n        Ok(_) => panic!("unsupported capability must be rejected: {}", expected),\n    };\n    let actual = match payload.downcast_ref::<&str>() {\n        Some(message) => *message,\n        None => match payload.downcast_ref::<String>() {\n            Some(message) => message.as_str(),\n            None => panic!("rejection must carry its documented string message"),\n        },\n    };\n    assert_eq!(actual, expected);\n}\n\n#[traced_test]\nfn snapshot_reference_bridge_rejects_an_explicitly_unsupported_capability() {\n    let snapshot = UnsupportedSnapshot;\n    assert_rejection_message(\n        "dbtest_snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation",\n        || dbtest_snapshot_read_arc_from_snapshot_ref(&snapshot),\n    );\n}\n\n#[traced_test]\nfn snapshot_pointer_bridge_rejects_a_valid_unsupported_object() {\n    let snapshot = UnsupportedSnapshot;\n    assert_rejection_message(\n        "dbtest_snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation",\n        || {\n            let pointer: *const dyn Snapshot = &snapshot;\n            dbtest_snapshot_read_arc_from_snapshot_ptr(pointer)\n        },\n    );\n}\n\n#[traced_test]\nfn snapshot_read_options_reject_an_explicitly_unsupported_capability() {\n    let snapshot = UnsupportedSnapshot;\n    assert_rejection_message(\n        "dbtest_snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation",\n        || dbtest_read_options_from_snapshot_ref(&snapshot),\n    );\n}\n\n#[traced_test]\nfn modeldb_snapshot_arc_bridge_rejects_an_explicitly_unsupported_capability() {\n    let snapshot = UnsupportedSnapshot;\n    assert_rejection_message(\n        "snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation",\n        || snapshot_read_arc_from_snapshot_ref(&snapshot),\n    );\n}\n\n#[traced_test]\nfn snapshot_sequence_bridge_rejects_an_explicitly_unsupported_capability() {\n    let snapshot = UnsupportedSnapshot;\n    assert_rejection_message(\n        "dbtest_snapshot_sequence_from_snapshot_ref: snapshot does not provide a sequence reconstruction boundary",\n        || dbtest_snapshot_sequence_from_snapshot_ref(&snapshot),\n    );\n}\n\nfn compare_model_with_filesystem_rows(\n    model_rows: &[(&str, &str)],\n    filesystem_rows: &[(&str, &str)],\n    expected: bool,\n) {\n    let mut body = |dbtest: &mut DBTest| {\n        let options = Options::default();\n        let mut model = ModelDB::new(&options);\n        put_model_rows(&mut model, model_rows);\n        for &(key, value) in filesystem_rows {\n            let key_owned = String::from(key);\n            let value_owned = String::from(value);\n            let status =\n                dbtest_fixture_put_owned_string_pair(dbtest, &key_owned, &value_owned);\n            assert!(status.is_ok(), "filesystem put must succeed: {}", status.to_string());\n        }\n\n        // The pointers refer to distinct, live owners throughout comparison.\n        // compare_iterators owns and destroys each constructed iterator itself.\n        let model_pointer: *mut dyn DB = &mut model;\n        let filesystem_pointer: *mut dyn DB = dbtest.dbfull();\n        assert_eq!(\n            compare_iterators(0, model_pointer, filesystem_pointer, None, None),\n            expected\n        );\n    };\n\n    dbtest_fixture_run_across_option_configurations(&mut body);\n}\n\n#[traced_test]\nfn iterator_comparison_accepts_equal_model_and_filesystem_rows() {\n    compare_model_with_filesystem_rows(\n        &[("a", "one"), ("b", "two")],\n        &[("a", "one"), ("b", "two")],\n        true,\n    );\n}\n\n#[traced_test]\nfn iterator_comparison_rejects_different_keys_at_equal_length() {\n    compare_model_with_filesystem_rows(&[("a", "same")], &[("b", "same")], false);\n}\n\n#[traced_test]\nfn iterator_comparison_rejects_different_values_for_the_same_key() {\n    compare_model_with_filesystem_rows(&[("a", "model")], &[("a", "filesystem")], false);\n}\n\n#[traced_test]\nfn iterator_comparison_rejects_a_filesystem_proper_prefix() {\n    compare_model_with_filesystem_rows(\n        &[("a", "one"), ("b", "two")],\n        &[("a", "one")],\n        false,\n    );\n}\n\n#[traced_test]\nfn iterator_comparison_rejects_a_model_proper_prefix() {\n    compare_model_with_filesystem_rows(\n        &[("a", "one")],\n        &[("a", "one"), ("b", "two")],\n        false,\n    );\n}\n'

"""LLVM 22 source-level views from a full retained JSON export.

These diagnostic views do not replace LLVM's authoritative summary counts.
In particular exported branch rows omit folded-side flags, so absent outcome
observations cannot be equated with the summary's uncovered branch count.
"""
from pathlib import Path


def _integer(value, label):
    if isinstance(value, bool) or not isinstance(value, int) or value < 0:
        raise ValueError(label + " must be a nonnegative integer")
    return value


def _domain_path(filename, worktree, tracked):
    if not isinstance(filename, str) or not filename:
        raise ValueError("invalid coverage filename")
    path = Path(filename)
    path = (path if path.is_absolute() else worktree / path).resolve()
    try:
        relative = path.relative_to(worktree).as_posix()
    except ValueError:
        return None
    return relative if relative in tracked else None


def line_observations(segments):
    """Mirror LLVM22 LineCoverageStats on already merged per-file segments."""
    if not isinstance(segments, list):
        raise ValueError("segments must be an array")
    parsed = []
    for record in segments:
        if not isinstance(record, list) or len(record) != 6:
            raise ValueError("invalid coverage segment")
        line, column, count = [_integer(value, "segment coordinate/count") for value in record[:3]]
        if not line or not column or any(type(value) is not bool for value in record[3:]):
            raise ValueError("invalid coverage segment coordinate/flag")
        parsed.append([line, column, count, *record[3:]])
    if any((left[0], left[1]) > (right[0], right[1]) for left, right in zip(parsed, parsed[1:])):
        raise ValueError("coverage segments are not sorted")
    if not parsed:
        return []
    result, cursor, wrapped = [], 0, None
    for line in range(parsed[0][0], parsed[-1][0] + 1):
        start = cursor
        while cursor < len(parsed) and parsed[cursor][0] == line:
            cursor += 1
        current = parsed[start:cursor]
        starts = [s for s in current if not s[5] and s[3] and s[4]]
        start_skipped = bool(current and not current[0][3] and current[0][4])
        mapped = (not start_skipped and ((wrapped is not None and wrapped[3]) or bool(starts)))
        mapped = mapped or any(s[4] and s[3] for s in current)
        if mapped:
            count = wrapped[2] if wrapped is not None else 0
            if starts:
                count = max([count, *[s[2] for s in starts]])
            result.append({"line": line, "count": count, "executed": count > 0})
        if current:
            wrapped = current[-1]
    return result


def reconstruct_source_gaps(raw, worktree, tracked_files):
    """Return merged source-line views, logical function groups and branch hints.

    Function grouping mirrors LLVM22 FunctionInstantiationSetCollector and
    findMainViewFileID; it does not use fragile mangled-name rewriting. Branch
    views combine observation booleans across rows with identical coordinates,
    retaining an explicit folded-side ambiguity. No source files are written.
    """
    worktree = Path(worktree).resolve()
    tracked = set(tracked_files)
    if not isinstance(raw, dict) or not isinstance(raw.get("data"), list) or not raw["data"]:
        raise ValueError("full LLVM data array required")
    files, groups = {}, {}
    for datum in raw["data"]:
        if not isinstance(datum, dict) or not isinstance(datum.get("files"), list):
            raise ValueError("LLVM file records required")
        for item in datum["files"]:
            relative = _domain_path(item.get("filename"), worktree, tracked)
            if relative is None:
                continue
            if relative in files and files[relative] != item:
                raise ValueError("conflicting file records: " + relative)
            files[relative] = item
        if not isinstance(datum.get("functions"), list):
            raise ValueError("full LLVM function records required")
        for function in datum["functions"]:
            names = function.get("filenames")
            regions = function.get("regions")
            if not isinstance(names, list) or not isinstance(regions, list):
                raise ValueError("invalid function filenames/regions")
            expanded = set()
            for region in regions:
                if not isinstance(region, list) or len(region) < 8:
                    raise ValueError("invalid function region")
                if region[7] == 1:
                    file_id = _integer(region[6], "expanded file ID")
                    if file_id >= len(names):
                        raise ValueError("expanded file ID out of bounds")
                    expanded.add(file_id)
            main = next((index for index in range(len(names)) if index not in expanded), None)
            if main is None:
                continue
            relative = _domain_path(names[main], worktree, tracked)
            if relative is None:
                continue
            first = next((region for region in regions if region[5] == main), None)
            if first is None:
                raise ValueError("function lacks a region in its main file")
            line, column = _integer(first[0], "function line"), _integer(first[1], "function column")
            count = _integer(function.get("count"), "function execution count")
            key = (relative, line, column)
            group = groups.setdefault(key, {"path": relative, "start_line": line, "start_column": column,
                                            "any_instantiation_executed": False, "instantiation_count": 0,
                                            "names": []})
            group["any_instantiation_executed"] |= count > 0
            group["instantiation_count"] += 1
            group["names"].append(function.get("name", "<unnamed>"))
    per_file, never_observed_branches = [], []
    for relative, item in sorted(files.items()):
        lines = line_observations(item.get("segments"))
        own_groups = [group for key, group in sorted(groups.items()) if key[0] == relative]
        reconstructed = {"count": len(own_groups),
                         "covered": sum(group["any_instantiation_executed"] for group in own_groups)}
        exported = item.get("summary", {}).get("functions", {})
        function_counts_match = (reconstructed["count"] == exported.get("count")
                                 and reconstructed["covered"] == exported.get("covered"))
        per_file.append({"path": relative, "merged_file_lines": {
            "count": len(lines), "covered": sum(line["executed"] for line in lines),
            "uncovered_lines": [line["line"] for line in lines if not line["executed"]]},
            "logical_functions_reconstructed": reconstructed,
            "logical_function_counts_match_export": function_counts_match,
            "unexecuted_function_groups": [group for group in own_groups if not group["any_instantiation_executed"]]})
        branch_groups = {}
        for branch in item.get("branches", []):
            if not isinstance(branch, list) or len(branch) < 6:
                raise ValueError("invalid branch row")
            coords = tuple(_integer(x, "branch coordinate") for x in branch[:4])
            true_count = _integer(branch[4], "branch true count")
            false_count = _integer(branch[5], "branch false count")
            group = branch_groups.setdefault(coords, {"path": relative, "start_line": coords[0],
                "start_column": coords[1], "end_line": coords[2], "end_column": coords[3],
                "any_true_observed": False, "any_false_observed": False, "record_count": 0})
            group["any_true_observed"] |= true_count > 0
            group["any_false_observed"] |= false_count > 0
            group["record_count"] += 1
        for coords, group in sorted(branch_groups.items()):
            absent = [name for name in ("true", "false") if not group["any_" + name + "_observed"]]
            if absent:
                group.update(unobserved_sides=absent,
                             folded_side_status="not exported; an unobserved side may be compiler-folded")
                never_observed_branches.append(group)
    return {"llvm_semantics_version": "22.1.5",
            "summary_counts_replaced": False,
            "merged_line_note": "LLVM getCoverageForFile segment view combines code-region counters across instantiations; file summary uses maxima of per-function instantiation summaries and can differ",
            "branch_note": "Coordinate-union observation view only; JSON omits TrueFolded/FalseFolded, so these are not exact authoritative uncovered branch sides",
            "function_note": "Groups use main definition file and first matching region start location, as LLVM does; mismatching reconstructed totals remain explicit",
            "per_file": per_file,
            "branch_coordinates_with_unobserved_sides": never_observed_branches,
            "unmapped_sources": sorted(tracked - files.keys())}


if __name__ == "__main__":
    sys.exit(main())
