# LevelDB constructor/harness byte boundary086

Parent: `98365a0dc251301955e5bb596e163bb737583a32` on the existing
`release/leveldb-readiness062` branch. This source requires its own bounded client
qualification. Earlier results retain their recorded source/profile scopes.

The common Constructor owns a scoped BTreeMap of byte vectors and its comparator.
Insertion coalesces comparator-equivalent keys while preserving the first key and
last value. Byte finalization returns comparator-ordered keys plus a copied map;
text compatibility views validate the whole map before changing outputs/state.
The global unrelated text KVMap alias and legacy text signatures are retained.

Block and MemTable constructors share Arc<Box<dyn SliceComparator>> ownership
with their base. Moving the Box handle into an Arc does not move its comparator
allocation. Existing borrowed comparator addresses remain stable. Table/DB
constructor paths retain their existing owner structure. Matching comparator
names still require the caller's identical-order contract; names alone prove no
semantic equivalence. Direct byte maps with equivalent duplicate keys are rejected
before an existing block/table/memtable/database is replaced.

All four concrete constructors expose finish_impl_bytes. Table source ownership
and Block byte backing survive their iterators; these are borrowing iterators,
not the independent ModelDB iterator lifetime. Harness byte checks dispose of
iterators before refinalization and compare structured key/value bytes. Existing
text harness random choices are unchanged; formatted strings no longer act as
its equality oracle. New byte tests are opt-in and leave legacy stress tapes
separate. BlockIter's six unchecked UTF-8 fixture conversions become byte
assertions; its production prefixes are unchanged.

Native/doc verification selects blockconstructor, tableconstructor, memtable,
dbconstructor and blockiter only. Harness and randomized consumers compile with
no-run. A separate exact four-case harness target then runs block/table/memtable/DB
fixtures in both comparator orders with at most ten live entries, full256byte
values, NUL/empty/high-byte keys, seeking, rebuilding and legacy compatibility.
No original ignored/long harness or full randomized workload is executed. Its
results are reported separately from the five full affected-crate inventories.
Same-candidate retries audit preserved inputs, physical command/guard/output and
artifact bindings before retaining completed cases; no fabricated native seed is
used to skip changed original workloads.

The exact080storage helper/policy, shared warm033cache, incremental0, current
profile and finite resource budgets remain fixed. Original master, tags, held and
retired worktrees and real configured attribution are preserved. No cleanup,
profile/budget increase, debugger or interposition is introduced.

085qualified six utility regressions and one doctest command; harness/randomized
consumers compiled. Raw-byte generators remain opt-in; their legacy text workload
interfaces are explicitly distinct. Remaining publication checks include package
contents, licenses/notices, consumer examples, supported toolchain/features,
versions/publication order and final review. No universal memory-safety or full
stress/full-subsystem coverage claim is made.
