# SparseMerge: shared environment borrow gate repair

The diagnostic001 native run reached `fill_levels`, then panicked in
`make_room_for_write` on `RefCell<dyn Env>::borrow_mut`. Its final output was
`DBImpl::drop enter: scheduled=true`; the service stopped at 600 seconds with
175 ms CPU and 20 MiB peak memory. It did not reach the bulk range or manual
SparseMerge compactions. Original source/build inputs and user checkout were
preserved; no diagnostic001 commit was created.

The raw `Options.env` borrow bypassed the per-target serialization gate that
already exists in `EnvWrapper`. Background table construction and table-cache
verification opens had equivalent bypasses. This repair routes the following
calls through wrappers with that same target identity:

- `make_room_for_write`: slowdown sleep and new log-file creation;
- `build_table`: new SST creation and deletion of failed/empty output;
- `TableCache::find_table`: primary and legacy random-access opens.

Each gate covers only one forwarded environment operation. File-handle
append/sync/close, iterator work, database mutex reacquisition and condition
variable waits remain outside the gate. Missing-environment status handling,
file ownership/error handling, scheduling and compaction algorithms remain
unchanged. The raw DB mutex can remain locked when the initiating panic unwinds;
that explains why teardown most likely blocked at its first lock. This patch
does not skip teardown or clear the scheduled flag to evade that wait.

The scope is the inspected single-database live path. Other direct environment
helpers used during concurrent database opening are not globally qualified by
this patch. No claim that all database concurrency or panic recovery is fixed
is made.

## Qualification

The runner compiles `bitcoinleveldbt-sparsemerge` with DBImpl as a normal
dependency, so its unit-test-only inline background scheduling cannot replace
the production scheduling path. It runs two exact cases in separate protected
processes:

1. `db_test_env_borrow_gate_rotation`: 64 rounds of two-key writes, forced
   memtable flushes with status checks, newest-value reads, reopen/persistence
   reads, and explicit fixture destruction. This is a scheduler-sensitive
   consumer regression, not deterministic contention injection.
2. `db_test_sparse_merge`: the original 100,000 individual puts of 1,000-byte
   values, compression disabled, `fill_levels("A", "Z")`, the original sparse
   A/B100/C updates, both explicit flushes and all three manual compactions.
   All three original overlap assertions remain `<= 20 MiB`.

Bulk keys retain `B{:010}` formatting; literal `B100` remains a distinct key.
No reduced-workload substitute or new cardinality assumption is introduced.
Both cases use ordinary `#[test]` with sparse direct stderr progress and no
buffered tracing subscriber. Their success markers follow fixture destruction.

The original case retains three ordered overlap readouts in bytes. A pass
requires exactly one passing nonignored test per selected case, exact markers,
all original overlap checks, unchanged input/index fingerprints and matching
executable identity. A Rust panic found in the complete capped log is reported
as an observed failure even if teardown later reaches a resource cutoff. The
termination reason remains separate; absent libtest counts remain unknown.

## Run after installation

```sh
nix shell --offline --option max-jobs 0 --option builders '' \
  nixpkgs#python3 --command python3 \
  "$HOME/bitcoin-rs-sparsemerge-borrow-repair/contrib/test/leveldb-sparsemerge-borrow.py"
```

The build uses one job and one aggregate CPU, zero swap, 4 GiB hard / 3 GiB soft
memory, 128 tasks, 600 seconds and bounded output/files. The targeted case uses
512 MiB hard / 384 MiB soft memory and 30 seconds. The original case uses
2 GiB hard / 1.5 GiB soft memory and 600 seconds. Both case profiles restrict
tasks, output, individual file sizes and total sampled temporary data. The
guard is byte-identical to the accepted Issue178 resource controller. Kernel
cgroup settings and live service controls must verify before child admission.
There is no unrestricted fallback or automatic budget increase.

Cargo builds offline and locked. Nix uses cached inputs with local/remote
builders and substitutes disabled. The ordinary retained Issue178 Cargo cache
is reused under the shared protected-test lock when present; otherwise the
new workflow uses its own Git-metadata cache. Fresh native case execution is
always required.

## Preservation and history

This installer creates `repair/leveldb-sparsemerge-borrow` from the accepted
native environment base. The pending diagnostic001 branch, worktree, ownership
journals, known owned files and index remain intact. It preserves original
master/index/Makefile/user files, the snapshot tag and all four existing repair
refs. Unexpected changes are retained and cause refusal.

Only the three reviewed production source files, the SparseMerge test source,
new runner/guard/replay script, documentation and receipt belong to this repair.
A commit is created only after both native cases qualify. A cutoff without a
panic is inconclusive; a panic is failed; neither creates a passing checkpoint.
Pending edits, cache and evidence are retained for investigation/retry.
No merge, tag, push, unsafe buffered baseline or workspace suite is performed.
Full coverage is not claimed.
