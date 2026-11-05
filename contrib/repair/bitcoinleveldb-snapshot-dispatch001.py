#!/usr/bin/env python3
"""Apply, test and commit the snapshot diagnostic repair in a linked worktree.

Run with Python 3.11+ and Nix, from any directory. No push, merge or tag.
The script saves itself in the repair commit, making the operation replayable.
On an unexpected source or Git state it stops without discarding user work.
"""
import argparse
import datetime as dt
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shlex
import shutil
import signal
import subprocess
import sys
import tempfile
import traceback
import uuid

REPAIR = "snapshot-diagnostic-dispatch001"
REVISION = "004"
PREVIOUS_SCRIPT_SHA256 = "a9534d3c8c5e78fe349314ce61f0e9aabdf6a33df64ed000ec675223892e1fbd"
BRANCH = "repair/leveldb-snapshot-diagnostic-dispatch"
SOURCE = "bitcoinleveldb-modeldb/src/snapshot_vtable.rs"
TEST_SOURCE = "bitcoinleveldbt-snapshot/tests/get_snapshot2.rs"
REPLAY = "contrib/repair/bitcoinleveldb-snapshot-dispatch001.py"
PATHS = [SOURCE, TEST_SOURCE, REPLAY]
TEST_OLD_SHA256 = "03be740b7cfb36a780b6d3d74f1152945923ea20ccbb2350329ce7d71dc61872"
TEST_NEW_SHA256 = "32b897fdc856d1f94e40e756f986a7afda9356f932b5ab14c40f7f6799af3bc3"
FOCUSED = ["cargo", "test", "--locked", "-p", "bitcoinleveldbt-snapshot",
           "--test", "get_snapshot2", "--", "--test-threads=1"]
SUITE = ["cargo", "test", "--locked", "-p", "bitcoinleveldb-modeldb",
         "-p", "bitcoinleveldbt-snapshot",
         "--", "--test-threads=1"]


class Stop(RuntimeError):
    pass


def sha(data):
    return hashlib.sha256(data).hexdigest()


def test_source_plan(data):
    """Derive the oracle from the two successful puts per completed key case."""
    actual_sha = sha(data)
    if actual_sha == TEST_NEW_SHA256:
        return data
    if actual_sha != TEST_OLD_SHA256:
        raise Stop("Unexpected identical-snapshot test source; retained without modification. "
                   "Actual SHA256=" + actual_sha)
    text = data.decode("utf-8")
    anchor = '''                let v2 = dbtest_fixture_owned_string("v2");

                let observation ='''
    replacement = '''                let v2 = dbtest_fixture_owned_string("v2");

                // Each option configuration starts with a fresh database. Each
                // completed key case adds two puts; capture follows this case's first put.
                let expected_snapshot_sequence_number: SequenceNumber =
                    (2 * i + 1) as SequenceNumber;

                let observation ='''
    assertion = "                            Some(1),"
    if text.count(anchor) != 1 or text.count(assertion) != 3:
        raise Stop("Test edit anchors do not match the inspected source.")
    text = text.replace(anchor, replacement, 1).replace(
        assertion, "                            Some(expected_snapshot_sequence_number),")
    result = text.encode("utf-8")
    if sha(result) != TEST_NEW_SHA256:
        raise Stop("Generated test differs from the reviewed replacement.")
    return result


def normalize_nixpkgs(value):
    """Use one internal schema without inventing metadata absent from Nix output."""
    if isinstance(value, str):
        value = {"outPath": value, "rev": None, "narHash": None}
    if not isinstance(value, dict):
        raise Stop("Nix source metadata must be a path string or an object; received "
                   + type(value).__name__ + ".")
    path = value.get("storePath", value.get("outPath"))
    if ("storePath" in value and "outPath" in value
            and value["storePath"] != value["outPath"]):
        raise Stop("Nix source metadata contains conflicting storePath/outPath values.")
    if not isinstance(path, str) or not path or not Path(path).is_absolute():
        raise Stop("Nix source metadata lacks a nonempty absolute storePath/outPath string.")
    result = {"outPath": path}
    for key in ("rev", "narHash"):
        field = value.get(key)
        if field is not None and (not isinstance(field, str) or not field):
            raise Stop("Nix metadata field " + key + " must be a nonempty string or null.")
        result[key] = field
    if not Path(path).is_dir():
        raise Stop("The recorded Nix source directory is unavailable: " + path)
    return result


def atomic_write(path, data, mode=0o644):
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=".repair-", dir=path.parent)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(temporary, mode)
        os.replace(temporary, path)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


class Run:
    def __init__(self, repo):
        self.repo = repo
        self.origin = repo
        self.env = dict(os.environ)
        selection = ("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR",
                     "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES",
                     "GIT_PREFIX", "GIT_NAMESPACE")
        cleared = [name for name in selection if self.env.pop(name, None) is not None]
        self.log = None
        self.directory = None
        self.report = {"repair": REPAIR, "revision": REVISION, "branch": BRANCH,
                       "coverage": "not measured", "tests": {},
                       "outcome": "failed", "phase": "preflight",
                       "cleared_git_selection_variables": cleared}
        self.lock = None

    def say(self, message):
        print(message, flush=True)
        if self.log:
            self.log.write(message + "\n")
            self.log.flush()

    def capture(self, command, check=True, cwd=None):
        result = subprocess.run(command, cwd=cwd or self.repo, env=self.env, stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE)
        if check and result.returncode:
            raise Stop(f"Command failed ({result.returncode}): {shlex.join(command)}\n"
                       + result.stderr.decode(errors="replace"))
        return result

    def git(self, *args):
        return self.capture(["git", *args]).stdout.decode().strip()

    def names(self, *args):
        raw = self.capture(["git", args[0], "-z", *args[1:]]).stdout
        return set(os.fsdecode(x) for x in raw.split(b"\0") if x)

    def stream(self, command, collect=None):
        self.say("$ " + shlex.join(command))
        with subprocess.Popen(command, cwd=self.repo, stdout=subprocess.PIPE,
                              stderr=subprocess.STDOUT, start_new_session=True, env=self.env) as child:
            try:
                for raw in child.stdout:
                    line = raw.decode(errors="replace").rstrip("\n")
                    self.say(line)
                    if collect is not None:
                        collect.append(line)
                return child.wait()
            except BaseException:
                os.killpg(child.pid, signal.SIGTERM)
                try:
                    child.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    os.killpg(child.pid, signal.SIGKILL)
                    child.wait()
                raise

    def guard_paths(self):
        for name in PATHS:
            path = self.repo / name
            for part in [path, *path.parents]:
                if part == self.repo:
                    break
                if part.is_symlink():
                    raise Stop(f"Refusing symlink in repair path: {part}")
            if not path.resolve().is_relative_to(self.repo):
                raise Stop(f"Repair path leaves repository: {path}")

    def guard_git(self, allowed_changes, expected_head=None):
        self.guard_operations()
        if self.git("symbolic-ref", "--quiet", "--short", "HEAD") != BRANCH:
            raise Stop("The current branch changed; no commit was made.")
        if expected_head and self.git("rev-parse", "HEAD") != expected_head:
            raise Stop("HEAD changed while the script was running; no commit was made.")
        changed = self.names("diff", "--name-only")
        staged = self.names("diff", "--cached", "--name-only")
        if (changed | staged) - set(allowed_changes):
            raise Stop("Unrelated tracked or staged changes appeared: "
                       + repr(sorted((changed | staged) - set(allowed_changes))))
        if self.names("diff", "--name-only", "--diff-filter=U"):
            raise Stop("Unmerged index entries need resolution.")

    def guard_operations(self):
        for operation in ("MERGE_HEAD", "CHERRY_PICK_HEAD", "REVERT_HEAD",
                          "rebase-merge", "rebase-apply", "sequencer", "index.lock"):
            path = self.git("rev-parse", "--path-format=absolute", "--git-path", operation)
            if Path(path).exists():
                raise Stop(f"Git operation in progress: {operation}")

    def guard_index(self, baseline, patched, script):
        baseline_source = self.capture(["git", "show", baseline + ":" + SOURCE]).stdout
        baseline_test = self.capture(["git", "show", baseline + ":" + TEST_SOURCE]).stdout
        patched_test = test_source_plan((self.repo / TEST_SOURCE).read_bytes())
        for name, allowed in ((SOURCE, {baseline_source, patched}),
                              (TEST_SOURCE, {baseline_test, patched_test}), (REPLAY, {script})):
            staged = self.capture(["git", "show", ":" + name], check=False)
            if staged.returncode == 0 and staged.stdout not in allowed:
                raise Stop(f"Unexpected staged content in {name}; it has been preserved.")

    def save_state(self, path, state):
        atomic_write(path, (json.dumps(state, indent=2) + "\n").encode(), 0o600)

    def source_plan(self, text):
        try:
            patched, observations = replace_functions(text, REPLACEMENTS)
        except RustSourceMismatch as error:
            self.report["source_diagnostics"] = error.diagnostics
            for item in error.diagnostics:
                if "actual_item" in item:
                    name = "actual-" + item["function"] + ".rs"
                    atomic_write(self.directory / name, item["actual_item"].encode())
            raise Stop(str(error)) from error
        self.report["source_guard"] = observations
        return patched

    def origin_snapshot(self):
        def output(*args):
            return self.capture(["git", *args], cwd=self.origin).stdout
        branch = self.capture(["git", "symbolic-ref", "--quiet", "--short", "HEAD"],
                              cwd=self.origin, check=False)
        return {"branch": branch.stdout.decode().strip() if branch.returncode == 0 else "(detached)",
                "head": output("rev-parse", "HEAD").decode().strip(),
                "index_sha256": sha(output("ls-files", "--stage", "-z")),
                "tracked_diff_sha256": sha(output("diff", "--binary", "HEAD", "--")),
                "status": output("status", "--short").decode(errors="replace").strip()}

    def origin_preserved(self, after):
        keys = ("branch", "head", "index_sha256", "tracked_diff_sha256")
        return all(self.report["origin_before"][key] == after[key] for key in keys)

    def prepare_worktree(self, script_path, metadata):
        self.guard_operations()
        origin_before = self.origin_snapshot()
        self.report.update(origin_repository=str(self.origin), origin_before=origin_before)
        self.git("ls-files", "--error-unmatch", "--", SOURCE)
        if self.names("diff", "HEAD", "--name-only", "--", SOURCE):
            raise Stop("The repair source has uncommitted edits. Its actual functions are retained; "
                       "a committed baseline is required before creating the repair worktree.")
        # Validate committed source before creating a worktree or branch.
        committed = self.capture(["git", "show", origin_before["head"] + ":" + SOURCE]).stdout
        planned = self.source_plan(committed.decode("utf-8"))
        committed_test = self.capture(["git", "show", origin_before["head"] + ":" + TEST_SOURCE]).stdout
        test_source_plan(committed_test)
        layout_path = metadata / "worktree.json"
        script_sha = sha(script_path.read_bytes())
        if layout_path.exists():
            layout = json.loads(layout_path.read_text())
            if (layout.get("repair") != REPAIR
                    or layout.get("origin") != str(self.origin) or layout.get("branch") != BRANCH):
                raise Stop("Existing repair worktree journal belongs to another source/script identity.")
            state_path = metadata / "state.json"
            state_header = json.loads(state_path.read_text()) if state_path.exists() else {}
            if (layout.get("script_sha256") != script_sha
                    or state_header.get("script_sha256") == PREVIOUS_SCRIPT_SHA256):
                self.upgrade_applied_journal(layout_path, layout, metadata, script_path.read_bytes())
        else:
            if planned.encode("utf-8") == committed:
                raise Stop("Both functions are already repaired; no new worktree/branch was created.")
            branch_exists = self.capture(["git", "show-ref", "--verify", "--quiet",
                                          "refs/heads/" + BRANCH], check=False).returncode == 0
            if branch_exists or (metadata / "state.json").exists():
                raise Stop("Repair branch/state already exists without this revision's worktree journal.")
            path = self.origin.parent / (self.origin.name + "-snapshot-dispatch-repair")
            if path.exists():
                raise Stop("Worktree destination already exists and was not created by this script: " + str(path))
            layout = {"repair": REPAIR, "revision": REVISION, "origin": str(self.origin),
                      "branch": BRANCH, "baseline": origin_before["head"], "path": str(path),
                      "script_sha256": script_sha,
                      "cargo_target_dir": str((self.origin / "target").resolve())}
            # Intent is durable before Git changes branch/worktree metadata.
            self.save_state(layout_path, layout)
        path = Path(layout["path"])
        registered = None
        for block in self.git("worktree", "list", "--porcelain").split("\n\n"):
            fields = dict(line.split(" ", 1) for line in block.splitlines() if " " in line)
            if fields.get("worktree") == str(path):
                registered = fields
                break
        if registered:
            if registered.get("branch") != "refs/heads/" + BRANCH or not path.is_dir():
                raise Stop("Recorded worktree registration differs from the repair branch/path.")
        else:
            if path.exists():
                raise Stop("Unregistered repair directory exists; retained for inspection: " + str(path))
            ref = self.capture(["git", "rev-parse", "--verify", "refs/heads/" + BRANCH], check=False)
            if ref.returncode == 0:
                if ref.stdout.decode().strip() != layout["baseline"]:
                    raise Stop("Interrupted worktree creation left an unexpected branch tip.")
                self.git("worktree", "add", str(path), BRANCH)
            else:
                self.git("worktree", "add", "-b", BRANCH, str(path), layout["baseline"])
        self.repo = path
        self.layout_baseline = layout["baseline"]
        self.report.update(worktree=str(path), cargo_target_dir=layout["cargo_target_dir"],
                           baseline=layout["baseline"])
        self.say("Isolated repair worktree: " + str(path))
        self.say("Original checkout retained on " + origin_before["branch"])
        self.cargo_target_dir = layout["cargo_target_dir"]

    def upgrade_applied_journal(self, layout_path, layout, metadata, script):
        """Advance only the known, uncommitted Revision003 repair state."""
        new_sha = sha(script)
        allowed_sha = {PREVIOUS_SCRIPT_SHA256, new_sha}
        if (layout.get("revision") not in {"003", REVISION}
                or layout.get("script_sha256") not in allowed_sha):
            raise Stop("Existing repair journal belongs to an unrecognized script revision.")
        path = Path(layout.get("path", ""))
        expected = self.origin.parent / (self.origin.name + "-snapshot-dispatch-repair")
        if path != expected or not path.is_dir() or path.is_symlink():
            raise Stop("Revision003 repair worktree path differs from its recorded location.")
        state_path = metadata / "state.json"
        if not state_path.exists():
            raise Stop("Revision003 has no applied repair state; journal retained for inspection.")
        state = json.loads(state_path.read_text())
        if (state.get("repair") != REPAIR or state.get("baseline") != layout.get("baseline")
                or state.get("script_sha256") not in allowed_sha
                or state.get("checkpoint") or state.get("blocked_checkpoint")):
            raise Stop("Revision003 state differs from the known uncommitted repair.")
        def at_worktree(*args):
            return self.capture(["git", *args], cwd=path).stdout
        if at_worktree("rev-parse", "--show-toplevel").decode().strip() != str(path):
            raise Stop("Revision003 path is not its recorded worktree root.")
        if at_worktree("symbolic-ref", "--quiet", "--short", "HEAD").decode().strip() != BRANCH:
            raise Stop("Revision003 is no longer on the recorded repair branch.")
        if at_worktree("rev-parse", "HEAD").decode().strip() != layout["baseline"]:
            raise Stop("Revision003 HEAD advanced; its baseline was not adopted.")
        if at_worktree("diff", "--cached", "--name-only").strip():
            raise Stop("Revision003 has staged changes; they were preserved.")
        changed = {os.fsdecode(x) for x in at_worktree("diff", "--name-only", "-z").split(b"\0") if x}
        untracked = {os.fsdecode(x) for x in at_worktree("ls-files", "--others", "--exclude-standard", "-z").split(b"\0") if x}
        if changed != {SOURCE} or untracked != {REPLAY}:
            raise Stop("Revision003 worktree has changes beyond the retained repair; they were preserved.")
        current = (path / SOURCE).read_bytes()
        if sha(current) != state.get("after_sha256"):
            raise Stop("Retained dispatch source differs from the Revision003 applied hash.")
        baseline_source = at_worktree("show", layout["baseline"] + ":" + SOURCE)
        if sha(baseline_source) != state.get("before_sha256"):
            raise Stop("Revision003 baseline source differs from its recorded hash.")
        self.source_plan(current.decode("utf-8"))
        if any(item["matched"] != "repaired" for item in self.report["source_guard"]):
            raise Stop("Retained dispatch functions are not the recognized repaired items.")
        test_data = (path / TEST_SOURCE).read_bytes()
        if sha(test_data) != TEST_OLD_SHA256:
            raise Stop("Revision003 test source differs from the inspected unchanged test.")
        replay = (path / REPLAY).read_bytes()
        if sha(replay) not in allowed_sha:
            raise Stop("Retained replay script differs from the known predecessor or successor.")
        registered = False
        for block in self.git("worktree", "list", "--porcelain").split("\n\n"):
            fields = dict(line.split(" ", 1) for line in block.splitlines() if " " in line)
            if fields.get("worktree") == str(path):
                registered = fields.get("branch") == "refs/heads/" + BRANCH
        if not registered:
            raise Stop("Revision003 worktree registration differs from its journal.")
        backups = [
            (metadata / "worktree-revision003-before-upgrade.json", layout_path.read_bytes(), "layout"),
            (metadata / "state-revision003-before-upgrade.json", state_path.read_bytes(), "state"),
            (metadata / "replay-revision003-before-upgrade.py", replay, "replay"),
        ]
        # Backups precede every write. Recognized partial upgrades roll forward on rerun.
        for backup, data, kind in backups:
            if backup.exists():
                saved = backup.read_bytes()
            else:
                if kind == "replay":
                    valid_old = sha(data) == PREVIOUS_SCRIPT_SHA256
                else:
                    valid_old = json.loads(data).get("script_sha256") == PREVIOUS_SCRIPT_SHA256
                if not valid_old:
                    raise Stop("Partial journal upgrade lacks its predecessor backup; state retained.")
                atomic_write(backup, data, 0o600)
                saved = data
            if kind == "replay":
                valid = sha(saved) == PREVIOUS_SCRIPT_SHA256
            else:
                prior = json.loads(saved)
                valid = (prior.get("repair") == REPAIR and prior.get("baseline") == layout["baseline"]
                         and prior.get("script_sha256") == PREVIOUS_SCRIPT_SHA256)
            if not valid:
                raise Stop("Existing predecessor backup differs from the recognized repair state.")
        atomic_write(path / REPLAY, script, 0o755)
        state.update(script_sha256=new_sha, script_revision=REVISION,
                     test_before_sha256=TEST_OLD_SHA256, test_after_sha256=TEST_NEW_SHA256)
        self.save_state(state_path, state)
        layout.update(script_sha256=new_sha, revision=REVISION)
        self.save_state(layout_path, layout)
        self.report["journal_upgrade"] = {"from_revision": "003", "to_revision": REVISION,
                                          "previous_script_sha256": PREVIOUS_SCRIPT_SHA256,
                                          "backups": [str(item[0]) for item in backups],
                                          "dispatch_source_preserved": True}
        self.say("Resuming the applied Revision003 repair; journal upgraded to Revision004.")

    def execute(self, script_path, nixpkgs_ref):
        top = Path(self.git("rev-parse", "--show-toplevel")).resolve()
        if top != self.repo:
            raise Stop("--repo must identify the repository root.")
        metadata = Path(self.git("rev-parse", "--path-format=absolute", "--git-common-dir")) / "codex-repairs" / REPAIR
        metadata.mkdir(parents=True, exist_ok=True)
        self.lock = (metadata / "run.lock").open("a+")
        try:
            fcntl.flock(self.lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            raise Stop("Another copy of this repair script is running.")
        stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
        self.directory = metadata / "runs" / (stamp + "-" + uuid.uuid4().hex[:8])
        self.directory.mkdir(parents=True)
        self.log = (self.directory / "run.log").open("w", buffering=1)
        self.say(f"Repair: {REPAIR}\nRepository: {self.repo}\nRun: {self.directory}")
        self.prepare_worktree(script_path, metadata)
        self.guard_paths()
        self.guard_operations()
        # Validate identity without printing the user's configured email address.
        self.git("var", "GIT_AUTHOR_IDENT")
        self.git("var", "GIT_COMMITTER_IDENT")
        branch_result = self.capture(["git", "symbolic-ref", "--quiet", "--short", "HEAD"], check=False)
        if branch_result.returncode:
            raise Stop("Detached HEAD: select the intended baseline branch before running this script.")
        start_branch = branch_result.stdout.decode().strip()
        start_head = self.git("rev-parse", "HEAD")
        self.report.update(start_branch=start_branch, start_head=start_head)
        self.report["untracked_at_start"] = sorted(self.names("ls-files", "--others", "--exclude-standard"))
        self.git("ls-files", "--error-unmatch", "--", SOURCE)
        original = (self.repo / SOURCE).read_bytes()
        patched_text = self.source_plan(original.decode("utf-8"))
        patched = patched_text.encode("utf-8")
        test_original = (self.repo / TEST_SOURCE).read_bytes()
        test_patched = test_source_plan(test_original)
        script = script_path.read_bytes()
        replay_path = self.repo / REPLAY
        if replay_path.exists() and replay_path.read_bytes() != script:
            raise Stop(f"Existing {REPLAY} differs from this script.")
        state_path = metadata / "state.json"
        state = json.loads(state_path.read_text()) if state_path.exists() else None
        if state:
            if state.get("baseline") != self.layout_baseline:
                raise Stop("Worktree journal and repair state disagree about the immutable baseline.")
            if state.get("blocked_checkpoint"):
                raise Stop("A previous commit/hook changed validated content. Inspect the recorded "
                           "checkpoint before further work: " + state["blocked_checkpoint"])
            if state.get("repair") != REPAIR or state.get("script_sha256") != sha(script):
                raise Stop("Saved repair state belongs to a different script revision.")
            pending_branch = (not state.get("branch_created")
                              and start_branch == state["starting_branch"]
                              and start_head == state["baseline"])
            if start_branch != BRANCH and not pending_branch:
                raise Stop(f"Resume this repair on its existing branch: git switch {BRANCH}")
            if start_head not in {state["baseline"], state.get("checkpoint")}:
                # Recover a completed own commit if interruption preceded the journal write.
                parents = self.git("show", "-s", "--format=%P", "HEAD").split()
                own_message = f"Repair: {REPAIR}" in self.git("show", "-s", "--format=%B", "HEAD")
                committed_paths = self.names("diff", "--name-only", state["baseline"], "HEAD")
                exact = all(self.capture(["git", "show", "HEAD:" + name], check=False).stdout == data
                            for name, data in ((SOURCE, patched), (TEST_SOURCE, test_patched), (REPLAY, script)))
                if parents != [state["baseline"]] or not own_message or not exact or committed_paths != set(PATHS):
                    raise Stop("Repair branch HEAD differs from its recorded baseline/checkpoint.")
                state["checkpoint"] = start_head
                self.save_state(state_path, state)
            if sha(original) not in {state["before_sha256"], state["after_sha256"]}:
                raise Stop("Source contains edits outside the recorded repair.")
            if sha(patched) != state["after_sha256"]:
                raise Stop("Repaired source differs from the saved expected result.")
            if (state.get("test_before_sha256") != TEST_OLD_SHA256
                    or state.get("test_after_sha256") != TEST_NEW_SHA256):
                raise Stop("Saved test plan differs from the inspected source and reviewed correction.")
            if state.get("checkpoint"):
                for name, expected in ((SOURCE, patched), (TEST_SOURCE, test_patched), (REPLAY, script)):
                    if self.capture(["git", "show", state["checkpoint"] + ":" + name]).stdout != expected:
                        raise Stop("Recorded checkpoint content differs from the validated repair.")
            if start_branch == BRANCH:
                self.guard_git(PATHS, start_head)
            elif self.names("diff", "--name-only") or self.names("diff", "--cached", "--name-only"):
                raise Stop("Pending branch creation requires the original clean worktree/index.")
            self.guard_index(state["baseline"], patched, script)
        else:
            if start_head != self.layout_baseline:
                raise Stop("Repair worktree advanced before state initialization; baseline was not adopted.")
            if self.names("diff", "--name-only") or self.names("diff", "--cached", "--name-only"):
                raise Stop("Start from a clean tracked worktree and index. "
                           "Existing changes have been left intact.")
            exists = self.capture(["git", "show-ref", "--verify", "--quiet",
                                   "refs/heads/" + BRANCH], check=False).returncode == 0
            if exists and start_branch != BRANCH:
                raise Stop("Repair branch already exists without this script's saved state.")
            if original == patched:
                raise Stop("Repair is already present; no new repair branch or commit needed.")
            state = {"repair": REPAIR, "baseline": start_head,
                     "starting_branch": start_branch, "script_sha256": sha(script),
                     "before_sha256": sha(original), "after_sha256": sha(patched),
                     "test_before_sha256": TEST_OLD_SHA256, "test_after_sha256": TEST_NEW_SHA256,
                     "nixpkgs_ref": nixpkgs_ref, "branch_created": False}
        self.report.update(baseline=state["baseline"], source_before_sha256=sha(original),
                           source_after_sha256=sha(patched), script_sha256=sha(script),
                           test_before_sha256=sha(test_original), test_after_sha256=sha(test_patched))
        # Resolve the development inputs once and use that exact store path for both runs.
        self.report["phase"] = "environment"
        environment = state.get("nixpkgs")
        if environment is not None:
            environment = normalize_nixpkgs(environment)
        else:
            expr = ('let f = builtins.getFlake ' + json.dumps(state["nixpkgs_ref"]) +
                    '; in { storePath = f.outPath; rev = f.rev or null; narHash = f.narHash or null; }')
            raw_environment = json.loads(self.capture(
                ["nix", "eval", "--impure", "--json", "--expr", expr]).stdout)
            self.report["nixpkgs_response_type"] = type(raw_environment).__name__
            environment = normalize_nixpkgs(raw_environment)
        state["nixpkgs"] = environment
        self.report["nixpkgs"] = environment
        shell = ('with import ' + json.dumps(environment["outPath"]) + ' {}; mkShell { '
                 'nativeBuildInputs = [ pkg-config ]; buildInputs = [ libevent openssl clang ]; '
                 'LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib"; }')
        prefix = ["nix", "develop", "--impure", "--expr", shell, "--command", "env",
                  "GIT_CONFIG_COUNT=1", "GIT_CONFIG_KEY_0=url.https://github.com/.insteadOf",
                  "GIT_CONFIG_VALUE_0=git@github.com:", "CARGO_NET_GIT_FETCH_WITH_CLI=true",
                  "CARGO_BUILD_JOBS=4", "RUST_BACKTRACE=1",
                  "CARGO_TARGET_DIR=" + self.cargo_target_dir]
        check = ["sh", "-euc", 'test -r "$LIBCLANG_PATH/libclang.so"\n'
                 'pkg-config --modversion libevent openssl\nrustc -Vv\ncargo -V\n']
        environment_lines = []
        if self.stream(prefix + check, collect=environment_lines):
            raise Stop("Development environment verification failed.")
        self.report["environment_readout"] = environment_lines
        if (self.repo / SOURCE).read_bytes() != original:
            raise Stop("Source changed during environment setup; user edits have been preserved.")
        if (self.repo / TEST_SOURCE).read_bytes() != test_original:
            raise Stop("Test source changed during setup; user edits have been preserved.")
        if replay_path.exists() and replay_path.read_bytes() != script:
            raise Stop("Replay file changed during environment setup; user edits have been preserved.")
        self.guard_index(state["baseline"], patched, script)
        # Journal intent first, so interruption on either side of branch creation is resumable.
        self.save_state(state_path, state)
        if start_branch != BRANCH:
            if self.names("diff", "--name-only") or self.names("diff", "--cached", "--name-only"):
                raise Stop("Tracked/index changes appeared during setup; no branch created.")
            self.git("switch", "-c", BRANCH)
        state["branch_created"] = True
        self.save_state(state_path, state)
        self.report["phase"] = "apply"
        self.guard_git(PATHS, start_head)
        self.guard_index(state["baseline"], patched, script)
        atomic_write(replay_path, script, 0o755)
        atomic_write(self.repo / SOURCE, patched, (self.repo / SOURCE).stat().st_mode & 0o777)
        atomic_write(self.repo / TEST_SOURCE, test_patched, (self.repo / TEST_SOURCE).stat().st_mode & 0o777)
        self.git("diff", "--check")
        self.say(f"Applied dispatch repair and schedule-derived snapshot expectations on {BRANCH}.")
        for label, command in (("focused", FOCUSED), ("crate_suites", SUITE)):
            self.report["phase"] = label
            status = self.stream(prefix + command)
            self.report["tests"][label] = {"command": command, "exit_code": status}
            if status:
                raise Stop(f"{label} tests failed with exit {status}; changes retained, no commit.")
        self.report["phase"] = "checkpoint"
        if not self.origin_preserved(self.origin_snapshot()):
            raise Stop("Original checkout changed during validation; retained for inspection, no commit.")
        self.guard_paths()
        self.guard_git(PATHS, start_head)
        self.guard_index(state["baseline"], patched, script)
        if ((self.repo / SOURCE).read_bytes() != patched or replay_path.read_bytes() != script
                or (self.repo / TEST_SOURCE).read_bytes() != test_patched):
            raise Stop("Repair files changed during validation; no commit was made.")
        self.git("diff", "--check")
        changed = self.names("diff", "HEAD", "--name-only", "--", *PATHS)
        tracked_replay = self.capture(["git", "ls-files", "--error-unmatch", "--", REPLAY],
                                     check=False).returncode == 0
        created_checkpoint = False
        if changed or not tracked_replay:
            message = ("leveldb: repair snapshot diagnostic dispatch\n\n"
                       "Use Snapshot trait methods for kind and sequence observations;\n"
                       "retain vtable addresses as diagnostic metadata only. Avoid a\n"
                       "SnapshotImpl pointer cast through forwarding wrappers.\n\n"
                       "Derive each identical-snapshot sequence expectation from the\n"
                       "two successful puts per completed key case. Preserve all value,\n"
                       "internal-history, release and compaction checks.\n\n"
                       f"Repair: {REPAIR}\nBaseline: {state['baseline']}\n"
                       f"Script-Revision: {REVISION}\nCargo-Target-Dir: {self.cargo_target_dir}\n"
                       f"Nixpkgs: {json.dumps(environment, sort_keys=True)}\n"
                       f"Source-SHA256: {sha(patched)}\nReplay-SHA256: {sha(script)}\n\n"
                       f"Test-SHA256: {sha(test_patched)}\n\n"
                       "Environment readout:\n" + "\n".join("  " + line for line in environment_lines)
                       + "\n\n"
                       "Validation (locked dependencies; serial tests):\n"
                       + "\n".join("  PASS: " + shlex.join(c) for c in (FOCUSED, SUITE))
                       + "\n\nCoverage: not measured; merge/tag checkpoint pending.\n"
                       + "Untracked files at start: " + json.dumps(self.report["untracked_at_start"]) + "\n"
                       f"Replay: {REPLAY}\n")
            message_path = self.directory / "commit-message.txt"
            message_path.write_text(message)
            self.git("add", "--", *PATHS)
            self.guard_git(PATHS, start_head)
            status = self.stream(["git", "commit", "--only", "--file", str(message_path),
                                  "--", *PATHS])
            if status:
                raise Stop("Commit failed; validated changes retained. Hooks/signing were not bypassed.")
            created_checkpoint = True
        checkpoint = self.git("rev-parse", "HEAD")
        # Record the new HEAD even if a hook changed content; never silently create another commit.
        state["checkpoint"] = checkpoint
        self.save_state(state_path, state)
        if created_checkpoint:
            parents = self.git("show", "-s", "--format=%P", checkpoint).split()
            changed_paths = self.names("diff", "--name-only", start_head, checkpoint)
            if parents != [start_head] or changed_paths != set(PATHS):
                state["blocked_checkpoint"] = checkpoint
                self.save_state(state_path, state)
                raise Stop("Commit parent or changed paths differ from the tested checkpoint.")
        for name, expected in ((SOURCE, patched), (TEST_SOURCE, test_patched), (REPLAY, script)):
            committed = self.capture(["git", "show", checkpoint + ":" + name]).stdout
            if committed != expected or (self.repo / name).read_bytes() != expected:
                state["blocked_checkpoint"] = checkpoint
                self.save_state(state_path, state)
                raise Stop("Commit/hook changed validated content; inspect the checkpoint before proceeding.")
        self.guard_git([], checkpoint)
        self.report.update(outcome="passed", phase="complete", checkpoint=checkpoint,
                           merge="pending", tag="pending")
        self.say("Validated repair checkpoint: " + checkpoint)

    def finish(self, error=None):
        if error:
            self.report["error"] = str(error)
            self.say("STOP: " + str(error))
        if self.directory:
            if "origin_before" in self.report:
                origin_after = self.origin_snapshot()
                self.report["origin_after"] = origin_after
                preserved = self.origin_preserved(origin_after)
                self.report["origin_tracked_state_preserved"] = preserved
                if not preserved and self.report["outcome"] == "passed":
                    self.report.update(outcome="failed", phase="origin_preservation",
                                       error="Original checkout changed; changes retained for inspection.")
            self.report["run_directory"] = str(self.directory)
            self.report["log"] = str(self.directory / "run.log")
            for key, command in (("final_head", ["git", "rev-parse", "HEAD"]),
                                 ("git_status", ["git", "status", "--short"])):
                result = self.capture(command, check=False)
                self.report[key] = result.stdout.decode(errors="replace").strip()
            summaries = [line for line in (self.directory / "run.log").read_text().splitlines()
                         if line.startswith("test result:")]
            self.report["test_summaries"] = summaries
            if error:
                self.report["failure_tail"] = (self.directory / "run.log").read_text().splitlines()[-100:]
            content = json.dumps(self.report, indent=2) + "\n"
            atomic_write(self.directory / "RETURN.txt", content.encode())
            self.say("\nBEGIN_REPAIR_RETURN\n" + content.rstrip() + "\nEND_REPAIR_RETURN")
            self.say("Return file: " + str(self.directory / "RETURN.txt"))
        else:
            self.say(json.dumps(self.report, indent=2))
        if self.log:
            self.log.close()
        if self.lock:
            self.lock.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.home() / "bitcoin-rs")
    parser.add_argument("--nixpkgs-ref", default="nixpkgs")
    args = parser.parse_args()
    repo = args.repo.expanduser().resolve()
    if not repo.is_dir():
        parser.error("Repository directory does not exist: " + str(repo))
    for command in ("git", "nix"):
        if not shutil.which(command):
            parser.error("Required command unavailable: " + command)
    run = Run(repo)
    failure = None
    try:
        run.execute(Path(__file__).resolve(), args.nixpkgs_ref)
    except (Stop, OSError, ValueError, KeyboardInterrupt) as error:
        failure = str(error) or "Interrupted; repair state and output retained."
    except Exception as error:
        failure = type(error).__name__ + ": " + str(error)
        run.report["exception_type"] = type(error).__name__
        run.report["exception_traceback"] = traceback.format_exc().splitlines()
    finally:
        run.finish(failure)
    return 1 if failure or run.report["outcome"] != "passed" else 0


"""Conservative Rust lexical guards for two known complete function items.

Whitespace/comments outside literals and one optional trailing comma in the
exact known parameter signature are ignored. Other token spelling remains
significant; this is a narrow lexical guard, not a semantic-equivalence proof.
Unknown/unterminated constructs stop the operation.
"""
import difflib
import re


class RustSourceMismatch(ValueError):
    def __init__(self, message, diagnostics=None):
        super().__init__(message)
        self.diagnostics = diagnostics or []


def _rust_ident_start(character):
    return character == "_" or character.isidentifier()


def _rust_ident_continue(character):
    return ("a" + character).isidentifier()


def _rust_quoted(source, start, quote):
    index = start + 1
    while index < len(source):
        if source[index] == "\\":
            index += 2
        elif source[index] == quote:
            return index + 1
        else:
            index += 1
    raise RustSourceMismatch(f"Unterminated Rust literal at character {start}.")


def _rust_char_end(source, quote):
    """Return a char-literal end, or None when the apostrophe begins a lifetime."""
    index = quote + 1
    if index >= len(source):
        return None
    if source[index] == "\\":
        index += 1
        if index >= len(source):
            raise RustSourceMismatch(f"Unterminated character escape at {quote}.")
        if source[index] == "u":
            if source[index:index + 2] != "u{":
                raise RustSourceMismatch(f"Unsupported character escape at {quote}.")
            closing = source.find("}", index + 2)
            if closing < 0 or not re.fullmatch(r"[0-9A-Fa-f_]+", source[index + 2:closing]):
                raise RustSourceMismatch(f"Malformed Unicode character escape at {quote}.")
            index = closing + 1
        elif source[index] == "x":
            if not re.fullmatch(r"[0-9A-Fa-f]{2}", source[index + 1:index + 3]):
                raise RustSourceMismatch(f"Malformed byte character escape at {quote}.")
            index += 3
        else:
            index += 1
    else:
        index += 1
    return index + 1 if index < len(source) and source[index] == "'" else None


def rust_tokens(source):
    """Return (exact_token_spelling, start_offset, end_offset) triples.

    All offsets refer to the original Python string. Literal whitespace and
    escapes remain part of a literal token. Nested block comments are skipped.
    Multi-character punctuation retains adjacency, conservatively rejecting
    edits that alter joint tokens even where Rust could parse them equivalently.
    """
    punctuation = ("<<=", ">>=", "..=", "...", "::", "->", "=>", "==", "!=",
                   "<=", ">=", "&&", "||", "<<", ">>", "+=", "-=", "*=", "/=",
                   "%=", "^=", "&=", "|=", "..")
    singles = set("+-*/%^!&|=<>@.,;:#$?~()[]{}")
    tokens = []
    index = 0
    while index < len(source):
        start = index
        ch = source[index]
        if ch.isspace():
            index += 1
            continue
        if source.startswith("//", index):
            end = source.find("\n", index + 2)
            index = len(source) if end < 0 else end + 1
            continue
        if source.startswith("/*", index):
            depth = 1
            index += 2
            while index < len(source) and depth:
                if source.startswith("/*", index):
                    depth += 1
                    index += 2
                elif source.startswith("*/", index):
                    depth -= 1
                    index += 2
                else:
                    index += 1
            if depth:
                raise RustSourceMismatch(f"Unterminated block comment at character {start}.")
            continue
        # Rust raw string prefixes: r, br, cr. A raw identifier has no quote.
        raw = re.match(r'(?:br|cr|r)(#*)"', source[index:])
        if raw:
            ending = '"' + raw.group(1)
            end = source.find(ending, index + raw.end())
            if end < 0:
                raise RustSourceMismatch(f"Unterminated raw string at character {start}.")
            index = end + len(ending)
        elif ch == '"':
            index = _rust_quoted(source, index, '"')
        elif ch in "bc" and source[index + 1:index + 2] == '"':
            index = _rust_quoted(source, index + 1, '"')
        elif ch == "b" and source[index + 1:index + 2] == "'":
            end = _rust_char_end(source, index + 1)
            if end is None:
                raise RustSourceMismatch(f"Malformed byte character at character {start}.")
            index = end
        elif ch == "'":
            end = _rust_char_end(source, index)
            if end is not None:
                index = end
            elif index + 1 < len(source) and _rust_ident_start(source[index + 1]):
                index += 2
                while index < len(source) and _rust_ident_continue(source[index]):
                    index += 1
                if index < len(source) and source[index] == "'":
                    raise RustSourceMismatch(f"Malformed character literal at character {start}.")
            else:
                raise RustSourceMismatch(f"Unsupported apostrophe at character {start}.")
        elif source.startswith("r#", index) and index + 2 < len(source) and _rust_ident_start(source[index + 2]):
            index += 3
            while index < len(source) and _rust_ident_continue(source[index]):
                index += 1
        elif _rust_ident_start(ch):
            index += 1
            while index < len(source) and _rust_ident_continue(source[index]):
                index += 1
        elif ch.isascii() and ch.isdigit():
            number = re.match(r"(?:0[xX][0-9A-Fa-f_]+|0[oO][0-7_]+|0[bB][01_]+|[0-9][0-9_]*)", source[index:])
            index += number.end()
            # Dot starts a float unless it begins a range or a field access.
            if (index < len(source) and source[index] == "."
                    and not source.startswith("..", index)
                    and (index + 1 == len(source) or not _rust_ident_start(source[index + 1]))):
                index += 1
                while index < len(source) and (source[index].isascii() and source[index].isdigit() or source[index] == "_"):
                    index += 1
            exponent = re.match(r"[eE][+-]?[0-9][0-9_]*", source[index:])
            if exponent:
                index += exponent.end()
            if index < len(source) and _rust_ident_start(source[index]):
                index += 1
                while index < len(source) and _rust_ident_continue(source[index]):
                    index += 1
        else:
            joint = next((p for p in punctuation if source.startswith(p, index)), None)
            if joint:
                index += len(joint)
            elif ch in singles:
                index += 1
            else:
                raise RustSourceMismatch(f"Unsupported Rust character {ch!r} at character {start}.")
        tokens.append((source[start:index], start, index))
    return tokens


def _rust_function_span(tokens, name):
    values = [token[0] for token in tokens]
    candidates = [i for i in range(len(tokens) - 2) if values[i:i + 3] == ["pub", "fn", name]]
    if len(candidates) != 1:
        raise RustSourceMismatch(f"Expected exactly one pub fn {name}; found {len(candidates)}.")
    start = candidates[0]
    # Include immediately preceding outer attributes so unknown attributes fail
    # the whole-item guard rather than silently remaining on the replacement.
    while start and values[start - 1] == "]":
        balance = 1
        cursor = start - 2
        while cursor >= 0 and balance:
            if values[cursor] == "]":
                balance += 1
            elif values[cursor] == "[":
                balance -= 1
            cursor -= 1
        if balance or cursor < 0 or values[cursor] != "#":
            break
        start = cursor
    cursor = candidates[0] + 3
    signature_stack = []
    pairs = {")": "(", "]": "["}
    while cursor < len(tokens):
        token = values[cursor]
        if token in ("(", "["):
            signature_stack.append(token)
        elif token in pairs:
            if not signature_stack or signature_stack.pop() != pairs[token]:
                raise RustSourceMismatch(f"Unbalanced signature in {name}.")
        elif token == "{" and not signature_stack:
            break
        elif token == ";" and not signature_stack:
            raise RustSourceMismatch(f"Declaration without a body for {name}.")
        cursor += 1
    if cursor >= len(tokens):
        raise RustSourceMismatch(f"Missing body for {name}.")
    body_depth = 1
    cursor += 1
    while cursor < len(tokens) and body_depth:
        if values[cursor] == "{":
            body_depth += 1
        elif values[cursor] == "}":
            body_depth -= 1
        cursor += 1
    if body_depth:
        raise RustSourceMismatch(f"Unterminated body for {name}.")
    return start, cursor


def _rust_item_token_values(tokens, name):
    """Normalize only the optional comma in the two known one-parameter headers.

    The exact prefix prevents this from touching tuple types, macro inputs,
    calls, generic signatures, additional parameters, or function bodies.
    """
    values = [token[0] for token in tokens]
    prefix = ["pub", "fn", name, "(", "snapshot", ":", "&", "dyn", "Snapshot"]
    if values[:len(prefix)] == prefix and values[len(prefix):len(prefix) + 2] == [",", ")"]:
        return values[:len(prefix)] + values[len(prefix) + 1:]
    return values


def replace_functions(source, replacements):
    """Validate all complete items, then return patched text and observations.

    `replacements` contains `(name, expected_old_item, replacement_item)` triples.
    The input string and filesystem are never mutated. Already-repaired items
    retain their current formatting. Unknown substantive tokens fail together
    with bounded diagnostic data, including actual complete function bodies.
    """
    tokens = rust_tokens(source)
    plans = []
    observations = []
    diagnostics = []
    for name, old, new in replacements:
        try:
            start_token, end_token = _rust_function_span(tokens, name)
        except RustSourceMismatch as error:
            diagnostics.append({"function": name, "error": str(error)})
            continue
        begin, end = tokens[start_token][1], tokens[end_token - 1][2]
        actual = source[begin:end]
        actual_items = tokens[start_token:end_token]
        old_items, new_items = rust_tokens(old), rust_tokens(new)
        actual_tokens = _rust_item_token_values(actual_items, name)
        old_tokens = _rust_item_token_values(old_items, name)
        new_tokens = _rust_item_token_values(new_items, name)
        if actual_tokens == old_tokens:
            state, replacement, reference = "old", new, old
        elif actual_tokens == new_tokens:
            state, replacement, reference = "repaired", actual, new
        else:
            mismatch = next((i for i, (left, right) in enumerate(zip(old_tokens, actual_tokens))
                             if left != right), min(len(old_tokens), len(actual_tokens)))
            unified = list(difflib.unified_diff(old.splitlines(), actual.splitlines(),
                                              fromfile="expected_old", tofile="actual", lineterm=""))
            diagnostics.append({"function": name, "error": "Rust token stream differs from both accepted items",
                                "line": source.count("\n", 0, begin) + 1,
                                "first_different_token": mismatch,
                                "expected_token_window": old_tokens[max(0, mismatch - 4):mismatch + 8],
                                "actual_token_window": actual_tokens[max(0, mismatch - 4):mismatch + 8],
                                "actual_item": actual[:24000], "actual_item_truncated": len(actual) > 24000,
                                "diff": unified[:100], "diff_truncated": len(unified) > 100})
            continue
        observations.append({"function": name, "matched": state,
                             "formatting_or_comments_differ": actual != reference,
                             "optional_signature_comma_normalized": (
                                 actual_tokens != [item[0] for item in actual_items]
                                 or (state == "old" and old_tokens != [item[0] for item in old_items])
                                 or (state == "repaired" and new_tokens != [item[0] for item in new_items])),
                             "start_line": source.count("\n", 0, begin) + 1,
                             "end_line": source.count("\n", 0, end) + 1})
        plans.append((begin, end, replacement))
    if diagnostics:
        names = ", ".join(item["function"] for item in diagnostics)
        raise RustSourceMismatch("Source guard rejected substantive or ambiguous items: " + names, diagnostics)
    previous_begin = len(source) + 1
    patched = source
    for begin, end, replacement in sorted(plans, reverse=True):
        if end > previous_begin:
            raise RustSourceMismatch("Replacement item spans overlap.")
        patched = patched[:begin] + replacement + patched[end:]
        previous_begin = begin
    return patched, observations


REPLACEMENTS = [
    ('dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref', r"""pub fn dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref(snapshot: &dyn Snapshot) -> SnapshotDispatchConcreteImplementationKind {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref_entry",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref.entry"
    );

    let snapshot_vtable_ptr_value =
        snapshot_vtable_ptr_from_snapshot_ref(snapshot) as usize;
    let model_snapshot_vtable_ptr_value =
        snapshot_model_vtable_ptr() as usize;
    let snapshot_impl_vtable_ptr_value =
        snapshot_impl_vtable_ptr() as usize;

    let implementation_kind = match (
        snapshot_vtable_ptr_value == model_snapshot_vtable_ptr_value,
        snapshot_vtable_ptr_value == snapshot_impl_vtable_ptr_value,
    ) {
        (true, false) => SnapshotDispatchConcreteImplementationKind::ModelSnapshot,
        (false, true) => SnapshotDispatchConcreteImplementationKind::SnapshotImpl,
        (false, false) => SnapshotDispatchConcreteImplementationKind::Unsupported,
        (true, true) => {
            warn!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref_ambiguous_vtable_match",
                label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref.ambiguous_vtable_match",
                snapshot_vtable_ptr_value = snapshot_vtable_ptr_value,
                model_snapshot_vtable_ptr_value = model_snapshot_vtable_ptr_value,
                snapshot_impl_vtable_ptr_value = snapshot_impl_vtable_ptr_value
            );

            SnapshotDispatchConcreteImplementationKind::Unsupported
        }
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref_exit",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref.exit",
        snapshot_vtable_ptr_value = snapshot_vtable_ptr_value,
        model_snapshot_vtable_ptr_value = model_snapshot_vtable_ptr_value,
        snapshot_impl_vtable_ptr_value = snapshot_impl_vtable_ptr_value,
        implementation_kind = ?implementation_kind
    );

    implementation_kind
}""", r"""pub fn dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref(snapshot: &dyn Snapshot) -> SnapshotDispatchConcreteImplementationKind {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref_entry",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref.entry"
    );

    let snapshot_vtable_ptr_value =
        snapshot_vtable_ptr_from_snapshot_ref(snapshot) as usize;
    let model_snapshot_vtable_ptr_value =
        snapshot_model_vtable_ptr() as usize;
    let snapshot_impl_vtable_ptr_value =
        snapshot_impl_vtable_ptr() as usize;

    // Vtable addresses are diagnostic metadata, not stable type identities.
    let implementation_kind = snapshot.snapshot_runtime_implementation_kind();

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref_exit",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref.exit",
        snapshot_vtable_ptr_value = snapshot_vtable_ptr_value,
        model_snapshot_vtable_ptr_value = model_snapshot_vtable_ptr_value,
        snapshot_impl_vtable_ptr_value = snapshot_impl_vtable_ptr_value,
        implementation_kind = ?implementation_kind
    );

    implementation_kind
}"""),
    ('dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref', r"""pub fn dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(snapshot: &dyn Snapshot) -> BitcoinLevelDbTestSnapshotDispatchConcreteImplementationObservation {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_entry",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.entry"
    );

    let (snapshot_data_ptr, snapshot_vtable_ptr) =
        snapshot_trait_object_parts(snapshot);

    let snapshot_data_ptr_value = snapshot_data_ptr as usize;
    let snapshot_vtable_ptr_value = snapshot_vtable_ptr as usize;
    let model_snapshot_vtable_ptr_value = snapshot_model_vtable_ptr() as usize;
    let snapshot_impl_vtable_ptr_value = snapshot_impl_vtable_ptr() as usize;

    let implementation_kind =
        dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref(
            snapshot,
        );

    let snapshot_sequence_number_hint = match implementation_kind {
        SnapshotDispatchConcreteImplementationKind::ModelSnapshot => None,
        SnapshotDispatchConcreteImplementationKind::SnapshotImpl => {
            let snapshot_impl_ptr = snapshot_data_ptr as *const SnapshotImpl;

            match snapshot_impl_ptr.is_null() {
                true => {
                    warn!(
                        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                        event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_null_snapshot_impl_ptr",
                        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.null_snapshot_impl_ptr"
                    );
                    None
                }
                false => {
                    let sequence_number = unsafe {
                        *(*snapshot_impl_ptr).sequence_number()
                    };
                    Some(sequence_number)
                }
            }
        }
        SnapshotDispatchConcreteImplementationKind::Unsupported => None,
    };

    let observation_builder =
        BitcoinLevelDbTestSnapshotDispatchConcreteImplementationObservationBuilder::default()
            .snapshot_data_ptr_value(snapshot_data_ptr_value)
            .snapshot_vtable_ptr_value(snapshot_vtable_ptr_value)
            .model_snapshot_vtable_ptr_value(model_snapshot_vtable_ptr_value)
            .snapshot_impl_vtable_ptr_value(snapshot_impl_vtable_ptr_value)
            .implementation_kind(implementation_kind)
            .snapshot_sequence_number_hint(snapshot_sequence_number_hint);

    let observation = match observation_builder.build() {
        Ok(value) => value,
        Err(builder_error) => {
            error!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_builder_failed",
                label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.builder_failed",
                builder_error = ?builder_error
            );
            panic!();
        }
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_exit",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.exit",
        snapshot_data_ptr_value = *observation.snapshot_data_ptr_value(),
        snapshot_vtable_ptr_value = *observation.snapshot_vtable_ptr_value(),
        model_snapshot_vtable_ptr_value = *observation.model_snapshot_vtable_ptr_value(),
        snapshot_impl_vtable_ptr_value = *observation.snapshot_impl_vtable_ptr_value(),
        implementation_kind = ?observation.implementation_kind(),
        snapshot_sequence_number_hint = ?observation.snapshot_sequence_number_hint()
    );

    observation
}""", r"""pub fn dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(snapshot: &dyn Snapshot) -> BitcoinLevelDbTestSnapshotDispatchConcreteImplementationObservation {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_entry",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.entry"
    );

    let (snapshot_data_ptr, snapshot_vtable_ptr) =
        snapshot_trait_object_parts(snapshot);

    let snapshot_data_ptr_value = snapshot_data_ptr as usize;
    let snapshot_vtable_ptr_value = snapshot_vtable_ptr as usize;
    let model_snapshot_vtable_ptr_value = snapshot_model_vtable_ptr() as usize;
    let snapshot_impl_vtable_ptr_value = snapshot_impl_vtable_ptr() as usize;

    let implementation_kind =
        dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref(
            snapshot,
        );

    // Use the trait method so forwarding wrappers preserve their own layout.
    let snapshot_sequence_number_hint =
        snapshot.snapshot_sequence_number_for_read_reconstruction();

    let observation_builder =
        BitcoinLevelDbTestSnapshotDispatchConcreteImplementationObservationBuilder::default()
            .snapshot_data_ptr_value(snapshot_data_ptr_value)
            .snapshot_vtable_ptr_value(snapshot_vtable_ptr_value)
            .model_snapshot_vtable_ptr_value(model_snapshot_vtable_ptr_value)
            .snapshot_impl_vtable_ptr_value(snapshot_impl_vtable_ptr_value)
            .implementation_kind(implementation_kind)
            .snapshot_sequence_number_hint(snapshot_sequence_number_hint);

    let observation = match observation_builder.build() {
        Ok(value) => value,
        Err(builder_error) => {
            error!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_builder_failed",
                label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.builder_failed",
                builder_error = ?builder_error
            );
            panic!();
        }
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_exit",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.exit",
        snapshot_data_ptr_value = *observation.snapshot_data_ptr_value(),
        snapshot_vtable_ptr_value = *observation.snapshot_vtable_ptr_value(),
        model_snapshot_vtable_ptr_value = *observation.model_snapshot_vtable_ptr_value(),
        snapshot_impl_vtable_ptr_value = *observation.snapshot_impl_vtable_ptr_value(),
        implementation_kind = ?observation.implementation_kind(),
        snapshot_sequence_number_hint = ?observation.snapshot_sequence_number_hint()
    );

    observation
}"""),
]

if __name__ == "__main__":
    sys.exit(main())
