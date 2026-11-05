#!/usr/bin/env python3
"""Add and validate a portable pinned native development shell for bitcoin-rs."""
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
import subprocess
import sys
import traceback
import uuid

BASE = "00939eb9c9a96478e10cf1847e3c167c9e4bc915"
WORKFLOW = "nix-development-shell001"
BRANCH = "build/nix-development-shell"
FINALIZER_PATH = "contrib/repair/bitcoinleveldb-snapshot-finalize001.py"
FINALIZER_SHA256 = "d6be4492a2039891038fea972b894836b45ee3af9d329420d809e02aec25da3c"
REPLAY_PATH = "contrib/repair/bitcoinrs-nix-development-shell001.py"
PATHS = {"shell.nix", REPLAY_PATH}
PIN_SOURCE = Path("/nix/store/77dbgds155bbz3vd3qywq1sii07i5ljs-source")
LOCKFILE = Path("/etc/nixos/flake.lock")
NAR_HASH = "sha256-30sZNZoA1cqF5JNO9fVX+wgiQYjB7HJqqJ4ztCDeBZE="
EXPECTED_TEST_COUNT = 71
SUITE = ["cargo", "test", "--locked", "-p", "bitcoinleveldb-modeldb", "-p", "bitcoinleveldbt-snapshot", "--", "--test-threads=1"]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return (json.dumps(value, sort_keys=True, indent=2) + "\n").encode()


def trusted_finalizer(origin):
    path = origin / FINALIZER_PATH
    if sha(path.read_bytes()) != FINALIZER_SHA256 or path.is_symlink():
        raise RuntimeError("Accepted integration runner changed; no environment files were written.")
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("accepted_snapshot_finalizer001", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def pin_records(value):
    records = []
    if isinstance(value, dict):
        locked = value.get("locked")
        if isinstance(locked, dict):
            github = (locked.get("type") == "github" and str(locked.get("owner", "")).lower() == "nixos" and locked.get("repo") == "nixpkgs")
            git = (locked.get("type") == "git" and re.fullmatch(r"(?:git\+)?https://github\.com/NixOS/nixpkgs(?:\.git)?/?", str(locked.get("url", "")), re.IGNORECASE))
            if github or git:
                records.append({key: locked.get(key) for key in ("type", "owner", "repo", "url", "rev", "narHash") if key in locked})
        for item in value.values():
            records.extend(pin_records(item))
    elif isinstance(value, list):
        for item in value:
            records.extend(pin_records(item))
    return records


def resolve_pin(runner, report):
    diagnostics = []
    if runner.capture(["nix", "hash", "path", "--type", "sha256", "--sri", str(PIN_SOURCE)]).stdout.decode().strip() != NAR_HASH:
        raise RuntimeError("The previously accepted Nix source no longer has its recorded content hash.")
    try:
        records = pin_records(json.loads(LOCKFILE.read_text()))
        diagnostics.append({"source": str(LOCKFILE), "records": records})
    except (OSError, ValueError) as error:
        diagnostics.append({"source": str(LOCKFILE), "error": str(error)})
    command = ["nix", "flake", "metadata", "--json", "--offline", "--no-write-lock-file", "nixpkgs"]
    for online in (False, True):
        result = runner.capture(command if not online else [item for item in command if item != "--offline"], check=False)
        try:
            records = pin_records(json.loads(result.stdout)) if result.returncode == 0 else []
        except ValueError:
            records = []
        diagnostics.append({"source": "nixpkgs flake metadata " + ("online" if online else "offline"), "exit_code": result.returncode, "records": records})
        candidates = {str(record["rev"]).lower() for item in diagnostics for record in item.get("records", [])
                      if record.get("narHash") == NAR_HASH and re.fullmatch(r"[0-9a-fA-F]{40}", str(record.get("rev", "")))}
        if candidates or online:
            break
    report["pin_metadata"] = diagnostics
    if len(candidates) != 1:
        raise RuntimeError("An unambiguous immutable nixpkgs revision matching the accepted content hash was not found; pin diagnostics returned.")
    rev = candidates.pop()
    url = "https://github.com/NixOS/nixpkgs/archive/" + rev + ".tar.gz"
    expression = "toString (builtins.fetchTarball { url = " + json.dumps(url) + "; sha256 = " + json.dumps(NAR_HASH) + "; })"
    fetched = json.loads(runner.capture(["nix", "eval", "--impure", "--json", "--expr", expression]).stdout)
    if not isinstance(fetched, str) or not Path(fetched).is_absolute():
        raise RuntimeError("Pinned tarball evaluation did not return an absolute source path.")
    measured = runner.capture(["nix", "hash", "path", "--type", "sha256", "--sri", fetched]).stdout.decode().strip()
    if measured != NAR_HASH:
        raise RuntimeError("Fixed-fetch nixpkgs content differs from the accepted Nix source.")
    return {"rev": rev, "url": url, "narHash": NAR_HASH, "verified_path": fetched}


def shell_bytes(pin):
    return ('''# Native libraries for the existing Rust toolchain.
# Usage from this checkout: nix-shell --run make
let
  nixpkgs = builtins.fetchTarball {
    url = "''' + pin["url"] + '''";
    sha256 = "''' + pin["narHash"] + '''";
  };
  pkgs = import nixpkgs { config = {}; overlays = []; };
in pkgs.mkShell {
  name = "bitcoin-rs";
  nativeBuildInputs = [ pkgs.pkg-config pkgs.gnumake ];
  buildInputs = [ pkgs.libevent pkgs.openssl pkgs.clang ];
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
''').encode()


def root_file_guard(runner, final, checkpoint=None):
    for name in ("shell.nix", "flake.nix", "flake.lock"):
        tracked = final.blob(runner, BASE, name, False)
        path = runner.origin / name
        allowed = name == "shell.nix" and checkpoint and final.bootstrap_git(runner.origin, "rev-parse", "HEAD") == checkpoint
        if tracked is not None or ((path.exists() or path.is_symlink()) and not allowed):
            raise RuntimeError("Existing root environment file requires review; retained: " + name)


def worker_guard(runner, final, head, files, pending):
    runner.guard_operations()
    if runner.git("symbolic-ref", "--quiet", "--short", "HEAD") != BRANCH or runner.git("rev-parse", "HEAD") != head:
        raise RuntimeError("Development-shell branch advanced beyond its recorded checkpoints.")
    dirty = final.names(runner, "diff", "--no-renames", "--name-only") | final.names(runner, "diff", "--cached", "--no-renames", "--name-only")
    untracked = final.names(runner, "ls-files", "--others", "--exclude-standard")
    if (dirty | untracked) - (PATHS if pending else set()):
        raise RuntimeError("Development-shell worktree contains unrelated changes; retained for review.")
    configs = {".cargo/config", ".cargo/config.toml"}
    known_configs = final.names(runner, "ls-files", "--", *sorted(configs))
    if any((runner.repo / name).exists() or (runner.repo / name).is_symlink() for name in configs - known_configs):
        raise RuntimeError("Development-shell worktree has untracked/ignored Cargo configuration.")
    for name, data in files.items():
        path = runner.repo / name
        if any(part.is_symlink() for part in [path, *path.parents] if part != runner.repo and runner.repo in part.parents):
            raise RuntimeError("Development-shell file has a symlink ancestor: " + name)
        if path.exists() and path.read_bytes() != data:
            raise RuntimeError("Unexpected development-shell working file retained: " + name)
        indexed = final.blob(runner, "", name, False)
        if indexed is not None and indexed != data:
            raise RuntimeError("Unexpected development-shell staged content retained: " + name)


def own_commit(runner, final, head, files):
    if runner.git("show", "-s", "--format=%P", head).split() != [BASE] or final.changed(runner, BASE, head) != PATHS:
        return False
    return all(final.blob(runner, head, name, False) == data and runner.git("ls-tree", head, "--", name).startswith(("100755" if name == REPLAY_PATH else "100644") + " blob ") for name, data in files.items())


def tests_valid(value):
    return (value.get("exit_code") == 0 and value.get("passed_count") == EXPECTED_TEST_COUNT and value.get("command") == SUITE
            and bool(value.get("summaries")) and all(len(row) == 3 and not row[1] and not row[2] for row in value["summaries"])
            and sum(row[0] for row in value["summaries"]) == EXPECTED_TEST_COUNT)


def run_devshell(origin, script_path):
    runner = final = module = None
    locks = []
    report = {"workflow": WORKFLOW, "outcome": "failed", "phase": "preflight", "base": BASE, "branch": BRANCH, "push": "not requested", "new_tag": "not requested"}
    failure = None
    try:
        final = trusted_finalizer(origin)
        if Path(final.bootstrap_git(origin, "rev-parse", "--show-toplevel")).resolve() != origin:
            raise RuntimeError("--repo must identify the original checkout root.")
        common = Path(final.bootstrap_git(origin, "rev-parse", "--path-format=absolute", "--git-common-dir"))
        prior = common / "codex-repairs" / final.REPAIR
        metadata = common / "codex-repairs" / WORKFLOW
        metadata.mkdir(parents=True, exist_ok=True)
        for path in (prior / "run.lock", metadata / "run.lock"):
            lock = path.open("a+")
            locks.append(lock)
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        coverage = final.trusted_coverage(origin)
        module, runner = coverage.load_repair_runner(origin, origin)
        directory = metadata / "runs" / (dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])
        directory.mkdir(parents=True)
        runner.directory, runner.log, runner.report = directory, (directory / "run.log").open("w", buffering=1), report
        report.update(origin_repository=str(origin), run_directory=str(directory))
        if sha(final.blob(runner, BASE, FINALIZER_PATH)) != FINALIZER_SHA256:
            raise RuntimeError("Reviewed base does not contain the accepted integration runner.")
        prior_state = json.loads((prior / "finalize001-state.json").read_text())
        boundaries = json.loads((prior / "boundary-tests001-state.json").read_text())
        compiler = boundaries["measurement"]["compiler"]
        tag_oid = runner.git("rev-parse", "refs/tags/" + final.TAG)
        if (prior_state.get("admin_checkpoint") != BASE or prior_state.get("phase") != "complete"
                or prior_state.get("script_sha256") != FINALIZER_SHA256 or prior_state.get("tag_object") != tag_oid
                or prior_state.get("integration_validated") is not True or not tests_valid(prior_state.get("integration_tests", {}))):
            raise RuntimeError("Prior integration/tag evidence does not match the reviewed base.")
        state_path = metadata / "state.json"
        state = json.loads(state_path.read_text()) if state_path.exists() else None
        script = script_path.read_bytes()
        identity = {"workflow": WORKFLOW, "base": BASE, "script_sha256": sha(script), "branch": BRANCH, "previous_tag_object": tag_oid}
        if state and (any(state.get(key) != value for key, value in identity.items()) or state.get("blocked_checkpoint")):
            raise RuntimeError("Development-shell journal belongs to other inputs or needs inspection.")
        state = state or dict(identity, phase="pin_pending")
        checkpoint = state.get("checkpoint")
        if final.bootstrap_git(origin, "symbolic-ref", "--quiet", "--short", "HEAD") != "master" or final.bootstrap_git(origin, "rev-parse", "HEAD") not in {BASE, checkpoint}:
            raise RuntimeError("Original master is not at the reviewed base or owned shell checkpoint.")
        runner.guard_operations()
        final.environment_guard(runner)
        root_file_guard(runner, final, checkpoint)
        before = final.userdata(runner, PATHS)
        if state.get("userdata") is not None and state["userdata"] != before:
            raise RuntimeError("Original Makefile/index/untracked state changed since this workflow began; retained.")
        state["userdata"] = before
        report.update(userdata_before_sha256=sha(encoded(before)), untracked_count=len(before["untracked"]), previous_tag_object=tag_oid)
        report["phase"] = "pin_verification"
        pin = resolve_pin(runner, report)
        if state.get("pin") not in (None, pin):
            raise RuntimeError("Resolved portable nixpkgs pin differs from the recorded workflow.")
        shell = shell_bytes(pin)
        files = {"shell.nix": shell, REPLAY_PATH: script}
        state.update(pin=pin, shell_sha256=sha(shell))
        if state.get("tests_validated") and not tests_valid(state.get("tests", {})):
            raise RuntimeError("Recorded shell test qualification changed.")
        worktree = origin.parent / (origin.name + "-nix-development-shell")
        layout_path = metadata / "worktree.json"
        layout = dict(identity, origin=str(origin), path=str(worktree))
        registered = final.bootstrap_git(origin, "worktree", "list", "--porcelain")
        branch_ref = runner.capture(["git", "rev-parse", "--verify", "refs/heads/" + BRANCH], check=False)
        if layout_path.exists():
            if json.loads(layout_path.read_text()) != layout:
                raise RuntimeError("Development-shell worktree ownership journal differs.")
        else:
            if worktree.exists() or branch_ref.returncode == 0:
                raise RuntimeError("Development-shell branch/worktree already exists without this workflow's ownership journal.")
            runner.save_state(layout_path, layout)
        runner.save_state(state_path, state)
        if ("worktree " + str(worktree) + "\n") not in registered:
            if worktree.exists() or (branch_ref.returncode == 0 and branch_ref.stdout.decode().strip() != BASE):
                raise RuntimeError("Incomplete worktree setup needs review; existing paths were retained.")
            command = ["git", "worktree", "add"] + (["-b", BRANCH, str(worktree), BASE] if branch_ref.returncode else [str(worktree), BRANCH])
            if runner.stream(command):
                raise RuntimeError("Isolated development-shell worktree setup failed; owned state retained.")
        if worktree.is_symlink():
            raise RuntimeError("Development-shell worktree path is a symlink.")
        runner.repo = worktree
        head = runner.git("rev-parse", "HEAD")
        if not checkpoint and head != BASE:
            if not state.get("tests_validated") or not own_commit(runner, final, head, files):
                raise RuntimeError("Development-shell branch has an unreviewed commit.")
            checkpoint = state["checkpoint"] = head
        if checkpoint and not own_commit(runner, final, checkpoint, files):
            raise RuntimeError("Owned development-shell checkpoint differs from its exact two-file scope.")
        worker_guard(runner, final, checkpoint or BASE, files, not checkpoint)
        manifest = final.integration_inputs(runner, BASE)
        origin_head = final.bootstrap_git(origin, "rev-parse", "HEAD")
        def original_guard(head):
            final.origin_guard(runner, head, PATHS, before, manifest)
            if runner.git("rev-parse", "refs/tags/" + final.TAG) != tag_oid:
                raise RuntimeError("The prior repair tag changed during this workflow; it was not rewritten.")
        original_guard(origin_head)
        if not checkpoint:
            for name, data in files.items():
                module.atomic_write(worktree / name, data, 0o755 if name == REPLAY_PATH else 0o644)
        report.update(pin=pin, worktree=str(worktree), tests_reused=bool(state.get("tests_validated")))
        runner.env.update(CARGO_TARGET_DIR=str(origin / "target"), CARGO_NET_GIT_FETCH_WITH_CLI="true", CARGO_BUILD_JOBS="4", RUST_BACKTRACE="1")
        build_env = ["env", "GIT_CONFIG_COUNT=1", "GIT_CONFIG_KEY_0=url.https://github.com/.insteadOf", "GIT_CONFIG_VALUE_0=git@github.com:"]
        shell_command = ["nix-shell", str(worktree / "shell.nix"), "--run"]
        report["phase"] = "native_environment_probe"
        native = runner.capture(shell_command + ['pkg-config --modversion libevent openssl && test -r "$LIBCLANG_PATH/libclang.so"']).stdout.decode().strip()
        if len(native.splitlines()) != 2:
            raise RuntimeError("Native dependency probe did not report libevent and OpenSSL versions.")
        for command, expected in ((["rustc", "-Vv"], compiler["rustc_vv"]), (["cargo", "-V"], compiler["cargo_version"])):
            if runner.capture(shell_command + [shlex.join(command)]).stdout.decode().strip() != expected:
                raise RuntimeError("Development shell changed the previously accepted Rust/Cargo toolchain.")
        report["native_versions"] = state["native_versions"] = {"libevent": native.splitlines()[0], "openssl": native.splitlines()[1], "libclang_shared_library_readable": True}
        if not state.get("tests_validated"):
            report["phase"] = "crate_tests"
            output = []
            status = runner.stream(shell_command + [shlex.join(build_env + SUITE)], collect=output)
            summaries = [list(map(int, match.groups())) for line in output if (match := re.match(r"^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;", line))]
            result = {"command": SUITE, "exit_code": status, "passed_count": sum(row[0] for row in summaries), "summaries": summaries}
            state["tests"] = report["tests"] = result
            if not tests_valid(result):
                runner.save_state(state_path, state)
                raise RuntimeError("Pinned-shell suite did not reproduce all 71 passing tests; no checkpoint or merge was made.")
            worker_guard(runner, final, BASE, files, True)
            original_guard(origin_head)
            state.update(tests_validated=True, phase="tests_validated")
            runner.save_state(state_path, state)
        report["tests"] = state["tests"]
        if not checkpoint:
            report["phase"] = "validated_checkpoint"
            runner.git("add", "--chmod=-x", "--", "shell.nix")
            runner.git("add", "--chmod=+x", "--", REPLAY_PATH)
            worker_guard(runner, final, BASE, files, True)
            message = directory / "commit-message.txt"
            message.write_text("build: add pinned native Nix development shell\n\nValidated native dependencies and unchanged Rust toolchain.\n" + str(EXPECTED_TEST_COUNT) + " two-crate tests passed; no failed or ignored tests.\n\nEnvironment-Workflow: " + WORKFLOW + "\nBase: " + BASE + "\n")
            if runner.stream(["git", "commit", "--file", str(message)]):
                raise RuntimeError("Validated environment commit failed; exact known files retained for retry.")
            checkpoint = runner.git("rev-parse", "HEAD")
            if not own_commit(runner, final, checkpoint, files):
                state["blocked_checkpoint"] = checkpoint
                runner.save_state(state_path, state)
                raise RuntimeError("Environment commit changed beyond its exact parent, files, blobs or modes.")
            state.update(checkpoint=checkpoint, phase="checkpoint")
            runner.save_state(state_path, state)
        worker_guard(runner, final, checkpoint, files, False)
        original_guard(origin_head)
        report.update(checkpoint=checkpoint, make_reused=bool(state.get("make_validated")))
        if state.get("make_validated") and state.get("make", {}).get("exit_code") != 0:
            raise RuntimeError("Recorded Makefile validation is inconsistent.")
        if not state.get("make_validated"):
            report["phase"] = "original_make"
            runner.repo = origin
            status = runner.stream(shell_command + [shlex.join(build_env + ["make"])])
            runner.repo = worktree
            state["make"] = report["make"] = {"command": ["nix-shell", "shell.nix", "--run", "make"], "exit_code": status, "working_directory": str(origin)}
            original_guard(origin_head)
            worker_guard(runner, final, checkpoint, files, False)
            if status:
                runner.save_state(state_path, state)
                raise RuntimeError("Make failed in the preserved original checkout; validated branch checkpoint retained and master was not advanced.")
            state.update(make_validated=True, phase="make_validated")
            runner.save_state(state_path, state)
        report["make"] = state["make"]
        original_guard(origin_head)
        if origin_head == BASE:
            report["phase"] = "fast_forward"
            runner.repo = origin
            status = runner.stream(["git", "merge", "--ff-only", "--no-autostash", "--no-overwrite-ignore", checkpoint])
            runner.repo = worktree
            if status:
                raise RuntimeError("Fast-forward failed; validated branch and existing user work retained.")
        original_guard(checkpoint)
        worker_guard(runner, final, checkpoint, files, False)
        state.update(phase="complete", merged=True)
        runner.save_state(state_path, state)
        report.update(outcome="passed", phase="complete", master_head=checkpoint, userdata_after_sha256=sha(encoded(final.userdata(runner, PATHS))), userdata_preserved=True)
    except KeyboardInterrupt:
        failure = "Interrupted; owned checkpoints and user work retained for retry."
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
                report.update(master_head=final.bootstrap_git(origin, "rev-parse", "HEAD"), original_status=final.bootstrap_git(origin, "status", "--short"), return_file=str(directory / "ENVIRONMENT_RETURN.txt"))
                module.atomic_write(Path(report["return_file"]), encoded(report))
            except Exception as error:
                report.update(outcome="failed", finalization_error=str(error))
            runner.log.close()
        for lock in reversed(locks):
            lock.close()
        print("BEGIN_ENVIRONMENT_RETURN")
        print(json.dumps(report, indent=2))
        print("END_ENVIRONMENT_RETURN")
        if report.get("return_file"):
            print("Return file: " + report["return_file"])
    return 0 if report["outcome"] == "passed" else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.home() / "bitcoin-rs")
    args = parser.parse_args()
    return run_devshell(args.repo.expanduser().resolve(), Path(__file__).resolve())


if __name__ == "__main__":
    sys.exit(main())
