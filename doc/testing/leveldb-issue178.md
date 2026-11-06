# Issue178: bounded original-scale regression

The original issue178 regression uses `NUM_KEYS = 1_100_000`, constructs two
whole insertion batches and one whole deletion batch, manually compacts the
original unpadded key range, and scans the surviving first range.

Its former `#[traced_test]` wrapper selected an event-buffering subscriber.
The inspected locked tracing-setup 1.0.4 source stores formatted events in an
uncapped `Vec<String>` and installs no environment filter. The original key
helpers alone emit more than eleven million TRACE events during preparation.
This is a concrete resource defect in the test harness. It does not establish
that any particular native allocation, SIGKILL or stall came solely from it.

The repair uses ordinary `#[test]` functions, installs no tracing subscriber,
and emits sparse direct stderr progress. Both tests use the existing
`serial_test` lock so ordinary Cargo invocation cannot run them concurrently.
Database implementation code is
unchanged. The original workload, three complete batches, key spellings,
compression-off setting and compaction endpoints remain intact. The original
end key `my_key_1099999` is not the lexical maximum of the key set; changing it
or padding the suffixes would change the regression scenario.

The scan additionally checks strict key advancement, exact original key
domain and values, cardinality, and final iterator status. It fails on excess
or repeated keys instead of looping indefinitely. A blocked iterator/API call
is still bounded by the external service guard.

## Run

After installation, from an ordinary shell without custom Rust build overrides:

```sh
nix shell --offline --option max-jobs 0 --option builders '' \
  nixpkgs#python3 --command python3 \
  "$HOME/bitcoin-rs-issue178-repair/contrib/test/leveldb-issue178.py"
```

The default command runs two fresh exact cases in separate native processes:

| Case | Keys | Meaning |
| --- | ---: | --- |
| `issue178_bounded_diagnostic` | 1,024 | C-API and iterator smoke; does not qualify the original compaction scale |
| `issue178_test` | 1,100,000 | Original-scale regression with strengthened survivor checks |

`--diagnostic-only` runs just the small case and explicitly reports that the
original-scale case has not completed. Its pass does not qualify the full
regression. A single passing-case completion marker and the native test
summary are both required; an empty filter cannot pass verification.

## Finite execution policy

| Phase | Hard memory | Soft memory | Time | Tasks | File size | Test data |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Build/native tool check | 4 GiB | 3 GiB | 600 s | 128 | 512 MiB | Build cache monitored for free space |
| Small case | 512 MiB | 384 MiB | 30 s | 64 | 16 MiB | 128 MiB / 4,096 entries |
| Original case | 2 GiB | 1.5 GiB | 600 s | 64 | 256 MiB | 512 MiB / 4,096 entries |

Every admitted child has a one-CPU aggregate quota, zero permitted swap,
group OOM termination, a three-second stop timeout and no automatic restart.
The runner verifies kernel cgroup settings and live service controls before
permitting execution. Cargo builds offline/locked with one build job. Nix may
use cached inputs but cannot launch local or remote builders or substitutes.
Output, progress records, test-data growth and disk headroom are bounded.
Test-data limits are sampled checks and can briefly overshoot between samples.
There is no unrestricted fallback or automatic budget escalation.

The progress file contains at most 256 records / 64 KiB; every preparation and
scan loop reports at most once per 65,536 keys plus phase boundaries. The
guard output log is capped at 1 MiB per case. Completion requires cleanup to
return, so a close/destroy stall cannot be mistaken for a successful scan.

All generated Cargo artifacts live beneath the common Git directory's owned
`codex-repairs` metadata, outside source worktrees. Source/build inputs and
the Git index are captured before and after verification. Reports retain the
selected executable fingerprint, qualified test names, actual controls,
resource readouts and last progress phase. Builds and cases are not reused as
validation from an earlier invocation.

## Outcome and history

The installer preserves original master, its index, Makefile and user files;
it also guards the existing comparator/randomized branch refs and snapshot
tag. It works in `repair/leveldb-issue178` and commits only after both cases
pass with unchanged recorded inputs. No merge, tag or push is performed.

A timeout, memory limit, data limit or output limit is an **inconclusive bounded
observation**. It neither proves an engine hang nor qualifies a repair. The
pending worktree, source changes, cache and evidence remain available for the
next investigation. The original unsafe buffered baseline is not rerun; its
source remains available in the parent commit. Full crate/workspace coverage
and sparsemerge qualification are not claimed.
