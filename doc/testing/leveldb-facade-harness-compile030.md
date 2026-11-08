# Facade and harness compilation repair030

The completed baseline028 run observed two unresolved reexports in the facade
and ten unresolved references to shared test argument types in the harness.
This candidate repairs those import bindings before native test execution.

The source delta is exactly two library roots:

- Remove `pub use bitcoinleveldbt_bench::*;` and
  `pub use bitcoinleveldb_meta::*;` from `bitcoin-leveldb/src/lib.rs`.
  Neither is a declared facade dependency; the captured metadata crate is
  absent. The remaining 32 reexports keep their original order.
- Add `pub use bitcoinleveldbt_util::{TestArgs, TestType};` to
  `bitcoinleveldbt-dbtest/src/lib.rs`. This explicitly exposes the existing
  owner types through DBtest, matching both the harness library and its
  integration test imports. It does not create replacement types or change
  their identity. DBtest already declares the utility dependency.

All manifests, lockfiles, harness implementation and test bodies, workloads,
oracles and execution policies are unchanged. The two unresolved facade names
did not provide a usable API in the failed baseline; the DBtest reexport is an
additive compatibility surface.

The runner pins master `2a5093e69dd91dc6d770e7e8ed1d201751f9ad5c` and the exact
uploaded baseline028 report. It preserves the original user checkout, all old
branches/worktrees/journals/reports and the baseline cache. A new admission
probe runs before any new candidate/cache reservation. A handled RAM refusal
creates no candidate journal or checkout and may be retried. Once an attempt
reserves candidate state, automatic recompilation is blocked and its evidence
is retained for review.

The new candidate uses branch `repair/leveldb-facade-harness-compile030` and an
independent worktree and cache. Optional cache bytes are copied independently
under the existing 4 GiB seed allowance, without hardlinks, incremental data or
native test executables. Each compilation has one CPU, no swap, a 4 GiB memory
cap, 3 GiB high threshold, 128 tasks, 600 seconds, a 512 MiB per-file limit and
2 MiB output allowance. Every guarded phase rechecks resource controls and
headroom. No budget is weakened to obtain admission.

The two exact commands run separately through the offline development shell:

```text
cargo test --offline --locked --jobs 1 -p bitcoin-leveldb --lib --tests --no-run --message-format=json
cargo test --offline --locked --jobs 1 -p bitcoinleveldb-harness --lib --tests --no-run --message-format=json
```

A controlled Cargo101 failure is retained, and the other crate is still
compiled. A guard/protocol failure stops the attempt. A candidate commit is
created only after both fresh compilations pass and the one facade plus two
harness executables are bound to their package, target, source, private path,
ordinary executable mode and current SHA256 fingerprints. The receipt and
controller are committed with the two source edits and this document. Master
and all existing refs remain unchanged.

`compile_qualified: true` establishes compilation only. `repair_qualified`
remains false, the test count remains zero, and no native test artifact is
executed, including `--list`. Harness runtime control and qualification follow
after reviewing the compiled result. This run does not establish the cause of
earlier workstation lockups or claim full coverage.

The outer transport replaces the authorized original `return-buffer` only
after the coordinator exits and exact printed/saved report and preservation
checks agree. Its finite transport output also includes the complete saved
`COMPILE_REPAIR_RETURN.json`.
