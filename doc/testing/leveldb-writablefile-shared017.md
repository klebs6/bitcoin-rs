# Shared WritableFile repair017, controller revision002

The production adapter and all four Rust source/test payloads are unchanged
from original017. The adapter forwards the shared WritableFile interface and
returns an owned name snapshot after releasing its temporary RefCell borrow.
The six misplaced overlap tests remain at the production helper's owning crate;
all88assertions and the original nineteen selected cases are preserved.

Original017 successfully compiled both exact Cargo commands and produced all
four expected test executables. Its observer then rejected an explanatory
failure-note associated with E0133 warnings in a dependency, despite Cargo0,
build-finished=true and no error or ICE diagnostics. No selected native test ran.
This revision fixes that observer condition: explanatory notes remain recorded;
Cargo/build failure, primary error/ICE, incomplete protocol, wrong commands and
wrong executable inventories still block success. No diagnostic is suppressed.

Revision002 admits only that exact frozen original017 run, controller, report,
reserved journal and six-file staged candidate at the reviewed base. It first
archives the complete original journal byte for byte in a new run directory,
records the reviewed transition, and replaces only the candidate replay script
and this document. The four Rust payloads are checked byte-identical. The old
run's controller/report and predecessor015/016 reports, journals and worktrees
stay retained. A different error, changed staged tree, native pending/failure
flag, commit or partial transition blocks automatic continuation.

The existing repair/leveldb-writablefile-shared017 worktree and private Cargo
cache are reused. No new candidate branch or cache copy is made. Both exact
offline locked single-job no-run builds are rerun with the corrected observer:

- bitcoinleveldb-file: --lib --tests, producing three test executables
- bitcoinleveldb-versionsetutil: --test find_file_and_overlap, producing one

All nineteen selected native cases then run fresh:8 shared-file contracts,
2 filename regressions,3 direct file-search tests and6 relocated search/overlap
specifications. No previous test success is inferred or reused from original017.
Each exact command is bound to its Cargo-discovered executable and fingerprint;
the eight shared-file cases require their explicit completion markers. Safe
caught-panic contracts retain ordinary RefCell/unwind behavior.

Builds retain4GiB/600second limits; selected cases512MiB/30seconds, with one CPU,
swap disabled, task/file/output/data limits and the existing2GiB free-space
floor. An interrupted or failed revision002 native case stays in the journal
and blocks retry. An incomplete revision002 attempt requires review; it cannot
regain admission as the original reserved state.

One seven-file candidate commit is created only after all nineteen cases,
source/index checks, executable checks and original-state preservation pass.
Master remains unchanged. A complete unchanged qualification may be reverified
without building or executing again. This is selected coverage only; complete
workspace coverage and full randomized stress are not claimed.

The runner prints and saves WRITABLE_FILE_REPAIR_RETURN.json. After the
coordinator exits, the authorized return-buffer is replaced with the complete
output and exact return file. That late transport replacement is separate from
the coordinator's preservation comparison.
