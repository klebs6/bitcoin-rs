# Resource-controlled harness qualification031

The parent compile030 candidate compiled the facade and harness without running
native tests. This follow-up retains those two import fixes and changes only
the harness integration-test execution controls.

The fourteen original `harness.rs` tests use ordinary `#[test]` attributes.
Only `harness_randomized` and `harness_randomized_longdb` are ignored by default.
Every original case requires the real031 admission gate before executing its
body. It verifies the exact case and profile, seed 301, `RUST_LOG=off`, one private
mode 0700 temporary directory across `TMPDIR`, `TMP`, `TEMP`, `TEST_TMPDIR` and
the explicit case-data token, actual unified cgroup limits, and the process's
core/file-size limits. Environment tokens alone do not establish admission.

Ordinary cases require 512 MiB memory, 384 MiB memory high, 64 tasks, no swap, grouped
OOM handling, at most one CPU, no core dumps and 16 MiB soft/hard file-size limits.
The two heavy cases require 2 GiB memory, 1536 MiB memory high, 64 tasks and 256 MiB
soft/hard file-size limits with the same CPU/swap/OOM/core policy. The external
guard independently verifies runtime limits of 30 seconds for ordinary cases,
600 seconds for heavy cases, and cleanup; the Rust gate does not infer a timeout from environment
variables. Existing build budgets remain unchanged.

Each original body is retained inside a lexical scope. Its locals, including
the harness and iterators, finish normal teardown before its unique completion
marker. Assertion failures and blocked destructors cannot emit that marker.
Tagged progress statements consume no random numbers and perform no database
operations. The randomized case adds at most 256 progress records; longdb adds
15. Both retain the original mutation and oracle order.

All 16 argument configurations remain unchanged: six table, six block, two
memtable and two database configurations. The randomized entry-count schedule
still runs below 2000 with its original increments. Longdb still inserts 100000
entries and checks the original seven-level file-count postcondition. No seed
stream, key/value generator, workload bound, comparison or assertion is reduced.

Two new pure admission contracts check accepted controls and reject missing or
weakened limits, wrong case/profile tokens, logging/seed changes and inconsistent
temporary paths. They construct no database or environment worker.

The runner builds fresh facade and harness artifacts, inspects all/ignored
metadata, then runs the two pure cases, twelve original ordinary cases and two
original heavy cases sequentially, one exact test in each guarded process.
Its fresh empty private Cargo target avoids duplicating an optional
4 GiB predecessor cache; predecessor030 cache and artifacts stay unchanged.
The existing guard's 2 GiB disk-free floor remains active for every phase.

Qualification requires all 16 passing assessments and exact completion markers,
unchanged source/workload bindings, and preservation of existing user work and
prior evidence. Fresh compilation alone remains distinct from runtime case
qualification. Neither a historical workstation-freeze cause nor full LevelDB
coverage is claimed. The authorized `return-buffer` replacement occurs only
after the coordinator exits and exact printed/saved return validation succeeds.
