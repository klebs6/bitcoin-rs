# LevelDB Env file byte boundary076

Parent: `8ba4d91c82f342c17374966c934b918b288fd93d` on the existing
`release/leveldb-readiness062` branch. This source requires its own guarded client
result. The parent071 receipt separately records318 native passes, five successful
doctest commands with zero cases, and compilation-only harness compatibility.

## File contents and text

`read_file_to_bytes` clears a caller-owned `Vec<u8>` and appends exact fragments
from successful sequential reads. It retains the successful prefix on an open/read
error, excludes any fragment accompanying a failed read, and returns the original
file status. An open failure leaves the cleared destination empty. The existing
double-box sequential-file ownership convention and8192-byte scratch are retained.

`read_file_to_string` keeps its signature as an explicit UTF-8 text helper. It
validates the complete successful-read buffer once, allowing code points to cross
fragment boundaries. Invalid or truncated UTF-8 after an otherwise successful read
returns Corruption and leaves the string empty. No replacement characters are
inserted. An original I/O/open error takes precedence: the string receives its
successful prefix only when that entire prefix is valid UTF-8; otherwise it stays
empty. Valid literal replacement characters and embedded NULs remain valid data.

Both destinations require non-null, aligned, valid, uniquely writable pointers
for the call; null is asserted before dereference. File implementations retain
their existing Slice and lifetime contracts. The source does not claim to make
arbitrary raw-pointer use safe or repair unrelated Env threading/lifetime limits.

The captured external text-helper callers read the textual CURRENT file in
VersionSet recovery and the recovery test helper. They keep that text boundary.
A new recovery regression verifies invalid UTF-8 CURRENT is rejected as corruption
before manifest lookup. Arbitrary binary file contents have the separate byte API.
Write helpers already accept Slice; their byte format and ownership are unchanged.

## Focused qualification

Select Env and VersionSet native/doctest commands only. Require nine new regressions
and five existing file/recovery cases. Compile harness and recovery consumers with
`--no-run`; do not execute their workloads or doctests. No full randomized or
full-subsystem run is part of this repair. Prior071/070 results retain their own
source and feature-profile provenance.

The existing offline/locked/single-job observer, exact case/executable bindings,
resource guard, one-CPU/no-swap limits,2GiB disk floor and source-preservation rules
remain unchanged. Repeats adopt only the exact same candidate and audit retained
same-candidate successes before running pending checks. Failures retain compiler
diagnostics and successful earlier cases; no automatic budget increase or fallback
is permitted. The controller is ordinary readable Python with literal source.

Use the configured real Git identity on the same release branch. Preserve master,
tags, original/held/retired user work and historical receipts. No publication,
history rewriting, debugger/interposition experiment or cache cleanup is performed.
The new tests use ordinary allocated byte buffers and normal file inputs.

Retrieval outputs, lossy model/constructor/comparator/generator boundaries, packaged
consumers, license/notices, registry availability, publish order, supported
toolchain/features and final release identity remain gates in
[publication preparation](leveldb-readiness062.md).
