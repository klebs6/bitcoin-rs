#!/usr/bin/env python3
"""Integrate the validated snapshot repair and create its scoped annotated tag."""
import argparse
import datetime as dt
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
import traceback
import uuid

ORIGINAL_BASE = "8b3ee6e525132bd620f6e7e5c4c9e90121039e29"
SOURCE_CHECKPOINT = "5cf353e3241349bc3332c192abb5ba6901b57a0f"
TEST_CHECKPOINT = "76e080b118eb24523cafda6c239d8ff9b18b1b21"
OLD_COVERAGE_CHECKPOINT = "3fd7819012a56f795b5b950bd4fc52a29f7fe469"
REPAIR = "snapshot-diagnostic-dispatch001"
WORKFLOW = "snapshot-dispatch-finalize001"
BRANCH = "repair/leveldb-snapshot-diagnostic-dispatch"
REPAIR_RUNNER = "contrib/repair/bitcoinleveldb-snapshot-dispatch001.py"
REPAIR_RUNNER_SHA256 = "454cb0e7ee58985af5a0fbecc2234dad280999b4270c5dc7caacde88d3fe9979"
COVERAGE_RUNNER = "contrib/repair/bitcoinleveldb-snapshot-coverage001.py"
COVERAGE_RUNNER_SHA256 = "47cae6f1df4354bfda2da86a5294009d08628830d33e81b4efcdef6737dd960e"
BOUNDARY_RUNNER = "contrib/repair/bitcoinleveldb-snapshot-boundaries001.py"
BOUNDARY_RUNNER_SHA256 = "6d8f85a512b2c87d06da1e6161df026165d1e170a91e6de1e4f09dfcdb83bc5d"
REPLAY_PATH = "contrib/repair/bitcoinleveldb-snapshot-finalize001.py"
REPORT_PATH = "doc/repair/snapshot-diagnostic-dispatch001-coverage.json"
TAG = "bitcoinleveldb-snapshot-dispatch-repair001-20261105"
SOURCE_PATHS = {"bitcoinleveldb-modeldb/src/snapshot_vtable.rs",
                "bitcoinleveldbt-snapshot/tests/get_snapshot2.rs", REPAIR_RUNNER,
                COVERAGE_RUNNER, REPORT_PATH, BOUNDARY_RUNNER,
                "bitcoinleveldbt-snapshot/tests/snapshot_dispatch_boundary_contracts.rs"}
EXPECTED_METRICS = {"lines": (676, 760), "functions": (57, 57),
                    "regions": (997, 1151), "branches": (51, 52)}
EXPECTED_TEST_COUNT = 71
EXPECTED_UNMAPPED_COUNT = 4
SUITE = ["cargo", "test", "--locked", "-p", "bitcoinleveldb-modeldb", "-p",
         "bitcoinleveldbt-snapshot", "--", "--test-threads=1"]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return (json.dumps(value, sort_keys=True, indent=2) + "\n").encode()


def clean_env():
    env = dict(os.environ)
    for key in ("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR",
                "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_PREFIX", "GIT_NAMESPACE"):
        env.pop(key, None)
    env["GIT_OPTIONAL_LOCKS"] = "0"
    return env


def bootstrap_git(repo, *args):
    result = subprocess.run(["git", *args], cwd=repo, env=clean_env(), capture_output=True)
    if result.returncode:
        raise RuntimeError(result.stderr.decode(errors="replace"))
    return result.stdout.decode().strip()


def trusted_coverage(worktree):
    path = worktree / COVERAGE_RUNNER
    if sha(path.read_bytes()) != COVERAGE_RUNNER_SHA256:
        raise RuntimeError("Accepted coverage runner hash changed.")
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("accepted_snapshot_coverage001", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def blob(runner, commit, name, required=True):
    result = runner.capture(["git", "show", commit + ":" + name], check=False)
    if result.returncode and required:
        raise RuntimeError("Required Git blob unavailable: " + commit + ":" + name)
    return None if result.returncode else result.stdout


def names(runner, *args, cwd=None):
    result = runner.capture(["git", args[0], "-z", *args[1:]], cwd=cwd)
    return {os.fsdecode(item) for item in result.stdout.split(b"\0") if item}


def changed(runner, parent, head):
    return names(runner, "diff", "--no-renames", "--name-only", parent, head)


def fingerprint(path):
    try:
        info = path.lstat()
    except FileNotFoundError:
        return {"kind": "missing"}
    base = {"mode": stat.S_IMODE(info.st_mode)}
    if stat.S_ISLNK(info.st_mode):
        return dict(base, kind="symlink", target=os.fsdecode(os.readlink(path)))
    if not stat.S_ISREG(info.st_mode):
        raise RuntimeError("User-owned path has an unsupported file type; retained: " + str(path))
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for data in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(data)
    return dict(base, kind="file", size=info.st_size, sha256=digest.hexdigest())


def rust_input(name):
    return (name.endswith(".rs") or Path(name).name in {"Cargo.toml", "Cargo.lock", "rust-toolchain", "rust-toolchain.toml"}
            or ".cargo" in Path(name).parts)


def userdata(runner, incoming):
    origin = runner.origin
    dirty = names(runner, "diff", "--no-renames", "--name-only", "HEAD", cwd=origin)
    dirty |= names(runner, "diff", "--cached", "--no-renames", "--name-only", cwd=origin)
    untracked = names(runner, "ls-files", "--others", "--exclude-standard", cwd=origin)
    if dirty - {"Makefile"}:
        raise RuntimeError("Only the supplied Makefile edit is supported during integration; retained: " + repr(sorted(dirty)))
    cargo_configs = {".cargo/config", ".cargo/config.toml"}
    tracked_configs = names(runner, "ls-files", "--", *sorted(cargo_configs), cwd=origin)
    if any((origin / name).exists() or (origin / name).is_symlink() for name in cargo_configs - tracked_configs):
        raise RuntimeError("Untracked/ignored repository Cargo configuration requires review.")
    if any(rust_input(name) for name in untracked):
        raise RuntimeError("Untracked Rust/Cargo inputs require review before integration.")
    for local in dirty | untracked:
        if any(local == entry or local.startswith(entry + "/") or entry.startswith(local + "/") for entry in incoming):
            raise RuntimeError("User-owned path collides with an incoming path: " + local)
    ignored = names(runner, "ls-files", "--others", "--", *sorted(incoming), cwd=origin)
    if ignored:
        raise RuntimeError("An incoming path collides with an ignored/untracked file: " + repr(sorted(ignored)))
    for name in incoming:
        for ancestor in (origin / name).parents:
            if ancestor == origin:
                break
            if ancestor.is_symlink() or (ancestor.exists() and not ancestor.is_dir()):
                raise RuntimeError("Incoming path has a file/symlink ancestor: " + str(ancestor))
    tracked = {}
    for name in sorted(dirty):
        index = runner.capture(["git", "ls-files", "--stage", "-z", "--", name], cwd=origin).stdout
        diff = runner.capture(["git", "diff", "--no-ext-diff", "--no-textconv", "--binary", "HEAD", "--", name], cwd=origin).stdout
        tracked[name] = {"working": fingerprint(origin / name), "index_entries_hex": index.hex(), "diff_sha256": sha(diff)}
    return {"tracked": tracked, "untracked": {name: fingerprint(origin / name) for name in sorted(untracked)}}


def worker_guard(runner, expected_head, script, allow_script=False):
    runner.guard_git([REPLAY_PATH] if allow_script else [], expected_head)
    allowed = {REPLAY_PATH} if allow_script else set()
    tracked = names(runner, "diff", "--no-renames", "--name-only") | names(runner, "diff", "--cached", "--no-renames", "--name-only")
    if tracked - allowed:
        raise RuntimeError("Repair index/worktree includes unrelated changes.")
    untracked = names(runner, "ls-files", "--others", "--exclude-standard")
    if untracked - ({REPLAY_PATH} if allow_script else set()):
        raise RuntimeError("Repair worktree has unexpected untracked files.")
    path = runner.repo / REPLAY_PATH
    if path.is_symlink() or not path.resolve().is_relative_to(runner.repo.resolve()):
        raise RuntimeError("Finalizer replay path leaves the worktree.")
    for data in (path.read_bytes() if path.exists() else None, blob(runner, "", REPLAY_PATH, False)):
        if data is not None and data != script:
            raise RuntimeError("Unexpected finalizer file/index content was retained.")


def own_admin(runner, commit, script):
    return (runner.git("show", "-s", "--format=%P", commit).split() == [SOURCE_CHECKPOINT]
            and changed(runner, SOURCE_CHECKPOINT, commit) == {REPLAY_PATH}
            and blob(runner, commit, REPLAY_PATH, False) == script
            and runner.git("ls-tree", commit, "--", REPLAY_PATH).split("\t", 1)[0].startswith("100755 blob "))


def integration_inputs(runner, commit):
    records = runner.capture(["git", "ls-tree", "-r", "-z", commit]).stdout
    result = {}
    for record in records.split(b"\0"):
        if record:
            fields, name = record.split(b"\t", 1)
            name = os.fsdecode(name)
            if rust_input(name):
                result[name] = fields.decode()
    return result


def origin_guard(runner, head, incoming, expected_userdata, manifest):
    previous = runner.repo
    runner.repo = runner.origin
    try:
        runner.guard_operations()
        if runner.git("symbolic-ref", "--quiet", "--short", "HEAD") != "master" or runner.git("rev-parse", "HEAD") != head:
            raise RuntimeError("Original master advanced beyond the recorded integration step.")
        if userdata(runner, incoming) != expected_userdata:
            raise RuntimeError("User-owned Makefile/index/untracked state changed; no work discarded.")
        if names(runner, "diff", "--no-renames", "--name-only", head) & manifest.keys():
            raise RuntimeError("Integrated Rust/build inputs differ from the validated commit.")
        if names(runner, "diff", "--cached", "--no-renames", "--name-only", head) & manifest.keys():
            raise RuntimeError("Staged Rust/build inputs differ from the validated commit.")
    finally:
        runner.repo = previous


def tag_object(runner, target, message, expected_oid=None):
    ref = runner.capture(["git", "rev-parse", "--verify", "refs/tags/" + TAG], check=False)
    if ref.returncode:
        if expected_oid:
            raise RuntimeError("Previously recorded tag disappeared; it was not recreated.")
        return None
    oid = ref.stdout.decode().strip()
    if expected_oid and oid != expected_oid:
        raise RuntimeError("Recorded tag object changed; it was not replaced.")
    if runner.git("cat-file", "-t", oid) != "tag":
        raise RuntimeError("Existing tag is not annotated; it was not replaced.")
    data = runner.capture(["git", "cat-file", "tag", oid]).stdout.decode()
    header, body = data.split("\n\n", 1)
    headers = dict(line.split(" ", 1) for line in header.splitlines())
    signature = runner.capture(["git", "for-each-ref", "--format=%(contents:signature)", "refs/tags/" + TAG]).stdout.decode().rstrip("\n")
    if signature:
        if not body.rstrip("\n").endswith(signature):
            raise RuntimeError("Annotated tag signature boundary is ambiguous.")
        body = body.rstrip("\n")[:-len(signature)]
    if (headers.get("object") != target or headers.get("type") != "commit" or headers.get("tag") != TAG
            or body.rstrip("\n") != message.rstrip("\n")):
        raise RuntimeError("Existing tag target or scoped annotation differs; it was not replaced.")
    return oid


def annotation_for(admin, script, measurement):
    metrics = measurement["coverage"]["metrics"]
    return ("Snapshot diagnostic dispatch repair001: scoped integration checkpoint\n\n"
                      "Runtime kind and sequence observations use the Snapshot trait contract.\n"
                      "Regression and boundary tests passed: " + str(EXPECTED_TEST_COUNT) + " tests, default-feature host configuration.\n"
                      "Coverage domain: bitcoinleveldb-modeldb/src and bitcoinleveldbt-snapshot/src.\n"
                      "Mapped " + "; ".join(key + " " + str(metrics[key]["covered"]) + "/" + str(metrics[key]["count"])
                                               for key in ("functions", "lines", "regions", "branches")) + ".\n"
                      + str(len(measurement["coverage"]["unmapped_source_files"])) + " tracked source files are unmapped. Full crate/all-features coverage is NOT claimed.\n\n"
                      "Source-Checkpoint: " + SOURCE_CHECKPOINT + "\nTest-Checkpoint: " + TEST_CHECKPOINT + "\nIntegrated-Commit: " + admin + "\n"
                      "Finalizer-SHA256: " + sha(script) + "\nNixpkgs: " + json.dumps(measurement["nixpkgs"], sort_keys=True) + "\n"
                      "Compiler: " + measurement["compiler"]["rustc_vv"].replace("\n", "; ") + "\n"
                      "Validation: " + " ".join(SUITE) + "\n")


def environment_guard(runner):
    overrides = [key for key in runner.env if (key in {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_TARGET", "CARGO_BUILD_RUSTFLAGS", "CARGO_BUILD_RUSTC", "CARGO_BUILD_RUSTC_WRAPPER", "RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER"}
                 or key.startswith("CARGO_PROFILE_") or (key.startswith("CARGO_TARGET_") and key.endswith(("_RUSTFLAGS", "_RUSTC_WRAPPER")))) and runner.env[key]]
    if overrides:
        raise RuntimeError("Unexpected Rust/build overrides require review: " + repr(overrides))


def run_finalize(origin, script_path):
    runner = module = coverage = lock = None
    report = {"workflow": WORKFLOW, "outcome": "failed", "phase": "preflight", "source_checkpoint": SOURCE_CHECKPOINT,
              "test_checkpoint": TEST_CHECKPOINT, "tag": TAG, "push": "not requested"}
    failure = None
    try:
        if Path(bootstrap_git(origin, "rev-parse", "--show-toplevel")).resolve() != origin:
            raise RuntimeError("--repo must identify the original checkout root.")
        metadata = Path(bootstrap_git(origin, "rev-parse", "--path-format=absolute", "--git-common-dir")) / "codex-repairs" / REPAIR
        lock = (metadata / "run.lock").open("a+")
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        layout = json.loads((metadata / "worktree.json").read_text())
        repair_state = json.loads((metadata / "state.json").read_text())
        old_coverage = json.loads((metadata / "coverage-state001.json").read_text())
        boundaries = json.loads((metadata / "boundary-tests001-state.json").read_text())
        worktree = Path(layout["path"])
        coverage = trusted_coverage(worktree)
        if (layout.get("repair") != REPAIR or layout.get("origin") != str(origin) or layout.get("branch") != BRANCH
                or layout.get("revision") != "004" or layout.get("baseline") != ORIGINAL_BASE
                or layout.get("script_sha256") != REPAIR_RUNNER_SHA256
                or repair_state.get("baseline") != ORIGINAL_BASE or repair_state.get("checkpoint") != coverage.EXPECTED_COMMIT
                or repair_state.get("script_sha256") != REPAIR_RUNNER_SHA256
                or old_coverage.get("checkpoint") != OLD_COVERAGE_CHECKPOINT
                or old_coverage.get("script_sha256") != COVERAGE_RUNNER_SHA256
                or boundaries.get("script_sha256") != BOUNDARY_RUNNER_SHA256
                or boundaries.get("test_checkpoint") != TEST_CHECKPOINT or boundaries.get("coverage_checkpoint") != SOURCE_CHECKPOINT
                or any(state.get("blocked_checkpoint") for state in (repair_state, old_coverage, boundaries))):
            raise RuntimeError("Accepted repair/coverage/boundary journals differ from the supplied checkpoints.")
        registration = bootstrap_git(origin, "worktree", "list", "--porcelain")
        if ("worktree " + str(worktree) + "\n") not in registration or worktree.is_symlink():
            raise RuntimeError("Accepted repair worktree registration changed.")
        module, runner = coverage.load_repair_runner(worktree, origin)
        runner.repo = worktree
        stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
        directory = metadata / "finalize-runs" / (stamp + "-" + uuid.uuid4().hex[:8])
        directory.mkdir(parents=True)
        runner.directory, runner.log, runner.report = directory, (directory / "run.log").open("w", buffering=1), report
        report.update(origin_repository=str(origin), worktree=str(worktree), run_directory=str(directory))
        for name, expected in ((REPAIR_RUNNER, REPAIR_RUNNER_SHA256), (COVERAGE_RUNNER, COVERAGE_RUNNER_SHA256), (BOUNDARY_RUNNER, BOUNDARY_RUNNER_SHA256)):
            if sha(blob(runner, SOURCE_CHECKPOINT, name)) != expected or sha((worktree / name).read_bytes()) != expected:
                raise RuntimeError("Accepted runner changed: " + name)
        if changed(runner, ORIGINAL_BASE, SOURCE_CHECKPOINT) != SOURCE_PATHS:
            raise RuntimeError("Repair history includes paths beyond the reviewed seven-file scope.")
        measurement_bytes = blob(runner, SOURCE_CHECKPOINT, REPORT_PATH)
        measurement = json.loads(measurement_bytes)
        if (boundaries.get("normal_validated") is not True or boundaries.get("coverage_validated") is not True
                or any(boundaries.get("normal_tests", {}).get(key, {}).get("exit_code") != 0 for key in ("focused", "crate_suites"))
                or boundaries.get("measurement") != measurement or sha(measurement_bytes) != boundaries.get("report_sha256")
                or measurement.get("tested_commit") != TEST_CHECKPOINT or measurement.get("instrumented_test_exit_code") != 0
                or sha(Path(boundaries["raw_path"]).read_bytes()) != boundaries.get("raw_sha256")
                or measurement.get("raw_json_sha256") != boundaries.get("raw_sha256")):
            raise RuntimeError("Test and coverage evidence differs from the accepted source checkpoint.")
        metrics = measurement["coverage"]["metrics"]
        if (any((metrics[key]["covered"], metrics[key]["count"]) != value for key, value in EXPECTED_METRICS.items())
                or len(measurement["coverage"]["unmapped_source_files"]) != EXPECTED_UNMAPPED_COUNT):
            raise RuntimeError("Coverage scope/counts differ from the supplied qualified repair.")
        script = script_path.read_bytes()
        state_path = metadata / "finalize001-state.json"
        state = json.loads(state_path.read_text()) if state_path.exists() else None
        identity = {"workflow": WORKFLOW, "source_checkpoint": SOURCE_CHECKPOINT, "original_base": ORIGINAL_BASE,
                    "script_sha256": sha(script), "measurement_sha256": sha(measurement_bytes), "tag": TAG}
        if state and (any(state.get(key) != value for key, value in identity.items()) or state.get("blocked_checkpoint")):
            raise RuntimeError("Finalizer state belongs to another script/checkpoint or needs inspection.")
        state = state or dict(identity, phase="admin_pending")
        if state.get("integration_validated"):
            prior = state.get("integration_tests", {})
            if (state["integration_validated"] is not True or prior.get("exit_code") != 0
                    or prior.get("passed_count") != EXPECTED_TEST_COUNT or prior.get("command") != SUITE
                    or not prior.get("summaries") or any(len(item) != 3 or item[1] or item[2] for item in prior["summaries"])
                    or sum(item[0] for item in prior["summaries"]) != EXPECTED_TEST_COUNT):
                raise RuntimeError("Recorded integration-test qualification changed.")
        head = runner.git("rev-parse", "HEAD")
        if head != SOURCE_CHECKPOINT and not state.get("admin_checkpoint"):
            if not own_admin(runner, head, script):
                raise RuntimeError("Repair branch advanced beyond its source/admin checkpoints.")
            state["admin_checkpoint"] = head
        if state.get("admin_checkpoint") and not own_admin(runner, state["admin_checkpoint"], script):
            raise RuntimeError("Recorded administrative commit changed.")
        existing_tag = runner.capture(["git", "rev-parse", "--verify", "refs/tags/" + TAG], check=False)
        if existing_tag.returncode == 0:
            if not state.get("admin_checkpoint"):
                raise RuntimeError("Tag name already exists before this integration; it was not replaced.")
            tag_object(runner, state["admin_checkpoint"], annotation_for(state["admin_checkpoint"], script, measurement), state.get("tag_object"))
        environment_guard(runner)
        expected_worker = state.get("admin_checkpoint", SOURCE_CHECKPOINT)
        worker_guard(runner, expected_worker, script, allow_script=not state.get("admin_checkpoint"))
        incoming = SOURCE_PATHS | {REPLAY_PATH}
        before = userdata(runner, incoming)
        if state.get("userdata") is not None and state["userdata"] != before:
            raise RuntimeError("User-owned data changed since finalization began; retained for review.")
        state["userdata"] = before
        origin_head = bootstrap_git(origin, "rev-parse", "HEAD")
        if bootstrap_git(origin, "symbolic-ref", "--quiet", "--short", "HEAD") != "master":
            raise RuntimeError("Original checkout is no longer on master.")
        if origin_head not in {ORIGINAL_BASE, state.get("admin_checkpoint")}:
            raise RuntimeError("Original master has an unreviewed commit; no merge was attempted.")
        runner.save_state(state_path, state)
        if not state.get("admin_checkpoint"):
            report["phase"] = "administrative_checkpoint"
            module.atomic_write(worktree / REPLAY_PATH, script, 0o755)
            runner.git("add", "--chmod=+x", "--", REPLAY_PATH)
            worker_guard(runner, SOURCE_CHECKPOINT, script, True)
            message = directory / "admin-commit.txt"
            message.write_text("leveldb: record snapshot repair integration procedure\n\nFinalize-Workflow: " + WORKFLOW + "\nSource-Checkpoint: " + SOURCE_CHECKPOINT + "\n")
            if runner.stream(["git", "commit", "--file", str(message)]):
                raise RuntimeError("Administrative commit failed; known script retained for retry.")
            admin = runner.git("rev-parse", "HEAD")
            if not own_admin(runner, admin, script):
                state["blocked_checkpoint"] = admin
                runner.save_state(state_path, state)
                raise RuntimeError("Administrative commit parent, path or blob changed.")
            state.update(admin_checkpoint=admin, phase="admin_checkpoint")
            runner.save_state(state_path, state)
        admin = state["admin_checkpoint"]
        worker_guard(runner, admin, script)
        manifest = integration_inputs(runner, admin)
        if state.get("integration_inputs") not in (None, manifest):
            raise RuntimeError("Recorded Rust/build input manifest changed.")
        state["integration_inputs"] = manifest
        report.update(admin_checkpoint=admin, integration_inputs_sha256=sha(encoded(manifest)),
                      userdata_before_sha256=sha(encoded(before)), untracked_count=len(before["untracked"]))
        origin_guard(runner, origin_head, incoming, before, manifest)
        state["phase"] = "merge_intent"
        runner.save_state(state_path, state)
        if origin_head == ORIGINAL_BASE:
            report["phase"] = "fast_forward"
            runner.repo = origin
            status = runner.stream(["git", "merge", "--ff-only", "--no-autostash", "--no-overwrite-ignore", admin])
            runner.repo = worktree
            if status:
                raise RuntimeError("Fast-forward failed; existing work was retained.")
        origin_guard(runner, admin, incoming, before, manifest)
        state.update(merge_done=True, phase="merged")
        runner.save_state(state_path, state)
        report.update(master_head=admin, integration_validation_reused=bool(state.get("integration_validated")))
        runner.repo = origin
        if not state.get("integration_validated"):
            report["phase"] = "integration_tests"
            nixpkgs = module.normalize_nixpkgs(measurement["nixpkgs"])
            shell = ('with import ' + json.dumps(nixpkgs["outPath"]) + ' {}; mkShell { nativeBuildInputs = [ pkg-config ]; '
                     'buildInputs = [ libevent openssl clang ]; LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib"; }')
            prefix = ["nix", "develop", "--impure", "--expr", shell, "--command", "env", "GIT_CONFIG_COUNT=1",
                      "GIT_CONFIG_KEY_0=url.https://github.com/.insteadOf", "GIT_CONFIG_VALUE_0=git@github.com:",
                      "CARGO_NET_GIT_FETCH_WITH_CLI=true", "CARGO_BUILD_JOBS=4", "RUST_BACKTRACE=1", "CARGO_TARGET_DIR=" + str(origin / "target")]
            for command, expected in ((["rustc", "-Vv"], measurement["compiler"]["rustc_vv"]), (["cargo", "-V"], measurement["compiler"]["cargo_version"])):
                output = runner.capture(prefix + command).stdout.decode().strip()
                if output != expected:
                    raise RuntimeError("Integration compiler/Cargo differs from accepted coverage tools: " + output)
            readout = []
            status = runner.stream(prefix + SUITE, collect=readout)
            summaries = [tuple(map(int, found.groups())) for line in readout
                         if (found := re.match(r"^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;", line))]
            count = sum(item[0] for item in summaries)
            report["integration_tests"] = {"command": SUITE, "exit_code": status, "passed_count": count, "summaries": summaries}
            if status or count != EXPECTED_TEST_COUNT or any(item[1] or item[2] for item in summaries):
                raise RuntimeError("Post-merge suite did not reproduce the accepted passing tests; merged master retained without a tag.")
            origin_guard(runner, admin, incoming, before, manifest)
            state.update(integration_validated=True, integration_tests=report["integration_tests"], phase="integration_validated")
            runner.save_state(state_path, state)
        report["integration_tests"] = state["integration_tests"]
        origin_guard(runner, admin, incoming, before, manifest)
        annotation = annotation_for(admin, script, measurement)
        report["phase"] = "annotated_tag"
        oid = tag_object(runner, admin, annotation, state.get("tag_object"))
        if oid is None:
            message = directory / "tag-message.txt"
            message.write_text(annotation)
            if runner.stream(["git", "tag", "--annotate", "--cleanup=verbatim", "--file", str(message), TAG, admin]):
                raise RuntimeError("Annotated tag creation failed; validated merged state retained for retry.")
            oid = tag_object(runner, admin, annotation)
        state.update(tag_object=oid, phase="complete")
        runner.save_state(state_path, state)
        origin_guard(runner, admin, incoming, before, manifest)
        runner.repo = worktree
        worker_guard(runner, admin, script)
        report.update(outcome="passed", phase="complete", tag_object=oid, tag_target=admin,
                      userdata_after_sha256=sha(encoded(userdata(runner, incoming))), userdata_preserved=True,
                      remaining_coverage=measurement["coverage"], full_coverage_claimed=False)
    except KeyboardInterrupt:
        failure = "Interrupted; recorded commits, merged master and user work retained."
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
                report.update(master_head=bootstrap_git(origin, "rev-parse", "HEAD"), repair_head=bootstrap_git(worktree, "rev-parse", "HEAD"),
                              original_status=bootstrap_git(origin, "status", "--short"), return_file=str(runner.directory / "FINALIZE_RETURN.txt"))
                module.atomic_write(Path(report["return_file"]), encoded(report))
            except Exception as final_error:
                report.update(outcome="failed", finalization_error=str(final_error))
            runner.log.close()
        if lock:
            lock.close()
        print("BEGIN_FINALIZE_RETURN")
        print(json.dumps(report, indent=2))
        print("END_FINALIZE_RETURN")
        if report.get("return_file"):
            print("Return file: " + report["return_file"])
    return 0 if report["outcome"] == "passed" else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.home() / "bitcoin-rs")
    args = parser.parse_args()
    return run_finalize(args.repo.expanduser().resolve(), Path(__file__).resolve())


if __name__ == "__main__":
    sys.exit(main())
