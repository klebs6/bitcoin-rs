# LevelDB reverse-byte comparator077

Parent: `707e3a9aadca098dc4be750caf81fa8915e4c53c` on the existing
`release/leveldb-readiness062` branch. The076 receipt separately qualified Env file
reads with136 native passes, two doctest commands with zero cases, and compilation
of harness/recovery consumers. This077 source needs its own guarded result.

## Exact ordering and helpers

`reverse(&Slice)` returns `Vec<u8>` by reversing exact input bytes. Comparison
delegates the reversed byte slices to the unchanged bytewise comparator. It
therefore distinguishes every byte key, including high-bit values and embedded
NULs. It neither reverses Unicode scalar values nor negates bytewise order.
Byte-based logging replaces the previous lossy textual debug arguments.

Separator and successor implementations remain byte-based and unchanged. They
transform to reversed space, delegate shortening and transform back. New literal
regressions verify shortening results and comparator-order bounds, including
prefix, empty and maximal-byte cases.

`increment` deliberately changes its raw destination from String to Vec<u8>. It
appends one NUL for the bytewise comparator name and prepends one NUL for this
reverse comparator, matching append-in-reversed-space semantics. It consumes the
comparator box. Other custom comparator orderings are outside its contract. The
destination still requires a non-null, valid, aligned and uniquely writable
pointer; null is asserted before dereference. All tests use ordinary allocated
byte buffers and valid borrowed slices.

## Persistent identity

The corrected component uses `bitcoinrs.ReverseBytewiseComparator.BytesV1`, with
no legacy-name alias. Its actual binary-key ordering differs from the former
lossy implementation named `leveldb.ReverseBytewiseComparator`. Options requires
the same name and exact ordering on reopen; VersionSet recovery rejects a
persisted-name mismatch. Retaining the old name would conceal that incompatibility.
The independent harness comparator and its name are untouched.

An existing database using this component's old name must be recreated from
original logical records under the new comparator. Rewriting the manifest name
does not convert its ordering or recover keys previously collapsed by lossy
comparison. This repair performs no database migration or data-file modification.
See the upstream [comparator reopen contract](https://github.com/google/leveldb/blob/main/include/leveldb/options.h).

## Focused qualification

Only bitcoinleveldb-reversekeycomparator native/doctest commands are selected.
Require10 new integration regressions and the two existing ASCII cases. Captured
Rust sources show no external reverse/increment callers, and captured workspace
manifests show no local consumer dependency on this crate. No consumer or harness
workload, full subsystem or randomized suite is compiled/run for this isolated
repair. That source review does not establish compatibility with unknown external
users; these are intentional prepublication Rust API and comparator-identity
changes. Dependency/lockfile/package-version metadata remains unchanged.

The existing offline/locked/single-job observer, exact case/executable bindings,
finite resource guard, one-CPU/no-swap limits,2GiB floor and preservation rules
are unchanged. Repeated application adopts only the exact candidate and audits
same-candidate successes before pending checks. No budgets are raised, no
unrestricted fallback is attempted, and no prior result is relabeled as077.

Use the configured real Git identity on the same release branch. Preserve master,
tags, original/held/retired user work and historical receipts. No publication,
history rewriting, debugger/interposition experiment or cache cleanup is performed.
The controller is readable Python with literal source and uses return-buffer.

Retrieval outputs, model/constructor/generator data, remaining lossy text call
sites, packaged consumers, license/notices, registry order, versions and supported
toolchain/features remain gates in [publication preparation](leveldb-readiness062.md).
