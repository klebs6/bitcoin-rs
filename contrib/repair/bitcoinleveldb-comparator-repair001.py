#!/usr/bin/env python3
"""Resume and repair comparator mock lookups with owned-cache preservation."""

PAYLOADS = {'contrib/test/resource_guard.py': b'"""Fail-closed systemd user-service containment for build and test children."""\nimport collections\nimport json\nimport os\nfrom pathlib import Path\nimport re\nimport resource\nimport selectors\nimport shutil\nimport subprocess\nimport sys\nimport time\nimport uuid\n\nMIB = 1024 * 1024\nBUDGETS = {\n    "build": dict(memory=4096*MIB, high=3072*MIB, tasks=128, seconds=600, file=512*MIB, output=2*MIB),\n    "case": dict(memory=512*MIB, high=384*MIB, tasks=64, seconds=30, file=16*MIB, output=MIB),\n    "probe": dict(memory=128*MIB, high=96*MIB, tasks=32, seconds=10, file=16*MIB, output=MIB),\n}\nPROPERTIES = ("LoadState", "ActiveState", "SubState", "Type", "Restart", "RemainAfterExit", "Result",\n              "ExecMainCode", "ExecMainStatus", "RuntimeMaxUSec", "TimeoutStopUSec", "KillMode",\n              "OOMPolicy", "SendSIGKILL", "MemoryMax", "MemoryHigh", "MemorySwapMax", "TasksMax",\n              "CPUQuotaPerSecUSec", "MemoryPeak", "CPUUsageNSec", "ControlGroup")\n\n\ndef _atomic_json(path, value):\n    path = Path(path)\n    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)\n    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)\n    with os.fdopen(fd, "w") as stream:\n        json.dump(value, stream, sort_keys=True, indent=2)\n        stream.write("\\n")\n        stream.flush()\n        os.fsync(stream.fileno())\n    os.replace(temporary, path)\n\n\ndef _events(path):\n    try:\n        return {key: int(value) for key, value in (line.split() for line in Path(path).read_text().splitlines())}\n    except (OSError, ValueError):\n        return None\n\n\ndef _cgroup_snapshot(path):\n    result = {"path": str(path)}\n    for name in ("memory.max", "memory.high", "memory.swap.max", "cpu.max", "pids.max", "memory.oom.group", "memory.current", "memory.peak"):\n        try:\n            result[name] = (path / name).read_text().strip()\n        except OSError:\n            result[name] = None\n    result["memory.events"] = _events(path / "memory.events")\n    return result\n\n\ndef _headroom(path, budget):\n    """Conservative admission policy; observations can race later allocations."""\n    root = Path("/sys/fs/cgroup").resolve()\n    required = budget["memory"] + 256*MIB\n    match = re.search(r"^MemAvailable:\\s+(\\d+)\\s+kB\\s*$", Path("/proc/meminfo").read_text(), re.MULTILINE)\n    if not match:\n        raise RuntimeError("MemAvailable is unavailable; workload admission was refused.")\n    result = {"required_bytes": required, "mem_available_bytes": int(match.group(1))*1024, "ancestors": []}\n    if result["mem_available_bytes"] < required:\n        raise RuntimeError("Host memory headroom is below the declared admission margin: " + repr(result))\n    ancestor = path.parent\n    while ancestor.is_relative_to(root):\n        row = {"path": str(ancestor)}\n        try:\n            maximum = (ancestor / "memory.max").read_text().strip()\n        except OSError:\n            if ancestor != root:\n                raise RuntimeError("Ancestor memory.max is unreadable: " + str(ancestor))\n            maximum = None\n        try:\n            current = int((ancestor / "memory.current").read_text().strip())\n        except (OSError, ValueError):\n            if ancestor != root or maximum is not None:\n                raise RuntimeError("Ancestor memory.current is unreadable: " + str(ancestor))\n            current = None\n        row.update(maximum=maximum, current=current)\n        result["ancestors"].append(row)\n        if maximum not in (None, "max"):\n            try:\n                remaining = int(maximum) - current\n            except (TypeError, ValueError):\n                raise RuntimeError("Ancestor memory limit has an invalid value: " + str(ancestor))\n            row["remaining_bytes"] = remaining\n            if remaining < required:\n                raise RuntimeError("Ancestor memory headroom is below the declared admission margin: " + repr(result))\n        if ancestor == root:\n            break\n        ancestor = ancestor.parent\n    return result\n\n\ndef _kernel_probe(budget):\n    roots = [line[3:] for line in Path("/proc/self/cgroup").read_text().splitlines() if line.startswith("0::")]\n    if len(roots) != 1 or not Path("/sys/fs/cgroup/cgroup.controllers").is_file():\n        raise RuntimeError("A cgroup-v2 process membership and controller hierarchy are required.")\n    root = Path("/sys/fs/cgroup").resolve()\n    path = (root / roots[0].lstrip("/")).resolve()\n    if not path.is_relative_to(root):\n        raise RuntimeError("Unexpected cgroup path.")\n    result = _cgroup_snapshot(path)\n    expected = {"memory.max": str(budget["memory"]), "memory.high": str(budget["high"]),\n                "memory.swap.max": "0", "pids.max": str(budget["tasks"]), "memory.oom.group": "1"}\n    if any(result[key] != value for key, value in expected.items()):\n        raise RuntimeError("Kernel memory/swap/task/OOM controls differ from the requested policy: " + repr(result))\n    quota, period = result["cpu.max"].split()\n    if quota == "max" or int(quota) <= 0 or int(period) <= 0 or int(quota) > int(period):\n        raise RuntimeError("Kernel CPU quota is absent or exceeds one CPU.")\n    result["rlimit_core"] = list(resource.getrlimit(resource.RLIMIT_CORE))\n    result["rlimit_fsize"] = list(resource.getrlimit(resource.RLIMIT_FSIZE))\n    if result["rlimit_core"] != [0, 0] or result["rlimit_fsize"] != [budget["file"], budget["file"]]:\n        raise RuntimeError("Core/file-size resource limits were not installed.")\n    result["headroom"] = _headroom(path, budget)\n    return result\n\n\ndef guard_child(plan_path):\n    """Internal entry point; no requested command executes before both checks pass."""\n    plan = json.loads(Path(plan_path).read_text())\n    probe_path = Path(plan["probe"])\n    try:\n        budget = BUDGETS[plan["kind"]]\n        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))\n        resource.setrlimit(resource.RLIMIT_FSIZE, (budget["file"], budget["file"]))\n        snapshot = _kernel_probe(budget)\n        _atomic_json(probe_path, {"ok": True, "pid": os.getpid(), "kernel": snapshot})\n        until = time.monotonic() + 6\n        permit = Path(plan["permit"])\n        while not permit.exists():\n            if time.monotonic() >= until:\n                raise RuntimeError("Parent did not authorize execution after verifying service properties.")\n            time.sleep(0.025)\n        if permit.read_text() != plan["nonce"]:\n            raise RuntimeError("Execution authorization does not match this child.")\n        os.chdir(plan["cwd"])\n        os.execvpe(plan["command"][0], plan["command"], plan["env"])\n    except BaseException as error:\n        _atomic_json(probe_path, {"ok": False, "error": type(error).__name__ + ": " + str(error)})\n        print("GUARD_CHILD_REFUSED: " + str(error), file=sys.stderr)\n        return 78\n\n\ndef _seconds(value):\n    if value is None or value == "infinity":\n        return None\n    if value.isdigit():\n        return int(value) / 1000000\n    units = {"us": 0.000001, "ms": 0.001, "s": 1, "min": 60, "h": 3600, "d": 86400, "w": 604800}\n    compact = value.replace(" ", "").replace("\xc2\xb5s", "us").replace("\xce\xbcs", "us")\n    parts = re.findall(r"(\\d+(?:\\.\\d+)?)(us|ms|min|s|h|d|w)", compact)\n    if not parts or "".join(number + unit for number, unit in parts) != compact:\n        return None\n    return sum(float(number) * units[unit] for number, unit in parts)\n\n\ndef _service_controls(properties, budget):\n    expected = {"Type": "exec", "Restart": "no", "RemainAfterExit": "no", "KillMode": "control-group", "OOMPolicy": "kill", "SendSIGKILL": "yes"}\n    if any(properties.get(key) != value for key, value in expected.items()):\n        return False\n    runtime = _seconds(properties.get("RuntimeMaxUSec"))\n    stop = _seconds(properties.get("TimeoutStopUSec"))\n    return runtime is not None and 0 < runtime <= budget["seconds"] + 0.001 and stop is not None and 0 < stop <= 3.001\n\n\ndef _tree_usage(root):\n    size = count = 0\n    todo = [Path(root)]\n    try:\n        while todo:\n            with os.scandir(todo.pop()) as entries:\n                for entry in entries:\n                    count += 1\n                    if count > 4096:\n                        return size, count\n                    info = entry.stat(follow_symlinks=False)\n                    size += info.st_size\n                    if size > 128*MIB:\n                        return size, count\n                    if entry.is_dir(follow_symlinks=False):\n                        todo.append(Path(entry.path))\n    except FileNotFoundError:\n        pass\n    return size, count\n\n\nclass Guard:\n    def __init__(self, directory, script_path, base_env):\n        self.directory = Path(directory).resolve()\n        self.directory.mkdir(parents=True, exist_ok=True, mode=0o700)\n        self.script_path = Path(script_path).resolve()\n        self.base_env = dict(base_env)\n        self.systemd_run = shutil.which("systemd-run", path=self.base_env.get("PATH"))\n        self.systemctl = shutil.which("systemctl", path=self.base_env.get("PATH"))\n\n    def _ctl(self, *args):\n        child = None\n        try:\n            child = subprocess.Popen([self.systemctl, "--user", *args], env=self.base_env, stdin=subprocess.DEVNULL,\n                                     stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True)\n            os.set_blocking(child.stdout.fileno(), False)\n            buffer = bytearray()\n            until = time.monotonic() + 1.5\n            with selectors.DefaultSelector() as selector:\n                selector.register(child.stdout.fileno(), selectors.EVENT_READ)\n                while child.poll() is None or selector.get_map():\n                    if time.monotonic() >= until or len(buffer) > 32768:\n                        child.kill()\n                        return {"exit_code": None, "error": "Control query exceeded its time/output budget.", "stdout": bytes(buffer[:32768]).decode(errors="replace")}\n                    for key, _ in selector.select(0.05):\n                        try:\n                            data = os.read(key.fd, 4096)\n                        except BlockingIOError:\n                            continue\n                        if data:\n                            buffer.extend(data)\n                        else:\n                            selector.unregister(key.fd)\n            return {"exit_code": child.returncode, "stdout": buffer.decode(errors="replace"), "stderr": ""}\n        except OSError as error:\n            return {"exit_code": None, "error": str(error)}\n        finally:\n            if child:\n                if child.poll() is None:\n                    child.kill()\n                try:\n                    child.wait(timeout=0.2)\n                except subprocess.TimeoutExpired:\n                    pass\n                child.stdout.close()\n\n    def _show(self, unit):\n        result = self._ctl("show", unit, *["--property=" + key for key in PROPERTIES])\n        if result.get("exit_code") == 0:\n            return dict(line.split("=", 1) for line in result["stdout"].splitlines() if "=" in line)\n        return {"unavailable": result}\n\n    def preflight(self):\n        return self.run("preflight", [sys.executable, "-c", "print(\'GUARD_PREFLIGHT_OK\')"], self.directory, {}, kind="probe")\n\n    def run(self, label, command, cwd, env, kind="case", tmpdir=None):\n        budget = dict(BUDGETS[kind])\n        result = {"label": label, "kind": kind, "budget": budget, "outcome": "controls_unavailable", "verified": False, "exit_code": None}\n        if not self.systemd_run or not self.systemctl:\n            return dict(result, error="systemd-run and systemctl are required; unrestricted execution was refused.")\n        manager = self._ctl("show", "--property=Version", "--value")\n        if manager.get("exit_code") != 0:\n            return dict(result, error="The systemd user manager is unavailable.", manager=manager)\n        merged = dict(self.base_env)\n        merged.update({str(key): str(value) for key, value in env.items()})\n        executable = shutil.which(str(command[0]), path=merged.get("PATH"))\n        if executable is None:\n            return dict(result, error="Command executable was not found: " + str(command[0]))\n        # Keep the executable entry name: Nix dispatches nix-shell by argv[0].\n        command = [os.path.abspath(executable), *map(str, command[1:])]\n        stem = re.sub(r"[^a-zA-Z0-9_.-]", "-", str(label))[:40] + "-" + uuid.uuid4().hex[:12]\n        directory = self.directory / stem\n        directory.mkdir(mode=0o700)\n        data = Path(tmpdir).resolve() if tmpdir else directory / "test-data"\n        data.mkdir(parents=True, exist_ok=True, mode=0o700)\n        merged["TEST_TMPDIR"] = str(data)\n        plan = {"kind": kind, "command": command, "cwd": str(Path(cwd).resolve()), "env": merged,\n                "probe": str(directory / "probe.json"), "permit": str(directory / "permit"), "nonce": uuid.uuid4().hex}\n        plan_path = directory / "plan.json"\n        _atomic_json(plan_path, plan)\n        unit = "bitcoin-rs-guard-" + stem + ".service"\n        argv = [self.systemd_run, "--user", "--wait", "--pipe", "--service-type=exec", "--unit=" + unit,\n                "--property=MemoryAccounting=yes", "--property=CPUAccounting=yes", "--property=TasksAccounting=yes",\n                "--property=MemoryHigh=" + str(budget["high"]), "--property=MemoryMax=" + str(budget["memory"]),\n                "--property=MemorySwapMax=0", "--property=CPUQuota=100%", "--property=TasksMax=" + str(budget["tasks"]),\n                "--property=RuntimeMaxSec=" + str(budget["seconds"]) + "s", "--property=RuntimeRandomizedExtraSec=0",\n                "--property=TimeoutStartSec=5s", "--property=TimeoutStopSec=3s", "--property=KillMode=control-group",\n                "--property=OOMPolicy=kill", "--property=SendSIGKILL=yes", "--property=Restart=no", "--property=LimitCORE=0",\n                str(Path(sys.executable).resolve()), str(self.script_path), "--guard-child", str(plan_path)]\n        result.update(unit=unit, command=command, cwd=plan["cwd"], data_directory=str(data), log=str(directory / "output.log"))\n        started = time.monotonic()\n        deadline, termination_until = started + budget["seconds"] + 5, None\n        reason = None\n        child = None\n        tail = bytearray()\n        output_bytes = 0\n        probe = None\n        last_sample = started\n        last_cgroup_sample = started\n        snapshots = collections.deque(maxlen=8)\n        try:\n            child = subprocess.Popen(argv, env=self.base_env, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,\n                                     stderr=subprocess.STDOUT, bufsize=0)\n            fd = child.stdout.fileno()\n            os.set_blocking(fd, False)\n            selector = selectors.DefaultSelector()\n            selector.register(fd, selectors.EVENT_READ)\n            with (directory / "output.log").open("wb") as log:\n                while child.poll() is None or selector.get_map():\n                    now = time.monotonic()\n                    if probe is None and Path(plan["probe"]).exists():\n                        probe = json.loads(Path(plan["probe"]).read_text())\n                        result["probe"] = probe\n                        if not probe.get("ok"):\n                            reason = "controls_unavailable"\n                        else:\n                            properties = self._show(unit)\n                            result["verified_properties"] = properties\n                            if not _service_controls(properties, budget):\n                                reason = "controls_unavailable"\n                            else:\n                                Path(plan["permit"]).write_text(plan["nonce"])\n                                result["verified"] = True\n                    if now - last_sample >= 0.1:\n                        last_sample = now\n                        if kind == "case":\n                            size, count = _tree_usage(data)\n                            result["data_bytes"], result["data_entries"] = size, count\n                            if size > 128*MIB or count > 4096:\n                                reason = reason or "data_limit"\n                        if shutil.disk_usage(directory).free < 2*1024*MIB:\n                            reason = reason or "disk_free_limit"\n                        if probe and probe.get("ok") and now - last_cgroup_sample >= 0.5:\n                            snapshots.append(_cgroup_snapshot(Path(probe["kernel"]["path"])))\n                            last_cgroup_sample = now\n                    if now >= deadline:\n                        reason = reason or "parent_timeout"\n                    if reason and termination_until is None:\n                        result["stop_request"] = self._ctl("--no-block", "stop", unit)\n                        termination_until = time.monotonic() + 5\n                    if termination_until is not None and time.monotonic() >= termination_until:\n                        result["kill_request"] = self._ctl("kill", "--kill-whom=all", "--signal=KILL", unit)\n                        break\n                    for key, _ in selector.select(0.1):\n                        for _chunk in range(16):\n                            try:\n                                raw = os.read(key.fd, 65536)\n                            except BlockingIOError:\n                                break\n                            if not raw:\n                                selector.unregister(key.fd)\n                                break\n                            old_bytes = output_bytes\n                            output_bytes += len(raw)\n                            if old_bytes < budget["output"]:\n                                log.write(raw[:budget["output"] - old_bytes])\n                            tail.extend(raw)\n                            del tail[:-65536]\n                            if output_bytes > budget["output"]:\n                                reason = reason or "output_limit"\n                            text = tail.decode(errors="replace").lower()\n                            if "ignoring" in text and "restricted" in text and any(word in text for word in ("max-jobs", "builders", "substitute")):\n                                reason = reason or "restricted_nix_setting_ignored"\n                selector.close()\n            if child.poll() is None:\n                child.terminate()\n                try:\n                    child.wait(timeout=0.5)\n                except subprocess.TimeoutExpired:\n                    child.kill()\n                    child.wait(timeout=0.5)\n            result["exit_code"] = child.returncode\n        except KeyboardInterrupt:\n            reason = "interrupted"\n            result["stop_request"] = self._ctl("--no-block", "stop", unit)\n            result["kill_request"] = self._ctl("kill", "--kill-whom=all", "--signal=KILL", unit)\n        except Exception as error:\n            reason = reason or "guard_error"\n            result["error"] = type(error).__name__ + ": " + str(error)\n        finally:\n            if child and child.poll() is None:\n                result["emergency_stop"] = self._ctl("--no-block", "stop", unit)\n                result["emergency_kill"] = self._ctl("kill", "--kill-whom=all", "--signal=KILL", unit)\n                child.kill()\n                try:\n                    child.wait(timeout=0.2)\n                except subprocess.TimeoutExpired:\n                    result["client_reap_pending"] = True\n            result.update(elapsed_seconds=time.monotonic() - started, output_bytes=output_bytes,\n                          tail=tail.decode(errors="replace"), live_cgroup=list(snapshots), postmortem=self._show(unit))\n            if probe and probe.get("ok"):\n                result["final_cgroup"] = _cgroup_snapshot(Path(probe["kernel"]["path"]))\n            properties = result["postmortem"]\n            if properties.get("Result") == "oom-kill":\n                reason = reason or "memory_limit"\n            elif properties.get("Result") == "timeout":\n                reason = reason or "service_timeout"\n            result["outcome"] = reason or ("exited" if result["verified"] else "controls_unavailable")\n            result["cleanup"] = {"stop": self._ctl("--no-block", "stop", unit), "reset": self._ctl("reset-failed", unit)}\n            _atomic_json(directory / "result.json", result)\n            if child and child.stdout:\n                child.stdout.close()\n        return result\n\n\nif __name__ == "__main__":\n    if len(sys.argv) == 3 and sys.argv[1] == "--guard-child":\n        sys.exit(guard_child(sys.argv[2]))\n    raise SystemExit("Internal resource controller; use a crate verification command.")\n', 'contrib/test/leveldb-comparator.py': b'"""Register only the dedicated comparator build cache in local Git metadata."""\nimport hashlib\nimport os\nfrom pathlib import Path\nimport stat\nimport uuid\n\n\ndef _cache_write(path, data, mode):\n    path = Path(path)\n    path.parent.mkdir(parents=True, exist_ok=True)\n    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)\n    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, mode)\n    try:\n        with os.fdopen(fd, "wb") as stream:\n            stream.write(data); stream.flush(); os.fsync(stream.fileno())\n        os.chmod(temporary, mode)\n        os.replace(temporary, path)\n    finally:\n        if temporary.exists():\n            temporary.unlink()\n\n\ndef prepare_comparator_cache(owner, common, cache, evidence, git_call):\n    """Preserve all existing rules and prove the one owned cache is ignored.\n\n    git_call returns a CompletedProcess, uses owner as cwd, and does not raise\n    for a nonzero status. No tracked paths or source/index bytes are changed.\n    """\n    owner, common, cache, evidence = map(Path, (owner, common, cache, evidence))\n    if cache != owner / "target-comparator":\n        if cache.is_relative_to(owner):\n            raise RuntimeError("Use target-comparator or an external path for the comparator cache.")\n        return {"cache": str(cache), "changed": False, "reason": "external cache"}\n    if cache.is_symlink() or (cache.exists() and not cache.is_dir()):\n        raise RuntimeError("Comparator cache must be an ordinary directory.")\n    tracked = git_call("ls-files", "-z", "--", "target-comparator")\n    if tracked.returncode or tracked.stdout:\n        raise RuntimeError("Comparator cache contains tracked paths or could not be inspected.")\n    exclude = common / "info" / "exclude"\n    if exclude.parent.is_symlink() or exclude.is_symlink():\n        raise RuntimeError("Git cache exclusion metadata is a symlink; retained.")\n    if exclude.exists() and (not exclude.is_file() or exclude.stat().st_size > 1024 * 1024):\n        raise RuntimeError("Git cache exclusion metadata is not a capped ordinary file.")\n    before = exclude.read_bytes() if exclude.exists() else None\n    mode = stat.S_IMODE(exclude.stat().st_mode) if before is not None else 0o644\n    ignored = git_call("check-ignore", "--quiet", "--no-index", "--", "target-comparator/")\n    if ignored.returncode not in (0, 1):\n        raise RuntimeError("Could not verify comparator cache exclusion.")\n    result = {"cache": str(cache), "exclude": str(exclude), "changed": False,\n              "rule": "/target-comparator/", "mode": mode,\n              "before_sha256": hashlib.sha256(before).hexdigest() if before is not None else None}\n    if ignored.returncode == 0:\n        result["after_sha256"] = result["before_sha256"]\n        return result\n    evidence.mkdir(parents=True, exist_ok=True)\n    backup = evidence / "git-info-exclude.before"\n    if backup.is_symlink() or backup.exists():\n        raise RuntimeError("Cache exclusion backup path is already occupied.")\n    _cache_write(backup, before if before is not None else b"", 0o600)\n    after = (before or b"")\n    if after and not after.endswith(b"\\n"):\n        after += b"\\n"\n    after += b"/target-comparator/\\n"\n    if (exclude.read_bytes() if exclude.exists() else None) != before:\n        raise RuntimeError("Git exclusion rules changed concurrently; retained.")\n    _cache_write(exclude, after, mode)\n    checked = git_call("check-ignore", "--quiet", "--no-index", "--", "target-comparator/")\n    if checked.returncode != 0:\n        raise RuntimeError("Exact comparator cache rule did not take effect; backup retained.")\n    result.update(changed=True, backup=str(backup), before_existed=before is not None,\n                  after_sha256=hashlib.sha256(after).hexdigest())\n    return result\n\n#!/usr/bin/env python3\n"""Fresh, resource-controlled comparator regression checks."""\nimport argparse\nimport datetime as dt\nimport fcntl\nimport hashlib\nimport importlib.util\nimport json\nimport os\nfrom pathlib import Path\nimport re\nimport shlex\nimport stat\nimport subprocess\nimport sys\nimport uuid\n\nWORKFLOW = "leveldb-comparator-check001"\nLOCK_WORKFLOW = "leveldb-resource-tests001"\nCONTROLLER_PATH = "contrib/test/resource_guard.py"\nCONTROLLER_SHA256 = "7de80b8c219e9ae62473aa4e7a9727578c992fd6472c67599f4e87fe7d0b61db"\nSHELL_SHA256 = "8f316c4f007426302e26962e13def8c7d5bc0efa85d5e1549c55e85ba0d7c651"\nCRATE = "bitcoinleveldb-comparator"\nUNIT_TARGET = "bitcoinleveldb_comparator"\nTARGETS = {UNIT_TARGET: ("lib", "src/lib.rs"),\n           "comparator": ("test", "tests/comparator.rs"),\n           "mock_bytewise_contracts": ("test", "tests/mock_bytewise_contracts.rs")}\nUNIT_NAMES = ["borrowed_slice_comparator_adapter_box_returns_none_for_null_pointer",\n              "borrowed_slice_comparator_adapter_box_forwards_full_slice_comparator_surface"]\nCONTRACT_CASES = {name: "COMPARATOR_CONTRACT_OK case=" + name for name in (\n    "mock_bytewise_pointer_contract", "corrupt_mock_bytewise_pointer_contract")}\nTESTS = {UNIT_TARGET: UNIT_NAMES,\n         "comparator": ["test_compare_basic", "test_find_shortest_separator", "test_find_short_successor", "test_bytewise_comparator_singleton"],\n         "mock_bytewise_contracts": list(CONTRACT_CASES)}\nCASES = [(target, name) for target, names in TESTS.items() for name in names]\nFAILURE_FILES = {"mock_bytewise_pointer_contract": "mock_comparator.rs",\n                 "corrupt_mock_bytewise_pointer_contract": "mock_corrupt_comparator.rs"}\nBUILD = ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", CRATE, "--lib", "--tests", "--no-run", "--message-format=json"]\nRUSTC_VV = """rustc 1.97.0-nightly (9eb3be26b 2026-05-18)\nbinary: rustc\ncommit-hash: 9eb3be26b46eccea1de7448ea9cc3c1d20bb1a35\ncommit-date: 2026-05-18\nhost: x86_64-unknown-linux-gnu\nrelease: 1.97.0-nightly\nLLVM version: 22.1.4"""\nCARGO_VERSION = "cargo 1.97.0-nightly (4d1f98451 2026-05-15)"\nGIT_SELECTION = {"GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR", "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_PREFIX", "GIT_NAMESPACE"}\n\n\ndef encoded(value):\n    return (json.dumps(value, indent=2, sort_keys=True) + "\\n").encode()\n\n\ndef atomic_json(path, value):\n    path = Path(path)\n    path.parent.mkdir(parents=True, exist_ok=True)\n    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)\n    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)\n    try:\n        with os.fdopen(fd, "wb") as stream:\n            stream.write(encoded(value)); stream.flush(); os.fsync(stream.fileno())\n        os.replace(temporary, path)\n    finally:\n        if temporary.exists():\n            temporary.unlink()\n\n\ndef fingerprint(path):\n    path = Path(path)\n    info = path.lstat()\n    if not stat.S_ISREG(info.st_mode):\n        raise RuntimeError("Expected an ordinary file: " + str(path))\n    digest = hashlib.sha256()\n    with path.open("rb") as stream:\n        for chunk in iter(lambda: stream.read(65536), b""):\n            digest.update(chunk)\n    return {"kind": "file", "mode": stat.S_IMODE(info.st_mode), "size": info.st_size, "sha256": digest.hexdigest()}\n\n\ndef ordinary_path(path, root):\n    path, root = Path(path), Path(root)\n    if not path.is_relative_to(root):\n        raise RuntimeError("Expected a path inside the selected repository: " + str(path))\n    for part in [path, *path.parents]:\n        if part == root:\n            break\n        if part.is_symlink():\n            raise RuntimeError("Expected an ordinary repository path: " + str(path))\n    return path\n\n\ndef repository_dirs(repo):\n    """Read standard Git worktree metadata without spawning an unguarded child."""\n    dotgit = repo / ".git"\n    if dotgit.is_symlink():\n        raise RuntimeError("The selected .git path is a symlink.")\n    if dotgit.is_dir():\n        gitdir = dotgit.resolve()\n    elif dotgit.is_file() and dotgit.stat().st_size < 16384:\n        text = dotgit.read_text().strip()\n        if not text.startswith("gitdir: "):\n            raise RuntimeError("Unsupported Git worktree metadata.")\n        gitdir = (repo / text[8:]).resolve()\n    else:\n        raise RuntimeError("--repo must name a Git checkout root.")\n    common_file = gitdir / "commondir"\n    if common_file.exists():\n        if common_file.is_symlink() or common_file.stat().st_size >= 16384:\n            raise RuntimeError("Unsupported common Git directory metadata.")\n        common = (gitdir / common_file.read_text().strip()).resolve()\n    else:\n        common = gitdir\n    if not (gitdir / "HEAD").is_file() or not (common / "objects").is_dir():\n        raise RuntimeError("Incomplete Git checkout metadata.")\n    return gitdir, common\n\n\ndef checked_environment():\n    env = dict(os.environ)\n    cleared = sorted(key for key in GIT_SELECTION if env.pop(key, None) is not None)\n    bad = [key for key, value in env.items() if value and (key in {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_TARGET", "CARGO_BUILD_RUSTFLAGS", "CARGO_BUILD_BUILD_DIR", "CARGO_BUILD_RUSTC", "CARGO_BUILD_RUSTC_WRAPPER", "RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER"} or key.startswith("CARGO_PROFILE_") or (key.startswith("CARGO_TARGET_") and key.endswith(("_RUSTFLAGS", "_RUSTC_WRAPPER"))))]\n    if bad:\n        raise RuntimeError("Unexpected Rust/build overrides require review: " + repr(sorted(bad)))\n    env["GIT_OPTIONAL_LOCKS"] = "0"\n    return env, cleared\n\n\ndef load_controller(repo):\n    path = ordinary_path(repo / CONTROLLER_PATH, repo)\n    if fingerprint(path)["sha256"] != CONTROLLER_SHA256:\n        raise RuntimeError("The reviewed resource controller differs from its accepted SHA256.")\n    if fingerprint(ordinary_path(repo / "shell.nix", repo))["sha256"] != SHELL_SHA256:\n        raise RuntimeError("The selected shell.nix differs from the accepted pinned native environment.")\n    sys.dont_write_bytecode = True\n    spec = importlib.util.spec_from_file_location("comparator_resource_guard001", path)\n    module = importlib.util.module_from_spec(spec)\n    spec.loader.exec_module(module)\n    return module, path\n\n\ndef capture_inputs(repo, output):\n    """Internal probe. Its Git children and hashing run inside the probe service."""\n    env, _ = checked_environment()\n    def git(*args):\n        result = subprocess.run(["git", "-c", "core.fsmonitor=false", *args], cwd=repo, env=env, capture_output=True, timeout=5)\n        if result.returncode or len(result.stdout) > 4*1024*1024 or len(result.stderr) > 65536:\n            raise RuntimeError("Guarded Git metadata query failed or exceeded its output allowance.")\n        return result.stdout\n    if Path(os.fsdecode(git("rev-parse", "--show-toplevel")).strip()).resolve() != repo:\n        raise RuntimeError("--repo must identify the checkout root.")\n    gitdir, common = repository_dirs(repo)\n    names = {os.fsdecode(name) for name in git("ls-files", "-z", "--cached", "--others", "--exclude-standard").split(b"\\0") if name}\n    if len(names) > 100000:\n        raise RuntimeError("Input inventory exceeds the finite metadata allowance.")\n    selected = {name for name in names if name.endswith((".rs", ".nix")) or Path(name).name in {"Cargo.toml", "Cargo.lock", "rust-toolchain", "rust-toolchain.toml"} or ".cargo" in Path(name).parts}\n    selected.update({CONTROLLER_PATH, "contrib/test/leveldb-comparator.py", "shell.nix"})\n    inputs = {}\n    total = 0\n    for name in sorted(selected):\n        path = ordinary_path(repo / name, repo)\n        if path.stat().st_size > 16*1024*1024:\n            raise RuntimeError("A build input exceeds the finite inventory allowance: " + name)\n        total += path.stat().st_size\n        if total > 256*1024*1024:\n            raise RuntimeError("Build input inventory exceeds 256 MiB.")\n        inputs[name] = fingerprint(path)\n    configurations = {}\n    homes = [Path(env.get("CARGO_HOME", Path.home() / ".cargo"))]\n    homes.extend(parent / ".cargo" for parent in [repo, *repo.parents])\n    for home in homes:\n        for name in ["config", "config.toml"]:\n            path = home / name\n            if path.exists() or path.is_symlink():\n                configurations[str(path)] = fingerprint(path)\n    result = {"head": git("rev-parse", "HEAD").decode().strip(), "inputs": inputs, "cargo_configurations": configurations,\n              "index": fingerprint(gitdir / "index") if (gitdir / "index").exists() else None,\n              "git_directory": str(gitdir), "common_directory": str(common)}\n    atomic_json(output, result)\n    print("COMPARATOR_INPUTS_RECORDED", flush=True)\n\n\ndef _load_json(path):\n    path = Path(path)\n    if path.is_symlink() or not path.is_file() or path.stat().st_size > 8*1024*1024:\n        raise RuntimeError("Expected capped ordinary JSON evidence: " + str(path))\n    return json.loads(path.read_text())\n\n\n\ndef artifacts_from(controller, result, target_dir, repo):\n    found = {}\n    expected = set(TARGETS)\n    for line in log_text(result).splitlines():\n        try:\n            item = json.loads(line)\n        except ValueError:\n            continue\n        if not isinstance(item, dict):\n            continue\n        target = item.get("target", {})\n        name = target.get("name")\n        if item.get("reason") != "compiler-artifact" or name not in expected or not item.get("executable"):\n            continue\n        required_kind, source = TARGETS[name]\n        if required_kind not in target.get("kind", []) or item.get("profile", {}).get("test") is not True or Path(target.get("src_path", "")).resolve() != (repo / CRATE / source).resolve():\n            raise RuntimeError("Cargo returned an unexpected unit/integration-test artifact.")\n        path = Path(item["executable"])\n        if not path.is_absolute() or path.is_symlink() or not path.resolve().is_relative_to(target_dir) or not path.is_file() or not path.stat().st_mode & 0o111:\n            raise RuntimeError("Cargo test executable must be an ordinary executable inside the selected target cache.")\n        for part in [path, *path.parents]:\n            if part == target_dir:\n                break\n            if part.is_symlink():\n                raise RuntimeError("Cargo test executable path contains a symlink.")\n        record = {"path": str(path), "fingerprint": fingerprint(path)}\n        if name in found and found[name] != record:\n            raise RuntimeError("Cargo returned different executables for one target.")\n        found[name] = record\n    if set(found) != expected:\n        raise RuntimeError("Capped Cargo output did not identify all three expected test executables.")\n    return found\n\n\ndef log_text(result):\n    path = Path(result["log"])\n    if path.is_symlink() or not path.is_file() or path.stat().st_size > result["budget"]["output"]:\n        raise RuntimeError("Guard log is not an ordinary file within its output allowance.")\n    return path.read_bytes().decode(errors="replace")\n\n\ndef compact(result):\n    return {key: value for key, value in result.items() if key not in {"tail", "command"}} | {"tail": result.get("tail", "")[-2400:]}\n\n\ndef require_run(result, allowed_exits=(0,)):\n    if result.get("outcome") != "exited" or result.get("verified") is not True or result.get("exit_code") not in allowed_exits:\n        raise RuntimeError("Resource-controlled phase failed: " + str(result.get("label")) + ": "\n                           + str(result.get("outcome")) + ", exit=" + str(result.get("exit_code")))\n\n\ndef bound_names(target, text):\n    listed = [match[1] for line in text.splitlines() if (match := re.match(r"^(.+): test$", line))]\n    result = {}\n    for full_name in listed:\n        logical = full_name.rsplit("::", 1)[-1] if target == UNIT_TARGET else full_name\n        if logical in result or logical not in TESTS[target]:\n            raise RuntimeError("Unexpected or duplicated test metadata: " + target + ": " + full_name)\n        result[logical] = full_name\n    if set(result) != set(TESTS[target]):\n        raise RuntimeError("Required test metadata differs: " + target)\n    return result\n\n\ndef test_summary(text, name, expected_failure=False):\n    rows = [(match[1], *map(int, match.groups()[1:])) for line in text.splitlines()\n            if (match := re.match(r"^test result: (ok|FAILED)\\. (\\d+) passed; (\\d+) failed; (\\d+) ignored;", line))]\n    marker = CONTRACT_CASES.get(name)\n    if expected_failure:\n        if (rows != [("FAILED", 0, 1, 0)] or "not yet implemented" not in text\n                or FAILURE_FILES[name] not in text or marker in text):\n            raise RuntimeError("Expected stub failure was not established for " + name)\n        return {"passed": 0, "failed": 1, "ignored": 0, "expected_failure": True,\n                "failure_kind": "unimplemented comparator accessor", "last_progress": text.splitlines()[-16:]}\n    if rows != [("ok", 1, 0, 0)] or (marker is not None and text.splitlines().count(marker) != 1):\n        raise RuntimeError("Test did not produce one passing result and its required completion marker: " + name)\n    return {"passed": 1, "failed": 0, "ignored": 0, "expected_failure": False,\n            "marker": marker, "last_progress": text.splitlines()[-12:]}\n\n\ndef verify(repo, target_dir, directory, before_phase=None, expected_failures=frozenset()):\n    """Always execute fresh tests; expected stub failures are installer-only."""\n    repo, target_dir, directory = map(lambda p: Path(p).expanduser().resolve(), (repo, target_dir, directory))\n    directory.mkdir(parents=True, exist_ok=False, mode=0o700)\n    report_path = directory / "VERIFICATION_RETURN.json"\n    expected_failures = set(expected_failures)\n    report = {"workflow": WORKFLOW, "outcome": "failed", "phase": "preflight", "repository": str(repo),\n              "target_directory": str(target_dir), "run_directory": str(directory), "workspace_suite": "not run",\n              "full_coverage_claimed": False, "validation_reused": False, "build_command": BUILD,\n              "expected_failures": sorted(expected_failures), "readouts": {}, "cases": {}}\n    lock = None\n    try:\n        if expected_failures - CONTRACT_CASES.keys():\n            raise RuntimeError("Only the two new comparator contracts may be expected stub failures.")\n        controller, controller_path = load_controller(repo)\n        env, report["cleared_git_selection_variables"] = checked_environment()\n        _, common = repository_dirs(repo)\n        metadata = common / "codex-repairs" / LOCK_WORKFLOW\n        metadata.mkdir(parents=True, exist_ok=True)\n        lock_path = metadata / "run.lock"\n        if lock_path.is_symlink():\n            raise RuntimeError("Verification lock is a symlink.")\n        lock = lock_path.open("a+")\n        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)\n        def cache_git(*args):\n            return subprocess.run(["git", "-c", "core.fsmonitor=false", *args], cwd=common.parent,\n                                  env=env, capture_output=True, timeout=5)\n        report["cache_exclusion"] = prepare_comparator_cache(common.parent, common, target_dir,\n                                                              directory / "cache-exclusion", cache_git)\n        target_dir.mkdir(parents=True, exist_ok=True)\n        report.update(budgets=controller.BUDGETS, controller_sha256=CONTROLLER_SHA256, shell_sha256=SHELL_SHA256)\n        guard = controller.Guard(directory / "guard", controller_path, env)\n        runtime_env = {"CARGO_TARGET_DIR": str(target_dir), "CARGO_BUILD_JOBS": "1", "CARGO_NET_OFFLINE": "true",\n                       "RUST_BACKTRACE": "1", "RUSTFLAGS": "-Awarnings", "CARGO_NET_GIT_FETCH_WITH_CLI": "true"}\n        def phase(label, command, kind="case", extra=None, tmpdir=None, allowed_exits=(0,)):\n            if before_phase:\n                before_phase(label)\n            report["phase"] = label\n            atomic_json(report_path, report)\n            print("Protected comparator phase: " + label, flush=True)\n            result = guard.run(label, command, repo, dict(runtime_env, **(extra or {})), kind=kind, tmpdir=tmpdir)\n            report["readouts"][label] = compact(result)\n            atomic_json(report_path, report)\n            require_run(result, allowed_exits)\n            return result\n        preflight = phase("preflight", [sys.executable, "-c", "print(\'GUARD_PREFLIGHT_OK\')"], "probe")\n        if "GUARD_PREFLIGHT_OK" not in log_text(preflight):\n            raise RuntimeError("Guard preflight success marker is absent.")\n        def snapshot(label):\n            output = directory / (label + ".json")\n            result = phase(label, [sys.executable, str(Path(__file__).resolve()), "--capture-inputs", str(repo), str(output)],\n                           "probe", extra={"RUSTFLAGS": ""})\n            if "COMPARATOR_INPUTS_RECORDED" not in log_text(result):\n                raise RuntimeError("Input snapshot completion evidence is absent.")\n            return _load_json(output)\n        initial = snapshot("inputs-before")\n        report["head"] = initial["head"]\n        report["inputs_before_sha256"] = hashlib.sha256(encoded(initial)).hexdigest()\n        nix = ["nix-shell", "--option", "max-jobs", "0", "--option", "builders", "", "--option", "substitute", "false",\n               str(repo / "shell.nix"), "--run"]\n        native = phase("native-tools", nix + ["rustc -Vv && cargo -V"], "build")\n        if RUSTC_VV not in log_text(native) or CARGO_VERSION not in log_text(native).splitlines():\n            raise RuntimeError("Rust/Cargo differs from the accepted native toolchain.")\n        built = phase("compile-tests", nix + [shlex.join(BUILD)], "build")\n        artifacts = artifacts_from(controller, built, target_dir, repo)\n        report["artifacts"] = artifacts\n        listings, actual_names = {}, {}\n        for target in TARGETS:\n            path = Path(artifacts[target]["path"])\n            if fingerprint(path) != artifacts[target]["fingerprint"]:\n                raise RuntimeError("Compiled test executable changed before metadata inspection.")\n            result = phase("list-" + target, [str(path), "--list"])\n            bound = bound_names(target, log_text(result))\n            actual_names.update(bound)\n            listings[target] = sorted(bound.values())\n        report["protected_test_lists"] = listings\n        for target, name in CASES:\n            path = Path(artifacts[target]["path"])\n            if fingerprint(path) != artifacts[target]["fingerprint"]:\n                raise RuntimeError("Compiled test executable changed before case execution.")\n            expected_failure = name in expected_failures\n            result = phase(name, [str(path), "--exact", actual_names[name], "--nocapture", "--test-threads=1"],\n                           tmpdir=directory / ("case-data-" + name), allowed_exits=(101,) if expected_failure else (0,))\n            report["cases"][name] = test_summary(log_text(result), name, expected_failure)\n            report["cases"][name].update(target=target, executed_name=actual_names[name])\n            atomic_json(report_path, report)\n        for target, record in artifacts.items():\n            if fingerprint(Path(record["path"])) != record["fingerprint"]:\n                raise RuntimeError("Compiled test executable changed during verification: " + target)\n        final = snapshot("inputs-after")\n        report["inputs_after_sha256"] = hashlib.sha256(encoded(final)).hexdigest()\n        if final != initial:\n            raise RuntimeError("Repository/build inputs or Git index changed during verification; results were not accepted.")\n        report.update(outcome="baseline_confirmed" if expected_failures else "passed", phase="complete",\n                      passed=sum(row["passed"] for row in report["cases"].values()),\n                      expected_failed=len(expected_failures), test_count=len(CASES), inputs_preserved=True)\n        atomic_json(report_path, report)\n        return report\n    except BaseException as error:\n        report.update(outcome="failed", error=type(error).__name__ + ": " + str(error))\n        atomic_json(report_path, report)\n        raise\n    finally:\n        if lock:\n            lock.close()\n\n\ndef main():\n    if len(sys.argv) == 4 and sys.argv[1] == "--capture-inputs":\n        capture_inputs(Path(sys.argv[2]).resolve(), Path(sys.argv[3])); return 0\n    parser = argparse.ArgumentParser(description=__doc__)\n    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[2], help="checkout to verify; never implicitly switched")\n    parser.add_argument("--target-dir", type=Path, help="dedicated cache; default COMMON_GIT_DIR/../target-comparator")\n    args = parser.parse_args()\n    repo = args.repo.expanduser().resolve()\n    directory = None\n    try:\n        _, common = repository_dirs(repo)\n        target = args.target_dir.expanduser().resolve() if args.target_dir else common.parent / "target-comparator"\n        directory = common / "codex-repairs" / WORKFLOW / "runs" / (dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])\n        report = verify(repo, target, directory)\n    except BaseException as error:\n        report = _load_json(directory / "VERIFICATION_RETURN.json") if directory and (directory / "VERIFICATION_RETURN.json").is_file() else {"workflow": WORKFLOW, "outcome": "failed", "error": type(error).__name__ + ": " + str(error)}\n    print("BEGIN_COMPARATOR_CHECK_RETURN"); print(json.dumps(report, indent=2)); print("END_COMPARATOR_CHECK_RETURN")\n    if directory:\n        print("Return file: " + str(directory / "VERIFICATION_RETURN.json"))\n    return 0 if report.get("outcome") == "passed" else 1\n\n\nif __name__ == "__main__":\n    sys.exit(main())\n', 'bitcoinleveldb-comparator/tests/mock_bytewise_contracts.rs': b'// Focused contracts for the mocks\' canonical bytewise-comparator capability.\nuse bitcoin_imports::Named;\nuse bitcoinleveldb_comparator::*;\nuse bitcoinleveldb_slice::Slice;\n\nfn assert_canonical_bytewise(pointer: *const dyn SliceComparator) {\n    let canonical = bytewise_comparator();\n    assert!(!pointer.is_null(), "the delegated comparator must be non-null");\n    assert!(!canonical.is_null());\n    assert!(core::ptr::addr_eq(pointer, canonical),\n            "the capability must refer to the canonical singleton object");\n    // The preceding address check and the public lifetime contract establish\n    // the expected singleton before any dereference of the returned pointer.\n    let comparator = unsafe { &*pointer };\n    assert_eq!(comparator.name().as_ref(), "leveldb.BytewiseComparator");\n    let keys: [&[u8]; 10] = [\n        b"", b"\\0", b"\\0\\0", b"a", b"a\\0", b"\\x7f", b"\\x80",\n        b"\\xfe", b"\\xff", b"\\xff\\0",\n    ];\n    for left in &keys {\n        for right in &keys {\n            let a = Slice::from(*left);\n            let b = Slice::from(*right);\n            assert_eq!(comparator.compare(&a, &b).cmp(&0), left.cmp(right),\n                       "canonical byte ordering differs for {:?} and {:?}", left, right);\n        }\n    }\n\n    for (start, limit, expected) in [\n        (b"".to_vec(), b"a".to_vec(), b"".to_vec()),\n        (b"a".to_vec(), b"aa".to_vec(), b"a".to_vec()),\n        (b"a\\0tail".to_vec(), b"a\\x02".to_vec(), b"a\\x01".to_vec()),\n        (vec![0xfe, 0], vec![0xff], vec![0xfe, 0]),\n        (vec![0xff, 0], vec![0xff, 1], vec![0xff, 0]),\n    ] {\n        assert!(start < limit, "separator fixture must satisfy its precondition");\n        let mut shortened = start.clone();\n        comparator.find_shortest_separator(&mut shortened, &limit);\n        assert_eq!(shortened, expected);\n        assert!(start <= shortened && shortened < limit);\n        assert!(shortened.len() <= start.len());\n    }\n    for (original, expected) in [\n        (vec![], vec![]),\n        (vec![0xff, 0xff], vec![0xff, 0xff]),\n        (vec![0xfe, 0xff], vec![0xff]),\n        (vec![0, 0xff], vec![1]),\n    ] {\n        let mut successor = original.clone();\n        comparator.find_short_successor(&mut successor);\n        assert_eq!(successor, expected);\n        assert!(successor >= original);\n        assert!(successor.len() <= original.len());\n    }\n}\n\nfn assert_mock_forwarding<M: SliceComparator + \'static>(mock: &M, corrupt: bool) {\n    let underlying = mock as *const M as *const dyn SliceComparator;\n    // Keep the concrete mock alive until the adapter is dropped at function\n    // exit, satisfying the adapter\'s documented non-owning lifetime condition.\n    let adapter = borrowed_slice_comparator_adapter_box(underlying)\n        .expect("a live non-null mock must be adaptable");\n    assert_eq!(adapter.name().as_ref(), "dummy-comparator");\n    let a = Slice::from(&b"a"[..]);\n    let b = Slice::from(&b"b"[..]);\n    if corrupt {\n        assert_eq!(mock.compare(&a, &b), 0);\n        assert_eq!(adapter.compare(&a, &b), 0,\n                   "requesting bytewise capability must not replace the corrupt mock");\n    } else {\n        assert!(mock.compare(&a, &b) < 0);\n        assert!(adapter.compare(&a, &b) < 0);\n    }\n    let mut separator = b"foo1".to_vec();\n    adapter.find_shortest_separator(&mut separator, b"foo9");\n    assert_eq!(separator, b"foo1", "the mock\'s no-op shortener must remain a no-op");\n    let mut successor = b"foo".to_vec();\n    adapter.find_short_successor(&mut successor);\n    assert_eq!(successor, b"foo", "the adapter must preserve the mock\'s no-op successor");\n    assert_canonical_bytewise(adapter.bytewise_comparator());\n    if corrupt {\n        assert_eq!(adapter.compare(&a, &b), 0,\n                   "querying the capability must leave adapter dispatch unchanged");\n    }\n}\n\n#[test]\nfn mock_bytewise_pointer_contract() {\n    let pointer = {\n        let mock = MockComparator::default();\n        assert_mock_forwarding(&mock, false);\n        mock.bytewise_comparator()\n    };\n    assert_canonical_bytewise(pointer);\n    for _ in 0..8 {\n        assert_canonical_bytewise(MockComparator::default().bytewise_comparator());\n    }\n    eprintln!("\\nCOMPARATOR_CONTRACT_OK case=mock_bytewise_pointer_contract");\n}\n\n#[test]\nfn corrupt_mock_bytewise_pointer_contract() {\n    let pointer = {\n        let mock = MockCorruptComparator::default();\n        assert_mock_forwarding(&mock, true);\n        mock.bytewise_comparator()\n    };\n    assert_canonical_bytewise(pointer);\n    for _ in 0..8 {\n        assert_canonical_bytewise(MockCorruptComparator::default().bytewise_comparator());\n    }\n    eprintln!("\\nCOMPARATOR_CONTRACT_OK case=corrupt_mock_bytewise_pointer_contract");\n}\n', 'doc/testing/leveldb-comparator.md': b'# Guarded comparator verification\n\nRun the eight comparator unit/integration cases through the repository command:\n\n```sh\nnix shell --offline --option max-jobs 0 --option builders \'\' \\\n  nixpkgs#python3 --command python3 \\\n  "$HOME/bitcoin-rs-comparator-repair/contrib/test/leveldb-comparator.py" \\\n  --repo "$HOME/bitcoin-rs-comparator-repair"\n```\n\nThe default checkout is the checkout containing the command. `--repo` selects a\ndifferent checkout explicitly; the command never switches branches. A checkout\nmust contain the reviewed standalone `contrib/test/resource_guard.py`, pinned\n`shell.nix`, and comparator tests. No historical installer journal is required.\n\nEvery invocation checks the native toolchain, compiles as needed, identifies\nthe library-unit and two integration-test executables, lists their required\ntests, and runs one exact test process at a time. Two adapter unit tests and four\nexisting comparator integration tests run alongside two mock-accessor contracts.\nUnit-test module prefixes are taken from the observed list; the two expected\nfunction basenames must each occur exactly once. Integration names match exactly.\nPrevious receipts never substitute for a fresh execution.\n\nThe new contracts check that both mocks return the program-lifetime bytewise\ncomparator pointer. They also exercise forwarding through a borrowed adapter\nwithout changing each mock\'s own comparison, name, or key-shortening behavior.\nThe deliberately corrupt mock remains deliberately corrupt.\n\nThe installer first adds these contracts against the old implementation. It\naccepts the two specified `todo!()` panics as the baseline only when each test\nexits normally with libtest status 101, exactly one failure, the matching source\nfilename, and `not yet implemented` in its panic output. A timeout, OOM, unrelated\nassertion, ignored case, or passing old stub is not an accepted baseline. The\nnormal repository command exposes no option to accept failures. After repair,\nall eight cases must pass; only that result can qualify the repair checkpoint.\n\n## Resources and concurrent work\n\n| Phase | Memory hard / soft | Deadline | Tasks | Output cap |\n| --- | --- | --- | --- | --- |\n| Build/native tools | 4 GiB / 3 GiB | 600 s | 128 | 2 MiB |\n| Individual case/metadata | 512 MiB / 384 MiB | 30 s | 64 | 1 MiB |\n| Preflight/input probe | 128 MiB / 96 MiB | 10 s | 32 | 1 MiB |\n\nAll children use verified systemd user services, one CPU\'s aggregate quota,\ndisabled swap, grouped OOM handling, whole-cgroup termination, and capped file\nsizes. The controller also enforces output, test-data, disk-headroom, and external\nwall-clock checks. Missing controls cause refusal before the requested command.\nResource termination is a failed verification, distinct from assertion failure.\n\nCargo uses `--offline --locked --jobs 1`; exact test subprocesses use\n`--test-threads=1`. Nix evaluation uses local jobs zero, an empty builder list,\nand substitution disabled. Cached requirements must already be available.\nThere is no unrestricted retry or system-configuration change.\n\nThe default Cargo cache is `target-comparator` beside the common Git directory,\nseparate from the randomized repair\'s `target` cache. The command registers only\nthe exact `/target-comparator/` rule in local `.git/info/exclude`, retaining the\nexisting rules and backing up the file before any edit. It refuses a symlink or\ntracked content at that cache path. Generated Rust files in the cache therefore\nremain separate from source-input checks; untracked Rust elsewhere is still\nchecked. `--target-dir PATH` can select an external cache. A dedicated cache avoids ordinary target-directory contention\nwith another repair, at the cost of compiling dependencies into that cache on\nits first run. Build-directory overrides in Cargo configuration can alter that\nlayout; configurations are inventoried, and environment overrides require review.\n\nA nonblocking `leveldb-resource-tests001` lock serializes cooperating comparator\nverification commands in the same repository. Independent older controllers have\ntheir own locks and budgets. Source isolation and resource limits should be used\nfor other concurrent builds as well.\n\n## Reports and scope\n\nReports print between `BEGIN_COMPARATOR_CHECK_RETURN` and\n`END_COMPARATOR_CHECK_RETURN`; the saved `VERIFICATION_RETURN.json` path follows.\nCapped phase logs and reports are retained under the common Git directory at\n`codex-repairs/leveldb-comparator-check001/runs/`, including failed runs.\n\nGuarded input probes compare selected tracked/nonignored Rust, Cargo, Nix,\ncontroller/runner files, Cargo configurations, HEAD, and index before and after\nverification. A difference prevents acceptance. This is an observation across\nthe run, not an immutable filesystem snapshot or an inventory of every external\nincluded asset. The routine command changes no source, index, branch, commit,\nor tag; Cargo updates its selected build cache.\n\nThe result qualifies the selected comparator unit/integration cases. It does\nnot run the workspace-wide suite, documentation tests, randomized stress, or a\ncoverage measurement. Full coverage is not claimed.\n'}

"""Register only the dedicated comparator build cache in local Git metadata."""
import hashlib
import os
from pathlib import Path
import stat
import uuid


def _cache_write(path, data, mode):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, mode)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(data); stream.flush(); os.fsync(stream.fileno())
        os.chmod(temporary, mode)
        os.replace(temporary, path)
    finally:
        if temporary.exists():
            temporary.unlink()


def prepare_comparator_cache(owner, common, cache, evidence, git_call):
    """Preserve all existing rules and prove the one owned cache is ignored.

    git_call returns a CompletedProcess, uses owner as cwd, and does not raise
    for a nonzero status. No tracked paths or source/index bytes are changed.
    """
    owner, common, cache, evidence = map(Path, (owner, common, cache, evidence))
    if cache != owner / "target-comparator":
        if cache.is_relative_to(owner):
            raise RuntimeError("Use target-comparator or an external path for the comparator cache.")
        return {"cache": str(cache), "changed": False, "reason": "external cache"}
    if cache.is_symlink() or (cache.exists() and not cache.is_dir()):
        raise RuntimeError("Comparator cache must be an ordinary directory.")
    tracked = git_call("ls-files", "-z", "--", "target-comparator")
    if tracked.returncode or tracked.stdout:
        raise RuntimeError("Comparator cache contains tracked paths or could not be inspected.")
    exclude = common / "info" / "exclude"
    if exclude.parent.is_symlink() or exclude.is_symlink():
        raise RuntimeError("Git cache exclusion metadata is a symlink; retained.")
    if exclude.exists() and (not exclude.is_file() or exclude.stat().st_size > 1024 * 1024):
        raise RuntimeError("Git cache exclusion metadata is not a capped ordinary file.")
    before = exclude.read_bytes() if exclude.exists() else None
    mode = stat.S_IMODE(exclude.stat().st_mode) if before is not None else 0o644
    ignored = git_call("check-ignore", "--quiet", "--no-index", "--", "target-comparator/")
    if ignored.returncode not in (0, 1):
        raise RuntimeError("Could not verify comparator cache exclusion.")
    result = {"cache": str(cache), "exclude": str(exclude), "changed": False,
              "rule": "/target-comparator/", "mode": mode,
              "before_sha256": hashlib.sha256(before).hexdigest() if before is not None else None}
    if ignored.returncode == 0:
        result["after_sha256"] = result["before_sha256"]
        return result
    evidence.mkdir(parents=True, exist_ok=True)
    backup = evidence / "git-info-exclude.before"
    if backup.is_symlink() or backup.exists():
        raise RuntimeError("Cache exclusion backup path is already occupied.")
    _cache_write(backup, before if before is not None else b"", 0o600)
    after = (before or b"")
    if after and not after.endswith(b"\n"):
        after += b"\n"
    after += b"/target-comparator/\n"
    if (exclude.read_bytes() if exclude.exists() else None) != before:
        raise RuntimeError("Git exclusion rules changed concurrently; retained.")
    _cache_write(exclude, after, mode)
    checked = git_call("check-ignore", "--quiet", "--no-index", "--", "target-comparator/")
    if checked.returncode != 0:
        raise RuntimeError("Exact comparator cache rule did not take effect; backup retained.")
    result.update(changed=True, backup=str(backup), before_existed=before is not None,
                  after_sha256=hashlib.sha256(after).hexdigest())
    return result

"""Exact-token method replacement, preserving the surrounding Git source bytes."""

MOCK_METHOD_OLD = "fn bytewise_comparator(&self) -> *const (dyn SliceComparator + 'static) { todo!() }"
MOCK_METHOD_NEW = "fn bytewise_comparator(&self) -> *const (dyn SliceComparator + 'static) { crate::bytewise_comparator() }"


def comparator_source_plan(data, rust_tokens):
    """Use the accepted snapshot runner's Rust lexer; accept only the two bodies.

    The caller supplies bytes from the reviewed baseline Git blob. Comments,
    whitespace, file banners, and EOF bytes outside the replaced expression
    are preserved. Working/index ownership remains the installer's concern.
    """
    source = data.decode("utf-8")
    tokens = rust_tokens(source)
    values = [item[0] for item in tokens]
    candidates = [i for i in range(len(values) - 1)
                  if values[i:i + 2] == ["fn", "bytewise_comparator"]]
    if len(candidates) != 1:
        raise RuntimeError("Expected exactly one bytewise_comparator method.")
    begin = candidates[0]
    if begin and values[begin - 1] not in {"{", "}", ";"}:
        raise RuntimeError("Unexpected attribute or modifier before comparator method.")
    body = begin
    while body < len(tokens) and values[body] != "{":
        if values[body] == ";":
            raise RuntimeError("Comparator method is a declaration without a body.")
        body += 1
    if body == len(tokens):
        raise RuntimeError("Comparator method body is absent.")
    depth, end = 1, body + 1
    while end < len(tokens) and depth:
        if values[end] == "{":
            depth += 1
        elif values[end] == "}":
            depth -= 1
        end += 1
    if depth:
        raise RuntimeError("Comparator method body is unterminated.")
    actual = values[begin:end]
    old = [item[0] for item in rust_tokens(MOCK_METHOD_OLD)]
    new = [item[0] for item in rust_tokens(MOCK_METHOD_NEW)]
    if actual not in (old, new):
        raise RuntimeError("Comparator method differs from both reviewed old/new token streams.")
    matched = "old" if actual == old else "repaired"
    observation = {"function": "bytewise_comparator", "matched": matched,
                   "start_line": source.count("\n", 0, tokens[begin][1]) + 1,
                   "end_line": source.count("\n", 0, tokens[end - 1][2]) + 1}
    if matched == "repaired":
        return data, observation
    # Replace only the todo!() expression. Preserve its surrounding comments
    # and whitespace as well as the complete method signature and file bytes.
    expression_start = tokens[body + 1][1]
    expression_end = tokens[end - 2][2]
    changed = source[:expression_start] + "crate::bytewise_comparator()" + source[expression_end:]
    return changed.encode("utf-8"), observation

"""Isolate, reproduce, repair, and verify the comparator mock contracts.

Assembly provides PAYLOADS containing the guard, runner, tests, and documentation.
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

BASE = "a8beeac26124749a69d0ad6f7a999891f8be73f1"
BRANCH = "repair/leveldb-comparator"
WORKFLOW = "leveldb-comparator-repair001"
SCRIPT_REVISION = "002"
PREVIOUS_SCRIPT_SHA = "16319d4a308dfcf075f6a0342f46ca94273de9624193d2e70d9a7090595c47db"
WORKTREE_SUFFIX = "-comparator-repair"
ENV_SCRIPT = "contrib/repair/bitcoinrs-nix-development-shell001.py"
ENV_SHA = "cd12ff326dc5aff610f3afede38c40ede2d4e46dd84801dd04a80a1faf6f0c97"
GUARD_PATH = "contrib/test/resource_guard.py"
GUARD_SHA = "7de80b8c219e9ae62473aa4e7a9727578c992fd6472c67599f4e87fe7d0b61db"
ROUTINE_PATH = "contrib/test/leveldb-comparator.py"
REGRESSION_PATH = "bitcoinleveldb-comparator/tests/mock_bytewise_contracts.rs"
DOCUMENT_PATH = "doc/testing/leveldb-comparator.md"
REPLAY_PATH = "contrib/repair/bitcoinleveldb-comparator-repair001.py"
RECEIPT_PATH = "doc/repair/leveldb-comparator-repair001.json"
SOURCE_PATHS = {"bitcoinleveldb-comparator/src/mock_comparator.rs", "bitcoinleveldb-comparator/src/mock_corrupt_comparator.rs"}
PATHS = SOURCE_PATHS | {GUARD_PATH, ROUTINE_PATH, REGRESSION_PATH, DOCUMENT_PATH, REPLAY_PATH, RECEIPT_PATH}
EXECUTABLES = {GUARD_PATH, ROUTINE_PATH, REPLAY_PATH}
REGRESSIONS = {"mock_bytewise_pointer_contract", "corrupt_mock_bytewise_pointer_contract"}
OLD_ITEM = "fn bytewise_comparator(&self) -> *const (dyn SliceComparator + 'static) { todo!() }"
NEW_ITEM = "fn bytewise_comparator(&self) -> *const (dyn SliceComparator + 'static) { crate::bytewise_comparator() }"


def checked_import(path, digest, name):
    path = Path(path)
    if path.is_symlink() or hashlib.sha256(path.read_bytes()).hexdigest() != digest:
        raise RuntimeError("Reviewed controller differs: " + str(path))
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def require_run(result):
    if result.get("verified") is not True or result.get("outcome") != "exited" or result.get("exit_code") != 0:
        raise RuntimeError("Protected phase failed: " + str(result.get("label")) + ", outcome=" + str(result.get("outcome")) + ", exit=" + str(result.get("exit_code")))


def worker_guard(runner, final, head, files, old, pending, initial=False, staged_files=None):
    runner.guard_operations()
    if runner.git("symbolic-ref", "--quiet", "--short", "HEAD") != BRANCH or runner.git("rev-parse", "HEAD") != head:
        raise RuntimeError("Comparator branch advanced beyond its owned checkpoint.")
    dirty = final.names(runner, "diff", "--no-renames", "--name-only") | final.names(runner, "diff", "--cached", "--no-renames", "--name-only")
    untracked = final.names(runner, "ls-files", "--others", "--exclude-standard")
    if (dirty | untracked) - (PATHS if pending else set()):
        raise RuntimeError("Comparator worktree contains unrelated changes; retained.")
    configs = {".cargo/config", ".cargo/config.toml"}
    tracked = final.names(runner, "ls-files", "--", *sorted(configs))
    if any((runner.repo / name).exists() or (runner.repo / name).is_symlink() for name in configs - tracked):
        raise RuntimeError("Untracked/ignored Cargo configuration requires review.")
    for name, expected in files.items():
        path = runner.repo / name
        for part in [path, *path.parents]:
            if part == runner.repo:
                break
            if part.is_symlink() or (part != path and part.exists() and not part.is_dir()):
                raise RuntimeError("Owned path has a symlink or non-directory ancestor: " + name)
        current = path.read_bytes() if path.exists() else None
        indexed = final.blob(runner, "", name, False)
        if current not in ({expected, old.get(name)} if initial and pending else {expected}):
            raise RuntimeError("Unexpected owned working content retained: " + name)
        allowed_index = {expected, old.get(name)} if pending else {expected}
        if pending and staged_files is not None:
            allowed_index.add(staged_files[name])
        if indexed not in allowed_index:
            raise RuntimeError("Unexpected owned staged content retained: " + name)
        if path.exists() and path.stat().st_mode & 0o7777 != (0o755 if name in EXECUTABLES else 0o644):
            raise RuntimeError("Unexpected owned permissions retained: " + name)


def own_commit(runner, final, head, files):
    return (runner.git("show", "-s", "--format=%P", head).split() == [BASE]
            and final.changed(runner, BASE, head) == PATHS
            and all(final.blob(runner, head, name, False) == data
                    and runner.git("ls-tree", head, "--", name).startswith(("100755" if name in EXECUTABLES else "100644") + " blob ")
                    for name, data in files.items()))


def validate_result(result, baseline=False):
    if result.get("outcome") != ("baseline_confirmed" if baseline else "passed") or len(result.get("cases", {})) != 8:
        raise RuntimeError("Comparator verifier did not complete its eight-case contract.")
    if result.get("inputs_preserved") is not True or result.get("full_coverage_claimed") is not False:
        raise RuntimeError("Comparator verifier input/scope evidence differs.")
    for name, row in result["cases"].items():
        expected = baseline and name in REGRESSIONS
        if row.get("expected_failure", False) != expected or (row.get("passed"), row.get("failed"), row.get("ignored")) != ((0, 1, 0) if expected else (1, 0, 0)):
            raise RuntimeError("Comparator case outcome differs: " + name)


def migrate_pending_baseline(runner, final, repair, metadata, state_path, layout_path,
                             state, identity, files, old, worktree, script_path):
    """Upgrade only the owned pre-test predecessor; retain immutable backups."""
    layout = json.loads(layout_path.read_text())
    new_layout = dict(identity, origin=str(runner.origin), path=str(worktree))
    if all(state.get(key) == value for key, value in identity.items()) and layout == new_layout and state.get("upgrade", {}).get("status") != "in_progress":
        return state
    backup = metadata / "upgrade-revision002"
    if backup.is_symlink():
        raise RuntimeError("Comparator upgrade backup directory is a symlink.")
    backup.mkdir(exist_ok=True, mode=0o700)
    paths = {"state": backup / "state.json", "layout": backup / "worktree.json", "replay": backup / "replay.py"}
    manifest_path = backup / "manifest.json"
    if any(path.is_symlink() for path in [*paths.values(), manifest_path]):
        raise RuntimeError("Comparator upgrade evidence contains a symlink.")
    replay_path = worktree / REPLAY_PATH
    if not manifest_path.exists():
        if final.sha(replay_path.read_bytes()) != PREVIOUS_SCRIPT_SHA:
            raise RuntimeError("Unknown predecessor installer retained; upgrade stopped.")
        originals = {"state": state_path.read_bytes(), "layout": layout_path.read_bytes(), "replay": replay_path.read_bytes()}
        for name, path in paths.items():
            if path.exists() and path.read_bytes() != originals[name]:
                raise RuntimeError("Existing comparator predecessor backup differs.")
            if not path.exists():
                repair.atomic_write(path, originals[name], 0o600)
    if not all(path.is_file() for path in paths.values()):
        raise RuntimeError("Comparator predecessor backup is incomplete.")
    backed = {name: path.read_bytes() for name, path in paths.items()}
    if final.sha(backed["replay"]) != PREVIOUS_SCRIPT_SHA:
        raise RuntimeError("Comparator predecessor script hash differs.")
    previous = checked_import(paths["replay"], PREVIOUS_SCRIPT_SHA, "comparator_predecessor001")
    previous_files = dict(previous.PAYLOADS, **{REPLAY_PATH: backed["replay"]},
                          **{name: files[name] for name in SOURCE_PATHS})
    previous_identity = {"workflow": WORKFLOW, "base": BASE, "branch": BRANCH,
                         "file_hashes": {name: final.sha(data) for name, data in previous_files.items()}}
    previous_state = json.loads(backed["state"])
    previous_layout = dict(previous_identity, origin=str(runner.origin), path=str(worktree))
    allowed = set(previous_identity) | {"phase", "git_readouts"}
    if (any(previous_state.get(key) != value for key, value in previous_identity.items())
            or set(previous_state) - allowed or previous_state.get("phase") != "pending"
            or set(previous_state.get("git_readouts", {})) != {"git-worktree-add"}
            or json.loads(backed["layout"]) != previous_layout):
        raise RuntimeError("Only the owned incomplete pre-test comparator baseline can be upgraded.")
    require_run(previous_state["git_readouts"]["git-worktree-add"])
    manifest = {"workflow": WORKFLOW, "migration": "revision002", "from_identity": previous_identity,
                "to_identity": identity, "backup_hashes": {name: final.sha(data) for name, data in backed.items()}}
    if manifest_path.exists():
        if json.loads(manifest_path.read_text()) != manifest:
            raise RuntimeError("Comparator upgrade manifest changed.")
    else:
        repair.atomic_write(manifest_path, final.encoded(manifest), 0o600)
    upgraded = dict(previous_state, **identity,
                    upgrade={"status": "in_progress", "from_script_sha256": PREVIOUS_SCRIPT_SHA,
                             "to_script_sha256": identity["file_hashes"][REPLAY_PATH], "backup_directory": str(backup)})
    complete = dict(upgraded, upgrade=dict(upgraded["upgrade"], status="complete"))
    if state not in (previous_state, upgraded, complete) or layout not in (previous_layout, new_layout):
        raise RuntimeError("Comparator partial upgrade contains unknown journal changes.")
    saved = runner.repo; runner.repo = worktree
    try:
        runner.guard_operations()
        if (worktree.is_symlink() or runner.git("rev-parse", "HEAD") != BASE
                or runner.git("symbolic-ref", "--quiet", "--short", "HEAD") != BRANCH
                or Path(runner.git("rev-parse", "--path-format=absolute", "--git-common-dir")).resolve()
                   != Path(final.bootstrap_git(runner.origin, "rev-parse", "--path-format=absolute", "--git-common-dir")).resolve()):
            raise RuntimeError("Comparator predecessor worktree identity differs.")
        if (final.names(runner, "diff", "--no-renames", "--name-only")
                or final.names(runner, "diff", "--cached", "--no-renames", "--name-only")
                or final.names(runner, "ls-files", "--others", "--exclude-standard") != set(PAYLOADS) | {REPLAY_PATH}):
            raise RuntimeError("Comparator predecessor working/index scope differs; retained.")
        baseline_previous = dict(previous_files, **{name: old[name] for name in SOURCE_PATHS}, **{RECEIPT_PATH: None})
        baseline_new = dict(files, **{name: old[name] for name in SOURCE_PATHS}, **{RECEIPT_PATH: None})
        for name in PATHS:
            path = worktree / name
            for part in [path, *path.parents]:
                if part == worktree:
                    break
                if part.is_symlink():
                    raise RuntimeError("Comparator predecessor owned path is a symlink.")
            actual = path.read_bytes() if path.exists() else None
            if actual not in {baseline_previous.get(name), baseline_new.get(name)}:
                raise RuntimeError("Unknown comparator predecessor file retained: " + name)
            if path.exists() and path.stat().st_mode & 0o7777 != (0o755 if name in EXECUTABLES else 0o644):
                raise RuntimeError("Comparator predecessor file mode differs: " + name)
        for name in set(PAYLOADS) | {REPLAY_PATH}:
            repair.atomic_write(worktree / name, files[name], 0o755 if name in EXECUTABLES else 0o644)
        runner.save_state(state_path, upgraded)
        runner.save_state(layout_path, new_layout)
        runner.save_state(state_path, complete)
        runner.report["journal_upgrade"] = complete["upgrade"]
        return complete
    finally:
        runner.repo = saved


def run_install(origin, script_path):
    report = {"workflow": WORKFLOW, "script_revision": SCRIPT_REVISION, "outcome": "failed", "phase": "preflight", "base": BASE, "branch": BRANCH,
              "merge": "not performed at this checkpoint", "tag": "not performed at this checkpoint", "full_coverage_claimed": False}
    locks = []
    runner = final = repair = original_guard = directory = shared_lock = None
    failure = None
    try:
        dev = checked_import(origin / ENV_SCRIPT, ENV_SHA, "accepted_devshell001")
        final = dev.trusted_finalizer(origin)
        if (Path(final.bootstrap_git(origin, "rev-parse", "--show-toplevel")).resolve() != origin
                or final.bootstrap_git(origin, "rev-parse", "HEAD") != BASE
                or final.bootstrap_git(origin, "symbolic-ref", "--quiet", "--short", "HEAD") != "master"):
            raise RuntimeError("--repo must identify the original master at the accepted environment checkpoint.")
        common = Path(final.bootstrap_git(origin, "rev-parse", "--path-format=absolute", "--git-common-dir"))
        metadata = common / "codex-repairs" / WORKFLOW
        metadata.mkdir(parents=True, exist_ok=True)
        shared_lock_path = common / "codex-repairs" / "leveldb-resource-tests001" / "run.lock"
        shared_lock_path.parent.mkdir(parents=True, exist_ok=True)
        for path in (common / "codex-repairs" / final.REPAIR / "run.lock",
                     common / "codex-repairs" / dev.WORKFLOW / "run.lock", metadata / "run.lock", shared_lock_path):
            if path.is_symlink():
                raise RuntimeError("Workflow lock is a symlink.")
            lock = path.open("a+"); locks.append(lock); fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        shared_lock = locks[-1]
        repair, runner = final.trusted_coverage(origin).load_repair_runner(origin, origin)
        runner.env["GIT_OPTIONAL_LOCKS"] = "0"
        directory = metadata / "runs" / (dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-" + uuid.uuid4().hex[:8])
        directory.mkdir(parents=True)
        runner.directory, runner.log, runner.report = directory, (directory / "run.log").open("w", buffering=1), report
        report.update(origin_repository=str(origin), run_directory=str(directory), cargo_target_directory=str(origin / "target-comparator"))
        final.environment_guard(runner); runner.guard_operations()
        report["cache_exclusion"] = prepare_comparator_cache(origin, common, origin / "target-comparator",
            directory / "cache-exclusion", lambda *args: runner.capture(["git", "-c", "core.fsmonitor=false", *args], cwd=origin, check=False))
        if set(PAYLOADS) != {GUARD_PATH, ROUTINE_PATH, REGRESSION_PATH, DOCUMENT_PATH} or final.sha(PAYLOADS[GUARD_PATH]) != GUARD_SHA:
            raise RuntimeError("Reviewed comparator payload set differs.")
        if (origin / "shell.nix").read_bytes() != final.blob(runner, BASE, "shell.nix"):
            raise RuntimeError("Accepted native shell changed.")
        old = {name: final.blob(runner, BASE, name, False) for name in PATHS}
        if any(old[name] is not None for name in PATHS - SOURCE_PATHS):
            raise RuntimeError("A new comparator path already exists at the reviewed base.")
        files = dict(PAYLOADS, **{REPLAY_PATH: script_path.read_bytes()})
        report["source_guard"] = []
        for name in sorted(SOURCE_PATHS):
            if old[name] is None:
                raise RuntimeError("Comparator source is missing: " + name)
            patched, observation = comparator_source_plan(old[name], repair.rust_tokens)
            observations = [observation]
            if observation.get("matched") != "old":
                raise RuntimeError("Comparator baseline must contain its reviewed unimplemented function: " + name)
            files[name] = patched
            report["source_guard"].append({"path": name, "baseline_sha256": final.sha(old[name]), "patched_sha256": final.sha(files[name]), "observations": observations})
        before = final.userdata(runner, PATHS)
        original_index = final.fingerprint(Path(runner.git("rev-parse", "--path-format=absolute", "--git-path", "index")))
        snapshot_tag = runner.git("rev-parse", "refs/tags/" + final.TAG)
        prior_randomized = runner.capture(["git", "rev-parse", "--verify", "refs/heads/repair/leveldb-randomized-bounded"], check=False)
        randomized_head = prior_randomized.stdout.decode().strip() if prior_randomized.returncode == 0 else None
        inputs = final.integration_inputs(runner, BASE)
        report["userdata_before_sha256"] = final.sha(final.encoded(before))
        def original_guard():
            saved = runner.repo; runner.repo = origin
            try:
                runner.guard_operations()
                if (runner.git("symbolic-ref", "--quiet", "--short", "HEAD") != "master" or runner.git("rev-parse", "HEAD") != BASE
                        or final.userdata(runner, PATHS) != before
                        or final.fingerprint(Path(runner.git("rev-parse", "--path-format=absolute", "--git-path", "index"))) != original_index):
                    raise RuntimeError("Original branch/index/user-owned state changed; retained.")
                if (final.names(runner, "diff", "--no-renames", "--name-only", BASE) | final.names(runner, "diff", "--cached", "--no-renames", "--name-only", BASE)) & inputs.keys():
                    raise RuntimeError("Original Rust/build inputs changed.")
                if runner.git("rev-parse", "refs/tags/" + final.TAG) != snapshot_tag:
                    raise RuntimeError("Previous snapshot tag changed.")
                if randomized_head is not None and runner.git("rev-parse", "refs/heads/repair/leveldb-randomized-bounded") != randomized_head:
                    raise RuntimeError("Recorded randomized branch changed.")
            finally:
                runner.repo = saved
        original_guard()
        identity = {"workflow": WORKFLOW, "base": BASE, "branch": BRANCH, "file_hashes": {name: final.sha(data) for name, data in files.items()}}
        state_path = metadata / "state.json"
        layout_path = metadata / "worktree.json"
        if state_path.is_symlink() or layout_path.is_symlink():
            raise RuntimeError("Comparator ownership journal is a symlink.")
        state = json.loads(state_path.read_text()) if state_path.exists() else dict(identity, phase="pending")
        worktree = origin.parent / (origin.name + WORKTREE_SUFFIX)
        if state_path.exists() and layout_path.exists():
            state = migrate_pending_baseline(runner, final, repair, metadata, state_path, layout_path,
                                             state, identity, files, old, worktree, script_path)
        if any(state.get(key) != value for key, value in identity.items()) or state.get("blocked_checkpoint"):
            raise RuntimeError("Comparator checkpoint journal identity changed; retained.")
        files[RECEIPT_PATH] = final.encoded(state["receipt"]) if state.get("receipt") is not None else None
        worktree = origin.parent / (origin.name + WORKTREE_SUFFIX)
        if worktree.is_symlink():
            raise RuntimeError("Comparator worktree path is a symlink.")
        layout = dict(identity, origin=str(origin), path=str(worktree))
        branch = runner.capture(["git", "rev-parse", "--verify", "refs/heads/" + BRANCH], check=False)
        if layout_path.exists():
            if json.loads(layout_path.read_text()) != layout:
                raise RuntimeError("Comparator worktree ownership differs.")
        elif worktree.exists() or branch.returncode == 0:
            raise RuntimeError("Existing comparator branch/worktree has no matching ownership journal.")
        else:
            runner.save_state(layout_path, layout)
        runner.save_state(state_path, state)
        boot = directory / "resource_guard.py"
        repair.atomic_write(boot, PAYLOADS[GUARD_PATH], 0o755)
        guard_module = checked_import(boot, GUARD_SHA, "resource_guard001")
        guard = guard_module.Guard(directory / "git-guard", boot, runner.env)
        preflight = guard.preflight(); report["git_preflight"] = preflight; require_run(preflight)
        def guarded_git(label, command, cwd):
            original_guard(); report["phase"] = label
            result = guard.run(label, command, cwd, {}, kind="build")
            report.setdefault("git_readouts", {})[label] = result
            state.setdefault("git_readouts", {})[label] = result
            runner.save_state(state_path, state); require_run(result)
            return result
        if ("worktree " + str(worktree) + "\n") not in final.bootstrap_git(origin, "worktree", "list", "--porcelain"):
            if worktree.exists() or (branch.returncode == 0 and branch.stdout.decode().strip() != BASE):
                raise RuntimeError("Comparator worktree setup has unknown existing state.")
            guarded_git("git-worktree-add", ["git", "worktree", "add"] + (["-b", BRANCH, str(worktree), BASE] if branch.returncode else [str(worktree), BRANCH]), origin)
        if Path(final.bootstrap_git(worktree, "rev-parse", "--path-format=absolute", "--git-common-dir")).resolve() != common.resolve():
            raise RuntimeError("Comparator worktree belongs to another repository.")
        runner.repo = worktree
        checkpoint = state.get("checkpoint")
        head = runner.git("rev-parse", "HEAD")
        if head != BASE:
            if state.get("receipt") is None or checkpoint not in (None, head) or not own_commit(runner, final, head, files):
                raise RuntimeError("Comparator branch contains an unowned checkpoint.")
            require_run(state.get("git_readouts", {}).get("git-commit", {}))
            validate_result(state["receipt"]["verification"])
            checkpoint = state["checkpoint"] = head
        elif checkpoint is not None:
            raise RuntimeError("Recorded comparator checkpoint no longer matches the branch.")
        worker_guard(runner, final, checkpoint or BASE, files, old, not checkpoint, initial=not checkpoint)
        baseline_files = dict(files, **{name: old[name] for name in SOURCE_PATHS})
        current_files = files if checkpoint else baseline_files
        if not checkpoint:
            for name, data in current_files.items():
                if data is not None:
                    repair.atomic_write(worktree / name, data, 0o755 if name in EXECUTABLES else 0o644)
        routine = checked_import(worktree / ROUTINE_PATH, final.sha(PAYLOADS[ROUTINE_PATH]), "routine_comparator001")
        def before_phase(label):
            report["phase"] = label; runner.say("Comparator phase: " + label)
            original_guard(); worker_guard(runner, final, checkpoint or BASE, current_files, old, not checkpoint, staged_files=files)
        def verify(label, expected_failures=frozenset()):
            fcntl.flock(shared_lock, fcntl.LOCK_UN)
            try:
                result = routine.verify(worktree, origin / "target-comparator", directory / label, before_phase=before_phase, expected_failures=expected_failures)
            except BaseException:
                evidence = directory / label / "VERIFICATION_RETURN.json"
                if evidence.is_file() and evidence.stat().st_size <= 2 * 1024 * 1024:
                    report[label] = json.loads(evidence.read_text())
                raise
            report[label] = result
            fcntl.flock(shared_lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            return result
        if not checkpoint:
            baseline = verify("baseline", REGRESSIONS); validate_result(baseline, True)
            state["baseline"] = baseline; runner.save_state(state_path, state)
            original_guard(); worker_guard(runner, final, BASE, current_files, old, True, staged_files=files)
            for name in SOURCE_PATHS:
                repair.atomic_write(worktree / name, files[name], 0o644)
            current_files = files
        verification = verify("verification"); validate_result(verification)
        original_guard(); worker_guard(runner, final, checkpoint or BASE, files, old, not checkpoint)
        if not checkpoint:
            receipt = {"workflow": WORKFLOW, "base": BASE, "file_hashes": identity["file_hashes"], "baseline": state["baseline"], "verification": verification, "full_coverage_claimed": False}
            state.update(receipt=receipt, phase="validated"); runner.save_state(state_path, state)
            files[RECEIPT_PATH] = final.encoded(receipt)
            repair.atomic_write(worktree / RECEIPT_PATH, files[RECEIPT_PATH], 0o644)
            for name in sorted(PATHS):
                runner.git("add", "--chmod=" + ("+x" if name in EXECUTABLES else "-x"), "--", name)
            original_guard(); worker_guard(runner, final, BASE, files, old, True)
            message = directory / "commit-message.txt"
            message.write_text("fix: return canonical bytewise comparator from mock implementations\n\n"
                               "Both mock lookup methods previously panicked. Return the program-lifetime built-in comparator while retaining each mock's comparison behavior.\n"
                               "Reproduced both TODO panics, then passed all eight comparator cases under verified resource limits.\n"
                               "Add a reusable protected verification command. No full coverage claim.\n\nRepair-Workflow: " + WORKFLOW + "\nBase: " + BASE + "\n")
            guarded_git("git-commit", ["git", "commit", "--file", str(message)], worktree)
            checkpoint = runner.git("rev-parse", "HEAD")
            if not own_commit(runner, final, checkpoint, files):
                state["blocked_checkpoint"] = checkpoint; runner.save_state(state_path, state)
                raise RuntimeError("Comparator commit exceeded its exact eight-file scope; retained.")
        original_guard(); worker_guard(runner, final, checkpoint, files, old, False)
        state.update(checkpoint=checkpoint, phase="complete"); runner.save_state(state_path, state)
        report.update(outcome="passed", phase="complete", checkpoint=checkpoint, worktree=str(worktree),
                      original_preserved=True, userdata_after_sha256=final.sha(final.encoded(final.userdata(runner, PATHS))), randomized_head_preserved=randomized_head)
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
                if original_guard:
                    original_guard(); report["original_preserved"] = True
                report.update(original_head=final.bootstrap_git(origin, "rev-parse", "HEAD"), original_status=final.bootstrap_git(origin, "status", "--short"))
            except Exception as error:
                report.update(outcome="failed", original_preserved=False, preservation_error=str(error))
            if directory is not None:
                report["return_file"] = str(directory / "COMPARATOR_RETURN.txt")
                try:
                    repair.atomic_write(Path(report["return_file"]), final.encoded(report), 0o600)
                except Exception as error:
                    report.update(outcome="failed", finalization_error=str(error))
            if runner.log:
                runner.log.close()
        for lock in reversed(locks):
            lock.close()
        print("BEGIN_COMPARATOR_RETURN"); print(json.dumps(report, indent=2)); print("END_COMPARATOR_RETURN")
        if report.get("return_file"):
            print("Return file: " + report["return_file"])
    return 0 if report["outcome"] == "passed" else 1


def main():
    parser = argparse.ArgumentParser(description="Reproduce and repair comparator mock lookup panics in an isolated worktree.")
    parser.add_argument("--repo", type=Path, default=Path.home() / "bitcoin-rs")
    args = parser.parse_args()
    return run_install(args.repo.expanduser().resolve(), Path(__file__).resolve())


if __name__ == "__main__":
    sys.exit(main())
