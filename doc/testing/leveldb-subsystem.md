# Guarded LevelDB subsystem tests

From the repository root, run:

```sh
nix shell --offline --option max-jobs 0 --option builders '' \
  nixpkgs#python3 --command python3 bitcoin-leveldb/check.py
```

This explicit entry point discovers every current workspace member whose
package name starts with `bitcoinleveldb-` or `bitcoinleveldbt-`, together with
`bitcoin-leveldb`. Newly added matching crates are included automatically.
It does not invoke nested Cargo builds from a Rust unit test.

To select exact package names, repeat `--crate`:

```sh
python3 bitcoin-leveldb/check.py --crate bitcoinleveldb-table \
  --crate bitcoinleveldb-comparator
```

The runner builds each selected package through an offline, locked,
single-job Cargo invocation in the repository's development shell. It binds
Cargo's JSON executable records to the selected package, target source and
current executable bytes. Each executable is listed twice to discover all
tests and the ignored subset. Tests then run individually by exact name with
one test thread. Ignored tests are explicitly executed. The repaired
randomized, issue320 and harness cases retain their existing admission
environment and exact completion markers. The five original stress cases
run last, after all other selected packages and doctests.

Doctests run separately, including ignored doctests. Custom `harness=false`
targets run once as their own command; the report records their command exit
status and does not invent an internal test count. Zero-test executables and
packages without eligible native targets are reported explicitly.

The scope is the current host and Cargo's default CLI feature selection.
Actual resolved package features and inactive required-feature targets are
reported. This is not an all-features or cross-platform matrix. Bench target
metadata and benchmark entries are retained as explicit exclusions; timed
benchmarks are not run. Passing this command establishes success within this
declared scope, not universal coverage.

Every native operation runs inside a verified systemd user service. There is
no unrestricted fallback. Limits include one CPU, zero swap, finite tasks,
runtime, output and file size, a memory admission margin and a 2 GiB free-disk
floor. Builds have 4 GiB / 600 seconds. Routine tests have 512 MiB / 30 seconds
and 128 MiB private data. Ignored tests, custom harnesses and original stress
cases have 2 GiB / 600 seconds and 512 MiB private data. Doctests, including
rustdoc compilation, use that same 2 GiB stress envelope. Data directories are
private; there are no automatic budget increases or retries.

Current source, runner files, index and ref inventories are compared before
and after each package and around the final original stress cases. A source
change or unavailable controls stops the run. A normal compile/test failure
is retained and the runner continues to collect results from other packages
while admission remains available. Compiler errors, failed-case tails and
bounded relevant failed-package sources travel in the report so the next
repair can proceed without a separate collection run. Collection omissions
and finite allowance stops remain explicit.

Only private run metadata, test data and the Cargo cache are written by the
permanent runner. It makes no source changes or Git ref updates. It obtains
the repository repair workflow locks and lets Cargo acquire its own cache
locks; it does not hold Cargo's locks while spawning Cargo. It reuses the
existing final-crates runner cache when present, avoiding a duplicate large
cache. On another checkout it creates its own private cache. No data/cache
cleanup is automatic.

Reports live under `.git/codex-repairs/leveldb-subsystem055/runs/`. The main
record is `SUBSYSTEM_RETURN.json`; complete guard records, logs, Cargo
evidence, source fingerprints and input comparisons remain beside it.
`outcome: passed` requires a completed package inventory, successful fresh
compilation, every discovered test executed and passed, successful eligible
doctests/custom commands, and preserved inputs. Failed or incomplete runs
must not be presented as perfect success.

The permanent runner uses the ordinary sibling `leveldb-final-crates.py` for
the repaired-case catalog and source capture, and the pinned ordinary
`leveldb_resource_guard.py` for resource controls. It does not import
historical repair controllers. The one-time installer creates a separate
tooling branch and commit, verifies the current accepted base, and preserves
the original working tree and existing staged runner candidate. It does not
promote its new branch to master.
