# SparseMerge internal environment ownership repair 011

This candidate is based on held borrow repair e1736f59dc8e10b301cd4c0c3ea10302a9c46e1a.
It does not update master or merge the held branch. Source capture 010 binds the
reviewed old files. The earlier native allocator failure and its raw evidence
remain retained; its precise damaging instruction has not been established.

The candidate removes short-lived Env Rc owners from audited internal background
paths. Table construction, memtable rotation and manifest publication borrow the
existing EnvWrapper. Each Env method uses the existing per-target access gate;
no new gate is held across DB-mutex reacquisition. TableCache keeps its stable
Env-owning Options allocation for VersionSet, but cached SSTables receive a
field-by-field settings snapshot with env=None. Comparator/filter Arc identity,
cache pointer and all other option fields are preserved. Public Options::clone,
Table::open and owning file/table helper signatures are preserved.

The scope is these internal DB paths. Public Options.env remains Rc<RefCell<dyn
Env>>; this does not make arbitrary cross-thread sharing or every raw-pointer DB
API safe. Compaction, scheduling and shutdown waits are unchanged.

The replay/controller creates repair/leveldb-sparsemerge-ownership and a dedicated
sibling worktree. It seeds an independent regular-file Cargo cache from retained
artifacts, without hard links or incremental state. The seed is bounded at 4 GiB,
and requires its copy size plus 2 GiB free space. Existing cache bytes and the
retained failing ordinary binary are never compilation destinations.

Compilation is offline/locked, one Cargo job, under 4 GiB/600 s and one CPU. Unit and
rotation cases have 512 MiB/30 s; each of three ordinary SparseMerge runs has 2 GiB/600 s,
512 MiB test data and 4096 entries. A reviewed guard derivative extends the existing
data/entry monitor to the named stress profile. Kernel/service controls must be
verified before commands are admitted. No debugger or deliberate invalid-memory
control is used.

Validation selects ownership snapshot/cache lifecycle tests, existing table build
contracts, missing-environment contracts, manifest persistence, the 64-round
asynchronous rotation regression, and three unchanged 100000-key SparseMerge runs.
Every selected case must be listed uniquely, report exactly one passing test and
exit normally. Original SparseMerge also requires its completion marker. This is
not a full suite or coverage claim.

A qualified checkpoint is committed only in the isolated candidate branch. Any
failed or interrupted admitted native attempt blocks later automatic qualification
of the same candidate, so a passing retry cannot erase a retained failure. Build
failures and partial private caches are retained. Interrupted validation/commit
finalization is retained for review rather than reset or overwritten. Master,
other existing refs, user files, retained source and original binary are checked
again. No merge, tag, remote operation or cleanup is performed.
