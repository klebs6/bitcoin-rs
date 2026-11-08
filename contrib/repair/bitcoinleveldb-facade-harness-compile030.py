#!/usr/bin/env python3
"""Compile-only facade/harness repair030: two exact import-binding fixes, two fresh guarded no-run builds, isolated candidate commit on compilation success; no native tests or master updates."""
import argparse
import datetime as dt
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shlex
import shutil
import signal
import stat
import subprocess
import sys
import types
import uuid
MIB027 = 1024 * 1024
WORKFLOW = 'leveldb-facade-harness-compile030'
BASE = '2a5093e69dd91dc6d770e7e8ed1d201751f9ad5c'
BASE_TREE = '061d6ba025e0d2803bf4add4029aebc978d1fbfd'
BRANCH = 'repair/leveldb-facade-harness-compile030'
SUFFIX = '-leveldb-facade-harness-compile030'
REPLAY_PATH = 'contrib/repair/bitcoinleveldb-facade-harness-compile030.py'
DOCUMENT_PATH = 'doc/testing/leveldb-facade-harness-compile030.md'
RECEIPT_PATH = 'doc/repair/leveldb-facade-harness-compile030.json'
COMMIT_MESSAGE = b'fix: restore LevelDB facade and harness import bindings\n\nRemove two undeclared facade reexports and expose the existing shared test\nargument types through DBtest. Qualify both guarded no-run compilations;\nnative tests and runtime repair qualification remain pending.\n'


SOURCE_PAYLOADS = {'bitcoin-leveldb/src/lib.rs': b'// ---------------- [ File: bitcoin-leveldb/src/lib.rs ]\npub use bitcoinleveldb_arena::*;\npub use bitcoinleveldb_batch::*;\npub use bitcoinleveldb_bloom::*;\npub use bitcoinleveldb_cache::*;\npub use bitcoinleveldb_coding::*;\npub use bitcoinleveldb_comparator::*;\npub use bitcoinleveldb_compat::*;\npub use bitcoinleveldb_crc32::*;\npub use bitcoinleveldb_db::*;\npub use bitcoinleveldb_dumpfile::*;\npub use bitcoinleveldb_duplex::*;\npub use bitcoinleveldb_env::*;\npub use bitcoinleveldb_file::*;\npub use bitcoinleveldb_filter::*;\npub use bitcoinleveldb_hash::*;\npub use bitcoinleveldb_histogram::*;\npub use bitcoinleveldb_limiter::*;\npub use bitcoinleveldb_log::*;\npub use bitcoinleveldb_lru::*;\npub use bitcoinleveldb_memenv::*;\npub use bitcoinleveldb_memtable::*;\npub use bitcoinleveldb_merger::*;\npub use bitcoinleveldb_options::*;\npub use bitcoinleveldb_posix::*;\npub use bitcoinleveldb_rand::*;\npub use bitcoinleveldb_repair::*;\npub use bitcoinleveldb_skiplist::*;\npub use bitcoinleveldb_slice::*;\npub use bitcoinleveldb_status::*;\npub use bitcoinleveldb_sync::*;\npub use bitcoinleveldb_table::*;\npub use bitcoinleveldb_util::*;\n', 'bitcoinleveldbt-dbtest/src/lib.rs': b'// ---------------- [ File: bitcoinleveldbt-dbtest/src/lib.rs ]\n#[macro_use] mod imports; use imports::*;\n\npub use bitcoinleveldbt_util::{TestArgs, TestType};\n\n//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_test.cc]\nx!{dbtest}\nx!{fixture}\nx!{probes}\n'}
DOCUMENT_BYTES = b'# Facade and harness compilation repair030\n\nThe completed baseline028 run observed two unresolved reexports in the facade\nand ten unresolved references to shared test argument types in the harness.\nThis candidate repairs those import bindings before native test execution.\n\nThe source delta is exactly two library roots:\n\n- Remove `pub use bitcoinleveldbt_bench::*;` and\n  `pub use bitcoinleveldb_meta::*;` from `bitcoin-leveldb/src/lib.rs`.\n  Neither is a declared facade dependency; the captured metadata crate is\n  absent. The remaining 32 reexports keep their original order.\n- Add `pub use bitcoinleveldbt_util::{TestArgs, TestType};` to\n  `bitcoinleveldbt-dbtest/src/lib.rs`. This explicitly exposes the existing\n  owner types through DBtest, matching both the harness library and its\n  integration test imports. It does not create replacement types or change\n  their identity. DBtest already declares the utility dependency.\n\nAll manifests, lockfiles, harness implementation and test bodies, workloads,\noracles and execution policies are unchanged. The two unresolved facade names\ndid not provide a usable API in the failed baseline; the DBtest reexport is an\nadditive compatibility surface.\n\nThe runner pins master `2a5093e69dd91dc6d770e7e8ed1d201751f9ad5c` and the exact\nuploaded baseline028 report. It preserves the original user checkout, all old\nbranches/worktrees/journals/reports and the baseline cache. A new admission\nprobe runs before any new candidate/cache reservation. A handled RAM refusal\ncreates no candidate journal or checkout and may be retried. Once an attempt\nreserves candidate state, automatic recompilation is blocked and its evidence\nis retained for review.\n\nThe new candidate uses branch `repair/leveldb-facade-harness-compile030` and an\nindependent worktree and cache. Optional cache bytes are copied independently\nunder the existing 4 GiB seed allowance, without hardlinks, incremental data or\nnative test executables. Each compilation has one CPU, no swap, a 4 GiB memory\ncap, 3 GiB high threshold, 128 tasks, 600 seconds, a 512 MiB per-file limit and\n2 MiB output allowance. Every guarded phase rechecks resource controls and\nheadroom. No budget is weakened to obtain admission.\n\nThe two exact commands run separately through the offline development shell:\n\n```text\ncargo test --offline --locked --jobs 1 -p bitcoin-leveldb --lib --tests --no-run --message-format=json\ncargo test --offline --locked --jobs 1 -p bitcoinleveldb-harness --lib --tests --no-run --message-format=json\n```\n\nA controlled Cargo101 failure is retained, and the other crate is still\ncompiled. A guard/protocol failure stops the attempt. A candidate commit is\ncreated only after both fresh compilations pass and the one facade plus two\nharness executables are bound to their package, target, source, private path,\nordinary executable mode and current SHA256 fingerprints. The receipt and\ncontroller are committed with the two source edits and this document. Master\nand all existing refs remain unchanged.\n\n`compile_qualified: true` establishes compilation only. `repair_qualified`\nremains false, the test count remains zero, and no native test artifact is\nexecuted, including `--list`. Harness runtime control and qualification follow\nafter reviewing the compiled result. This run does not establish the cause of\nearlier workstation lockups or claim full coverage.\n\nThe outer transport replaces the authorized original `return-buffer` only\nafter the coordinator exits and exact printed/saved report and preservation\nchecks agree. Its finite transport output also includes the complete saved\n`COMPILE_REPAIR_RETURN.json`.\n'
INITIAL_PATHS = tuple(sorted(SOURCE_PAYLOADS) + [REPLAY_PATH, DOCUMENT_PATH])


def bootstrap_read(path, limit):
    path = Path(path)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Expected an absolute ordinary path.')
    fd = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:-1]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd); fd = child
        child = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=fd)
        with os.fdopen(child,'rb') as stream:
            before = os.fstat(stream.fileno())
            if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
                raise RuntimeError('File is indirect or too large.')
            raw = stream.read(limit+1)
            after = os.fstat(stream.fileno())
        fields = ('st_dev','st_ino','st_mode','st_size','st_mtime_ns','st_ctime_ns')
        current = path.lstat()
        if len(raw)>limit or any(getattr(before,k)!=getattr(after,k) or getattr(before,k)!=getattr(current,k) for k in fields):
            raise RuntimeError('File changed during read.')
        return raw
    finally:
        os.close(fd)


def _candidate_initial023():
    """The embedding payload supplies its exact finite path inventory."""
    paths = globals().get('INITIAL_PATHS')
    if (not isinstance(paths, (list, tuple, set, frozenset)) or not paths
            or len(set(paths)) != len(paths) or not {REPLAY_PATH, DOCUMENT_PATH}.issubset(paths)
            or RECEIPT_PATH in paths):
        raise RuntimeError('Embedding randomized payload did not supply its exact initial path inventory.')
    for name in paths:
        if not isinstance(name, str): raise RuntimeError('Candidate source path is not text.')
        path = Path(name)
        if (not re.fullmatch(r'[A-Za-z0-9_./-]+', name) or path.is_absolute() or '..' in path.parts
                or str(path) != name or path.parts[0] == '.git'):
            raise RuntimeError('Candidate source path is outside its literal relative plan.')
    return frozenset(paths)

def _candidate_oid023(value):
    return isinstance(value, str) and re.fullmatch(r'[0-9a-f]{40}', value) is not None

def _candidate_blob023(raw):
    return hashlib.sha1(b'blob ' + str(len(raw)).encode() + b'\0' + raw).hexdigest()

def _candidate_files023(core, files):
    if (not isinstance(files, dict) or set(files) not in (_candidate_initial023(), _candidate_initial023() | {RECEIPT_PATH})
            or any(not isinstance(raw, bytes) or len(raw) > 2 * core.MIB for raw in files.values())):
        raise RuntimeError('Randomized candidate requires the exact planned initial/final bounded file payloads.')
    return files

def candidate_original_guard023(core, repo):
    """Verify original master identity while allowing unrelated staged/user work."""
    repo = Path(repo)
    if not repo.is_absolute() or '..' in repo.parts:
        raise RuntimeError('Original Randomized repository is not an absolute ordinary checkout.')
    fd = core.open_dir(repo); os.close(fd)
    inv = core.Inventory(repo); core.repository(inv)
    _, head = inv.git('rev-parse', '--verify', 'HEAD')
    _, branch = inv.git('symbolic-ref', '--quiet', 'HEAD')
    _, tree = inv.git('rev-parse', '--verify', 'HEAD^{tree}')
    if (head.decode().strip(), branch.decode().strip(), tree.decode().strip()) != (BASE, 'refs/heads/master', BASE_TREE):
        raise RuntimeError('Original master differs from the exact reviewed Randomized base/tree.')
    code, filters = inv.git('config', '--name-only', '--get-regexp', r'^filter\.', allowed=(0, 1))
    if code == 0 and filters.strip():
        raise RuntimeError('Configured Git filters require review before candidate inspection.')
    return inv

def _candidate_location023(core, repo, worktree, expected_head):
    repo, worktree = Path(repo), Path(worktree)
    if (not _candidate_oid023(expected_head) or not worktree.is_absolute() or '..' in worktree.parts
            or worktree != repo.parent / (repo.name + SUFFIX)):
        raise RuntimeError('Randomized candidate location or expected head differs from the isolated plan.')
    fd = core.open_dir(worktree); os.close(fd)
    inv = candidate_original_guard023(core, repo)
    _, top = inv.git('rev-parse', '--show-toplevel', cwd=worktree)
    _, common = inv.git('rev-parse', '--path-format=absolute', '--git-common-dir', cwd=worktree)
    _, head = inv.git('rev-parse', '--verify', 'HEAD', cwd=worktree)
    _, branch = inv.git('symbolic-ref', '--quiet', 'HEAD', cwd=worktree)
    if (Path(os.fsdecode(top).strip()) != worktree or Path(os.fsdecode(common).strip()) != repo / '.git'
            or head.decode().strip() != expected_head or branch.decode().strip() != 'refs/heads/' + BRANCH):
        raise RuntimeError('Randomized candidate top level, common root, branch or checkpoint differs.')
    code, filters = inv.git('config', '--name-only', '--get-regexp', r'^filter\.', allowed=(0, 1), cwd=worktree)
    if code == 0 and filters.strip():
        raise RuntimeError('Configured candidate Git filters require review before index/source inspection.')
    _, flags = inv.git('ls-files', '-v', '-z', cwd=worktree)
    if any(row and row[:2] != b'H ' for row in flags.split(b'\0')):
        raise RuntimeError('Randomized candidate index contains a hidden/nonstandard visibility flag.')
    return inv

def _candidate_reservation023(core, inv, worktree, files, expected_head):
    if expected_head != BASE or set(files) != _candidate_initial023():
        raise RuntimeError('Pristine reservation validation is restricted to BASE and the initial planned file scope.')
    _, status = inv.git('status', '--porcelain=v1', '-z', '--untracked-files=all', cwd=worktree)
    _, untracked = inv.git('ls-files', '--others', '-z', cwd=worktree)
    if status or untracked:
        raise RuntimeError('Randomized reservation must remain a pristine BASE checkout before installation.')
    _, tree = inv.git('ls-tree', '-r', '-z', BASE, '--', *sorted(files), cwd=worktree)
    existing = {}
    for row in tree.split(b'\0'):
        if not row: continue
        header, name = row.split(b'\t', 1); name = os.fsdecode(name)
        mode, kind, blob = header.decode('ascii').split()
        if name not in files or name in existing or mode not in {'100644', '100755'} or kind != 'blob' or not _candidate_oid023(blob):
            raise RuntimeError('Reserved randomized BASE inputs have an unexpected path/type.')
        existing[name] = (mode, blob)
    for name in sorted(set(files) | {RECEIPT_PATH}):
        path = worktree / name
        descriptor = core.open_dir(worktree)
        try:
            for part in Path(name).parts[:-1]:
                try:
                    child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=descriptor)
                except FileNotFoundError:
                    break
                os.close(descriptor); descriptor = child
        finally:
            os.close(descriptor)
        if name not in existing:
            if os.path.lexists(path):
                raise RuntimeError('A new randomized candidate path is occupied before source installation: ' + name)
            continue
        mode, blob = existing[name]
        _, committed = inv.git('cat-file', 'blob', blob, cwd=worktree)
        fingerprint, live = core.read_file(path, 2 * core.MIB)
        if live != committed or fingerprint['mode'] != (int(mode, 8) & 0o777):
            raise RuntimeError('Reserved randomized source differs from committed BASE bytes/mode: ' + name)
    return {'head': expected_head, 'branch': 'refs/heads/' + BRANCH, 'phase': 'pristine_base',
            'changed_paths': [], 'index_entries': {}}

def check_candidate023(core, repo, worktree, files, expectedhead, allow_base_files=False):
    """Validate pristine reservation, exact staged initial/final scope, or exact committed final scope."""
    if type(allow_base_files) is not bool:
        raise RuntimeError('Randomized reservation-mode flag must be boolean.')
    files = _candidate_files023(core, files)
    repo, worktree = Path(repo), Path(worktree)
    inv = _candidate_location023(core, repo, worktree, expectedhead)
    if allow_base_files:
        return _candidate_reservation023(core, inv, worktree, files, expectedhead)
    staged = expectedhead == BASE
    if not staged and set(files) != _candidate_initial023() | {RECEIPT_PATH}:
        raise RuntimeError('Committed Randomized qualification requires the complete planned final scope.')
    _, unstaged = inv.git('diff', '--name-only', '--no-ext-diff', '--no-textconv', '-z', cwd=worktree)
    _, untracked = inv.git('ls-files', '--others', '-z', cwd=worktree)
    if unstaged or untracked:
        raise RuntimeError('Randomized candidate has unstaged or untracked bytes outside its exact installed tree.')
    _, raw_staged = inv.git('diff', '--cached', '--name-only', '--no-ext-diff', '--no-textconv', '-z', cwd=worktree)
    changed = [os.fsdecode(item) for item in raw_staged.split(b'\0') if item]
    wanted = set(files) if staged else set()
    if set(changed) != wanted or len(changed) != len(wanted):
        raise RuntimeError('Randomized candidate staged paths differ from the exact installed scope.')
    expected = {}
    for name, data in files.items():
        fingerprint, live = core.read_file(worktree / name, 2 * core.MIB)
        mode = '100755' if name == REPLAY_PATH else '100644'
        if live != data or fingerprint['mode'] != (int(mode, 8) & 0o777):
            raise RuntimeError('Randomized candidate live bytes or mode differ: ' + name)
        expected[name] = [mode, _candidate_blob023(data)]
    _, raw_index = inv.git('ls-files', '--stage', '-z', '--', *sorted(files), cwd=worktree)
    indexed = {}
    for row in raw_index.split(b'\0'):
        if not row:
            continue
        header, name = row.split(b'\t', 1)
        mode, blob, stage = header.decode('ascii').split()
        name = os.fsdecode(name)
        if stage != '0' or name in indexed or not _candidate_oid023(blob):
            raise RuntimeError('Randomized candidate index has ambiguous/unmerged entries.')
        indexed[name] = [mode, blob]
    if indexed != expected:
        raise RuntimeError('Randomized candidate index blobs/modes differ from the reviewed live bytes.')
    result = {'head': expectedhead, 'branch': 'refs/heads/' + BRANCH,
              'phase': 'staged' if staged else 'committed', 'changed_paths': sorted(files), 'index_entries': indexed}
    if not staged:
        _, parents = inv.git('rev-list', '--parents', '-n', '1', expectedhead, cwd=worktree)
        if parents.decode().split() != [expectedhead, BASE]:
            raise RuntimeError('Randomized committed candidate must have the exact single BASE parent.')
        _, raw = inv.git('diff-tree', '--no-commit-id', '-r', '--raw', '-z', '--no-abbrev',
                         '--no-renames', '--no-ext-diff', '--no-textconv', BASE, expectedhead, cwd=worktree)
        parts = raw.split(b'\0')
        if parts and not parts[-1]: parts.pop()
        if len(parts) != 2 * len(files):
            raise RuntimeError('Randomized committed candidate has a different path count.')
        committed = {}
        for offset in range(0, len(parts), 2):
            fields = parts[offset].decode('ascii').split(); name = os.fsdecode(parts[offset + 1])
            if (len(fields) != 5 or not fields[0].startswith(':') or fields[4] not in {'A', 'M'}
                    or name in committed):
                raise RuntimeError('Randomized committed candidate has an unexpected operation.')
            committed[name] = [fields[1], fields[3]]
        if committed != expected:
            raise RuntimeError('Randomized committed candidate source/path/mode scope differs.')
        _, tree = inv.git('rev-parse', '--verify', expectedhead + '^{tree}', cwd=worktree)
        result['tree'] = tree.decode().strip()
    return result

def configure_git_helper023(helper):
    """Configure only the pinned015 generic finite Git/commit helper surface."""
    helper.WORKFLOW, helper.BASE, helper.BRANCH, helper.SUFFIX = WORKFLOW, BASE, BRANCH, SUFFIX
    helper.REPLAY_PATH, helper.DOCUMENT_PATH, helper.RECEIPT_PATH = REPLAY_PATH, DOCUMENT_PATH, RECEIPT_PATH
    helper._w015_initial_paths = lambda: set(_candidate_initial023())
    helper._w015_original = candidate_original_guard023
    def exact_candidate(core, repo, worktree, files, head, staged=True):
        if type(staged) is not bool or staged != (head == BASE):
            raise RuntimeError('Randomized Git helper requested a different candidate phase.')
        return check_candidate023(core, repo, worktree, files, head)
    helper.check_candidate = exact_candidate
    return helper


"""Finite Cargo observation for the two reviewed facade and harness build commands.

The parent must launch this observer under the reviewed resource guard. This
module never builds unless cargo_observer is explicitly invoked with one of two exact
offline no-run command shapes. Cargo stderr remains attached to that guard.
"""
import json
import os
from pathlib import Path
import re
import selectors
import subprocess
import sys
import time
import uuid
from urllib.parse import unquote, urlsplit

MAX_MACHINE_BYTES = 64 * 1024 * 1024
MAX_RECORD_BYTES = 1024 * 1024
MAX_RECORDS = 50000
MAX_EVIDENCE_BYTES = 512 * 1024
MAX_ERROR_RECORD_BYTES = 16 * 1024
MAX_ERROR_RECORDS = 32
MAX_ERROR_BYTES = 256 * 1024
MAX_WARNING_RENDERED_BYTES = 8 * 1024
MAX_ERROR_SPANS = 8
OBSERVER_SECONDS = 580
ALLOWED_PACKAGES = ("bitcoin-leveldb", "bitcoinleveldb-harness")
COMMANDS027 = {
    "facade": ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", "bitcoin-leveldb",
               "--lib", "--tests", "--no-run", "--message-format=json"],
    "harness": ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", "bitcoinleveldb-harness",
                "--lib", "--tests", "--no-run", "--message-format=json"],
}
EXPECTED_TARGETS027 = {
    "facade": {"bitcoin_leveldb": ("lib", "src/lib.rs")},
    "harness": {"bitcoinleveldb_harness": ("lib", "src/lib.rs"),
                "harness": ("test", "tests/harness.rs")},
}
# Keep the reviewed grammar and target bindings independent of caller-visible
# convenience maps. The package is derived from the exact matched command.
_COMMAND_SHAPES027 = tuple((label, tuple(command)) for label, command in COMMANDS027.items())
_TARGET_SHAPES027 = {label: tuple(targets.items()) for label, targets in EXPECTED_TARGETS027.items()}



MAX_EXECUTABLES = 40


def _observer_spec(spec):
    if not isinstance(spec, dict):
        raise RuntimeError("Cargo observer specification is not an object.")
    command = spec.get("command")
    if not isinstance(command, list):
        raise RuntimeError("Cargo observer command must be the exact reviewed argument list.")
    matches = [label for label, shape in _COMMAND_SHAPES027 if tuple(command) == shape]
    if len(matches) != 1:
        raise RuntimeError("Cargo observer requires one of the two exact reviewed facade/harness library/test commands; no alternate command is admitted.")
    label = matches[0]
    expected = [command[7]]
    targets = dict(_TARGET_SHAPES027[label])
    supplied = spec.get("expected_packages", expected)
    if not isinstance(supplied, list) or supplied != expected:
        raise RuntimeError("Cargo expected package must equal the singleton derived from the exact command.")
    paths = {}
    for key in ("cwd", "target_directory", "evidence_path"):
        value = spec.get(key)
        if not isinstance(value, str) or not value or "\x00" in value:
            raise RuntimeError("Cargo observer path is absent or malformed: " + key)
        path = Path(value)
        if not path.is_absolute() or ".." in path.parts:
            raise RuntimeError("Cargo observer requires an absolute path: " + key)
        paths[key] = path
    if not paths["cwd"].is_dir() or not paths["evidence_path"].parent.is_dir():
        raise RuntimeError("Cargo observer working or evidence directory is absent.")
    if os.path.lexists(paths["evidence_path"]):
        raise RuntimeError("Cargo observer refuses to replace existing evidence.")
    paths["expected_packages"] = tuple(expected)
    paths["expected_artifacts"] = {
        expected[0] + "::" + name: {"kind": kind, "source": paths["cwd"] / expected[0] / source}
        for name, (kind, source) in targets.items()
    }
    return list(command), expected, paths



def _atomic_evidence(path, record):
    raw = json.dumps(record, sort_keys=True, ensure_ascii=False, separators=(",", ":"), allow_nan=False).encode("utf-8") + b"\n"
    if len(raw) > MAX_EVIDENCE_BYTES:
        raise RuntimeError("Cargo evidence exceeds its retained-byte allowance.")
    path = Path(path)
    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        # Linking installs the completed file atomically and cannot overwrite a
        # concurrently created receipt. Both files are in the same directory.
        os.link(temporary, path, follow_symlinks=False)
    finally:
        try:
            temporary.unlink()
        except FileNotFoundError:
            pass


def _stop_cargo(child):
    if child.poll() is None:
        child.terminate()
        try:
            child.wait(timeout=2)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait(timeout=2)
    else:
        child.wait(timeout=2)


def _artifact(record, expected, paths, evidence):
    target = record.get("target")
    profile = record.get("profile")
    if not isinstance(target, dict) or not isinstance(profile, dict):
        raise RuntimeError("Cargo compiler artifact lacks target/profile data.")
    if target.get("kind") == ["custom-build"]:
        evidence["build_script_artifacts_discarded"] += 1
        return
    executable = record.get("executable")
    if executable is None:
        evidence["dependency_artifacts_discarded"] += 1
        return
    kind = target.get("kind")
    if profile.get("test") is not True or kind not in (["lib"], ["test"]):
        raise RuntimeError("Cargo emitted an executable outside the expected lib/test profile.")
    name = target.get("name")
    if not isinstance(name, str) or not re.fullmatch(r"[A-Za-z0-9_][A-Za-z0-9_-]{0,127}", name):
        raise RuntimeError("Cargo test target name is malformed.")
    source = target.get("src_path")
    if not isinstance(source, str) or not source or len(source) > 4096 or "\x00" in source:
        raise RuntimeError("Cargo test source path is absent or malformed.")
    source_path = Path(source)
    if not source_path.is_absolute() or ".." in source_path.parts:
        raise RuntimeError("Cargo test source path is not an absolute package-local path.")
    try:
        relative = source_path.relative_to(paths["cwd"])
    except ValueError as error:
        raise RuntimeError("Cargo test source is outside the private baseline worktree.") from error
    if len(relative.parts) < 2 or relative.parts[0] not in expected:
        raise RuntimeError("Cargo test source does not belong to a reviewed package.")
    package_name = relative.parts[0]
    package_id = record.get("package_id")
    if not isinstance(package_id, str) or not package_id or len(package_id) > 4096:
        raise RuntimeError("Cargo package identity is absent or too long.")
    # Modern Cargo path IDs may end in either #version or #name@version.
    # Source membership, rather than a guessed fragment format, selects the
    # package. A supplied path identity must independently name the same root.
    if package_id.startswith("path+"):
        identity = urlsplit(package_id[5:])
        identity_path = Path(unquote(identity.path))
        if (identity.scheme != "file" or identity.netloc not in ("", "localhost")
                or not identity_path.is_absolute() or ".." in identity_path.parts
                or identity_path != paths["cwd"] / package_name):
            raise RuntimeError("Cargo path package identity differs from its source package.")
    elif "://" in package_id:
        raise RuntimeError("Cargo expected local source has an external package identity.")
    key = package_name + "::" + name
    binding = paths["expected_artifacts"].get(key)
    if binding is None or kind != [binding["kind"]] or source_path != binding["source"]:
        raise RuntimeError("Cargo executable target kind/name/source differs from the exact selected command.")
    if key in evidence["artifacts"]:
        raise RuntimeError("Cargo emitted a duplicate package/target test executable.")
    if len(evidence["artifacts"]) >= MAX_EXECUTABLES:
        raise RuntimeError("Cargo test executable count exceeds its allowance.")
    if not isinstance(executable, str) or not executable or len(executable) > 4096 or "\x00" in executable:
        raise RuntimeError("Cargo executable path is malformed or too long.")
    executable_path = Path(executable)
    prefixes = {name, name.replace("-", "_")}
    filename_ok = any(re.fullmatch(re.escape(prefix) + r"-[0-9a-f]+", executable_path.name) for prefix in prefixes)
    if (not executable_path.is_absolute() or ".." in executable_path.parts
            or executable_path.parent != paths["target_directory"] / "debug" / "deps"
            or not filename_ok):
        raise RuntimeError("Cargo executable does not belong to the isolated debug test directory.")
    if any(item["executable"] == executable for item in evidence["artifacts"].values()):
        raise RuntimeError("Cargo reused an executable path for distinct package/target identities.")
    evidence["artifacts"][key] = {
        "package": package_name, "target_name": name, "kind": list(kind),
        "src_path": source, "executable": executable,
        "package_id": package_id, "fresh": record.get("fresh") is True,
    }


def _diagnostic_json(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False,
                      separators=(",", ":"), allow_nan=False).encode("utf-8")


def _diagnostic_text(value, allowance):
    if not isinstance(value, str):
        raise RuntimeError("Cargo diagnostic text has an unexpected type.")
    raw = value.encode("utf-8")
    retained = raw[:max(0, allowance)].decode("utf-8", errors="ignore")
    return retained, len(raw) - len(retained.encode("utf-8"))


def _diagnostic_path(value, paths):
    """Classify a lexical path without opening any compiler-provided filename."""
    if not isinstance(value, str) or "\x00" in value:
        raise RuntimeError("Cargo diagnostic path is absent or malformed.")
    if len(value.encode("utf-8")) > 4096:
        text, omitted = _diagnostic_text(value, 512)
        return {"file_name": text, "qualified_path": None,
                "path_scope": "overlong_path_omitted", "omitted_path_bytes": omitted}
    if value.startswith("<") and value.endswith(">"):
        return {"file_name": value, "qualified_path": None, "path_scope": "synthetic"}
    path = Path(value)
    if not value or ".." in path.parts:
        return {"file_name": value, "qualified_path": None,
                "path_scope": "unqualified_empty_or_parent_components"}
    absolute = path if path.is_absolute() else paths["cwd"] / path
    if len(str(absolute).encode("utf-8")) > 4096:
        return {"file_name": value, "qualified_path": None,
                "path_scope": "qualified_path_exceeds_allowance"}
    scope = "external"
    try:
        relative = absolute.relative_to(paths["target_directory"])
        scope = "target_output"
    except ValueError:
        try:
            relative = absolute.relative_to(paths["cwd"])
            scope = ("expected_package_source" if relative.parts
                     and relative.parts[0] in paths.get("expected_packages", ALLOWED_PACKAGES) else "other_worktree_source")
        except ValueError:
            pass
    return {"file_name": value, "qualified_path": str(absolute), "path_scope": scope,
            "qualification": "lexical_absolute" if path.is_absolute() else "lexical_observer_cwd",
            "file_bytes_verified": False}


def _diagnostic_target(record, paths):
    target = record.get("target")
    if not isinstance(target, dict):
        raise RuntimeError("Cargo compiler diagnostic lacks a target object.")
    name, omitted_name = _diagnostic_text(target.get("name", ""), 256)
    kinds = target.get("kind", [])
    if (not isinstance(kinds, list) or any(not isinstance(kind, str) for kind in kinds)):
        raise RuntimeError("Cargo compiler diagnostic target kind is malformed.")
    retained_kinds = [_diagnostic_text(kind, 64)[0] for kind in kinds[:8]]
    package, omitted_package = _diagnostic_text(record.get("package_id", ""), 2048)
    return {"name": name, "omitted_name_bytes": omitted_name,
            "kind": retained_kinds, "omitted_kinds": max(0, len(kinds) - 8),
            "src_path": _diagnostic_path(target.get("src_path", ""), paths),
            "package_id": package, "omitted_package_id_bytes": omitted_package}


def _priority_diagnostic(record, message, paths):
    level = message["level"]
    code = message.get("code")
    if code is not None:
        if not isinstance(code, dict) or not isinstance(code.get("code"), str):
            raise RuntimeError("Cargo compiler error code is malformed.")
        code, omitted_code = _diagnostic_text(code["code"], 128)
    else:
        omitted_code = 0
    text, omitted_message = _diagnostic_text(message.get("message", ""), 4096)
    spans = message.get("spans", [])
    if not isinstance(spans, list) or any(not isinstance(span, dict) for span in spans):
        raise RuntimeError("Cargo compiler diagnostic spans are malformed.")
    children = message.get("children", [])
    if not isinstance(children, list):
        raise RuntimeError("Cargo compiler diagnostic children are malformed.")
    result = {"level": level, "code": code, "message": text,
              "target": _diagnostic_target(record, paths), "spans": [], "rendered": "",
              "omitted_code_bytes": omitted_code, "omitted_message_bytes": omitted_message,
              "spans_observed": len(spans), "spans_retained": 0, "spans_omitted": len(spans),
              "children_observed": len(children), "children_retained_separately": False,
              "rendered_bytes_observed": 0, "rendered_bytes_retained": 0,
              "rendered_bytes_omitted": 0}
    # Retain primary locations before secondary locations, preserving order
    # within either group. No span opens or executes any named source.
    spans = sorted(spans, key=lambda span: span.get("is_primary") is not True)
    for span in spans:
        if len(result["spans"]) >= MAX_ERROR_SPANS:
            break
        location = _diagnostic_path(span.get("file_name", ""), paths)
        location["is_primary"] = span.get("is_primary") is True
        for field in ("byte_start", "byte_end", "line_start", "line_end", "column_start", "column_end"):
            value = span.get(field)
            if value is not None and (type(value) is not int or not 0 <= value <= 2 ** 63 - 1):
                raise RuntimeError("Cargo diagnostic span coordinate is malformed.")
            location[field] = value
        label = span.get("label")
        if label is not None:
            location["label"], location["omitted_label_bytes"] = _diagnostic_text(label, 512)
        candidate = dict(result, spans=result["spans"] + [location],
                         spans_retained=len(result["spans"]) + 1,
                         spans_omitted=len(spans) - len(result["spans"]) - 1)
        # Reserve a few bytes for subsequent decimal omission-count growth.
        if len(_diagnostic_json(candidate)) <= MAX_ERROR_RECORD_BYTES - 64:
            result = candidate
    rendered = message.get("rendered") or ""
    raw_rendered = rendered.encode("utf-8")
    result["rendered_bytes_observed"] = len(raw_rendered)
    # The exact encoded record, including JSON escaping and omission fields,
    # determines admission. Warning output never consumes this allowance.
    allowance = min(len(raw_rendered), MAX_ERROR_RECORD_BYTES)
    while True:
        snippet, omitted = _diagnostic_text(rendered, allowance)
        result.update(rendered=snippet, rendered_bytes_retained=len(snippet.encode("utf-8")),
                      rendered_bytes_omitted=omitted)
        excess = len(_diagnostic_json(result)) - MAX_ERROR_RECORD_BYTES
        if excess <= 0:
            return result
        if allowance == 0:
            return None
        allowance = max(0, allowance - max(excess, 64))


def _compiler_diagnostic(record, paths, evidence):
    message = record.get("message")
    if not isinstance(message, dict):
        raise RuntimeError("Cargo compiler message is not an object.")
    level = message.get("level")
    if level not in {"error", "warning", "note", "help", "failure-note", "ice"}:
        raise RuntimeError("Cargo compiler diagnostic level is absent or unknown.")
    rendered = message.get("rendered")
    if rendered is not None and not isinstance(rendered, str):
        raise RuntimeError("Cargo rendered diagnostic is not text.")
    evidence["diagnostic_records"] += 1
    evidence["diagnostic_levels"][level] = evidence["diagnostic_levels"].get(level, 0) + 1
    raw = (rendered or "").encode("utf-8")
    if level in {"error", "failure-note", "ice"}:
        evidence["priority_records_observed"] += 1
        candidate = _priority_diagnostic(record, message, paths)
        encoded = _diagnostic_json(candidate) if candidate is not None else b""
        reason = ("record_byte_allowance" if candidate is None else
                  "record_count_allowance" if evidence["priority_records_retained"] >= MAX_ERROR_RECORDS else
                  "total_byte_allowance" if evidence["priority_bytes_retained"] + len(encoded) > MAX_ERROR_BYTES else None)
        if reason is not None:
            evidence["priority_records_omitted"] += 1
            counts = evidence["priority_omission_reasons"]
            counts[reason] = counts.get(reason, 0) + 1
            snippet = b""
        else:
            evidence["priority_diagnostics"].append(candidate)
            evidence["priority_records_retained"] += 1
            evidence["priority_bytes_retained"] += len(encoded)
            snippet = candidate["rendered"].encode("utf-8")
            evidence["priority_rendered_bytes"] += len(snippet)
    else:
        allowance = max(0, MAX_WARNING_RENDERED_BYTES - evidence["warning_rendered_bytes"])
        snippet = _diagnostic_text(rendered or "", allowance)[0].encode("utf-8")
        evidence["warning_rendered_bytes"] += len(snippet)
    if snippet:
        sys.stderr.write(snippet.decode("utf-8"))
        sys.stderr.flush()
    evidence["rendered_bytes"] += len(snippet)
    evidence["rendered_bytes_omitted"] += len(raw) - len(snippet)


def _record(raw, expected, paths, evidence):
    if len(raw) > MAX_RECORD_BYTES:
        raise RuntimeError("Cargo machine record exceeds its per-record allowance.")
    if not raw.strip():
        return
    evidence["records"] += 1
    if evidence["records"] > MAX_RECORDS:
        raise RuntimeError("Cargo machine record count exceeds its allowance.")
    try:
        record = json.loads(raw.decode("utf-8"))
    except (ValueError, UnicodeError) as error:
        raise RuntimeError("Cargo stdout contains malformed machine JSON.") from error
    if not isinstance(record, dict):
        raise RuntimeError("Cargo machine record is not an object.")
    reason = record.get("reason")
    if reason == "compiler-artifact":
        _artifact(record, expected, paths, evidence)
    elif reason == "build-script-executed":
        evidence["build_script_records_discarded"] += 1
    elif reason == "compiler-message":
        _compiler_diagnostic(record, paths, evidence)
    elif reason == "build-finished":
        if evidence["build_finished"] is not None or type(record.get("success")) is not bool:
            raise RuntimeError("Cargo build-finished record is duplicate or malformed.")
        evidence["build_finished"] = record["success"]
    elif reason == "future-incompat-report":
        evidence["other_records_discarded"] += 1
    else:
        raise RuntimeError("Cargo emitted an unknown machine-record reason.")


def cargo_observer(spec):
    """Run only the fixed offline compilation command and retain small evidence.

    Return 0 only for a complete successful target inventory; return 101 for a
    valid Cargo101 diagnostic failure, preserving any valid partial artifacts.
    On malformed output or an output allowance violation, stop Cargo promptly.
    """
    evidence = {
        "schema": "bitcoinrs-final027-cargo-observer-v1", "command": None,
        "cwd": None, "target_directory": None, "expected_packages": [],
        "cargo_exit_code": None, "observer_exit_code": 2, "compile_outcome": None, "error": None,
        "artifacts": {}, "machine_bytes": 0, "records": 0,
        "dependency_artifacts_discarded": 0, "build_script_records_discarded": 0,
        "build_script_artifacts_discarded": 0,
        "diagnostic_records": 0, "other_records_discarded": 0,
        "rendered_bytes": 0, "rendered_bytes_omitted": 0,
        "diagnostic_levels": {}, "priority_diagnostics": [],
        "priority_records_observed": 0, "priority_records_retained": 0,
        "priority_records_omitted": 0, "priority_bytes_retained": 0,
        "priority_omission_reasons": {}, "warning_rendered_bytes": 0,
        "priority_rendered_bytes": 0,
        "build_finished": None,
    }
    child, paths = None, None
    try:
        command, expected, paths = _observer_spec(spec)
        evidence.update(command=command, cwd=str(paths["cwd"]),
                        target_directory=str(paths["target_directory"]), expected_packages=expected)
        env = dict(os.environ)
        env.update(CARGO_TARGET_DIR=str(paths["target_directory"]), CARGO_BUILD_JOBS="1", CARGO_INCREMENTAL="0")
        child = subprocess.Popen(command, cwd=paths["cwd"], env=env,
                                 stdin=subprocess.DEVNULL, stdout=subprocess.PIPE)
        os.set_blocking(child.stdout.fileno(), False)
        buffer = bytearray()
        deadline = time.monotonic() + OBSERVER_SECONDS
        with selectors.DefaultSelector() as selector:
            selector.register(child.stdout, selectors.EVENT_READ)
            while selector.get_map():
                if time.monotonic() >= deadline:
                    raise RuntimeError("Cargo observer reached its wall-clock allowance.")
                for key, _ in selector.select(0.1):
                    try:
                        chunk = os.read(key.fileobj.fileno(), 65536)
                    except BlockingIOError:
                        continue
                    if not chunk:
                        selector.unregister(key.fileobj)
                        if buffer:
                            _record(bytes(buffer), expected, paths, evidence)
                            buffer.clear()
                        continue
                    evidence["machine_bytes"] += len(chunk)
                    if evidence["machine_bytes"] > MAX_MACHINE_BYTES:
                        raise RuntimeError("Cargo machine stdout exceeds its total-byte allowance.")
                    buffer.extend(chunk)
                    while b"\n" in buffer:
                        newline = buffer.index(b"\n")
                        _record(bytes(buffer[:newline]), expected, paths, evidence)
                        del buffer[:newline + 1]
                    if len(buffer) > MAX_RECORD_BYTES:
                        raise RuntimeError("Cargo machine record exceeds its per-record allowance.")
        remaining = max(0.1, deadline - time.monotonic())
        evidence["cargo_exit_code"] = child.wait(timeout=remaining)
        if evidence["cargo_exit_code"] == 101:
            # A fully consumed, valid Cargo stream may end before build-finished
            # when Cargo itself fails. Keep its diagnostic failure distinct from
            # observer parsing, framing, target or resource-evidence failures.
            if evidence["build_finished"] is True:
                raise RuntimeError("Cargo reported a successful build despite its failed exit101.")
            evidence.update(observer_exit_code=101, compile_outcome="failed")
        elif evidence["cargo_exit_code"] != 0:
            raise RuntimeError("Cargo exited with an unreviewed unsuccessful status.")
        else:
            if evidence["build_finished"] is not True:
                raise RuntimeError("Cargo did not report a successful completed build.")
            # rustc may label explanatory output (for example --explain E0133
            # following an unsafe-op warning) as failure-note even on success.
            # Keep that output prioritized, but use error/ICE plus Cargo's actual
            # process/build-finished result to decide whether compilation passed.
            if (evidence["diagnostic_levels"].get("error", 0)
                    or evidence["diagnostic_levels"].get("ice", 0)):
                raise RuntimeError("Cargo reported error diagnostics despite a successful process/build result.")
            if set(evidence["artifacts"]) != set(paths["expected_artifacts"]):
                raise RuntimeError("Cargo did not emit the exact complete executable inventory for its selected command.")
            evidence.update(observer_exit_code=0, compile_outcome="passed")
    except Exception as error:
        evidence["error"] = (type(error).__name__ + ": " + str(error))[:2048]
        if child is not None:
            try:
                _stop_cargo(child)
            except Exception as stop_error:
                evidence["cleanup_error"] = str(stop_error)[:512]
            evidence["cargo_exit_code"] = child.poll()
    finally:
        if child is not None and child.stdout is not None:
            child.stdout.close()
    if paths is None:
        sys.stderr.write("FINAL027_CARGO_OBSERVER_REJECTED " + (evidence["error"] or "invalid specification") + "\n")
        return 2
    try:
        _atomic_evidence(paths["evidence_path"], evidence)
    except Exception as error:
        sys.stderr.write("FINAL027_CARGO_EVIDENCE_ERROR " + str(error)[:1024] + "\n")
        return 3
    print("FINAL027_CARGO_OBSERVER records=" + str(evidence["records"])
          + " executables=" + str(len(evidence["artifacts"]))
          + " native_exit=" + str(evidence["cargo_exit_code"])
          + " observer_exit=" + str(evidence["observer_exit_code"]), flush=True)
    return evidence["observer_exit_code"]


"""Exact in-memory source transform for the two final-crate compile bindings.

This module has no file writes or subprocess entry points. It changes only two
library roots; manifests, harness bodies, test bodies and runtime work remain
outside the payload.
"""
import hashlib


FACADE_PATH = 'bitcoin-leveldb/src/lib.rs'
DBTEST_PATH = 'bitcoinleveldbt-dbtest/src/lib.rs'
ORIGINAL_SOURCE_HASHES = {
    FACADE_PATH: '575c1becad78140698601b0c5c2d8e432dc63a3a8f929018430e45912247bdcc',
    DBTEST_PATH: '17901b8e2ac83d8b0e5d46419e0fe6f69f1e38910d483a9775df0141462fd8b1',
}
REMOVED_REEXPORTS = (
    b'pub use bitcoinleveldbt_bench::*;\n',
    b'pub use bitcoinleveldb_meta::*;\n',
)
SHARED_TYPE_REEXPORT = b'pub use bitcoinleveldbt_util::{TestArgs, TestType};\n'
DBTEST_ROOT_ANCHOR = b'#[macro_use] mod imports; use imports::*;\n'


def _replace_one(raw, old, new):
    if raw.count(old) != 1:
        raise RuntimeError('Exact compile-binding source anchor is absent or repeated.')
    return raw.replace(old, new, 1)


def build_payload(originals):
    """Return exactly two hash-bound replacement files, without performing I/O."""
    if not isinstance(originals, dict) or set(originals) != set(ORIGINAL_SOURCE_HASHES):
        raise RuntimeError('Exactly the two captured final-crate library roots are required.')
    for name, expected_sha256 in ORIGINAL_SOURCE_HASHES.items():
        raw = originals[name]
        if not isinstance(raw, bytes) or hashlib.sha256(raw).hexdigest() != expected_sha256:
            raise RuntimeError('Compile-binding source differs from the successful028 capture: ' + name)
    facade = originals[FACADE_PATH]
    for statement in REMOVED_REEXPORTS:
        facade = _replace_one(facade, statement, b'')
    dbtest = _replace_one(originals[DBTEST_PATH], DBTEST_ROOT_ANCHOR,
                         DBTEST_ROOT_ANCHOR + b'\n' + SHARED_TYPE_REEXPORT)
    return {FACADE_PATH: facade, DBTEST_PATH: dbtest}


def source_review_notes030():
    """Small compile-only review statement for the embedding report/document."""
    return {
        'facade': 'Remove two unresolved facade exports: the benchmark crate is not a declared facade dependency, and the metadata crate is absent from the captured checkout. All other declared facade exports remain in their original order.',
        'shared_test_types': 'Add explicit public reexports of TestArgs and TestType from bitcoinleveldbt-util through bitcoinleveldbt-dbtest. That existing direct dependency owns the types; reexporting preserves their identity and exposes the API already named by harness library and integration-test imports.',
        'preserved': 'Every manifest, lockfile, harness implementation, test body, workload size, oracle, execution policy and runtime behavior is unchanged by this two-file source payload.',
        'validation_scope': 'Source bindings and import/dependency structure are checked in Python fixtures. Fresh guarded no-run Cargo compilation is still required; native tests and full coverage are not claimed.',
    }


"""Read-only qualification of fresh compile030 no-run evidence and artifacts."""
import os
from pathlib import Path
import re
import shlex
from urllib.parse import unquote, urlsplit


_Q030_WORKFLOW = 'leveldb-facade-harness-compile030'
_Q030_SUFFIX = '-leveldb-facade-harness-compile030'
_Q030_SCHEMA = 'bitcoinrs-final027-cargo-observer-v1'
_Q030_COMMANDS = {
    'facade': ('cargo', 'test', '--offline', '--locked', '--jobs', '1', '-p', 'bitcoin-leveldb',
               '--lib', '--tests', '--no-run', '--message-format=json'),
    'harness': ('cargo', 'test', '--offline', '--locked', '--jobs', '1', '-p', 'bitcoinleveldb-harness',
                '--lib', '--tests', '--no-run', '--message-format=json')}
_Q030_TARGETS = {
    'facade': {'bitcoin_leveldb': ('lib', 'src/lib.rs')},
    'harness': {'bitcoinleveldb_harness': ('lib', 'src/lib.rs'), 'harness': ('test', 'tests/harness.rs')}}


def _q030_path(value, description):
    if (not isinstance(value, (str, Path)) or not str(value) or len(str(value)) > 4096
            or '\x00' in str(value)):
        raise RuntimeError('Compile030 ' + description + ' is not a bounded literal path.')
    path = Path(value)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Compile030 ' + description + ' is not an absolute ordinary path.')
    return path


def _q030_context(core, label, spec, target, guard_command, run_directory):
    if label not in _Q030_COMMANDS or not isinstance(spec, dict):
        raise RuntimeError('Compile030 requires one of its two reviewed package specifications.')
    run = _q030_path(run_directory, 'run directory')
    target = _q030_path(target, 'target directory')
    own = run.parent.parent
    if (not re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', run.name)
            or run.parent.name != 'runs' or own.name != _Q030_WORKFLOW
            or own.parent.name != 'codex-repairs' or own.parent.parent.name != '.git'
            or target != own / 'cargo-target'):
        raise RuntimeError('Compile030 evidence or cache is outside its independent fresh workflow.')
    repo = own.parent.parent.parent
    worktree = repo.parent / (repo.name + _Q030_SUFFIX)
    expected = {'command': list(_Q030_COMMANDS[label]), 'cwd': str(worktree),
                'target_directory': str(target), 'evidence_path': str(run / ('cargo-' + label + '.json'))}
    if spec != expected:
        raise RuntimeError('Compile030 observer specification differs from its exact fresh no-run plan.')
    for path in (run, worktree, target):
        descriptor = core.open_dir(path)
        os.close(descriptor)
    if (not isinstance(guard_command, list) or len(guard_command) != 13
            or any(not isinstance(value, str) or '\x00' in value or len(value) > 65536
                   for value in guard_command)
            or not Path(guard_command[0]).is_absolute() or Path(guard_command[0]).name != 'nix-shell'
            or guard_command[1:12] != ['--option', 'max-jobs', '0', '--option', 'builders', '',
                                       '--option', 'substitute', 'false', str(worktree / 'shell.nix'), '--run']):
        raise RuntimeError('Compile030 outer guard command differs from its offline Nix compilation plan.')
    try:
        inner = shlex.split(guard_command[-1])
    except ValueError as error:
        raise RuntimeError('Compile030 observer shell argument is malformed.') from error
    if (len(inner) != 7 or not Path(inner[0]).is_absolute() or Path(inner[0]).name != 'python3'
            or inner[1:] != ['-I', '-S', str(run / 'compilation-controller030.py'), '--cargo-observer',
                             str(repo), str(run / ('cargo-' + label + '.spec.json'))]):
        raise RuntimeError('Compile030 outer command is not bound to this fresh observer/script specification.')
    return repo, worktree, target


def _q030_package_id(value, package_root):
    if not isinstance(value, str) or not value or len(value) > 4096 or '\x00' in value:
        raise RuntimeError('Compile030 artifact package identity is absent or malformed.')
    if value.startswith('path+'):
        parsed = urlsplit(value[5:])
        path = Path(unquote(parsed.path))
        if (parsed.scheme != 'file' or parsed.netloc not in ('', 'localhost')
                or not path.is_absolute() or '..' in path.parts or path != package_root):
            raise RuntimeError('Compile030 artifact path package identity differs from its exact source root.')
    elif '://' in value:
        raise RuntimeError('Compile030 local artifact has an external package identity.')


def _q030_fingerprint(value, maximum):
    return (isinstance(value, dict) and value.get('kind') == 'file'
            and type(value.get('mode')) is int and 0 <= value['mode'] <= 0o777
            and value['mode'] & 0o111 != 0
            and type(value.get('size')) is int and 0 < value['size'] <= maximum
            and isinstance(value.get('sha256'), str)
            and re.fullmatch(r'[0-9a-f]{64}', value['sha256']) is not None)


def qualify_compile030(core, label, row, evidence, spec, target, *, guard_command, run_directory):
    """Require one freshly invoked030 build and bind its complete executable inventory.

    This routine only reads files. It never executes an artifact or reuses a
    preceding baseline failure as qualification. The caller invokes it for both
    labels after each newly executed guarded compilation.
    """
    _, worktree, target = _q030_context(core, label, spec, target, guard_command, run_directory)
    budget = core.load_guard().BUDGETS['build']
    if (not isinstance(row, dict) or row.get('verified') is not True or row.get('outcome') != 'exited'
            or type(row.get('exit_code')) is not int or row['exit_code'] != 0
            or row.get('kind') != 'build' or row.get('budget') != budget
            or any(type(value) is not int for value in row.get('budget', {}).values())
            or row.get('command') != guard_command or row.get('cwd') != str(worktree)):
        raise RuntimeError('Compile030 lacks its exact verified zero-exit finite guard command/cwd/budget.')
    core.require_guard(row)
    package = _Q030_COMMANDS[label][7]
    expected_keys = {package + '::' + name for name in _Q030_TARGETS[label]}
    if (not isinstance(evidence, dict) or evidence.get('schema') != _Q030_SCHEMA
            or evidence.get('command') != spec['command'] or evidence.get('cwd') != spec['cwd']
            or evidence.get('target_directory') != spec['target_directory']
            or evidence.get('expected_packages') != [package]
            or type(evidence.get('cargo_exit_code')) is not int or evidence['cargo_exit_code'] != 0
            or type(evidence.get('observer_exit_code')) is not int or evidence['observer_exit_code'] != 0
            or evidence.get('build_finished') is not True or evidence.get('compile_outcome') != 'passed'
            or 'error' not in evidence or evidence['error'] is not None
            or not isinstance(evidence.get('artifacts'), dict) or set(evidence['artifacts']) != expected_keys):
        raise RuntimeError('Compile030 requires exact successful observer evidence and every selected artifact.')
    levels = evidence.get('diagnostic_levels')
    if (not isinstance(levels, dict) or not set(levels).issubset({'error', 'warning', 'note', 'help', 'failure-note', 'ice'})
            or any(type(value) is not int or not 0 <= value <= 50000 for value in levels.values())
            or any(levels.get(level, 0) for level in ('error', 'ice'))):
        raise RuntimeError('Compile030 successful evidence contains malformed or failing compiler diagnostics.')
    priority = {name: evidence.get(name) for name in
                ('priority_records_observed', 'priority_records_retained', 'priority_records_omitted')}
    if (any(type(value) is not int or not 0 <= value <= 50000 for value in priority.values())
            or priority['priority_records_observed'] != priority['priority_records_retained'] + priority['priority_records_omitted']
            or priority['priority_records_observed'] != sum(levels.get(level, 0) for level in ('error', 'failure-note', 'ice'))
            or priority['priority_records_omitted'] != 0):
        raise RuntimeError('Compile030 priority diagnostic retention is incomplete or malformed.')
    artifacts, paths = {}, set()
    maximum = 256 * core.MIB
    for name, (kind, relative_source) in _Q030_TARGETS[label].items():
        key = package + '::' + name
        record = evidence['artifacts'][key]
        if not isinstance(record, dict):
            raise RuntimeError('Compile030 artifact record is not an object.')
        path = _q030_path(record.get('executable'), 'artifact executable')
        if (path.parent != target / 'debug/deps' or path in paths
                or re.fullmatch(re.escape(name) + r'-[0-9a-f]+', path.name) is None
                or record.get('package') != package or record.get('target_name') != name
                or record.get('kind') != [kind]
                or record.get('src_path') != str(worktree / package / relative_source)):
            raise RuntimeError('Compile030 artifact package/target/source/private executable binding differs.')
        _q030_package_id(record.get('package_id'), worktree / package)
        fingerprint, _ = core.read_file(path, maximum, content=False)
        if not _q030_fingerprint(fingerprint, maximum):
            raise RuntimeError('Compile030 artifact lacks ordinary executable mode/size/SHA256 evidence.')
        paths.add(path)
        artifacts[key] = {'path': str(path), 'fingerprint': fingerprint}
    for item in artifacts.values():
        if core.read_file(Path(item['path']), maximum, content=False)[0] != item['fingerprint']:
            raise RuntimeError('Compile030 artifact changed during its read-only qualification.')
    return {'schema': 'bitcoinrs-compile030-qualification-v1', 'outcome': 'compiled',
            'cargo_exit_code': 0, 'observer_exit_code': 0,
            'artifacts': artifacts, 'expected_artifacts': sorted(expected_keys),
            'native_cases_executed': 0, 'validation_reused': False,
            'diagnostic_levels': dict(levels), **priority, 'repair_qualified': False,
            'scope': 'fresh successful offline no-run compilation only; no native-case qualification'}


"""Finite compile-only candidate workers; prior028 state is read and preserved."""

def load030(name, path, size, sha):
    raw = bootstrap_read(path, 2 * MIB027)
    if len(raw) != size or hashlib.sha256(raw).hexdigest() != sha:
        raise RuntimeError('Reviewed retained runtime differs: ' + path.name)
    module = types.ModuleType(name); module.__file__ = str(path); sys.modules[name] = module
    exec(compile(raw, str(path), 'exec'), module.__dict__)
    return module


def runtime030(repo):
    retained = repo / '.git/codex-repairs'
    baseline = load030('compile030_baseline028',
        retained / 'leveldb-facade-harness-baseline028/runs/20261108T185235Z-7ccb8aac/baseline-controller028.py',
        143039, '874a136dca616dbe1c930cf89678cd5c261beab2216c5f659b609769aa1bf3ae')
    previous = load030('compile030_runtime025',
        retained / 'leveldb-randomized-quiet025/runs/20261108T170855Z-4f20bcdb/randomized-controller025.py',
        231817, '7f045e8759bdd6021ed0b3c4ff340a29d625f61b1df611a2b3564ffe4fed4f83')
    finalizer, core, helper, _ = previous.runtime023(repo)
    configure_git_helper023(helper)
    core.WORKFLOW, core.BRANCH, core.WORKTREE_SUFFIX = WORKFLOW, BRANCH, SUFFIX
    return baseline, previous, finalizer, core, helper


def baseline030(baseline, core, repo):
    """Bind the exact uploaded compiler failures; no failed validation is reused."""
    base = baseline.promoted_baseline027(core, repo)
    run = repo / '.git/codex-repairs/leveldb-facade-harness-baseline028/runs/20261108T185235Z-7ccb8aac'
    path = run / 'BASELINE_RETURN.json'; fp, raw = core.read_file(path, 2 * MIB027)
    if fp.get('size') != 999730 or fp.get('sha256') != '205d72d83e041235db8651e129cf48295a8ffd24945e0a0355e0cf699600dc8a':
        raise RuntimeError('Uploaded completed028 compilation report differs.')
    prior = json.loads(raw)
    expected = {'workflow': 'leveldb-facade-harness-baseline028', 'controller_revision': 2,
        'outcome': 'diagnostic_complete', 'phase': 'complete', 'diagnostic_only': True,
        'repair_qualified': False, 'source_edits_started': False, 'master_updated': False,
        'existing_refs_updated': False, 'builds_started': True, 'crate_builds_started': True,
        'native_tests_started': False, 'debuggers_started': False, 'test_count': 0,
        'original_preserved': True, 'base': BASE, 'base_tree': BASE_TREE,
        'repository': str(repo), 'run_directory': str(run), 'return_file': str(path)}
    if (any(type(prior.get(k)) is not type(v) or prior.get(k) != v for k, v in expected.items())
            or prior.get('baseline_before') != prior.get('baseline_after')
            or prior.get('preservation_before_sha256') != prior.get('preservation_after_sha256')
            or prior.get('checkout_before') != prior.get('checkout_after')
            or baseline.checkout_binding027(core, repo) != prior.get('checkout_after')):
        raise RuntimeError('Retained028 compilation/preservation identity differs.')
    capture = prior['source_capture']
    proof = baseline.verify_sources027(core, repo, capture)
    if (capture.get('input_count') != 124 or capture.get('source_count') != 58
            or capture.get('source_text_complete') is not True or capture.get('source_omissions') != []
            or proof.get('inputs_preserved') is not True or proof != prior.get('source_verification_final')):
        raise RuntimeError('Current selected source inputs differ from the reviewed028 capture.')
    evidence_bindings = {}
    for label, errors in (('facade', 2), ('harness', 10)):
        summary = prior['compilations'][label]; observed = prior['cargo_evidence'][label]
        evidence_path = run / ('cargo-' + label + '.json')
        evidence_fp, evidence_raw = core.read_file(evidence_path, MIB027)
        row = prior['readouts']['compile-' + label]
        if (summary.get('outcome') != 'compiler_failure_observed'
                or summary.get('cargo_exit_code') != 101 or summary.get('observer_exit_code') != 101
                or summary.get('artifacts') != {} or summary.get('native_cases_executed') != 0
                or evidence_fp != summary.get('evidence_fingerprint') or json.loads(evidence_raw) != observed
                or observed.get('command') != COMMANDS027[label] or observed.get('error') is not None
                or observed.get('diagnostic_levels', {}).get('error') != errors
                or observed.get('priority_records_omitted') != 0
                or row.get('verified') is not True or row.get('outcome') != 'exited'
                or type(row.get('exit_code')) is not int or row['exit_code'] != 101
                or row.get('budget') != core.load_guard().BUDGETS['build']):
            raise RuntimeError('Retained028 exact compiler failure evidence differs: ' + label)
        evidence_bindings[label] = {'path': str(evidence_path), 'fingerprint': evidence_fp,
                                  'errors': errors, 'qualification_reused': False}
    state_path = run.parent.parent / 'state.json'; state_fp, state_raw = core.read_file(state_path, MIB027)
    state = json.loads(state_raw)
    if (state.get('workflow') != 'leveldb-facade-harness-baseline028' or state.get('phase') != 'complete'
            or state.get('report') != str(path) or state.get('report_fingerprint') != fp):
        raise RuntimeError('Retained028 completed journal no longer binds the uploaded report.')
    return {'base': base, 'report': str(path), 'report_fingerprint': fp,
        'source_verification': proof, 'retained_journal': str(state_path), 'retained_journal_fingerprint': state_fp,
        'compiler_failures': evidence_bindings, 'source_capture': capture,
        'classification': 'read_only_prior_failure_binding_not_reused_compilation_qualification'}


def files030(raw):
    if not isinstance(raw, bytes) or len(raw) > 2 * MIB027:
        raise RuntimeError('Compile repair controller bytes exceed their finite allowance.')
    files = dict(SOURCE_PAYLOADS)
    files.update({REPLAY_PATH: raw, DOCUMENT_PATH: DOCUMENT_BYTES})
    if set(files) != set(INITIAL_PATHS): raise RuntimeError('Compile repair file plan differs.')
    return files


def owned030(core, repo, path, names):
    path = Path(path)
    if (not path.is_absolute() or '..' in path.parts or path.name not in names
            or path.parent.parent != repo / '.git/codex-repairs' / WORKFLOW / 'runs'
            or re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', path.parent.name) is None):
        raise RuntimeError('Compile repair worker path differs from its finite owned output scope.')
    fd = core.open_dir(path.parent); os.close(fd)


def inputs_worker030(repo, output):
    baseline, _, _, core, _ = runtime030(repo)
    owned030(core, repo, output, {'inputs-before.json', 'inputs-after.json'})
    core.exclusive(output, core.encoded(baseline030(baseline, core, repo)))
    print('COMPILE030_SOURCE_AND_PRIOR_EVIDENCE_REVERIFIED', flush=True)
    return 0


def identity_worker030(repo, spec_path, output):
    _, _, finalizer, core, _ = runtime030(repo)
    owned030(core, repo, spec_path, {'identity-input.json'}); owned030(core, repo, output, {'identity.json'})
    if spec_path.parent != output.parent: raise RuntimeError('Identity paths belong to different compile repair runs.')
    spec = json.loads(core.read_file(spec_path, MIB027)[1]); inv = candidate_original_guard023(core, repo)
    env = dict(inv.env)
    for key in finalizer.IDENTITY_ENV: env.pop(key, None)
    if not isinstance(spec, dict): raise RuntimeError('Configured identity input is malformed.')
    for key, value in spec.items():
        if key not in finalizer.IDENTITY_ENV or not isinstance(value, str) or '\0' in value:
            raise RuntimeError('Configured identity override differs from the finite plan.')
        env[key] = value
    result = {}
    for role in ('AUTHOR', 'COMMITTER'):
        p = subprocess.run(inv.prefix + ['-c', 'user.useConfigOnly=true', 'var', 'GIT_' + role + '_IDENT'],
            cwd=repo, env=env, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10, check=False)
        if p.returncode or len(p.stdout) > 4096 or len(p.stderr) > 8192:
            raise RuntimeError('Configured Git identity is unavailable; no identity was guessed.')
        result[role.lower()] = finalizer.parse_ident(p.stdout.decode().rstrip('\n'))
    core.exclusive(output, core.encoded(result)); print('COMPILE030_CONFIGURED_IDENTITIES_RECORDED', flush=True)
    return 0


def install_worker030(repo):
    baseline, _, _, core, helper = runtime030(repo); intake = baseline030(baseline, core, repo)
    worktree = repo.parent / (repo.name + SUFFIX)
    raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB027); files = files030(raw)
    check_candidate023(core, repo, worktree, files, BASE, allow_base_files=True)
    for name, expected_sha in ORIGINAL_SOURCE_HASHES.items():
        fp, source = core.read_file(worktree / name, MIB027)
        captured = intake['source_capture']['files'][name]
        if (fp['sha256'] != expected_sha or captured.get('committed_sha256') != expected_sha
                or captured.get('text', '').encode() != source):
            raise RuntimeError('Compile replacement source differs from its captured committed input: ' + name)
    for name, content in sorted(files.items()):
        destination = worktree / name; destination.parent.mkdir(parents=True, exist_ok=True)
        fd = core.open_dir(destination.parent); os.close(fd)
        mode = 0o755 if name == REPLAY_PATH else 0o644
        if name in ORIGINAL_SOURCE_HASHES:
            temporary = destination.with_name(destination.name + '.compile030-' + uuid.uuid4().hex)
            helper.candidate_exclusive015(core, temporary, content, mode); os.replace(temporary, destination)
        else:
            if os.path.lexists(destination): raise RuntimeError('New compile repair path is occupied: ' + name)
            helper.candidate_exclusive015(core, destination, content, mode)
    print('COMPILE030_EXACT_SOURCE_INSTALLED', flush=True)
    return 0


def seed_worker030(repo, source, target, output):
    _, _, _, core, helper = runtime030(repo); owned030(core, repo, output, {'cache-copy.json'})
    if (source != repo / '.git/codex-repairs/leveldb-facade-harness-baseline028/cargo-target'
            or target != repo / '.git/codex-repairs' / WORKFLOW / 'cargo-target'):
        raise RuntimeError('Cache seed differs from the independent030 copy plan.')
    raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB027)
    check_candidate023(core, repo, repo.parent / (repo.name + SUFFIX), files030(raw), BASE)
    return helper.copy_cache_worker_revision002(core, source, target, output)


def observer_worker030(repo, spec_path):
    _, _, _, core, _ = runtime030(repo)
    owned030(core, repo, spec_path, {'cargo-' + name + '.spec.json' for name in COMMANDS027})
    worktree = repo.parent / (repo.name + SUFFIX)
    raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB027)
    check_candidate023(core, repo, worktree, files030(raw), BASE)
    label = spec_path.name[len('cargo-'):-len('.spec.json')]
    spec = json.loads(core.read_file(spec_path, MIB027)[1])
    expected = {'command': COMMANDS027[label], 'cwd': str(worktree),
        'target_directory': str(repo / '.git/codex-repairs' / WORKFLOW / 'cargo-target'),
        'evidence_path': str(spec_path.parent / ('cargo-' + label + '.json'))}
    if spec != expected: raise RuntimeError('Fresh compile030 Cargo observer specification differs.')
    return cargo_observer(spec)


def git_worker030(repo, spec):
    _, _, _, core, helper = runtime030(repo)
    previous = os.umask(0o022)
    try: return helper.run_git_operation(core, repo, spec)
    finally: os.umask(previous)


def artifact_check030(core, compilations):
    for label in ('facade', 'harness'):
        for item in compilations[label]['artifacts'].values():
            if core.read_file(Path(item['path']), 256 * MIB027, content=False)[0] != item['fingerprint']:
                raise RuntimeError('Fresh compile030 artifact changed before publication.')


"""Post-exit transport for the facade/harness compile repair030.

Only the separately authorized original return-buffer is replaced after the
coordinator has exited and printed/saved return JSON identities agree.
"""
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import stat
import subprocess
import sys
import threading
import uuid


_TRANSPORT_MAX_BYTES = 8 * 1024 * 1024


_TRANSPORT_WORKFLOW = "leveldb-facade-harness-compile030"


_TRANSPORT_BEGIN = b"BEGIN_FACADE_HARNESS_COMPILE_RETURN"


_TRANSPORT_END = b"END_FACADE_HARNESS_COMPILE_RETURN"

_TRANSPORT_ARTIFACTS030 = {
    "facade": {"bitcoin-leveldb::bitcoin_leveldb"},
    "harness": {"bitcoinleveldb-harness::bitcoinleveldb_harness", "bitcoinleveldb-harness::harness"},
}
_TRANSPORT_PATHS030 = {
    "bitcoin-leveldb/src/lib.rs", "bitcoinleveldbt-dbtest/src/lib.rs",
    "contrib/repair/bitcoinleveldb-facade-harness-compile030.py",
    "doc/repair/leveldb-facade-harness-compile030.json", "doc/testing/leveldb-facade-harness-compile030.md",
}


def _transport_compilations030(report, repo):
    compilations = report.get("compilations")
    if not isinstance(compilations, dict) or set(compilations) != set(_TRANSPORT_ARTIFACTS030):
        raise RuntimeError("Compile-repair result lacks both separate crate compilations.")
    successes = 0
    target = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW / "cargo-target"
    if report.get("target_directory") != str(target):
        raise RuntimeError("Compile-repair artifacts do not identify the owned private cache.")
    for label, expected in _TRANSPORT_ARTIFACTS030.items():
        value = compilations[label]
        if not isinstance(value, dict): raise RuntimeError("Crate compilation summary is malformed.")
        code = value.get("cargo_exit_code")
        levels = value.get("diagnostic_levels")
        artifacts = value.get("artifacts")
        if (type(code) is not int or code not in {0, 101}
                or type(value.get("observer_exit_code")) is not int or value["observer_exit_code"] != code
                or value.get("expected_artifacts") != sorted(expected)
                or type(value.get("native_cases_executed")) is not int or value["native_cases_executed"] != 0
                or value.get("repair_qualified") is not False
                or type(value.get("priority_records_omitted")) is not int or value["priority_records_omitted"] != 0
                or not isinstance(levels, dict) or any(type(count) is not int or count < 0 for count in levels.values())
                or not isinstance(artifacts, dict) or not set(artifacts).issubset(expected)):
            raise RuntimeError("Crate compilation status, diagnostics or expected artifact inventory differs.")
        if code == 0:
            if (value.get("outcome") != "compiled" or set(artifacts) != expected
                    or any(levels.get(level, 0) != 0 for level in ("error", "ice"))):
                raise RuntimeError("Qualified compile repair lacks a complete successful crate artifact inventory.")
            successes += 1
        elif value.get("outcome") != "compiler_failure_observed":
            raise RuntimeError("Failed compiler status is not classified as compiler_failure_observed.")
        for key, artifact in artifacts.items():
            name = key.split("::", 1)[1]
            if not isinstance(artifact, dict) or set(artifact) != {"path", "fingerprint"}:
                raise RuntimeError("Compile artifact entry is malformed.")
            path = Path(artifact.get("path", "")); fp = artifact.get("fingerprint")
            if (path.parent != target / "debug/deps" or not re.fullmatch(re.escape(name) + r"-[0-9a-f]+", path.name)
                    or not isinstance(fp, dict) or set(fp) != {"kind", "mode", "size", "sha256"}
                    or fp.get("kind") != "file" or type(fp.get("mode")) is not int
                    or not 0 <= fp["mode"] <= 0o777 or not fp["mode"] & 0o111
                    or type(fp.get("size")) is not int or not 0 < fp["size"] <= 256 * 1024 * 1024
                    or not isinstance(fp.get("sha256"), str) or not re.fullmatch(r"[0-9a-f]{64}", fp["sha256"])):
                raise RuntimeError("Compile artifact path or ordinary executable fingerprint differs.")
    return successes


class _TransportReplacementError(RuntimeError):
    buffer_replaced = True


def _transport_open_directory(path):
    path = Path(path)
    if not path.is_absolute() or ".." in path.parts:
        raise RuntimeError("Transport requires an absolute ordinary directory.")
    fd = os.open("/", os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd)
            fd = child
        return fd
    except BaseException:
        os.close(fd)
        raise


def _transport_stamp(info):
    return tuple(getattr(info, key) for key in
                 ("st_dev", "st_ino", "st_mode", "st_uid", "st_size", "st_mtime_ns", "st_ctime_ns"))


def _transport_read_at(directory_fd, name, limit, own=False):
    fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=directory_fd)
    try:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
            raise RuntimeError("Transport input is not a bounded ordinary file: " + name)
        if own and before.st_uid != os.getuid():
            raise RuntimeError("Transport return-buffer is not owned by the current user.")
        pieces, total, digest = [], 0, hashlib.sha256()
        while True:
            chunk = os.read(fd, min(65536, limit + 1 - total))
            if not chunk:
                break
            total += len(chunk)
            if total > limit:
                raise RuntimeError("Transport input grew beyond its byte allowance.")
            pieces.append(chunk)
            digest.update(chunk)
        after = os.fstat(fd)
        current = os.stat(name, dir_fd=directory_fd, follow_symlinks=False)
        if (total != before.st_size or _transport_stamp(before) != _transport_stamp(after)
                or _transport_stamp(before) != _transport_stamp(current)):
            raise RuntimeError("Transport input changed during its ordinary-file read.")
        fp = {"kind": "file", "mode": stat.S_IMODE(before.st_mode), "size": total,
              "sha256": digest.hexdigest()}
        return b"".join(pieces), fp, _transport_stamp(before)
    finally:
        os.close(fd)


def _transport_read_file(path, limit):
    path = Path(path)
    parent = _transport_open_directory(path.parent)
    try:
        return _transport_read_at(parent, path.name, limit)
    finally:
        os.close(parent)


def _transport_emit(raw):
    view = memoryview(raw)
    while view:
        try:
            written = os.write(1, view)
        except InterruptedError:
            continue
        if written <= 0:
            raise RuntimeError("Terminal output did not accept captured bytes.")
        view = view[written:]


def _transport_unique_pairs(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise RuntimeError("Compile-repair transport JSON contains duplicate fields.")
        result[key] = value
    return result


def _transport_report(capture, repo, child_status):
    lines = capture.splitlines()
    begins = [index for index, line in enumerate(lines) if line == _TRANSPORT_BEGIN]
    ends = [index for index, line in enumerate(lines) if line == _TRANSPORT_END]
    if len(begins) != 1 or len(ends) != 1 or begins[0] >= ends[0]:
        raise RuntimeError("Completed output lacks one unambiguous facade/harness compile repair return JSON block.")
    report = json.loads(b"\n".join(lines[begins[0] + 1:ends[0]]).decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if (not isinstance(report, dict) or report.get("workflow") != _TRANSPORT_WORKFLOW
            or report.get("repository") != str(repo) or not isinstance(report.get("outcome"), str)):
        raise RuntimeError("Printed facade/harness compile repair return does not identify this coordinator/repository.")
    value = report.get("return_file")
    if not isinstance(value, str) or not value or "\x00" in value:
        raise RuntimeError("Facade/harness compile repair return-file path is absent or malformed.")
    path = Path(value)
    root = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW / "runs"
    if not path.is_absolute() or ".." in path.parts:
        raise RuntimeError("Facade/harness compile repair return-file path is not ordinary and absolute.")
    try:
        relative = path.relative_to(root)
    except ValueError as error:
        raise RuntimeError("Facade/harness compile repair return file is outside the owned workflow.") from error
    if len(relative.parts) != 2 or relative.name != "COMPILE_REPAIR_RETURN.json":
        raise RuntimeError("Facade/harness compile repair return-file layout differs from the owned run layout.")
    if "run_directory" in report and report["run_directory"] != str(path.parent):
        raise RuntimeError("Compile-repair run directory differs from its return file.")
    raw, fp, _ = _transport_read_file(path, _TRANSPORT_MAX_BYTES)
    retained = json.loads(raw.decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if retained != report:
        raise RuntimeError("Printed facade/harness compile repair return differs from its retained JSON file.")
    successful = report["outcome"] in {"compile_repair_qualified", "diagnostic_complete", "admission_deferred"}
    if successful:
        if report.get("original_preserved") is not True:
            raise RuntimeError("Completed facade/harness compile repair return lacks verified original preservation.")
        if (report.get("diagnostic_only") is not True or report.get("repair_qualified") is not False
                or report.get("native_tests_started") is not False or type(report.get("test_count")) is not int
                or report["test_count"] != 0 or report.get("master_updated") is not False
                or report.get("other_refs_updated") is not False):
            raise RuntimeError("Completed compile-repair return is not an unqualified diagnostic without native tests.")
        if report["outcome"] == "admission_deferred":
            no_changes = ("builds_started", "crate_builds_started", "source_edits_started", "worktree_created",
                          "own_journal_created", "cache_copy_started")
            if (report.get("phase") != "build-admission" or report.get("compilations") != {}
                    or report.get("compile_qualified") is not False
                    or any(report.get(key) is not None for key in ("candidate_head", "candidate_tree", "created_commit"))
                    or any(report.get(field) is not False for field in no_changes)):
                raise RuntimeError("Deferred compile-repair return does not prove build admission before worktree, journal, cache or compiler mutation.")
        else:
            if report.get("phase") != "complete":
                raise RuntimeError("Completed compile-repair return is not an unqualified diagnostic without native tests.")
            successes = _transport_compilations030(report, repo)
            if report["outcome"] == "compile_repair_qualified":
                changed = report.get("changed_paths")
                if (report.get("compile_qualified") is not True or successes != 2
                        or report.get("builds_started") is not True or report.get("crate_builds_started") is not True
                        or not isinstance(changed, list) or len(changed) != len(_TRANSPORT_PATHS030)
                        or set(changed) != _TRANSPORT_PATHS030
                        or any(not isinstance(report.get(key), str) or not re.fullmatch(r"[0-9a-f]{40}", report[key])
                               for key in ("candidate_head", "candidate_tree"))):
                    raise RuntimeError("Compile-repair qualification lacks both successful builds and the exact committed source scope.")
            elif (report.get("compile_qualified") is not False or successes == 2
                    or any(report.get(key) is not None for key in ("candidate_head", "candidate_tree", "created_commit"))):
                raise RuntimeError("Compiler-failure diagnostic cannot claim a qualified compile repair or a created commit.")
    if (child_status == 0) != successful:
        raise RuntimeError("Coordinator exit status disagrees with its recorded outcome.")
    return report, path, raw, fp


def _transport_existing_state(repo_fd):
    try:
        raw, fp, stamp = _transport_read_at(repo_fd, "return-buffer", _TRANSPORT_MAX_BYTES, own=True)
        return fp, stamp
    except FileNotFoundError:
        return {"kind": "absent"}, None


def _transport_replace_buffer(repo, payload):
    if len(payload) > _TRANSPORT_MAX_BYTES:
        raise RuntimeError("Complete output plus return JSON exceeds the 8 MiB return-buffer allowance.")
    repo_fd = _transport_open_directory(repo)
    temporary = ".return-buffer-transport-" + uuid.uuid4().hex + ".tmp"
    made = False
    replacement_done = False
    try:
        before, stamp = _transport_existing_state(repo_fd)
        mode = before["mode"] if before["kind"] == "file" else 0o600
        fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=repo_fd)
        made = True
        with os.fdopen(fd, "wb") as stream:
            os.fchmod(stream.fileno(), mode)
            stream.write(payload)
            stream.flush()
            os.fsync(stream.fileno())
        try:
            current = os.stat("return-buffer", dir_fd=repo_fd, follow_symlinks=False)
        except FileNotFoundError:
            current = None
        if (stamp is None and current is not None) or (stamp is not None and (current is None or _transport_stamp(current) != stamp)):
            raise RuntimeError("Return-buffer changed before its authorized atomic replacement.")
        # Replacing the directory entry never follows a target symlink and
        # never writes through a source hard link to another pathname.
        os.replace(temporary, "return-buffer", src_dir_fd=repo_fd, dst_dir_fd=repo_fd)
        replacement_done = True
        made = False
        os.fsync(repo_fd)
        written, after, _ = _transport_read_at(repo_fd, "return-buffer", _TRANSPORT_MAX_BYTES, own=True)
        if written != payload or after["mode"] != mode:
            raise RuntimeError("Published return-buffer does not match the complete authorized payload.")
        return before, after
    except BaseException as error:
        if replacement_done:
            raise _TransportReplacementError("Return-buffer was replaced, but final durability/readback verification failed: " + str(error)) from error
        raise
    finally:
        if made:
            try:
                os.unlink(temporary, dir_fd=repo_fd)
            except FileNotFoundError:
                pass
        os.close(repo_fd)


def _transport_write_receipt(directory, value):
    raw = (json.dumps(value, sort_keys=True, indent=2) + "\n").encode()
    if len(raw) > 65536:
        raise RuntimeError("Transport receipt exceeds its finite allowance.")
    parent = _transport_open_directory(directory)
    try:
        fd = os.open("TRANSPORT_RETURN.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=parent)
        with os.fdopen(fd, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        os.fsync(parent)
    finally:
        os.close(parent)


def _transport_lock(repo):
    workflow = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW
    parent = _transport_open_directory(workflow)
    try:
        fd = os.open("run.lock", os.O_RDWR | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=parent)
    finally:
        os.close(parent)
    try:
        info = os.fstat(fd)
        if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid():
            raise RuntimeError("Existing compile-repair transport lock is nonordinary or foreign-owned.")
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return fd
    except BaseException:
        os.close(fd)
        raise


def run_with_transport(script_path, repo, apply):
    """Run one coordinator child, mirror every byte, then replace one buffer."""
    child = None
    child_status = None
    capture = bytearray()
    overflow = False
    mirror_error = None
    report = return_path = return_fp = None
    buffer_written = False
    buffer_replaced = False
    lock_fd = None
    previous_handlers = {}
    total_output = 0
    try:
        if type(apply) is not bool:
            raise RuntimeError("Transport apply/preview selection is malformed.")
        repo = Path(os.path.abspath(Path(repo).expanduser()))
        script_path = Path(os.path.abspath(Path(script_path).expanduser()))
        if any(char in str(repo) for char in ("\x00", "\n", "\r")):
            raise RuntimeError("Transport repository path framing differs.")
        fd = _transport_open_directory(repo)
        os.close(fd)
        _transport_read_file(script_path, 2 * 1024 * 1024)
        command = [sys.executable, "-I", "-S", str(script_path), "--coordinator", str(repo), "apply" if apply else "preview"]
        child = subprocess.Popen(command, cwd=repo, stdin=subprocess.DEVNULL,
                                 stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                 bufsize=0, start_new_session=True)
        if threading.current_thread() is threading.main_thread():
            def forward(signum, frame):
                if child.poll() is None:
                    try:
                        child.send_signal(signum)
                    except ProcessLookupError:
                        pass
            for signum in (signal.SIGINT, signal.SIGTERM):
                previous_handlers[signum] = signal.signal(signum, forward)
        while True:
            try:
                chunk = os.read(child.stdout.fileno(), 65536)
            except InterruptedError:
                continue
            if not chunk:
                break
            total_output += len(chunk)
            if mirror_error is None:
                try:
                    _transport_emit(chunk)
                except OSError as error:
                    mirror_error = str(error)
            room = _TRANSPORT_MAX_BYTES - len(capture)
            if len(chunk) > room:
                overflow = True
            if room > 0:
                capture.extend(chunk[:room])
        child_status = child.wait()
        if overflow:
            raise RuntimeError("Complete coordinator output exceeds the 8 MiB capture allowance; prior return-buffer was retained.")
        report, return_path, return_raw, return_fp = _transport_report(bytes(capture), repo, child_status)
        if mirror_error is not None:
            raise RuntimeError("Terminal mirroring failed: " + mirror_error)
        confirmation = ("RETURN_BUFFER_UPDATED_AFTER_COORDINATOR_EXIT " + str(repo / "return-buffer") + "\n").encode()
        payload = (bytes(capture) + b"\nBEGIN_FACADE_HARNESS_COMPILE_RETURN_FILE_JSON\n" + return_raw
                   + (b"" if return_raw.endswith(b"\n") else b"\n")
                   + b"END_FACADE_HARNESS_COMPILE_RETURN_FILE_JSON\n" + confirmation)
        if len(payload) > _TRANSPORT_MAX_BYTES:
            raise RuntimeError("Complete coordinator output plus exact return file exceeds the 8 MiB payload allowance; prior return-buffer was retained.")
        # No lock is held during the coordinator. Reacquire its existing own
        # workflow lock afterward to avoid overlapping a new snapshot/run.
        lock_fd = _transport_lock(repo)
        before, after = _transport_replace_buffer(repo, payload)
        buffer_written = True
        buffer_replaced = True
        transport = {"schema": "bitcoinrs-facade-harness-compile030-buffer-transport-v1", "outcome": "written",
                     "coordinator_exit_code": child_status, "coordinator_outcome": report["outcome"],
                     "coordinator_return_file": str(return_path), "coordinator_return_fingerprint": return_fp,
                     "target": str(repo / "return-buffer"), "captured_output_bytes": total_output,
                     "payload_bytes": len(payload), "payload_sha256": hashlib.sha256(payload).hexdigest(),
                     "full_output_captured": True, "authorized_overwrite_after_coordinator": True,
                     "coordinator_preservation_verified": (report.get("original_preserved") is True),
                     "preservation_scope": "Buffer publication occurs after coordinator exit and return-file validation; preservation results belong to the coordinator report.",
                     "buffer_before": before, "buffer_after": after}
        _transport_write_receipt(return_path.parent, transport)
        _transport_emit(confirmation)
        return 0 if child_status == 0 else 1
    except BaseException as error:
        buffer_replaced = buffer_replaced or getattr(error, "buffer_replaced", False)
        if child is not None and child_status is None:
            # Transport failures must not terminate/restart an in-flight
            # evidence capture. Finish draining its bounded coordinator lifecycle.
            try:
                while True:
                    chunk = os.read(child.stdout.fileno(), 65536)
                    if not chunk:
                        break
                    try:
                        _transport_emit(chunk)
                    except OSError:
                        pass
                child_status = child.wait()
            except BaseException as cleanup_error:
                sys.stderr.write("Transport could not finish observing the coordinator: " + str(cleanup_error)[:512] + "\n")
        actual = "coordinator_exit=" + str(child_status)
        if report is not None:
            actual += " coordinator_outcome=" + report["outcome"]
        suffix = ("; return-buffer was already written and verified" if buffer_written
                  else "; return-buffer was already replaced; final verification is incomplete" if buffer_replaced
                  else "; prior return-buffer was retained")
        sys.stderr.write("Return-buffer transport failed: " + type(error).__name__ + ": " + str(error)[:2048] + "; " + actual + suffix + ". No coordinator retry was started.\n")
        sys.stderr.flush()
        return 1
    finally:
        for signum, handler in previous_handlers.items():
            signal.signal(signum, handler)
        if child is not None and child.stdout is not None:
            child.stdout.close()
        if lock_fd is not None:
            os.close(lock_fd)


def run030(repo):
    report = {'workflow': WORKFLOW, 'controller_revision': 1, 'outcome': 'failed', 'phase': 'preflight',
        'diagnostic_only': True, 'compile_qualified': False, 'repair_qualified': False,
        'master_updated': False, 'other_refs_updated': False, 'source_edits_started': False,
        'worktree_created': False, 'own_journal_created': False, 'cache_copy_started': False,
        'builds_started': False, 'crate_builds_started': False, 'native_tests_started': False,
        'debuggers_started': False, 'test_count': 0, 'full_coverage_claimed': False,
        'validation_reused': False, 'runtime_validation_claimed': False,
        'base': BASE, 'base_tree': BASE_TREE, 'branch': BRANCH,
        'targets': ['bitcoin-leveldb', 'bitcoinleveldb-harness'], 'compilations': {}, 'readouts': {},
        'source_review': source_review_notes030(),
        'scope': 'Two hash-bound import/reexport fixes in a new candidate checkout. Compile both crates separately under unchanged finite controls. Never execute native test artifacts or update master; compile-only qualification is not runtime repair qualification.',
        'postrun_transport': 'Replace authorized return-buffer only after coordinator exit and exact printed/saved report validation.'}
    held, core, baseline, directory, worktree, before, lineage = [], None, None, None, None, None, None
    state, state_path, files, candidate_head, installed = None, None, None, BASE, False
    try:
        repo = Path(os.path.abspath(Path(repo).expanduser())); report['repository'] = str(repo)
        baseline, previous, finalizer, core, helper = runtime030(repo); previous.checked_environment023(core)
        common = core.repository(core.Inventory(repo)); held = core.acquire_locks(common)
        own = common / 'codex-repairs' / WORKFLOW; own.mkdir(mode=0o700, exist_ok=True)
        fd = core.open_dir(own); os.close(fd)
        lock = os.open(own / 'run.lock', os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600)
        held.append((lock, own / 'run.lock'))
        if not stat.S_ISREG(os.fstat(lock).st_mode): raise RuntimeError('Compile repair workflow lock is not ordinary.')
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        candidate_original_guard023(core, repo); worktree = repo.parent / (repo.name + SUFFIX)
        before = core.normalized_original(core.protected_snapshot(core, repo), worktree)
        lineage = baseline030(baseline, core, repo)
        report.update(intake_before=lineage, preservation_before_sha256=core.digest(core.encoded(before)),
                      locks_held=[str(path) for _, path in held], worktree=str(worktree))
        runs = own / 'runs'; runs.mkdir(mode=0o700, exist_ok=True); fd = core.open_dir(runs); os.close(fd)
        directory = runs / (dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ') + '-' + uuid.uuid4().hex[:8])
        directory.mkdir(mode=0o700)
        script = directory / 'compilation-controller030.py'; raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB027)
        core.exclusive(script, raw, 0o700); files = files030(raw)
        report.update(run_directory=str(directory), controller_sha256=core.digest(raw), guard_sha256=core.GUARD_SHA,
                      source_plan={name: core.digest(value) for name, value in files.items()})
        state_path = own / 'state.json'
        guard = core.load_guard().Guard(directory / 'guard', script,
                                       dict(core.Inventory(repo).env, BITCOIN_RS_COMPILE030_REPO=str(repo)))
        def preserve():
            candidate_original_guard023(core, repo)
            if (core.normalized_original(core.protected_snapshot(core, repo), worktree) != before
                    or baseline030(baseline, core, repo) != lineage):
                raise RuntimeError('Protected original, prior compilation evidence or read input source changed.')
        def phase(label, command, kind='source', cwd=None, env=None, compiler=False):
            preserve(); report['phase'] = label
            print('Protected final-crates compile030 phase: ' + label, flush=True)
            row = guard.run(label, command, cwd or repo, env or {}, kind=kind)
            report['readouts'][label] = row
            if not compiler: core.require_guard(row)
            return row
        def git(label, args, cwd, extra=None, tag=None):
            tag = tag or label; spec = directory / (tag + '.git.spec.json'); output = directory / (tag + '.git.json')
            value = {'label': label, 'args': args, 'cwd': str(cwd), 'output': str(output)}
            value.update(extra or {}); core.exclusive(spec, core.encoded(value))
            phase('git-' + tag, [sys.executable, '-I', '-S', str(script), '--git', str(repo), str(spec)], 'build', cwd)
            evidence = json.loads(core.read_file(output, 2 * MIB027)[1]); report.setdefault('git_evidence', {})[tag] = evidence
            if evidence.get('exit_code') != 0 or evidence.get('args') != args:
                raise RuntimeError('Exact isolated compile repair Git evidence differs.')
            return evidence['stdout'].strip()
        phase('preflight', [sys.executable, '-I', '-S', '-c', "print('GUARD_PREFLIGHT_OK')"], 'probe')
        inputs = directory / 'inputs-before.json'
        phase('inputs-before', [sys.executable, '-I', '-S', str(script), '--inputs', str(repo), str(inputs)])
        if json.loads(core.read_file(inputs, 2 * MIB027)[1]) != lineage:
            raise RuntimeError('Fresh input proof differs from the retained028 source/compiler binding.')
        if os.path.lexists(state_path):
            raise RuntimeError('An earlier compile030 reservation is retained; automatic recompilation is blocked.')
        inv = candidate_original_guard023(core, repo)
        _, existing = inv.git('for-each-ref', '--format=%(refname)', 'refs/heads/' + BRANCH)
        if existing.strip() or os.path.lexists(worktree) or os.path.lexists(own / 'cargo-target'):
            raise RuntimeError('Compile repair branch, checkout or private target is occupied; retained without reuse.')
        admission_command = [sys.executable, '-I', '-S', '-c', "print('COMPILE030_BUILD_ADMISSION_OK')"]
        admission = phase('build-admission', admission_command, 'build', compiler=True)
        report['build_admission'] = baseline.build_admission028(core, admission, admission_command, repo)
        if report['build_admission']['admitted'] is False:
            _, existing = inv.git('for-each-ref', '--format=%(refname)', 'refs/heads/' + BRANCH)
            if existing.strip() or any(os.path.lexists(p) for p in (state_path, worktree, own / 'cargo-target')):
                raise RuntimeError('Deferred compile admission cannot reserve candidate or cache state.')
            raise baseline.AdmissionDeferred028()
        state = {'workflow': WORKFLOW, 'base': BASE, 'branch': BRANCH, 'worktree': str(worktree),
            'run_directory': str(directory), 'phase': 'reserved', 'source_plan': report['source_plan']}
        core.atomic_json(state_path, state); report['own_journal_created'] = True
        identity_spec, identity_out = directory / 'identity-input.json', directory / 'identity.json'
        core.exclusive(identity_spec, core.encoded({key: os.environ[key] for key in finalizer.IDENTITY_ENV if key in os.environ}))
        phase('read-configured-identity', [sys.executable, '-I', '-S', str(script), '--identity', str(repo), str(identity_spec), str(identity_out)])
        ident = json.loads(core.read_file(identity_out, MIB027)[1]); report['configured_identity'] = ident
        identity_env = {'GIT_' + role.upper() + '_' + field.upper(): ident[role][field.lower()]
                        for role in ('author', 'committer') for field in ('NAME', 'EMAIL')}
        git('worktree', ['worktree', 'add', '-b', BRANCH, str(worktree), BASE], repo)
        report['worktree_created'] = True; report['new_ref_created'] = 'refs/heads/' + BRANCH
        report['source_edits_started'] = True
        phase('install-source', [sys.executable, '-I', '-S', str(script), '--install', str(repo)], 'source', worktree)
        git('add', ['add', '--', *sorted(files)], worktree)
        check_candidate023(core, repo, worktree, files, BASE); installed = True
        state['phase'] = 'installed'; core.atomic_json(state_path, state)
        source_cache = common / 'codex-repairs/leveldb-facade-harness-baseline028/cargo-target'
        target = own / 'cargo-target'; report['target_directory'] = str(target)
        cache_output = directory / 'cache-copy.json'; cache_locks = []
        try:
            for name in ('.cargo-build-lock', '.cargo-lock'):
                fd = os.open(source_cache / 'debug' / name, os.O_RDWR | os.O_NOFOLLOW); cache_locks.append(fd)
                if not stat.S_ISREG(os.fstat(fd).st_mode): raise RuntimeError('Retained cache lock is not ordinary.')
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
            report['cache_copy_started'] = True
            phase('seed-private-cache', [sys.executable, '-I', '-S', str(script), '--copy-cache', str(repo),
                                        str(source_cache), str(target), str(cache_output)], 'build')
        finally:
            for fd in cache_locks: os.close(fd)
        report['cache_copy'] = json.loads(core.read_file(cache_output, MIB027)[1])
        if report['cache_copy'].get('outcome') != 'copied' or report['cache_copy'].get('hard_links_created') is not False:
            raise RuntimeError('Independent compile repair cache seed did not verify.')
        nix_shell = shutil.which('nix-shell', path=core.Inventory(repo).env.get('PATH'))
        if nix_shell is None:
            raise RuntimeError('Offline Nix shell entry point is unavailable; no alternate build was attempted.')
        # Preserve the dispatch entry name: resolving the Nix symlink changes argv[0].
        nix_shell = os.path.abspath(nix_shell)
        for label in ('facade', 'harness'):
            preserve(); check_candidate023(core, repo, worktree, files, BASE)
            spec_path = directory / ('cargo-' + label + '.spec.json'); evidence_path = directory / ('cargo-' + label + '.json')
            spec = {'command': COMMANDS027[label], 'cwd': str(worktree), 'target_directory': str(target),
                    'evidence_path': str(evidence_path)}
            core.exclusive(spec_path, core.encoded(spec)); state.update(phase='compiling', compilation_pending=label)
            core.atomic_json(state_path, state)
            command = [nix_shell, '--option', 'max-jobs', '0', '--option', 'builders', '', '--option', 'substitute', 'false',
                str(worktree / 'shell.nix'), '--run',
                shlex.join([sys.executable, '-I', '-S', str(script), '--cargo-observer', str(repo), str(spec_path)])]
            report.update(builds_started=True, crate_builds_started=True)
            row = phase('compile-' + label, command, 'build', worktree,
                {'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '1', 'CARGO_INCREMENTAL': '0', 'RUST_LOG': 'off'}, compiler=True)
            if not evidence_path.exists(): raise RuntimeError('Fresh Cargo observer did not publish bounded evidence.')
            evidence = json.loads(core.read_file(evidence_path, MIB027)[1]); report.setdefault('cargo_evidence', {})[label] = evidence
            if row.get('exit_code') == 0:
                summary = qualify_compile030(core, label, row, evidence, spec, target,
                    guard_command=command, run_directory=directory)
            else:
                if (row.get('command') != command or row.get('cwd') != str(worktree)
                        or evidence.get('schema') != 'bitcoinrs-final027-cargo-observer-v1'
                        or evidence.get('priority_records_omitted') != 0):
                    raise RuntimeError('Compiler failure lacks exact command/source evidence or omits priority diagnostics.')
                summary = baseline.classify_compile027(core, label, row, evidence, spec, target)
            summary['evidence_file'] = str(evidence_path)
            summary['evidence_fingerprint'] = core.read_file(evidence_path, MIB027, content=False)[0]
            report['compilations'][label] = summary
            state.pop('compilation_pending'); state.setdefault('compilations', {})[label] = summary['outcome']
            state['phase'] = 'compiled-' + label; core.atomic_json(state_path, state)
            check_candidate023(core, repo, worktree, files, BASE)
        inputs = directory / 'inputs-after.json'
        phase('inputs-after', [sys.executable, '-I', '-S', str(script), '--inputs', str(repo), str(inputs)])
        if json.loads(core.read_file(inputs, 2 * MIB027)[1]) != lineage:
            raise RuntimeError('Original source/compiler evidence changed during candidate compilation.')
        preserve(); artifact_check030(core, report['compilations'])
        if any(value['outcome'] != 'compiled' for value in report['compilations'].values()):
            report.update(outcome='diagnostic_complete', phase='complete',
                diagnostic_result='At least one fresh controlled compilation reported compiler errors; no candidate commit was created.')
        else:
            receipt = {'workflow': WORKFLOW, 'controller_revision': 1, 'base': BASE, 'base_tree': BASE_TREE,
                'compile_qualified': True, 'repair_qualified': False, 'runtime_validation_claimed': False,
                'source_plan': report['source_plan'], 'source_review': report['source_review'],
                'compilations': report['compilations'], 'commands': COMMANDS027,
                'guard_sha256': core.GUARD_SHA, 'run_directory': str(directory), 'test_count': 0,
                'native_tests_started': False, 'full_coverage_claimed': False,
                'prior_report': lineage['report'], 'prior_report_fingerprint': lineage['report_fingerprint']}
            destination = worktree / RECEIPT_PATH; destination.parent.mkdir(parents=True, exist_ok=True)
            fd = core.open_dir(destination.parent); os.close(fd)
            helper.candidate_exclusive015(core, destination, core.encoded(receipt), 0o644)
            files[RECEIPT_PATH] = core.encoded(receipt)
            git('add', ['add', '--', *sorted(files)], worktree, tag='add-final')
            check_candidate023(core, repo, worktree, files, BASE)
            tree = git('tree', ['write-tree'], worktree)
            if not _candidate_oid023(tree): raise RuntimeError('Compile candidate tree identity is absent.')
            message = directory / 'commit-message.txt'; core.exclusive(message, COMMIT_MESSAGE)
            intent = {'tree': tree, 'parents': [BASE], 'identity': ident, 'message_sha256': core.digest(COMMIT_MESSAGE)}
            state.update(phase='commit-intent', receipt=receipt, commit_intent=intent); core.atomic_json(state_path, state)
            head = git('commit-tree', ['commit-tree', tree, '-p', BASE, '-F', str(message)], worktree,
                       {'tree': tree, 'identity_env': identity_env})
            if not _candidate_oid023(head): raise RuntimeError('Compile candidate commit identity is absent.')
            state['created_commit'] = head; core.atomic_json(state_path, state)
            git('head', ['update-ref', 'refs/heads/' + BRANCH, head, BASE], worktree, {'head': head})
            candidate_head = head
            helper.verify_candidate_commit(finalizer, core, repo, head, intent, files)
            preserve(); artifact_check030(core, report['compilations'])
            state['head'] = head
            report.update(outcome='compile_repair_qualified', phase='complete', compile_qualified=True,
                candidate_head=head, candidate_tree=tree, receipt=receipt, changed_paths=sorted(files))
    except BaseException as error:
        if baseline is not None and isinstance(error, baseline.AdmissionDeferred028):
            report.update(outcome='admission_deferred', phase='build-admission',
                diagnostic_result='Unchanged build admission refused before candidate/cache reservation.')
        else:
            report.update(outcome='failed', compile_qualified=False, error=type(error).__name__ + ': ' + str(error))
    finally:
        if before is not None:
            try:
                report['intake_after'] = baseline030(baseline, core, repo)
                after = core.normalized_original(core.protected_snapshot(core, repo), worktree)
                report['preservation_after_sha256'] = core.digest(core.encoded(after))
                report['original_preserved'] = after == before and report['intake_after'] == lineage
                if report['original_preserved'] is not True: raise RuntimeError('Original user/evidence/source state was not preserved.')
                if installed:
                    report['candidate_after'] = check_candidate023(core, repo, worktree, files, candidate_head)
                if set(report['compilations']) == {'facade', 'harness'}:
                    artifact_check030(core, report['compilations'])
            except BaseException as error:
                report.update(outcome='failed', compile_qualified=False, original_preserved=False, preservation_error=str(error))
        if directory is not None:
            report['return_file'] = str(directory / 'COMPILE_REPAIR_RETURN.json')
            if len(core.encoded(report)) > 3 * MIB027:
                report.update(outcome='failed', compile_qualified=False, publication_error='Compile repair return exceeds its 3 MiB allowance.')
                report.pop('intake_before', None); report.pop('intake_after', None)
            try:
                core.atomic_json(Path(report['return_file']), report)
                if state is not None and report['outcome'] in {'compile_repair_qualified', 'diagnostic_complete'}:
                    state.update(phase='complete' if report['compile_qualified'] else 'diagnostic-complete',
                        report=report['return_file'], report_fingerprint=core.read_file(Path(report['return_file']), 3 * MIB027, content=False)[0])
                    core.atomic_json(state_path, state)
            except BaseException as error:
                report.update(outcome='failed', compile_qualified=False, publication_error=str(error))
                core.atomic_json(Path(report['return_file']), report)
        for fd, _ in held: os.close(fd)
    return report


def main030():
    sys.dont_write_bytecode = True; resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    if len(sys.argv) == 3 and sys.argv[1] == '--guard-child':
        plan = json.loads(bootstrap_read(Path(sys.argv[2]), MIB027))
        _, _, _, core, _ = runtime030(Path(plan['env']['BITCOIN_RS_COMPILE030_REPO']))
        return core.load_guard().guard_child(Path(sys.argv[2]))
    if len(sys.argv) == 4 and sys.argv[1] == '--inputs':
        return inputs_worker030(Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 5 and sys.argv[1] == '--identity':
        return identity_worker030(Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]))
    if len(sys.argv) == 4 and sys.argv[1] == '--git':
        return git_worker030(Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 3 and sys.argv[1] == '--install': return install_worker030(Path(sys.argv[2]))
    if len(sys.argv) == 6 and sys.argv[1] == '--copy-cache':
        return seed_worker030(Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]), Path(sys.argv[5]))
    if len(sys.argv) == 4 and sys.argv[1] == '--cargo-observer':
        return observer_worker030(Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 4 and sys.argv[1] == '--coordinator' and sys.argv[3] == 'apply':
        resource.setrlimit(resource.RLIMIT_AS, (768 * MIB027,) * 2)
        signal.signal(signal.SIGALRM, lambda *_: (_ for _ in ()).throw(RuntimeError('Compile030 coordinator deadline reached.')))
        signal.alarm(3600)
        report = run030(Path(sys.argv[2]))
        print('BEGIN_FACADE_HARNESS_COMPILE_RETURN', flush=True); print(json.dumps(report, indent=2), flush=True)
        print('END_FACADE_HARNESS_COMPILE_RETURN', flush=True)
        return 0 if report['outcome'] in {'compile_repair_qualified', 'diagnostic_complete', 'admission_deferred'} else 1
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=Path.home() / 'bitcoin-rs')
    args = parser.parse_args()
    return run_with_transport(Path(__file__).absolute(), Path(os.path.abspath(args.repo.expanduser())), True)


if __name__ == '__main__': raise SystemExit(main030())
