#!/usr/bin/env python3
"""Repair audited internal SparseMerge Env ownership in a separate worktree.

Offline bounded compilation and normal correctness tests only. No debugger,
allocator experiments, merge, tag, remote operation or original-cache overwrite.
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
import shlex
import shutil
import signal
import stat
import sys
import time
import types
import uuid
MIB=1024*1024
MAX_FILE=8*MIB
MAX_REPORT=32*MIB
WORKFLOW='leveldb-sparsemerge-ownership011'
MASTER='3d099856f684b262201329722a9087fbc35ccc1d'
BORROW='e1736f59dc8e10b301cd4c0c3ea10302a9c46e1a'
BORROW_REF='refs/heads/repair/leveldb-sparsemerge-borrow'
BRANCH='repair/leveldb-sparsemerge-ownership'
WORKTREE_SUFFIX='-sparsemerge-ownership-repair011'
REPLAY_PATH='contrib/repair/bitcoinleveldb-sparsemerge-ownership011.py'
DOCUMENT_PATH='doc/testing/leveldb-sparsemerge-ownership011.md'
RECEIPT_PATH='doc/repair/leveldb-sparsemerge-ownership011.json'
SOURCE_RECEIPT_REL='codex-repairs/leveldb-sparsemerge-ownership-source010/runs/20261107T163531Z-32619697/OWNERSHIP_SOURCE_RETURN.json'
SOURCE_RECEIPT_SHA='e2f0768b8b8a4d8d16c7214b6c5906f947f739f58ba75c2d3a6a3079a2bd417b'
ARTIFACT_REL='codex-repairs/leveldb-issue178-diagnostic001/cargo-target/debug/deps/bitcoinleveldbt_sparsemerge-bc41809dbee003f8'
ARTIFACT={'kind':'file','mode':493,'size':60997840,'sha256':'0c6d230de0dbd87547e7f9e86f33e86b0940b7ccf978b606dcfa6ffe2aa75dcc'}
PACKAGES=('bitcoinleveldb-options','bitcoinleveldb-tablecache','bitcoinleveldb-dbimpl','bitcoinleveldb-versionset','bitcoinleveldbt-sparsemerge')
CASES=(
 ('bitcoinleveldb_options','table_snapshot_preserves_settings_without_env_owner',None,'case',1),
 ('bitcoinleveldb_options','ordinary_options_clone_keeps_env_ownership',None,'case',1),
 ('bitcoinleveldb_tablecache','cached_table_open_and_eviction_preserve_env_owner_count',None,'case',1),
 ('bitcoinleveldb_tablecache','build_table_with_empty_iterator_produces_no_file',None,'case',1),
 ('bitcoinleveldb_tablecache','build_table_with_single_entry_builds_and_verifies_table',None,'case',1),
 ('bitcoinleveldb_tablecache','build_table_propagates_iterator_status_on_error',None,'case',1),
 ('bitcoinleveldb_dbimpl','write_level_0table_returns_error_when_options_env_is_none',None,'case',1),
 ('bitcoinleveldb_dbimpl','make_room_for_write_returns_error_when_options_env_is_none_without_touching_memtable',None,'case',1),
 ('bitcoinleveldb_versionset','log_and_apply_persists_edit_so_recover_sees_it',None,'case',1),
 ('bitcoinleveldb_versionset','log_and_apply_multiple_edits_monotonically_increase_file_numbers',None,'case',1),
 ('bitcoinleveldbt_sparsemerge','db_test_env_borrow_gate_rotation','SPARSEMERGE_CONTRACT_OK case=db_test_env_borrow_gate_rotation','case',1),
 ('bitcoinleveldbt_sparsemerge','db_test_sparse_merge','SPARSEMERGE_CONTRACT_OK case=db_test_sparse_merge','stress',3),
)
BASE_GUARD_SHA='b823684a678ee02ea7390ebf31ab92c458ff3cd4eaa64d79908a12c420fbb9cc'
GUARD_SHA='3accdfb32cc1904c6177361a05afcc0b6ade8ea14fc8df728a9e477048e1b9aa'
GUARD_SOURCE=b'"""Fail-closed systemd user-service containment for build and test children."""\nimport collections\nimport json\nimport os\nfrom pathlib import Path\nimport re\nimport resource\nimport selectors\nimport shutil\nimport subprocess\nimport sys\nimport time\nimport uuid\n\nMIB = 1024 * 1024\nBUDGETS = {\n    "build": dict(memory=4096*MIB, high=3072*MIB, tasks=128, seconds=600, file=512*MIB, output=2*MIB),\n    "case": dict(memory=512*MIB, high=384*MIB, tasks=64, seconds=30, file=16*MIB, output=MIB),\n    "probe": dict(memory=128*MIB, high=96*MIB, tasks=32, seconds=10, file=16*MIB, output=MIB),\n}\nPROPERTIES = ("LoadState", "ActiveState", "SubState", "Type", "Restart", "RemainAfterExit", "Result",\n              "ExecMainCode", "ExecMainStatus", "RuntimeMaxUSec", "TimeoutStopUSec", "KillMode",\n              "OOMPolicy", "SendSIGKILL", "MemoryMax", "MemoryHigh", "MemorySwapMax", "TasksMax",\n              "CPUQuotaPerSecUSec", "MemoryPeak", "CPUUsageNSec", "ControlGroup")\n\n\ndef _atomic_json(path, value):\n    path = Path(path)\n    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)\n    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)\n    with os.fdopen(fd, "w") as stream:\n        json.dump(value, stream, sort_keys=True, indent=2)\n        stream.write("\\n")\n        stream.flush()\n        os.fsync(stream.fileno())\n    os.replace(temporary, path)\n\n\ndef _events(path):\n    try:\n        return {key: int(value) for key, value in (line.split() for line in Path(path).read_text().splitlines())}\n    except (OSError, ValueError):\n        return None\n\n\ndef _cgroup_snapshot(path):\n    result = {"path": str(path)}\n    for name in ("memory.max", "memory.high", "memory.swap.max", "cpu.max", "pids.max", "memory.oom.group", "memory.current", "memory.peak"):\n        try:\n            result[name] = (path / name).read_text().strip()\n        except OSError:\n            result[name] = None\n    result["memory.events"] = _events(path / "memory.events")\n    return result\n\n\ndef _headroom(path, budget):\n    """Conservative admission policy; observations can race later allocations."""\n    root = Path("/sys/fs/cgroup").resolve()\n    required = budget["memory"] + 256*MIB\n    match = re.search(r"^MemAvailable:\\s+(\\d+)\\s+kB\\s*$", Path("/proc/meminfo").read_text(), re.MULTILINE)\n    if not match:\n        raise RuntimeError("MemAvailable is unavailable; workload admission was refused.")\n    result = {"required_bytes": required, "mem_available_bytes": int(match.group(1))*1024, "ancestors": []}\n    if result["mem_available_bytes"] < required:\n        raise RuntimeError("Host memory headroom is below the declared admission margin: " + repr(result))\n    ancestor = path.parent\n    while ancestor.is_relative_to(root):\n        row = {"path": str(ancestor)}\n        try:\n            maximum = (ancestor / "memory.max").read_text().strip()\n        except OSError:\n            if ancestor != root:\n                raise RuntimeError("Ancestor memory.max is unreadable: " + str(ancestor))\n            maximum = None\n        try:\n            current = int((ancestor / "memory.current").read_text().strip())\n        except (OSError, ValueError):\n            if ancestor != root or maximum is not None:\n                raise RuntimeError("Ancestor memory.current is unreadable: " + str(ancestor))\n            current = None\n        row.update(maximum=maximum, current=current)\n        result["ancestors"].append(row)\n        if maximum not in (None, "max"):\n            try:\n                remaining = int(maximum) - current\n            except (TypeError, ValueError):\n                raise RuntimeError("Ancestor memory limit has an invalid value: " + str(ancestor))\n            row["remaining_bytes"] = remaining\n            if remaining < required:\n                raise RuntimeError("Ancestor memory headroom is below the declared admission margin: " + repr(result))\n        if ancestor == root:\n            break\n        ancestor = ancestor.parent\n    return result\n\n\ndef _kernel_probe(budget):\n    roots = [line[3:] for line in Path("/proc/self/cgroup").read_text().splitlines() if line.startswith("0::")]\n    if len(roots) != 1 or not Path("/sys/fs/cgroup/cgroup.controllers").is_file():\n        raise RuntimeError("A cgroup-v2 process membership and controller hierarchy are required.")\n    root = Path("/sys/fs/cgroup").resolve()\n    path = (root / roots[0].lstrip("/")).resolve()\n    if not path.is_relative_to(root):\n        raise RuntimeError("Unexpected cgroup path.")\n    result = _cgroup_snapshot(path)\n    expected = {"memory.max": str(budget["memory"]), "memory.high": str(budget["high"]),\n                "memory.swap.max": "0", "pids.max": str(budget["tasks"]), "memory.oom.group": "1"}\n    if any(result[key] != value for key, value in expected.items()):\n        raise RuntimeError("Kernel memory/swap/task/OOM controls differ from the requested policy: " + repr(result))\n    quota, period = result["cpu.max"].split()\n    if quota == "max" or int(quota) <= 0 or int(period) <= 0 or int(quota) > int(period):\n        raise RuntimeError("Kernel CPU quota is absent or exceeds one CPU.")\n    result["rlimit_core"] = list(resource.getrlimit(resource.RLIMIT_CORE))\n    result["rlimit_fsize"] = list(resource.getrlimit(resource.RLIMIT_FSIZE))\n    if result["rlimit_core"] != [0, 0] or result["rlimit_fsize"] != [budget["file"], budget["file"]]:\n        raise RuntimeError("Core/file-size resource limits were not installed.")\n    result["headroom"] = _headroom(path, budget)\n    return result\n\n\ndef guard_child(plan_path):\n    """Internal entry point; no requested command executes before both checks pass."""\n    plan = json.loads(Path(plan_path).read_text())\n    probe_path = Path(plan["probe"])\n    try:\n        budget = BUDGETS[plan["kind"]]\n        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))\n        resource.setrlimit(resource.RLIMIT_FSIZE, (budget["file"], budget["file"]))\n        snapshot = _kernel_probe(budget)\n        _atomic_json(probe_path, {"ok": True, "pid": os.getpid(), "kernel": snapshot})\n        until = time.monotonic() + 6\n        permit = Path(plan["permit"])\n        while not permit.exists():\n            if time.monotonic() >= until:\n                raise RuntimeError("Parent did not authorize execution after verifying service properties.")\n            time.sleep(0.025)\n        if permit.read_text() != plan["nonce"]:\n            raise RuntimeError("Execution authorization does not match this child.")\n        os.chdir(plan["cwd"])\n        os.execvpe(plan["command"][0], plan["command"], plan["env"])\n    except BaseException as error:\n        _atomic_json(probe_path, {"ok": False, "error": type(error).__name__ + ": " + str(error)})\n        print("GUARD_CHILD_REFUSED: " + str(error), file=sys.stderr)\n        return 78\n\n\ndef _seconds(value):\n    if value is None or value == "infinity":\n        return None\n    if value.isdigit():\n        return int(value) / 1000000\n    units = {"us": 0.000001, "ms": 0.001, "s": 1, "min": 60, "h": 3600, "d": 86400, "w": 604800}\n    compact = value.replace(" ", "").replace("\xc2\xb5s", "us").replace("\xce\xbcs", "us")\n    parts = re.findall(r"(\\d+(?:\\.\\d+)?)(us|ms|min|s|h|d|w)", compact)\n    if not parts or "".join(number + unit for number, unit in parts) != compact:\n        return None\n    return sum(float(number) * units[unit] for number, unit in parts)\n\n\ndef _service_controls(properties, budget):\n    expected = {"Type": "exec", "Restart": "no", "RemainAfterExit": "no", "KillMode": "control-group", "OOMPolicy": "kill", "SendSIGKILL": "yes"}\n    if any(properties.get(key) != value for key, value in expected.items()):\n        return False\n    runtime = _seconds(properties.get("RuntimeMaxUSec"))\n    stop = _seconds(properties.get("TimeoutStopUSec"))\n    return runtime is not None and 0 < runtime <= budget["seconds"] + 0.001 and stop is not None and 0 < stop <= 3.001\n\n\ndef _tree_usage(root, max_bytes=128*MIB, max_entries=4096):\n    size = count = 0\n    todo = [Path(root)]\n    try:\n        while todo:\n            with os.scandir(todo.pop()) as entries:\n                for entry in entries:\n                    count += 1\n                    if count > max_entries:\n                        return size, count\n                    info = entry.stat(follow_symlinks=False)\n                    size += info.st_size\n                    if size > max_bytes:\n                        return size, count\n                    if entry.is_dir(follow_symlinks=False):\n                        todo.append(Path(entry.path))\n    except FileNotFoundError:\n        pass\n    return size, count\n\n\nclass Guard:\n    def __init__(self, directory, script_path, base_env):\n        self.directory = Path(directory).resolve()\n        self.directory.mkdir(parents=True, exist_ok=True, mode=0o700)\n        self.script_path = Path(script_path).resolve()\n        self.base_env = dict(base_env)\n        self.systemd_run = shutil.which("systemd-run", path=self.base_env.get("PATH"))\n        self.systemctl = shutil.which("systemctl", path=self.base_env.get("PATH"))\n\n    def _ctl(self, *args):\n        child = None\n        try:\n            child = subprocess.Popen([self.systemctl, "--user", *args], env=self.base_env, stdin=subprocess.DEVNULL,\n                                     stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True)\n            os.set_blocking(child.stdout.fileno(), False)\n            buffer = bytearray()\n            until = time.monotonic() + 1.5\n            with selectors.DefaultSelector() as selector:\n                selector.register(child.stdout.fileno(), selectors.EVENT_READ)\n                while child.poll() is None or selector.get_map():\n                    if time.monotonic() >= until or len(buffer) > 32768:\n                        child.kill()\n                        return {"exit_code": None, "error": "Control query exceeded its time/output budget.", "stdout": bytes(buffer[:32768]).decode(errors="replace")}\n                    for key, _ in selector.select(0.05):\n                        try:\n                            data = os.read(key.fd, 4096)\n                        except BlockingIOError:\n                            continue\n                        if data:\n                            buffer.extend(data)\n                        else:\n                            selector.unregister(key.fd)\n            return {"exit_code": child.returncode, "stdout": buffer.decode(errors="replace"), "stderr": ""}\n        except OSError as error:\n            return {"exit_code": None, "error": str(error)}\n        finally:\n            if child:\n                if child.poll() is None:\n                    child.kill()\n                try:\n                    child.wait(timeout=0.2)\n                except subprocess.TimeoutExpired:\n                    pass\n                child.stdout.close()\n\n    def _show(self, unit):\n        result = self._ctl("show", unit, *["--property=" + key for key in PROPERTIES])\n        if result.get("exit_code") == 0:\n            return dict(line.split("=", 1) for line in result["stdout"].splitlines() if "=" in line)\n        return {"unavailable": result}\n\n    def preflight(self):\n        return self.run("preflight", [sys.executable, "-c", "print(\'GUARD_PREFLIGHT_OK\')"], self.directory, {}, kind="probe")\n\n    def run(self, label, command, cwd, env, kind="case", tmpdir=None):\n        budget = dict(BUDGETS[kind])\n        result = {"label": label, "kind": kind, "budget": budget, "outcome": "controls_unavailable", "verified": False, "exit_code": None}\n        if not self.systemd_run or not self.systemctl:\n            return dict(result, error="systemd-run and systemctl are required; unrestricted execution was refused.")\n        manager = self._ctl("show", "--property=Version", "--value")\n        if manager.get("exit_code") != 0:\n            return dict(result, error="The systemd user manager is unavailable.", manager=manager)\n        merged = dict(self.base_env)\n        merged.update({str(key): str(value) for key, value in env.items()})\n        executable = shutil.which(str(command[0]), path=merged.get("PATH"))\n        if executable is None:\n            return dict(result, error="Command executable was not found: " + str(command[0]))\n        # Keep the executable entry name: Nix dispatches nix-shell by argv[0].\n        command = [os.path.abspath(executable), *map(str, command[1:])]\n        stem = re.sub(r"[^a-zA-Z0-9_.-]", "-", str(label))[:40] + "-" + uuid.uuid4().hex[:12]\n        directory = self.directory / stem\n        directory.mkdir(mode=0o700)\n        data = Path(tmpdir).resolve() if tmpdir else directory / "test-data"\n        data.mkdir(parents=True, exist_ok=True, mode=0o700)\n        merged["TEST_TMPDIR"] = str(data)\n        plan = {"kind": kind, "command": command, "cwd": str(Path(cwd).resolve()), "env": merged,\n                "probe": str(directory / "probe.json"), "permit": str(directory / "permit"), "nonce": uuid.uuid4().hex}\n        plan_path = directory / "plan.json"\n        _atomic_json(plan_path, plan)\n        unit = "bitcoin-rs-guard-" + stem + ".service"\n        argv = [self.systemd_run, "--user", "--wait", "--pipe", "--service-type=exec", "--unit=" + unit,\n                "--property=MemoryAccounting=yes", "--property=CPUAccounting=yes", "--property=TasksAccounting=yes",\n                "--property=MemoryHigh=" + str(budget["high"]), "--property=MemoryMax=" + str(budget["memory"]),\n                "--property=MemorySwapMax=0", "--property=CPUQuota=100%", "--property=TasksMax=" + str(budget["tasks"]),\n                "--property=RuntimeMaxSec=" + str(budget["seconds"]) + "s", "--property=RuntimeRandomizedExtraSec=0",\n                "--property=TimeoutStartSec=5s", "--property=TimeoutStopSec=3s", "--property=KillMode=control-group",\n                "--property=OOMPolicy=kill", "--property=SendSIGKILL=yes", "--property=Restart=no", "--property=LimitCORE=0",\n                str(Path(sys.executable).resolve()), str(self.script_path), "--guard-child", str(plan_path)]\n        result.update(unit=unit, command=command, cwd=plan["cwd"], data_directory=str(data), log=str(directory / "output.log"))\n        started = time.monotonic()\n        deadline, termination_until = started + budget["seconds"] + 5, None\n        reason = None\n        child = None\n        tail = bytearray()\n        output_bytes = 0\n        probe = None\n        last_sample = started\n        last_cgroup_sample = started\n        snapshots = collections.deque(maxlen=8)\n        try:\n            child = subprocess.Popen(argv, env=self.base_env, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,\n                                     stderr=subprocess.STDOUT, bufsize=0)\n            fd = child.stdout.fileno()\n            os.set_blocking(fd, False)\n            selector = selectors.DefaultSelector()\n            selector.register(fd, selectors.EVENT_READ)\n            with (directory / "output.log").open("wb") as log:\n                while child.poll() is None or selector.get_map():\n                    now = time.monotonic()\n                    if probe is None and Path(plan["probe"]).exists():\n                        probe = json.loads(Path(plan["probe"]).read_text())\n                        result["probe"] = probe\n                        if not probe.get("ok"):\n                            reason = "controls_unavailable"\n                        else:\n                            properties = self._show(unit)\n                            result["verified_properties"] = properties\n                            if not _service_controls(properties, budget):\n                                reason = "controls_unavailable"\n                            else:\n                                Path(plan["permit"]).write_text(plan["nonce"])\n                                result["verified"] = True\n                    if now - last_sample >= 0.1:\n                        last_sample = now\n                        if kind in {"case", "stress"}:\n                            data_limit = budget.get("data", 128*MIB)\n                            entry_limit = budget.get("entries", 4096)\n                            size, count = _tree_usage(data, data_limit, entry_limit)\n                            result["data_bytes"], result["data_entries"] = size, count\n                            if size > data_limit or count > entry_limit:\n                                reason = reason or "data_limit"\n                        if shutil.disk_usage(directory).free < 2*1024*MIB:\n                            reason = reason or "disk_free_limit"\n                        if probe and probe.get("ok") and now - last_cgroup_sample >= 0.5:\n                            snapshots.append(_cgroup_snapshot(Path(probe["kernel"]["path"])))\n                            last_cgroup_sample = now\n                    if now >= deadline:\n                        reason = reason or "parent_timeout"\n                    if reason and termination_until is None:\n                        result["stop_request"] = self._ctl("--no-block", "stop", unit)\n                        termination_until = time.monotonic() + 5\n                    if termination_until is not None and time.monotonic() >= termination_until:\n                        result["kill_request"] = self._ctl("kill", "--kill-whom=all", "--signal=KILL", unit)\n                        break\n                    for key, _ in selector.select(0.1):\n                        for _chunk in range(16):\n                            try:\n                                raw = os.read(key.fd, 65536)\n                            except BlockingIOError:\n                                break\n                            if not raw:\n                                selector.unregister(key.fd)\n                                break\n                            old_bytes = output_bytes\n                            output_bytes += len(raw)\n                            if old_bytes < budget["output"]:\n                                log.write(raw[:budget["output"] - old_bytes])\n                            tail.extend(raw)\n                            del tail[:-65536]\n                            if output_bytes > budget["output"]:\n                                reason = reason or "output_limit"\n                            text = tail.decode(errors="replace").lower()\n                            if "ignoring" in text and "restricted" in text and any(word in text for word in ("max-jobs", "builders", "substitute")):\n                                reason = reason or "restricted_nix_setting_ignored"\n                selector.close()\n            if child.poll() is None:\n                child.terminate()\n                try:\n                    child.wait(timeout=0.5)\n                except subprocess.TimeoutExpired:\n                    child.kill()\n                    child.wait(timeout=0.5)\n            result["exit_code"] = child.returncode\n        except KeyboardInterrupt:\n            reason = "interrupted"\n            result["stop_request"] = self._ctl("--no-block", "stop", unit)\n            result["kill_request"] = self._ctl("kill", "--kill-whom=all", "--signal=KILL", unit)\n        except Exception as error:\n            reason = reason or "guard_error"\n            result["error"] = type(error).__name__ + ": " + str(error)\n        finally:\n            if child and child.poll() is None:\n                result["emergency_stop"] = self._ctl("--no-block", "stop", unit)\n                result["emergency_kill"] = self._ctl("kill", "--kill-whom=all", "--signal=KILL", unit)\n                child.kill()\n                try:\n                    child.wait(timeout=0.2)\n                except subprocess.TimeoutExpired:\n                    result["client_reap_pending"] = True\n            result.update(elapsed_seconds=time.monotonic() - started, output_bytes=output_bytes,\n                          tail=tail.decode(errors="replace"), live_cgroup=list(snapshots), postmortem=self._show(unit))\n            if probe and probe.get("ok"):\n                result["final_cgroup"] = _cgroup_snapshot(Path(probe["kernel"]["path"]))\n            properties = result["postmortem"]\n            if properties.get("Result") == "oom-kill":\n                reason = reason or "memory_limit"\n            elif properties.get("Result") == "timeout":\n                reason = reason or "service_timeout"\n            result["outcome"] = reason or ("exited" if result["verified"] else "controls_unavailable")\n            result["cleanup"] = {"stop": self._ctl("--no-block", "stop", unit), "reset": self._ctl("reset-failed", unit)}\n            _atomic_json(directory / "result.json", result)\n            if child and child.stdout:\n                child.stdout.close()\n        return result\n\n\nif __name__ == "__main__":\n    if len(sys.argv) == 3 and sys.argv[1] == "--guard-child":\n        sys.exit(guard_child(sys.argv[2]))\n    raise SystemExit("Internal resource controller; use a crate verification command.")\n'
PATCHES={'bitcoinleveldb-dbimpl/src/make_room_for_write.rs': {'after': '// ---------------- [ File: bitcoinleveldb-dbimpl/src/make_room_for_write.rs ]\ncrate::ix!();\n\nimpl DBImpl {\n    /// REQUIRES: mutex is held\n    /// \n    /// REQUIRES: this thread is currently at the front of the writer queue\n    /// \n    /// force - compact even if there is room?\n    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]\n    pub fn make_room_for_write(&mut self, mut force: bool) -> crate::Status {\n        self.mutex.assert_held();\n        assert!(!self.writers.is_empty());\n\n        if self.options.env().is_none() {\n            tracing::error!("make_room_for_write: Options.env is None");\n            return Status::invalid_argument(\n                &Slice::from_str("env"),\n                Some(&Slice::from_str("missing from Options")),\n            );\n        }\n\n        if self.mem.is_null() {\n            tracing::error!(\n                dbname = %self.dbname,\n                force,\n                writers_len = self.writers.len() as u64,\n                "make_room_for_write: memtable pointer was null; DBImpl is not opened/initialized"\n            );\n\n            return Status::invalid_argument(\n                &Slice::from_str("mem"),\n                Some(&Slice::from_str("null memtable pointer")),\n            );\n        }\n\n        let versions: *mut VersionSet = self.versions as *mut VersionSet;\n\n        tracing::debug!(\n            force,\n            writers_len = self.writers.len(),\n            "make_room_for_write: begin"\n        );\n\n        let mut allow_delay: bool = !force;\n        let mut s: Status = Status::ok();\n\n        loop {\n            if !self.bg_error.is_ok() {\n                // Yield previous error.\n                s = self.bg_error.clone();\n                tracing::debug!(\n                    status = %s.to_string(),\n                    "make_room_for_write: exiting due to background error"\n                );\n                break;\n            }\n\n            let l0_files: i32 = unsafe { (*versions).num_level_files(0) };\n\n            if allow_delay && l0_files >= (L0_SLOWDOWN_WRITES_TRIGGER as i32) {\n                tracing::debug!(\n                    l0_files,\n                    trigger = L0_SLOWDOWN_WRITES_TRIGGER as i32,\n                    "make_room_for_write: delaying write to reduce L0 pressure"\n                );\n\n                unsafe {\n                    self.mutex.unlock();\n                }\n\n                self.env.as_mut().sleep_for_microseconds(1000);\n\n                // Do not delay a single write more than once.\n                allow_delay = false;\n\n                self.mutex.lock();\n                continue;\n            }\n\n            let mem_usage: usize = unsafe { (*self.mem).approximate_memory_usage() };\n            let write_buffer_limit: usize = *self.options.write_buffer_size();\n\n            if !force && mem_usage <= write_buffer_limit {\n                tracing::trace!(\n                    mem_usage,\n                    write_buffer_limit,\n                    "make_room_for_write: sufficient space in memtable"\n                );\n                break;\n            }\n\n            if !self.imm.is_null() {\n                // Mirror upstream LevelDB semantics exactly:\n                // if an immutable memtable is already outstanding, we must wait for\n                // background compaction to finish instead of rotating another memtable\n                // and overwriting self.imm.\n                tracing::info!(\n                    imm_ptr = self.imm as usize,\n                    mem_usage,\n                    write_buffer_limit,\n                    "make_room_for_write: current memtable full; waiting for immutable memtable compaction"\n                );\n\n                let mut cv_guard = self.background_work_finished_mutex.lock();\n\n                unsafe {\n                    self.mutex.unlock();\n                }\n\n                self.background_work_finished_signal.wait(&mut cv_guard);\n\n                drop(cv_guard);\n\n                self.mutex.lock();\n\n                tracing::debug!(\n                    imm_ptr = self.imm as usize,\n                    bg_error = %self.bg_error.to_string(),\n                    "make_room_for_write: woke while waiting for immutable memtable compaction"\n                );\n\n                continue;\n            }\n\n            if l0_files >= (L0_STOP_WRITES_TRIGGER as i32) {\n                // Mirror upstream LevelDB semantics exactly:\n                // when level-0 is too full, writers must wait for background work\n                // to make progress before creating yet another level-0 file.\n                tracing::info!(\n                    l0_files,\n                    trigger = L0_STOP_WRITES_TRIGGER as i32,\n                    "make_room_for_write: too many L0 files; waiting for background compaction"\n                );\n\n                let mut cv_guard = self.background_work_finished_mutex.lock();\n\n                unsafe {\n                    self.mutex.unlock();\n                }\n\n                self.background_work_finished_signal.wait(&mut cv_guard);\n\n                drop(cv_guard);\n\n                self.mutex.lock();\n\n                tracing::debug!(\n                    l0_files = unsafe { (*versions).num_level_files(0) },\n                    bg_error = %self.bg_error.to_string(),\n                    "make_room_for_write: woke while waiting for L0 pressure to drop"\n                );\n\n                continue;\n            }\n\n            // Attempt to switch to a new memtable and trigger compaction of old.\n            assert_eq!(unsafe { (*versions).prev_log_number() }, 0);\n            assert!(\n                self.imm.is_null(),\n                "make_room_for_write: attempted to rotate memtable while immutable memtable was still outstanding"\n            );\n\n            let new_log_number: u64 = unsafe { (*versions).new_file_number() };\n            let fname: String = log_file_name(&self.dbname, new_log_number);\n\n            tracing::info!(\n                log_number = new_log_number,\n                file = %fname,\n                "make_room_for_write: switching to new log file"\n            );\n\n            let mut new_logfile_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();\n\n            s = self.env.as_mut().new_writable_file(\n                &fname,\n                &mut new_logfile_ptr as *mut *mut Box<dyn WritableFile>,\n            );\n\n            if !s.is_ok() {\n                tracing::warn!(\n                    log_number = new_log_number,\n                    status = %s.to_string(),\n                    "make_room_for_write: failed to create new log file; reusing file number"\n                );\n                unsafe {\n                    (*versions).reuse_file_number(new_log_number);\n                }\n                break;\n            }\n\n            if new_logfile_ptr.is_null() {\n                tracing::error!(\n                    log_number = new_log_number,\n                    file = %fname,\n                    "Env::new_writable_file returned ok but output file pointer was null"\n                );\n\n                let msg: Slice = Slice::from_str("new_writable_file returned ok but output was null");\n                let fname_slice: Slice = Slice::from_str(&fname);\n                s = Status::corruption(&msg, Some(&fname_slice));\n\n                unsafe {\n                    (*versions).reuse_file_number(new_log_number);\n                }\n                break;\n            }\n\n            let new_logfile_box: Box<dyn WritableFile> = unsafe { *Box::from_raw(new_logfile_ptr) };\n            let new_logfile: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(new_logfile_box));\n\n            if !self.log.is_null() {\n                unsafe {\n                    drop(Box::from_raw(self.log));\n                }\n                self.log = core::ptr::null_mut();\n            }\n\n            self.logfile = new_logfile;\n            self.logfile_number = new_log_number;\n\n            self.log = Box::into_raw(Box::new(LogWriter::new(self.logfile.clone(), 0)));\n\n            self.imm = self.mem;\n            self.has_imm\n                .store(true, core::sync::atomic::Ordering::Release);\n\n            self.mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));\n            unsafe {\n                (*self.mem).ref_();\n            }\n\n            // Do not force another compaction if we now have room.\n            force = false;\n\n            tracing::debug!(\n                log_number = new_log_number,\n                imm_ptr = self.imm as usize,\n                mem_ptr = self.mem as usize,\n                "make_room_for_write: installed new log + memtable; scheduling compaction"\n            );\n\n            self.maybe_schedule_compaction();\n        }\n\n        tracing::debug!(status = %s.to_string(), "make_room_for_write: end");\n        s\n    }\n\n    #[cfg(test)]\n    pub fn test_install_temporary_immutable_memtable_and_release_after_delay_for_make_room_for_write_harness(\n        &mut self,\n        release_after_millis: u64\n    ) -> std::thread::JoinHandle<()>\n    {\n        trace!(\n            target: "bitcoinleveldb_dbimpl::make_room_for_write_regression",\n            event = "dbimpl_test_install_temporary_immutable_memtable_entry",\n            release_after_millis = release_after_millis,\n            dbname = %self.dbname\n        );\n\n        self.mutex.lock();\n\n        assert!(\n            !self.mem.is_null(),\n            "test_install_temporary_immutable_memtable_and_release_after_delay_for_make_room_for_write_harness: DB must be opened"\n        );\n        assert!(\n            self.imm.is_null(),\n            "test_install_temporary_immutable_memtable_and_release_after_delay_for_make_room_for_write_harness: harness requires imm to be null before injection"\n        );\n\n        let injected_imm: *mut MemTable =\n            Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));\n\n        unsafe {\n            (*injected_imm).ref_();\n        }\n\n        self.imm = injected_imm;\n        self.has_imm\n            .store(true, core::sync::atomic::Ordering::Release);\n\n        unsafe {\n            self.mutex.unlock();\n        }\n\n        let db_ptr_value: usize = self as *mut DBImpl as usize;\n        let injected_imm_ptr_value: usize = injected_imm as usize;\n\n        std::thread::spawn(move || {\n            std::thread::sleep(std::time::Duration::from_millis(release_after_millis));\n\n            unsafe {\n                let db: &mut DBImpl = &mut *(db_ptr_value as *mut DBImpl);\n                let injected_imm: *mut MemTable = injected_imm_ptr_value as *mut MemTable;\n\n                db.mutex.lock();\n\n                let injected_still_installed: bool = db.imm == injected_imm;\n\n                if injected_still_installed {\n                    db.imm = core::ptr::null_mut();\n                    db.has_imm\n                        .store(false, core::sync::atomic::Ordering::Release);\n                }\n\n                (*injected_imm).unref();\n\n                {\n                    let _cv_guard = db.background_work_finished_mutex.lock();\n                    db.background_work_finished_signal.signal_all();\n                }\n\n                db.mutex.unlock();\n            }\n        })\n    }\n}\n\n#[cfg(test)]\nmod make_room_for_write_interface_contract_suite {\n    use super::*;\n\n    fn build_temp_db_path_for_make_room_for_write_suite() -> String {\n        let tmp = TempDir::new().unwrap();\n        let dbname = tmp.path().to_string_lossy().to_string();\n        dbname\n    }\n\n    fn build_options_with_env_or_panic_for_make_room_for_write_suite() -> Options {\n        let env = PosixEnv::shared();\n        let options: Options = Options::with_env(env);\n\n        if options.env().is_none() {\n            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run make_room_for_write suite");\n            panic!();\n        }\n\n        options\n    }\n\n    #[traced_test]\n    fn make_room_for_write_signature_is_stable() {\n        tracing::info!("Asserting DBImpl::make_room_for_write signature is stable");\n        type Sig = fn(&mut DBImpl, bool) -> Status;\n        let _sig: Sig = DBImpl::make_room_for_write;\n        tracing::debug!("Signature check compiled");\n    }\n\n    #[traced_test]\n    fn make_room_for_write_method_item_is_addressable() {\n        tracing::info!("Asserting DBImpl::make_room_for_write method item is addressable");\n        let _m = DBImpl::make_room_for_write;\n        let _ = _m;\n    }\n\n    #[traced_test]\n    fn make_room_for_write_returns_error_when_options_env_is_none_without_touching_memtable() {\n        let dbname = build_temp_db_path_for_make_room_for_write_suite();\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = build_options_with_env_or_panic_for_make_room_for_write_suite();\n        let mut db: DBImpl = DBImpl::new(&options, &dbname);\n\n        let mut writer: DBImplWriter = DBImplWriter::new(&mut db.mutex);\n\n        db.mutex.lock();\n        db.writers.push_back(&mut writer as *mut DBImplWriter);\n\n        db.options.set_env(None);\n\n        tracing::info!("Calling make_room_for_write with Options.env=None; expecting non-OK Status");\n        let s: Status = db.make_room_for_write(false);\n\n        tracing::debug!(status = %s.to_string(), "make_room_for_write returned");\n        assert!(\n            !s.is_ok(),\n            "make_room_for_write must return non-OK when Options.env is None"\n        );\n\n        db.writers.clear();\n        unsafe { db.mutex.unlock() };\n\n        drop(db);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n\n    #[cfg(test)]\n    #[traced_test]\n    fn dbimpl_make_room_for_write_waits_for_outstanding_immutable_memtable_regression() {\n        init_test_runtime();\n\n        let temporary_database_directory =\n            bitcoinleveldb_dbimpl_live_compaction_boundary_build_temporary_database_directory_path(\n                "dbimpl_make_room_for_write_waits_for_immutable_memtable_regression",\n            );\n\n        let mut database_instance =\n            bitcoinleveldb_dbimpl_live_compaction_boundary_open_database_instance_via_dbopen(\n                &temporary_database_directory,\n            );\n\n        let releaser =\n            database_instance\n            .test_install_temporary_immutable_memtable_and_release_after_delay_for_make_room_for_write_harness(\n                300,\n            );\n\n        let started_at = std::time::Instant::now();\n\n        let status = <DBImpl as DBWrite>::write(\n            database_instance.as_mut(),\n            &WriteOptions::default(),\n            core::ptr::null_mut(),\n        );\n\n        let elapsed = started_at.elapsed();\n\n        assert!(status.is_ok());\n        assert!(releaser.join().is_ok());\n\n        assert!(\n            elapsed >= std::time::Duration::from_millis(200),\n            "make_room_for_write returned before the outstanding immutable memtable was released: elapsed={:?}",\n            elapsed\n        );\n\n        drop(database_instance);\n\n        let cleanup_options = Options::with_env(PosixEnv::shared());\n        let cleanup_status = bitcoinleveldb_dbconstructor::destroydb(&temporary_database_directory, &cleanup_options);\n        assert!(cleanup_status.is_ok());\n\n    }\n}\n', 'after_sha256': '87a2253217a7fbd061851b27ab8b36a6630cf6e66b517fe5363328687f370150', 'before_sha256': '9772dc4b58a9089072bba0b6f4488b3f1f2dd91128b991f8ba9a9e40a1f0ce47'}, 'bitcoinleveldb-dbimpl/src/write_level0_table.rs': {'after': '// ---------------- [ File: bitcoinleveldb-dbimpl/src/write_level0_table.rs ]\ncrate::ix!();\n\nimpl DBImpl {\n    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]\n    pub fn write_level_0table(\n        &mut self,\n        mem: *mut MemTable,\n        edit: *mut VersionEdit,\n        base: *mut Version,\n    ) -> crate::Status {\n        self.mutex.assert_held();\n\n        if self.options.env().is_none() {\n            tracing::error!("write_level_0table: Options.env is None");\n            return Status::invalid_argument(\n                &Slice::from_str("env"),\n                Some(&Slice::from_str("missing from Options")),\n            );\n        }\n\n        let start_micros: u64 = self.env.as_mut().now_micros();\n\n        let mut meta: FileMetaData = Default::default();\n        meta.set_number(unsafe { (*(self.versions as *mut VersionSet)).new_file_number() });\n\n        self.pending_outputs.insert(*meta.number());\n\n        let iter: *mut LevelDBIterator = unsafe { (*mem).new_iterator() };\n\n        tracing::info!(file_number = meta.number(), "Level-0 table started");\n\n        let mut s: Status = Status::ok();\n\n        unsafe {\n            self.mutex.unlock();\n        }\n\n        // The DB-owned wrapper outlives this background operation. Borrow it\n        // rather than changing the shared Env Rc count while the DB mutex is\n        // released; individual Env calls retain the wrapper\'s access gate.\n        s = build_table_with_env(\n            &self.dbname,\n            self.env.as_mut(),\n            &self.options,\n            self.table_cache as *mut TableCache,\n            iter,\n            &mut meta,\n        );\n\n        self.mutex.lock();\n\n        tracing::info!(\n            file_number = meta.number(),\n            bytes = meta.file_size(),\n            status = %s.to_string(),\n            "Level-0 table finished"\n        );\n\n        unsafe {\n            drop(Box::from_raw(iter));\n        }\n\n        self.pending_outputs.remove(meta.number());\n\n        // Note that if file_size is zero, the file has been deleted and\n        // should not be added to the manifest.\n        let mut level: i32 = 0;\n\n        if s.is_ok() && *meta.file_size() > 0 {\n            let min_user_key: Slice = meta.smallest().user_key();\n            let max_user_key: Slice = meta.largest().user_key();\n\n            if !base.is_null() {\n                level =\n                    unsafe { (*base).pick_level_for_mem_table_output(&min_user_key, &max_user_key) };\n            }\n\n            unsafe {\n                (*edit).add_file(\n                    level,\n                    *meta.number(),\n                    *meta.file_size(),\n                    meta.smallest(),\n                    meta.largest(),\n                );\n            }\n        }\n\n        let mut stats: CompactionStats = Default::default();\n        stats.set_micros((self.env.as_mut().now_micros() - start_micros) as i64);\n        stats.set_bytes_written(*meta.file_size() as i64);\n\n        self.stats[level as usize].add(&stats);\n\n        s\n    }\n}\n\n#[cfg(test)]\nmod write_level0_table_interface_contract_suite {\n    use super::*;\n\n    fn build_temp_db_path_for_write_level0_table_suite() -> String {\n        let tmp = TempDir::new().unwrap();\n        let dbname = tmp.path().to_string_lossy().to_string();\n        dbname\n    }\n\n    fn build_options_with_env_or_panic_for_write_level0_table_suite() -> Options {\n        let env = PosixEnv::shared();\n        let options: Options = Options::with_env(env);\n\n        if options.env().is_none() {\n            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run write_level_0table suite");\n            panic!();\n        }\n\n        options\n    }\n\n    #[traced_test]\n    fn write_level_0table_signature_is_stable_for_dbimpl_public_method() {\n        tracing::info!("Asserting DBImpl::write_level_0table signature is stable");\n\n        type WriteL0Sig = fn(&mut DBImpl, *mut MemTable, *mut VersionEdit, *mut Version) -> Status;\n        let _sig: WriteL0Sig = DBImpl::write_level_0table;\n\n        tracing::debug!("DBImpl::write_level_0table signature check compiled");\n    }\n\n    #[traced_test]\n    fn write_level_0table_method_item_is_addressable() {\n        tracing::info!("Asserting DBImpl::write_level_0table method item is addressable");\n        let _m = DBImpl::write_level_0table;\n        let _ = _m;\n    }\n\n    #[traced_test]\n    fn write_level_0table_returns_error_when_options_env_is_none() {\n        let dbname = build_temp_db_path_for_write_level0_table_suite();\n        let _ = std::fs::create_dir_all(&dbname);\n\n        let options = build_options_with_env_or_panic_for_write_level0_table_suite();\n        let mut db: DBImpl = DBImpl::new(&options, &dbname);\n\n        db.options.set_env(None);\n\n        let mut edit: VersionEdit = Default::default();\n\n        db.mutex.lock();\n\n        tracing::info!("Calling write_level_0table with Options.env=None; expecting non-OK Status without dereferencing inputs");\n\n        let s: Status = db.write_level_0table(\n            core::ptr::null_mut(),\n            &mut edit as *mut VersionEdit,\n            core::ptr::null_mut(),\n        );\n\n        unsafe { db.mutex.unlock() };\n\n        tracing::debug!(status = %s.to_string(), "write_level_0table returned");\n        assert!(!s.is_ok(), "write_level_0table must return non-OK when Options.env is None");\n\n        drop(db);\n        let _ = std::fs::remove_dir_all(&dbname);\n    }\n}\n', 'after_sha256': '4145c64477b41291f5590eb4d74b74e09500e7bcaf362c2d9def9d2df2f4b7b6', 'before_sha256': 'a7f482b477171fd5f2ca7a7bbcfb68d0bc124359465eef0191515d3d458068e9'}, 'bitcoinleveldb-env/src/set_current_file.rs': {'after': '// ---------------- [ File: bitcoinleveldb-env/src/set_current_file.rs ]\ncrate::ix!();\n\n/**\n  | Make the CURRENT file point to the descriptor\n  | file with the specified number.\n  |\n  */\npub fn set_current_file(\n    env:                Rc<RefCell<dyn Env>>,\n    dbname:             &String,\n    descriptor_number:  u64,\n) -> Status {\n    let mut env = EnvWrapper::new(env);\n    set_current_file_with_env(&mut env, dbname, descriptor_number)\n}\n\n/// Install CURRENT through an existing Env view. No environment Rc is cloned or\n/// dropped here; the caller retains the target and provides its synchronization.\npub fn set_current_file_with_env(\n    env:                &mut dyn Env,\n    dbname:             &String,\n    descriptor_number:  u64,\n) -> Status {\n    trace!(\n        dbname = %dbname,\n        descriptor_number,\n        "set_current_file: start"\n    );\n\n    // C++: std::string manifest = DescriptorFileName(dbname, descriptor_number);\n    let manifest = descriptor_file_name(dbname, descriptor_number);\n\n    // C++: Slice contents = manifest;\n    let mut contents = Slice::from(&manifest);\n\n    // C++: assert(contents.starts_with(dbname + "/"));\n    let prefix_string = {\n        let mut s = dbname.clone();\n        s.push(\'/\');\n        s\n    };\n    let prefix_slice = Slice::from(&prefix_string);\n    assert!(\n        contents.starts_with(&prefix_slice),\n        "set_current_file: manifest \'{}\' does not start with \'{}\'",\n        manifest,\n        prefix_string,\n    );\n\n    // C++: contents.remove_prefix(dbname.size() + 1);\n    contents.remove_prefix(dbname.len() + 1);\n\n    // C++: std::string tmp = TempFileName(dbname, descriptor_number);\n    let tmp = temp_file_name(dbname, descriptor_number);\n\n    // C++: Status s = WriteStringToFileSync(env, contents.ToString() + "\\n", tmp);\n    let content_string = {\n        let mut s = contents.to_string();\n        s.push(\'\\n\');\n        s\n    };\n    let content_slice = Slice::from(&content_string);\n\n    trace!(\n        dbname = %dbname,\n        tmp = %tmp,\n        "set_current_file: writing CURRENT contents to temporary file"\n    );\n\n    let mut status = do_write_string_to_file_with_env(env, &content_slice, &tmp, true);\n\n    if status.is_ok() {\n        let current_name = current_file_name(dbname);\n        trace!(\n            tmp = %tmp,\n            current = %current_name,\n            "set_current_file: renaming temporary file to CURRENT"\n        );\n        status = env.rename_file(&tmp, &current_name);\n    }\n\n    if !status.is_ok() {\n        trace!(\n            tmp = %tmp,\n            status_str = %status.to_string(),\n            "set_current_file: operation failed, deleting temporary file"\n        );\n        let _ = env.delete_file(&tmp);\n    }\n\n    debug!(\n        dbname = %dbname,\n        descriptor_number,\n        ok = status.is_ok(),\n        "set_current_file: completed"\n    );\n    status\n}\n', 'after_sha256': 'b9289f6b2547ce6382c72b6e5b7f20a6188394716c99bc2fb836d1733d7ed623', 'before_sha256': 'ca254518ca20a772dced29a844bb9fa96c66aed92371d3329788e3a3f5ce409f'}, 'bitcoinleveldb-env/src/write_string_to_file_impl.rs': {'after': '// ---------------- [ File: bitcoinleveldb-env/src/write_string_to_file_impl.rs ]\ncrate::ix!();\n\npub fn do_write_string_to_file(\n    env:         Rc<RefCell<dyn Env>>,\n    data:        &Slice,\n    fname:       &String,\n    should_sync: bool,\n) -> Status {\n    let mut env = EnvWrapper::new(env);\n    do_write_string_to_file_with_env(&mut env, data, fname, should_sync)\n}\n\n/// Write through a caller-owned Env view without changing environment Rc\n/// ownership. The caller keeps the target alive and supplies its synchronization.\npub fn do_write_string_to_file_with_env(\n    env:         &mut dyn Env,\n    data:        &Slice,\n    fname:       &String,\n    should_sync: bool,\n) -> Status {\n    use std::ptr;\n\n    trace!(\n        file = %fname,\n        should_sync,\n        data_len = *data.size(),\n        "do_write_string_to_file: start"\n    );\n\n    // C++: WritableFile* file; Status s = env->NewWritableFile(fname, &file);\n    let mut file_ptr: *mut Box<dyn WritableFile> = ptr::null_mut();\n    let mut status = env.new_writable_file(fname, &mut file_ptr);\n\n    if !status.is_ok() {\n        debug!(\n            file = %fname,\n            status_str = %status.to_string(),\n            "do_write_string_to_file: NewWritableFile failed"\n        );\n        return status;\n    }\n\n    // Take ownership of allocated file (mirrors C++ delete file).\n    // We were given `*mut Box<dyn WritableFile>`, so reconstruct the outer Box\n    // and then borrow the inner `Box<dyn WritableFile>` mutably.\n    let mut file_holder: Box<Box<dyn WritableFile>> = unsafe {\n        assert!(\n            !file_ptr.is_null(),\n            "Env::NewWritableFile returned a null file pointer"\n        );\n        Box::from_raw(file_ptr)\n    };\n\n    let file: &mut Box<dyn WritableFile> = file_holder.as_mut();\n\n    // C++: s = file->Append(data);\n    status = file.append(data);\n\n    // C++: if (s.ok() && should_sync) { s = file->Sync(); }\n    if status.is_ok() && should_sync {\n        trace!(file = %fname, "do_write_string_to_file: calling Sync");\n        status = file.sync();\n    }\n\n    // C++: if (s.ok()) { s = file->Close(); }\n    if status.is_ok() {\n        trace!(file = %fname, "do_write_string_to_file: calling Close");\n        status = file.close();\n    }\n\n    // `file_holder` drops here, mirroring `delete file` in C++.\n\n    // C++: if (!s.ok()) { env->DeleteFile(fname); }\n    if !status.is_ok() {\n        debug!(\n            file = %fname,\n            status_str = %status.to_string(),\n            "do_write_string_to_file: error, deleting file"\n        );\n        let _ = env.delete_file(fname);\n    }\n\n    debug!(\n        file = %fname,\n        should_sync,\n        ok = status.is_ok(),\n        "do_write_string_to_file: completed"\n    );\n    status\n}\n', 'after_sha256': '123ba773e3f27ba314da9d542d05e6497a829abcfcffbd62573920c0d66923ac', 'before_sha256': 'a056e3dcb7d8ead37058b6c764d90326855833df66808423e611011012ba1ab2'}, 'bitcoinleveldb-options/src/options.rs': {'after': '// ---------------- [ File: bitcoinleveldb-options/src/options.rs ]\ncrate::ix!();\n\n//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/options.cc]\n//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/options.h]\n\n/// Options to control the behavior of a\n/// database (passed to DB::Open)\n/// \n#[derive(Clone,Getters,MutGetters,Setters)]\n#[getset(get="pub",get_mut="pub",set="pub")]\npub struct Options {\n\n    /* -------- Parameters that affect behavior  -------- */\n\n    /// Comparator used to define the order of keys in the table.\n    ///\n    /// Default: a comparator that uses lexicographic byte-wise ordering\n    ///\n    /// REQUIRES: The client must ensure that the comparator supplied here has\n    /// the same name and orders keys *exactly* the same as the comparator\n    /// provided to previous open calls on the same DB.\n    ///\n    comparator:        Arc<dyn SliceComparator>,\n\n    /// If true, the database will be created if it is missing.\n    ///\n    create_if_missing: bool,\n\n    /// If true, an error is raised if the database already exists.\n    ///\n    error_if_exists:   bool,\n\n    /// If true, the implementation will do aggressive checking of the data it\n    /// is processing and will stop early if it detects any errors.\n    ///\n    /// This may have unforeseen ramifications: for example, a corruption of one\n    /// DB entry may cause a large number of entries to become unreadable or for\n    /// the entire DB to become unopenable.\n    ///\n    paranoid_checks:   bool,\n\n    /// Use the specified object to interact with the environment, e.g. to\n    /// read/write files, schedule background work, etc.\n    ///\n    /// Default: `None`. A higher layer is expected to substitute the\n    /// process‑global default environment before first use if this is left\n    /// unset.\n    ///\n    env:               Option<Rc<RefCell<dyn Env>>>,\n\n    /// Any internal progress/error information generated by the DB will be\n    /// written to `info_log` if it is `Some`, or to a file stored in the same\n    /// directory as the DB contents if `info_log` is `None`.\n    ///\n    info_log:          Option<*mut dyn Logger>,\n\n    /* ------ Parameters that affect performance  ------ */\n\n    /// Amount of data to build up in memory (backed by an unsorted log on disk)\n    /// before converting to a sorted on-disk file.\n    ///\n    /// Larger values increase performance, especially during bulk loads.\n    ///\n    /// Up to two write buffers may be held in memory at the same time, so you\n    /// may wish to adjust this parameter to control memory usage.\n    ///\n    /// Also, a larger write buffer will result in a longer recovery time the\n    /// next time the database is opened.\n    ///\n    write_buffer_size:      usize,\n\n    /// Number of open files that can be used by the DB.\n    ///\n    /// You may need to increase this if your database has a large working set\n    /// (budget one open file per 2MB of working set).\n    ///\n    max_open_files:         i32,\n\n    /// Control over blocks (user data is stored in a set of blocks, and a block\n    /// is the unit of reading from disk).\n    ///\n    /// If non-null, use the specified cache for blocks.\n    ///\n    /// If null, leveldb will automatically create and use an 8MB internal\n    /// cache.\n    ///\n    block_cache:            *mut Cache,\n\n    /// Approximate size of user data packed per block.\n    ///\n    /// Note that the block size specified here corresponds to uncompressed\n    /// data.\n    ///\n    /// The actual size of the unit read from disk may be smaller if compression\n    /// is enabled.  This parameter can be changed dynamically.\n    ///\n    block_size:             usize,\n\n    /// Number of keys between restart points for delta encoding of keys.\n    ///\n    /// This parameter can be changed dynamically. Most clients should leave\n    /// this parameter alone.\n    ///\n    block_restart_interval: i32,\n\n    /// Leveldb will write up to this amount of bytes to a file before switching\n    /// to a new one.\n    ///\n    /// Most clients should leave this parameter alone.  However if your\n    /// filesystem is more efficient with larger files, you could consider\n    /// increasing the value.  The downside will be longer compactions and hence\n    /// longer latency/performance hiccups.\n    ///\n    /// Another reason to increase this parameter might be when you are\n    /// initially populating a large database.\n    ///\n    max_file_size:          usize,\n\n    /// Compress blocks using the specified compression algorithm.  This\n    /// parameter can be changed dynamically.\n    ///\n    /// Default: kSnappyCompression, which gives lightweight but fast\n    /// compression.\n    ///\n    /// Typical speeds of kSnappyCompression on an Intel(R) Core(TM)2 2.4GHz:\n    ///\n    ///    ~200-500MB/s compression\n    ///    ~400-800MB/s decompression\n    ///\n    /// Note that these speeds are significantly faster than most persistent\n    /// storage speeds, and therefore it is typically never worth switching to\n    /// kNoCompression.  Even if the input data is incompressible, the\n    /// kSnappyCompression implementation will efficiently detect that and will\n    /// switch to uncompressed mode.\n    ///\n    compression: CompressionType,\n\n    /// EXPERIMENTAL: If true, append to existing MANIFEST and log files when\n    /// a database is opened.  This can significantly speed up open.\n    ///\n    /// Default: currently false, but may become true later.\n    ///\n    reuse_logs:    bool,\n\n    /// If non-null, use the specified filter policy to reduce disk reads.\n    ///\n    /// Many applications will benefit from passing the result of\n    /// `new_bloom_filter_policy()` here.\n    ///\n    filter_policy: Arc<dyn FilterPolicy>,\n}\n\nimpl core::fmt::Debug for Options {\n    fn fmt(&self, f: &mut core::fmt::Formatter<\'_>) -> core::fmt::Result {\n        // Comparator + FilterPolicy are trait objects → show only their names.\n        let comparator_name = self.comparator().name();\n        let filter_name     = self.filter_policy().name();\n\n        // For env and info_log we cannot safely deref → show pointer identity.\n        let env_repr = match self.env() {\n            Some(rc) => format!("{:p}", Rc::as_ptr(rc)),\n            None     => String::from("None"),\n        };\n\n        let info_log_repr = match self.info_log() {\n            Some(ptr) => format!("{:p}", ptr),\n            None      => String::from("None"),\n        };\n\n        // block_cache is a raw pointer, show the address only.\n        let block_cache_repr = format!("{:p}", self.block_cache());\n\n        f.debug_struct("Options")\n            .field("comparator", &comparator_name)\n            .field("create_if_missing", self.create_if_missing())\n            .field("error_if_exists",  self.error_if_exists())\n            .field("paranoid_checks",  self.paranoid_checks())\n            .field("env",              &env_repr)\n            .field("info_log",         &info_log_repr)\n            .field("write_buffer_size",      self.write_buffer_size())\n            .field("max_open_files",         self.max_open_files())\n            .field("block_cache",            &block_cache_repr)\n            .field("block_size",             self.block_size())\n            .field("block_restart_interval", self.block_restart_interval())\n            .field("max_file_size",          self.max_file_size())\n            .field("compression",            self.compression())\n            .field("reuse_logs",             self.reuse_logs())\n            .field("filter_policy",          &filter_name)\n            .finish()\n    }\n}\n\nimpl Options {\n\n    /// Copy settings used by cached SSTables without acquiring an Env owner.\n    ///\n    /// TableCache keeps its environment alive until after it destroys its\n    /// cached tables. Table readers use the comparator, filter and I/O settings\n    /// here, but never the environment. Construct this field by field: cloning\n    /// an Options and then clearing `env` would still modify its Rc counters.\n    /// This does not change the ordinary Clone implementation.\n    pub fn snapshot_for_table(&self) -> Self {\n        Self {\n            comparator: Arc::clone(&self.comparator),\n            create_if_missing: self.create_if_missing,\n            error_if_exists: self.error_if_exists,\n            paranoid_checks: self.paranoid_checks,\n            env: None,\n            info_log: self.info_log,\n            write_buffer_size: self.write_buffer_size,\n            max_open_files: self.max_open_files,\n            block_cache: self.block_cache,\n            block_size: self.block_size,\n            block_restart_interval: self.block_restart_interval,\n            max_file_size: self.max_file_size,\n            compression: self.compression,\n            reuse_logs: self.reuse_logs,\n            filter_policy: Arc::clone(&self.filter_policy),\n        }\n    }\n\n    pub fn with_env(env: Rc<RefCell<dyn Env>>) -> Self {\n        let mut x = Self::default();\n        x.set_env(Some(env));\n        x\n    }\n\n    pub fn internal_key_comparator(&self) -> InternalKeyComparator {\n        let user_cmp_ptr: *const dyn SliceComparator =\n            self.comparator().as_ref() as *const dyn SliceComparator;\n\n        trace!(\n            user_comparator_ptr = %format!("{:p}", user_cmp_ptr),\n            "Options::internal_key_comparator"\n        );\n\n        InternalKeyComparator::new(user_cmp_ptr)\n    }\n\n    pub fn delete_info_log_if_owned(&mut self) -> Status {\n        if let Some(ptr) = self.info_log.take() {\n            unsafe {\n                drop(Box::from_raw(ptr));\n            }\n        }\n        Status::ok()\n    }\n\n    pub fn delete_block_cache_if_owned(&mut self) -> Status {\n        let cache_ptr: *mut Cache = self.block_cache;\n        if !cache_ptr.is_null() {\n            unsafe {\n                drop(Box::from_raw(cache_ptr));\n            }\n            self.block_cache = core::ptr::null_mut();\n        }\n        Status::ok()\n    }\n}\n\nimpl Default for Options {\n\n    fn default() -> Self {\n        trace!("Options::default: initializing default database options");\n\n        Options {\n            // -------- Behavior parameters --------\n            comparator:             Arc::new(BytewiseComparatorImpl::default()),\n            create_if_missing:      false,\n            error_if_exists:        false,\n            paranoid_checks:        false,\n            env:                    Some(bitcoinleveldb_posixenv::PosixEnv::shared()),\n            info_log:               None,\n\n            // -------- Performance parameters --------\n            write_buffer_size:      4 * 1024 * 1024,\n            max_open_files:         1000,\n            block_cache:            core::ptr::null_mut(),\n            block_size:             4 * 1024,\n            block_restart_interval: 16,\n            max_file_size:          2 * 1024 * 1024,\n            compression:            CompressionType::Snappy,\n            reuse_logs:             false,\n\n            // LevelDB uses nullptr here; we must supply *some* policy.\n            // A no-op / pass-through filter is the closest semantic match.\n            filter_policy:          Arc::new(NullFilterPolicy::default()),\n        }\n    }\n}\n\n#[cfg(test)]\nmod table_option_snapshot_contracts {\n    use super::*;\n\n    #[test]\n    fn table_snapshot_preserves_settings_without_env_owner() {\n        let mut options = Options::with_env(bitcoinleveldb_posixenv::PosixEnv::shared());\n        options.create_if_missing = true;\n        options.error_if_exists = true;\n        options.paranoid_checks = true;\n        options.write_buffer_size = 12345;\n        options.max_open_files = 73;\n        options.block_size = 257;\n        options.block_restart_interval = 3;\n        options.max_file_size = 45678;\n        options.compression = CompressionType::None;\n        options.reuse_logs = true;\n\n        let env = options.env.as_ref().unwrap();\n        let owners_before = Rc::strong_count(env);\n        let snapshot = options.snapshot_for_table();\n\n        assert_eq!(Rc::strong_count(env), owners_before);\n        assert!(snapshot.env.is_none());\n        assert!(Arc::ptr_eq(&snapshot.comparator, &options.comparator));\n        assert!(Arc::ptr_eq(&snapshot.filter_policy, &options.filter_policy));\n        assert_eq!(snapshot.create_if_missing, options.create_if_missing);\n        assert_eq!(snapshot.error_if_exists, options.error_if_exists);\n        assert_eq!(snapshot.paranoid_checks, options.paranoid_checks);\n        assert_eq!(snapshot.info_log, options.info_log);\n        assert_eq!(snapshot.write_buffer_size, options.write_buffer_size);\n        assert_eq!(snapshot.max_open_files, options.max_open_files);\n        assert_eq!(snapshot.block_cache, options.block_cache);\n        assert_eq!(snapshot.block_size, options.block_size);\n        assert_eq!(snapshot.block_restart_interval, options.block_restart_interval);\n        assert_eq!(snapshot.max_file_size, options.max_file_size);\n        assert_eq!(snapshot.compression, options.compression);\n        assert_eq!(snapshot.reuse_logs, options.reuse_logs);\n        drop(snapshot);\n        assert_eq!(Rc::strong_count(env), owners_before);\n    }\n\n    #[test]\n    fn ordinary_options_clone_keeps_env_ownership() {\n        let options = Options::with_env(bitcoinleveldb_posixenv::PosixEnv::shared());\n        let env = options.env.as_ref().unwrap();\n        let owners_before = Rc::strong_count(env);\n        let cloned = options.clone();\n        assert!(Rc::ptr_eq(cloned.env.as_ref().unwrap(), env));\n        assert_eq!(Rc::strong_count(env), owners_before + 1);\n        drop(cloned);\n        assert_eq!(Rc::strong_count(env), owners_before);\n    }\n}\n\n#[cfg(test)]\nmod options_defaults_and_accessors_suite {\n    use super::*;\n    use tracing::{debug, info, trace};\n\n    #[traced_test]\n    fn options_default_matches_expected_constants_and_null_pointers() {\n        trace!("options_defaults_and_accessors_suite: start");\n\n        let opts = Options::default();\n\n        info!(\n            create_if_missing = *opts.create_if_missing(),\n            error_if_exists   = *opts.error_if_exists(),\n            paranoid_checks   = *opts.paranoid_checks(),\n            write_buffer_size = *opts.write_buffer_size(),\n            max_open_files    = *opts.max_open_files(),\n            block_cache_ptr   = %format!("{:p}", *opts.block_cache()),\n            block_size        = *opts.block_size(),\n            block_restart_interval = *opts.block_restart_interval(),\n            max_file_size     = *opts.max_file_size(),\n            compression = ?opts.compression(),\n            reuse_logs  = *opts.reuse_logs(),\n            env_is_some = opts.env().is_some(),\n            info_log_is_some = opts.info_log().is_some(),\n            "Options::default snapshot"\n        );\n\n        assert!(!*opts.create_if_missing());\n        assert!(!*opts.error_if_exists());\n        assert!(!*opts.paranoid_checks());\n\n        assert_eq!(*opts.write_buffer_size(), 4 * 1024 * 1024);\n        assert_eq!(*opts.max_open_files(), 1000);\n\n        assert!((*opts.block_cache()).is_null());\n\n        assert_eq!(*opts.block_size(), 4 * 1024);\n        assert_eq!(*opts.block_restart_interval(), 16);\n        assert_eq!(*opts.max_file_size(), 2 * 1024 * 1024);\n        assert_eq!(*opts.compression(), CompressionType::Snappy);\n        assert!(!*opts.reuse_logs());\n\n        assert!(\n            opts.env().is_none(),\n            "expected Options::default to leave env unset"\n        );\n        assert!(\n            opts.info_log().is_none(),\n            "expected default info_log to be unset"\n        );\n\n        let cmp_name = opts.comparator().name();\n        let fp_name = opts.filter_policy().name();\n        debug!(cmp_name = %cmp_name, fp_name = %fp_name, "trait-object names");\n\n        assert!(\n            !cmp_name.as_ref().is_empty(),\n            "comparator name must be non-empty"\n        );\n        assert_eq!(fp_name.as_ref(), "null-filter-policy");\n\n        trace!("options_defaults_and_accessors_suite: done");\n    }\n\n    #[traced_test]\n    fn options_internal_key_comparator_is_constructible_and_named() {\n        trace!("options_defaults_and_accessors_suite: start");\n\n        let opts = Options::default();\n        let icmp = opts.internal_key_comparator();\n\n        let name = icmp.name();\n        info!(name = %name, "InternalKeyComparator name()");\n\n        assert!(!name.as_ref().is_empty());\n\n        trace!("options_defaults_and_accessors_suite: done");\n    }\n\n    #[traced_test]\n    fn options_debug_format_is_non_panicking_and_includes_field_names() {\n        trace!("options_defaults_and_accessors_suite: start");\n\n        let opts = Options::default();\n        let s = format!("{:?}", opts);\n\n        debug!(dbg = %s, "Options debug output");\n\n        assert!(s.contains("Options"));\n        assert!(s.contains("max_open_files"));\n        assert!(s.contains("write_buffer_size"));\n        assert!(s.contains("compression"));\n\n        trace!("options_defaults_and_accessors_suite: done");\n    }\n}\n', 'after_sha256': 'cfc0f70481e91334bc4e6996ff9a9c98815410b1ab7409fb27ba776ef0b70649', 'before_sha256': '0ea671a8f5738dd5a24c1902ca1f568095e2ceb59bb2578e49d0867f74793ad7'}, 'bitcoinleveldb-tablecache/src/build_table.rs': {'after': '// ---------------- [ File: bitcoinleveldb-tablecache/src/build_table.rs ]\ncrate::ix!();\n\n//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/builder.h]\n//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/builder.cc]\n\n/**\n  | Build a Table file from the contents\n  | of *iter.\n  |\n  | The generated file will be named according\n  | to meta->number.\n  |\n  | On success, the rest of *meta will be\n  | filled with metadata about the generated\n  | table.\n  |\n  | If no data is present in *iter, meta->file_size\n  | will be set to zero, and no Table file\n  | will be produced.\n  |\n  */\npub fn build_table(\n    dbname:      &String,\n    env:         Rc<RefCell<dyn Env>>,\n    options:     &Options,\n    table_cache: *mut TableCache,\n    iter:        *mut LevelDBIterator,\n    meta:        *mut FileMetaData,\n) -> crate::Status {\n    let mut env = EnvWrapper::new(env);\n    build_table_with_env(dbname, &mut env, options, table_cache, iter, meta)\n}\n\n/// Build a table through an existing Env view without creating or releasing an\n/// Rc owner of the environment. The caller keeps the target alive and provides\n/// the Env view\'s synchronization for the duration of this call.\npub fn build_table_with_env(\n    dbname:      &String,\n    env:         &mut dyn Env,\n    options:     &Options,\n    table_cache: *mut TableCache,\n    iter:        *mut LevelDBIterator,\n    meta:        *mut FileMetaData,\n) -> crate::Status {\n    unsafe {\n\n        assert!(\n            !table_cache.is_null(),\n            "build_table: table_cache pointer is null"\n        );\n        assert!(!iter.is_null(), "build_table: iter pointer is null");\n        assert!(!meta.is_null(), "build_table: meta pointer is null");\n\n        let meta_ref: &mut FileMetaData = &mut *meta;\n        let iter_ref: &mut LevelDBIterator = &mut *iter;\n\n        // Reset file_size before we start.\n        meta_ref.set_file_size(0);\n\n        trace!(\n            "build_table: start; dbname=\'{}\', file_number={}",\n            dbname,\n            meta_ref.number()\n        );\n\n        // Seek source iterator to first element.\n        iter_ref.seek_to_first();\n        trace!("build_table: iterator seek_to_first completed");\n\n        // Compute table file name.\n        let fname = table_file_name(dbname, *meta_ref.number());\n        trace!("build_table: table filename = \'{}\'", fname);\n\n        let mut status = crate::Status::ok();\n\n        if iter_ref.valid() {\n            // Acquire a WritableFile from the Env.\n            let mut file_box_ptr: *mut Box<dyn WritableFile> =\n                core::ptr::null_mut();\n            {\n                trace!(\n                    "build_table: calling Env::new_writable_file for \'{}\'",\n                    fname\n                );\n                status = env.new_writable_file(&fname, &mut file_box_ptr);\n            }\n\n            if status.is_ok() {\n                assert!(\n                    !file_box_ptr.is_null(),\n                    "build_table: Env returned OK but file pointer is null"\n                );\n\n                // Reconstruct outer Box<Box<dyn WritableFile>> and borrow\n                // the inner Box<dyn WritableFile> for the builder and I/O.\n                let mut file_holder: Box<Box<dyn WritableFile>> =\n                    Box::from_raw(file_box_ptr);\n                let writable_box: &mut Box<dyn WritableFile> =\n                    file_holder.as_mut();\n                let file_ptr: *mut dyn WritableFile = &mut **writable_box;\n\n                trace!(\n                    "build_table: WritableFile created; constructing TableBuilder (file_number={}, fname=\'{}\')",\n                    meta_ref.number(),\n                    fname\n                );\n\n                let mut builder = TableBuilder::new(options, file_ptr);\n\n                // Initialize smallest key from the first entry.\n                let first_key = iter_ref.key();\n                meta_ref.smallest_mut().decode_from(&first_key);\n\n                // Add all entries in order.\n                while iter_ref.valid() {\n                    let key = iter_ref.key();\n                    let value = iter_ref.value();\n\n                    meta_ref.largest_mut().decode_from(&key);\n                    builder.add(&key, &value);\n\n                    iter_ref.next();\n                }\n\n                // Finish and check for builder errors.\n                status = builder.finish();\n                if status.is_ok() {\n                    meta_ref.set_file_size(builder.file_size());\n                    debug!(\n                        "build_table: builder finished; file_size={} bytes",\n                        meta_ref.file_size()\n                    );\n                    assert!(\n                        *meta_ref.file_size() > 0,\n                        "build_table: builder finished with zero file_size"\n                    );\n                } else {\n                    error!(\n                        "build_table: TableBuilder::finish returned non-OK status (file_number={}, fname=\'{}\')",\n                        meta_ref.number(),\n                        fname\n                    );\n                }\n\n                drop(builder);\n\n                // Now sync and close the file if everything is still OK.\n                if status.is_ok() {\n                    trace!("build_table: syncing file \'{}\'", fname);\n                    status = writable_box.sync();\n                }\n\n                if status.is_ok() {\n                    trace!("build_table: closing file \'{}\'", fname);\n                    status = writable_box.close();\n                }\n\n                // Drop the file holder Box to release heap resources allocated by Env.\n                drop(file_holder);\n\n                // Verify that the table is actually usable via TableCache.\n                if status.is_ok() {\n                    trace!(\n                        "build_table: verifying table via TableCache; file_number={}, file_size={}",\n                        meta_ref.number(),\n                        meta_ref.file_size()\n                    );\n\n                    let cache_ref: &mut TableCache = &mut *table_cache;\n                    let read_options = ReadOptions::default();\n\n                    let it_ptr = cache_ref.new_iterator(\n                        &read_options,\n                        *meta_ref.number(),\n                        *meta_ref.file_size(),\n                        core::ptr::null_mut(),\n                    );\n\n                    if !it_ptr.is_null() {\n                        let it: &mut LevelDBIterator = &mut *it_ptr;\n                        let it_status = it.status();\n                        if !it_status.is_ok() {\n                            error!(\n                                "build_table: verification iterator returned non-OK status (file_number={}, fname=\'{}\')",\n                                meta_ref.number(),\n                                fname\n                            );\n                            status = it_status;\n                        }\n                        // Iterator is heap-allocated; drop it explicitly.\n                        drop(Box::from_raw(it_ptr));\n                    } else {\n                        error!(\n                            "build_table: TableCache::new_iterator returned null for verification (file_number={}, fname=\'{}\')",\n                            meta_ref.number(),\n                            fname\n                        );\n                        let msg =\n                            b"table verification iterator allocation failed";\n                        let msg_slice = Slice::from(&msg[..]);\n                        status = crate::Status::corruption(&msg_slice, None);\n                    }\n                }\n            } else {\n                error!(\n                    "build_table: Env::new_writable_file failed for \'{}\'",\n                    fname\n                );\n            }\n        } else {\n            trace!(\n                "build_table: source iterator invalid at first entry; no table will be created (file_number={})",\n                meta_ref.number()\n            );\n        }\n\n        // Always check for input iterator errors at the end.\n        let iter_status = iter_ref.status();\n        if !iter_status.is_ok() {\n            warn!(\n                "build_table: overriding status with underlying iterator status (file_number={})",\n                meta_ref.number()\n            );\n            status = iter_status;\n        }\n\n        // Decide whether to keep or delete the file.\n        if status.is_ok() && *meta_ref.file_size() > 0 {\n            // Keep it\n            info!(\n                "build_table: successfully built table \'{}\' (file_number={}, file_size={})",\n                fname,\n                meta_ref.number(),\n                meta_ref.file_size()\n            );\n        } else {\n            warn!(\n                ok = status.is_ok(),\n                file_size = *meta_ref.file_size(),\n                "build_table: build failed or produced empty file; deleting \'{}\'",\n                fname\n            );\n\n            let delete_status = env.delete_file(&fname);\n            if !delete_status.is_ok() {\n                error!(\n                    "build_table: failed to delete partial table file \'{}\' after error",\n                    fname\n                );\n            }\n        }\n\n        status\n    }\n}\n\n#[cfg(test)]\nmod build_table_behavior_tests {\n    use super::*;\n    use crate::table_cache_test_support::*;\n    use std::cell::RefCell;\n    use std::ffi::c_void;\n    use std::rc::Rc;\n\n    #[traced_test]\n    fn build_table_with_empty_iterator_produces_no_file() {\n        let (env, state) = make_in_memory_env();\n        let mut options = make_options_with_env(env.clone());\n\n        let dbname = String::from("build_table_empty_iterator_db");\n        let mut meta = FileMetaData::default();\n        meta.set_number(1);\n\n        let mut table_cache = TableCache::new(&dbname, &options, 16);\n        let table_cache_ptr: *mut TableCache = &mut table_cache;\n        let iter_ptr = make_empty_iterator();\n        let meta_ptr: *mut FileMetaData = &mut meta;\n\n        trace!("calling build_table (empty iterator case)");\n        let status = build_table(\n            &dbname,\n            env.clone(),\n            &options,\n            table_cache_ptr,\n            iter_ptr,\n            meta_ptr,\n        );\n\n        unsafe {\n            drop(Box::from_raw(iter_ptr));\n        }\n\n        assert!(status.is_ok(), "status must be OK for empty iterator");\n        assert_eq!(\n            *meta.file_size(),\n            0,\n            "file_size must remain zero when iterator has no entries"\n        );\n\n        let fname = table_file_name(&dbname, *meta.number());\n        let files_snapshot = {\n            let guard = state.lock();\n            guard.files.clone()\n        };\n\n        assert!(\n            !files_snapshot.contains_key(&fname),\n            "no file should be materialized for empty iterator; fname=\'{}\'",\n            fname\n        );\n    }\n\n    #[traced_test]\n    fn build_table_with_single_entry_builds_and_verifies_table() {\n        let (env, _state) = make_in_memory_env();\n        let mut options = make_options_with_env(env.clone());\n\n        let dbname = String::from("build_table_single_entry_db");\n        let mut meta = FileMetaData::default();\n        meta.set_number(2);\n\n        let mut table_cache = TableCache::new(&dbname, &options, 32);\n        let table_cache_ptr: *mut TableCache = &mut table_cache;\n\n        let key = b"alpha".to_vec();\n        let value = b"value-alpha".to_vec();\n        let iter_ptr =\n            make_iterator_from_kv_pairs(&[(key.clone(), value.clone())]);\n        let meta_ptr: *mut FileMetaData = &mut meta;\n\n        trace!("calling build_table (single entry)");\n        let status = build_table(\n            &dbname,\n            env.clone(),\n            &options,\n            table_cache_ptr,\n            iter_ptr,\n            meta_ptr,\n        );\n\n        unsafe {\n            drop(Box::from_raw(iter_ptr));\n        }\n\n        assert!(\n            status.is_ok(),\n            "build_table should succeed: {:?}",\n            status.to_string()\n        );\n        assert!(\n            *meta.file_size() > 0,\n            "file_size must be > 0 for non-empty table"\n        );\n\n        let read_options = ReadOptions::default();\n\n        #[derive(Default)]\n        struct CapturedKV {\n            key:   Option<String>,\n            value: Option<String>,\n        }\n\n        fn capture_kv(arg: *mut c_void, k: &Slice, v: &Slice) -> c_void {\n            unsafe {\n                let c: &mut CapturedKV = &mut *(arg as *mut CapturedKV);\n                c.key = Some(k.to_string());\n                c.value = Some(v.to_string());\n                core::mem::zeroed()\n            }\n        }\n\n        let mut captured = CapturedKV::default();\n        let arg_ptr: *mut c_void = &mut captured as *mut _ as *mut c_void;\n\n        info!("invoking TableCache::get to verify built table contents");\n        let get_status = table_cache.get(\n            &read_options,\n            *meta.number(),\n            *meta.file_size(),\n            &Slice::from("alpha"),\n            arg_ptr,\n            capture_kv,\n        );\n\n        assert!(\n            get_status.is_ok(),\n            "TableCache::get must succeed for existing key: {}",\n            get_status.to_string()\n        );\n        assert_eq!(captured.key.as_deref(), Some("alpha"));\n        assert_eq!(captured.value.as_deref(), Some("value-alpha"));\n    }\n\n    #[traced_test]\n    fn build_table_propagates_iterator_status_on_error() {\n        use bitcoinleveldb_status::StatusCode;\n\n        let (env, _state) = make_in_memory_env();\n        let mut options = make_options_with_env(env.clone());\n\n        let dbname = String::from("build_table_iter_status_db");\n        let mut meta = FileMetaData::default();\n        meta.set_number(3);\n\n        let mut table_cache = TableCache::new(&dbname, &options, 8);\n        let table_cache_ptr: *mut TableCache = &mut table_cache;\n\n        let error_msg = Slice::from("iter-corruption");\n        let iter_status = Status::corruption(&error_msg, None);\n\n        let iface =\n            VecLevelDBIterator::new(Vec::new(), iter_status.clone());\n        let wrapper =\n            bitcoinleveldb_iterator::LevelDBIterator::new(Some(Box::new(\n                iface,\n            )));\n        let iter_ptr = Box::into_raw(Box::new(wrapper));\n\n        let meta_ptr: *mut FileMetaData = &mut meta;\n\n        let status = build_table(\n            &dbname,\n            env.clone(),\n            &options,\n            table_cache_ptr,\n            iter_ptr,\n            meta_ptr,\n        );\n\n        unsafe {\n            drop(Box::from_raw(iter_ptr));\n        }\n\n        assert!(\n            !status.is_ok(),\n            "status must not be OK when underlying iterator has error"\n        );\n        assert_eq!(\n            status.code(),\n            StatusCode::Corruption,\n            "build_table must propagate iterator status code"\n        );\n    }\n}\n', 'after_sha256': '91d5a3a9aa73b8d354a64bb889b585cf932cd1ccd51b174fc673a2f209878d09', 'before_sha256': '15ffa9102de62d2baa5a330fb3798de723743dc5da976fc75b0841807f7e52ca'}, 'bitcoinleveldb-tablecache/src/table_cache.rs': {'after': '// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache.rs ]\n/*!\n  | Thread-safe (provides internal synchronization)\n  |\n  */\n\ncrate::ix!();\n\n//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/table_cache.h]\n//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/table_cache.cc]\n\npub struct TableCache {\n    env:     EnvWrapper,\n    dbname:  String,\n    options: Rc<Options>,\n    table_options: Options,\n    cache:   *mut Cache,\n}\n\nimpl Drop for TableCache {\n    fn drop(&mut self) {\n        unsafe {\n            if self.cache.is_null() {\n                trace!(\n                    "TableCache::drop: cache pointer is null; nothing to free (dbname=\'{}\')",\n                    self.dbname\n                );\n                return;\n            }\n\n            trace!(\n                "TableCache::drop: deleting Cache @ {:?} for dbname=\'{}\'",\n                self.cache,\n                self.dbname\n            );\n\n            let _cache_box: Box<Cache> = Box::from_raw(self.cache);\n            self.cache = core::ptr::null_mut();\n        }\n    }\n}\n\nimpl TableCache {\n    pub fn new(dbname: &String, options: &Options, entries: i32) -> Self {\n        trace!(\n            "TableCache::new: dbname=\'{}\', entries={}",\n            dbname,\n            entries\n        );\n\n        let capacity: usize = if entries <= 0 {\n            1\n        } else {\n            entries as usize\n        };\n\n        let cache_ptr: *mut Cache =\n            bitcoinleveldb_cache::new_lru_cache(capacity);\n\n        unsafe {\n            assert!(\n                !cache_ptr.is_null(),\n                "TableCache::new: new_lru_cache returned null"\n            );\n        }\n\n        let env_rc = {\n            let opt_env = options.env();\n            assert!(\n                opt_env.is_some(),\n                "TableCache::new: Options.env is None; env must be configured before creating TableCache"\n            );\n            opt_env.as_ref().unwrap().clone()\n        };\n\n        TableCache {\n            env:     EnvWrapper::new(env_rc),\n            dbname:  dbname.clone(),\n            options: Rc::new(options.clone()),\n            table_options: options.snapshot_for_table(),\n            cache:   cache_ptr,\n        }\n    }\n\n    #[inline]\n    pub fn env_handle(&self) -> Rc<RefCell<dyn Env>> {\n        trace!(\n            "TableCache::env_handle: cloning Env handle for dbname=\'{}\'",\n            self.dbname\n        );\n        self.env.target()\n    }\n\n    #[inline]\n    pub(crate) fn env_mut(&mut self) -> &mut dyn Env {\n        &mut self.env\n    }\n\n    #[inline]\n    pub fn dbname_str(&self) -> &String {\n        &self.dbname\n    }\n\n    #[inline]\n    pub fn cache_raw(&self) -> *mut Cache {\n        self.cache\n    }\n\n    #[inline]\n    pub fn options_ref(&self) -> &Options {\n        &*self.options\n    }\n\n    /// Cached tables need no independent environment ownership. The cache is\n    /// destroyed in Drop before either environment-owning field is dropped.\n    #[inline]\n    pub(crate) fn table_options_ref(&self) -> &Options {\n        &self.table_options\n    }\n}\n\n#[cfg(test)]\nmod table_cache_construction_tests {\n    use super::*;\n    use crate::table_cache_test_support::*;\n    use std::cell::RefCell;\n    use std::rc::Rc;\n\n    #[traced_test]\n    fn table_cache_new_initializes_with_valid_cache_pointer() {\n        let (env, _state) = make_in_memory_env();\n        let mut options = make_options_with_env(env.clone());\n\n        let dbname = String::from("table_cache_new_db");\n        options.set_max_open_files(64);\n\n        trace!("constructing TableCache via new");\n        let table_cache = TableCache::new(&dbname, &options, 16);\n\n        let _ = table_cache;\n    }\n}\n', 'after_sha256': '5776f2440d894cf0b18218d8b76f565087f66b54723cd243ec9c873a88f8b3f3', 'before_sha256': 'bd8198a9a0e4d384537d0815d60a32af31519706eff8914f28ff0009db8319f4'}, 'bitcoinleveldb-tablecache/src/table_cache_find_table.rs': {'after': '// ---------------- [ File: bitcoinleveldb-tablecache/src/table_cache_find_table.rs ]\ncrate::ix!();\n\nimpl TableCache {\n    pub fn find_table(\n        &mut self,\n        file_number: u64,\n        file_size:   u64,\n        handle:      *mut *mut CacheHandle,\n    ) -> Status {\n        unsafe {\n            assert!(\n                !handle.is_null(),\n                "TableCache::find_table: handle out-parameter pointer is null"\n            );\n\n            trace!(\n                "TableCache::find_table: file_number={}, file_size={}",\n                file_number,\n                file_size\n            );\n\n            let mut status = Status::ok();\n\n            let mut buf = [0u8; core::mem::size_of::<u64>()];\n            let mut v = file_number;\n            for i in 0..core::mem::size_of::<u64>() {\n                buf[i] = (v & 0xff) as u8;\n                v >>= 8;\n            }\n            let key = Slice::from(&buf[..]);\n\n            let cache_ptr = self.cache_raw();\n            assert!(\n                !cache_ptr.is_null(),\n                "TableCache::find_table: cache pointer is null"\n            );\n            let cache_ref = &mut *cache_ptr;\n\n            let mut cache_handle: *mut CacheHandle = cache_ref.lookup(&key);\n            *handle = cache_handle;\n\n            if cache_handle.is_null() {\n                trace!(\n                    "TableCache::find_table: cache miss for file_number={}",\n                    file_number\n                );\n\n                let mut file_holder_ptr_opt: Option<*mut Box<dyn RandomAccessFile>> = None;\n                let mut table_ptr: *mut bitcoinleveldb_table::Table =\n                    core::ptr::null_mut();\n\n                let mut fname =\n                    table_file_name(self.dbname_str(), file_number);\n\n                {\n                    let mut file_box_ptr: *mut Box<dyn RandomAccessFile> =\n                        core::ptr::null_mut();\n\n                    trace!(\n                        "TableCache::find_table: calling Env::new_random_access_file(\'{}\')",\n                        fname\n                    );\n\n                    status = self.env_mut().new_random_access_file(\n                        &fname,\n                        &mut file_box_ptr,\n                    );\n\n                    if status.is_ok() {\n                        assert!(\n                            !file_box_ptr.is_null(),\n                            "TableCache::find_table: Env returned OK but file holder pointer is null"\n                        );\n                        file_holder_ptr_opt = Some(file_box_ptr);\n                    }\n                }\n\n                if !status.is_ok() {\n                    let old_fname =\n                        sst_table_file_name(self.dbname_str(), file_number);\n\n                    let mut file_box_ptr: *mut Box<dyn RandomAccessFile> =\n                        core::ptr::null_mut();\n\n                    trace!(\n                        "TableCache::find_table: primary name \'{}\' failed; trying legacy name \'{}\'",\n                        fname,\n                        old_fname\n                    );\n\n                    let s2 = self.env_mut().new_random_access_file(\n                        &old_fname,\n                        &mut file_box_ptr,\n                    );\n\n                    if s2.is_ok() {\n                        status = Status::ok();\n                        assert!(\n                            !file_box_ptr.is_null(),\n                            "TableCache::find_table: Env returned OK but legacy file holder pointer is null"\n                        );\n                        file_holder_ptr_opt = Some(file_box_ptr);\n                        fname = old_fname;\n                    } else {\n                        trace!(\n                            "TableCache::find_table: legacy name \'{}\' also failed",\n                            old_fname\n                        );\n                    }\n                }\n\n                if status.is_ok() {\n                    trace!(\n                        "TableCache::find_table: opening Table for file=\'{}\'",\n                        fname\n                    );\n\n                    if let Some(file_holder_ptr) = file_holder_ptr_opt {\n                        let adapter =\n                            BorrowedRandomAccessFileAdapter::new(file_holder_ptr, &fname);\n                        let file_rc: Rc<RefCell<dyn RandomAccessFile>> =\n                            Rc::new(RefCell::new(adapter));\n\n                        let open_result = bitcoinleveldb_table::Table::open(\n                            self.table_options_ref(),\n                            file_rc,\n                            file_size,\n                        );\n\n                        match open_result {\n                            Ok(table_box) => {\n                                let table_out: *mut bitcoinleveldb_table::Table =\n                                    Box::into_raw(table_box);\n                                assert!(\n                                    !table_out.is_null(),\n                                    "TableCache::find_table: Table::open returned Ok but table pointer is null"\n                                );\n                                table_ptr = table_out;\n                                status = Status::ok();\n                            }\n                            Err(s) => {\n                                error!(\n                                    "TableCache::find_table: Table::open failed for file=\'{}\'",\n                                    fname\n                                );\n                                status = s;\n                            }\n                        }\n                    } else {\n                        let msg = b"TableCache::find_table: file holder pointer missing despite OK status";\n                        let msg_slice = Slice::from(&msg[..]);\n                        error!(\n                            "TableCache::find_table: missing file holder for file=\'{}\'",\n                            fname\n                        );\n                        status = Status::corruption(&msg_slice, None);\n                    }\n                }\n\n                if !status.is_ok() {\n                    debug_assert!(\n                        table_ptr.is_null(),\n                        "TableCache::find_table: non-OK status but table pointer is non-null"\n                    );\n\n                    if let Some(file_holder_ptr) = file_holder_ptr_opt {\n                        trace!(\n                            "TableCache::find_table: deleting RandomAccessFile holder @ {:?} after open failure",\n                            file_holder_ptr\n                        );\n                        let _file_holder: Box<Box<dyn RandomAccessFile>> =\n                            Box::from_raw(file_holder_ptr);\n                    }\n                } else {\n                    trace!(\n                        "TableCache::find_table: inserting TableAndFile into cache for file_number={}",\n                        file_number\n                    );\n\n                    let file_holder_ptr = file_holder_ptr_opt.expect(\n                        "TableCache::find_table: file holder pointer missing when inserting into cache",\n                    );\n\n                    let tf = Box::new(TableAndFile::new(\n                        file_holder_ptr,\n                        table_ptr,\n                    ));\n\n                    let tf_ptr: *mut TableAndFile = Box::into_raw(tf);\n\n                    cache_handle = cache_ref.insert(\n                        &key,\n                        tf_ptr as *mut c_void,\n                        1,\n                        delete_entry,\n                    );\n                    *handle = cache_handle;\n\n                    debug_assert!(\n                        !cache_handle.is_null(),\n                        "TableCache::find_table: cache insert returned null handle"\n                    );\n                }\n            } else {\n                trace!(\n                    "TableCache::find_table: cache hit for file_number={}",\n                    file_number\n                );\n            }\n\n            status\n        }\n    }\n}\n\n\n#[cfg(test)]\nmod table_cache_find_table_tests {\n    use super::*;\n    use crate::table_cache_test_support::*;\n    use std::cell::RefCell;\n    use std::ptr;\n    use std::rc::Rc;\n\n    #[test]\n    fn cached_table_open_and_eviction_preserve_env_owner_count() {\n        let (env, _state) = make_in_memory_env();\n        let options = make_options_with_env(env.clone());\n        let dbname = String::from("cached_table_env_owner_count");\n        let owners_before_cache = Rc::strong_count(&env);\n        let mut table_cache = TableCache::new(&dbname, &options, 4);\n        let owners_with_cache = Rc::strong_count(&env);\n        assert_eq!(owners_with_cache, owners_before_cache + 2);\n        assert!(table_cache.options_ref().env().is_some());\n        assert!(table_cache.table_options_ref().env().is_none());\n\n        let mut meta = FileMetaData::default();\n        meta.set_number(71);\n        let source = make_iterator_from_kv_pairs(&[(b"key".to_vec(), b"value".to_vec())]);\n        let status = build_table(\n            &dbname,\n            env.clone(),\n            &options,\n            &mut table_cache,\n            source,\n            &mut meta,\n        );\n        unsafe { drop(Box::from_raw(source)); }\n        assert!(status.is_ok());\n        assert_eq!(Rc::strong_count(&env), owners_with_cache);\n\n        table_cache.evict(*meta.number());\n        assert_eq!(Rc::strong_count(&env), owners_with_cache);\n        let iterator = table_cache.new_iterator(\n            &ReadOptions::default(),\n            *meta.number(),\n            *meta.file_size(),\n            ptr::null_mut(),\n        );\n        assert!(!iterator.is_null());\n        assert_eq!(Rc::strong_count(&env), owners_with_cache);\n        unsafe {\n            (*iterator).seek_to_first();\n            assert!((*iterator).valid());\n            assert_eq!((*iterator).key().to_string(), "key");\n            assert_eq!((*iterator).value().to_string(), "value");\n            assert!((*iterator).status().is_ok());\n            drop(Box::from_raw(iterator));\n        }\n        assert_eq!(Rc::strong_count(&env), owners_with_cache);\n        table_cache.evict(*meta.number());\n        assert_eq!(Rc::strong_count(&env), owners_with_cache);\n        drop(table_cache);\n        assert_eq!(Rc::strong_count(&env), owners_before_cache);\n    }\n\n    #[traced_test]\n    fn find_table_returns_error_when_random_access_creation_fails() {\n        let (env, state) = make_in_memory_env();\n        let mut options = make_options_with_env(env.clone());\n\n        {\n            let mut guard = state.lock();\n            guard.fail_new_random_access = true;\n        }\n\n        let dbname = String::from("find_table_fail_env_db");\n        let mut table_cache = TableCache::new(&dbname, &options, 4);\n\n        let mut handle: *mut CacheHandle = ptr::null_mut();\n        let status = table_cache.find_table(100, 1024, &mut handle);\n\n        assert!(\n            !status.is_ok(),\n            "find_table must fail if env::NewRandomAccessFile fails"\n        );\n        assert!(handle.is_null(), "handle must remain null on failure");\n    }\n\n    #[traced_test]\n    fn find_table_caches_and_reuses_table_handles() {\n        let (env, state) = make_in_memory_env();\n        let mut options = make_options_with_env(env.clone());\n\n        let dbname = String::from("find_table_cache_db");\n        let mut meta = FileMetaData::default();\n        meta.set_number(51);\n\n        let mut table_cache = TableCache::new(&dbname, &options, 32);\n        let table_cache_ptr: *mut TableCache = &mut table_cache;\n\n        let key = b"k-cache".to_vec();\n        let val = b"v-cache".to_vec();\n        let iter_ptr =\n            make_iterator_from_kv_pairs(&[(key.clone(), val.clone())]);\n        let meta_ptr: *mut FileMetaData = &mut meta;\n\n        let build_status = build_table(\n            &dbname,\n            env.clone(),\n            &options,\n            table_cache_ptr,\n            iter_ptr,\n            meta_ptr,\n        );\n        unsafe {\n            drop(Box::from_raw(iter_ptr));\n        }\n\n        assert!(build_status.is_ok());\n\n        let fname = table_file_name(&dbname, *meta.number());\n\n        let mut handle1: *mut CacheHandle = std::ptr::null_mut();\n        let s1 =\n            table_cache.find_table(*meta.number(), *meta.file_size(), &mut handle1);\n        assert!(s1.is_ok());\n        assert!(!handle1.is_null());\n\n        let open_count_after_first = {\n            let guard = state.lock();\n            *guard.random_open_count.get(&fname).unwrap_or(&0)\n        };\n\n        let mut handle2: *mut CacheHandle = std::ptr::null_mut();\n        let s2 =\n            table_cache.find_table(*meta.number(), *meta.file_size(), &mut handle2);\n        assert!(s2.is_ok());\n        assert!(!handle2.is_null());\n\n        let open_count_after_second = {\n            let guard = state.lock();\n            *guard.random_open_count.get(&fname).unwrap_or(&0)\n        };\n\n        assert_eq!(\n            open_count_after_second, open_count_after_first,\n            "subsequent find_table calls should hit cache and not reopen file"\n        );\n\n        info!(\n            "handles: handle1={:?}, handle2={:?}",\n            handle1, handle2\n        );\n    }\n}\n', 'after_sha256': '3a7b2b072dd7290f0b37563cb44d9eb50c9a5a39da2773e3342bf7b643f79af5', 'before_sha256': 'eb99da595fe9a1e4c05b6c8821a2b6d167279ba89b4fb797005c07a701ceeb8f'}, 'bitcoinleveldb-versionset/src/version_set.rs': {'after': '// ---------------- [ File: bitcoinleveldb-versionset/src/version_set.rs ]\ncrate::ix!();\n\n#[derive(Builder, CopyGetters, Getters, MutGetters, Setters)]\n#[builder(name = "VersionSetStateBuilder", vis = "pub(crate)", pattern = "owned")]\npub struct VersionSet {\n    env: Box<dyn Env>,\n\n    #[getset(get = "pub(crate)")]\n    dbname: String,\n\n    #[getset(get_copy = "pub(crate)")]\n    options: *const Options,\n\n    #[getset(get_copy = "pub(crate)")]\n    table_cache: *const TableCache,\n\n    #[getset(get = "pub(crate)")]\n    icmp: InternalKeyComparator,\n\n    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]\n    next_file_number: u64,\n\n    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]\n    manifest_file_number: u64,\n\n    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]\n    last_sequence: u64,\n\n    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]\n    log_number: u64,\n\n    /// 0 or backing store for memtable being\n    /// compacted\n    /// \n    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]\n    prev_log_number: u64,\n\n    /// Opened lazily\n    /// \n    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]\n    descriptor_file: *mut dyn WritableFile,\n\n    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]\n    descriptor_log: *mut LogWriter,\n\n    /// Head of circular doubly-linked list\n    /// of versions.\n    /// \n    #[getset(get_mut = "pub(crate)")]\n    dummy_versions: Version,\n\n    /// == dummy_versions_.prev_\n    /// \n    #[getset(get_copy = "pub(crate)", set = "pub(crate)")]\n    current: *mut Version,\n\n    /// Per-level key at which the next compaction\n    /// at that level should start.\n    /// \n    /// Either an empty string, or a valid\n    /// \n    /// InternalKey.\n    /// \n    #[getset(get_mut = "pub(crate)")]\n    compact_pointer: [String; NUM_LEVELS],\n}\n\nimpl VersionSet {\n    /// Borrow the wrapper owned by this VersionSet without cloning its Env Rc.\n    /// The wrapper serializes each forwarded call and releases its gate before\n    /// the caller can reacquire a database mutex.\n    pub(crate) fn env_mut(&mut self) -> &mut dyn Env {\n        self.env.as_mut()\n    }\n\n    #[inline]\n    pub(crate) fn dummy_versions_mut_ptr(&mut self) -> *mut Version {\n        core::ptr::addr_of_mut!(self.dummy_versions)\n    }\n}\n\nimpl VersionSet {\n    pub fn new(\n        dbname:      &String,\n        options:     *const Options,\n        table_cache: *mut TableCache,\n        cmp:         *const InternalKeyComparator,\n    ) -> Box<Self> {\n        trace!(\n            "VersionSet::new(boxed): enter; dbname=\'{}\' options_ptr={:p} table_cache_ptr={:p} cmp_ptr={:p}",\n            dbname, options, table_cache, cmp\n        );\n\n        let vset = VersionSet::new_internal(dbname, options, table_cache, cmp);\n\n        trace!(\n            "VersionSet::new(boxed): exit; next_file_number={} manifest_file_number={} current={:p}",\n            vset.next_file_number(),\n            vset.manifest_file_number(),\n            vset.current()\n        );\n\n        vset\n    }\n\n    pub(crate) fn new_internal(\n        dbname:      &String,\n        options:     *const Options,\n        table_cache: *mut TableCache,\n        cmp:         *const InternalKeyComparator,\n    ) -> Box<Self> {\n        assert!(!options.is_null(), "VersionSet::new_internal: options is null");\n        assert!(!cmp.is_null(), "VersionSet::new_internal: cmp is null");\n\n        let env_box: Box<dyn Env> = unsafe {\n            let env_opt = (*options).env();\n            let env_rc = env_opt\n                .as_ref()\n                .expect("VersionSet::new_internal: Options.env is None")\n                .clone();\n            Box::new(EnvWrapper::new(env_rc))\n        };\n\n        let icmp_copy: InternalKeyComparator =\n            unsafe { InternalKeyComparator::new((*cmp).user_comparator()) };\n\n        let dummy_files: [Vec<*mut FileMetaData>; NUM_LEVELS] =\n            core::array::from_fn(|_| Vec::new());\n\n        let compact_pointer_init: [String; NUM_LEVELS] =\n            core::array::from_fn(|_| String::new());\n\n        // Build the dummy sentinel with null links; we will patch links *after* boxing.\n        let dummy_versions = VersionBuilder::default()\n            .vset(Self::null_versionset_interface_ptr())\n            .next(core::ptr::null_mut())\n            .prev(core::ptr::null_mut())\n            .refs(0)\n            .files(dummy_files)\n            .file_to_compact(core::ptr::null_mut())\n            .file_to_compact_level(-1)\n            .compaction_score(-1.0)\n            .compaction_level(-1)\n            .build()\n            .unwrap();\n\n        // IMPORTANT: heap-allocate first so that &mut *vset has a stable address.\n        let mut vset: Box<VersionSet> = Box::new(\n            VersionSetStateBuilder::default()\n                .env(env_box)\n                .dbname(dbname.clone())\n                .options(options)\n                .table_cache(table_cache as *const TableCache)\n                .icmp(icmp_copy)\n                .next_file_number(2)\n                .manifest_file_number(0)\n                .last_sequence(0)\n                .log_number(0)\n                .prev_log_number(0)\n                .descriptor_file(Self::null_writable_file_ptr())\n                .descriptor_log(core::ptr::null_mut())\n                .dummy_versions(dummy_versions)\n                .current(core::ptr::null_mut())\n                .compact_pointer(compact_pointer_init)\n                .build()\n                .unwrap(),\n        );\n\n        // Now it is safe to take self-referential pointers.\n        let vset_iface_ptr: *mut dyn VersionSetInterface =\n            (&mut *vset as &mut dyn VersionSetInterface) as *mut dyn VersionSetInterface;\n\n        let dummy_ptr: *mut Version = vset.dummy_versions_mut_ptr();\n\n        unsafe {\n            // Make dummy a proper sentinel at its final (boxed) address.\n            (*dummy_ptr).set_vset(vset_iface_ptr);\n            (*dummy_ptr).set_next(dummy_ptr);\n            (*dummy_ptr).set_prev(dummy_ptr);\n        }\n\n        // Create the initial current version (heap) and link it to the sentinel.\n        let initial_files: [Vec<*mut FileMetaData>; NUM_LEVELS] =\n            core::array::from_fn(|_| Vec::new());\n\n        let mut initial_v = Box::new(\n            VersionBuilder::default()\n                .vset(vset_iface_ptr)\n                .next(core::ptr::null_mut())\n                .prev(core::ptr::null_mut())\n                .refs(0)\n                .files(initial_files)\n                .file_to_compact(core::ptr::null_mut())\n                .file_to_compact_level(-1)\n                .compaction_score(-1.0)\n                .compaction_level(-1)\n                .build()\n                .unwrap(),\n        );\n\n        assert_eq!(\n            unsafe { *initial_v.refs() },\n            0,\n            "VersionSet::new_internal: initial version refs must be 0"\n        );\n\n        let v_ptr: *mut Version = &mut *initial_v;\n\n        vset.set_current(v_ptr);\n        initial_v.ref_();\n\n        initial_v.set_prev(dummy_ptr);\n        initial_v.set_next(dummy_ptr);\n\n        unsafe {\n            (*dummy_ptr).set_next(v_ptr);\n            (*dummy_ptr).set_prev(v_ptr);\n        }\n\n        let _leaked: *mut Version = Box::into_raw(initial_v);\n        let _ = _leaked;\n\n        #[cfg(any(test, debug_assertions))]\n        {\n            vset.trace_version_list_pointers_for_versionset_move_diagnostics(\n                "VersionSet::new_internal: post-box init",\n            );\n            vset.assert_version_list_sentinel_consistency_for_versionset_move_diagnostics(\n                "VersionSet::new_internal: post-box init",\n            );\n        }\n\n        vset\n    }\n}\n\nimpl VersionSetInterface for VersionSet {}\nimpl VersionSetVersionInterface for VersionSet {}\nimpl CompactionInterface for VersionSet {}\n\n#[cfg(test)]\nmod version_set_exhaustive_test_suite {\n    use super::*;\n\n    #[traced_test]\n    fn versionset_new_initializes_expected_state_invariants() {\n        let dir = build_unique_temporary_database_directory_path("versionset_new_invariants");\n        std::fs::create_dir_all(&dir).unwrap();\n        let dbname = dir.to_string_lossy().to_string();\n\n        let env = PosixEnv::shared();\n        let options = Box::new(Options::with_env(env));\n        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));\n        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 4));\n\n        let vs = VersionSet::new(\n            &dbname,\n            options.as_ref(),\n            table_cache.as_mut() as *mut TableCache,\n            icmp.as_ref() as *const InternalKeyComparator,\n        );\n\n        debug!(\n            dbname = %vs.dbname(),\n            next_file_number = vs.next_file_number(),\n            manifest_file_number = vs.manifest_file_number(),\n            last_sequence = vs.last_sequence(),\n            log_number = vs.log_number(),\n            prev_log_number = vs.prev_log_number(),\n            current_ptr = %format!("{:p}", vs.current()),\n            "constructed versionset state"\n        );\n\n        assert_eq!(vs.dbname().as_str(), dbname.as_str(), "dbname must be stored");\n        assert!(!vs.options().is_null(), "options pointer must not be null");\n        assert!(!vs.table_cache().is_null(), "table_cache pointer must not be null");\n\n        assert_eq!(vs.next_file_number(), 2, "default next_file_number must be 2");\n        assert_eq!(vs.manifest_file_number(), 0, "default manifest_file_number must be 0");\n        assert_eq!(vs.last_sequence(), 0, "default last_sequence must be 0");\n        assert_eq!(vs.log_number(), 0, "default log_number must be 0");\n        assert_eq!(vs.prev_log_number(), 0, "default prev_log_number must be 0");\n\n        assert!(!vs.current().is_null(), "current version must be initialized in new_internal");\n\n        remove_directory_tree_best_effort(&dir);\n    }\n}\n', 'after_sha256': '2fa6dd52d536e3ddfecd311daa00bd2a48ffc0f98cdd5e3eaffdaac83556d30e', 'before_sha256': '50be5b208b93fa6b0d424e213da7ae9e8f9a892828ff334ebcfacbe5c9ceb943'}, 'bitcoinleveldb-versionset/src/version_set_log_and_apply.rs': {'after': '// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_log_and_apply.rs ]\ncrate::ix!();\n\nimpl VersionEditLogAndApply for VersionSet {\n    /// Apply *edit to the current version to form a new descriptor that is both saved to\n    /// persistent state and installed as the new current version.  Will release *mu while actually\n    /// writing to the file.\n    /// \n    /// REQUIRES: *mu is held on entry.\n    /// \n    /// REQUIRES: no other thread concurrently calls LogAndApply()\n    ///\n    #[EXCLUSIVE_LOCKS_REQUIRED(mu)]\n    fn log_and_apply(&mut self, edit: *mut VersionEdit, mu: *mut RawMutex) -> Status {\n        let tid = std::thread::current().id();\n        let t_enter = std::time::Instant::now();\n\n        tracing::trace!(\n            ?tid,\n            edit_ptr = edit as usize,\n            mu_ptr = mu as usize,\n            current_ptr = self.current() as usize,\n            "VersionSet::log_and_apply: enter"\n        );\n\n        assert!(!edit.is_null(), "VersionSet::log_and_apply: edit is null");\n        assert!(!mu.is_null(), "VersionSet::log_and_apply: mu is null");\n        assert!(\n            !self.current().is_null(),\n            "VersionSet::log_and_apply: current is null"\n        );\n\n        unsafe {\n            let e: &mut VersionEdit = &mut *edit;\n\n            if *e.has_log_number() {\n                assert!(\n                    *e.log_number() >= self.log_number(),\n                    "VersionSet::log_and_apply: edit.log_number < log_number_"\n                );\n                assert!(\n                    *e.log_number() < self.next_file_number(),\n                    "VersionSet::log_and_apply: edit.log_number >= next_file_number_"\n                );\n            } else {\n                e.set_log_number(self.log_number());\n            }\n\n            if !*e.has_prev_log_number() {\n                e.set_prev_log_number(self.prev_log_number());\n            }\n\n            e.set_next_file(self.next_file_number());\n            e.set_last_sequence(self.last_sequence());\n\n            // Version* v = new Version(this);\n            let vset_iface_ptr: *mut dyn VersionSetInterface =\n                (self as &mut dyn VersionSetInterface) as *mut dyn VersionSetInterface;\n\n            let v_files: [Vec<*mut FileMetaData>; NUM_LEVELS] =\n                core::array::from_fn(|_| Vec::new());\n\n            let v_ptr: *mut Version = Box::into_raw(Box::new(\n                VersionBuilder::default()\n                    .vset(vset_iface_ptr)\n                    .next(core::ptr::null_mut())\n                    .prev(core::ptr::null_mut())\n                    .refs(0)\n                    .files(v_files)\n                    .file_to_compact(core::ptr::null_mut())\n                    .file_to_compact_level(-1)\n                    .compaction_score(-1.0)\n                    .compaction_level(-1)\n                    .build()\n                    .unwrap(),\n            ));\n\n            {\n                // Builder builder(this, current_);\n                tracing::trace!(\n                    ?tid,\n                    v_ptr = v_ptr as usize,\n                    base_ptr = self.current() as usize,\n                    "VersionSet::log_and_apply: constructing builder/applying edit"\n                );\n\n                let mut builder = VersionSetBuilder::new(self, self.current());\n                builder.apply(edit);\n\n                tracing::trace!(\n                    ?tid,\n                    v_ptr = v_ptr as usize,\n                    "VersionSet::log_and_apply: builder.save_to begin"\n                );\n                builder.save_to(v_ptr);\n                tracing::trace!(\n                    ?tid,\n                    v_ptr = v_ptr as usize,\n                    "VersionSet::log_and_apply: builder.save_to end"\n                );\n            }\n\n            self.finalize(v_ptr);\n\n            // Initialize new descriptor log file if necessary by creating\n            // a temporary file that contains a snapshot of the current version.\n            let mut new_manifest_file: String = String::new();\n            let mut s: Status = Status::ok();\n\n            if self.descriptor_log().is_null() {\n                tracing::trace!(\n                    ?tid,\n                    dbname = %self.dbname(),\n                    "VersionSet::log_and_apply: descriptor_log is null; creating new MANIFEST"\n                );\n\n                // No reason to unacquire_from_raw_mutex *mu here since we only hit this path in the\n                // first call to LogAndApply (when opening the database).\n                assert!(\n                    self.descriptor_file().is_null(),\n                    "VersionSet::log_and_apply: descriptor_log is null but descriptor_file is not null"\n                );\n\n                new_manifest_file =\n                    descriptor_file_name(self.dbname(), self.manifest_file_number());\n                e.set_next_file(self.next_file_number());\n\n                assert!(\n                    (*self.options()).env().is_some(),\n                    "VersionSet::log_and_apply: Options.env is None"\n                );\n\n                let mut file_box_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();\n\n                tracing::info!(\n                    ?tid,\n                    dbname = %self.dbname(),\n                    manifest = %new_manifest_file,\n                    "VersionSet::log_and_apply: Env::new_writable_file begin"\n                );\n\n                let t_create_manifest = std::time::Instant::now();\n                s = self.env_mut()\n                    .new_writable_file(&new_manifest_file, &mut file_box_ptr);\n\n                tracing::info!(\n                    ?tid,\n                    dbname = %self.dbname(),\n                    manifest = %new_manifest_file,\n                    status_ok = s.is_ok(),\n                    status = %s.to_string(),\n                    elapsed_ms = t_create_manifest.elapsed().as_millis() as u64,\n                    "VersionSet::log_and_apply: Env::new_writable_file end"\n                );\n\n                if s.is_ok() {\n                    assert!(\n                        !file_box_ptr.is_null(),\n                        "VersionSet::log_and_apply: Env returned OK but file pointer is null"\n                    );\n\n                    // Take ownership of Env allocation and store as a raw *mut dyn WritableFile.\n                    let file_holder: Box<Box<dyn WritableFile>> = Box::from_raw(file_box_ptr);\n                    let file_inner: Box<dyn WritableFile> = *file_holder;\n                    let raw_file: *mut dyn WritableFile = Box::into_raw(file_inner);\n\n                    self.set_descriptor_file(raw_file);\n\n                    let dest: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(\n                        BorrowedWritableFileForManifest::new(self.descriptor_file()),\n                    ));\n\n                    let lw = LogWriter::new(dest, 0);\n                    self.set_descriptor_log(Box::into_raw(Box::new(lw)));\n\n                    tracing::info!(\n                        ?tid,\n                        dbname = %self.dbname(),\n                        descriptor_log_ptr = self.descriptor_log() as usize,\n                        descriptor_file_ptr = (self.descriptor_file() as *mut ()) as usize,\n                        "VersionSet::log_and_apply: write_snapshot begin"\n                    );\n\n                    let t_snapshot = std::time::Instant::now();\n                    s = self.write_snapshot();\n\n                    tracing::info!(\n                        ?tid,\n                        dbname = %self.dbname(),\n                        status_ok = s.is_ok(),\n                        status = %s.to_string(),\n                        elapsed_ms = t_snapshot.elapsed().as_millis() as u64,\n                        "VersionSet::log_and_apply: write_snapshot end"\n                    );\n                }\n            }\n\n            // Unacquire_from_raw_mutex during expensive MANIFEST log write\n            {\n                tracing::info!(\n                    ?tid,\n                    mu_ptr = mu as usize,\n                    "VersionSet::log_and_apply: about to unacquire_from_raw_mutex mu for MANIFEST write"\n                );\n\n                (*mu).unlock();\n\n                let t_unacquire_from_raw_mutexed = std::time::Instant::now();\n\n                tracing::info!(\n                    ?tid,\n                    mu_ptr = mu as usize,\n                    "VersionSet::log_and_apply: mu unacquire_from_raw_mutexed"\n                );\n\n                // Write new record to MANIFEST log\n                if s.is_ok() {\n                    let mut record: String = String::new();\n                    e.encode_to(&mut record as *mut String);\n\n                    tracing::info!(\n                        ?tid,\n                        dbname = %self.dbname(),\n                        bytes = record.len() as u64,\n                        descriptor_log_ptr = self.descriptor_log() as usize,\n                        "VersionSet::log_and_apply: descriptor_log.add_record begin"\n                    );\n\n                    let t_add_record = std::time::Instant::now();\n                    let rec_slice = Slice::from(record.as_bytes());\n                    s = (*self.descriptor_log()).add_record(&rec_slice);\n\n                    tracing::info!(\n                        ?tid,\n                        dbname = %self.dbname(),\n                        status_ok = s.is_ok(),\n                        status = %s.to_string(),\n                        elapsed_ms = t_add_record.elapsed().as_millis() as u64,\n                        "VersionSet::log_and_apply: descriptor_log.add_record end"\n                    );\n\n                    if s.is_ok() {\n                        tracing::info!(\n                            ?tid,\n                            dbname = %self.dbname(),\n                            descriptor_file_ptr = (self.descriptor_file() as *mut ()) as usize,\n                            "VersionSet::log_and_apply: descriptor_file.sync begin"\n                        );\n\n                        let t_sync = std::time::Instant::now();\n                        s = (*self.descriptor_file()).sync();\n\n                        tracing::info!(\n                            ?tid,\n                            dbname = %self.dbname(),\n                            status_ok = s.is_ok(),\n                            status = %s.to_string(),\n                            elapsed_ms = t_sync.elapsed().as_millis() as u64,\n                            "VersionSet::log_and_apply: descriptor_file.sync end"\n                        );\n                    }\n\n                    if !s.is_ok() {\n                        tracing::error!(\n                            ?tid,\n                            dbname = %self.dbname(),\n                            status = %s.to_string(),\n                            "VersionSet::log_and_apply: MANIFEST write failed"\n                        );\n                    }\n                }\n\n                // If we just created a new descriptor file, install it by writing a\n                // new CURRENT file that points to it.\n                if s.is_ok() && !new_manifest_file.is_empty() {\n                    assert!(\n                        (*self.options()).env().is_some(),\n                        "VersionSet::log_and_apply: Options.env is None"\n                    );\n\n                    tracing::info!(\n                        ?tid,\n                        dbname = %self.dbname(),\n                        manifest_file_number = self.manifest_file_number(),\n                        "VersionSet::log_and_apply: set_current_file begin"\n                    );\n\n                    let t_set_current = std::time::Instant::now();\n                    // CURRENT publication must not clone/drop an Env Rc while\n                    // mu is released. The persistent wrapper gates each Env\n                    // call and holds no gate when mu is reacquired below.\n                    let current_dbname = self.dbname().clone();\n                    let current_manifest_number = self.manifest_file_number();\n                    s = set_current_file_with_env(\n                        self.env_mut(),\n                        &current_dbname,\n                        current_manifest_number,\n                    );\n\n                    tracing::info!(\n                        ?tid,\n                        dbname = %self.dbname(),\n                        status_ok = s.is_ok(),\n                        status = %s.to_string(),\n                        elapsed_ms = t_set_current.elapsed().as_millis() as u64,\n                        "VersionSet::log_and_apply: set_current_file end"\n                    );\n                }\n\n                tracing::info!(\n                    ?tid,\n                    mu_ptr = mu as usize,\n                    unacquire_from_raw_mutexed_elapsed_ms = t_unacquire_from_raw_mutexed.elapsed().as_millis() as u64,\n                    "VersionSet::log_and_apply: about to re-acquire_from_raw_mutex mu after MANIFEST write"\n                );\n\n                let t_reacquire_from_raw_mutex_wait = std::time::Instant::now();\n                (*mu).lock();\n\n                tracing::info!(\n                    ?tid,\n                    mu_ptr = mu as usize,\n                    reacquire_from_raw_mutex_wait_ms = t_reacquire_from_raw_mutex_wait.elapsed().as_millis() as u64,\n                    "VersionSet::log_and_apply: mu re-acquire_from_raw_mutexed"\n                );\n            }\n\n            // Install the new version\n            if s.is_ok() {\n                tracing::info!(\n                    ?tid,\n                    dbname = %self.dbname(),\n                    v_ptr = v_ptr as usize,\n                    "VersionSet::log_and_apply: append_version begin"\n                );\n\n                let t_append = std::time::Instant::now();\n                self.append_version(v_ptr);\n\n                tracing::info!(\n                    ?tid,\n                    dbname = %self.dbname(),\n                    v_ptr = v_ptr as usize,\n                    elapsed_ms = t_append.elapsed().as_millis() as u64,\n                    "VersionSet::log_and_apply: append_version end"\n                );\n\n                self.set_log_number(*e.log_number());\n                self.set_prev_log_number(*e.prev_log_number());\n            } else {\n                drop(Box::from_raw(v_ptr));\n\n                if !new_manifest_file.is_empty() {\n                    if !self.descriptor_log().is_null() {\n                        drop(Box::from_raw(self.descriptor_log()));\n                        self.set_descriptor_log(core::ptr::null_mut());\n                    }\n\n                    if !self.descriptor_file().is_null() {\n                        drop(Box::<dyn WritableFile>::from_raw(self.descriptor_file()));\n                        self.set_descriptor_file(VersionSet::null_writable_file_ptr());\n                    }\n\n                    assert!(\n                        (*self.options()).env().is_some(),\n                        "VersionSet::log_and_apply: Options.env is None"\n                    );\n\n                    let del_status = self.env_mut().delete_file(&new_manifest_file);\n                    if !del_status.is_ok() {\n                        tracing::warn!(\n                            ?tid,\n                            dbname = %self.dbname(),\n                            manifest = %new_manifest_file,\n                            status = %del_status.to_string(),\n                            "VersionSet::log_and_apply: failed to delete new manifest after error"\n                        );\n                    }\n                }\n            }\n\n            tracing::info!(\n                ?tid,\n                dbname = %self.dbname(),\n                status_ok = s.is_ok(),\n                status = %s.to_string(),\n                elapsed_ms = t_enter.elapsed().as_millis() as u64,\n                "VersionSet::log_and_apply: exit"\n            );\n\n            s\n        }\n    }\n}\n\n#[cfg(test)]\nmod version_set_log_and_apply_exhaustive_test_suite {\n    use super::*;\n\n    #[traced_test]\n    fn log_and_apply_persists_edit_so_recover_sees_it() {\n        let dir = build_unique_temporary_database_directory_path("versionset_log_and_apply_persist");\n        std::fs::create_dir_all(&dir).unwrap();\n        let dbname = Box::new(dir.to_string_lossy().to_string());\n\n        let env = PosixEnv::shared();\n        let mut options = Box::new(Options::with_env(env.clone()));\n        options.set_create_if_missing(true);\n        options.set_error_if_exists(false);\n\n        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));\n\n        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));\n        let mut mu = Box::new(RawMutex::INIT);\n\n        let mut vs = VersionSet::new(\n            dbname.as_ref(),\n            options.as_ref(),\n            table_cache.as_mut() as *mut TableCache,\n            icmp.as_ref() as *const InternalKeyComparator,\n        );\n\n        let mut save_manifest: bool = false;\n        let st0 = vs.recover(&mut save_manifest as *mut bool);\n        assert_status_is_ok_or_panic(&st0, "recover before log_and_apply");\n\n        let manifest_before = find_manifest_file_in_directory(&dir).expect("MANIFEST-* must exist after recover");\n        let size_before = std::fs::metadata(&manifest_before).unwrap().len();\n        debug!(path = %manifest_before.display(), size_before, "manifest before");\n\n        let mut edit = VersionEdit::default();\n        let fnum = vs.new_file_number();\n        let smallest = make_value_internal_key_for_user_key("a", 1);\n        let largest = make_value_internal_key_for_user_key("k", 1);\n        edit.add_file(1, fnum, 100, &smallest, &largest);\n\n        let _guard = RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);\n        let st1 = vs.log_and_apply(&mut edit as *mut VersionEdit, mu.as_mut() as *mut RawMutex);\n        assert_status_is_ok_or_panic(&st1, "log_and_apply");\n\n        let size_after = std::fs::metadata(&manifest_before).unwrap().len();\n        debug!(size_after, "manifest after");\n        assert!(size_after >= size_before, "manifest must not shrink after log_and_apply");\n\n        let mut options2 = Box::new(Options::with_env(env));\n        options2.set_create_if_missing(false);\n        options2.set_error_if_exists(false);\n\n        let icmp2 = Box::new(build_internal_key_comparator_from_database_options(options2.as_ref()));\n\n        let mut table_cache2 = Box::new(TableCache::new(dbname.as_ref(), options2.as_ref(), 128));\n\n        let mut vs2 = VersionSet::new(\n            dbname.as_ref(),\n            options2.as_ref(),\n            table_cache2.as_mut() as *mut TableCache,\n            icmp2.as_ref() as *const InternalKeyComparator,\n        );\n\n        let mut save_manifest2: bool = false;\n        let st2 = vs2.recover(&mut save_manifest2 as *mut bool);\n        info!(save_manifest2, status = ?st2, "recover after log_and_apply");\n        assert_status_is_ok_or_panic(&st2, "recover after log_and_apply");\n\n        let n = vs2.num_level_files(1);\n        debug!(n, "num_level_files(1) after recover");\n        assert!(n >= 1, "expected at least one file in level 1 after recover");\n\n        remove_directory_tree_best_effort(&dir);\n    }\n\n    #[traced_test]\n    fn log_and_apply_multiple_edits_monotonically_increase_file_numbers() {\n        let dir = build_unique_temporary_database_directory_path("versionset_log_and_apply_file_numbers");\n        std::fs::create_dir_all(&dir).unwrap();\n        let dbname = Box::new(dir.to_string_lossy().to_string());\n\n        let env = PosixEnv::shared();\n        let mut options = Box::new(Options::with_env(env));\n        options.set_create_if_missing(true);\n        options.set_error_if_exists(false);\n\n        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));\n\n        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));\n        let mut mu = Box::new(RawMutex::INIT);\n\n        let mut vs = VersionSet::new(\n            dbname.as_ref(),\n            options.as_ref(),\n            table_cache.as_mut() as *mut TableCache,\n            icmp.as_ref() as *const InternalKeyComparator,\n        );\n\n        let mut save_manifest: bool = false;\n        let st0 = vs.recover(&mut save_manifest as *mut bool);\n        assert_status_is_ok_or_panic(&st0, "recover");\n\n        let f1 = vs.new_file_number();\n        let f2 = vs.new_file_number();\n        debug!(f1, f2, "allocated new file numbers");\n        assert!(f2 > f1, "file numbers must be monotonically increasing");\n\n        let _guard = RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);\n\n        let mut e1 = VersionEdit::default();\n        e1.add_file(0, f1, 10, &make_value_internal_key_for_user_key("a", 1), &make_value_internal_key_for_user_key("b", 1));\n        let st1 = vs.log_and_apply(&mut e1 as *mut VersionEdit, mu.as_mut() as *mut RawMutex);\n        assert_status_is_ok_or_panic(&st1, "log_and_apply e1");\n\n        let mut e2 = VersionEdit::default();\n        e2.add_file(0, f2, 10, &make_value_internal_key_for_user_key("c", 1), &make_value_internal_key_for_user_key("d", 1));\n        let st2 = vs.log_and_apply(&mut e2 as *mut VersionEdit, mu.as_mut() as *mut RawMutex);\n        assert_status_is_ok_or_panic(&st2, "log_and_apply e2");\n\n        let l0 = vs.num_level_files(0);\n        debug!(l0, "num_level_files(0)");\n        assert!(l0 >= 2, "expected at least two L0 files after two edits");\n\n        remove_directory_tree_best_effort(&dir);\n    }\n}\n', 'after_sha256': '1c9c03b5aa08148c5ca2f202bcda4b58d9b3fc07b19d213f1e8e023a7775d75a', 'before_sha256': 'e9586da453582d075a52b63966f4348fd6ce9c6ff2a2fcd5b11829f5429bf38a'}}
DOCUMENT='# SparseMerge internal environment ownership repair 011\n\nThis candidate is based on held borrow repair e1736f59dc8e10b301cd4c0c3ea10302a9c46e1a.\nIt does not update master or merge the held branch. Source capture 010 binds the\nreviewed old files. The earlier native allocator failure and its raw evidence\nremain retained; its precise damaging instruction has not been established.\n\nThe candidate removes short-lived Env Rc owners from audited internal background\npaths. Table construction, memtable rotation and manifest publication borrow the\nexisting EnvWrapper. Each Env method uses the existing per-target access gate;\nno new gate is held across DB-mutex reacquisition. TableCache keeps its stable\nEnv-owning Options allocation for VersionSet, but cached SSTables receive a\nfield-by-field settings snapshot with env=None. Comparator/filter Arc identity,\ncache pointer and all other option fields are preserved. Public Options::clone,\nTable::open and owning file/table helper signatures are preserved.\n\nThe scope is these internal DB paths. Public Options.env remains Rc<RefCell<dyn\nEnv>>; this does not make arbitrary cross-thread sharing or every raw-pointer DB\nAPI safe. Compaction, scheduling and shutdown waits are unchanged.\n\nThe replay/controller creates repair/leveldb-sparsemerge-ownership and a dedicated\nsibling worktree. It seeds an independent regular-file Cargo cache from retained\nartifacts, without hard links or incremental state. The seed is bounded at 4 GiB,\nand requires its copy size plus 2 GiB free space. Existing cache bytes and the\nretained failing ordinary binary are never compilation destinations.\n\nCompilation is offline/locked, one Cargo job, under 4 GiB/600 s and one CPU. Unit and\nrotation cases have 512 MiB/30 s; each of three ordinary SparseMerge runs has 2 GiB/600 s,\n512 MiB test data and 4096 entries. A reviewed guard derivative extends the existing\ndata/entry monitor to the named stress profile. Kernel/service controls must be\nverified before commands are admitted. No debugger or deliberate invalid-memory\ncontrol is used.\n\nValidation selects ownership snapshot/cache lifecycle tests, existing table build\ncontracts, missing-environment contracts, manifest persistence, the 64-round\nasynchronous rotation regression, and three unchanged 100000-key SparseMerge runs.\nEvery selected case must be listed uniquely, report exactly one passing test and\nexit normally. Original SparseMerge also requires its completion marker. This is\nnot a full suite or coverage claim.\n\nA qualified checkpoint is committed only in the isolated candidate branch. Any\nfailed or interrupted admitted native attempt blocks later automatic qualification\nof the same candidate, so a passing retry cannot erase a retained failure. Build\nfailures and partial private caches are retained. Interrupted validation/commit\nfinalization is retained for review rather than reset or overwritten. Master,\nother existing refs, user files, retained source and original binary are checked\nagain. No merge, tag, remote operation or cleanup is performed.\n'


import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import selectors
import signal
import stat
import subprocess
import sys
import time
MAX_BRANCHES = 40
MAX_ENTRIES = 8000
MAX_PATHS = 300
MAX_FILE_BYTES = 8 * 1024 * 1024
MAX_TOTAL_READ = 64 * 1024 * 1024
MAX_OUTPUT = 4 * 1024 * 1024
COMMAND_SECONDS = 20
TOTAL_SECONDS = 180

class AuditError(RuntimeError):
    pass

def child_limits():
    resource.setrlimit(resource.RLIMIT_AS, (256 * 1024 * 1024,) * 2)
    resource.setrlimit(resource.RLIMIT_CPU, (COMMAND_SECONDS,) * 2)
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_FSIZE, (0, 0))
    resource.setrlimit(resource.RLIMIT_NOFILE, (64, 64))

def bounded_file(path):
    path = Path(path)
    before = path.lstat()
    if not stat.S_ISREG(before.st_mode) or before.st_size > MAX_FILE_BYTES:
        raise AuditError('File exceeds its finite allowance or is nonregular: ' + str(path))
    fd = os.open(path, os.O_RDONLY | getattr(os, 'O_NOFOLLOW', 0))
    with os.fdopen(fd, 'rb') as stream:
        raw = stream.read(MAX_FILE_BYTES + 1)
        after = os.fstat(stream.fileno())
    current = path.lstat()
    fields = ('st_dev', 'st_ino', 'st_size', 'st_mtime_ns', 'st_ctime_ns')
    if len(raw) > MAX_FILE_BYTES or any((getattr(before, k) != getattr(after, k) for k in fields)) or any((getattr(before, k) != getattr(current, k) for k in fields)):
        raise AuditError('File changed while reading or exceeded its allowance: ' + str(path))
    return raw

def fingerprint(path):
    path = Path(path)
    try:
        info = path.lstat()
    except FileNotFoundError:
        return {'kind': 'absent'}
    if stat.S_ISLNK(info.st_mode):
        return {'kind': 'symlink', 'target': os.readlink(path)}
    if not stat.S_ISREG(info.st_mode):
        return {'kind': 'nonregular', 'mode': stat.S_IMODE(info.st_mode)}
    if info.st_size > MAX_FILE_BYTES:
        raise AuditError('Snapshot file exceeds its finite allowance: ' + str(path))
    data = bounded_file(path)
    return {'kind': 'file', 'mode': stat.S_IMODE(info.st_mode), 'size': len(data), 'sha256': hashlib.sha256(data).hexdigest()}

class Inventory:

    def __init__(self, repo):
        self.repo = Path(repo).expanduser().resolve()
        self.started = time.monotonic()
        self.commands = []
        self.limitations = []
        self.bytes_read = 0
        self.entries = self.files = self.runs = 0
        self.env = {k: v for k, v in os.environ.items() if not k.startswith('GIT_')}
        self.env.update(GIT_OPTIONAL_LOCKS='0', GIT_NO_REPLACE_OBJECTS='1', GIT_NO_LAZY_FETCH='1', GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_SYSTEM='/dev/null', GIT_CONFIG_GLOBAL='/dev/null', GIT_ATTR_NOSYSTEM='1', LC_ALL='C', GIT_TERMINAL_PROMPT='0')
        self.prefix = ['git', '--no-pager', '--no-replace-objects', '--no-optional-locks', '-c', 'core.fsmonitor=false', '-c', 'core.untrackedCache=false', '-c', 'core.preloadIndex=false', '-c', 'core.hooksPath=/dev/null', '-c', 'diff.external=', '-c', 'protocol.allow=never', '-c', 'credential.helper=']

    def note(self, text):
        if text not in self.limitations:
            self.limitations.append(text)

    def git(self, *args, allowed=(0,), cwd=None):
        if not args or (args[0] not in {'rev-parse', 'symbolic-ref', 'for-each-ref', 'status', 'config', 'ls-files', 'merge-base', 'rev-list', 'diff-tree', 'ls-tree', 'cat-file', 'log', 'diff'} and args != ('worktree', 'list', '--porcelain', '-z')):
            raise AuditError('Git verb is outside the read-only whitelist.')
        remaining = TOTAL_SECONDS - (time.monotonic() - self.started)
        if remaining <= 0:
            raise AuditError('Total metadata audit deadline reached.')
        deadline = time.monotonic() + min(COMMAND_SECONDS, remaining)
        command = self.prefix + list(args)
        child = subprocess.Popen(command, cwd=cwd or self.repo, env=self.env, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True, preexec_fn=child_limits)
        selector = selectors.DefaultSelector()
        selector.register(child.stdout, selectors.EVENT_READ, 'stdout')
        selector.register(child.stderr, selectors.EVENT_READ, 'stderr')
        data = {'stdout': bytearray(), 'stderr': bytearray()}
        failure = None
        try:
            while selector.get_map():
                if time.monotonic() >= deadline:
                    failure = 'wall-time limit'
                    break
                events = selector.select(min(0.1, max(0, deadline - time.monotonic())))
                for key, _ in events:
                    chunk = os.read(key.fileobj.fileno(), 65536)
                    if not chunk:
                        selector.unregister(key.fileobj)
                        continue
                    data[key.data].extend(chunk)
                    if sum(map(len, data.values())) > MAX_OUTPUT:
                        failure = 'output limit'
                        break
                if failure:
                    break
            if failure:
                os.killpg(child.pid, signal.SIGKILL)
            try:
                code = child.wait(timeout=1)
            except subprocess.TimeoutExpired:
                failure = failure or 'exit-wait limit'
                os.killpg(child.pid, signal.SIGKILL)
                code = child.wait(timeout=1)
        finally:
            if child.poll() is None:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait(timeout=1)
            selector.close()
            child.stdout.close()
            child.stderr.close()
        row = {'verb': args[0], 'arguments': list(args[1:]), 'exit_code': code, 'stdout_bytes': len(data['stdout']), 'stderr_bytes': len(data['stderr']), 'cutoff': failure}
        self.commands.append(row)
        if failure or code not in allowed:
            row['error_excerpt'] = bytes(data['stderr'][:2048]).decode(errors='replace')
            raise AuditError('Read-only Git query failed: ' + args[0] + ': ' + str(failure or code))
        return (code, bytes(data['stdout']))

    def entries_at(self, directory):
        result = []
        with os.scandir(directory) as entries:
            for entry in entries:
                self.entries += 1
                if self.entries > MAX_ENTRIES:
                    raise AuditError('Metadata directory-entry cap reached.')
                result.append(entry)
        return sorted(result, key=lambda e: e.name)

    def worktree_snapshot(self):
        _, raw = self.git('worktree', 'list', '--porcelain', '-z')
        result, row = ([], {})
        for field in raw.split(b'\x00'):
            if not field:
                if row:
                    result.append(row)
                    row = {}
                continue
            key, _, value = os.fsdecode(field).partition(' ')
            row[key] = value or True
        if row:
            result.append(row)
        if len(result) > MAX_BRANCHES + 1:
            raise AuditError('Linked-worktree inventory exceeds its finite allowance.')
        for item in result:
            if item.get('bare'):
                continue
            path = Path(item['worktree'])
            if not path.is_dir() or path.is_symlink():
                raise AuditError('Linked worktree is absent or indirect: ' + str(path))
            code, keys = self.git('config', '--name-only', '--get-regexp', '^filter\\.', allowed=(0, 1), cwd=path)
            if code == 0 and keys.strip():
                raise AuditError('Configured Git filters require separate review: ' + str(path))
            _, status_raw = self.git('status', '--porcelain=v1', '-z', '--untracked-files=normal', cwd=path)
            _, index = self.git('rev-parse', '--path-format=absolute', '--git-path', 'index', cwd=path)
            item['status_records'] = [os.fsdecode(x) for x in status_raw.split(b'\x00') if x]
            item['index_fingerprint'] = fingerprint(Path(os.fsdecode(index).strip()))
            item['user_files'] = {name: fingerprint(path / name) for name in ('Makefile', 'return-buffer')}
            _, dirty_raw = self.git('ls-files', '--modified', '--others', '--exclude-standard', '-z', cwd=path)
            _, staged_raw = self.git('diff', '--cached', '--name-only', '-z', '--no-ext-diff', '--no-textconv', cwd=path)
            dirty_names = sorted({os.fsdecode(x) for x in (dirty_raw + staged_raw).split(b'\x00') if x})
            if len(dirty_names) > MAX_PATHS:
                raise AuditError('Dirty worktree file inventory exceeds its finite allowance: ' + str(path))
            item['dirty_file_fingerprints'] = {}
            for name in dirty_names:
                target = path / name
                if Path(name).is_absolute() or '..' in Path(name).parts:
                    raise AuditError('Unexpected dirty worktree path: ' + name)
                for parent in target.parents:
                    if parent == path:
                        break
                    if parent.is_symlink():
                        raise AuditError('Indirect dirty-file ancestor: ' + str(target))
                if target.exists() and (not target.is_symlink()):
                    self.bytes_read += target.stat().st_size
                    if self.bytes_read > MAX_TOTAL_READ:
                        raise AuditError('Aggregate snapshot and report byte allowance reached.')
                item['dirty_file_fingerprints'][name] = fingerprint(target)
        return result

    def snapshot(self):
        _, refs = self.git('for-each-ref', '--format=%(refname)%00%(objectname)', 'refs/heads', 'refs/tags', 'refs/replace')
        _, head = self.git('rev-parse', '--verify', 'HEAD')
        code, symbolic = self.git('symbolic-ref', '--quiet', 'HEAD', allowed=(0, 1))
        _, status_data = self.git('status', '--porcelain=v1', '-z', '--untracked-files=normal')
        _, index_path = self.git('rev-parse', '--path-format=absolute', '--git-path', 'index')
        return {'head': head.decode().strip(), 'symbolic_head': symbolic.decode().strip() if code == 0 else None, 'refs': refs.decode(errors='replace').splitlines(), 'status_records': [x.decode(errors='replace') for x in status_data.split(b'\x00') if x], 'index': fingerprint(Path(os.fsdecode(index_path).strip())), 'Makefile': fingerprint(self.repo / 'Makefile'), 'return-buffer': fingerprint(self.repo / 'return-buffer'), 'worktrees': self.worktree_snapshot()}


def encoded(value):
    return (json.dumps(value, indent=2, sort_keys=True) + '\n').encode()

def digest(value):
    return hashlib.sha256(value).hexdigest()

def open_dir(path):
    path = Path(path)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Expected an absolute ordinary directory.')
    fd = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd)
            fd = child
        return fd
    except BaseException:
        os.close(fd)
        raise

def open_file(path):
    path = Path(path)
    parent = open_dir(path.parent)
    try:
        fd = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=parent)
    finally:
        os.close(parent)
    if not stat.S_ISREG(os.fstat(fd).st_mode):
        os.close(fd)
        raise RuntimeError('Expected an ordinary file: ' + str(path))
    return fd

def identity(info):
    return tuple((getattr(info, key) for key in ('st_dev', 'st_ino', 'st_mode', 'st_size', 'st_mtime_ns', 'st_ctime_ns')))

def read_file(path, limit=MAX_FILE, content=True):
    fd = open_file(path)
    try:
        before = os.fstat(fd)
        if before.st_size > limit:
            raise RuntimeError('File exceeds its finite allowance: ' + str(path))
        total, hasher, pieces = (0, hashlib.sha256(), [])
        until = time.monotonic() + 30
        while True:
            if time.monotonic() > until:
                raise RuntimeError('File read deadline reached.')
            chunk = os.read(fd, min(MIB, limit + 1 - total))
            if not chunk:
                break
            total += len(chunk)
            if total > limit:
                raise RuntimeError('File grew beyond its allowance.')
            hasher.update(chunk)
            if content:
                pieces.append(chunk)
        after = os.fstat(fd)
        current = os.stat(path, follow_symlinks=False)
        if total != before.st_size or identity(before) != identity(after) or identity(before) != identity(current):
            raise RuntimeError('File changed during capture: ' + str(path))
        fp = {'kind': 'file', 'mode': stat.S_IMODE(before.st_mode), 'size': total, 'sha256': hasher.hexdigest()}
        return (fp, b''.join(pieces) if content else None)
    finally:
        os.close(fd)

def exclusive(path, data, mode=384):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, mode)
    with os.fdopen(fd, 'wb') as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())

def atomic_json(path, value):
    raw = encoded(value)
    if len(raw) > MAX_REPORT:
        raise RuntimeError('Evidence report exceeds its finite allowance.')
    path = Path(path)
    temporary = path.with_name(path.name + '.tmp-' + uuid.uuid4().hex)
    exclusive(temporary, raw)
    os.replace(temporary, path)

def repository(inv):
    _, raw = inv.git('rev-parse', '--path-format=absolute', '--git-common-dir')
    common = Path(os.fsdecode(raw).strip())
    if common != inv.repo / '.git':
        raise RuntimeError('--repo must name the original ordinary checkout.')
    inv.common = common
    return common

def ready(inv, common):
    _, head = inv.git('rev-parse', '--verify', 'HEAD')
    _, branch = inv.git('symbolic-ref', '--quiet', 'HEAD')
    _, held = inv.git('rev-parse', '--verify', BORROW_REF + '^{commit}')
    if (head.decode().strip(), branch.decode().strip(), held.decode().strip()) != (MASTER, 'refs/heads/master', BORROW):
        raise RuntimeError('Promoted master or held borrow checkpoint differs; no new test was attempted.')
    borrow = inv.repo.parent / (inv.repo.name + '-sparsemerge-borrow-repair')
    fd = open_dir(borrow)
    os.close(fd)
    _, actual = inv.git('rev-parse', '--verify', 'HEAD', cwd=borrow)
    _, linked_common = inv.git('rev-parse', '--path-format=absolute', '--git-common-dir', cwd=borrow)
    code, filters = inv.git('config', '--name-only', '--get-regexp', '^filter\\.', cwd=borrow, allowed=(0, 1))
    if code == 0 and filters.strip():
        raise RuntimeError('Configured Git filters require separate review.')
    _, status_raw = inv.git('status', '--porcelain=v1', '-z', '--untracked-files=all', cwd=borrow)
    if actual.decode().strip() != BORROW or Path(os.fsdecode(linked_common).strip()) != common or status_raw:
        raise RuntimeError('Held borrow checkout is absent, dirty or at a different checkpoint.')
    return borrow

def protected_snapshot(audit, repo):
    inv = audit.Inventory(repo)
    common = repository(inv)
    code, filters = inv.git('config', '--name-only', '--get-regexp', '^filter\\.', allowed=(0, 1))
    if code == 0 and filters.strip():
        raise RuntimeError('Configured Git filters require review before status inspection.')
    snapshot = inv.snapshot()
    files = {str(common / name): audit.fingerprint(common / name) for name in ('config', 'ORIG_HEAD', 'info/exclude', 'info/grafts')}
    root = common / 'codex-repairs'
    for item in inv.entries_at(root):
        if item.is_symlink() or not item.is_dir(follow_symlinks=False):
            raise RuntimeError('Existing workflow metadata is indirect.')
        if item.name == WORKFLOW:
            continue
        for name in ('state.json', 'worktree.json'):
            path = Path(item.path) / name
            files[str(path)] = audit.fingerprint(path)
    return {'snapshot': snapshot, 'journals_and_config': files}

def acquire_locks(common):
    held = []
    try:
        fd = open_dir(common / 'codex-repairs')
        try:
            with os.scandir(fd) as scan:
                entries = list(scan)
            if len(entries) > 128:
                raise RuntimeError('Workflow lock inventory exceeds its allowance.')
            if any((e.is_symlink() or not e.is_dir(follow_symlinks=False) for e in entries)):
                raise RuntimeError('Workflow lock directory is indirect.')
        finally:
            os.close(fd)
        for entry in sorted(entries, key=lambda e: e.name):
            if entry.name == WORKFLOW:
                continue
            path = common / 'codex-repairs' / entry.name / 'run.lock'
            parent = open_dir(path.parent)
            try:
                try:
                    lock = os.open(path.name, os.O_RDWR | os.O_NOFOLLOW, dir_fd=parent)
                except FileNotFoundError:
                    continue
            finally:
                os.close(parent)
            held.append((lock, path))
            if not stat.S_ISREG(os.fstat(lock).st_mode):
                raise RuntimeError('Workflow lock is not ordinary.')
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        if not any((p == common / 'codex-repairs/leveldb-sparsemerge-borrow002/run.lock' for _, p in held)):
            raise RuntimeError('The retained borrow workflow lock is absent.')
        return held
    except BaseException:
        for lock, _ in held:
            os.close(lock)
        raise

def require_guard(result):
    if result.get('outcome') != 'exited' or result.get('verified') is not True or result.get('exit_code') != 0:
        raise RuntimeError('Protected collection phase did not complete: ' + str(result.get('label')))


"""Finite Cargo machine-output observation and exact single-case assessment.

The parent must launch this observer under the reviewed resource guard. This
module never builds unless cargo_observer is explicitly invoked with its fixed
offline no-run command shape. Cargo stderr remains attached to that guard.
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

MAX_MACHINE_BYTES = 64 * 1024 * 1024
MAX_RECORD_BYTES = 1024 * 1024
MAX_RECORDS = 50000
MAX_EVIDENCE_BYTES = 128 * 1024
MAX_RENDERED_BYTES = 16 * 1024
MAX_RENDERED_TOTAL = 64 * 1024
OBSERVER_SECONDS = 580
ALLOWED_PACKAGES = (
    "bitcoinleveldb-options", "bitcoinleveldb-tablecache", "bitcoinleveldb-dbimpl",
    "bitcoinleveldb-versionset", "bitcoinleveldbt-sparsemerge",
)
ALLOWED_TARGETS = tuple(name.replace("-", "_") for name in ALLOWED_PACKAGES)


def _observer_spec(spec):
    if not isinstance(spec, dict):
        raise RuntimeError("Cargo observer specification is not an object.")
    expected = spec.get("expected_targets")
    if (not isinstance(expected, list) or len(expected) != len(ALLOWED_TARGETS)
            or any(not isinstance(name, str) for name in expected)
            or set(expected) != set(ALLOWED_TARGETS)):
        raise RuntimeError("Cargo observer requires exactly the five reviewed library targets.")
    command = spec.get("command")
    if (not isinstance(command, list) or len(command) < 2
            or any(not isinstance(arg, str) or not arg or "\x00" in arg for arg in command)
            or Path(command[0]).name != "cargo" or command[1] != "test"):
        raise RuntimeError("Cargo observer command does not name cargo test.")
    flags, packages, jobs = set(), [], None
    index = 2
    allowed_flags = {"--offline", "--locked", "--lib", "--no-run", "--message-format=json"}
    while index < len(command):
        arg = command[index]
        if arg in allowed_flags:
            if arg in flags:
                raise RuntimeError("Cargo observer command repeats a required flag.")
            flags.add(arg)
            index += 1
        elif arg == "--jobs":
            if jobs is not None or index + 1 >= len(command):
                raise RuntimeError("Cargo observer jobs argument is malformed.")
            jobs = command[index + 1]
            index += 2
        elif arg in {"-p", "--package"}:
            if index + 1 >= len(command):
                raise RuntimeError("Cargo observer package argument is absent.")
            packages.append(command[index + 1])
            index += 2
        else:
            raise RuntimeError("Cargo observer command contains an unreviewed argument.")
    if (flags != allowed_flags or jobs != "1" or len(packages) != len(ALLOWED_PACKAGES)
            or set(packages) != set(ALLOWED_PACKAGES)):
        raise RuntimeError("Cargo observer requires offline locked single-job no-run builds of the five reviewed libraries.")
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
    return list(command), list(expected), paths


def _atomic_evidence(path, record):
    raw = json.dumps(record, sort_keys=True, indent=2).encode("utf-8") + b"\n"
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
    # Cargo may report a build-script compiler artifact with an executable.
    # It is not a libtest artifact and must never be selected or executed here.
    if target.get("kind") == ["custom-build"]:
        evidence["build_script_artifacts_discarded"] += 1
        return
    executable = record.get("executable")
    if executable is None:
        evidence["dependency_artifacts_discarded"] += 1
        return
    name = target.get("name")
    if name not in expected:
        raise RuntimeError("Cargo emitted an executable for an unexpected target.")
    if profile.get("test") is not True or "lib" not in target.get("kind", []):
        raise RuntimeError("Cargo emitted an executable outside the expected library-test profile.")
    if name in evidence["artifacts"]:
        raise RuntimeError("Cargo emitted a duplicate expected test executable.")
    if not isinstance(executable, str) or not executable or len(executable) > 4096:
        raise RuntimeError("Cargo executable path is malformed or too long.")
    executable_path = Path(executable)
    if (not executable_path.is_absolute() or ".." in executable_path.parts
            or executable_path.parent != paths["target_directory"] / "debug" / "deps"
            or not re.fullmatch(re.escape(name) + r"-[0-9a-f]+", executable_path.name)):
        raise RuntimeError("Cargo executable does not belong to the isolated debug test directory.")
    package = record.get("package_id")
    if not isinstance(package, str) or not package or len(package) > 4096:
        raise RuntimeError("Cargo package identity is absent or too long.")
    evidence["artifacts"][name] = {
        "target_name": name, "executable": executable,
        "package_id": package, "fresh": record.get("fresh") is True,
    }


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
        message = record.get("message", {})
        if not isinstance(message, dict):
            raise RuntimeError("Cargo compiler message is not an object.")
        evidence["diagnostic_records"] += 1
        rendered = message.get("rendered")
        if rendered is not None and not isinstance(rendered, str):
            raise RuntimeError("Cargo rendered diagnostic is not text.")
        if rendered:
            allowance = min(MAX_RENDERED_BYTES, MAX_RENDERED_TOTAL - evidence["rendered_bytes"])
            rendered_raw = rendered.encode("utf-8")
            snippet = rendered_raw[:max(allowance, 0)]
            if snippet:
                sys.stderr.write(snippet.decode("utf-8", errors="replace"))
                sys.stderr.flush()
                evidence["rendered_bytes"] += len(snippet)
            evidence["rendered_bytes_omitted"] += len(rendered_raw) - len(snippet)
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

    Return 0 only when Cargo succeeded and produced every expected executable.
    On malformed output or an output allowance violation, stop Cargo promptly.
    """
    evidence = {
        "schema": "bitcoinrs-source011-cargo-observer-v1", "command": None,
        "cwd": None, "target_directory": None, "expected_targets": [],
        "cargo_exit_code": None, "observer_exit_code": 2, "error": None,
        "artifacts": {}, "machine_bytes": 0, "records": 0,
        "dependency_artifacts_discarded": 0, "build_script_records_discarded": 0,
        "build_script_artifacts_discarded": 0,
        "diagnostic_records": 0, "other_records_discarded": 0,
        "rendered_bytes": 0, "rendered_bytes_omitted": 0,
        "build_finished": None,
    }
    child, paths = None, None
    try:
        command, expected, paths = _observer_spec(spec)
        evidence.update(command=command, cwd=str(paths["cwd"]),
                        target_directory=str(paths["target_directory"]), expected_targets=expected)
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
        if evidence["cargo_exit_code"] != 0:
            raise RuntimeError("Cargo exited unsuccessfully.")
        if evidence["build_finished"] is not True:
            raise RuntimeError("Cargo did not report a successful completed build.")
        if set(evidence["artifacts"]) != set(expected):
            raise RuntimeError("Cargo did not emit all five expected library-test executables.")
        evidence["observer_exit_code"] = 0
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
        sys.stderr.write("SOURCE011_CARGO_OBSERVER_REJECTED " + (evidence["error"] or "invalid specification") + "\n")
        return 2
    try:
        _atomic_evidence(paths["evidence_path"], evidence)
    except Exception as error:
        sys.stderr.write("SOURCE011_CARGO_EVIDENCE_ERROR " + str(error)[:1024] + "\n")
        return 3
    print("SOURCE011_CARGO_OBSERVER records=" + str(evidence["records"])
          + " executables=" + str(len(evidence["artifacts"]))
          + " native_exit=" + str(evidence["cargo_exit_code"])
          + " observer_exit=" + str(evidence["observer_exit_code"]), flush=True)
    return evidence["observer_exit_code"]


def assess_case(result, log, expected_name, marker=None):
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


def load_guard():
    if digest(GUARD_SOURCE) != GUARD_SHA:
        raise RuntimeError("Reviewed resource guard differs.")
    module = types.ModuleType("ownership011_resource_guard")
    module.__file__ = __file__
    exec(compile(GUARD_SOURCE, __file__, "exec"), module.__dict__)
    module.BUDGETS["source"] = dict(memory=512*MIB, high=384*MIB, tasks=32,
                                  seconds=180, file=32*MIB, output=MIB)
    module.BUDGETS["stress"] = dict(memory=2048*MIB, high=1536*MIB, tasks=64,
                                  seconds=600, file=256*MIB, output=MIB,
                                  data=512*MIB, entries=4096)
    return module


def checked_environment():
    for name in ("LD_PRELOAD", "LD_AUDIT", "GIT_CONFIG_COUNT", "GIT_CONFIG_PARAMETERS",
                 "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS"):
        if os.environ.get(name):
            raise RuntimeError("Execution override requires review: " + name)


def normalized_original(value, worktree):
    value = json.loads(json.dumps(value))
    value["snapshot"]["refs"] = [row for row in value["snapshot"]["refs"]
        if row.split("\x00", 1)[0] != "refs/heads/" + BRANCH]
    value["snapshot"]["worktrees"] = [row for row in value["snapshot"]["worktrees"]
        if row.get("worktree") != str(worktree)]
    return value


def source_receipt(common, borrow, full=False):
    path = common / SOURCE_RECEIPT_REL
    fp, raw = read_file(path, 16*MIB)
    value = json.loads(raw)
    canonical = digest(json.dumps(value, sort_keys=True, separators=(",", ":")).encode())
    if canonical != SOURCE_RECEIPT_SHA or value.get("original_preserved") is not True:
        raise RuntimeError("Reviewed source010 evidence differs.")
    sources = value["capture"]["sources"]
    if sources.get("source_omissions") or sources.get("unresolved_scoped_edges"):
        raise RuntimeError("Reviewed scoped source capture is incomplete.")
    selected = sources["files"] if full else {p: sources["files"][p] for p in PATCHES}
    for name, item in selected.items():
        observed, _ = read_file(borrow / name, MIB, content=False)
        if observed != item["fingerprint"]:
            raise RuntimeError("Held captured source changed: " + name)
    artifact, _ = read_file(common / ARTIFACT_REL, 128*MIB, content=False)
    if artifact != ARTIFACT:
        raise RuntimeError("Retained ordinary executable changed.")
    return {"path": str(path), "fingerprint": fp, "canonical_sha256": canonical,
            "source_count_verified": len(selected), "retained_artifact_reverified": True}


def input_worker(repo, output, full):
    checked_environment()
    inv = Inventory(repo)
    common = repository(inv)
    borrow = ready(inv, common)
    value = source_receipt(common, borrow, full)
    atomic_json(output, dict(value, outcome="verified"))
    print("OWNERSHIP_INPUTS_VERIFIED", flush=True)
    return 0


def copy_cache_worker(source, target, output):
    """Seed a private cache using independent regular-file copies, never links."""
    for path in (source, target.parent):
        fd = open_dir(path)
        os.close(fd)
    target.mkdir(mode=0o700, exist_ok=True)
    fd = open_dir(target)
    os.close(fd)
    selected, total, entries = [], 0, 0
    roots = [source / "debug" / name for name in ("deps", "build", ".fingerprint")]
    for root in roots:
        if not root.exists():
            raise RuntimeError("Retained cache seed directory is absent: " + str(root))
        fd = open_dir(root)
        os.close(fd)
        for current, dirs, files in os.walk(root, followlinks=False):
            dirs.sort()
            files.sort()
            entries += len(dirs) + len(files)
            if entries > 30000:
                raise RuntimeError("Cache seed entry limit reached.")
            for name in dirs:
                if (Path(current) / name).is_symlink():
                    raise RuntimeError("Cache seed contains an indirect directory.")
            for name in files:
                src = Path(current) / name
                info = src.lstat()
                if not stat.S_ISREG(info.st_mode) or info.st_size > 512*MIB:
                    raise RuntimeError("Cache seed contains an indirect or oversized file: " + str(src))
                total += info.st_size
                if total > 4*1024*MIB:
                    raise RuntimeError("Cache seed exceeds 4 GiB; no unrestricted copy was attempted.")
                selected.append((src, src.relative_to(source), info.st_size))
    if shutil.disk_usage(target).free < total + 2*1024*MIB:
        raise RuntimeError("Private cache requires its copy size plus 2 GiB free space.")
    copied = reused = 0
    for src, relative, size in selected:
        dst = target / relative
        dst.parent.mkdir(parents=True, exist_ok=True)
        fd = open_dir(dst.parent)
        os.close(fd)
        expected, _ = read_file(src, 512*MIB, content=False)
        if dst.exists() or dst.is_symlink():
            actual, _ = read_file(dst, 512*MIB, content=False)
            if actual != expected or os.stat(src).st_ino == os.stat(dst).st_ino:
                raise RuntimeError("Existing private cache seed differs or aliases retained bytes.")
            reused += 1
            continue
        # The new destination is exclusive. The source is opened read-only.
        temporary = dst.with_name(dst.name + ".copy-" + uuid.uuid4().hex)
        src_fd = open_file(src)
        dst_fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, expected["mode"])
        try:
            with os.fdopen(dst_fd, "wb") as writer:
                while True:
                    chunk = os.read(src_fd, MIB)
                    if not chunk:
                        break
                    writer.write(chunk)
                writer.flush()
                os.fsync(writer.fileno())
        finally:
            os.close(src_fd)
        observed, _ = read_file(temporary, 512*MIB, content=False)
        if observed != expected:
            raise RuntimeError("Cache seed changed during its independent copy.")
        os.replace(temporary, dst)
        copied += 1
        if shutil.disk_usage(target).free < 2*1024*MIB:
            raise RuntimeError("Private cache free-space floor reached; partial copy retained.")
    atomic_json(output, {"outcome": "copied", "source": str(source), "target": str(target),
        "bytes": total, "files_copied": copied, "files_reused": reused,
        "hard_links_created": False, "incremental_copied": False})
    print("PRIVATE_CACHE_SEEDED", flush=True)
    return 0


def candidate_check(repo, worktree, files, expected_head, allow_base_files=False):
    fd = open_dir(worktree)
    os.close(fd)
    inv = Inventory(repo)
    _, head = inv.git("rev-parse", "--verify", "HEAD", cwd=worktree)
    _, branch = inv.git("symbolic-ref", "--quiet", "HEAD", cwd=worktree)
    if head.decode().strip() != expected_head or branch.decode().strip() != "refs/heads/" + BRANCH:
        raise RuntimeError("Candidate branch or checkpoint changed.")
    _, common = inv.git("rev-parse", "--path-format=absolute", "--git-common-dir", cwd=worktree)
    if Path(os.fsdecode(common).strip()) != repo / ".git":
        raise RuntimeError("Candidate belongs to another repository.")
    _, dirty = inv.git("ls-files", "--modified", "--others", "--exclude-standard", "-z", cwd=worktree)
    _, staged = inv.git("diff", "--cached", "--name-only", "--no-ext-diff", "--no-textconv", "-z", cwd=worktree)
    names = {os.fsdecode(x) for x in (dirty + staged).split(b"\0") if x}
    if names - set(files):
        raise RuntimeError("Candidate contains unowned modifications: " + repr(sorted(names - set(files))))
    for name, data in files.items():
        path = worktree / name
        if data is None:
            if path.exists() or path.is_symlink():
                raise RuntimeError("Unexpected candidate receipt before validation.")
            continue
        fp, current = read_file(path, 2*MIB)
        if current != data:
            if not (allow_base_files and name in PATCHES and fp["sha256"] == PATCHES[name]["before_sha256"]):
                raise RuntimeError("Candidate file differs: " + name)
        expected_mode = 0o755 if name == REPLAY_PATH else 0o644
        if fp["mode"] != expected_mode:
            raise RuntimeError("Candidate file mode differs: " + name)
    return inv


def candidate_location(repo, worktree, files):
    fd = open_dir(worktree)
    os.close(fd)
    inv = Inventory(repo)
    _, top = inv.git("rev-parse", "--show-toplevel", cwd=worktree)
    _, head = inv.git("rev-parse", "--verify", "HEAD", cwd=worktree)
    _, branch = inv.git("symbolic-ref", "--quiet", "HEAD", cwd=worktree)
    _, common = inv.git("rev-parse", "--path-format=absolute", "--git-common-dir", cwd=worktree)
    if (Path(os.fsdecode(top).strip()) != worktree
            or Path(os.fsdecode(common).strip()) != repo / ".git"
            or head.decode().strip() != BORROW
            or branch.decode().strip() != "refs/heads/" + BRANCH):
        raise RuntimeError("Candidate location/branch/base differs before source installation.")
    _, dirty = inv.git("ls-files", "--modified", "--others", "--exclude-standard", "-z", cwd=worktree)
    _, staged = inv.git("diff", "--cached", "--name-only", "--no-ext-diff", "--no-textconv", "-z", cwd=worktree)
    names = {os.fsdecode(x) for x in (dirty + staged).split(b"\0") if x}
    if names - set(files):
        raise RuntimeError("Candidate has unowned changes before source installation.")
    for name, data in files.items():
        path = worktree / name
        if not path.exists() and not path.is_symlink():
            if name in PATCHES:
                raise RuntimeError("Required candidate source is absent: " + name)
            continue
        fp, raw = read_file(path, 2*MIB)
        allowed = {digest(data)} if data is not None else set()
        if name in PATCHES:
            allowed.add(PATCHES[name]["before_sha256"])
        if fp["sha256"] not in allowed:
            raise RuntimeError("Candidate contains unowned existing bytes: " + name)


def write_candidate_file(path, data, mode):
    path.parent.mkdir(parents=True, exist_ok=True)
    fd = open_dir(path.parent)
    os.close(fd)
    if path.is_symlink():
        raise RuntimeError("Candidate output path is indirect.")
    temporary = path.with_name(path.name + ".new-" + uuid.uuid4().hex)
    exclusive(temporary, data, mode)
    os.replace(temporary, path)


def run(repo):
    report = {"workflow": WORKFLOW, "outcome": "failed", "phase": "preflight",
        "base": BORROW, "expected_master": MASTER, "branch": BRANCH,
        "repair_qualified": False, "cause_established": False, "master_updated": False,
        "existing_refs_updated": False, "builds_started": False, "native_tests_started": False,
        "debuggers_started": False, "full_coverage_claimed": False, "readouts": {}, "cases": [],
        "scope": "Isolated candidate for audited internal Env ownership paths; public Rc Env API is not made universally thread-safe.",
        "merge": "not performed", "tag": "not created"}
    held, cache_locks = [], []
    directory = initial = guard = state_path = state = None
    try:
        checked_environment()
        repo = Path(os.path.abspath(Path(repo).expanduser()))
        fd = open_dir(repo)
        os.close(fd)
        inv = Inventory(repo)
        common = repository(inv)
        held = acquire_locks(common)
        borrow = ready(inv, common)
        worktree = repo.parent / (repo.name + WORKTREE_SUFFIX)
        initial = normalized_original(protected_snapshot(sys.modules[__name__], repo), worktree)
        own = common / "codex-repairs" / WORKFLOW
        own.mkdir(mode=0o700, exist_ok=True)
        fd = open_dir(own)
        os.close(fd)
        own_lock = os.open(own / "run.lock", os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600)
        held.append((own_lock, own / "run.lock"))
        if not stat.S_ISREG(os.fstat(own_lock).st_mode):
            raise RuntimeError("Workflow lock is not ordinary.")
        fcntl.flock(own_lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        runs = own / "runs"
        runs.mkdir(mode=0o700, exist_ok=True)
        fd = open_dir(runs)
        os.close(fd)
        directory = runs / (dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])
        directory.mkdir(mode=0o700)
        script_data = read_file(Path(__file__).absolute(), 2*MIB)[1]
        script = directory / "ownership-controller011.py"
        exclusive(script, script_data, 0o700)
        files = {p: x["after"].encode() for p, x in PATCHES.items()}
        files.update({REPLAY_PATH: script_data, DOCUMENT_PATH: DOCUMENT.encode(), RECEIPT_PATH: None})
        identity = {"workflow": WORKFLOW, "base": BORROW, "master": MASTER,
            "branch": BRANCH, "worktree": str(worktree),
            "file_hashes": {p: digest(x) for p, x in files.items() if x is not None}}
        state_path = own / "state.json"
        if state_path.exists() or state_path.is_symlink():
            state = json.loads(read_file(state_path, 16*MIB)[1])
            if state.get("identity") != identity:
                raise RuntimeError("Existing candidate journal has a different source plan; retained.")
        else:
            _, ref = inv.git("rev-parse", "--verify", "refs/heads/" + BRANCH, allowed=(0, 128))
            if worktree.exists() or worktree.is_symlink() or ref.strip():
                raise RuntimeError("Existing candidate branch/path has no matching journal; retained.")
            state = {"identity": identity, "phase": "planned", "runs": []}
            atomic_json(state_path, state)
        if state.get("native_failure_observed") or state.get("native_pending"):
            raise RuntimeError("This candidate already has a retained native failure; a new reviewed revision is required.")
        if state.get("phase") == "validated" and not state.get("checkpoint"):
            raise RuntimeError("Candidate finalization was interrupted; validated files and any commit are retained for review.")
        state["runs"].append(str(directory))
        atomic_json(state_path, state)
        guard_module = load_guard()
        guard = guard_module.Guard(directory / "guard", script, inv.env)
        target = own / "cargo-target"
        report.update(repository=str(repo), worktree=str(worktree), target_directory=str(target),
            return_file=str(directory / "OWNERSHIP_REPAIR_RETURN.json"), controller_sha256=digest(script_data),
            preservation_before_sha256=digest(encoded(initial)), guard_sha256=GUARD_SHA,
            base_guard_sha256=BASE_GUARD_SHA,
            guard_delta="The existing data/entry monitor also covers the named stress profile.",
            locks_held=[str(p) for _, p in held], source_plan=identity["file_hashes"])
        def original_check():
            ready(Inventory(repo), common)
            current = normalized_original(protected_snapshot(sys.modules[__name__], repo), worktree)
            if current != initial:
                raise RuntimeError("Protected original state changed.")
        def phase(label, command, kind="build", cwd=None, env=None, data=None):
            original_check()
            report["phase"] = label
            print("Protected ownership repair phase: " + label, flush=True)
            row = guard.run(label, command, cwd or worktree, env or {}, kind=kind, tmpdir=data)
            report["readouts"][label] = row
            require_guard(row)
            return row
        phase("preflight", [sys.executable, "-I", "-S", "-c", "print('GUARD_PREFLIGHT_OK')"], "probe", repo)
        phase("inputs-before", [sys.executable, "-I", "-S", str(script), "--inputs", str(repo),
            str(directory / "inputs-before.json"), "full"], "source", repo)
        report["inputs_before"] = json.loads(read_file(directory / "inputs-before.json", 2*MIB)[1])
        if state.get("checkpoint"):
            if state.get("last_outcome") not in {"qualified_candidate", "already_qualified"}:
                raise RuntimeError("Retained checkpoint lacks completed final preservation verification; review is required.")
            receipt = state["receipt"]
            files[RECEIPT_PATH] = encoded(receipt)
            candidate_check(repo, worktree, files, state["checkpoint"])
            for item in receipt["artifacts"].values():
                observed, _ = read_file(Path(item["path"]), 256*MIB, content=False)
                if observed != item["fingerprint"]:
                    raise RuntimeError("Previously qualified private artifact differs.")
            report.update(outcome="already_qualified", phase="complete", checkpoint=state["checkpoint"],
                          repair_qualified=True, validation_reused=True, verification=receipt)
            return report
        inv = Inventory(repo)
        _, current_branch = inv.git("rev-parse", "--verify", "refs/heads/" + BRANCH, allowed=(0, 128))
        if not worktree.exists():
            if current_branch.strip():
                raise RuntimeError("Candidate branch exists without its owned worktree.")
            phase("git-worktree-add", inv.prefix + ["worktree", "add", "-b", BRANCH, str(worktree), BORROW], cwd=repo)
        candidate_location(repo, worktree, files)
        if not state.get("files_installed"):
            # A previously interrupted installation may contain only planned old/new bytes.
            for name in PATCHES:
                fp, current = read_file(worktree / name, MIB)
                if fp["sha256"] not in {PATCHES[name]["before_sha256"], PATCHES[name]["after_sha256"]}:
                    raise RuntimeError("Candidate has an unexpected source before installation: " + name)
            for name, data in files.items():
                if data is not None:
                    path = worktree / name
                    if name not in PATCHES and path.exists() and read_file(path, 2*MIB)[1] != data:
                        raise RuntimeError("New candidate path has unowned bytes: " + name)
                    write_candidate_file(path, data, 0o755 if name == REPLAY_PATH else 0o644)
            state["files_installed"] = True
            atomic_json(state_path, state)
        candidate_check(repo, worktree, files, BORROW)
        if not state.get("cache_seeded"):
            seed = (common / ARTIFACT_REL).parent.parent.parent
            for name in (".cargo-build-lock", ".cargo-lock"):
                path = seed / "debug" / name
                if path.exists():
                    fd = os.open(path, os.O_RDWR | os.O_NOFOLLOW)
                    if not stat.S_ISREG(os.fstat(fd).st_mode):
                        os.close(fd)
                        raise RuntimeError("Retained Cargo lock is indirect.")
                    cache_locks.append((fd, path))
                    fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
            phase("seed-private-cache", [sys.executable, "-I", "-S", str(script), "--copy-cache", str(seed),
                str(target), str(directory / "cache-copy.json")], cwd=repo)
            report["cache_copy"] = json.loads(read_file(directory / "cache-copy.json", MIB)[1])
            for fd, _ in cache_locks:
                os.close(fd)
            cache_locks.clear()
            state["cache_seeded"] = True
            atomic_json(state_path, state)
        observer_spec = directory / "cargo-observer.spec.json"
        evidence_path = directory / "cargo-artifacts.json"
        command = ["cargo", "test", "--offline", "--locked", "--jobs", "1"]
        for package in PACKAGES:
            command.extend(["-p", package])
        command.extend(["--lib", "--no-run", "--message-format=json"])
        exclusive(observer_spec, encoded({"command": command, "cwd": str(worktree),
            "target_directory": str(target), "evidence_path": str(evidence_path),
            "expected_targets": [p.replace("-", "_") for p in PACKAGES]}))
        nix = ["nix-shell", "--option", "max-jobs", "0", "--option", "builders", "",
               "--option", "substitute", "false", str(worktree / "shell.nix"), "--run",
               shlex.join([sys.executable, "-I", "-S", str(script), "--cargo-observer", str(observer_spec)])]
        report["builds_started"] = True
        try:
            phase("compile-targeted-tests", nix, env={"CARGO_TARGET_DIR": str(target), "CARGO_BUILD_JOBS": "1", "CARGO_INCREMENTAL": "0"})
        finally:
            if evidence_path.is_file():
                report["cargo_evidence"] = json.loads(read_file(evidence_path, MIB)[1])
        evidence = report["cargo_evidence"]
        if evidence.get("observer_exit_code") != 0 or evidence.get("cargo_exit_code") != 0:
            raise RuntimeError("Cargo artifact observation did not complete.")
        artifacts = {}
        listings = {}
        for target_name, item in evidence["artifacts"].items():
            executable = Path(item["executable"])
            if not executable.is_absolute() or not executable.is_relative_to(target):
                raise RuntimeError("Test artifact lies outside the private cache.")
            fp, _ = read_file(executable, 256*MIB, content=False)
            artifacts[target_name] = {"path": str(executable), "fingerprint": fp}
            row = phase("list-" + target_name, [str(executable), "--list"], "case", env={"RUST_LOG": "off"})
            text = read_file(Path(row["log"]), MIB)[1].decode(errors="replace")
            listings[target_name] = [line[:-6] for line in text.splitlines() if line.endswith(": test")]
        bound_cases = []
        for target_name, suffix, marker, kind, repetitions in CASES:
            matches = [n for n in listings[target_name] if n == suffix or n.endswith("::" + suffix)]
            if len(matches) != 1:
                raise RuntimeError("Selected test binding is absent or ambiguous: " + suffix)
            for repeat in range(1, repetitions+1):
                bound_cases.append((target_name, matches[0], marker, kind, repeat))
        report["expected_test_count"] = len(bound_cases)
        for target_name, name, marker, kind, repeat in bound_cases:
            candidate_check(repo, worktree, files, BORROW)
            label = name.rsplit("::", 1)[-1] + "-" + str(repeat)
            report["phase"] = label
            print("Protected ownership repair phase: " + label, flush=True)
            state["native_pending"] = {"run": str(directory), "case": name, "label": label}
            atomic_json(state_path, state)
            report["native_tests_started"] = True
            row = guard.run(label, [artifacts[target_name]["path"], "--exact", name, "--nocapture", "--test-threads=1"],
                worktree, {"RUST_LOG": "off", "RUST_BACKTRACE": "1"}, kind=kind,
                tmpdir=directory / ("case-data-" + label))
            report["readouts"][label] = row
            log = read_file(Path(row["log"]), MIB)[1].decode(errors="replace") if row.get("log") else ""
            assessment = assess_case(row, log, name, marker)
            report["cases"].append(dict(assessment, target=target_name, repetition=repeat, log=log))
            if assessment.get("outcome") != "passed":
                state["native_failure_observed"] = {"run": str(directory), "case": name, "assessment": assessment}
                atomic_json(state_path, state)
                raise RuntimeError("Selected native case did not pass; candidate retained and blocked from qualification.")
            state.pop("native_pending", None)
            atomic_json(state_path, state)
            original_check()
        before_commit_inputs = directory / "inputs-before-commit.json"
        phase("inputs-before-commit", [sys.executable, "-I", "-S", str(script), "--inputs", str(repo),
            str(before_commit_inputs), "full"], "source", repo)
        report["inputs_before_commit"] = json.loads(read_file(before_commit_inputs, 2*MIB)[1])
        receipt = {"workflow": WORKFLOW, "base": BORROW, "master_at_validation": MASTER,
            "file_hashes": identity["file_hashes"], "artifacts": artifacts,
            "passed": len(report["cases"]), "expected_test_count": len(bound_cases),
            "cases": [{k:v for k,v in c.items() if k != "log"} for c in report["cases"]],
            "repair_qualified": True, "qualification_scope": report["scope"], "cause_established": False,
            "full_coverage_claimed": False, "debuggers_used": False,
            "retained_source_receipt": report["inputs_before"], "ordinary_sparsemerge_repetitions": 3}
        files[RECEIPT_PATH] = encoded(receipt)
        write_candidate_file(worktree / RECEIPT_PATH, files[RECEIPT_PATH], 0o644)
        candidate_check(repo, worktree, files, BORROW)
        original_check()
        state.update(receipt=receipt, phase="validated")
        atomic_json(state_path, state)
        inv = Inventory(repo)
        phase("git-stage", inv.prefix + ["add", "--", *sorted(files)])
        inv = Inventory(repo)
        _, staged = inv.git("diff", "--cached", "--name-only", "--no-ext-diff", "--no-textconv", "-z", cwd=worktree)
        if {os.fsdecode(x) for x in staged.split(b"\0") if x} != set(files):
            raise RuntimeError("Staged candidate scope differs.")
        message = directory / "commit-message.txt"
        exclusive(message, ("fix: retain stable Env ownership during background table work\n\n"
            "Borrow persistent environment wrappers and keep cached table settings free of Env Rc owners.\n"
            "Preserve public owning APIs, table settings, async compaction and shutdown waits.\n"
            "Selected ownership, manifest and rotation regressions plus three ordinary SparseMerge runs passed under verified resource limits.\n"
            "No full suite, coverage or universal thread-safety claim.\n\nRepair-Workflow: " + WORKFLOW + "\n").encode())
        phase("git-commit", inv.prefix + ["commit", "--file", str(message)])
        inv = Inventory(repo)
        _, head = inv.git("rev-parse", "--verify", "HEAD", cwd=worktree)
        checkpoint = head.decode().strip()
        _, parents = inv.git("rev-list", "--parents", "-n", "1", checkpoint)
        if parents.decode().split() != [checkpoint, BORROW]:
            raise RuntimeError("Candidate commit ancestry differs.")
        _, changed = inv.git("diff-tree", "--no-commit-id", "-r", "--name-only", "--no-ext-diff", "--no-textconv", "-z", BORROW, checkpoint)
        if {os.fsdecode(x) for x in changed.split(b"\0") if x} != set(files):
            raise RuntimeError("Candidate commit exceeded the reviewed file scope.")
        candidate_check(repo, worktree, files, checkpoint)
        state.update(checkpoint=checkpoint, phase="complete")
        atomic_json(state_path, state)
        report.update(outcome="qualified_candidate", phase="complete", checkpoint=checkpoint,
                      repair_qualified=True, validation_reused=False, verification=receipt,
                      test_count=len(report["cases"]))
    except BaseException as error:
        report.update(outcome="failed", repair_qualified=False, error=type(error).__name__ + ": " + str(error))
    finally:
        if directory is not None and guard is not None:
            try:
                print("Protected ownership repair phase: inputs-after", flush=True)
                output = directory / "inputs-after.json"
                row = guard.run("inputs-after", [sys.executable, "-I", "-S", str(script), "--inputs", str(repo), str(output), "full"], repo, {}, kind="source")
                report["readouts"]["inputs-after"] = row
                require_guard(row)
                report["inputs_after"] = json.loads(read_file(output, 2*MIB)[1])
            except BaseException as error:
                report.update(outcome="failed", repair_qualified=False, preservation_error=str(error))
        if initial is not None:
            try:
                after = normalized_original(protected_snapshot(sys.modules[__name__], repo), worktree)
                report["preservation_after_sha256"] = digest(encoded(after))
                report["original_preserved"] = after == initial
                if after != initial:
                    raise RuntimeError("Protected original refs, worktrees or user files changed.")
            except BaseException as error:
                report.update(outcome="failed", repair_qualified=False, original_preserved=False, preservation_error=str(error))
        if state_path is not None and state is not None and directory is not None:
            state["last_outcome"] = report["outcome"]
            atomic_json(state_path, state)
        if directory is not None:
            report["return_file"] = str(directory / "OWNERSHIP_REPAIR_RETURN.json")
            atomic_json(Path(report["return_file"]), report)
        for fd, _ in cache_locks + held:
            os.close(fd)
    return report


def main():
    sys.dont_write_bytecode = True
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    # Service children receive their verified cgroup budget. Do not impose the
    # coordinator's smaller address-space limit on Cargo or native test children.
    if len(sys.argv) == 3 and sys.argv[1] == "--guard-child":
        return load_guard().guard_child(Path(sys.argv[2]))
    if len(sys.argv) == 5 and sys.argv[1] == "--inputs":
        return input_worker(Path(sys.argv[2]), Path(sys.argv[3]), sys.argv[4] == "full")
    if len(sys.argv) == 5 and sys.argv[1] == "--copy-cache":
        return copy_cache_worker(Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]))
    if len(sys.argv) == 3 and sys.argv[1] == "--cargo-observer":
        return cargo_observer(json.loads(read_file(Path(sys.argv[2]), MIB)[1]))
    resource.setrlimit(resource.RLIMIT_AS, (768*MIB,)*2)
    signal.signal(signal.SIGALRM, lambda *_: (_ for _ in ()).throw(RuntimeError("Ownership repair deadline reached.")))
    signal.alarm(5400)
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.home()/"bitcoin-rs")
    report = run(parser.parse_args().repo)
    summary = {k:v for k,v in report.items() if k not in {"readouts", "cases", "verification", "cargo_evidence", "source_plan"}}
    summary["case_results"] = [{k:v for k,v in x.items() if k != "log"} for x in report["cases"]]
    print("BEGIN_OWNERSHIP_REPAIR_SUMMARY")
    print(json.dumps(summary, indent=2))
    print("END_OWNERSHIP_REPAIR_SUMMARY")
    if report.get("return_file"):
        print("Send back the full JSON file: " + report["return_file"])
    return 0 if report["outcome"] in {"qualified_candidate", "already_qualified"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
