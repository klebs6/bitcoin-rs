#!/usr/bin/env python3
"""Measure coverage at the accepted snapshot repair commit and record the baseline."""
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

EXPECTED_COMMIT = "b81eb950afae8208a173bb132ebdde23912572c9"
REPAIR = "snapshot-diagnostic-dispatch001"
BRANCH = "repair/leveldb-snapshot-diagnostic-dispatch"
REPAIR_REPLAY = "contrib/repair/bitcoinleveldb-snapshot-dispatch001.py"
REPAIR_REPLAY_SHA256 = "454cb0e7ee58985af5a0fbecc2234dad280999b4270c5dc7caacde88d3fe9979"
COVERAGE_REPLAY = "contrib/repair/bitcoinleveldb-snapshot-coverage001.py"
COVERAGE_REPORT = "doc/repair/snapshot-diagnostic-dispatch001-coverage.json"
COVERAGE_PATHS = [COVERAGE_REPLAY, COVERAGE_REPORT]
SOURCE_ROOTS = ["bitcoinleveldb-modeldb/src", "bitcoinleveldbt-snapshot/src"]


def file_sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def clean_git_environment():
    env = dict(os.environ)
    for key in ("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_COMMON_DIR",
                "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES",
                "GIT_PREFIX", "GIT_NAMESPACE"):
        env.pop(key, None)
    env["GIT_OPTIONAL_LOCKS"] = "0"
    return env


def bootstrap_git(repo, *args):
    result = subprocess.run(["git", *args], cwd=repo, env=clean_git_environment(),
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode:
        raise RuntimeError(result.stderr.decode(errors="replace"))
    return result.stdout.decode().strip()


def load_repair_runner(worktree, origin):
    path = worktree / REPAIR_REPLAY
    if file_sha(path) != REPAIR_REPLAY_SHA256:
        raise RuntimeError("The committed repair runner differs from the accepted Revision004.")
    # The exact trusted file contains orchestration definitions, not an application entry run.
    sys.dont_write_bytecode = True
    spec = importlib.util.spec_from_file_location("accepted_snapshot_repair004", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    runner = module.Run(origin)
    runner.env = clean_git_environment()
    return module, runner


def assert_coverage_tree(runner, expected_head, allow_outputs=False):
    runner.guard_git(COVERAGE_PATHS if allow_outputs else [], expected_head)
    untracked = runner.names("ls-files", "--others", "--exclude-standard")
    if untracked - (set(COVERAGE_PATHS) if allow_outputs else set()):
        raise RuntimeError("Unexpected untracked worktree files were retained: " + repr(sorted(untracked)))
    if not runner.origin_preserved(runner.origin_snapshot()):
        raise RuntimeError("Original checkout changed during coverage; no changes were discarded.")


def source_packet(runner, paths, maximum_bytes=1024 * 1024):
    result, remaining = [], maximum_bytes
    for name in paths:
        path = runner.repo / name
        if not path.resolve().is_relative_to(runner.repo.resolve()):
            raise RuntimeError("Source report path escapes the worktree.")
        data = path.read_bytes()
        item = {"path": name, "sha256": hashlib.sha256(data).hexdigest(), "size_bytes": len(data)}
        if len(data) <= remaining:
            item["content"] = data.decode("utf-8")
            remaining -= len(data)
        else:
            item["content_omitted"] = "source packet size bound"
        result.append(item)
    return result


def run_coverage(origin, script_path):
    runner = None
    lock = None
    report = {"workflow": "snapshot-dispatch-coverage001", "outcome": "failed",
              "phase": "preflight", "tested_commit": EXPECTED_COMMIT,
              "merge": "pending", "tag": "pending", "coverage": "not measured"}
    failure = None
    try:
        origin = origin.resolve()
        if Path(bootstrap_git(origin, "rev-parse", "--show-toplevel")).resolve() != origin:
            raise RuntimeError("--repo must identify the original checkout root.")
        common = Path(bootstrap_git(origin, "rev-parse", "--path-format=absolute", "--git-common-dir"))
        metadata = common / "codex-repairs" / REPAIR
        layout = json.loads((metadata / "worktree.json").read_text())
        repair_state = json.loads((metadata / "state.json").read_text())
        if (layout.get("repair") != REPAIR or layout.get("origin") != str(origin)
                or layout.get("branch") != BRANCH or layout.get("revision") != "004"
                or layout.get("script_sha256") != REPAIR_REPLAY_SHA256
                or repair_state.get("checkpoint") != EXPECTED_COMMIT
                or repair_state.get("script_sha256") != REPAIR_REPLAY_SHA256):
            raise RuntimeError("Repair journal does not identify the accepted Revision004 checkpoint.")
        worktree = Path(layout["path"])
        registered = False
        for block in bootstrap_git(origin, "worktree", "list", "--porcelain").split("\n\n"):
            fields = dict(line.split(" ", 1) for line in block.splitlines() if " " in line)
            if fields.get("worktree") == str(worktree):
                registered = fields.get("branch") == "refs/heads/" + BRANCH
        if not registered or not worktree.is_dir() or worktree.is_symlink():
            raise RuntimeError("Accepted repair worktree is unavailable or its registration changed.")
        module, runner = load_repair_runner(worktree, origin)
        runner.repo = worktree
        lock = (metadata / "run.lock").open("a+")
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
        directory = metadata / "coverage-runs" / (stamp + "-" + uuid.uuid4().hex[:8])
        directory.mkdir(parents=True)
        runner.directory = directory
        runner.log = (directory / "run.log").open("w", buffering=1)
        runner.report = report
        report.update(origin_repository=str(origin), worktree=str(worktree),
                      origin_before=runner.origin_snapshot(), run_directory=str(directory),
                      script_sha256=file_sha(script_path))
        runner.say("Coverage at accepted commit: " + EXPECTED_COMMIT)
        state_path = metadata / "coverage-state001.json"
        state = json.loads(state_path.read_text()) if state_path.exists() else None
        if state and (state.get("script_sha256") != report["script_sha256"]
                      or state.get("tested_commit") != EXPECTED_COMMIT):
            raise RuntimeError("Existing coverage state belongs to another source or script revision.")
        if state and state.get("blocked_checkpoint"):
            raise RuntimeError("An altered coverage checkpoint requires inspection: " + state["blocked_checkpoint"])
        current_head = runner.git("rev-parse", "HEAD")
        accepted_heads = {EXPECTED_COMMIT}
        if state and state.get("checkpoint"):
            accepted_heads.add(state["checkpoint"])
        if current_head not in accepted_heads:
            # Recover only this workflow's exact just-completed commit after interrupted journaling.
            own = bool(state and state.get("validated")
                       and runner.git("show", "-s", "--format=%P", "HEAD").split() == [EXPECTED_COMMIT]
                       and runner.names("diff", "--name-only", EXPECTED_COMMIT, "HEAD") == set(COVERAGE_PATHS))
            if not own:
                raise RuntimeError("Repair branch advanced beyond the accepted source/coverage checkpoint.")
        assert_coverage_tree(runner, current_head, allow_outputs=bool(state and state.get("validated")))
        runner.git("var", "GIT_AUTHOR_IDENT")
        runner.git("var", "GIT_COMMITTER_IDENT")
        tracked = sorted(runner.names("ls-files", "--", *SOURCE_ROOTS))
        tracked = [name for name in tracked if name.endswith(".rs")]
        if not tracked:
            raise RuntimeError("No tracked Rust source files were found in the declared coverage domain.")
        manifest = {name: file_sha(worktree / name) for name in tracked}
        if state and state.get("validated"):
            if state.get("source_manifest") != manifest:
                raise RuntimeError("Source manifest differs from the previously measured code.")
            raw_path = Path(state["raw_path"])
            if file_sha(raw_path) != state["raw_sha256"]:
                raise RuntimeError("Retained raw coverage JSON differs from its validated hash.")
            measured = state["measurement"]
            runner.say("Reusing the completed measurement for unchanged source; tests were not repeated.")
        else:
            if current_head != EXPECTED_COMMIT:
                raise RuntimeError("A new measurement must begin at the accepted source commit.")
            report["phase"] = "tool_compatibility"
            nixpkgs = module.normalize_nixpkgs(repair_state["nixpkgs"])
            tools = prepare_coverage_tools(runner, nixpkgs["outPath"], directory)
            assert_coverage_tree(runner, EXPECTED_COMMIT)
            report["phase"] = "instrumented_tests"
            target = origin / "target" / "snapshot-dispatch-coverage001"
            if target.is_symlink() or target.resolve() == (origin / "target").resolve():
                raise RuntimeError("Coverage target must be its own directory beneath the build cache.")
            raw_path = directory / "llvm-coverage-full.json"
            command = coverage_command(tools, target.resolve(), raw_path.resolve())
            report.update(coverage_command=command, coverage_target=str(target),
                          raw_coverage_json=str(raw_path))
            status = runner.stream(command)
            report["instrumented_test_exit_code"] = status
            if status:
                raise RuntimeError("Instrumented tests failed; no coverage checkpoint was committed.")
            assert_coverage_tree(runner, EXPECTED_COMMIT)
            report["phase"] = "coverage_analysis"
            raw = json.loads(raw_path.read_bytes())
            summary = summarize_coverage(raw, worktree, tracked)
            measured = {"workflow": report["workflow"], "tested_commit": EXPECTED_COMMIT,
                        "source_manifest": manifest, "nixpkgs": nixpkgs,
                        "compiler": tools["compiler"], "tool_versions": tools["tool_versions"],
                        "branch_probe_passed": tools["branch_supported"],
                        "instrumented_test_exit_code": status, "coverage": summary,
                        "raw_json_sha256": file_sha(raw_path),
                        "merge": "pending", "tag": "pending"}
            state = {"workflow": report["workflow"], "script_sha256": report["script_sha256"],
                     "tested_commit": EXPECTED_COMMIT, "validated": True,
                     "source_manifest": manifest, "raw_path": str(raw_path),
                     "raw_sha256": file_sha(raw_path), "measurement": measured}
            runner.save_state(state_path, state)
        report["measurement"] = measured
        report["coverage"] = "measured; qualification pending review"
        report["phase"] = "coverage_checkpoint"
        report_bytes = (json.dumps(measured, indent=2, sort_keys=True) + "\n").encode()
        script_bytes = script_path.read_bytes()
        expected_files = [(COVERAGE_REPLAY, script_bytes), (COVERAGE_REPORT, report_bytes)]
        # Any retained partial outputs must be exactly this workflow's generated contents.
        for name, expected in expected_files:
            path = worktree / name
            if not path.resolve().is_relative_to(worktree.resolve()) or path.is_symlink():
                raise RuntimeError("Coverage output path leaves the worktree or is a symlink.")
            if path.exists() and path.read_bytes() != expected:
                raise RuntimeError("Existing coverage output differs from this workflow: " + name)
            staged = runner.capture(["git", "show", ":" + name], check=False)
            if staged.returncode == 0 and staged.stdout != expected:
                raise RuntimeError("Unexpected staged coverage output was retained: " + name)
        assert_coverage_tree(runner, current_head, allow_outputs=True)
        already_committed = all(runner.capture(["git", "show", "HEAD:" + name], check=False).stdout == expected
                                for name, expected in expected_files)
        if not already_committed:
            if current_head != EXPECTED_COMMIT:
                raise RuntimeError("Coverage checkpoint content differs; no new commit was made.")
            for name, expected in expected_files:
                module.atomic_write(worktree / name, expected, 0o755 if name == COVERAGE_REPLAY else 0o644)
            runner.git("diff", "--check")
            runner.git("add", "--", *COVERAGE_PATHS)
            assert_coverage_tree(runner, EXPECTED_COMMIT, allow_outputs=True)
            message = ("leveldb: record snapshot repair coverage baseline\n\n"
                       "Record branch-instrumented results for the two repaired crate source roots\n"
                       "with the accepted default-feature host configuration. Preserve unmapped\n"
                       "files and uncovered code in the report; merge/tag remain pending.\n\n"
                       "Tested-Commit: " + EXPECTED_COMMIT + "\n"
                       "Coverage-Workflow: snapshot-dispatch-coverage001\n"
                       "Raw-JSON-SHA256: " + state["raw_sha256"] + "\n")
            message_path = directory / "commit-message.txt"
            message_path.write_text(message)
            status = runner.stream(["git", "commit", "--only", "--file", str(message_path),
                                    "--", *COVERAGE_PATHS])
            if status:
                raise RuntimeError("Coverage commit failed; measurement retained for a retry without rebuilding.")
        checkpoint = runner.git("rev-parse", "HEAD")
        state["checkpoint"] = checkpoint
        runner.save_state(state_path, state)
        parent = runner.git("show", "-s", "--format=%P", checkpoint).split()
        correct_commit = (parent == [EXPECTED_COMMIT]
                          and runner.names("diff", "--name-only", EXPECTED_COMMIT, checkpoint) == set(COVERAGE_PATHS)
                          and all(runner.capture(["git", "show", checkpoint + ":" + name]).stdout == expected
                                  and (worktree / name).read_bytes() == expected for name, expected in expected_files))
        if not correct_commit:
            state["blocked_checkpoint"] = checkpoint
            runner.save_state(state_path, state)
            raise RuntimeError("Coverage checkpoint parent, paths or content changed; inspect before proceeding.")
        assert_coverage_tree(runner, checkpoint)
        report.update(outcome="passed", phase="complete", coverage_checkpoint=checkpoint)
        runner.say("Coverage measurement checkpoint: " + checkpoint)
    except KeyboardInterrupt:
        failure = "Interrupted; coverage work and original source were retained."
    except Exception as error:
        failure = type(error).__name__ + ": " + str(error)
        report["exception_traceback"] = traceback.format_exc().splitlines()
    finally:
        if failure:
            report.update(outcome="failed", error=failure)
        if runner and runner.log:
            if failure:
                runner.say("STOP: " + failure)
            try:
                report["origin_after"] = runner.origin_snapshot()
                report["origin_tracked_state_preserved"] = runner.origin_preserved(report["origin_after"])
                if not report["origin_tracked_state_preserved"]:
                    report.update(outcome="failed", error="Original tracked state changed; no changes discarded.")
                report["final_head"] = runner.git("rev-parse", "HEAD")
                report["git_status"] = runner.git("status", "--short")
                names = sorted(runner.names("ls-files", "--", *SOURCE_ROOTS))
                report["current_source_files"] = source_packet(runner, [name for name in names if name.endswith(".rs")])
                report["return_file"] = str(runner.directory / "COVERAGE_RETURN.txt")
                content = json.dumps(report, ensure_ascii=False, indent=2) + "\n"
                module.atomic_write(Path(report["return_file"]), content.encode())
            except Exception as final_error:
                report.update(outcome="failed", finalization_error=str(final_error))
            runner.log.close()
        if lock:
            lock.close()
        print("BEGIN_COVERAGE_RETURN")
        print(json.dumps(report, ensure_ascii=False, indent=2))
        print("END_COVERAGE_RETURN")
        if report.get("return_file"):
            print("Return file: " + report["return_file"])
    return 0 if report["outcome"] == "passed" else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.home() / "bitcoin-rs")
    args = parser.parse_args()
    return run_coverage(args.repo.expanduser().resolve(), Path(__file__).resolve())


"""Coverage-tool preparation for the accepted snapshot repair checkpoint.

This module uses only the standard library and a supplied Run-compatible runner.
It does not install global components or modify repository sources. Nix provides
temporary tools; a two-path Rust probe checks the real coverage data pipeline.
"""
import json as _coverage_json
import os as _coverage_os
from pathlib import Path as _CoveragePath
import re as _coverage_re
import shlex as _coverage_shlex


class CoverageToolError(RuntimeError):
    pass


def _coverage_capture(runner, command, *, maximum_error_chars=6000):
    runner.say("$ " + _coverage_shlex.join([str(part) for part in command]))
    result = runner.capture([str(part) for part in command], check=False)
    if result.returncode:
        diagnostic = (result.stdout + result.stderr).decode(errors="replace")
        raise CoverageToolError(
            "Coverage tool command failed (exit " + str(result.returncode) + "): "
            + _coverage_shlex.join([str(part) for part in command]) + "\n"
            + diagnostic[-maximum_error_chars:]
        )
    return result.stdout


def _coverage_llvm_major(version_text):
    match = _coverage_re.search(r"\bLLVM version\s+(\d+)(?:\.|\b)", version_text)
    if not match:
        raise CoverageToolError("Could not identify LLVM tool version: " + version_text[:1000])
    return int(match.group(1))


def _coverage_nix_shell_expression(nixpkgs_path, llvm_attribute=None):
    expression = (
        "let pkgs = import " + _coverage_json.dumps(str(nixpkgs_path)) + " {}; "
    )
    if llvm_attribute is not None:
        expression += "coverageLlvm = pkgs." + llvm_attribute + ".llvm; "
    expression += (
        "in pkgs.mkShell { nativeBuildInputs = [ pkgs.pkg-config pkgs.cargo-llvm-cov"
    )
    if llvm_attribute is not None:
        expression += " coverageLlvm"
    return expression + (
        " ]; buildInputs = [ pkgs.libevent pkgs.openssl pkgs.clang ]; "
        'LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib"; }'
    )


def _coverage_prefix(expression, llvm_cov=None, llvm_profdata=None):
    prefix = [
        "nix", "develop", "--impure", "--expr", expression, "--command", "env",
        "GIT_CONFIG_COUNT=1", "GIT_CONFIG_KEY_0=url.https://github.com/.insteadOf",
        "GIT_CONFIG_VALUE_0=git@github.com:", "CARGO_NET_GIT_FETCH_WITH_CLI=true",
        "CARGO_BUILD_JOBS=4", "RUST_BACKTRACE=1", "CARGO_LLVM_COV_SETUP=no",
    ]
    if llvm_cov is not None:
        prefix.append("LLVM_COV=" + str(llvm_cov))
    if llvm_profdata is not None:
        prefix.append("LLVM_PROFDATA=" + str(llvm_profdata))
    return prefix


def _coverage_compiler_metadata(runner, prefix):
    rustc_vv = _coverage_capture(runner, prefix + ["rustc", "-Vv"]).decode().strip()
    fields = {}
    for line in rustc_vv.splitlines():
        if ": " in line:
            key, value = line.split(": ", 1)
            fields[key] = value
    if (fields.get("release") != "1.97.0-nightly"
            or fields.get("commit-hash") != "9eb3be26b46eccea1de7448ea9cc3c1d20bb1a35"
            or fields.get("LLVM version") != "22.1.4"
            or fields.get("host") != "x86_64-unknown-linux-gnu"):
        raise CoverageToolError("Compiler differs from the validated repair toolchain:\n" + rustc_vv)
    cargo_version = _coverage_capture(runner, prefix + ["cargo", "-V"]).decode().strip()
    if not cargo_version.startswith("cargo 1.97.0-nightly (4d1f98451 "):
        raise CoverageToolError("Cargo differs from the validated repair toolchain: " + cargo_version)
    sysroot = _coverage_capture(runner, prefix + ["rustc", "--print", "sysroot"]).decode().strip()
    target_libdir = _coverage_capture(
        runner, prefix + ["rustc", "--print", "target-libdir"]
    ).decode().strip()
    if not _CoveragePath(sysroot).is_absolute() or not _CoveragePath(target_libdir).is_absolute():
        raise CoverageToolError("Compiler returned a nonabsolute sysroot or target-libdir.")
    return {"rustc_vv": rustc_vv, "cargo_version": cargo_version,
            "release": fields["release"], "commit_hash": fields["commit-hash"],
            "llvm_version": fields["LLVM version"], "host": fields["host"],
            "sysroot": sysroot, "target_libdir": target_libdir}


def prepare_coverage_tools(runner, nixpkgs_path, run_dir):
    """Prepare temporary tools and verify branch-profile compatibility.

    `runner` supplies capture(command, check=False), say(message), and report.
    If optional sysroot tools are absent, only the matching Nix LLVM-major
    package is considered. An unavailable or incompatible setup fails before
    the crate rebuild, with partial probe details retained in runner.report.
    """
    nixpkgs_path = _CoveragePath(nixpkgs_path)
    run_dir = _CoveragePath(run_dir)
    if not nixpkgs_path.is_absolute() or not nixpkgs_path.is_dir():
        raise CoverageToolError("Pinned nixpkgs source directory is unavailable: " + str(nixpkgs_path))
    if not run_dir.is_absolute():
        raise CoverageToolError("Coverage run directory must be absolute.")
    run_dir.mkdir(parents=True, exist_ok=True)
    info = {"nixpkgs_path": str(nixpkgs_path), "branch_supported": False,
            "global_component_installations": False}
    if hasattr(runner, "report"):
        runner.report["coverage_tool_probe"] = info

    expression = _coverage_nix_shell_expression(nixpkgs_path)
    prefix = _coverage_prefix(expression)
    compiler = _coverage_compiler_metadata(runner, prefix)
    info["compiler"] = compiler
    major = int(compiler["llvm_version"].split(".", 1)[0])
    rustlib_bin = _CoveragePath(compiler["target_libdir"]).parent / "bin"
    llvm_cov = rustlib_bin / "llvm-cov"
    llvm_profdata = rustlib_bin / "llvm-profdata"
    if (llvm_cov.is_file() and llvm_profdata.is_file()
            and _coverage_os.access(llvm_cov, _coverage_os.X_OK)
            and _coverage_os.access(llvm_profdata, _coverage_os.X_OK)):
        info["tool_source"] = "rustc-sysroot"
    else:
        attribute = "llvmPackages_" + str(major)
        probe = (
            "let pkgs = import " + _coverage_json.dumps(str(nixpkgs_path)) + " {}; "
            "in { available = builtins.hasAttr " + _coverage_json.dumps(attribute)
            + " pkgs && builtins.hasAttr \"llvm\" (builtins.getAttr "
            + _coverage_json.dumps(attribute) + " pkgs); }"
        )
        available = _coverage_json.loads(_coverage_capture(
            runner, ["nix", "eval", "--impure", "--json", "--expr", probe]
        ))
        if not isinstance(available, dict) or available.get("available") is not True:
            raise CoverageToolError(
                "Matching sysroot LLVM tools are absent and pinned nixpkgs does not provide "
                + attribute + ".llvm. No global component was installed."
            )
        path_expression = (
            "let pkgs = import " + _coverage_json.dumps(str(nixpkgs_path))
            + " {}; in pkgs." + attribute + ".llvm.outPath"
        )
        llvm_path = _coverage_capture(
            runner, ["nix", "eval", "--impure", "--raw", "--expr", path_expression]
        ).decode().strip()
        if not _CoveragePath(llvm_path).is_absolute():
            raise CoverageToolError("Nix returned a nonabsolute LLVM package path.")
        llvm_cov = _CoveragePath(llvm_path) / "bin" / "llvm-cov"
        llvm_profdata = _CoveragePath(llvm_path) / "bin" / "llvm-profdata"
        expression = _coverage_nix_shell_expression(nixpkgs_path, attribute)
        info["tool_source"] = "nixpkgs." + attribute + ".llvm"
    prefix = _coverage_prefix(expression, llvm_cov, llvm_profdata)
    final_compiler = _coverage_compiler_metadata(runner, prefix)
    if final_compiler != compiler:
        raise CoverageToolError("Temporary coverage tools changed the selected compiler/Cargo.")

    versions = {
        "cargo_llvm_cov": _coverage_capture(runner, prefix + ["cargo", "llvm-cov", "--version"]).decode().strip(),
        "llvm_cov": _coverage_capture(runner, prefix + [str(llvm_cov), "--version"]).decode().strip(),
        "llvm_profdata": _coverage_capture(runner, prefix + [str(llvm_profdata), "--version"]).decode().strip(),
    }
    if (_coverage_llvm_major(versions["llvm_cov"]) != major
            or _coverage_llvm_major(versions["llvm_profdata"]) != major):
        raise CoverageToolError("Coverage LLVM tools do not match the compiler's LLVM major.")
    info.update(prefix=prefix, llvm_cov=str(llvm_cov), llvm_profdata=str(llvm_profdata),
                tool_versions=versions)
    help_text = _coverage_capture(runner, prefix + ["cargo", "llvm-cov", "--help"]).decode()
    required = ("--branch", "--json", "--output-path", "--no-cfg-coverage",
                "--no-cfg-coverage-nightly", "--locked")
    missing = [flag for flag in required if flag not in help_text]
    if missing:
        raise CoverageToolError("Pinned cargo-llvm-cov lacks required options: " + ", ".join(missing))

    smoke = run_dir / "tool-smoke"
    smoke.mkdir(exist_ok=False)
    source = smoke / "coverage_probe.rs"
    binary = smoke / "coverage_probe"
    source.write_text(
        '#[inline(never)]\n'
        'fn coverage_choice(flag: bool) -> u32 {\n'
        '    if flag { 17 } else { 23 }\n'
        '}\n'
        'fn main() {\n'
        '    let flag = matches!(std::env::args().nth(1), Some(value) if value == "yes");\n'
        '    std::process::exit((coverage_choice(flag) == 0) as i32);\n'
        '}\n'
    )
    _coverage_capture(runner, prefix + [
        "rustc", "--edition=2021", "--crate-name", "coverage_probe",
        "-C", "instrument-coverage", "-Z", "coverage-options=branch",
        "-C", "opt-level=0", str(source), "-o", str(binary),
    ])
    profile_pattern = smoke / "probe-%p-%m.profraw"
    for argument in ("yes", "no"):
        _coverage_capture(runner, prefix + [
            "LLVM_PROFILE_FILE=" + str(profile_pattern), str(binary), argument,
        ])
    profiles = sorted(smoke.glob("probe-*.profraw"))
    if not profiles:
        raise CoverageToolError("Instrumented smoke runs emitted no raw coverage profiles.")
    merged = smoke / "probe.profdata"
    _coverage_capture(runner, prefix + [
        str(llvm_profdata), "merge", "-sparse", *[str(path) for path in profiles],
        "-o", str(merged),
    ])
    raw_json = _coverage_capture(runner, prefix + [
        str(llvm_cov), "export", str(binary), "--instr-profile=" + str(merged),
        "--format=text",
    ])
    export_path = smoke / "probe.json"
    export_path.write_bytes(raw_json)
    exported = _coverage_json.loads(raw_json)
    files = [file for block in exported.get("data", []) for file in block.get("files", [])]
    own = [file for file in files if _CoveragePath(file.get("filename", "")).resolve() == source.resolve()]
    if len(own) != 1:
        raise CoverageToolError("Smoke export lacks one unambiguous source-file coverage record.")
    summary = own[0].get("summary", {})
    branches = summary.get("branches", {})
    choice_readouts = [record for record in own[0].get("branches", [])
                       if isinstance(record, list) and len(record) >= 6 and record[0] == 3]
    if (branches.get("count", 0) < 2 or branches.get("covered", 0) < 2
            or not any(isinstance(record[4], (int, float))
                       and isinstance(record[5], (int, float))
                       and record[4] > 0 and record[5] > 0 for record in choice_readouts)):
        raise CoverageToolError("Smoke export does not demonstrate both outcomes of the known branch.")
    info.update(branch_supported=True, smoke_json=str(export_path),
                smoke_summary=summary, smoke_choice_branch_readouts=choice_readouts,
                smoke_profile_count=len(profiles))
    if hasattr(runner, "origin"):
        info["ordinary_target_dir"] = str((_CoveragePath(runner.origin) / "target").resolve())
    return info


def coverage_command(info, targetdir, rawjson):
    """Build the two-package command using a dedicated coverage target directory."""
    if info.get("branch_supported") is not True:
        raise CoverageToolError("Branch compatibility probe did not pass; crate coverage was not started.")
    targetdir, rawjson = _CoveragePath(targetdir), _CoveragePath(rawjson)
    if not targetdir.is_absolute() or not rawjson.is_absolute():
        raise CoverageToolError("Coverage target and JSON report paths must be absolute.")
    ordinary = info.get("ordinary_target_dir")
    if ordinary and targetdir.resolve() == _CoveragePath(ordinary).resolve():
        raise CoverageToolError("Coverage target must not be the ordinary Cargo target directory.")
    return [*info["prefix"], "CARGO_LLVM_COV_TARGET_DIR=" + str(targetdir),
            "CARGO_TARGET_DIR=" + str(targetdir), "CARGO_INCREMENTAL=0",
            "cargo", "llvm-cov", "--branch", "--no-cfg-coverage",
            "--no-cfg-coverage-nightly", "--locked", "-p", "bitcoinleveldb-modeldb",
            "-p", "bitcoinleveldbt-snapshot", "--json", "--output-path", str(rawjson),
            "--", "--test-threads=1"]


"""Summarize a full LLVM export without broadening the declared source domain.

The helper reads source for a conservative exclusion audit. It never writes
files, removes unmapped sources, or interprets zero counts as measured coverage.
"""
import hashlib
from pathlib import Path
import re


class CoverageReportError(ValueError):
    pass


COVERAGE_ROOTS = ("bitcoinleveldb-modeldb/src", "bitcoinleveldbt-snapshot/src")
COVERAGE_METRICS = ("lines", "functions", "regions", "branches")
COVERAGE_DETAIL_LIMIT = 1000


def _coverage_domain_path(filename, worktree):
    if not isinstance(filename, str) or not filename:
        raise CoverageReportError("Coverage filename must be a nonempty string.")
    path = Path(filename)
    path = (path if path.is_absolute() else worktree / path).resolve()
    try:
        relative = path.relative_to(worktree).as_posix()
    except ValueError:
        return None
    if relative.endswith(".rs") and any(relative.startswith(root + "/") for root in COVERAGE_ROOTS):
        return relative
    return None


def _coverage_integer(value, label):
    if isinstance(value, bool) or not isinstance(value, int) or value < 0:
        raise CoverageReportError(label + " must be a nonnegative integer.")
    return value


def _coverage_metric(value, label):
    if not isinstance(value, dict):
        raise CoverageReportError(label + " summary is missing or is not an object.")
    count = _coverage_integer(value.get("count"), label + ".count")
    covered = _coverage_integer(value.get("covered"), label + ".covered")
    if covered > count:
        raise CoverageReportError(label + ".covered exceeds .count.")
    return {"count": count, "covered": covered, "uncovered": count - covered,
            "percent": 100.0 * covered / count if count else None}


def _coverage_source_audit(worktree, paths):
    findings, errors, hashes = [], [], {}
    attrs = re.compile(r"#\s*\[\s*(?:cfg|cfg_attr)\s*\([\s\S]{0,4000}?\)\s*\]")
    off = re.compile(r"\bcoverage\s*\(\s*off\s*\)|\bno_coverage\b")
    conditional = re.compile(r"\b(?:coverage|coverage_nightly|tarpaulin|tarpaulin_include)\b")
    for path in paths:
        try:
            source_path = worktree / path
            if not source_path.resolve().is_relative_to(worktree):
                raise CoverageReportError("Source leaves the worktree.")
            data = source_path.read_bytes()
            hashes[path] = hashlib.sha256(data).hexdigest()
            source = data.decode("utf-8")
        except (OSError, UnicodeError, CoverageReportError) as error:
            errors.append({"path": path, "error": str(error)})
            continue
        candidates = [(m.start(), m.end(), "coverage_off") for m in off.finditer(source)]
        candidates += [(m.start(), m.end(), "coverage_specific_cfg") for m in attrs.finditer(source)
                       if conditional.search(m.group())]
        for start, end, kind in candidates:
            findings.append({"path": path, "line": source.count("\n", 0, start) + 1,
                             "kind": kind, "text": source[start:end][:500],
                             "text_truncated": end - start > 500,
                             "assessment": "potential exclusion or coverage-specific configuration; review required"})
    return findings, errors, hashes


def summarize_coverage(raw_json, worktree, tracked_files):
    """Return exact per-file totals, missing mappings, and bounded gap details.

    Inputs are a full LLVM JSON export, the actual source worktree, and Git's
    complete tracked file inventory. Coverage percentages cover mapped code in
    the selected default-feature host configuration, not all feature combinations.
    """
    worktree = Path(worktree).resolve()
    if not isinstance(raw_json, dict) or not isinstance(raw_json.get("data"), list) or not raw_json["data"]:
        raise CoverageReportError("Full LLVM JSON export requires a nonempty data array.")
    if raw_json.get("type", "llvm.coverage.json.export") != "llvm.coverage.json.export":
        raise CoverageReportError("Unexpected LLVM export type.")
    if not isinstance(tracked_files, (list, tuple)) or any(not isinstance(p, str) for p in tracked_files):
        raise CoverageReportError("Tracked inventory must be a sequence of path strings.")
    tracked = set()
    for filename in tracked_files:
        # Git's inventory is lexical: a tracked symlink must remain inventoried
        # even if its target later fails the source-boundary audit.
        path = Path(filename)
        if path.is_absolute():
            try:
                path = path.relative_to(worktree)
            except ValueError:
                continue
        if ".." in path.parts:
            raise CoverageReportError("Tracked inventory contains a parent path: " + filename)
        relative = path.as_posix()
        if relative.endswith(".rs") and any(relative.startswith(root + "/") for root in COVERAGE_ROOTS):
            tracked.add(relative)
    files, untracked = {}, set()
    branch_gaps, region_gaps, function_gaps = {}, {}, {}
    for datum in raw_json["data"]:
        if not isinstance(datum, dict) or not isinstance(datum.get("files"), list):
            raise CoverageReportError("Each LLVM export data item requires a files array.")
        for item in datum["files"]:
            if not isinstance(item, dict):
                raise CoverageReportError("LLVM file record must be an object.")
            relative = _coverage_domain_path(item.get("filename"), worktree)
            if relative is None:
                continue
            if relative not in tracked:
                untracked.add(relative)
                continue
            if relative in files:
                if files[relative] != item:
                    raise CoverageReportError("Conflicting coverage mappings for " + relative)
                continue
            files[relative] = item
            summary = item.get("summary")
            if not isinstance(summary, dict):
                raise CoverageReportError("Coverage summary missing for " + relative)
            for metric in COVERAGE_METRICS:
                _coverage_metric(summary.get(metric), relative + ":" + metric)
            branches = item.get("branches", [])
            if not isinstance(branches, list):
                raise CoverageReportError("Branch regions must be an array for " + relative)
            for branch in branches:
                if not isinstance(branch, list) or len(branch) < 6:
                    raise CoverageReportError("Malformed branch region for " + relative)
                coords = tuple(_coverage_integer(v, relative + ":branch coordinate") for v in branch[:4])
                true_count = _coverage_integer(branch[4], relative + ":branch true count")
                false_count = _coverage_integer(branch[5], relative + ":branch false count")
                if true_count == 0 or false_count == 0:
                    key = (relative, *coords, true_count, false_count)
                    branch_gaps[key] = {"path": relative, "start_line": coords[0], "start_column": coords[1],
                                        "end_line": coords[2], "end_column": coords[3],
                                        "true_count": true_count, "false_count": false_count,
                                        "zero_sides": [side for side, count in (("true", true_count), ("false", false_count)) if count == 0]}
        functions = datum.get("functions")
        if not isinstance(functions, list):
            raise CoverageReportError("Full export functions field must be an array.")
        for function in functions:
            if not isinstance(function, dict) or not isinstance(function.get("filenames"), list):
                raise CoverageReportError("Malformed LLVM function record.")
            names = [_coverage_domain_path(name, worktree) for name in function["filenames"]]
            untracked.update(name for name in names if name is not None and name not in tracked)
            target_names = sorted({name for name in names if name in tracked})
            if not target_names:
                continue
            name = function.get("name", "<unnamed>")
            if not isinstance(name, str):
                raise CoverageReportError("LLVM function name must be a string.")
            count = _coverage_integer(function.get("count"), name + ":function count")
            if count == 0:
                key = (name, tuple(target_names))
                function_gaps[key] = {"name": name, "paths": target_names, "count": 0}
            regions = function.get("regions", [])
            if not isinstance(regions, list):
                raise CoverageReportError("Function regions must be an array.")
            for region in regions:
                if not isinstance(region, list) or len(region) < 8:
                    raise CoverageReportError("Malformed code region in " + name)
                execution = _coverage_integer(region[4], name + ":region execution count")
                file_id = _coverage_integer(region[5], name + ":region file index")
                if file_id >= len(names):
                    raise CoverageReportError("Region file index outside function filenames in " + name)
                relative = names[file_id]
                if relative not in tracked or region[7] != 0 or execution:
                    continue
                coords = tuple(_coverage_integer(v, name + ":region coordinate") for v in region[:4])
                key = (relative, *coords)
                region_gaps[key] = {"path": relative, "start_line": coords[0], "start_column": coords[1],
                                    "end_line": coords[2], "end_column": coords[3], "count": 0}
    totals = {name: {"count": 0, "covered": 0} for name in COVERAGE_METRICS}
    per_file = []
    for relative, item in sorted(files.items()):
        metrics = {metric: _coverage_metric(item["summary"].get(metric), relative + ":" + metric)
                   for metric in COVERAGE_METRICS}
        per_file.append({"path": relative, "metrics": metrics})
        for metric, value in metrics.items():
            totals[metric]["count"] += value["count"]
            totals[metric]["covered"] += value["covered"]
    totals = {metric: _coverage_metric(value, "total:" + metric) for metric, value in totals.items()}
    exclusions, audit_errors, hashes = _coverage_source_audit(worktree, sorted(tracked))
    for item in per_file:
        item["source_sha256"] = hashes.get(item["path"])
    unmapped = sorted(tracked - files.keys())
    zero_denominators = [metric for metric, value in totals.items() if value["count"] == 0]
    mapped_full = bool(files) and not zero_denominators and all(
        value["covered"] == value["count"] for value in totals.values())
    result = {
        "scope": {"roots": list(COVERAGE_ROOTS), "inventory": "all tracked .rs files under these roots",
                  "configuration": "current host target and default features; normal cfg preserved",
                  "all_features_proof": False, "branch_instrumentation_requires_external_smoke_validation": True},
        "export": {"type": raw_json.get("type"), "version": raw_json.get("version")},
        "metrics": totals, "per_file": per_file, "tracked_source_files": sorted(tracked),
        "zero_denominator_metrics": zero_denominators,
        "mapped_source_files": sorted(files), "unmapped_source_files": unmapped,
        "untracked_mapped_sources": sorted(untracked), "exclusion_audit": exclusions,
        "source_audit_errors": audit_errors, "full_coverage_for_mapped_code": mapped_full,
        "full_coverage_candidate": bool(mapped_full and not unmapped and not untracked and not exclusions and not audit_errors),
        "zero_count_regions_note": "zero-count code regions from individual function mappings; not a replacement for merged per-file summary counts",
    }
    for label, records in (("zero_count_branches", branch_gaps), ("zero_count_regions", region_gaps),
                           ("uncovered_functions", function_gaps)):
        ordered = [records[key] for key in sorted(records)]
        result[label] = ordered[:COVERAGE_DETAIL_LIMIT]
        result[label + "_count"] = len(ordered)
        result[label + "_truncated"] = len(ordered) > COVERAGE_DETAIL_LIMIT
    return result


if __name__ == "__main__":
    sys.exit(main())
