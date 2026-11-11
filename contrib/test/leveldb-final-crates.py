#!/usr/bin/env python3
"""Run the repaired LevelDB crates under finite resource controls on the current working tree.

Default: all four crates, routine44 cases. Explicit --suite full adds the five
original stress cases. No source edits, Git ref changes or historical repair
controller imports; each invocation builds through Cargo and runs exact cases.
"""
import argparse
import datetime as dt
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import selectors
import shlex
import shutil
import signal
import stat
import subprocess
import sys
import time
import types
import uuid
from types import MappingProxyType
sys.dont_write_bytecode = True
MIB033 = 1024 * 1024
GUARD_SHA033 = '97b4ee49e7dfd2d120f4e91d7912d2df8efd764d26ec372097d67cb2ec3093ae'
INPUT_CODE_PATHS033 = ('contrib/test/leveldb-final-crates.py', 'contrib/test/leveldb_resource_guard.py')


"""Bounded standard-library filesystem and Git support for the ordinary runner."""

def encoded033(value):
    return (json.dumps(value, sort_keys=True, indent=2, allow_nan=False) + '\n').encode()


def digest033(raw):
    return hashlib.sha256(raw).hexdigest()


def ordinary_dir(path):
    path = Path(path)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Expected an absolute ordinary directory.')
    descriptor = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=descriptor)
            os.close(descriptor); descriptor = child
        return descriptor
    except BaseException:
        os.close(descriptor); raise


def read_file033(path, limit, content=True):
    path = Path(path)
    if type(limit) is not int or limit < 0 or type(content) is not bool:
        raise RuntimeError('Ordinary file read requires a finite limit and Boolean content selection.')
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Ordinary file path must be absolute.')
    parent = ordinary_dir(path.parent)
    descriptor = None
    try:
        descriptor = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=parent)
        before = os.fstat(descriptor)
        if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
            raise RuntimeError('Expected a bounded ordinary file: ' + str(path))
        digest = hashlib.sha256(); pieces = []; total = 0
        while True:
            block = os.read(descriptor, min(65536, limit + 1 - total))
            if not block: break
            total += len(block)
            if total > limit: raise RuntimeError('File grew past its finite allowance: ' + str(path))
            digest.update(block)
            if content: pieces.append(block)
        after = os.fstat(descriptor); current = os.stat(path.name, dir_fd=parent, follow_symlinks=False)
        fields = ('st_dev', 'st_ino', 'st_mode', 'st_size', 'st_mtime_ns', 'st_ctime_ns')
        if (total != before.st_size or any(getattr(before, field) != getattr(after, field)
                or getattr(before, field) != getattr(current, field) for field in fields)):
            raise RuntimeError('File changed during its bounded read: ' + str(path))
        return {'kind': 'file', 'mode': stat.S_IMODE(before.st_mode), 'size': total,
                'sha256': digest.hexdigest()}, b''.join(pieces) if content else None
    finally:
        if descriptor is not None: os.close(descriptor)
        os.close(parent)


def atomic_json033(path, value):
    path = Path(path); raw = encoded033(value)
    if len(raw) > 8 * MIB033: raise RuntimeError('Runner JSON exceeds its 8 MiB allowance.')
    parent = ordinary_dir(path.parent); temporary = '.' + path.name + '.tmp-' + uuid.uuid4().hex
    made = False
    try:
        try:
            info = os.stat(path.name, dir_fd=parent, follow_symlinks=False)
            if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid():
                raise RuntimeError('Runner output is nonordinary or foreign-owned.')
        except FileNotFoundError: pass
        descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=parent)
        made = True
        with os.fdopen(descriptor, 'wb') as stream:
            stream.write(raw); stream.flush(); os.fsync(stream.fileno())
        os.replace(temporary, path.name, src_dir_fd=parent, dst_dir_fd=parent)
        made = False; os.fsync(parent)
    finally:
        if made:
            try: os.unlink(temporary, dir_fd=parent)
            except FileNotFoundError: pass
        os.close(parent)


def checked_environment033():
    exact = {'LD_PRELOAD', 'LD_AUDIT', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTC', 'RUSTDOC',
        'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER', 'RUSTUP_TOOLCHAIN', 'CARGO_BUILD_TARGET',
        'CARGO_BUILD_RUSTFLAGS', 'CARGO_BUILD_RUSTC', 'CARGO_BUILD_RUSTC_WRAPPER'}
    bad = [key for key, value in os.environ.items() if value and (key in exact
        or key.startswith('CARGO_PROFILE_') or key.startswith('GIT_CONFIG_COUNT')
        or key == 'GIT_CONFIG_PARAMETERS' or (key.startswith('CARGO_TARGET_')
            and key.endswith(('_RUSTFLAGS', '_RUSTC_WRAPPER', '_RUNNER'))))]
    if bad: raise RuntimeError('Execution or build overrides require review: ' + repr(sorted(bad)))
    env = {key: value for key, value in os.environ.items()
           if not key.startswith(('GIT_', 'BITCOIN_RS_')) and key not in {'PYTHONPATH', 'PYTHONHOME'}}
    env.update(GIT_OPTIONAL_LOCKS='0', GIT_NO_REPLACE_OBJECTS='1', GIT_NO_LAZY_FETCH='1',
        GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_SYSTEM='/dev/null', GIT_CONFIG_GLOBAL='/dev/null',
        GIT_ATTR_NOSYSTEM='1', GIT_TERMINAL_PROMPT='0', LC_ALL='C', RUST_LOG='off', RUST_BACKTRACE='0')
    return env


def repository_dirs033(repo):
    repo = Path(repo); descriptor = ordinary_dir(repo); os.close(descriptor)
    path = repo / '.git'
    info = path.lstat()
    if stat.S_ISDIR(info.st_mode):
        gitdir = path
    elif stat.S_ISREG(info.st_mode):
        raw = read_file033(path, 16384)[1].decode().strip()
        if not raw.startswith('gitdir: ') or '\n' in raw:
            raise RuntimeError('Unsupported Git worktree metadata.')
        gitdir = Path(os.path.abspath(repo / raw[8:]))
    else: raise RuntimeError('Repository .git must be an ordinary file or directory.')
    descriptor = ordinary_dir(gitdir); os.close(descriptor)
    common_path = gitdir / 'commondir'
    try:
        raw = read_file033(common_path, 16384)[1].decode().strip()
        if not raw or '\n' in raw: raise RuntimeError('Malformed Git common-directory metadata.')
        common = Path(os.path.abspath(gitdir / raw))
    except FileNotFoundError:
        common = gitdir
    for directory in (common, common / 'objects'):
        descriptor = ordinary_dir(directory); os.close(descriptor)
    read_file033(gitdir / 'HEAD', 16384)
    return gitdir, common


def git033(repo, args, allowed=(0,)):
    queries = {
        ('rev-parse', 'HEAD'), ('rev-parse', '--verify', 'HEAD'),
        ('rev-parse', '--verify', 'HEAD^{tree}'),
        ('rev-parse', '--symbolic-full-name', 'HEAD'),
        ('status', '--porcelain=v1', '-z', '--untracked-files=all'),
        ('ls-files', '--stage', '-z'),
        ('ls-files', '--others', '--exclude-standard', '-z'),
        ('ls-files', '--others', '--ignored', '--exclude-standard', '-z'),
        ('config', '--name-only', '--get-regexp', r'^filter\.')}
    if (not isinstance(args, (list, tuple)) or any(not isinstance(part, str) for part in args)
            or tuple(args) not in queries or tuple(allowed) not in {(0,), (0, 1)}
            or tuple(allowed) == (0, 1) and args[0] != 'config'):
        raise RuntimeError('Git query differs from the runner read-only grammar.')
    prefix = ['git', '--no-pager', '--no-replace-objects', '--no-optional-locks',
        '-c', 'core.fsmonitor=false', '-c', 'core.untrackedCache=false', '-c', 'core.preloadIndex=false',
        '-c', 'core.hooksPath=/dev/null', '-c', 'diff.external=', '-c', 'protocol.allow=never', '-c', 'credential.helper=']
    child = subprocess.Popen(prefix + list(args), cwd=repo, env=checked_environment033(), stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
    selector = selectors.DefaultSelector(); pieces = {child.stdout: bytearray(), child.stderr: bytearray()}
    deadline = time.monotonic() + 20
    try:
        for stream in pieces:
            os.set_blocking(stream.fileno(), False); selector.register(stream, selectors.EVENT_READ)
        while selector.get_map():
            if time.monotonic() > deadline: raise RuntimeError('Read-only Git query deadline reached.')
            for event, _ in selector.select(min(0.1, max(0, deadline - time.monotonic()))):
                block = os.read(event.fileobj.fileno(), 65536)
                if not block: selector.unregister(event.fileobj); continue
                pieces[event.fileobj].extend(block)
                maximum = 8 * MIB033 if event.fileobj is child.stdout else 65536
                if len(pieces[event.fileobj]) > maximum: raise RuntimeError('Read-only Git query output cap reached.')
        code = child.wait(timeout=max(0.1, deadline - time.monotonic()))
        if code not in allowed: raise RuntimeError('Read-only Git query failed: ' + args[0] + ': ' + str(code))
        return bytes(pieces[child.stdout])
    finally:
        if child.poll() is None:
            try: os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError: pass
            child.wait(timeout=2)
        selector.close(); child.stdout.close(); child.stderr.close()


def load_guard033(repo=None):
    path = Path(__file__).absolute().with_name('leveldb_resource_guard.py')
    fp, raw = read_file033(path, MIB033)
    if fp['sha256'] != GUARD_SHA033:
        raise RuntimeError('The standalone resource guard differs from its reviewed SHA256.')
    module = types.ModuleType('leveldb_final_crates_guard033'); module.__file__ = str(path)
    sys.modules[module.__name__] = module; exec(compile(raw, str(path), 'exec'), module.__dict__)
    return module


def metadata_root033(repo):
    _, common = repository_dirs033(repo)
    root = common / 'codex-repairs'
    root.mkdir(mode=0o700, exist_ok=True); descriptor = ordinary_dir(root); os.close(descriptor)
    own = root / 'leveldb-final-crates-check033'
    own.mkdir(mode=0o700, exist_ok=True); descriptor = ordinary_dir(own); os.close(descriptor)
    return own


def run_root033(repo):
    own = metadata_root033(repo)
    descriptor = os.open(own / 'run.lock', os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW | os.O_NONBLOCK, 0o600)
    try:
        info = os.fstat(descriptor)
        if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid():
            raise RuntimeError('Runner lock is nonordinary or foreign-owned.')
        fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        cache = own / ('cargo-target-' + digest033(str(repo).encode())[:16])
        cache.mkdir(mode=0o700, exist_ok=True); fd = ordinary_dir(cache); os.close(fd)
        return own, cache, descriptor
    except BaseException:
        os.close(descriptor); raise


def worker_output033(repo, path):
    _, common = repository_dirs033(repo); path = Path(path)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Runner worker path must be absolute.')
    relative = path.relative_to(common / 'codex-repairs')
    if (len(relative.parts) not in {4, 5} or relative.parts[0] not in
            {'leveldb-final-crates-check033', 'leveldb-final-crates-runner033'} or relative.parts[1] != 'runs'
            or re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', relative.parts[2]) is None
            or len(relative.parts) == 5 and relative.parts[3] != 'verification'
            or re.fullmatch(r'[A-Za-z0-9_.-]+\.json', path.name) is None):
        raise RuntimeError('Runner worker output differs from its owned run scope.')
    descriptor = ordinary_dir(path.parent); os.close(descriptor)


def capture_worker033(repo, output):
    worker_output033(repo, output)
    if os.path.lexists(output): raise RuntimeError('Runner snapshot output already exists.')
    atomic_json033(output, capture_inputs033(repo)); print('FINAL_CRATES_INPUTS_CAPTURED', flush=True)
    return 0


def verify_inputs_worker033(repo, baseline_path, output):
    for path in (baseline_path, output): worker_output033(repo, path)
    if baseline_path.parent != output.parent or baseline_path == output or os.path.lexists(output):
        raise RuntimeError('Input verification must create a new proof beside its retained baseline.')
    baseline_fp, raw = read_file033(baseline_path, 8 * MIB033)
    baseline = json.loads(raw)
    if encoded033(baseline) != raw:
        raise RuntimeError('Retained input baseline is not its canonical JSON record.')
    observed = capture_inputs033(repo); observed_raw = encoded033(observed)
    if read_file033(baseline_path, 8 * MIB033, content=False)[0] != baseline_fp:
        raise RuntimeError('Retained input baseline changed during verification.')
    same = observed_raw == raw
    proof = {'schema': 'bitcoinrs-working-code-input-verification033-v1', 'inputs_preserved': same,
        'baseline_file': str(baseline_path), 'baseline_fingerprint': baseline_fp,
        'canonical_sha256': digest033(observed_raw), 'input_count': observed.get('input_count'),
        'scope': 'Fresh complete input capture compared byte-for-byte with the retained canonical baseline.'}
    if not same:
        observed_path = output.with_name(output.stem + '.observed.json')
        worker_output033(repo, observed_path)
        if os.path.lexists(observed_path): raise RuntimeError('Changed-input evidence path is occupied.')
        atomic_json033(observed_path, observed)
        proof['observed_file'] = str(observed_path)
        proof['observed_fingerprint'] = read_file033(observed_path, 8 * MIB033, content=False)[0]
    atomic_json033(output, proof)
    print('FINAL_CRATES_INPUTS_UNCHANGED' if same else 'FINAL_CRATES_INPUTS_CHANGED', flush=True)
    return 0 if same else 1


def observer_worker033(repo, spec_path):
    worker_output033(repo, spec_path)
    spec = json.loads(read_file033(spec_path, MIB033)[1])
    if spec.get('cwd') != str(repo): raise RuntimeError('Cargo observer repository differs from its fixed worker argument.')
    worker_output033(repo, Path(spec['evidence_path']))
    return cargo_observer033(spec)


"""Immutable catalog for the four repaired LevelDB crate test surfaces.

Module import and all selectors operate only on supplied values. No repository,
file, subprocess, test executable or environment is read or changed here.
"""
from collections.abc import Mapping
from pathlib import Path
from types import MappingProxyType


def _freeze033(value):
    if isinstance(value, Mapping):
        return MappingProxyType({key: _freeze033(item) for key, item in value.items()})
    if isinstance(value, (tuple, list)):
        return tuple(_freeze033(item) for item in value)
    return value


COMMANDS033 = _freeze033({'randomized': ['cargo',
                'test',
                '--offline',
                '--locked',
                '--jobs',
                '1',
                '-p',
                'bitcoinleveldbt-randomized',
                '--no-run',
                '--message-format=json'],
 'issue320': ['cargo',
              'test',
              '--offline',
              '--locked',
              '--jobs',
              '1',
              '-p',
              'bitcoinleveldbt-issue320',
              '--lib',
              '--no-run',
              '--message-format=json'],
 'facade': ['cargo',
            'test',
            '--offline',
            '--locked',
            '--jobs',
            '1',
            '-p',
            'bitcoin-leveldb',
            '--lib',
            '--tests',
            '--no-run',
            '--message-format=json'],
 'harness': ['cargo',
             'test',
             '--offline',
             '--locked',
             '--jobs',
             '1',
             '-p',
             'bitcoinleveldb-harness',
             '--lib',
             '--tests',
             '--no-run',
             '--message-format=json']})

TARGETS033 = _freeze033({'randomized': {'package': 'bitcoinleveldbt-randomized',
                'artifacts': {'bitcoinleveldbt_randomized': ('lib', 'src/lib.rs'),
                              'execution_control_contracts': ('test', 'tests/execution_control_contracts.rs'),
                              'replay_contracts': ('test', 'tests/replay_contracts.rs'),
                              'bounded_contracts': ('test', 'tests/bounded_contracts.rs'),
                              'randomized': ('test', 'tests/randomized.rs'),
                              'bisect': ('test', 'tests/bisect.rs')}},
 'issue320': {'package': 'bitcoinleveldbt-issue320',
              'artifacts': {'bitcoinleveldbt_issue320': ('lib', 'src/lib.rs')}},
 'facade': {'package': 'bitcoin-leveldb', 'artifacts': {'bitcoin_leveldb': ('lib', 'src/lib.rs')}},
 'harness': {'package': 'bitcoinleveldb-harness',
             'artifacts': {'bitcoinleveldb_harness': ('lib', 'src/lib.rs'),
                           'execution_control_contracts': ('test', 'tests/execution_control_contracts.rs'),
                           'harness': ('test', 'tests/harness.rs')}}})

CASES033 = _freeze033([{'crate': 'randomized',
  'target': 'execution_control_contracts',
  'suffix': 'randomized_stress_limits_reject_unbounded_controls',
  'name': 'randomized_stress_limits_reject_unbounded_controls',
  'profile': 'pure',
  'ignored': False,
  'marker': 'RANDOMIZED_CONTROL_OK case=randomized_stress_limits_reject_unbounded_controls',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'execution_control_contracts',
  'suffix': 'randomized_stress_policy_rejects_wrong_seed_logging_and_data_paths',
  'name': 'randomized_stress_policy_rejects_wrong_seed_logging_and_data_paths',
  'profile': 'pure',
  'ignored': False,
  'marker': 'RANDOMIZED_CONTROL_OK case=randomized_stress_policy_rejects_wrong_seed_logging_and_data_paths',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'replay_contracts',
  'suffix': 'replay_matches_legacy_tape',
  'name': 'replay_matches_legacy_tape',
  'profile': 'replay',
  'ignored': False,
  'marker': 'REPLAY_CONTRACT_OK case=replay_matches_legacy_tape',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'replay_contracts',
  'suffix': 'replay_preserves_configuration_offsets',
  'name': 'replay_preserves_configuration_offsets',
  'profile': 'replay',
  'ignored': False,
  'marker': 'REPLAY_CONTRACT_OK case=replay_preserves_configuration_offsets',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'replay_contracts',
  'suffix': 'replay_preserves_short_prefixes',
  'name': 'replay_preserves_short_prefixes',
  'profile': 'replay',
  'ignored': False,
  'marker': 'REPLAY_CONTRACT_OK case=replay_preserves_short_prefixes',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'replay_contracts',
  'suffix': 'replay_covers_batch_order',
  'name': 'replay_covers_batch_order',
  'profile': 'replay',
  'ignored': False,
  'marker': 'REPLAY_CONTRACT_OK case=replay_covers_batch_order',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'bounded_contracts',
  'suffix': 'bounded_default',
  'name': 'bounded_default',
  'profile': 'bounded',
  'ignored': False,
  'marker': 'BOUNDED_RANDOMIZED_OK config=0 steps=34',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'bounded_contracts',
  'suffix': 'bounded_reuse',
  'name': 'bounded_reuse',
  'profile': 'bounded',
  'ignored': False,
  'marker': 'BOUNDED_RANDOMIZED_OK config=1 steps=34',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'bounded_contracts',
  'suffix': 'bounded_filter',
  'name': 'bounded_filter',
  'profile': 'bounded',
  'ignored': False,
  'marker': 'BOUNDED_RANDOMIZED_OK config=2 steps=34',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'bounded_contracts',
  'suffix': 'bounded_uncompressed',
  'name': 'bounded_uncompressed',
  'profile': 'bounded',
  'ignored': False,
  'marker': 'BOUNDED_RANDOMIZED_OK config=3 steps=34',
  'seed': 301},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_execution_profile_contract',
  'name': 'bitcoinleveldbt_issue320::issue320_execution_profile_contract',
  'profile': 'pure',
  'ignored': False,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_guard_predicate_rejects_unbounded_and_invalid_controls',
  'name': 'bitcoinleveldbt_issue320::issue320_guard_predicate_rejects_unbounded_and_invalid_controls',
  'profile': 'pure',
  'ignored': False,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_bounded_diagnostic',
  'name': 'bitcoinleveldbt_issue320::issue320_bounded_diagnostic',
  'profile': 'bounded',
  'ignored': True,
  'marker': 'ISSUE320_CASE_OK case=issue320_bounded_diagnostic',
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_control_overwrite_only_without_snapshots_matches_model_after_forced_compactions',
  'name': 'bitcoinleveldbt_issue320::issue320_isolation_tests::issue320_control_overwrite_only_without_snapshots_matches_model_after_forced_compactions',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_forced_compactions',
  'name': 'bitcoinleveldbt_issue320::issue320_fanout_reducer_tests::issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_forced_compactions',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal',
  'name': 'bitcoinleveldbt_issue320::issue320_fanout_reducer_tests::issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_forced_compactions',
  'name': 'bitcoinleveldbt_issue320::issue320_fanout_reducer_tests::issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_forced_compactions',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_reopen_with_verified_nonempty_wal',
  'name': 'bitcoinleveldbt_issue320::issue320_fanout_reducer_tests::issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_reopen_with_verified_nonempty_wal',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_forced_compactions',
  'name': 'bitcoinleveldbt_issue320::issue320_fanout_reducer_tests::issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_forced_compactions',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal',
  'name': 'bitcoinleveldbt_issue320::issue320_fanout_reducer_tests::issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_overwrite_only_with_live_snapshots_matches_model_after_forced_compactions',
  'name': 'bitcoinleveldbt_issue320::issue320_isolation_tests::issue320_overwrite_only_with_live_snapshots_matches_model_after_forced_compactions',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_overwrite_only_with_live_snapshots_matches_model_after_reopen',
  'name': 'bitcoinleveldbt_issue320::issue320_isolation_tests::issue320_overwrite_only_with_live_snapshots_matches_model_after_reopen',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_probe_single_key_with_live_snapshot_manual_compaction_reports_retained_versions_on_failure',
  'name': 'bitcoinleveldbt_issue320::issue320_snapshot_probe_tests::issue320_probe_single_key_with_live_snapshot_manual_compaction_reports_retained_versions_on_failure',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_probe_single_key_with_live_snapshot_reopen_reports_before_and_after_state_compactly',
  'name': 'bitcoinleveldbt_issue320::issue320_snapshot_probe_tests::issue320_probe_single_key_with_live_snapshot_reopen_reports_before_and_after_state_compactly',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_probe_single_key_with_live_snapshot_survives_flush_without_manual_compaction',
  'name': 'bitcoinleveldbt_issue320::issue320_snapshot_probe_tests::issue320_probe_single_key_with_live_snapshot_survives_flush_without_manual_compaction',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_probe_single_key_without_snapshot_keeps_only_newest_version_after_manual_compaction',
  'name': 'bitcoinleveldbt_issue320::issue320_snapshot_probe_tests::issue320_probe_single_key_without_snapshot_keeps_only_newest_version_after_manual_compaction',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_reducer_two_keys_one_snapshot_matches_model_after_forced_compactions',
  'name': 'bitcoinleveldbt_issue320::issue320_two_key_reducer_tests::issue320_reducer_two_keys_one_snapshot_matches_model_after_forced_compactions',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_reducer_two_keys_one_snapshot_matches_model_after_reopen_with_wal_tail',
  'name': 'bitcoinleveldbt_issue320::issue320_two_key_reducer_tests::issue320_reducer_two_keys_one_snapshot_matches_model_after_reopen_with_wal_tail',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_reducer_two_keys_two_snapshots_matches_model_after_forced_compactions',
  'name': 'bitcoinleveldbt_issue320::issue320_two_key_reducer_tests::issue320_reducer_two_keys_two_snapshots_matches_model_after_forced_compactions',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_reducer_two_keys_two_snapshots_matches_model_after_reopen_with_wal_tail',
  'name': 'bitcoinleveldbt_issue320::issue320_two_key_reducer_tests::issue320_reducer_two_keys_two_snapshots_matches_model_after_reopen_with_wal_tail',
  'profile': 'case',
  'ignored': True,
  'marker': None,
  'seed': 0},
 {'crate': 'harness',
  'target': 'execution_control_contracts',
  'suffix': 'admission_accepts_complete_controls',
  'name': 'admission_accepts_complete_controls',
  'profile': 'pure',
  'ignored': False,
  'marker': 'HARNESS_CONTROL_CONTRACT_OK case=admission_accepts_complete_controls',
  'seed': 301},
 {'crate': 'harness',
  'target': 'execution_control_contracts',
  'suffix': 'admission_rejects_missing_or_weakened_controls',
  'name': 'admission_rejects_missing_or_weakened_controls',
  'profile': 'pure',
  'ignored': False,
  'marker': 'HARNESS_CONTROL_CONTRACT_OK case=admission_rejects_missing_or_weakened_controls',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_empty',
  'name': 'harness_empty',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_empty',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_zero_restart_points_in_block',
  'name': 'harness_zero_restart_points_in_block',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_zero_restart_points_in_block',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_simple_empty_key',
  'name': 'harness_simple_empty_key',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_simple_empty_key',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_simple_single',
  'name': 'harness_simple_single',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_simple_single',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_simple_multi',
  'name': 'harness_simple_multi',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_simple_multi',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_simple_special_key',
  'name': 'harness_simple_special_key',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_simple_special_key',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_db_pointer_is_null_when_not_db_test',
  'name': 'harness_db_pointer_is_null_when_not_db_test',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_db_pointer_is_null_when_not_db_test',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_reinitialization_switches_constructor_types_without_panicking',
  'name': 'harness_reinitialization_switches_constructor_types_without_panicking',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_reinitialization_switches_constructor_types_without_panicking',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_duplicate_keys_overwrite_previous_value',
  'name': 'harness_duplicate_keys_overwrite_previous_value',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_duplicate_keys_overwrite_previous_value',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_handles_embedded_nul_and_empty_values',
  'name': 'harness_handles_embedded_nul_and_empty_values',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_handles_embedded_nul_and_empty_values',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_pick_random_key_returns_foo_on_empty_keyset',
  'name': 'harness_pick_random_key_returns_foo_on_empty_keyset',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_pick_random_key_returns_foo_on_empty_keyset',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_to_string_formats_iterator_entries_and_end_sentinel',
  'name': 'harness_to_string_formats_iterator_entries_and_end_sentinel',
  'profile': 'case',
  'ignored': False,
  'marker': 'HARNESS_CONTRACT_OK case=harness_to_string_formats_iterator_entries_and_end_sentinel',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'randomized',
  'suffix': 'db_test_randomized',
  'name': 'db_test_randomized',
  'profile': 'full_original',
  'ignored': True,
  'marker': 'RANDOMIZED_STRESS_OK case=db_test_randomized configurations=4 steps=40000 seed=301',
  'seed': 301},
 {'crate': 'randomized',
  'target': 'bisect',
  'suffix': 'db_test_bisect_randomized',
  'name': 'db_test_bisect_randomized',
  'profile': 'full_original',
  'ignored': True,
  'marker': 'RANDOMIZED_STRESS_OK case=db_test_bisect_randomized configurations=4 steps=40000 seed=301',
  'seed': 301},
 {'crate': 'issue320',
  'target': 'bitcoinleveldbt_issue320',
  'suffix': 'issue320_test',
  'name': 'bitcoinleveldbt_issue320::issue320_test',
  'profile': 'full_original',
  'ignored': True,
  'marker': 'ISSUE320_CASE_OK case=issue320_test',
  'seed': 0},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_randomized',
  'name': 'harness_randomized',
  'profile': 'full_original',
  'ignored': True,
  'marker': 'HARNESS_CONTRACT_OK case=harness_randomized',
  'seed': 301},
 {'crate': 'harness',
  'target': 'harness',
  'suffix': 'harness_randomized_longdb',
  'name': 'harness_randomized_longdb',
  'profile': 'full_original',
  'ignored': True,
  'marker': 'HARNESS_CONTRACT_OK case=harness_randomized_longdb',
  'seed': 301}])

ALIASES033 = tuple(TARGETS033)
GATE_VARIABLES033 = frozenset((
    'BITCOIN_RS_RANDOMIZED_GUARD', 'BITCOIN_RS_RANDOMIZED_STRESS_GUARD',
    'BITCOIN_RS_RANDOMIZED_CASE_DATA', 'BITCOIN_RS_ISSUE320_GUARD',
    'BITCOIN_RS_HARNESS031_PROTECTED', 'BITCOIN_RS_HARNESS031_CASE',
    'BITCOIN_RS_HARNESS031_CASE_DATA',
))
_CASE_INDEX033 = MappingProxyType({(item['crate'], item['target'], item['name']): item for item in CASES033})


def select_crates033(selectors=None):
    """Map repeated public all/exact-package selectors to canonical aliases."""
    if selectors is None: selectors = ('all',)
    elif isinstance(selectors, str): selectors = (selectors,)
    elif not isinstance(selectors, (list, tuple)):
        raise ValueError('Crate selectors must be all or exact supported package names.')
    if not selectors: selectors = ('all',)
    packages = {row['package']: alias for alias, row in TARGETS033.items()}
    selected = set()
    for value in selectors:
        if not isinstance(value, str) or value not in ('all', *packages):
            raise ValueError('Unknown crate selector; use all or an exact supported package name.')
        if value == 'all': selected.update(ALIASES033)
        else: selected.add(packages[value])
    return tuple(alias for alias in ALIASES033 if alias in selected)


def _selection033(selection):
    if (not isinstance(selection, (tuple, list)) or not selection
            or any(not isinstance(alias, str) or alias not in TARGETS033 for alias in selection)):
        raise ValueError('Catalog selection must contain supported canonical crate aliases.')
    return tuple(alias for alias in ALIASES033 if alias in selection)


def selected_cases033(selection, suite='quick'):
    """Return a frozen selected suite; full originals are globally last."""
    aliases = _selection033(selection)
    if not isinstance(suite, str) or suite not in ('quick', 'full'):
        raise ValueError('Suite must be quick or full.')
    return tuple(item for item in CASES033 if item['crate'] in aliases
                 and (suite == 'full' or item['profile'] != 'full_original'))


def expectedlists033(selection):
    """Complete all/ignored listings, including stress cases in quick mode."""
    aliases = _selection033(selection)
    all_names, ignored_names = {}, {}
    for alias in aliases:
        package = TARGETS033[alias]['package']
        for target in TARGETS033[alias]['artifacts']:
            key = package + '::' + target
            items = [item for item in CASES033 if item['crate'] == alias and item['target'] == target]
            all_names[key] = tuple(sorted(item['name'] for item in items))
            ignored_names[key] = tuple(sorted(item['name'] for item in items if item['ignored']))
    return _freeze033({'all': all_names, 'ignored': ignored_names})


def _case033(item):
    if not isinstance(item, Mapping):
        raise ValueError('Selected case must be bound to the immutable repaired-crate catalog.')
    identity = (item.get('crate'), item.get('target'), item.get('name'))
    if any(type(value) is not str for value in identity):
        raise ValueError('Selected case identity must contain exact catalog strings.')
    expected = _CASE_INDEX033.get(identity)
    if expected is None or any(key not in item or type(item[key]) is not type(value) or item[key] != value
                               for key, value in expected.items()):
        raise ValueError('Selected case metadata differs from the immutable repaired-crate catalog.')
    return expected


def _path033(value, description):
    if not isinstance(value, (str, Path)):
        raise ValueError(description + ' must be an absolute ordinary literal path.')
    text = str(value)
    if not text or len(text) > 4096 or '\0' in text:
        raise ValueError(description + ' must be an absolute ordinary literal path.')
    path = Path(text)
    if not path.is_absolute() or '..' in path.parts or str(path) != text:
        raise ValueError(description + ' must be an absolute ordinary literal path.')
    return path


def case_command033(executable, item):
    selected = _case033(item); path = _path033(executable, 'Test executable')
    command = (str(path), '--exact', selected['name'])
    if selected['ignored']: command += ('--ignored',)
    return command + ('--nocapture', '--test-threads=1')


def case_environment033(item, data):
    selected = _case033(item); path = _path033(data, 'Case data root')
    if path.name != 'case-data-' + selected['suffix']:
        raise ValueError('Case data root basename must be case-data- plus the exact case suffix.')
    root = str(path)
    env = {'RUST_LOG': 'off', 'RUST_BACKTRACE': '0', 'TEST_RANDOM_SEED': str(selected['seed']),
           'TMPDIR': root, 'TMP': root, 'TEMP': root, 'TEST_TMPDIR': root}
    if selected['crate'] == 'randomized':
        if selected['profile'] in {'replay', 'bounded'}:
            env['BITCOIN_RS_RANDOMIZED_GUARD'] = 'v1'
        elif selected['profile'] == 'full_original':
            env.update(BITCOIN_RS_RANDOMIZED_STRESS_GUARD='v1', BITCOIN_RS_RANDOMIZED_CASE_DATA=root)
    elif selected['crate'] == 'issue320' and selected['ignored']:
        env['BITCOIN_RS_ISSUE320_GUARD'] = 'v1'
    elif selected['crate'] == 'harness' and selected['profile'] != 'pure':
        env.update(BITCOIN_RS_HARNESS031_PROTECTED='1', BITCOIN_RS_HARNESS031_CASE=selected['suffix'],
                   BITCOIN_RS_HARNESS031_CASE_DATA=root)
    return _freeze033(env)


"""Bounded working-tree code fingerprints; no source contents or repository writes.

The embedding runner supplies ordinary_dir, read_file033, encoded033, digest033,
git033 and repository_dirs033. Git queries must disable optional index writes,
hooks/fsmonitor and configured filters. This collector never launches a build.
"""
import os
from pathlib import Path
import re
import stat
import time


_INPUT_EXTENSIONS033 = frozenset(('.rs', '.nix', '.c', '.cc', '.cpp', '.h', '.hpp', '.toml'))
_INPUT_CODE_PATHS033 = ('contrib/test/leveldb-final-crates.py', 'contrib/test/leveldb_resource_guard.py')
_INPUT_OPTIONAL_PATHS033 = ('Cargo.toml', 'Cargo.lock', 'rust-toolchain', 'rust-toolchain.toml',
                          '.cargo/config', '.cargo/config.toml')
_INPUT_MAX_FILES033 = 50000
_INPUT_MAX_FILE033 = 16 * 1024 * 1024
_INPUT_MAX_TOTAL033 = 256 * 1024 * 1024
_INPUT_MAX_JSON033 = 8 * 1024 * 1024
_INPUT_SECONDS033 = 180
_INPUT_MAX_ENTRIES033 = 250000
_INPUT_MAX_DIRECTORIES033 = 50000


def _input_deadline033(deadline):
    if time.monotonic() >= deadline:
        raise RuntimeError('Working-tree input fingerprinting reached its 180-second allowance.')


def _input_relative033(value):
    if (not isinstance(value, str) or not value or len(value.encode('utf-8')) > 4096
            or any(ord(char) < 32 or ord(char) == 127 for char in value)):
        raise RuntimeError('A selected code path is not bounded ordinary UTF-8 text.')
    path = Path(value)
    if (not path.parts or path.is_absolute() or '..' in path.parts
            or str(path) != value or path.parts[0] == '.git'):
        raise RuntimeError('A selected code path is outside the literal repository boundary.')
    return path


def _input_decode033(raw):
    try:
        value = raw.decode('utf-8')
    except UnicodeError as error:
        raise RuntimeError('Git returned a non-UTF-8 code path; no incomplete inventory was accepted.') from error
    _input_relative033(value)
    return value


def _input_fp033(value):
    return (isinstance(value, dict) and set(value) == {'kind', 'mode', 'size', 'sha256'}
            and value.get('kind') == 'file' and type(value.get('mode')) is int
            and 0 <= value['mode'] <= 0o7777 and type(value.get('size')) is int
            and 0 <= value['size'] <= _INPUT_MAX_FILE033 and isinstance(value.get('sha256'), str)
            and re.fullmatch(r'[0-9a-f]{64}', value['sha256']) is not None)


def _input_optional033(path):
    try:
        fp, _ = read_file033(path, _INPUT_MAX_FILE033, content=False)
    except FileNotFoundError:
        return {'kind': 'absent'}
    if not _input_fp033(fp):
        raise RuntimeError('Repository metadata lacks an ordinary bounded fingerprint: ' + path.name)
    return fp


def _input_metadata033(repo, deadline):
    _input_deadline033(deadline)
    gitdir, common = map(Path, repository_dirs033(repo))
    for path in (repo, gitdir, common):
        fd = ordinary_dir(path)
        os.close(fd)
    # Read index before Git inspection as well as after it: optional Git index
    # refreshes are not an authorized side effect of input observation.
    index_before = _input_optional033(gitdir / 'index')
    if git033(repo, ['config', '--name-only', '--get-regexp', r'^filter\.'], allowed=(0, 1)).strip():
        raise RuntimeError('Configured Git filters require review before input status inspection.')
    values = {}
    for key, args in (
        ('head', ['rev-parse', '--verify', 'HEAD']),
        ('tree', ['rev-parse', '--verify', 'HEAD^{tree}']),
        ('head_reference', ['rev-parse', '--symbolic-full-name', 'HEAD'])):
        _input_deadline033(deadline)
        raw = git033(repo, args)
        try:
            value = raw.decode('utf-8').strip()
        except UnicodeError as error:
            raise RuntimeError('Repository identity is not ordinary UTF-8 text.') from error
        if (key in {'head', 'tree'} and re.fullmatch(r'(?:[0-9a-f]{40}|[0-9a-f]{64})', value) is None
                or key == 'head_reference' and (not value or len(value) > 4096
                    or any(ord(char) < 32 or ord(char) == 127 for char in value)
                    or value != 'HEAD' and not value.startswith('refs/'))):
            raise RuntimeError('Repository HEAD/tree/reference identity is absent or malformed.')
        values[key] = value
    if len(values['head']) != len(values['tree']):
        raise RuntimeError('Repository HEAD/tree object formats differ.')
    _input_deadline033(deadline)
    status = git033(repo, ['status', '--porcelain=v1', '-z', '--untracked-files=all'])
    index_after = _input_optional033(gitdir / 'index')
    if index_after != index_before:
        raise RuntimeError('Repository index changed during read-only metadata inspection.')
    values.update(git_directory=str(gitdir), common_directory=str(common), index_fingerprint=index_after,
        head_file_fingerprint=_input_optional033(gitdir / 'HEAD'),
        status_sha256=digest033(status), status_bytes=len(status), preexisting_dirty=bool(status))
    _input_deadline033(deadline)
    return values


def _input_inventory033(repo, deadline):
    commands = (
        ('tracked', ['ls-files', '--stage', '-z']),
        ('untracked', ['ls-files', '--others', '--exclude-standard', '-z']),
        ('ignored', ['ls-files', '--others', '--ignored', '--exclude-standard', '-z']))
    categories, summaries = {}, {}
    for kind, args in commands:
        _input_deadline033(deadline)
        raw = git033(repo, args)
        if raw and not raw.endswith(b'\0'):
            raise RuntimeError('Git input inventory is not completely NUL-framed.')
        rows = raw.split(b'\0')[:-1] if raw else []
        if len(rows) > 3 * _INPUT_MAX_ENTRIES033:
            raise RuntimeError('Git input inventory exceeds its finite file-count allowance.')
        names = set()
        for row in rows:
            if kind == 'tracked':
                header, separator, name = row.partition(b'\t')
                fields = header.split()
                if (separator != b'\t' or len(fields) != 3
                        or fields[0] not in {b'100644', b'100755', b'120000', b'160000'}
                        or re.fullmatch(rb'(?:[0-9a-f]{40}|[0-9a-f]{64})', fields[1]) is None
                        or fields[2] not in {b'0', b'1', b'2', b'3'}):
                    raise RuntimeError('Git tracked input index entries are malformed.')
                if fields[0] == b'160000':
                    raise RuntimeError('Gitlinks require a separate source boundary; no opaque nested repository was accepted.')
            else:
                name = row
                if name in names:
                    raise RuntimeError('Git input inventory repeats an untracked or ignored path.')
            if name.endswith(b'/'):
                raise RuntimeError('Git returned an opaque directory instead of enumerable repository files.')
            names.add(name)
        categories[kind] = names
        summaries[kind] = {'sha256': digest033(raw), 'bytes': len(raw), 'entries': len(rows), 'paths': len(names)}
    if (categories['tracked'] & categories['untracked'] or categories['tracked'] & categories['ignored']
            or categories['untracked'] & categories['ignored']):
        raise RuntimeError('Git tracked/untracked/ignored input classifications changed during collection.')
    if len(set.union(*categories.values())) > _INPUT_MAX_ENTRIES033:
        raise RuntimeError('Git input inventory exceeds its 250000-entry allowance.')
    _input_deadline033(deadline)
    return categories, summaries


def _input_selected033(name):
    path = Path(name)
    return (path.suffix in _INPUT_EXTENSIONS033 or path.name in {'Cargo.lock', 'rust-toolchain'}
            or path.parent.name == '.cargo' and path.name in {'config', 'config.toml'})


def _input_filesystem033(repo, code_paths, deadline):
    """Cross-check only eligible names/types; Git omits untracked FIFOs.

    Ordinary directories are enumerated without following links. Nonsource
    file contents/attributes are not read; only eligible files are stat-checked.
    The root.git namespace alone is excluded, so ignored target sources remain
    part of the declared source scope.
    """
    pending, selected, entries, directories = [''], set(), 0, 0
    while pending:
        _input_deadline033(deadline)
        relative = pending.pop()
        directories += 1
        if directories > _INPUT_MAX_DIRECTORIES033:
            raise RuntimeError('Source directory cross-check exceeds its 50000-directory allowance.')
        fd = ordinary_dir(repo / relative)
        try:
            with os.scandir(fd) as scan:
                for entry in scan:
                    _input_deadline033(deadline)
                    if not relative and entry.name == '.git':
                        continue
                    entries += 1
                    if entries > _INPUT_MAX_ENTRIES033:
                        raise RuntimeError('Source directory cross-check exceeds its 250000-entry allowance.')
                    name = str(Path(relative) / entry.name) if relative else entry.name
                    eligible = _input_selected033(name) or name in code_paths or name in _INPUT_OPTIONAL_PATHS033
                    directory = entry.is_dir(follow_symlinks=False)
                    if eligible:
                        _input_relative033(name)
                        if not stat.S_ISREG(entry.stat(follow_symlinks=False).st_mode):
                            raise RuntimeError('Selected code input must be an ordinary nonsymlink file: ' + name)
                        selected.add(name)
                        if len(selected) > _INPUT_MAX_FILES033:
                            raise RuntimeError('Selected code inputs exceed the 50000-file allowance.')
                    if directory:
                        pending.append(name)
        finally:
            os.close(fd)
    return selected, {'selected_paths': len(selected), 'selected_paths_sha256': digest033(encoded033(sorted(selected))),
                      'directory_entries_scanned': entries, 'directories_scanned': directories}


def _input_stamp033(info):
    return tuple(getattr(info, key) for key in ('st_dev', 'st_ino', 'st_mode', 'st_uid', 'st_size', 'st_mtime_ns', 'st_ctime_ns'))


def _input_read033(repo, name, may_be_absent):
    relative = _input_relative033(name)
    descriptor = ordinary_dir(repo)
    try:
        for part in relative.parts[:-1]:
            try:
                child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=descriptor)
            except FileNotFoundError:
                if may_be_absent: return {'kind': 'absent'}
                raise RuntimeError('An enumerated code input disappeared during collection: ' + name)
            os.close(descriptor)
            descriptor = child
        try:
            info = os.stat(relative.name, dir_fd=descriptor, follow_symlinks=False)
        except FileNotFoundError:
            if may_be_absent: return {'kind': 'absent'}
            raise RuntimeError('An enumerated code input disappeared during collection: ' + name)
        if not stat.S_ISREG(info.st_mode):
            raise RuntimeError('Selected code input must be an ordinary nonsymlink file: ' + name)
        if info.st_size > _INPUT_MAX_FILE033:
            raise RuntimeError('Selected code input exceeds its 16 MiB per-file allowance: ' + name)
        fp, _ = read_file033(repo / relative, _INPUT_MAX_FILE033, content=False)
        current = os.stat(relative.name, dir_fd=descriptor, follow_symlinks=False)
        if (_input_stamp033(info) != _input_stamp033(current) or not _input_fp033(fp)
                or fp['mode'] != stat.S_IMODE(info.st_mode) or fp['size'] != info.st_size):
            raise RuntimeError('Selected code input changed during fingerprinting: ' + name)
        return fp
    finally:
        os.close(descriptor)


def capture_inputs033(repo):
    """Return a complete bounded fingerprint list for the declared code scope.

    Preexisting staged/unstaged/untracked/ignored code is accepted and hashed
    as it exists. Tracked deletions and optional standard-config absences are
    explicit. The caller stores this no-content record and compares subsequent
    captures exactly; it must not treat mtime or a prior successful run as reuse.
    """
    repo = Path(repo)
    if not repo.is_absolute() or '..' in repo.parts:
        raise RuntimeError('Working-tree input capture requires an absolute ordinary repository.')
    code_paths = globals().get('INPUT_CODE_PATHS033', _INPUT_CODE_PATHS033)
    if (not isinstance(code_paths, (list, tuple)) or tuple(code_paths) != _INPUT_CODE_PATHS033):
        raise RuntimeError('The runner and guard code-input paths differ from the exact declared pair.')
    deadline = time.monotonic() + _INPUT_SECONDS033
    before = _input_metadata033(repo, deadline)
    categories, inventory_before = _input_inventory033(repo, deadline)
    selected, tracked = set(), set()
    for kind, names in categories.items():
        for raw_name in names:
            # Only selected input names enter the returned record. Even opaque
            # unrelated names remain bound by the raw Git inventory digests.
            try:
                name = raw_name.decode('utf-8')
            except UnicodeError:
                parts = raw_name.split(b'/')
                if (any(raw_name.endswith(suffix.encode()) for suffix in _INPUT_EXTENSIONS033)
                        or parts[-1] in {b'Cargo.lock', b'rust-toolchain'}
                        or len(parts) >= 2 and parts[-2] == b'.cargo' and parts[-1] in {b'config', b'config.toml'}):
                    _input_decode033(raw_name)
                continue
            if _input_selected033(name):
                selected.add(_input_decode033(raw_name))
                if kind == 'tracked': tracked.add(name)
    selected.update(_INPUT_OPTIONAL_PATHS033)
    selected.update(code_paths)
    if len(selected) > _INPUT_MAX_FILES033:
        raise RuntimeError('Selected code inputs exceed the 50000-file allowance.')
    filesystem_names, filesystem_before = _input_filesystem033(repo, code_paths, deadline)
    if filesystem_names - selected:
        raise RuntimeError('Git omitted an eligible working-tree source path; no partial code inventory was accepted.')
    files, total = {}, 0
    for name in sorted(selected):
        _input_deadline033(deadline)
        fp = _input_read033(repo, name, name in tracked or name in _INPUT_OPTIONAL_PATHS033)
        if name in code_paths and fp.get('kind') != 'file':
            raise RuntimeError('The runner and guard must both be ordinary present code inputs.')
        total += fp.get('size', 0)
        if total > _INPUT_MAX_TOTAL033:
            raise RuntimeError('Working-tree code inputs exceed their 256 MiB aggregate allowance.')
        files[name] = fp
    # Porcelain remains " M" when an already-dirty file changes again. Repeat
    # actual fingerprints as well as Git/name metadata: mtime caching or status
    # alone would miss a persistent edit made after an earlier file was read.
    # This is bounded consistency observation, not an atomic filesystem snapshot.
    for name, expected in files.items():
        _input_deadline033(deadline)
        current = _input_read033(repo, name, name in tracked or name in _INPUT_OPTIONAL_PATHS033)
        if current != expected:
            raise RuntimeError('Selected code input changed between bounded fingerprint passes: ' + name)
    _, filesystem_after = _input_filesystem033(repo, code_paths, deadline)
    _, inventory_after = _input_inventory033(repo, deadline)
    after = _input_metadata033(repo, deadline)
    if before != after or inventory_before != inventory_after or filesystem_before != filesystem_after:
        raise RuntimeError('Git HEAD/tree/index/status or source-file inventory changed during input capture.')
    result = {'schema': 'bitcoinrs-working-code-inputs033-v1', 'repository': str(repo),
        'metadata': before, 'git_inventory': inventory_before, 'filesystem_inventory': filesystem_before, 'files': files,
        'input_count': len(files), 'present_count': sum(fp.get('kind') == 'file' for fp in files.values()),
        'absent_count': sum(fp.get('kind') == 'absent' for fp in files.values()), 'input_bytes': total,
        'files_sha256': digest033(encoded033(files)),
        'scope': {'extensions': sorted(_INPUT_EXTENSIONS033), 'mandatory_code_paths': list(code_paths),
            'optional_root_paths': list(_INPUT_OPTIONAL_PATHS033),
            'additional_names': ['Cargo.lock', 'rust-toolchain', '.cargo/config', '.cargo/config.toml'],
            'git_categories': ['tracked', 'untracked', 'ignored'], 'source_contents_retained': False,
            'external_source_paths_followed': False,
            'filesystem_crosscheck': {'entry_limit': _INPUT_MAX_ENTRIES033, 'directory_limit': _INPUT_MAX_DIRECTORIES033,
                                     'skipped_namespace': '.git', 'eligible_special_files_refused': True},
            'limit': 'Declared repository code/config suffixes and names; arbitrary build-script assets and external toolchain/cache/config contents are outside this input record.'}}
    if len(encoded033(result)) > _INPUT_MAX_JSON033:
        raise RuntimeError('Working-tree input fingerprints exceed their 8 MiB JSON allowance; no partial capture was accepted.')
    _input_deadline033(deadline)
    return result


"""Finite Cargo observation for the four ordinary final-crate runner commands.

The parent must launch this observer under the reviewed resource guard. This
module never builds unless cargo_observer033 is explicitly invoked with one of four exact
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
# The catalog is embedded before this component. Freeze its reviewed values;
# caller-visible maps cannot subsequently widen a running observer's grammar.
ALLOWED_PACKAGES = tuple(metadata["package"] for metadata in TARGETS033.values())
_COMMAND_SHAPES033 = tuple((label, tuple(command)) for label, command in COMMANDS033.items())
_TARGET_SHAPES033 = {label: tuple(metadata["artifacts"].items()) for label, metadata in TARGETS033.items()}
_PACKAGE_SHAPES033 = {label: metadata["package"] for label, metadata in TARGETS033.items()}



MAX_EXECUTABLES = 40


def _observer_context033(spec):
    required = {"command", "cwd", "target_directory", "evidence_path", "guard_command", "run_directory", "allowed_run_root"}
    if not isinstance(spec, dict) or not required.issubset(spec) or set(spec) - required - {"crate", "expected_packages"}:
        raise RuntimeError("Cargo observer specification differs from its seven-field current-run context.")
    command = spec.get("command")
    if not isinstance(command, list):
        raise RuntimeError("Cargo observer command must be the exact reviewed argument list.")
    matches = [label for label, shape in _COMMAND_SHAPES033 if tuple(command) == shape]
    if len(matches) != 1:
        raise RuntimeError("Cargo observer requires one of the four exact reviewed final-crate no-run commands.")
    label = matches[0]
    expected = [_PACKAGE_SHAPES033[label]]
    if command[7] != expected[0] or spec.get("crate", label) != label:
        raise RuntimeError("Cargo crate alias/package differs from the exact matched command.")
    if spec.get("expected_packages", expected) != expected:
        raise RuntimeError("Cargo expected package must be the exact derived singleton.")
    guard_command = spec.get("guard_command")
    if (not isinstance(guard_command, list) or not guard_command or len(guard_command) > 64
            or any(not isinstance(value, str) or "\x00" in value or len(value) > 65536 for value in guard_command)
            or not Path(guard_command[0]).is_absolute()):
        raise RuntimeError("Cargo outer guard command is not an exact absolute current-run argument list.")
    paths = {}
    for key in ("cwd", "target_directory", "evidence_path", "run_directory", "allowed_run_root"):
        value = spec.get(key)
        if not isinstance(value, str) or not value or len(value) > 4096 or "\x00" in value:
            raise RuntimeError("Cargo observer path is absent or malformed: " + key)
        path = Path(value)
        if not path.is_absolute() or ".." in path.parts or str(path) != value:
            raise RuntimeError("Cargo observer requires a canonical absolute path: " + key)
        paths[key] = path
    owner, run, target, evidence = (paths[key] for key in ("allowed_run_root", "run_directory", "target_directory", "evidence_path"))
    try:
        run_relative = run.relative_to(owner).parts
        target_relative = target.relative_to(owner).parts
    except ValueError as error:
        raise RuntimeError("Cargo run/cache is outside its explicit allowed metadata owner.") from error
    if (owner.parent.name != "codex-repairs" or len(run_relative) not in (2, 3) or run_relative[0] != "runs"
            or not re.fullmatch(r"[A-Za-z0-9_.-]{1,128}", run_relative[1])
            or (len(run_relative) == 3 and run_relative[2] != "verification")
            or not target_relative or evidence.parent != run or evidence.suffix != ".json"
            or not re.fullmatch(r"[A-Za-z0-9_.-]{1,128}", evidence.name)):
        raise RuntimeError("Cargo current evidence/run/cache layout differs from its finite owned scope.")
    for path in (paths["cwd"], owner, run, target):
        descriptor = ordinary_dir(path)
        os.close(descriptor)
    paths["expected_packages"] = tuple(expected)
    paths["alias"] = label
    paths["expected_artifacts"] = {
        expected[0] + "::" + name: {"kind": kind, "source": paths["cwd"] / expected[0] / source}
        for name, (kind, source) in _TARGET_SHAPES033[label]
    }
    return list(command), expected, paths


def _observer_spec(spec):
    command, expected, paths = _observer_context033(spec)
    if os.path.lexists(paths["evidence_path"]):
        raise RuntimeError("Cargo observer refuses to replace existing current-run evidence.")
    return command, expected, paths



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
        raise RuntimeError("Cargo test source is outside the selected working tree.") from error
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


def cargo_observer033(spec):
    """Run only the fixed offline compilation command and retain small evidence.

    Return 0 only for a complete successful target inventory; return 101 for a
    valid Cargo101 diagnostic failure, preserving any valid partial artifacts.
    On malformed output or an output allowance violation, stop Cargo promptly.
    """
    evidence = {
        "schema": "bitcoinrs-final-crates033-cargo-observer-v1", "command": None,
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
        sys.stderr.write("FINAL033_CARGO_OBSERVER_REJECTED " + (evidence["error"] or "invalid specification") + "\n")
        return 2
    try:
        _atomic_evidence(paths["evidence_path"], evidence)
    except Exception as error:
        sys.stderr.write("FINAL033_CARGO_EVIDENCE_ERROR " + str(error)[:1024] + "\n")
        return 3
    print("FINAL033_CARGO_OBSERVER records=" + str(evidence["records"])
          + " executables=" + str(len(evidence["artifacts"]))
          + " native_exit=" + str(evidence["cargo_exit_code"])
          + " observer_exit=" + str(evidence["observer_exit_code"]), flush=True)
    return evidence["observer_exit_code"]


"""Current-working-tree compiler and exact native-case qualification; read only."""
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import stat
import sys
from urllib.parse import unquote, urlsplit


def assess_case033(result, log, expected_name, marker=None):
    assessment = {"outcome": "failed", "expected_name": expected_name,
                  "passed": None, "failed": None, "ignored": None,
                  "marker": marker, "error": None}
    try:
        if (not isinstance(result, dict) or result.get("verified") is not True
                or result.get("outcome") != "exited"
                or type(result.get("exit_code")) is not int or result["exit_code"] != 0):
            raise RuntimeError("Selected case did not exit successfully under verified controls.")
        if not isinstance(log, str) or not isinstance(expected_name, str) or not expected_name or re.search(r"\s", expected_name):
            raise RuntimeError("Selected case log or expected name is malformed.")
        heads = re.findall(r"(?m)^test ([^\s]+) \.\.\.(?:[ \t]|$)", log)
        if heads != [expected_name]:
            raise RuntimeError("Selected case log does not contain exactly the requested test name.")
        summary_lines = [line for line in log.splitlines() if line.startswith("test result:")]
        if len(summary_lines) != 1:
            raise RuntimeError("Selected case log lacks one unambiguous test-result summary.")
        summary = re.fullmatch(r"test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in .+", summary_lines[0])
        if not summary:
            raise RuntimeError("Selected case summary is malformed.")
        passed, failed, ignored, measured = map(int, summary.group(2, 3, 4, 5))
        assessment.update(passed=passed, failed=failed, ignored=ignored)
        if summary.group(1) != "ok" or (passed, failed, ignored, measured) != (1, 0, 0, 0):
            raise RuntimeError("Selected case did not report exactly one passing non-ignored test.")
        if marker is not None:
            if not isinstance(marker, str) or not marker or "\n" in marker:
                raise RuntimeError("Required case marker is malformed.")
            count = len(re.findall(r"(?<!\S)" + re.escape(marker) + r"(?!\S)", log))
            if count != 1:
                raise RuntimeError("Selected case required marker is absent or repeated.")
        assessment["outcome"] = "passed"
    except Exception as error:
        assessment["error"] = str(error)[:1024]
    return assessment


def _equal033(left, right):
    return json.dumps(left, sort_keys=True, separators=(',', ':'), allow_nan=False) == json.dumps(right, sort_keys=True, separators=(',', ':'), allow_nan=False)


def _fingerprint033(value):
    return (isinstance(value, dict) and value.get('kind') == 'file'
            and type(value.get('mode')) is int and 0 <= value['mode'] <= 0o777 and bool(value['mode'] & 0o111)
            and type(value.get('size')) is int and 0 < value['size'] <= 256 * 1024 * 1024
            and isinstance(value.get('sha256'), str) and re.fullmatch(r'[0-9a-f]{64}', value['sha256']) is not None)


def _budget033(guard, kind):
    mib = 1024 * 1024
    caps = {'build': (4096*mib, 3072*mib, 128, 600, 512*mib, 2*mib),
            'probe': (128*mib, 96*mib, 32, 10, 16*mib, mib),
            'source': (512*mib, 384*mib, 32, 180, 32*mib, mib),
            'case': (512*mib, 384*mib, 64, 30, 16*mib, mib),
            'stress': (2048*mib, 1536*mib, 64, 600, 256*mib, mib)}
    if kind not in caps or not isinstance(getattr(guard, 'BUDGETS', None), dict) or kind not in guard.BUDGETS:
        raise RuntimeError('Runner033 lacks its finite guard budget kind.')
    budget = guard.BUDGETS[kind]
    keys = ('memory', 'high', 'tasks', 'seconds', 'file', 'output')
    if (not isinstance(budget, dict) or not set(keys).issubset(budget) or set(budget) - set(keys) - {'data', 'entries'}
            or any(type(budget[key]) is not int or not 0 < budget[key] <= cap for key, cap in zip(keys, caps[kind]))
            or budget['high'] > budget['memory']
            or any(type(budget[key]) is not int or not 0 < budget[key] <= maximum
                   for key, maximum in (('data', 512*mib), ('entries', 4096)) if key in budget)):
        raise RuntimeError('Runner033 guard budget is unbounded or exceeds its reviewed finite ceiling.')
    return dict(budget)


def _package_id033(value, expected_root):
    if not isinstance(value, str) or not value or len(value) > 4096 or '\x00' in value:
        raise RuntimeError('Runner033 executable package identity is malformed.')
    if value.startswith('path+'):
        parsed = urlsplit(value[5:]); path = Path(unquote(parsed.path))
        if parsed.scheme != 'file' or parsed.netloc not in ('', 'localhost') or '..' in path.parts or path != expected_root:
            raise RuntimeError('Runner033 executable package identity differs from its exact working-tree source.')
    elif '://' in value:
        raise RuntimeError('Runner033 local executable cannot have an external package identity.')


def qualify_compile033(guard, row, evidence, spec):
    """Read current observer JSON and return only its complete artifact bindings."""
    command, packages, paths = _observer_context033(spec)
    outer = spec['guard_command']
    if (len(outer) != 13 or Path(outer[0]).name != 'nix-shell'
            or outer[1:12] != ['--option', 'max-jobs', '0', '--option', 'builders', '',
                              '--option', 'substitute', 'false', str(paths['cwd'] / 'shell.nix'), '--run']):
        raise RuntimeError('Runner033 compiler outer command differs from the exact offline Nix grammar.')
    try:
        inner = shlex.split(outer[-1])
    except ValueError as error:
        raise RuntimeError('Runner033 compiler observer shell command is malformed.') from error
    if inner != [sys.executable, '-I', '-S', str(Path(__file__).absolute()), '--cargo-observer', str(paths['cwd']),
                 str(paths['run_directory'] / ('cargo-' + paths['alias'] + '.spec.json'))]:
        raise RuntimeError('Runner033 compiler outer command is not bound to this runner and current specification.')
    budget = _budget033(guard, 'build')
    if (not isinstance(row, dict) or row.get('verified') is not True or row.get('outcome') != 'exited'
            or type(row.get('exit_code')) is not int or row['exit_code'] != 0 or row.get('kind') != 'build'
            or not _equal033(row.get('budget'), budget) or row.get('command') != spec['guard_command']
            or row.get('cwd') != spec['cwd']):
        raise RuntimeError('Runner033 compilation lacks exact verified zero-exit guard command/cwd/budget.')
    evidence_fp, raw = read_file033(paths['evidence_path'], 512 * 1024)
    if evidence_fp.get('mode') != 0o600 or not _equal033(json.loads(raw), evidence):
        raise RuntimeError('Runner033 current observer JSON differs from the supplied successful evidence.')
    if (not isinstance(evidence, dict) or evidence.get('schema') != 'bitcoinrs-final-crates033-cargo-observer-v1'
            or evidence.get('command') != command or evidence.get('cwd') != spec['cwd']
            or evidence.get('target_directory') != spec['target_directory'] or evidence.get('expected_packages') != packages
            or type(evidence.get('cargo_exit_code')) is not int or evidence['cargo_exit_code'] != 0
            or type(evidence.get('observer_exit_code')) is not int or evidence['observer_exit_code'] != 0
            or evidence.get('build_finished') is not True or evidence.get('compile_outcome') != 'passed'
            or 'error' not in evidence or evidence['error'] is not None
            or not isinstance(evidence.get('artifacts'), dict)
            or set(evidence['artifacts']) != set(paths['expected_artifacts'])):
        raise RuntimeError('Runner033 compilation lacks exact successful observer status and complete current artifacts.')
    levels = evidence.get('diagnostic_levels')
    if (not isinstance(levels, dict) or set(levels) - {'error', 'warning', 'note', 'help', 'failure-note', 'ice'}
            or any(type(value) is not int or not 0 <= value <= 50000 for value in levels.values())
            or any(levels.get(level, 0) for level in ('error', 'ice'))):
        raise RuntimeError('Runner033 compiler diagnostics are malformed or contain error/ICE.')
    counts = [evidence.get(key) for key in ('priority_records_observed', 'priority_records_retained', 'priority_records_omitted')]
    if (any(type(value) is not int or not 0 <= value <= 50000 for value in counts)
            or counts[0] != counts[1] + counts[2] or counts[2] != 0
            or counts[0] != sum(levels.get(level, 0) for level in ('error', 'failure-note', 'ice'))):
        raise RuntimeError('Runner033 compiler priority diagnostic retention is incomplete or malformed.')
    artifacts, used = {}, set()
    for key, binding in paths['expected_artifacts'].items():
        record = evidence['artifacts'][key]
        if not isinstance(record, dict): raise RuntimeError('Runner033 compiler artifact is not an object.')
        executable = record.get('executable'); name = key.split('::', 1)[1]
        path = Path(executable) if isinstance(executable, str) else Path()
        if (not path.is_absolute() or '..' in path.parts or path in used
                or path.parent != paths['target_directory'] / 'debug/deps'
                or re.fullmatch(re.escape(name) + r'-[0-9a-f]+', path.name) is None
                or record.get('package') != packages[0] or record.get('target_name') != name
                or record.get('kind') != [binding['kind']] or record.get('src_path') != str(binding['source'])):
            raise RuntimeError('Runner033 artifact package/target/kind/source/private executable identity differs.')
        _package_id033(record.get('package_id'), paths['cwd'] / packages[0])
        fingerprint, _ = read_file033(path, 256 * 1024 * 1024, content=False)
        if not _fingerprint033(fingerprint): raise RuntimeError('Runner033 artifact lacks ordinary executable size/mode/SHA256 evidence.')
        artifacts[key] = {'path': str(path), 'fingerprint': fingerprint}; used.add(path)
    for value in artifacts.values():
        if not _equal033(read_file033(Path(value['path']), 256 * 1024 * 1024, content=False)[0], value['fingerprint']):
            raise RuntimeError('Runner033 compiler artifact changed during qualification.')
    if not _equal033(read_file033(paths['evidence_path'], 512 * 1024, content=False)[0], evidence_fp):
        raise RuntimeError('Runner033 current observer JSON changed during qualification.')
    return artifacts


def _kernel033(probe, budget):
    kernel = probe.get('kernel') if isinstance(probe, dict) else None
    if not isinstance(probe, dict) or probe.get('ok') is not True or not isinstance(kernel, dict):
        raise RuntimeError('Runner033 native probe lacks verified kernel admission.')
    expected = {'memory.max': str(budget['memory']), 'memory.high': str(budget['high']), 'memory.swap.max': '0',
                'memory.oom.group': '1', 'pids.max': str(budget['tasks'])}
    if any(kernel.get(key) != value for key, value in expected.items()):
        raise RuntimeError('Runner033 native kernel memory/swap/pids/OOM controls differ.')
    cpu = kernel.get('cpu.max')
    match = re.fullmatch(r'([0-9]{1,20}) ([0-9]{1,20})', cpu) if isinstance(cpu, str) else None
    if match is None or not 0 < int(match.group(1)) <= int(match.group(2)):
        raise RuntimeError('Runner033 native CPU quota is absent or exceeds one CPU.')
    for key, wanted in (('rlimit_core', [0, 0]), ('rlimit_fsize', [budget['file'], budget['file']])):
        value = kernel.get(key)
        if not isinstance(value, list) or any(type(part) is not int for part in value) or value != wanted:
            raise RuntimeError('Runner033 native core/file-size limits differ.')
    headroom = kernel.get('headroom'); required = budget['memory'] + 256 * 1024 * 1024
    if (not isinstance(headroom, dict) or type(headroom.get('required_bytes')) is not int or headroom['required_bytes'] != required
            or type(headroom.get('mem_available_bytes')) is not int or headroom['mem_available_bytes'] < required):
        raise RuntimeError('Runner033 native memory admission margin differs.')


def _item033(item):
    keys = ('crate', 'target', 'suffix', 'name', 'profile', 'ignored', 'marker', 'seed')
    matches = [value for value in CASES033 if isinstance(item, dict)
               and value['crate'] == item.get('crate') and value['suffix'] == item.get('suffix')]
    if (len(matches) != 1 or type(item.get('ignored')) is not bool
            or any(type(item.get(key)) is not type(matches[0][key]) or item.get(key) != matches[0][key] for key in keys)):
        raise RuntimeError('Runner033 selected native case differs from its exact catalog binding.')
    return dict(matches[0])


def nativebinding033(guard, report, item, row):
    """Bind one catalog case to current bytes, finite controls and exact execution."""
    item = _item033(item)
    if not isinstance(report, dict) or not isinstance(row, dict): raise RuntimeError('Runner033 native report/result is malformed.')
    run = Path(report.get('run_directory', '')); repo = Path(report.get('repository', '')); target = Path(report.get('target_directory', ''))
    if any(not value.is_absolute() or '..' in value.parts for value in (run, repo, target)):
        raise RuntimeError('Runner033 native context lacks absolute ordinary paths.')
    owners = [value for value in (run, *run.parents) if value.parent.name == 'codex-repairs']
    if len(owners) != 1: raise RuntimeError('Runner033 native run lacks one explicit metadata owner.')
    owner = owners[0]; alias = item['crate']
    context = {'command': list(COMMANDS033[alias]), 'cwd': str(repo), 'target_directory': str(target),
               'evidence_path': str(run / 'native-binding-context.json'), 'guard_command': row.get('command'),
               'run_directory': str(run), 'allowed_run_root': str(owner)}
    _observer_context033(context)
    if report.get('allowed_run_root', str(owner)) != str(owner): raise RuntimeError('Runner033 native allowed metadata owner differs.')
    selection = report.get('selected_crates')
    if (not isinstance(selection, (list, tuple)) or any(not isinstance(value, str) for value in selection)
            or tuple(selection) != tuple(value for value in TARGETS033 if value in selection) or alias not in selection):
        raise RuntimeError('Runner033 native selected-crate inventory differs.')
    expected = {TARGETS033[label]['package'] + '::' + name: name for label in selection
                for name in TARGETS033[label]['artifacts']}
    artifacts = report.get('artifacts')
    if not isinstance(artifacts, dict) or set(artifacts) != set(expected): raise RuntimeError('Runner033 native artifacts differ from selected complete crate inventories.')
    used = set()
    for key, name in expected.items():
        value = artifacts[key]
        path = Path(value.get('path', '')) if isinstance(value, dict) else Path()
        if (not path.is_absolute() or '..' in path.parts or path in used or path.parent != target / 'debug/deps'
                or re.fullmatch(re.escape(name) + r'-[0-9a-f]+', path.name) is None or not _fingerprint033(value.get('fingerprint'))):
            raise RuntimeError('Runner033 native artifact private path/fingerprint inventory differs.')
        used.add(path)
    key = TARGETS033[alias]['package'] + '::' + item['target']; selected = artifacts[key]
    if not _equal033(read_file033(Path(selected['path']), 256 * 1024 * 1024, content=False)[0], selected['fingerprint']):
        raise RuntimeError('Runner033 selected native executable changed.')
    data = run / ('case-data-' + item['suffix']); kind = 'stress' if item['profile'] == 'full_original' else 'case'
    budget = _budget033(guard, kind); command = list(case_command033(selected['path'], item))
    environment = dict(case_environment033(item, data))
    required = {'RUST_LOG': 'off', **{name: str(data) for name in ('TEST_TMPDIR', 'TMPDIR', 'TMP', 'TEMP')}}
    if item['seed'] is not None: required['TEST_RANDOM_SEED'] = str(item['seed'])
    if alias == 'randomized' and item['profile'] in {'replay', 'bounded'}: required['BITCOIN_RS_RANDOMIZED_GUARD'] = 'v1'
    if alias == 'randomized' and item['profile'] == 'full_original':
        required.update(BITCOIN_RS_RANDOMIZED_STRESS_GUARD='v1', BITCOIN_RS_RANDOMIZED_CASE_DATA=str(data))
    if alias == 'issue320' and item['profile'] != 'pure': required['BITCOIN_RS_ISSUE320_GUARD'] = 'v1'
    if alias == 'harness' and item['profile'] != 'pure':
        required.update(BITCOIN_RS_HARNESS031_PROTECTED='1', BITCOIN_RS_HARNESS031_CASE=item['name'], BITCOIN_RS_HARNESS031_CASE_DATA=str(data))
    if any(environment.get(name) != value for name, value in required.items()):
        raise RuntimeError('Runner033 environment helper omits a required quiet/seed/private-path/source admission control.')
    log_path = Path(row.get('log', '')); group = log_path.parent
    if (log_path.name != 'output.log' or group.parent != run / 'guard' or '..' in group.parts
            or not re.fullmatch(r'[A-Za-z0-9_.-]{1,128}', group.name) or row.get('command') != command
            or row.get('cwd') != str(repo) or row.get('kind') != kind or not _equal033(row.get('budget'), budget)
            or row.get('data_directory') != str(data) or row.get('verified') is not True or row.get('outcome') != 'exited'
            or type(row.get('exit_code')) is not int or row['exit_code'] != 0):
        raise RuntimeError('Runner033 native exact command, cwd, data, budget or successful result differs.')
    descriptor = ordinary_dir(data)
    try:
        info = os.fstat(descriptor)
        if stat.S_IMODE(info.st_mode) != 0o700 or info.st_uid != os.getuid(): raise RuntimeError('Runner033 native case data is not private owned mode0700.')
    finally: os.close(descriptor)
    values, fingerprints = {}, {}
    for name in ('output.log', 'plan.json', 'probe.json', 'permit', 'result.json'):
        limit = budget['output'] if name == 'output.log' else 1024 * 1024
        fingerprint, raw = read_file033(group / name, limit)
        fingerprints[str(group / name)] = fingerprint
        values[name] = raw if name in {'output.log', 'permit'} else json.loads(raw)
    plan, probe = values['plan.json'], values['probe.json']
    if (not isinstance(plan, dict) or plan.get('command') != command or plan.get('cwd') != str(repo) or plan.get('kind') != kind
            or not isinstance(plan.get('env'), dict) or any(plan['env'].get(name) != value for name, value in environment.items())
            or plan.get('probe') != str(group / 'probe.json') or plan.get('permit') != str(group / 'permit')
            or not isinstance(plan.get('nonce'), str) or not re.fullmatch(r'[0-9a-f]{32}', plan['nonce'])
            or values['permit'] != plan['nonce'].encode() or not _equal033(probe, row.get('probe'))
            or not _equal033(values['result.json'], row) or type(row.get('output_bytes')) is not int
            or row['output_bytes'] != len(values['output.log']) or row['output_bytes'] > budget['output']):
        raise RuntimeError('Runner033 native plan/environment/probe/permit/result/log binding differs.')
    _kernel033(probe, budget)
    assessed = assess_case033(row, values['output.log'].decode('utf-8', errors='replace'), item['name'], item['marker'])
    if assessed['outcome'] != 'passed': raise RuntimeError('Runner033 selected native case did not prove one exact passing nonignored test/marker.')
    for path, fingerprint in fingerprints.items():
        limit = budget['output'] if Path(path).name == 'output.log' else 1024 * 1024
        if not _equal033(read_file033(Path(path), limit, content=False)[0], fingerprint): raise RuntimeError('Runner033 native guard/log evidence changed during qualification.')
    if not _equal033(read_file033(Path(selected['path']), 256 * 1024 * 1024, content=False)[0], selected['fingerprint']):
        raise RuntimeError('Runner033 native executable changed during qualification.')
    return {'label': item['suffix'], 'crate': alias, 'kind': kind, 'required_environment': environment,
            'guard_files': fingerprints, 'assessment': assessed, 'selected_artifact': {'key': key, **selected}}


"""Ordinary working-tree verification; only private run/cache metadata is written."""
from collections.abc import Mapping
import datetime as _dt033
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import sys
import uuid


class _VerificationStop033(RuntimeError):
    def __init__(self, message, exit_code=1, outcome='failed'):
        super().__init__(message)
        self.exit_code, self.outcome = exit_code, outcome


def _plain033(value):
    if isinstance(value, Mapping):
        return {key: _plain033(item) for key, item in value.items()}
    if isinstance(value, (list, tuple)):
        return [_plain033(item) for item in value]
    return value


def _orchestration_selection033(crates):
    if crates is None: return tuple(select_crates033(None))
    if isinstance(crates, str): crates = [crates]
    if not isinstance(crates, (list, tuple)) or not crates:
        raise RuntimeError('Verification requires at least one explicit crate selector.')
    if all(isinstance(item, str) and item in TARGETS033 for item in crates):
        return tuple(alias for alias in TARGETS033 if alias in crates)
    return tuple(select_crates033(crates))


def _context033(repo, run, target):
    _, common = repository_dirs033(repo)
    metadata = Path(common) / 'codex-repairs'
    allowed = {'leveldb-final-crates-check033', 'leveldb-final-crates-runner033'}
    run, target = Path(run), Path(target)
    if any(not path.is_absolute() or '..' in path.parts for path in (run, target)):
        raise RuntimeError('Verification metadata and cache paths must be literal ordinary absolute paths.')
    try: relative = run.relative_to(metadata)
    except ValueError as error: raise RuntimeError('Verification run is outside the repository metadata.') from error
    if (len(relative.parts) not in {3, 4} or relative.parts[0] not in allowed or relative.parts[1] != 'runs'
            or re.fullmatch(r'[0-9]{8}T[0-9]{6}Z-[0-9a-f]{8}', relative.parts[2]) is None
            or len(relative.parts) == 4 and relative.parts[3] != 'verification'):
        raise RuntimeError('Verification run differs from its owned run/optional verification scope.')
    owner = metadata / relative.parts[0]
    try: cache_relative = target.relative_to(owner)
    except ValueError as error: raise RuntimeError('Verification cache is outside the same owned workflow.') from error
    if not cache_relative.parts or cache_relative.parts[0] in {'runs', 'run.lock'}:
        raise RuntimeError('Verification cache must be separate from run evidence and its lock.')
    for path in (owner, run, target):
        descriptor = ordinary_dir(path); os.close(descriptor)
    return owner


def _row_failure033(row, label):
    if not isinstance(row, dict):
        raise _VerificationStop033('Guard did not return an ordinary result for ' + label)
    if row.get('verified') is True and row.get('outcome') == 'exited' and type(row.get('exit_code')) is int and row['exit_code'] == 0:
        return
    if (row.get('outcome') == 'controls_unavailable'
            or row.get('verified') is not True and type(row.get('exit_code')) is int and row['exit_code'] == 78):
        raise _VerificationStop033('Verified execution controls are unavailable for ' + label, 78, 'controls_unavailable')
    raise _VerificationStop033('Guarded phase did not complete successfully: ' + label)


def _artifact_check033(artifacts):
    for key, value in artifacts.items():
        if read_file033(Path(value['path']), 256 * MIB033, content=False)[0] != value['fingerprint']:
            raise RuntimeError('Current compiled artifact changed during verification: ' + key)


def _list_names033(text):
    names = [line[:-6] for line in text.splitlines() if line.endswith(': test')]
    if len(names) != len(set(names)):
        raise RuntimeError('A selected executable listed duplicate native test names.')
    return names


def verify033(repo, crates, suite, *, run_directory=None, target_directory=None, already_locked=False, before_phase=None):
    """Compile current sources and run the selected exact cases once, without retries.

    Internal installer callers supply both private directories and hold their own
    workflow lock. The public runner obtains its normal workflow/cache lock.
    The full initial snapshot and fresh complete-comparison proofs remain
    external immutable JSON evidence files.
    """
    report = {'workflow': 'leveldb-final-crates-check033', 'outcome': 'failed', 'exit_code': 1,
              'phase': 'preflight', 'suite': suite, 'selected_crates': [], 'compilations': {}, 'artifacts': {},
              'test_lists': {'all': {}, 'ignored': {}}, 'cases': [], 'case_guards': {}, 'readouts': {},
              'snapshots': [], 'test_count': 0, 'expected_test_count': 0, 'passed': 0,
              'builds_started': False, 'native_tests_started': False, 'runtime_validation_claimed': False,
              'verification_passed': False, 'full_coverage_claimed': False, 'validation_reused': False,
              'source_writes_started': False, 'refs_updated': False, 'cache_copy_started': False,
              'inputs_preserved': False, 'full_cases_completed': [],
              'scope': 'Fresh Cargo evidence and exact guarded cases for the current stable working tree; preexisting dirty inputs are permitted. No source or ref changes, historical pass reuse, cache copy or automatic retries.'}
    lock_fd, run, target, state, baseline, baseline_encoded, module = None, None, None, None, None, None, None
    state_path, owner, snapshot_index, publication_owned = None, None, 0, False
    primary_error = None
    try:
        repo = Path(os.path.abspath(Path(repo).expanduser())); report['repository'] = str(repo)
        if type(already_locked) is not bool or before_phase is not None and not callable(before_phase):
            raise RuntimeError('Verification lock selection or preservation callback is malformed.')
        selected = _orchestration_selection033(crates)
        cases = tuple(dict(item) for item in selected_cases033(selected, suite))
        all_cases = tuple(dict(item) for item in selected_cases033(selected, 'full'))
        requested = {(item['crate'], item['suffix']) for item in cases}
        report.update(selected_crates=list(selected), selected_packages=[TARGETS033[alias]['package'] for alias in selected],
                      expected_test_count=len(cases),
                      omitted_cases=[{key: item[key] for key in ('crate', 'name', 'suffix', 'profile')}
                                     for item in all_cases if (item['crate'], item['suffix']) not in requested])
        base_env = checked_environment033()
        if already_locked:
            if run_directory is None or target_directory is None:
                raise RuntimeError('Already-locked verification requires both supplied private paths.')
            run, target = Path(run_directory), Path(target_directory)
            owner = _context033(repo, run, target)
        else:
            if run_directory is not None or target_directory is not None:
                raise RuntimeError('Private path overrides require an already-held installer workflow lock.')
            owner, target, lock_fd = run_root033(repo)
            owner, target = Path(owner), Path(target)
            runs = owner / 'runs'; runs.mkdir(mode=0o700, exist_ok=True)
            descriptor = ordinary_dir(runs); os.close(descriptor)
            run = runs / (_dt033.datetime.now(_dt033.timezone.utc).strftime('%Y%m%dT%H%M%SZ') + '-' + uuid.uuid4().hex[:8])
            run.mkdir(mode=0o700)
            owner = _context033(repo, run, target)
        if any(os.path.lexists(run / leaf) for leaf in ('verification-state.json', 'VERIFICATION_RETURN.json')):
            raise RuntimeError('Verification run is occupied; previous state and evidence were retained without retry.')
        publication_owned = True
        report.update(run_directory=str(run), target_directory=str(target), allowed_run_root=str(owner))
        state_path = run / 'verification-state.json'
        state = {'workflow': report['workflow'], 'repository': str(repo), 'run_directory': str(run),
                 'selected_crates': list(selected), 'suite': suite, 'phase': 'reserved'}
        atomic_json033(state_path, state)
        module = load_guard033(repo)
        guard = module.Guard(run / 'guard', Path(__file__).absolute(), base_env)

        def phase(label, command, *, kind='source', env=None, tmpdir=None, check=True):
            report['phase'] = label
            if before_phase is not None: before_phase(label)
            row = guard.run(label, list(command), repo, dict(env or {}), kind=kind, **({'tmpdir': tmpdir} if tmpdir is not None else {}))
            report['readouts'][label] = row
            if check: _row_failure033(row, label)
            return row

        def capture(label):
            nonlocal snapshot_index, baseline, baseline_encoded
            report['inputs_preserved'] = False
            snapshot_index += 1
            leaf = 'inputs-' + format(snapshot_index, '04d') + '.json'; path = run / leaf
            if baseline_encoded is None:
                command = [sys.executable, '-I', '-S', str(Path(__file__).absolute()),
                           '--capture-inputs', str(repo), str(path)]
            else:
                command = [sys.executable, '-I', '-S', str(Path(__file__).absolute()),
                           '--verify-inputs', str(repo), report['inputs_before']['path'], str(path)]
            row = phase('snapshot-' + label, command, check=False)
            if not os.path.lexists(path):
                _row_failure033(row, 'snapshot-' + label)
                raise RuntimeError('Guarded input capture did not publish its bounded evidence.')
            fingerprint, raw = read_file033(path, 8 * MIB033)
            value = json.loads(raw)
            if not isinstance(value, dict): raise RuntimeError('Current input snapshot/proof is not an object.')
            if baseline_encoded is None:
                _row_failure033(row, 'snapshot-' + label)
                encoded = encoded033(value)
                if raw != encoded: raise RuntimeError('Initial working-tree input snapshot is not canonical JSON.')
                baseline, baseline_encoded = value, encoded
                canonical = digest033(encoded); same = True
                report['inputs_before'] = {'path': str(path), 'fingerprint': fingerprint,
                                           'canonical_sha256': canonical}
            else:
                before = report['inputs_before']
                if (value.get('schema') != 'bitcoinrs-working-code-input-verification033-v1'
                        or value.get('baseline_file') != before['path']
                        or value.get('baseline_fingerprint') != before['fingerprint']
                        or value.get('input_count') != baseline.get('input_count')
                        or read_file033(Path(before['path']), 8 * MIB033, content=False)[0] != before['fingerprint']):
                    raise RuntimeError('Fresh input proof is not bound to its complete retained baseline.')
                canonical = value.get('canonical_sha256')
                same = value.get('inputs_preserved') is True and canonical == before['canonical_sha256']
                if value.get('inputs_preserved') is True and not same:
                    raise RuntimeError('Fresh input proof contradicts its whole-snapshot digest.')
            record = {'label': label, 'path': str(path), 'fingerprint': fingerprint,
                      'canonical_sha256': canonical, 'matches_initial': same,
                      'record_kind': 'full_initial_snapshot' if snapshot_index == 1 else 'fresh_complete_comparison_proof'}
            if value.get('observed_file') is not None:
                observed_path = path.with_name(path.stem + '.observed.json')
                if value.get('observed_file') != str(observed_path):
                    raise RuntimeError('Changed-input evidence is outside its exact owned proof path.')
                observed_fp = read_file033(observed_path, 8 * MIB033, content=False)[0]
                if value.get('observed_fingerprint') != observed_fp:
                    raise RuntimeError('Changed-input evidence fingerprint differs.')
                record.update(observed_file=str(observed_path), observed_fingerprint=observed_fp)
            report['snapshots'].append(record)
            report['inputs_after'] = {'path': str(path), 'fingerprint': fingerprint,
                                     'canonical_sha256': canonical}
            if not same:
                raise RuntimeError('Working-tree source or Git metadata changed during verification: ' + label)
            _row_failure033(row, 'snapshot-' + label)
            report['inputs_preserved'] = all(item['matches_initial'] for item in report['snapshots'])
            return value

        phase('preflight', [sys.executable, '-I', '-S', '-c', "print('FINAL_CRATES033_GUARD_PREFLIGHT_OK')"], kind='probe')
        capture('initial')
        nix_shell = shutil.which('nix-shell', path=base_env.get('PATH'))
        if nix_shell is None:
            raise _VerificationStop033('Offline nix-shell is unavailable; no alternate compilation was attempted.', 78, 'controls_unavailable')
        nix_shell = os.path.abspath(nix_shell)
        for alias in selected:
            capture('before-build-' + alias)
            label = 'compile-' + alias
            spec_path = run / ('cargo-' + alias + '.spec.json'); evidence_path = run / ('cargo-' + alias + '.json')
            command = [nix_shell, '--option', 'max-jobs', '0', '--option', 'builders', '', '--option', 'substitute', 'false',
                       str(repo / 'shell.nix'), '--run', shlex.join([sys.executable, '-I', '-S', str(Path(__file__).absolute()),
                       '--cargo-observer', str(repo), str(spec_path)])]
            spec = {'command': list(COMMANDS033[alias]), 'cwd': str(repo), 'target_directory': str(target),
                    'evidence_path': str(evidence_path), 'guard_command': command,
                    'run_directory': str(run), 'allowed_run_root': str(owner)}
            atomic_json033(spec_path, spec)
            state.update(phase=label, compilation_pending=alias); atomic_json033(state_path, state)
            build_error = None
            try:
                row = phase(label, command, kind='build', env={'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '1',
                    'CARGO_INCREMENTAL': '0', 'RUST_LOG': 'off'}, check=False)
                if row.get('verified') is True: report['builds_started'] = True
                if os.path.lexists(evidence_path):
                    evidence_fp, raw = read_file033(evidence_path, MIB033)
                    evidence = json.loads(raw)
                    report['compilations'][alias] = {'outcome': 'observed_not_qualified',
                        'command': spec['command'], 'evidence_file': str(evidence_path),
                        'evidence_fingerprint': evidence_fp, 'cargo_exit_code': evidence.get('cargo_exit_code'),
                        'observer_exit_code': evidence.get('observer_exit_code'),
                        'diagnostic_levels': evidence.get('diagnostic_levels', {})}
                _row_failure033(row, label)
                if alias not in report['compilations']:
                    raise RuntimeError('Current Cargo observer did not publish its bounded evidence file.')
                artifacts = qualify_compile033(module, row, evidence, spec)
                if set(artifacts) & set(report['artifacts']): raise RuntimeError('Current compilation repeated a selected artifact identity.')
                report['artifacts'].update(_plain033(artifacts))
                report['compilations'][alias] = {'outcome': 'compiled', 'command': spec['command'],
                    'evidence_file': str(evidence_path), 'evidence_fingerprint': evidence_fp,
                    'artifacts': _plain033(artifacts), 'cargo_exit_code': evidence.get('cargo_exit_code'),
                    'observer_exit_code': evidence.get('observer_exit_code'), 'diagnostic_levels': evidence.get('diagnostic_levels', {})}
            except BaseException as error:
                build_error = error
                state['compilation_failure'] = {'crate': alias, 'error': type(error).__name__ + ': ' + str(error)[:2048]}
                atomic_json033(state_path, state)
            try: capture('after-build-' + alias)
            except BaseException as error:
                if build_error is None: build_error = error
                else: report.setdefault('secondary_errors', []).append(type(error).__name__ + ': ' + str(error)[:2048])
            if build_error is not None: raise build_error
            state.pop('compilation_pending'); state['phase'] = 'compiled-' + alias; atomic_json033(state_path, state)
        _artifact_check033(report['artifacts'])
        expected_lists = _plain033(expectedlists033(selected))
        for key, artifact in sorted(report['artifacts'].items()):
            for selection in ('all', 'ignored'):
                label = 'list-' + key.replace('::', '-') + '-' + selection
                state.update(phase=label, native_pending={'kind': 'listing', 'target': key, 'selection': selection})
                atomic_json033(state_path, state)
                try:
                    row = phase(label, [artifact['path'], '--list'] + (['--ignored'] if selection == 'ignored' else []), kind='case', env={'RUST_LOG': 'off'})
                    report['native_tests_started'] = True
                    _, raw = read_file033(Path(row['log']), MIB033)
                    names = _list_names033(raw.decode('utf-8', errors='replace'))
                    expected = expected_lists[selection].get(key)
                    if expected is None or sorted(names) != sorted(expected):
                        raise RuntimeError('Current executable all/ignored inventory differs from the catalog: ' + key)
                    report['test_lists'][selection][key] = names
                except BaseException:
                    state['native_failure'] = dict(state['native_pending']); atomic_json033(state_path, state)
                    raise
                state.pop('native_pending'); atomic_json033(state_path, state)
        if set(report['test_lists']['all']) != set(expected_lists['all']) or set(report['test_lists']['ignored']) != set(expected_lists['ignored']):
            raise RuntimeError('Current executable metadata lacks an exact selected target inventory.')
        for item in cases:
            case_key = item['crate'] + '::' + item['suffix']; label = item['crate'] + '-' + item['suffix']
            capture('before-case-' + label)
            _artifact_check033(report['artifacts'])
            data = run / ('case-data-' + item['suffix']); data.mkdir(mode=0o700)
            descriptor = ordinary_dir(data); os.close(descriptor)
            state.update(phase=label, native_pending={'run': str(run), 'crate': item['crate'],
                         'case': item['name'], 'profile': item['profile']})
            atomic_json033(state_path, state)
            case_error = None
            try:
                executable = report['artifacts'][TARGETS033[item['crate']]['package'] + '::' + item['target']]['path']
                row = phase(label, list(case_command033(executable, item)),
                    kind='stress' if item['profile'] == 'full_original' else 'case',
                    env=dict(case_environment033(item, data)), tmpdir=data, check=False)
                if row.get('verified') is True: report['native_tests_started'] = True
                text = read_file033(Path(row['log']), MIB033)[1].decode('utf-8', errors='replace') if row.get('log') else ''
                assessed = assess_case033(row, text, item['name'], item['marker'])
                log_fp = read_file033(Path(row['log']), MIB033, content=False)[0] if row.get('log') else {'kind': 'absent'}
                report['cases'].append(dict(assessed, crate=item['crate'], target=item['target'], suffix=item['suffix'],
                    label=label, profile=item['profile'], ignored_by_default=item['ignored'],
                    log_path=row.get('log'), log_fingerprint=log_fp, guard_result=label))
                report['test_count'] = len(report['cases'])
                _row_failure033(row, label)
                if assessed.get('outcome') != 'passed': raise RuntimeError('Selected current native case did not pass: ' + case_key)
                report['case_guards'][case_key] = _plain033(nativebinding033(module, report, item, row))
                report['passed'] += 1
                if item['profile'] == 'full_original': report['full_cases_completed'].append(case_key)
            except BaseException as error:
                case_error = error
                state['native_failure'] = dict(state['native_pending']); atomic_json033(state_path, state)
            try: capture('after-case-' + label)
            except BaseException as error:
                if case_error is None: case_error = error
                else: report.setdefault('secondary_errors', []).append(type(error).__name__ + ': ' + str(error)[:2048])
            if case_error is not None: raise case_error
            state.pop('native_pending'); atomic_json033(state_path, state)
        capture('final')
        _artifact_check033(report['artifacts'])
        if report['test_count'] != len(cases) or report['passed'] != len(cases):
            raise RuntimeError('The exact requested current native case inventory did not complete.')
        report.update(outcome='passed' if cases else 'compiled_only', exit_code=0, phase='complete',
                      verification_passed=True, inputs_preserved=True, runtime_validation_claimed=bool(cases))
    except BaseException as error:
        primary_error = error
        report.update(outcome=getattr(error, 'outcome', 'failed'), exit_code=getattr(error, 'exit_code', 1),
                      error=type(error).__name__ + ': ' + str(error)[:4096], verification_passed=False)
    finally:
        if state is not None:
            state.update(phase='complete' if report['exit_code'] == 0 else 'failed', outcome=report['outcome'], exit_code=report['exit_code'])
            if primary_error is not None: state['error'] = report.get('error')
            try: atomic_json033(state_path, state)
            except BaseException as error:
                report.update(outcome='failed', exit_code=1, verification_passed=False, publication_error=str(error)[:2048])
        if run is not None and publication_owned:
            report['return_file'] = str(run / 'VERIFICATION_RETURN.json')
            try: atomic_json033(Path(report['return_file']), report)
            except BaseException as error:
                report.update(outcome='failed', exit_code=1, verification_passed=False, publication_error=str(error)[:2048])
                try: atomic_json033(Path(report['return_file']), report)
                except BaseException: pass
        if lock_fd is not None: os.close(lock_fd)
    return report


def main033(argv=None):
    sys.dont_write_bytecode = True
    arguments = sys.argv[1:] if argv is None else list(argv)
    if len(arguments) == 2 and arguments[0] == '--guard-child':
        return load_guard033().guard_child(Path(arguments[1]))
    if len(arguments) == 3 and arguments[0] == '--capture-inputs':
        return capture_worker033(Path(arguments[1]), Path(arguments[2]))
    if len(arguments) == 4 and arguments[0] == '--verify-inputs':
        return verify_inputs_worker033(Path(arguments[1]), Path(arguments[2]), Path(arguments[3]))
    if len(arguments) == 3 and arguments[0] == '--cargo-observer':
        return observer_worker033(Path(arguments[1]), Path(arguments[2]))
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=Path(__file__).absolute().parents[2])
    parser.add_argument('--crate', action='append', dest='crates',
        help='Repeatable exact package name or all; default all.')
    parser.add_argument('--suite', choices=('quick', 'full'), default='quick',
        help='quick excludes the five original stress cases; full runs every registered case.')
    args = parser.parse_args(arguments)
    try: select_crates033(args.crates or ['all'])
    except (RuntimeError, ValueError) as error: parser.error(str(error))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_AS, (768 * MIB033,) * 2)
    def stop(signum, frame):
        if signum == signal.SIGTERM: raise KeyboardInterrupt('Runner termination requested.')
        raise RuntimeError('Runner coordinator deadline reached.')
    signal.signal(signal.SIGALRM, stop); signal.signal(signal.SIGTERM, stop); signal.alarm(7200)
    report = verify033(Path(os.path.abspath(args.repo.expanduser())), args.crates or ['all'], args.suite)
    print('BEGIN_LEVELDB_FINAL_CRATES_RETURN', flush=True)
    print(json.dumps(report, indent=2, sort_keys=True), flush=True)
    print('END_LEVELDB_FINAL_CRATES_RETURN', flush=True)
    return report['exit_code']


if __name__ == '__main__': raise SystemExit(main033())
