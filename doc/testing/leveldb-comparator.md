# Guarded comparator verification

Run the eight comparator unit/integration cases through the repository command:

```sh
nix shell --offline --option max-jobs 0 --option builders '' \
  nixpkgs#python3 --command python3 \
  "$HOME/bitcoin-rs-comparator-repair/contrib/test/leveldb-comparator.py" \
  --repo "$HOME/bitcoin-rs-comparator-repair"
```

The default checkout is the checkout containing the command. `--repo` selects a
different checkout explicitly; the command never switches branches. A checkout
must contain the reviewed standalone `contrib/test/resource_guard.py`, pinned
`shell.nix`, and comparator tests. No historical installer journal is required.

Every invocation checks the native toolchain, compiles as needed, identifies
the library-unit and two integration-test executables, lists their required
tests, and runs one exact test process at a time. Two adapter unit tests and four
existing comparator integration tests run alongside two mock-accessor contracts.
Unit-test module prefixes are taken from the observed list; the two expected
function basenames must each occur exactly once. Integration names match exactly.
Previous receipts never substitute for a fresh execution.

The new contracts check that both mocks return the program-lifetime bytewise
comparator pointer. They also exercise forwarding through a borrowed adapter
without changing each mock's own comparison, name, or key-shortening behavior.
The deliberately corrupt mock remains deliberately corrupt.

The installer first adds these contracts against the old implementation. It
accepts the two specified `todo!()` panics as the baseline only when each test
exits normally with libtest status 101, exactly one failure, the matching source
filename, and `not yet implemented` in its panic output. A timeout, OOM, unrelated
assertion, ignored case, or passing old stub is not an accepted baseline. The
normal repository command exposes no option to accept failures. After repair,
all eight cases must pass; only that result can qualify the repair checkpoint.

## Resources and concurrent work

| Phase | Memory hard / soft | Deadline | Tasks | Output cap |
| --- | --- | --- | --- | --- |
| Build/native tools | 4 GiB / 3 GiB | 600 s | 128 | 2 MiB |
| Individual case/metadata | 512 MiB / 384 MiB | 30 s | 64 | 1 MiB |
| Preflight/input probe | 128 MiB / 96 MiB | 10 s | 32 | 1 MiB |

All children use verified systemd user services, one CPU's aggregate quota,
disabled swap, grouped OOM handling, whole-cgroup termination, and capped file
sizes. The controller also enforces output, test-data, disk-headroom, and external
wall-clock checks. Missing controls cause refusal before the requested command.
Resource termination is a failed verification, distinct from assertion failure.

Cargo uses `--offline --locked --jobs 1`; exact test subprocesses use
`--test-threads=1`. Nix evaluation uses local jobs zero, an empty builder list,
and substitution disabled. Cached requirements must already be available.
There is no unrestricted retry or system-configuration change.

The default Cargo cache is `target-comparator` beside the common Git directory,
separate from the randomized repair's `target` cache. The command registers only
the exact `/target-comparator/` rule in local `.git/info/exclude`, retaining the
existing rules and backing up the file before any edit. It refuses a symlink or
tracked content at that cache path. Generated Rust files in the cache therefore
remain separate from source-input checks; untracked Rust elsewhere is still
checked. `--target-dir PATH` can select an external cache. A dedicated cache avoids ordinary target-directory contention
with another repair, at the cost of compiling dependencies into that cache on
its first run. Build-directory overrides in Cargo configuration can alter that
layout; configurations are inventoried, and environment overrides require review.

A nonblocking `leveldb-resource-tests001` lock serializes cooperating comparator
verification commands in the same repository. Independent older controllers have
their own locks and budgets. Source isolation and resource limits should be used
for other concurrent builds as well.

## Reports and scope

Reports print between `BEGIN_COMPARATOR_CHECK_RETURN` and
`END_COMPARATOR_CHECK_RETURN`; the saved `VERIFICATION_RETURN.json` path follows.
Capped phase logs and reports are retained under the common Git directory at
`codex-repairs/leveldb-comparator-check001/runs/`, including failed runs.

Guarded input probes compare selected tracked/nonignored Rust, Cargo, Nix,
controller/runner files, Cargo configurations, HEAD, and index before and after
verification. A difference prevents acceptance. This is an observation across
the run, not an immutable filesystem snapshot or an inventory of every external
included asset. The routine command changes no source, index, branch, commit,
or tag; Cargo updates its selected build cache.

The result qualifies the selected comparator unit/integration cases. It does
not run the workspace-wide suite, documentation tests, randomized stress, or a
coverage measurement. Full coverage is not claimed.
