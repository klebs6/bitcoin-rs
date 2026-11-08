# Controlled randomized full-workload qualification025

This repair is bound to master 904156f818d38b443f02ff8fba6f2a2bfd5ed8a7 and
the selected current source captured by source022. It creates the isolated
branch repair/leveldb-randomized-quiet025 and its own worktree/cache.
It does not promote master. A qualified receipt is published only after all
twelve exact cases pass and original-state preservation verifies.

Run the downloaded single-file controller with cached Python:

```sh
nix shell --offline --option max-jobs 0 --option builders '' \
  nixpkgs#python3 --command python3 \
  "$HOME/Downloads/BitcoinRS-LevelDB-Randomized-QuietRepair025-20261108.py" \
  --repo "$HOME/bitcoin-rs"
```

The two original tests remain ignored by default. Their first action now
requires the runner marker, seed 301, RUST_LOG=off, one ordinary 0700 case-data
directory shared by TEST_TMPDIR/TMPDIR/TMP/TEMP, and actual cgroup controls:
memory at most 2 GiB, at most 64 tasks, swap 0, grouped OOM and at most one CPU.
Their historical operation generator, four configurations, 10,000 mutations
per configuration, mutation ordering and comparison cadence are unchanged.
Progress and full completion markers are explicit stderr records. The main
test's completion marker follows fixture teardown; the bisect marker follows
the successful return of all four complete configurations.

A test-local database snapshot lease preserves normal explicit release order
and releases an outstanding snapshot through its still-live owner on unwind.
The fixture/model outlive the lease. The lease is emptied before every reopen
and reacquires from the new database afterward. This repair's native suite does
not deliberately inject a panic into that cleanup path.

Cases run sequentially in separate guarded processes: two pure admission
contracts, four unchanged replay contracts, four unchanged 34-step bounded
cases, then db_test_randomized and db_test_bisect_randomized. Each original
performs 40,000 logical database mutation steps with seed 301. Pure operation
generation used to skip earlier configurations is additional CPU work, not
additional database mutations. Successful execution does not prove that a
background-dependent failure is repeatable or monotone under bisection.

The existing comparison oracle is retained. It compares after mutations
0,100,...,9900; the final 99 mutations have no terminal comparison. The shared
compare_iterators helper does not check iterator statuses. A passing full run
therefore establishes the declared historical workload and assertions, not
complete database correctness or exhaustive coverage. The earlier machine
freeze's cause remains unestablished by this workflow.

The Cargo no-run build is offline/locked with one job. Nix uses the retained
shell with local jobs 0, empty builders and substitution disabled. Exact Cargo
JSON records bind six test executables to package/target/source paths. Library
metadata must list zero cases; exactly the two original stress cases must be
ignored. Case command, executable bytes, guard admission, private data root,
result, exact test name and success marker are retained and reverified.

| Phase | Hard / soft memory | Deadline | Tasks | Output |
| --- | --- | --- | --- | --- |
| Build or independent cache seed |4GiB /3GiB |600s |128 |2MiB |
| Each ordinary case/list |512MiB /384MiB |30s |64 |1MiB |
| Each original stress case |2GiB /1536MiB |600s |64 |1MiB |
| Source/identity verification |512MiB /384MiB |180s |32 |1MiB |
| Guard preflight |128MiB /96MiB |10s |32 |1MiB |

All service phases have swap disabled and at most one aggregate CPU. Stress
data monitoring is capped at 512 MiB/4096 entries; ordinary cases at 128 MiB/4096.
The cache seed makes independent ordinary-file copies, excludes test binaries,
respects finite byte/file/disk allowances, and holds the source cache locks.
Missing controls or cached requirements cause failure without a fallback.

The journal records native_pending before dispatch and retains native_failure
on a failed case. An incomplete/failed attempt is not automatically retried.
An already-complete exact candidate may reverify its retained qualification;
that is reported as validation reuse, not a fresh native run. This installer
is tied to its reviewed base; later source revisions need a new qualification.

RANDOMIZED_REPAIR_RETURN.json and capped logs remain in this workflow's own
metadata directory. Only after coordinator exit and printed/saved JSON checks
does the transport replace the authorized return-buffer. Original user work,
unrelated refs/indices and earlier workflow journals must remain unchanged.


## Quiet recovery diagnostics025

This candidate is created on a new `repair/leveldb-randomized-quiet025` branch and
linked worktree from master 904156f818d38b443f02ff8fba6f2a2bfd5ed8a7. The earlier
randomized023 journal, failed candidate, reports, logs, and executable bytes remain
protected inputs. The script never resets or rewrites that journal and never
uses its ten passing cases as new qualification.

The retained Revision002 full `db_test_randomized` invocation reached database
work, then the guard stopped it with `outcome=output_limit`: 1,049,927 observed
bytes exceeded the 1,048,576-byte ceiling after about 3.87 seconds. Its output
prefix and retained rolling tail reconstruct the complete counted transcript.
The termination record is SIGTERM 15; the wrapper's exit 0 does not make the native
case pass. The full completion marker and libtest success result are absent.
The second full case was never started. This establishes the controlled output
stop only; it does not explain the historical workstation freeze or prove a
storage correctness defect.

`bitcoinleveldb-dbimpl/src/recover_log_file.rs` has duplicate direct `eprintln!`
diagnostics alongside its existing `tracing` events. This candidate removes only
those duplicate print statements. Every surrounding control-flow statement,
recovery operation, error/status return, assertion, counter, structured tracing
event, and argument outside the print statements remains byte-for-byte unchanged.
Existing tracing events preserve configurable recovery diagnostics. With the
selected `RUST_LOG=off` environment, those events remain quiet.

No guard output is filtered or discarded by an extra adapter, and no resource
limit is raised. Explicit randomized progress, completion markers, libtest
results, other diagnostic output, and error assertions remain observable. Other
unconditional prints outside this recovery method are unchanged; if they exceed
the same limit, qualification stops and retains that evidence.

The four Rust test/control payloads from randomized023 are reused byte-for-byte.
The corrected legacy `BITCOIN_RS_RANDOMIZED_GUARD=v1` admission token is supplied
to all four replay and four bounded cases. Both original 40,000-operation cases
still require their dedicated explicit stress admission token, seed 301, private
data root, one CPU, zero swap, finite task/memory limits, and verified group cleanup.

Qualification compiles six exact test executables and executes twelve fresh
selected cases: two pure admission contracts, four replay contracts, four bounded
cases, and both full original workloads. It uses an independent file-copy cache
seed from randomized023; it never hardlinks or writes the retained source cache.
Existing allocation, snapshot lifetime, oracle cadence, and generator limitations
remain as documented above. A source-only transformation check and Python/Git
fixtures do not replace the required native execution on Meerkat.

The new candidate is committed only after all twelve cases pass and all original
and retained evidence verifies. Master is never promoted by this script. A later
identical invocation may reverify an already complete025 candidate, but any new
failed, pending, or incomplete025 attempt remains blocked for separate review.
The terminal transport overwrites the authorized original `return-buffer` only
after the coordinator exits and printed/saved report identities validate.
