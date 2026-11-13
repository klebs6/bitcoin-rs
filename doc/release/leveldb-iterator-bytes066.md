# LevelDB iterator byte-owner repair066

Parent: `893eac6b825d9dfc4ac810d2278ccc85726261c1`, the exact063 patch on
`release/leveldb-readiness062`. Its065 receipt revalidated84 historical native
passes, including all11 original binary regressions, plus four doctest commands;
it ran no fresh native tests. These results belong to that parent source.

Publication remains held. This change does not identify the cause of the earlier
SparseMerge allocator abort or qualify all remaining binary-data paths.

## Source changes

- `DBIter` owns reverse keys/values and seek/skip scratch as `Vec<u8>`. Internal
  movement uses direct byte assignments and a safe local scratch owner instead
  of a mutable receiver plus a raw pointer into its saved fields. Disjoint copies
  retain capacity; overlapping source views are copied before mutation. Seek
  finishes encoding its target before clearing either saved owner, including
  when the target borrows the current reverse key or a large saved value.
- `MemTableIterator.tmp` and `encode_key` use byte vectors. The encoded seek
  target remains a varint32 byte length followed by the exact internal-key bytes.
  Encoding finishes before replacing scratch, so an input view can borrow its
  previous allocation. The returned pointer borrows scratch until mutation/drop.
- `KeyConvertingIterator::seek` uses the existing byte append helper. It still
  appends the maximum sequence/type tag and delegates synchronously. Iterator
  ownership and cleanup behavior are unchanged.
- With the captured callers migrated, the old `append_internal_key` raw sink
  now accepts `*mut Vec<u8>`. Its String-writing implementation is removed.
  The raw entrypoint snapshots the encoded input before appending, preserving a
  parsed user-key view into its previous destination allocation.
- Existing binary test helpers use byte owners too. Existing test names,
  attributes, assertions and workloads are preserved. Fourteen new regressions
  use literal wire expectations and allocated byte data: non-UTF8/NUL keys and
  values, snapshots/deletions, forward/reverse movement, direction transitions,
  self-derived seeks, varint boundaries, and raw sink/iterator ownership behavior.

The public Rust sink signatures change for `append_internal_key`, `encode_key`,
`DBIter::save_key` and `DBIter::find_next_user_entry`. The latter two require an
independent caller-owned initialized writable Vec; internal operations no longer
pass pointers into the mutable receiver. This is an explicit source API change,
not a C pointer/length ABI change. No package version or publication is selected.
Other raw Slice, comparator, iterator and allocation lifetime contracts remain.

## Verification and continuation

Controller066 checks the exact parent and byte plan, uses the configured real
Git author/committer and existing signing policy, and commits on the same release
branch. It verifies only `bitcoinleveldb-dbiter`, `bitcoinleveldb-key`,
`bitcoinleveldb-keyconvertingiterator`, `bitcoinleveldb-memtable` and
`bitcoinleveldbt-capi`, including their eligible doctests. Their changed code
requires new qualification; the parent's84 passes are not reused across this
source change. All14 new regressions must appear in the protected inventory.

A repeat application recognizes only an exact one-parent commit containing the
planned paths and byte fingerprints. It does not overwrite a moved/dirty state
or make a duplicate commit. Retained results for that exact candidate are audited
against the complete current input snapshot and original guard/log/observer
records before reuse; only missing or failed affected checks are run again.
Historical binary identities remain historical evidence even if old cache files
were later removed. Incomplete retained evidence stops for review rather than
silently triggering a blanket rerun.

Native execution keeps the permanent runner and warm Cargo target. Existing
budgets remain: one build job,4GiB build memory,512MiB ordinary cases,2GiB
doctest/stress envelopes, one CPU, no swap and finite time/output/data limits.
Original master, tags, user work and held/retired reservations remain preserved.
Failure retains the candidate and its evidence. No full subsystem or randomized
stress rerun, debugger, allocator interposition or publication is performed.

## Remaining release gates

See the [updated owner inventory](leveldb-binary-owners066.json) and
[release preparation](leveldb-readiness062.md). Coding and connected block/table/
manifest/compression buffers still need byte migration. DBGet/MemTable point
retrieval, compaction pointers, model/constructor data and reverse-comparator
paths still have unresolved textual or lossy interfaces. ModelDB point `get`
also remains separately unimplemented. Package-consumer, notices, feature/
toolchain and release-version checks remain pending.
