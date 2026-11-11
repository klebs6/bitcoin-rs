#!/usr/bin/env python3
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
# Explicit routine-runner profiles; all existing containment limits are retained.
BUDGETS['source'] = dict(memory=512*MIB, high=384*MIB, tasks=32, seconds=180,
                         file=32*MIB, output=MIB)
BUDGETS['case'].update(data=128*MIB, entries=4096)
BUDGETS['stress'] = dict(memory=2048*MIB, high=1536*MIB, tasks=64, seconds=600,
                         file=256*MIB, output=MIB, data=512*MIB, entries=4096)

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


def _tree_usage(root, max_bytes=128*MIB, max_entries=4096):
    size = count = 0
    todo = [Path(root)]
    try:
        while todo:
            with os.scandir(todo.pop()) as entries:
                for entry in entries:
                    count += 1
                    if count > max_entries:
                        return size, count
                    info = entry.stat(follow_symlinks=False)
                    size += info.st_size
                    if size > max_bytes:
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
                str(Path(sys.executable).resolve()), "-I", "-S", str(self.script_path), "--guard-child", str(plan_path)]
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
                        if kind in {"case", "stress"}:
                            data_limit = budget.get("data", 128*MIB)
                            entry_limit = budget.get("entries", 4096)
                            size, count = _tree_usage(data, data_limit, entry_limit)
                            result["data_bytes"], result["data_entries"] = size, count
                            if size > data_limit or count > entry_limit:
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


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "--guard-child":
        sys.exit(guard_child(sys.argv[2]))
    raise SystemExit("Internal resource controller; use a crate verification command.")
