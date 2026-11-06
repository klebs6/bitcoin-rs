# Resource-controlled randomized regression checks

Run the finite randomized contracts through the repository command:

```sh
nix shell --offline --option max-jobs 0 --option builders '' \
  nixpkgs#python3 --command python3 \
  contrib/test/leveldb-randomized.py
```

The command verifies the checkout containing the script. Before this branch is
merged, run it from the repair worktree, or select that checkout explicitly:

```sh
nix shell --offline --option max-jobs 0 --option builders '' \
  nixpkgs#python3 --command python3 \
  "$HOME/bitcoin-rs-randomized-bounded-repair/contrib/test/leveldb-randomized.py" \
  --repo "$HOME/bitcoin-rs-randomized-bounded-repair"
```

`--repo` selects the checkout to verify; it never silently switches branches or
redirects verification to another worktree. The default Cargo cache is the main
repository's `target` directory, shared by linked worktrees. `--target-dir PATH`
selects another cache. Every invocation compiles as needed, inspects the test
lists, and executes all four cases again. Previous receipts are not accepted as
substitutes for a fresh run.

The command leaves source files, branches, commits, tags, and the Git index
unchanged. It uses a nonblocking lock shared across verification of worktrees in
the same repository. A concurrent invocation fails without launching the build.

## Scope and evidence

The cases are `bounded_default`, `bounded_reuse`, `bounded_filter`, and
`bounded_uncompressed`. Each runs 34 logical steps, including writes, overwrites,
deletes, batches, iterator comparisons, saved snapshots, forced flushing,
reopening, and cleanup. Results are compared against an independent BTreeMap
oracle and the ModelDB. The names identify fixture configuration ordinals;
passing `bounded_filter` alone does not prove an active Bloom-filter path.

The command compiles the two existing long stress tests and verifies that both
are ignored. It never executes either stress workload and accepts no argument
for running them. Successful finite checks establish their declared contracts;
full stress execution and full coverage remain unmeasured.

The report prints between `BEGIN_RANDOMIZED_CHECK_RETURN` and
`END_RANDOMIZED_CHECK_RETURN`. Its path is printed afterward. Reports and capped
phase logs are stored under the common Git directory in
`codex-repairs/leveldb-randomized-check002/runs/`. Failed phases retain their
reports and logs. The selected input inventory is captured in guarded probes before and after
the run. It covers tracked and nonignored untracked Rust/Cargo/Nix files,
Cargo configurations, HEAD, and the index. A difference prevents acceptance.
It does not inventory every possible included data asset or ignored source
file, and it does not create an immutable filesystem snapshot.

## Environment and containment

The runner imports the SHA256-verified resource controller committed by the
bounded repair. It requires the accepted pinned `shell.nix` and the measured
Rust/Cargo versions. No original installer journal is needed for routine use.

Cargo runs with `--offline --locked --jobs 1` and `CARGO_NET_OFFLINE=true`.
Nix shell evaluation uses the cached pinned environment with local build jobs
zero, an empty remote-builder list, and substitution disabled. Missing cached
requirements cause failure; the runner does not fall back to an unrestricted
build or change system configuration. These controls are not a general network
sandbox for arbitrary build scripts.

Every build/test child is admitted through a systemd user service after the
parent and child verify its actual controls. If the user manager or required
cgroup-v2 controls are unavailable, execution is refused.

| Phase | Memory hard / soft | Deadline | Tasks | Output cap |
| --- | --- | --- | --- | --- |
| Build/native tools | 4 GiB / 3 GiB | 600 s | 128 | 2 MiB |
| Individual case/metadata | 512 MiB / 384 MiB | 30 s | 64 | 1 MiB |
| Preflight/input probe | 128 MiB / 96 MiB | 10 s | 32 | 1 MiB |

All phases are limited to one CPU in aggregate, have swap disabled, and terminate
the service's entire cgroup on failure. Core dumps are disabled. File-size,
test-data, disk-headroom, output, and external wall-clock checks remain active.
Soft memory-pressure events can occur during a successful build; they are not
reported as assertion failures. A resource-limit termination is a failed run.

A bare `cargo test`, or the user's existing Makefile, does not invoke this
controller. The new bounded cases refuse execution without their required
cgroup controls. Routine verification should therefore use the command above.

## Retained stress and replay limitations

The previous-boot journal records global OOM kills of bisect processes, including
one at about 27 GiB of anonymous resident memory. The exact locked tracing source
also contains an uncapped vector of formatted event strings. These are separate
verified findings: the source establishes a retention hazard, while no heap
profile attributes the entire observed process memory to those strings.

The main stress test initializes one PRNG outside its configuration loop. The
bisect test initializes a fresh PRNG for each configuration. Its full scan and
reduced prefixes share its own `execute_prefix` implementation, but this does
not establish faithful replay of the main test's later configuration streams.

A common seed also does not establish that a database failure involving
background work is repeatable or monotone in prefix length. Changing generator
sharing or replay semantics requires a separate behavior change and focused
checks; this command does not claim to repair or qualify the long-run reducer.
