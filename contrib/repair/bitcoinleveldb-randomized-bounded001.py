#!/usr/bin/env python3
"""Install and verify resource-controlled randomized contract tests."""


# Resource supervisor: no requested process runs before verified controls.

"""Fail-closed systemd user-service containment for build and test children."""
import collections
import json
import os
from pathlib import Path
import re
import resource
import selectors
import shutil
import subprocess
import sys
import time
import uuid

MIB = 1024 * 1024
BUDGETS = {
    "build": dict(memory=4096*MIB, high=3072*MIB, tasks=128, seconds=600, file=512*MIB, output=2*MIB),
    "case": dict(memory=512*MIB, high=384*MIB, tasks=64, seconds=30, file=16*MIB, output=MIB),
    "probe": dict(memory=128*MIB, high=96*MIB, tasks=32, seconds=10, file=16*MIB, output=MIB),
}
PROPERTIES = ("LoadState", "ActiveState", "SubState", "Type", "Restart", "RemainAfterExit", "Result",
              "ExecMainCode", "ExecMainStatus", "RuntimeMaxUSec", "TimeoutStopUSec", "KillMode",
              "OOMPolicy", "SendSIGKILL", "MemoryMax", "MemoryHigh", "MemorySwapMax", "TasksMax",
              "CPUQuotaPerSecUSec", "MemoryPeak", "CPUUsageNSec", "ControlGroup")


def _atomic_json(path, value):
    path = Path(path)
    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(fd, "w") as stream:
        json.dump(value, stream, sort_keys=True, indent=2)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(temporary, path)


def _events(path):
    try:
        return {key: int(value) for key, value in (line.split() for line in Path(path).read_text().splitlines())}
    except (OSError, ValueError):
        return None


def _cgroup_snapshot(path):
    result = {"path": str(path)}
    for name in ("memory.max", "memory.high", "memory.swap.max", "cpu.max", "pids.max", "memory.oom.group", "memory.current", "memory.peak"):
        try:
            result[name] = (path / name).read_text().strip()
        except OSError:
            result[name] = None
    result["memory.events"] = _events(path / "memory.events")
    return result


def _headroom(path, budget):
    """Conservative admission policy; observations can race later allocations."""
    root = Path("/sys/fs/cgroup").resolve()
    required = budget["memory"] + 256*MIB
    match = re.search(r"^MemAvailable:\s+(\d+)\s+kB\s*$", Path("/proc/meminfo").read_text(), re.MULTILINE)
    if not match:
        raise RuntimeError("MemAvailable is unavailable; workload admission was refused.")
    result = {"required_bytes": required, "mem_available_bytes": int(match.group(1))*1024, "ancestors": []}
    if result["mem_available_bytes"] < required:
        raise RuntimeError("Host memory headroom is below the declared admission margin: " + repr(result))
    ancestor = path.parent
    while ancestor.is_relative_to(root):
        row = {"path": str(ancestor)}
        try:
            maximum = (ancestor / "memory.max").read_text().strip()
        except OSError:
            if ancestor != root:
                raise RuntimeError("Ancestor memory.max is unreadable: " + str(ancestor))
            maximum = None
        try:
            current = int((ancestor / "memory.current").read_text().strip())
        except (OSError, ValueError):
            if ancestor != root or maximum is not None:
                raise RuntimeError("Ancestor memory.current is unreadable: " + str(ancestor))
            current = None
        row.update(maximum=maximum, current=current)
        result["ancestors"].append(row)
        if maximum not in (None, "max"):
            try:
                remaining = int(maximum) - current
            except (TypeError, ValueError):
                raise RuntimeError("Ancestor memory limit has an invalid value: " + str(ancestor))
            row["remaining_bytes"] = remaining
            if remaining < required:
                raise RuntimeError("Ancestor memory headroom is below the declared admission margin: " + repr(result))
        if ancestor == root:
            break
        ancestor = ancestor.parent
    return result


def _kernel_probe(budget):
    roots = [line[3:] for line in Path("/proc/self/cgroup").read_text().splitlines() if line.startswith("0::")]
    if len(roots) != 1 or not Path("/sys/fs/cgroup/cgroup.controllers").is_file():
        raise RuntimeError("A cgroup-v2 process membership and controller hierarchy are required.")
    root = Path("/sys/fs/cgroup").resolve()
    path = (root / roots[0].lstrip("/")).resolve()
    if not path.is_relative_to(root):
        raise RuntimeError("Unexpected cgroup path.")
    result = _cgroup_snapshot(path)
    expected = {"memory.max": str(budget["memory"]), "memory.high": str(budget["high"]),
                "memory.swap.max": "0", "pids.max": str(budget["tasks"]), "memory.oom.group": "1"}
    if any(result[key] != value for key, value in expected.items()):
        raise RuntimeError("Kernel memory/swap/task/OOM controls differ from the requested policy: " + repr(result))
    quota, period = result["cpu.max"].split()
    if quota == "max" or int(quota) <= 0 or int(period) <= 0 or int(quota) > int(period):
        raise RuntimeError("Kernel CPU quota is absent or exceeds one CPU.")
    result["rlimit_core"] = list(resource.getrlimit(resource.RLIMIT_CORE))
    result["rlimit_fsize"] = list(resource.getrlimit(resource.RLIMIT_FSIZE))
    if result["rlimit_core"] != [0, 0] or result["rlimit_fsize"] != [budget["file"], budget["file"]]:
        raise RuntimeError("Core/file-size resource limits were not installed.")
    result["headroom"] = _headroom(path, budget)
    return result


def guard_child(plan_path):
    """Internal entry point; no requested command executes before both checks pass."""
    plan = json.loads(Path(plan_path).read_text())
    probe_path = Path(plan["probe"])
    try:
        budget = BUDGETS[plan["kind"]]
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
        resource.setrlimit(resource.RLIMIT_FSIZE, (budget["file"], budget["file"]))
        snapshot = _kernel_probe(budget)
        _atomic_json(probe_path, {"ok": True, "pid": os.getpid(), "kernel": snapshot})
        until = time.monotonic() + 6
        permit = Path(plan["permit"])
        while not permit.exists():
            if time.monotonic() >= until:
                raise RuntimeError("Parent did not authorize execution after verifying service properties.")
            time.sleep(0.025)
        if permit.read_text() != plan["nonce"]:
            raise RuntimeError("Execution authorization does not match this child.")
        os.chdir(plan["cwd"])
        os.execvpe(plan["command"][0], plan["command"], plan["env"])
    except BaseException as error:
        _atomic_json(probe_path, {"ok": False, "error": type(error).__name__ + ": " + str(error)})
        print("GUARD_CHILD_REFUSED: " + str(error), file=sys.stderr)
        return 78


def _seconds(value):
    if value is None or value == "infinity":
        return None
    if value.isdigit():
        return int(value) / 1000000
    units = {"us": 0.000001, "ms": 0.001, "s": 1, "min": 60, "h": 3600, "d": 86400, "w": 604800}
    compact = value.replace(" ", "").replace("µs", "us").replace("μs", "us")
    parts = re.findall(r"(\d+(?:\.\d+)?)(us|ms|min|s|h|d|w)", compact)
    if not parts or "".join(number + unit for number, unit in parts) != compact:
        return None
    return sum(float(number) * units[unit] for number, unit in parts)


def _service_controls(properties, budget):
    expected = {"Type": "exec", "Restart": "no", "RemainAfterExit": "no", "KillMode": "control-group", "OOMPolicy": "kill", "SendSIGKILL": "yes"}
    if any(properties.get(key) != value for key, value in expected.items()):
        return False
    runtime = _seconds(properties.get("RuntimeMaxUSec"))
    stop = _seconds(properties.get("TimeoutStopUSec"))
    return runtime is not None and 0 < runtime <= budget["seconds"] + 0.001 and stop is not None and 0 < stop <= 3.001


def _tree_usage(root):
    size = count = 0
    todo = [Path(root)]
    try:
        while todo:
            with os.scandir(todo.pop()) as entries:
                for entry in entries:
                    count += 1
                    if count > 4096:
                        return size, count
                    info = entry.stat(follow_symlinks=False)
                    size += info.st_size
                    if size > 128*MIB:
                        return size, count
                    if entry.is_dir(follow_symlinks=False):
                        todo.append(Path(entry.path))
    except FileNotFoundError:
        pass
    return size, count


class Guard:
    def __init__(self, directory, script_path, base_env):
        self.directory = Path(directory).resolve()
        self.directory.mkdir(parents=True, exist_ok=True, mode=0o700)
        self.script_path = Path(script_path).resolve()
        self.base_env = dict(base_env)
        self.systemd_run = shutil.which("systemd-run", path=self.base_env.get("PATH"))
        self.systemctl = shutil.which("systemctl", path=self.base_env.get("PATH"))

    def _ctl(self, *args):
        child = None
        try:
            child = subprocess.Popen([self.systemctl, "--user", *args], env=self.base_env, stdin=subprocess.DEVNULL,
                                     stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True)
            os.set_blocking(child.stdout.fileno(), False)
            buffer = bytearray()
            until = time.monotonic() + 1.5
            with selectors.DefaultSelector() as selector:
                selector.register(child.stdout.fileno(), selectors.EVENT_READ)
                while child.poll() is None or selector.get_map():
                    if time.monotonic() >= until or len(buffer) > 32768:
                        child.kill()
                        return {"exit_code": None, "error": "Control query exceeded its time/output budget.", "stdout": bytes(buffer[:32768]).decode(errors="replace")}
                    for key, _ in selector.select(0.05):
                        try:
                            data = os.read(key.fd, 4096)
                        except BlockingIOError:
                            continue
                        if data:
                            buffer.extend(data)
                        else:
                            selector.unregister(key.fd)
            return {"exit_code": child.returncode, "stdout": buffer.decode(errors="replace"), "stderr": ""}
        except OSError as error:
            return {"exit_code": None, "error": str(error)}
        finally:
            if child:
                if child.poll() is None:
                    child.kill()
                try:
                    child.wait(timeout=0.2)
                except subprocess.TimeoutExpired:
                    pass
                child.stdout.close()

    def _show(self, unit):
        result = self._ctl("show", unit, *["--property=" + key for key in PROPERTIES])
        if result.get("exit_code") == 0:
            return dict(line.split("=", 1) for line in result["stdout"].splitlines() if "=" in line)
        return {"unavailable": result}

    def preflight(self):
        return self.run("preflight", [sys.executable, "-c", "print('GUARD_PREFLIGHT_OK')"], self.directory, {}, kind="probe")

    def run(self, label, command, cwd, env, kind="case", tmpdir=None):
        budget = dict(BUDGETS[kind])
        result = {"label": label, "kind": kind, "budget": budget, "outcome": "controls_unavailable", "verified": False, "exit_code": None}
        if not self.systemd_run or not self.systemctl:
            return dict(result, error="systemd-run and systemctl are required; unrestricted execution was refused.")
        manager = self._ctl("show", "--property=Version", "--value")
        if manager.get("exit_code") != 0:
            return dict(result, error="The systemd user manager is unavailable.", manager=manager)
        merged = dict(self.base_env)
        merged.update({str(key): str(value) for key, value in env.items()})
        executable = shutil.which(str(command[0]), path=merged.get("PATH"))
        if executable is None:
            return dict(result, error="Command executable was not found: " + str(command[0]))
        # Keep the executable entry name: Nix dispatches nix-shell by argv[0].
        command = [os.path.abspath(executable), *map(str, command[1:])]
        stem = re.sub(r"[^a-zA-Z0-9_.-]", "-", str(label))[:40] + "-" + uuid.uuid4().hex[:12]
        directory = self.directory / stem
        directory.mkdir(mode=0o700)
        data = Path(tmpdir).resolve() if tmpdir else directory / "test-data"
        data.mkdir(parents=True, exist_ok=True, mode=0o700)
        merged["TEST_TMPDIR"] = str(data)
        plan = {"kind": kind, "command": command, "cwd": str(Path(cwd).resolve()), "env": merged,
                "probe": str(directory / "probe.json"), "permit": str(directory / "permit"), "nonce": uuid.uuid4().hex}
        plan_path = directory / "plan.json"
        _atomic_json(plan_path, plan)
        unit = "bitcoin-rs-guard-" + stem + ".service"
        argv = [self.systemd_run, "--user", "--wait", "--pipe", "--service-type=exec", "--unit=" + unit,
                "--property=MemoryAccounting=yes", "--property=CPUAccounting=yes", "--property=TasksAccounting=yes",
                "--property=MemoryHigh=" + str(budget["high"]), "--property=MemoryMax=" + str(budget["memory"]),
                "--property=MemorySwapMax=0", "--property=CPUQuota=100%", "--property=TasksMax=" + str(budget["tasks"]),
                "--property=RuntimeMaxSec=" + str(budget["seconds"]) + "s", "--property=RuntimeRandomizedExtraSec=0",
                "--property=TimeoutStartSec=5s", "--property=TimeoutStopSec=3s", "--property=KillMode=control-group",
                "--property=OOMPolicy=kill", "--property=SendSIGKILL=yes", "--property=Restart=no", "--property=LimitCORE=0",
                str(Path(sys.executable).resolve()), str(self.script_path), "--guard-child", str(plan_path)]
        result.update(unit=unit, command=command, cwd=plan["cwd"], data_directory=str(data), log=str(directory / "output.log"))
        started = time.monotonic()
        deadline, termination_until = started + budget["seconds"] + 5, None
        reason = None
        child = None
        tail = bytearray()
        output_bytes = 0
        probe = None
        last_sample = started
        last_cgroup_sample = started
        snapshots = collections.deque(maxlen=8)
        try:
            child = subprocess.Popen(argv, env=self.base_env, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                                     stderr=subprocess.STDOUT, bufsize=0)
            fd = child.stdout.fileno()
            os.set_blocking(fd, False)
            selector = selectors.DefaultSelector()
            selector.register(fd, selectors.EVENT_READ)
            with (directory / "output.log").open("wb") as log:
                while child.poll() is None or selector.get_map():
                    now = time.monotonic()
                    if probe is None and Path(plan["probe"]).exists():
                        probe = json.loads(Path(plan["probe"]).read_text())
                        result["probe"] = probe
                        if not probe.get("ok"):
                            reason = "controls_unavailable"
                        else:
                            properties = self._show(unit)
                            result["verified_properties"] = properties
                            if not _service_controls(properties, budget):
                                reason = "controls_unavailable"
                            else:
                                Path(plan["permit"]).write_text(plan["nonce"])
                                result["verified"] = True
                    if now - last_sample >= 0.1:
                        last_sample = now
                        if kind == "case":
                            size, count = _tree_usage(data)
                            result["data_bytes"], result["data_entries"] = size, count
                            if size > 128*MIB or count > 4096:
                                reason = reason or "data_limit"
                        if shutil.disk_usage(directory).free < 2*1024*MIB:
                            reason = reason or "disk_free_limit"
                        if probe and probe.get("ok") and now - last_cgroup_sample >= 0.5:
                            snapshots.append(_cgroup_snapshot(Path(probe["kernel"]["path"])))
                            last_cgroup_sample = now
                    if now >= deadline:
                        reason = reason or "parent_timeout"
                    if reason and termination_until is None:
                        result["stop_request"] = self._ctl("--no-block", "stop", unit)
                        termination_until = time.monotonic() + 5
                    if termination_until is not None and time.monotonic() >= termination_until:
                        result["kill_request"] = self._ctl("kill", "--kill-whom=all", "--signal=KILL", unit)
                        break
                    for key, _ in selector.select(0.1):
                        for _chunk in range(16):
                            try:
                                raw = os.read(key.fd, 65536)
                            except BlockingIOError:
                                break
                            if not raw:
                                selector.unregister(key.fd)
                                break
                            old_bytes = output_bytes
                            output_bytes += len(raw)
                            if old_bytes < budget["output"]:
                                log.write(raw[:budget["output"] - old_bytes])
                            tail.extend(raw)
                            del tail[:-65536]
                            if output_bytes > budget["output"]:
                                reason = reason or "output_limit"
                            text = tail.decode(errors="replace").lower()
                            if "ignoring" in text and "restricted" in text and any(word in text for word in ("max-jobs", "builders", "substitute")):
                                reason = reason or "restricted_nix_setting_ignored"
                selector.close()
            if child.poll() is None:
                child.terminate()
                try:
                    child.wait(timeout=0.5)
                except subprocess.TimeoutExpired:
                    child.kill()
                    child.wait(timeout=0.5)
            result["exit_code"] = child.returncode
        except KeyboardInterrupt:
            reason = "interrupted"
            result["stop_request"] = self._ctl("--no-block", "stop", unit)
            result["kill_request"] = self._ctl("kill", "--kill-whom=all", "--signal=KILL", unit)
        except Exception as error:
            reason = reason or "guard_error"
            result["error"] = type(error).__name__ + ": " + str(error)
        finally:
            if child and child.poll() is None:
                result["emergency_stop"] = self._ctl("--no-block", "stop", unit)
                result["emergency_kill"] = self._ctl("kill", "--kill-whom=all", "--signal=KILL", unit)
                child.kill()
                try:
                    child.wait(timeout=0.2)
                except subprocess.TimeoutExpired:
                    result["client_reap_pending"] = True
            result.update(elapsed_seconds=time.monotonic() - started, output_bytes=output_bytes,
                          tail=tail.decode(errors="replace"), live_cgroup=list(snapshots), postmortem=self._show(unit))
            if probe and probe.get("ok"):
                result["final_cgroup"] = _cgroup_snapshot(Path(probe["kernel"]["path"]))
            properties = result["postmortem"]
            if properties.get("Result") == "oom-kill":
                reason = reason or "memory_limit"
            elif properties.get("Result") == "timeout":
                reason = reason or "service_timeout"
            result["outcome"] = reason or ("exited" if result["verified"] else "controls_unavailable")
            result["cleanup"] = {"stop": self._ctl("--no-block", "stop", unit), "reset": self._ctl("reset-failed", unit)}
            _atomic_json(directory / "result.json", result)
            if child and child.stdout:
                child.stdout.close()
        return result


# Read-only incident evidence, executed in a small protected service.

"""Small read-only host evidence collection; called inside the probe service."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tomllib


def collect_randomized_host_evidence(origin, destination):
    origin, destination = Path(origin), Path(destination)
    result = {"purpose": "previous-boot and installed tracing-source evidence; no cause inferred"}
    try:
        lock = tomllib.loads((origin / "Cargo.lock").read_text())
        wanted = [p for p in lock.get("package", []) if p.get("name") in {"traced-test", "tracing-setup"}]
        result["locked_tracing_packages"] = [{k: p[k] for k in ("name", "version", "source", "checksum") if k in p} for p in wanted]
        cargo_home = Path(os.environ.get("CARGO_HOME", str(Path.home() / ".cargo"))).expanduser()
        inspected = []
        retained = 0
        for package in wanted:
            for folder in sorted((cargo_home / "registry" / "src").glob("*/" + package["name"] + "-" + package["version"]))[:4]:
                item = {"package": package["name"], "version": package["version"], "directory": str(folder), "files": []}
                checksums = None
                archive = None
                item["package_checksum_matches_lock"] = None
                try:
                    candidate = json.loads((folder / ".cargo-checksum.json").read_text())
                    if not isinstance(candidate, dict) or not isinstance(candidate.get("files"), dict):
                        raise ValueError("checksum manifest has an unexpected shape")
                    checksums = candidate
                    item["package_checksum_matches_lock"] = checksums.get("package") == package.get("checksum")
                except (OSError, ValueError) as error:
                    item["checksum_manifest_unavailable"] = str(error)
                cached = cargo_home / "registry" / "cache" / folder.parent.name / (folder.name + ".crate")
                try:
                    if cached.is_symlink() or cached.stat().st_size > 4 * 1024 * 1024:
                        raise ValueError("cached crate is a symlink or exceeds the inspection budget")
                    cached_digest = hashlib.sha256(cached.read_bytes()).hexdigest()
                    item["cached_archive"] = {"path": str(cached), "sha256": cached_digest,
                                               "matches_lock": cached_digest == package.get("checksum")}
                    if item["cached_archive"]["matches_lock"]:
                        archive = tarfile.open(cached, mode="r:gz")
                except (OSError, ValueError, tarfile.TarError) as error:
                    item["cached_archive_unavailable"] = str(error)
                try:
                    for source in sorted((folder / "src").rglob("*.rs"))[:24]:
                        if source.is_symlink() or source.stat().st_size > 96 * 1024:
                            continue
                        data = source.read_bytes()
                        if retained + len(data) > 384 * 1024:
                            item["source_budget_reached"] = True
                            break
                        retained += len(data)
                        name = source.relative_to(folder).as_posix()
                        digest = hashlib.sha256(data).hexdigest()
                        manifest_match = checksums.get("files", {}).get(name) == digest if checksums is not None else None
                        archived_match = None
                        if archive is not None:
                            try:
                                member = archive.getmember(folder.name + "/" + name)
                                if member.isfile() and member.size <= 96 * 1024:
                                    stream = archive.extractfile(member)
                                    if stream is not None:
                                        with stream:
                                            archived_match = hashlib.sha256(stream.read(96 * 1024 + 1)).hexdigest() == digest
                            except (KeyError, OSError, ValueError, tarfile.TarError):
                                pass
                        verification = ("locked-archive" if archived_match is True else
                                        "checksum-manifest" if manifest_match is True and item["package_checksum_matches_lock"] is True else
                                        "unverified-local-source")
                        item["files"].append({"path": name, "sha256": digest,
                                              "matches_package_file_checksum": manifest_match,
                                              "matches_locked_archive": archived_match,
                                              "verification": verification,
                                              "content": data.decode(errors="replace")})
                except (OSError, ValueError) as error:
                    item["inspection_error"] = str(error)
                finally:
                    if archive is not None:
                        archive.close()
                inspected.append(item)
        result["installed_tracing_sources"] = inspected
    except (OSError, ValueError) as error:
        result["tracing_inspection_error"] = str(error)
    command = ["journalctl", "--boot=-1", "--dmesg", "--no-pager", "--lines=120",
               "--grep=Out of memory|oom-kill|Killed process|invoked oom-killer|blocked for more than|soft lockup|hard LOCKUP|watchdog|I/O error"]
    try:
        child = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=4)
        result["previous_boot_resource_journal"] = {"exit_code": child.returncode,
            "output": child.stdout[:192 * 1024].decode(errors="replace"),
            "truncated": len(child.stdout) > 192 * 1024}
    except (OSError, subprocess.TimeoutExpired) as error:
        result["previous_boot_resource_journal"] = {"unavailable": str(error)}
    for name in ("meminfo", "pressure/memory", "pressure/cpu", "pressure/io"):
        try:
            result["proc/" + name] = (Path("/proc") / name).read_text()[:16384]
        except OSError as error:
            result["proc/" + name] = {"unavailable": str(error)}
    destination.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print("HOST_EVIDENCE_RECORDED", flush=True)


# Exact integration-test payload; written unchanged to the repair worktree.

RUST_TEST_SOURCE = b'// Bounded model/database contract checks. The original stress tests remain separate.\n// Seed 301 defines this small tape; this is not a replay of the 40,000-step run.\nuse bitcoinleveldb_batch::*;\nuse bitcoinleveldb_dbinterface::*;\nuse bitcoinleveldb_iteratorinner::*;\nuse bitcoinleveldb_modeldb::*;\nuse bitcoinleveldb_options::*;\nuse bitcoinleveldb_rand::*;\nuse bitcoinleveldb_slice::*;\nuse bitcoinleveldb_snapshot::*;\nuse bitcoinleveldbt_dbtest::*;\nuse bitcoinleveldbt_snapshot::*;\nuse std::collections::{BTreeMap, BTreeSet};\nuse std::path::{Component, Path};\n\nconst KEY_LIMIT: usize = 8;\nconst SEEDED_STEPS: usize = 24;\nconst MEMORY_LIMIT: u64 = 512 * 1024 * 1024;\n\nfn read_control(path: &Path) -> String {\n    match std::fs::read_to_string(path) {\n        Ok(value) => value.trim().to_owned(),\n        Err(error) => panic!("use the protected repair runner: cannot read {}: {}", path.display(), error),\n    }\n}\n\nfn control_number(value: &str, name: &str) -> u64 {\n    match value.parse::<u64>() {\n        Ok(number) => number,\n        Err(_) => panic!("use the protected repair runner: {} is not a finite limit", name),\n    }\n}\n\nfn verify_protection() {\n    assert!(matches!(std::env::var("BITCOIN_RS_RANDOMIZED_GUARD"), Ok(value) if value == "v1"),\n            "use the protected repair runner; this test requires verified cgroup limits");\n    let memberships = read_control(Path::new("/proc/self/cgroup"));\n    let unified = match memberships.lines().find_map(|line| line.strip_prefix("0::")) {\n        Some(value) => value,\n        None => panic!("use the protected repair runner: unified cgroup membership missing"),\n    };\n    assert!(Path::new(unified).is_absolute(), "invalid unified cgroup path");\n    assert!(Path::new(unified).components().all(|part| matches!(part, Component::RootDir | Component::Normal(_))),\n            "invalid unified cgroup path components");\n    let group = Path::new("/sys/fs/cgroup").join(unified.trim_start_matches(\'/\'));\n    let memory = control_number(&read_control(&group.join("memory.max")), "memory.max");\n    let pids = control_number(&read_control(&group.join("pids.max")), "pids.max");\n    assert!(memory > 0 && memory <= MEMORY_LIMIT, "use the protected repair runner: memory.max exceeds 512 MiB");\n    assert!(pids > 0 && pids <= 64, "use the protected repair runner: pids.max exceeds 64");\n    assert_eq!(read_control(&group.join("memory.swap.max")), "0", "use the protected repair runner: swap must be disabled");\n    assert_eq!(read_control(&group.join("memory.oom.group")), "1", "use the protected repair runner: grouped OOM handling required");\n    let cpu = read_control(&group.join("cpu.max"));\n    let fields: Vec<&str> = cpu.split_whitespace().collect();\n    assert_eq!(fields.len(), 2, "invalid cpu.max");\n    let quota = control_number(fields[0], "cpu.max quota");\n    let period = control_number(fields[1], "cpu.max period");\n    assert!(quota > 0 && period > 0 && quota <= period,\n            "use the protected repair runner: CPU quota must be at most one CPU");\n    eprintln!("BOUNDED_RANDOMIZED_GUARD cgroup={} memory={} pids={} cpu={}/{}", group.display(), memory, pids, quota, period);\n}\n\n#[derive(Default)]\nstruct Counts {\n    puts: usize,\n    deletes: usize,\n    batches: usize,\n    empty_batches: usize,\n    repeated_key_batches: usize,\n    snapshots: usize,\n    releases: usize,\n    snapshot_after_mutation_checks: usize,\n    reopens: usize,\n    flushes: usize,\n}\n\nenum Update<\'a> {\n    Put(&\'a str, &\'a str),\n    Delete(&\'a str),\n}\n\nstruct Case {\n    fixture: DBTest,\n    model: ModelDB,\n    model_snapshot: Option<Box<dyn Snapshot>>,\n    db_snapshot: Option<Box<dyn Snapshot>>,\n    expected: BTreeMap<String, String>,\n    saved_expected: Option<BTreeMap<String, String>>,\n    config: usize,\n    step: usize,\n    counts: Counts,\n}\n\nimpl Drop for Case {\n    fn drop(&mut self) {\n        // SnapshotImpl owns a node still linked into the database\'s snapshot list.\n        // Release it before DBTest::drop, including when an assertion unwinds.\n        self.release_inner();\n    }\n}\n\nfn collect_rows<D: DBNewIterator + ?Sized>(db: &mut D, snapshot: Option<&dyn Snapshot>, is_model: bool) -> Vec<(String, String)> {\n    let mut options = ReadOptions::default();\n    options.set_snapshot(snapshot.map(|value| {\n        if is_model {\n            snapshot_read_arc_from_snapshot_ref(value)\n        } else {\n            dbtest_snapshot_read_arc_from_snapshot_ref(value)\n        }\n    }));\n    let pointer = db.new_iterator(&options);\n    assert!(!pointer.is_null(), "new_iterator returned null");\n    let mut iterator = unsafe { Box::from_raw(pointer) };\n    iterator.seek_to_first();\n    let mut rows: Vec<(String, String)> = Vec::new();\n    while iterator.valid() {\n        assert!(rows.len() < KEY_LIMIT, "iterator exceeded the finite key universe");\n        let key_slice = iterator.key();\n        let value_slice = iterator.value();\n        assert!(key_slice.as_bytes().len() <= 200, "unexpected oversized iterator key");\n        assert!(value_slice.as_bytes().len() <= 64, "unexpected oversized iterator value");\n        let key = key_slice.to_string();\n        let value = value_slice.to_string();\n        if let Some((previous, _)) = rows.last() {\n            assert!(previous.as_bytes() < key.as_bytes(), "iterator did not advance in strict key order");\n        }\n        rows.push((key, value));\n        iterator.next();\n    }\n    assert!(iterator.status().is_ok(), "iterator ended with an error");\n    rows\n}\n\nimpl Case {\n    fn new(config: usize) -> Self {\n        verify_protection(); // No database, environment worker, or iterator exists before this succeeds.\n        eprintln!("BOUNDED_RANDOMIZED_PHASE config={} step=0 phase=construct.before", config);\n        let mut fixture = DBTest::default();\n        for _ in 0..config {\n            assert!(fixture.change_options(), "requested option configuration unavailable");\n        }\n        let options = fixture.current_options();\n        eprintln!("BOUNDED_RANDOMIZED_OPTIONS config={}", config);\n        let model = ModelDB::new(&options);\n        let case = Self { fixture, model, model_snapshot: None, db_snapshot: None,\n                          expected: BTreeMap::new(), saved_expected: None,\n                          config, step: 0, counts: Counts::default() };\n        case.mark("construct.after");\n        case\n    }\n\n    fn mark(&self, phase: &str) {\n        eprintln!("BOUNDED_RANDOMIZED_PHASE config={} step={} phase={}", self.config, self.step, phase);\n    }\n\n    fn check(&mut self) {\n        let expected: Vec<_> = self.expected.iter().map(|(key, value)| (key.clone(), value.clone())).collect();\n        self.mark("compare.live.model.before");\n        let model_rows = collect_rows(&mut self.model, None, true);\n        self.mark("compare.live.database.before");\n        let pointer = self.fixture.dbfull();\n        assert!(!pointer.is_null(), "database missing during comparison");\n        let database_rows = unsafe { collect_rows(&mut *pointer, None, false) };\n        assert_eq!(model_rows, expected, "model differs from the independent expected map");\n        assert_eq!(database_rows, expected, "database differs from the independent expected map");\n        if let Some(saved) = self.saved_expected.as_ref() {\n            let expected_saved: Vec<_> = saved.iter().map(|(key, value)| (key.clone(), value.clone())).collect();\n            assert!(self.model_snapshot.is_some() && self.db_snapshot.is_some(), "saved snapshot pair is incomplete");\n            self.mark("compare.saved.model.before");\n            let model_saved = collect_rows(&mut self.model, self.model_snapshot.as_deref(), true);\n            self.mark("compare.saved.database.before");\n            let database_saved = unsafe { collect_rows(&mut *pointer, self.db_snapshot.as_deref(), false) };\n            assert_eq!(model_saved, expected_saved, "model snapshot changed after acquisition");\n            assert_eq!(database_saved, expected_saved, "database snapshot changed after acquisition");\n            if saved != &self.expected {\n                self.counts.snapshot_after_mutation_checks += 1;\n            }\n        }\n        self.mark("compare.after");\n    }\n\n    fn put(&mut self, key: &str, value: &str) {\n        self.mark("put.before");\n        let options = WriteOptions::default();\n        assert!(self.model.put(&options, &Slice::from_str(key), &Slice::from_str(value)).is_ok(), "model put failed");\n        assert!(self.fixture.put(&key.to_owned(), &value.to_owned()).is_ok(), "database put failed");\n        self.expected.insert(key.to_owned(), value.to_owned());\n        self.counts.puts += 1;\n        self.step += 1;\n        self.mark("put.after");\n        self.check();\n    }\n\n    fn delete(&mut self, key: &str) {\n        self.mark("delete.before");\n        let options = WriteOptions::default();\n        assert!(self.model.delete(&options, &Slice::from_str(key)).is_ok(), "model delete failed");\n        assert!(self.fixture.delete(&key.to_owned()).is_ok(), "database delete failed");\n        self.expected.remove(key);\n        self.counts.deletes += 1;\n        self.step += 1;\n        self.mark("delete.after");\n        self.check();\n    }\n\n    fn batch(&mut self, operations: &[Update<\'_>]) {\n        self.mark("batch.before");\n        let mut batch = WriteBatch::default();\n        let mut distinct = BTreeSet::new();\n        let mut repeated = false;\n        for operation in operations {\n            let key = match operation {\n                Update::Put(key, value) => { batch.put(&Slice::from_str(key), &Slice::from_str(value)); *key }\n                Update::Delete(key) => { batch.delete(&Slice::from_str(key)); *key }\n            };\n            repeated |= !distinct.insert(key);\n        }\n        let options = WriteOptions::default();\n        assert!(self.model.write(&options, &mut batch as *mut WriteBatch).is_ok(), "model batch failed");\n        let pointer = self.fixture.dbfull();\n        assert!(!pointer.is_null(), "database missing during batch");\n        assert!(unsafe { (&mut *pointer).write(&options, &mut batch as *mut WriteBatch) }.is_ok(), "database batch failed");\n        for operation in operations {\n            match operation {\n                Update::Put(key, value) => { self.expected.insert((*key).to_owned(), (*value).to_owned()); }\n                Update::Delete(key) => { self.expected.remove(*key); }\n            }\n        }\n        self.counts.batches += 1;\n        self.counts.empty_batches += usize::from(operations.is_empty());\n        self.counts.repeated_key_batches += usize::from(repeated);\n        self.step += 1;\n        self.mark("batch.after");\n        self.check();\n    }\n\n    fn save(&mut self) {\n        assert!(self.model_snapshot.is_none() && self.db_snapshot.is_none(), "release the previous snapshot pair first");\n        self.mark("snapshot.acquire.before");\n        self.model_snapshot = Some(self.model.get_snapshot());\n        let pointer = self.fixture.dbfull();\n        assert!(!pointer.is_null(), "database missing during snapshot acquisition");\n        self.db_snapshot = Some(unsafe { (&mut *pointer).get_snapshot() });\n        self.saved_expected = Some(self.expected.clone());\n        self.counts.snapshots += 1;\n        self.mark("snapshot.acquire.after");\n        self.check();\n    }\n\n    fn release_inner(&mut self) {\n        if self.model_snapshot.is_some() || self.db_snapshot.is_some() {\n            self.mark("snapshot.release.before");\n        }\n        if let Some(snapshot) = self.model_snapshot.take() {\n            self.model.release_snapshot(snapshot);\n        }\n        if let Some(snapshot) = self.db_snapshot.take() {\n            let pointer = self.fixture.dbfull();\n            if pointer.is_null() {\n                // Do not free a potentially linked node through ordinary Box drop.\n                // Terminate this contained process instead of dereferencing a missing DB.\n                eprintln!("BOUNDED_RANDOMIZED_FATAL snapshot cleanup has no live database");\n                std::process::abort();\n            }\n            unsafe { (&mut *pointer).release_snapshot(snapshot); }\n        }\n        self.saved_expected = None;\n    }\n\n    fn release(&mut self) {\n        assert!(self.model_snapshot.is_some() && self.db_snapshot.is_some(), "release requires a complete snapshot pair");\n        self.release_inner();\n        self.counts.releases += 1;\n        self.mark("snapshot.release.after");\n    }\n\n    fn reopen(&mut self) {\n        assert!(self.model_snapshot.is_none() && self.db_snapshot.is_none(), "snapshots must be released before reopen");\n        self.mark("reopen.before");\n        self.fixture.reopen(None);\n        self.counts.reopens += 1;\n        self.mark("reopen.after");\n        self.check();\n    }\n\n    fn flush(&mut self) {\n        assert!(self.db_snapshot.is_some(), "flush case must retain a saved snapshot");\n        self.mark("flush.before");\n        assert!(dbtest_fixture_test_compact_memtable_status(&mut self.fixture).is_ok(), "forced memtable flush failed");\n        self.counts.flushes += 1;\n        self.mark("flush.after");\n        self.check();\n    }\n}\n\nfn run_case(config: usize) {\n    let keys = ["".to_owned(), "a".to_owned(), "\\0".to_owned(), "batch".to_owned(),\n                "c".to_owned(), "d".to_owned(), "x".repeat(200), "\xce\xba\xce\xbb\xce\xb5\xce\xb9\xce\xb4\xce\xaf".to_owned()];\n    assert_eq!(keys.len(), KEY_LIMIT);\n    let mut case = Case::new(config);\n    case.put("a", "old");\n    case.put(&keys[6], "long-old");\n    case.save();\n    case.put("a", "new");\n    case.put("", "");\n    case.put("\\0", "second");\n    case.put(&keys[7], "unicode-key");\n    case.delete(&keys[6]);\n    case.delete("never-inserted");\n    case.batch(&[]);\n    case.batch(&[Update::Put("a", "temporary"), Update::Delete("a"),\n                 Update::Put("a", "final"), Update::Put("batch", "batch-value")]);\n    case.flush();\n    case.release();\n    case.reopen();\n    case.save();\n    let mut random = Random::new(301);\n    for index in 0..SEEDED_STEPS {\n        let key_index = random.uniform(KEY_LIMIT as i32) as usize;\n        let other_index = random.uniform(KEY_LIMIT as i32) as usize;\n        let value = format!("seed301-{}-{}", index, random.uniform(1000));\n        match random.uniform(3) {\n            0 => case.put(&keys[key_index], &value),\n            1 => case.delete(&keys[key_index]),\n            _ => case.batch(&[Update::Put(&keys[key_index], "intermediate"),\n                             Update::Delete(&keys[other_index]), Update::Put(&keys[key_index], &value)]),\n        }\n        if index == 11 {\n            case.release();\n            case.reopen();\n            case.save();\n        }\n    }\n    case.check();\n    case.release();\n    assert!(case.counts.puts > 0 && case.counts.deletes > 0 && case.counts.batches > 0);\n    assert_eq!(case.counts.empty_batches, 1);\n    assert!(case.counts.repeated_key_batches > 0 && case.counts.snapshot_after_mutation_checks > 0);\n    assert_eq!(case.counts.snapshots, 3);\n    assert_eq!(case.counts.releases, 3);\n    assert_eq!(case.counts.reopens, 2);\n    assert_eq!(case.counts.flushes, 1);\n    assert_eq!(case.step, 10 + SEEDED_STEPS);\n    let steps = case.step;\n    case.mark("cleanup.before");\n    drop(case); // A blocked destructor is a timed-out case, never a successful marker.\n    eprintln!("BOUNDED_RANDOMIZED_OK config={} steps={}", config, steps);\n}\n\n#[test]\nfn bounded_default() { run_case(0); }\n#[test]\nfn bounded_reuse() { run_case(1); }\n#[test]\nfn bounded_filter() { run_case(2); }\n#[test]\nfn bounded_uncompressed() { run_case(3); }\n'


# Isolated Git workflow and bounded validation.

# Assembly provides Guard, BUDGETS, guard_child, collect_randomized_host_evidence,
# and the reviewed Rust integration test as RUST_TEST_SOURCE (bytes).
import argparse
import datetime as dt
import fcntl
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shlex
import sys
import traceback
import uuid

BASE = "a8beeac26124749a69d0ad6f7a999891f8be73f1"
WORKFLOW = "leveldb-randomized-bounded001"
BRANCH = "repair/leveldb-randomized-bounded"
ENV_SCRIPT = "contrib/repair/bitcoinrs-nix-development-shell001.py"
SCRIPT_REVISION = "002"
OLD_SCRIPT_SHA256 = "ae28b714b8ff3c97e298ff87eaba2abf4293a3c1bb7637323325b7b4f9a62c63"
ENV_SHA256 = "cd12ff326dc5aff610f3afede38c40ede2d4e46dd84801dd04a80a1faf6f0c97"
REPLAY_PATH = "contrib/repair/bitcoinleveldb-randomized-bounded001.py"
RECEIPT_PATH = "doc/repair/leveldb-randomized-bounded001.json"
CRATE = "bitcoinleveldbt-randomized"
BOUNDED_PATH = CRATE + "/tests/bounded_contracts.rs"
STRESS = {"randomized": "db_test_randomized", "bisect": "db_test_bisect_randomized"}
CASES = [("bounded_default", 0), ("bounded_reuse", 1), ("bounded_filter", 2), ("bounded_uncompressed", 3)]
PATHS = {REPLAY_PATH, RECEIPT_PATH, BOUNDED_PATH} | {CRATE + "/tests/" + name + ".rs" for name in STRESS}
BUILD = ["cargo", "test", "--locked", "--jobs", "1", "-p", CRATE, "--no-run", "--message-format=json"]
IGNORE = '#[test]\n#[ignore = "long stress; use a resource-controlled runner explicitly"]\n'


def trusted_devshell(origin):
    path = origin / ENV_SCRIPT
    if path.is_symlink() or hashlib.sha256(path.read_bytes()).hexdigest() != ENV_SHA256:
        raise RuntimeError("Accepted native-environment runner changed.")
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("accepted_development_shell001", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def stress_replacement(data, function):
    text = data.decode("utf-8")
    anchor = "#[traced_test]\nfn " + function + "("
    if text.count(anchor) != 1 or len(re.findall(r"\bfn\s+" + re.escape(function) + r"\s*\(", text)) != 1:
        raise RuntimeError("Stress test marker differs from the reviewed committed function: " + function)
    return text.replace(anchor, IGNORE + "fn " + function + "(", 1).encode()


def worker_guard(runner, final, head, files, old, pending, initial=False):
    runner.guard_operations()
    if (Path(runner.git("rev-parse", "--show-toplevel")).resolve() != runner.repo.resolve()
            or Path(runner.git("rev-parse", "--path-format=absolute", "--git-common-dir")).resolve() != Path(final.bootstrap_git(runner.origin, "rev-parse", "--path-format=absolute", "--git-common-dir")).resolve()):
        raise RuntimeError("Registered bounded worktree no longer belongs to the original repository.")
    if runner.git("symbolic-ref", "--quiet", "--short", "HEAD") != BRANCH or runner.git("rev-parse", "HEAD") != head:
        raise RuntimeError("Bounded repair branch advanced beyond its owned checkpoints.")
    dirty = final.names(runner, "diff", "--no-renames", "--name-only") | final.names(runner, "diff", "--cached", "--no-renames", "--name-only")
    untracked = final.names(runner, "ls-files", "--others", "--exclude-standard")
    if (dirty | untracked) - (PATHS if pending else set()):
        raise RuntimeError("Bounded worktree contains unrelated changes; all work retained.")
    configs = {".cargo/config", ".cargo/config.toml"}
    if any((runner.repo / name).exists() or (runner.repo / name).is_symlink() for name in configs - final.names(runner, "ls-files", "--", *sorted(configs))):
        raise RuntimeError("Bounded worktree has untracked/ignored Cargo configuration.")
    for name, expected in files.items():
        path = runner.repo / name
        if any(part.is_symlink() for part in [path, *path.parents] if part != runner.repo and runner.repo in part.parents):
            raise RuntimeError("Owned repair path contains a symlink: " + name)
        actual = path.read_bytes() if path.exists() else None
        allowed = {expected} | ({old[name]} if initial and name in old else set())
        if initial and name not in old:
            allowed.add(None)
        if actual not in allowed:
            raise RuntimeError("Unexpected bounded working content retained: " + name)
        indexed = final.blob(runner, "", name, False)
        allowed_index = {expected} | ({old[name]} if pending and name in old else {None} if pending else set())
        if indexed not in allowed_index:
            raise RuntimeError("Unexpected bounded staged content retained: " + name)


def own_commit(runner, final, head, files):
    return (runner.git("show", "-s", "--format=%P", head).split() == [BASE] and final.changed(runner, BASE, head) == PATHS
            and all(final.blob(runner, head, name, False) == data and runner.git("ls-tree", head, "--", name).startswith(("100755" if name == REPLAY_PATH else "100644") + " blob ") for name, data in files.items()))


def log_text(result):
    path = Path(result["log"])
    if path.stat().st_size > result["budget"]["output"]:
        raise RuntimeError("Guard log exceeds its declared output cap.")
    return path.read_bytes().decode(errors="replace")


def require_run(result):
    if result.get("outcome") != "exited" or result.get("verified") is not True or result.get("exit_code") != 0:
        raise RuntimeError("Resource-controlled phase did not complete successfully: " + str(result.get("label")) + ": " + str(result.get("outcome")) + ", exit=" + str(result.get("exit_code")))


def compact(result):
    return {key: value for key, value in result.items() if key not in {"tail", "command"}} | {"tail": result.get("tail", "")[-2000:]}


def artifacts_from(result, origin, worktree, final):
    found = {}
    expected = set(STRESS) | {"bounded_contracts"}
    for line in log_text(result).splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        target = item.get("target", {})
        name = target.get("name")
        if item.get("reason") != "compiler-artifact" or name not in expected or not item.get("executable"):
            continue
        if "test" not in target.get("kind", []) or item.get("profile", {}).get("test") is not True or Path(target.get("src_path", "")).resolve() != (worktree / CRATE / "tests" / (name + ".rs")).resolve():
            raise RuntimeError("Cargo returned an unexpected integration-test artifact.")
        path = Path(item["executable"])
        if not path.is_absolute() or path.is_symlink() or not path.resolve().is_relative_to((origin / "target").resolve()) or not path.is_file() or not path.stat().st_mode & 0o111:
            raise RuntimeError("Cargo test executable is not an ordinary executable under the shared target cache.")
        record = {"path": str(path), "fingerprint": final.fingerprint(path)}
        if name in found and found[name] != record:
            raise RuntimeError("Cargo returned multiple different test executables for one target.")
        found[name] = record
    if set(found) != expected:
        raise RuntimeError("Capped Cargo JSON did not identify all three expected integration-test executables.")
    return found


def case_summary(text, ordinal):
    rows = [tuple(map(int, match.groups())) for line in text.splitlines() if (match := re.match(r"^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;", line))]
    marker = "BOUNDED_RANDOMIZED_OK config=" + str(ordinal) + " steps=34"
    if rows != [(1, 0, 0)] or marker not in text:
        raise RuntimeError("Bounded case did not produce its single passing test and completion marker.")
    return {"passed": 1, "failed": 0, "ignored": 0, "marker": marker, "last_progress": text.splitlines()[-12:]}


def validate_receipt(receipt, identity):
    expected_lists = {name: [function] for name, function in STRESS.items()} | {"bounded_contracts": sorted(name for name, _ in CASES)}
    if (receipt.get("workflow") != WORKFLOW or receipt.get("base") != BASE or receipt.get("seed") != 301
            or receipt.get("rustflags") != "-Awarnings" or receipt.get("build_command") != BUILD or receipt.get("full_coverage_claimed") is not False or receipt.get("file_hashes") != identity["file_hashes"]
            or receipt.get("shell_sha256") != identity["shell_sha256"] or receipt.get("budgets") != BUDGETS
            or receipt.get("bounded_passed") != 4 or receipt.get("stress_ignored") != 2 or receipt.get("full_stress") != "not rerun"
            or hashlib.sha256(json.dumps(receipt.get("toolchain"), sort_keys=True).encode()).hexdigest() != identity["toolchain_sha256"]
            or receipt.get("protected_test_lists") != expected_lists or set(receipt.get("bounded_cases", {})) != {name for name, _ in CASES}):
        raise RuntimeError("Cached bounded receipt differs from the verified source/scope.")
    for name, ordinal in CASES:
        row = receipt["bounded_cases"][name]
        if (row.get("passed"), row.get("failed"), row.get("ignored"), row.get("marker")) != (1, 0, 0, "BOUNDED_RANDOMIZED_OK config=" + str(ordinal) + " steps=34"):
            raise RuntimeError("Cached bounded case evidence is incomplete.")
    expected = {"native-tools", "compile-tests"} | {"list-" + name for name in expected_lists} | {name for name, _ in CASES}
    if set(receipt.get("resource_readouts", {})) != expected:
        raise RuntimeError("Cached resource-controlled phase evidence is incomplete.")
    for label, result in receipt["resource_readouts"].items():
        kind = "build" if label in {"native-tools", "compile-tests"} else "case"
        if result.get("label") != label or result.get("kind") != kind or result.get("budget") != BUDGETS[kind]:
            raise RuntimeError("Cached resource phase identity/budget changed.")
        require_run(result)


def migrate_native_dispatch_failure(runner, final, module, metadata, state_path, state, identity, files, old, before):
    worktree = runner.origin.parent / (runner.origin.name + "-randomized-bounded-repair")
    layout_path = metadata / "worktree.json"
    if state_path.is_symlink() or layout_path.is_symlink():
        raise RuntimeError("Revision002 predecessor journals contain a symlink; retained for review.")
    layout = json.loads(layout_path.read_text())
    new_layout = dict(identity, origin=str(runner.origin), path=str(worktree))
    if all(state.get(key) == value for key, value in identity.items()) and layout == new_layout and state.get("upgrade", {}).get("status") != "in_progress":
        return state
    predecessor_identity = dict(identity, script_sha256=OLD_SCRIPT_SHA256,
                                file_hashes=dict(identity["file_hashes"], **{REPLAY_PATH: OLD_SCRIPT_SHA256}))
    predecessor_layout = dict(predecessor_identity, origin=str(runner.origin), path=str(worktree))
    if layout not in (predecessor_layout, new_layout):
        raise RuntimeError("Revision002 migration layout is not a known predecessor/current identity.")
    if worktree.is_symlink() or ("worktree " + str(worktree) + "\n") not in final.bootstrap_git(runner.origin, "worktree", "list", "--porcelain"):
        raise RuntimeError("Revision002 migration requires the recorded linked worktree.")
    previous = runner.repo
    runner.repo = worktree
    try:
        replay_path = worktree / REPLAY_PATH
        replay = replay_path.read_bytes()
        if final.sha(replay) not in {OLD_SCRIPT_SHA256, identity["script_sha256"]}:
            raise RuntimeError("Unknown canonical replay was retained; migration stopped.")
        check_files = dict(files, **{REPLAY_PATH: replay, RECEIPT_PATH: None})
        worker_guard(runner, final, BASE, check_files, old, True)
        for name, data in check_files.items():
            if data is not None and (worktree / name).stat().st_mode & 0o7777 != (0o755 if name == REPLAY_PATH else 0o644):
                raise RuntimeError("Unknown owned-file permissions were retained; migration stopped: " + name)
        if (final.names(runner, "diff", "--cached", "--no-renames", "--name-only")
                or final.names(runner, "diff", "--no-renames", "--name-only") != set(old)
                or final.names(runner, "ls-files", "--others", "--exclude-standard") != {BOUNDED_PATH, REPLAY_PATH}):
            raise RuntimeError("Revision002 migration requires exactly the known unstaged failure files and a clean index.")
        backup = metadata / "upgrade-revision002"
        if backup.is_symlink():
            raise RuntimeError("Revision002 backup directory is a symlink.")
        backup.mkdir(exist_ok=True, mode=0o700)
        paths = {"state": backup / "state.json", "layout": backup / "worktree.json", "replay": backup / "replay.py"}
        manifest_path = backup / "manifest.json"
        if any(path.is_symlink() for path in [*paths.values(), manifest_path]):
            raise RuntimeError("Revision002 backup evidence contains a symlink.")
        if not manifest_path.exists():
            if (any(state.get(key) != value for key, value in predecessor_identity.items())
                    or layout != predecessor_layout or final.sha(replay) != OLD_SCRIPT_SHA256):
                raise RuntimeError("Partial migration lacks complete predecessor backup evidence.")
            originals = {"state": state_path.read_bytes(), "layout": layout_path.read_bytes(), "replay": replay}
            for name, path in paths.items():
                if path.exists() and path.read_bytes() != originals[name]:
                    raise RuntimeError("Existing predecessor backup differs; no file was overwritten.")
                if not path.exists():
                    module.atomic_write(path, originals[name], 0o600)
        if not all(path.is_file() for path in paths.values()):
            raise RuntimeError("Revision002 predecessor backup evidence is incomplete.")
        backed = {name: path.read_bytes() for name, path in paths.items()}
        predecessor = json.loads(backed["state"])
        hashes = {name: final.sha(data) for name, data in backed.items()}
        manifest = {"workflow": WORKFLOW, "migration": "revision002", "from_identity": predecessor_identity,
                    "to_identity": identity, "backup_hashes": hashes}
        if final.sha(backed["replay"]) != OLD_SCRIPT_SHA256 or json.loads(backed["layout"]) != predecessor_layout:
            raise RuntimeError("Revision002 backup ownership evidence differs from the predecessor.")
        allowed = set(predecessor_identity) | {"phase", "userdata", "preflight", "readouts", "git_readouts"}
        native = predecessor.get("readouts", {}).get("native-tools", {})
        if (any(predecessor.get(key) != value for key, value in predecessor_identity.items()) or set(predecessor) - allowed
                or predecessor.get("phase") != "native-tools" or set(predecessor.get("readouts", {})) != {"native-tools"}
                or native.get("verified") is not True or native.get("outcome") != "exited" or native.get("exit_code") != 1
                or native.get("label") != "native-tools" or native.get("kind") != "build" or native.get("budget") != BUDGETS["build"]
                or "shell.nix" not in native.get("tail", "") or "is not a recognised command" not in native.get("tail", "")
                or set(predecessor.get("git_readouts", {})) != {"git-worktree-add"}):
            raise RuntimeError("Revision002 accepts only the known native-tools multicall dispatch failure before compilation.")
        for label, result, kind in (("preflight", predecessor.get("preflight", {}), "probe"),
                                    ("git-worktree-add", predecessor["git_readouts"]["git-worktree-add"], "build")):
            require_run(result)
            if result.get("label") != label or result.get("kind") != kind or result.get("budget") != BUDGETS[kind]:
                raise RuntimeError("Predecessor resource/ownership phase evidence differs.")
        if "GUARD_PREFLIGHT_OK" not in predecessor["preflight"].get("tail", ""):
            raise RuntimeError("Predecessor resource preflight completion evidence is absent.")
        if manifest_path.exists():
            if json.loads(manifest_path.read_text()) != manifest:
                raise RuntimeError("Revision002 backup manifest changed or belongs to another revision.")
        else:
            module.atomic_write(manifest_path, final.encoded(manifest), 0o600)
        upgrade = {"from_script_sha256": OLD_SCRIPT_SHA256, "to_script_sha256": identity["script_sha256"],
                   "backup_directory": str(backup), "backup_hashes": hashes,
                   "previous_userdata_sha256": final.sha(final.encoded(predecessor.get("userdata"))), "status": "in_progress"}
        upgraded = {key: value for key, value in predecessor.items() if key not in {"readouts", "preflight", "git_readouts"}}
        upgraded.update(identity, userdata=before, phase="pending", upgrade=upgrade)
        if state != predecessor:
            normalized = dict(state, userdata=before, upgrade=dict(state.get("upgrade", {}), status="in_progress"))
            if normalized != upgraded:
                raise RuntimeError("Partial Revision002 state has unknown changes; retained for review.")
        module.atomic_write(replay_path, files[REPLAY_PATH], 0o755)
        runner.save_state(state_path, upgraded)
        runner.save_state(layout_path, new_layout)
        upgraded["upgrade"] = dict(upgrade, status="complete")
        runner.save_state(state_path, upgraded)
        runner.report["journal_upgrade"] = upgraded["upgrade"]
        return upgraded
    finally:
        runner.repo = previous


def run_bounded(origin, script_path):
    runner = final = module = original_guard = None
    locks = []
    report = {"workflow": WORKFLOW, "script_revision": SCRIPT_REVISION, "outcome": "failed", "phase": "preflight", "base": BASE, "branch": BRANCH, "full_stress": "not rerun", "merge": "not requested", "tag": "not requested", "full_coverage_claimed": False}
    failure = None
    try:
        dev = trusted_devshell(origin)
        final = dev.trusted_finalizer(origin)
        if Path(final.bootstrap_git(origin, "rev-parse", "--show-toplevel")).resolve() != origin or final.bootstrap_git(origin, "rev-parse", "HEAD") != BASE:
            raise RuntimeError("Original checkout must remain at the reviewed native-environment checkpoint.")
        original_branch = final.bootstrap_git(origin, "symbolic-ref", "--quiet", "--short", "HEAD")
        common = Path(final.bootstrap_git(origin, "rev-parse", "--path-format=absolute", "--git-common-dir")) / "codex-repairs"
        metadata = common / WORKFLOW
        metadata.mkdir(parents=True, exist_ok=True)
        for path in (common / final.REPAIR / "run.lock", common / dev.WORKFLOW / "run.lock", metadata / "run.lock"):
            lock = path.open("a+"); locks.append(lock)
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        module, runner = final.trusted_coverage(origin).load_repair_runner(origin, origin)
        directory = metadata / "runs" / (dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])
        directory.mkdir(parents=True)
        runner.directory, runner.log, runner.report = directory, (directory / "run.log").open("w", buffering=1), report
        report.update(origin_repository=str(origin), original_branch=original_branch, run_directory=str(directory), budgets=BUDGETS)
        env_state = json.loads((common / dev.WORKFLOW / "state.json").read_text())
        shell = final.blob(runner, BASE, "shell.nix")
        if (env_state.get("phase") != "complete" or env_state.get("checkpoint") != BASE or env_state.get("script_sha256") != ENV_SHA256
                or env_state.get("tests_validated") is not True or not dev.tests_valid(env_state.get("tests", {}))
                or env_state.get("make_validated") is not True or env_state.get("make", {}).get("exit_code") != 0
                or shell != dev.shell_bytes(env_state["pin"]) or (origin / "shell.nix").read_bytes() != shell
                or not dev.own_commit(runner, final, BASE, {"shell.nix": shell, ENV_SCRIPT: (origin / ENV_SCRIPT).read_bytes()})):
            raise RuntimeError("Validated native environment differs from the supplied checkpoint and journal.")
        tag_oid = runner.git("rev-parse", "refs/tags/" + final.TAG)
        if env_state.get("previous_tag_object") != tag_oid:
            raise RuntimeError("Previous snapshot repair tag changed; it was not rewritten.")
        final.environment_guard(runner)
        runner.guard_operations()
        before = final.userdata(runner, PATHS)
        script = script_path.read_bytes()
        compiler = json.loads((common / final.REPAIR / "boundary-tests001-state.json").read_text())["measurement"]["compiler"]
        old = {CRATE + "/tests/" + name + ".rs": final.blob(runner, BASE, CRATE + "/tests/" + name + ".rs") for name in STRESS}
        files = {name: stress_replacement(old[name], function) for name, function in ((CRATE + "/tests/" + key + ".rs", value) for key, value in STRESS.items())}
        if not isinstance(RUST_TEST_SOURCE, bytes) or not RUST_TEST_SOURCE:
            raise RuntimeError("Reviewed bounded Rust test payload is unavailable.")
        if any(final.blob(runner, BASE, name, False) is not None for name in (BOUNDED_PATH, REPLAY_PATH, RECEIPT_PATH)):
            raise RuntimeError("A new bounded repair artifact already exists in the reviewed base.")
        files.update({BOUNDED_PATH: RUST_TEST_SOURCE, REPLAY_PATH: script})
        identity = {"workflow": WORKFLOW, "base": BASE, "branch": BRANCH, "original_branch": original_branch, "script_sha256": final.sha(script), "shell_sha256": final.sha(shell), "toolchain_sha256": hashlib.sha256(json.dumps(compiler, sort_keys=True).encode()).hexdigest(), "file_hashes": {name: final.sha(data) for name, data in files.items()}}
        state_path = metadata / "state.json"
        state = json.loads(state_path.read_text()) if state_path.exists() else dict(identity, phase="pending")
        if state_path.exists() and (metadata / "worktree.json").exists():
            state = migrate_native_dispatch_failure(runner, final, module, metadata, state_path, state, identity, files, old, before)
        if any(state.get(key) != value for key, value in identity.items()) or state.get("blocked_checkpoint") or state.get("userdata", before) != before:
            raise RuntimeError("Bounded repair journal/input/user-data mismatch; no work discarded.")
        state["userdata"] = before
        if state.get("validated"):
            if state["validated"] is not True:
                raise RuntimeError("Cached validation flag is invalid.")
            validate_receipt(state.get("receipt", {}), identity)
        files[RECEIPT_PATH] = final.encoded(state["receipt"]) if state.get("validated") else None
        inputs = final.integration_inputs(runner, BASE)
        def original_guard():
            previous = runner.repo; runner.repo = origin
            try:
                runner.guard_operations()
                if runner.git("symbolic-ref", "--quiet", "--short", "HEAD") != original_branch or runner.git("rev-parse", "HEAD") != BASE or final.userdata(runner, PATHS) != before:
                    raise RuntimeError("Original branch, HEAD or user-owned state changed; retained for review.")
                if (final.names(runner, "diff", "--no-renames", "--name-only", BASE) | final.names(runner, "diff", "--cached", "--no-renames", "--name-only", BASE)) & inputs.keys():
                    raise RuntimeError("Original Rust/build inputs changed.")
                if runner.git("rev-parse", "refs/tags/" + final.TAG) != tag_oid:
                    raise RuntimeError("Previous repair tag changed.")
            finally:
                runner.repo = previous
        original_guard()
        worktree = origin.parent / (origin.name + "-randomized-bounded-repair")
        layout_path = metadata / "worktree.json"
        layout = dict(identity, origin=str(origin), path=str(worktree))
        branch = runner.capture(["git", "rev-parse", "--verify", "refs/heads/" + BRANCH], check=False)
        if layout_path.exists():
            if json.loads(layout_path.read_text()) != layout:
                raise RuntimeError("Bounded worktree ownership journal changed.")
        elif worktree.exists() or branch.returncode == 0:
            raise RuntimeError("Existing bounded branch/worktree has no matching ownership journal.")
        else:
            runner.save_state(layout_path, layout)
        runner.save_state(state_path, state)
        guard = None
        def ensure_guard():
            nonlocal guard
            if guard is None:
                report["phase"] = state["phase"] = "resource_preflight"
                guard = Guard(directory / "guard", script_path, runner.env)
                state["preflight"] = report["preflight"] = compact(guard.preflight())
                runner.save_state(state_path, state); require_run(state["preflight"])
                if "GUARD_PREFLIGHT_OK" not in log_text(state["preflight"]):
                    raise RuntimeError("Guard preflight completion marker is absent.")
            return guard
        def guarded_git(label, command, cwd):
            controlled = ensure_guard()
            report["phase"] = state["phase"] = label
            result = controlled.run(label, command, cwd, {}, kind="build")
            state.setdefault("git_readouts", {})[label] = report.setdefault("git_readouts", {})[label] = compact(result)
            runner.save_state(state_path, state); require_run(result)
        if not state.get("validated"):
            ensure_guard()
            evidence_path = directory / "host-evidence.json"
            report["host_evidence_guard"] = compact(guard.run("host-evidence", [sys.executable, str(script_path), "--host-evidence", str(origin), str(evidence_path)], origin, {}, kind="probe"))
            try:
                if evidence_path.is_file() and evidence_path.stat().st_size <= 768 * 1024:
                    report["host_evidence"] = json.loads(evidence_path.read_text())
            except (OSError, ValueError) as error:
                report["host_evidence_error"] = str(error)
        registered = final.bootstrap_git(origin, "worktree", "list", "--porcelain")
        if ("worktree " + str(worktree) + "\n") not in registered:
            if worktree.exists() or (branch.returncode == 0 and branch.stdout.decode().strip() != BASE):
                raise RuntimeError("Incomplete bounded worktree setup needs review.")
            guarded_git("git-worktree-add", ["git", "worktree", "add"] + (["-b", BRANCH, str(worktree), BASE] if branch.returncode else [str(worktree), BRANCH]), origin)
        if worktree.is_symlink():
            raise RuntimeError("Bounded worktree path is a symlink.")
        runner.repo = worktree
        checkpoint = state.get("checkpoint")
        head = runner.git("rev-parse", "HEAD")
        if not checkpoint and head != BASE:
            previous_commit_run = state.get("git_readouts", {}).get("git-commit")
            if previous_commit_run:
                require_run(previous_commit_run)
            if not state.get("validated") or not own_commit(runner, final, head, files):
                raise RuntimeError("Bounded branch has an unreviewed commit.")
            checkpoint = state["checkpoint"] = head
        if checkpoint and not own_commit(runner, final, checkpoint, files):
            raise RuntimeError("Bounded checkpoint differs from its exact parent/files/blobs/modes.")
        worker_guard(runner, final, checkpoint or BASE, files, old, not checkpoint, initial=not checkpoint)
        report.update(worktree=str(worktree), validation_reused=bool(state.get("validated")), userdata_before_sha256=final.sha(final.encoded(before)))
        if not checkpoint:
            for name, data in files.items():
                if data is not None:
                    module.atomic_write(worktree / name, data, 0o755 if name == REPLAY_PATH else 0o644)
        if not state.get("validated"):
            worker_guard(runner, final, BASE, files, old, True)
            nix = ["nix-shell", "--option", "max-jobs", "0", "--option", "builders", "", "--option", "substitute", "false", str(worktree / "shell.nix"), "--run"]
            env = {"CARGO_TARGET_DIR": str(origin / "target"), "CARGO_BUILD_JOBS": "1", "RUST_BACKTRACE": "1", "RUSTFLAGS": "-Awarnings", "CARGO_NET_GIT_FETCH_WITH_CLI": "true", "GIT_CONFIG_COUNT": "1", "GIT_CONFIG_KEY_0": "url.https://github.com/.insteadOf", "GIT_CONFIG_VALUE_0": "git@github.com:"}
            def phase(label, command, kind="case", extra=None, tmpdir=None):
                report["phase"] = state["phase"] = label; runner.say("Bounded phase: " + label)
                result = guard.run(label, command, worktree, dict(env, **(extra or {})), kind=kind, tmpdir=tmpdir)
                state.setdefault("readouts", {})[label] = report.setdefault("readouts", {})[label] = compact(result)
                if label == "compile-tests" and result.get("log"):
                    diagnostics = []
                    for line in log_text(result).splitlines():
                        try:
                            message = json.loads(line).get("message", {})
                            if message.get("level") == "error":
                                diagnostics.append(message.get("rendered", message.get("message", "")))
                        except (ValueError, AttributeError):
                            pass
                    if diagnostics:
                        report["build_diagnostics"] = "\n".join(diagnostics)[-32000:]
                runner.save_state(state_path, state); require_run(result)
                original_guard(); worker_guard(runner, final, BASE, files, old, True)
                return result
            native = phase("native-tools", nix + ["rustc -Vv && cargo -V"], "build")
            if compiler["rustc_vv"] not in log_text(native) or compiler["cargo_version"] not in log_text(native).splitlines():
                raise RuntimeError("Bounded build compiler/Cargo differs from the accepted native environment.")
            built = phase("compile-tests", nix + [shlex.join(BUILD)], "build")
            artifacts = artifacts_from(built, origin, worktree, final)
            state["artifacts"] = report["artifacts"] = artifacts
            listings, cases = {}, {}
            for name, expected in [(key, {value}) for key, value in STRESS.items()] + [("bounded_contracts", {name for name, _ in CASES})]:
                path = Path(artifacts[name]["path"])
                if final.fingerprint(path) != artifacts[name]["fingerprint"]:
                    raise RuntimeError("Compiled test executable changed before protected metadata inspection.")
                result = phase("list-" + name, [str(path), "--list"] + (["--ignored"] if name in STRESS else []))
                names = {match[1] for line in log_text(result).splitlines() if (match := re.match(r"^(.+): test$", line))}
                if names != expected:
                    raise RuntimeError("Protected test metadata differs from the required ordinary/ignored test set: " + name)
                listings[name] = sorted(names)
            for name, ordinal in CASES:
                path = Path(artifacts["bounded_contracts"]["path"])
                if final.fingerprint(path) != artifacts["bounded_contracts"]["fingerprint"]:
                    raise RuntimeError("Bounded test executable changed before a case.")
                result = phase(name, [str(path), "--exact", name, "--nocapture", "--test-threads=1"], extra={"BITCOIN_RS_RANDOMIZED_GUARD": "v1", "TEST_RANDOM_SEED": "301"}, tmpdir=directory / ("case-data-" + name))
                cases[name] = case_summary(log_text(result), ordinal)
                state["cases"] = report["cases"] = cases; runner.save_state(state_path, state)
            if final.fingerprint(Path(artifacts["bounded_contracts"]["path"])) != artifacts["bounded_contracts"]["fingerprint"]:
                raise RuntimeError("Bounded test executable changed during validation.")
            receipt = {"workflow": WORKFLOW, "base": BASE, "seed": 301, "rustflags": "-Awarnings", "build_command": BUILD, "toolchain": compiler, "shell_sha256": final.sha(shell), "budgets": BUDGETS, "bounded_cases": cases, "protected_test_lists": listings, "bounded_passed": 4, "stress_ignored": 2, "full_stress": "not rerun", "full_coverage_claimed": False, "file_hashes": identity["file_hashes"], "resource_readouts": state["readouts"]}
            state.update(validated=True, receipt=receipt, phase="validated"); runner.save_state(state_path, state)
            files[RECEIPT_PATH] = final.encoded(receipt)
        if not checkpoint:
            report["phase"] = "validated_checkpoint"
            module.atomic_write(worktree / RECEIPT_PATH, files[RECEIPT_PATH], 0o644)
            for name in files:
                runner.git("add", "--chmod=" + ("+x" if name == REPLAY_PATH else "-x"), "--", name)
            worker_guard(runner, final, BASE, files, old, True); original_guard()
            message = directory / "commit-message.txt"
            message.write_text("test: bound randomized regression checks and retain opt-in stress\n\nFour resource-controlled bounded contracts passed. Two long stress tests are ignored by default; full stress was not rerun.\nNo full coverage claim. Original checkout and Makefile preserved.\n\nRepair-Workflow: " + WORKFLOW + "\nBase: " + BASE + "\n")
            guarded_git("git-commit", ["git", "commit", "--file", str(message)], worktree)
            checkpoint = runner.git("rev-parse", "HEAD")
            if not own_commit(runner, final, checkpoint, files):
                state["blocked_checkpoint"] = checkpoint; runner.save_state(state_path, state)
                raise RuntimeError("Bounded commit exceeded its exact five-file scope.")
        worker_guard(runner, final, checkpoint, files, old, False); original_guard()
        state.update(checkpoint=checkpoint, phase="complete"); runner.save_state(state_path, state)
        report.update(outcome="passed", phase="complete", checkpoint=checkpoint, repair_head=checkpoint, cases=state["receipt"]["bounded_cases"], protected_test_lists=state["receipt"]["protected_test_lists"], original_preserved=True, userdata_after_sha256=final.sha(final.encoded(final.userdata(runner, PATHS))))
    except KeyboardInterrupt:
        failure = "Interrupted; logs and owned files retained. No unrestricted retry or incomplete checkpoint was made."
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
                if original_guard:
                    try:
                        original_guard(); report["original_preserved"] = True
                    except Exception as preservation_error:
                        report.update(original_preserved=False, preservation_error=str(preservation_error))
                report.update(original_head=final.bootstrap_git(origin, "rev-parse", "HEAD"), original_status=final.bootstrap_git(origin, "status", "--short"), return_file=str(directory / "BOUNDED_RETURN.txt"))
                module.atomic_write(Path(report["return_file"]), final.encoded(report))
            except Exception as error:
                report.update(outcome="failed", finalization_error=str(error))
            runner.log.close()
        for lock in reversed(locks):
            lock.close()
        print("BEGIN_BOUNDED_RETURN"); print(json.dumps(report, indent=2)); print("END_BOUNDED_RETURN")
        if report.get("return_file"):
            print("Return file: " + report["return_file"])
    return 0 if report["outcome"] == "passed" else 1


def main():
    if len(sys.argv) == 3 and sys.argv[1] == "--guard-child":
        return guard_child(sys.argv[2])
    if len(sys.argv) == 4 and sys.argv[1] == "--host-evidence":
        collect_randomized_host_evidence(Path(sys.argv[2]), Path(sys.argv[3])); return 0
    parser = argparse.ArgumentParser(description="Run four resource-controlled randomized contracts on an isolated repair branch.")
    parser.add_argument("--repo", type=Path, default=Path.home() / "bitcoin-rs")
    args = parser.parse_args()
    return run_bounded(args.repo.expanduser().resolve(), Path(__file__).resolve())


if __name__ == "__main__":
    sys.exit(main())
