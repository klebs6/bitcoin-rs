# Controlled Issue320 tests

This change controls how `bitcoinleveldbt-issue320` database tests are launched. It does not identify the cause of the reported host freeze or change a production database or C API implementation.

All 18 existing database tests remain present, retain their assertions, and become explicitly ignored by an ordinary Cargo test run. They use ordinary `#[test]` attributes rather than automatically installing a traced-test subscriber. A new ignored `issue320_bounded_diagnostic` exercises a smaller execution profile through the original C API workload body. Every database test checks its protection before creating a database, selecting a temporary database path, or drawing random values.

Two ordinary tests, `issue320_execution_profile_contract` and `issue320_guard_predicate_rejects_unbounded_and_invalid_controls`, check profile constants and the pure limit predicate. They do not open a database, read cgroup files, or install a tracing subscriber. An ordinary Cargo run therefore does not execute any of the 19 database cases; success of those two contracts is not database-workload coverage.

## Workload profiles

| Parameter | Original `issue320_test` | `issue320_bounded_diagnostic` |
| --- | ---: | ---: |
| Loop iterations | 200,000 | 256 |
| Model slots | 10,000 | 64 |
| Snapshot slots | 100 | 8 |
| Deletion target size | 10,000 | 32 |
| C random seed | 0 | 0 |
| Generated key and value size | 1,024 bytes | 1,024 bytes |
| Delete before replacement put | false | false |
| Retain and rotate snapshots | true | true |

The original profile preserves its workload bounds, random-number calls, C API call order, status/value assertions, and successful snapshot-release, close, database-destroy and option-destroy sequence. Added counters and progress records do not consume random numbers or perform database operations. The 17 other existing database cases retain their bodies apart from an initial protection check and their test attributes.

The bounded diagnostic is a scaled workload, not a replay of the first 256 operations of the original profile. Different model and snapshot indices and a different deletion threshold can change branches and later random-number consumption. Passing it does not establish that the 200,000-iteration profile passes.

## Protection and execution

Use the controlled runner in `contrib/repair/bitcoinleveldb-issue320-controlled020.py`. Database cases are selected individually with exact names; they must not be launched as an unrestricted group of ignored tests.

Each database case requires `BITCOIN_RS_ISSUE320_GUARD=v1` and verifies its actual cgroup v2 membership and these kernel controls:

| Control | Accepted value |
| --- | --- |
| `memory.max` | finite, positive, at most 2 GiB |
| `pids.max` | finite, positive, at most 64 |
| `memory.swap.max` | 0 |
| `memory.oom.group` | 1 |
| `cpu.max` | positive finite quota and period; quota at most period |

The environment marker alone is insufficient. The Rust check inspects existing limits; it does not create them. The runner must also establish and verify wall-clock, output, per-file and aggregate test-data controls, execute one case per contained process, and terminate the entire case process group when a limit is reached. Aggregate data monitoring is a monitored allowance rather than a filesystem quota. The run receipt records the selected limits and actual guard outcomes.

The shared C API workload emits `ISSUE320_PROGRESS` around opening, selected iterations and writes, snapshot release, close and cleanup. It emits `ISSUE320_COUNTS` and `ISSUE320_CASE_OK` only after its original cleanup calls return. The original database-destroy error handling is retained, so that marker alone does not prove successful removal of every test file. A case is accepted only when its required evidence accompanies a normal successful test exit; a partial progress record is not a pass. The existing diagnostics retain their existing assertions and failure paths.

## Evidence limits

The repair runner works in its isolated candidate checkout and records exact source, executable, case-selection and preservation bindings. Its receipt states which cases actually ran and whether the original full workload ran. Test enumeration, pure contracts, the bounded workload and existing small diagnostics are distinct evidence and must not be reported as full original-workload coverage.

A resource stop, assertion failure, panic or abnormal exit does not qualify a repair and does not establish the freeze's cause. This change adds no debugger, sanitizer, race detector, allocator intervention, privilege escalation, package installation, or deliberately invalid-memory input. The randomized, `bitcoin-leveldb`, and harness crates require their own review and verification.

## Guard binding names

The guard uses `swap_limit_text` for the swap-limit text to avoid the broad `bitcoin-imports` reexports of pointer and memory `swap` functions. This binding change does not alter the required `memory.swap.max=0` kernel control or any workload. Revision002 retains the compiler-only first attempt and refreshes only the exact candidate source, replay and documentation; the existing private cache is reused, and every native case still requires fresh qualification.

Case receipts keep `ignored` as the numeric libtest outcome (zero for an executed passing case); `ignored_by_default` separately records whether explicit selection was required.
