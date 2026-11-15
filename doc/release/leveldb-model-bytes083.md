# ModelDB byte boundary083

Parent: `68fbdc3be8dbaa94d690667da32698d22feefd65` on the existing
`release/leveldb-readiness062` branch. This source needs its own protected client
qualification. Earlier receipts retain their original source/profile scopes.

ModelDB now owns a BTreeMap of byte vectors. Storage order is internal: the
configured comparator controls logical key equivalence and visible order. Equal
replacement keeps the first stored key, matching the reference std::map model.
Lookup and equivalent-key writes/deletes scan this small reference map.

Point reads implement DBGet::get_bytes and inherit its checked text adapter.
Missing/error reads preserve caller output. Binary snapshot reads use an optional
typed Snapshot byte-map capability. Foreign snapshots fail explicitly. Iterators
copy all rows and retain only the comparator Arc via a closure, so they borrow
neither snapshot payload nor the environment-bearing Options.

ModelSnapshot retains new_from_map and a strict optional text cache. The legacy
map_ref is valid only for completely UTF-8 maps and otherwise fails explicitly;
byte_map_ref preserves arbitrary keys and values. Snapshot cloning always copies
the authoritative byte map. The separate legacy ModelIter API is retained, with
bytewise lower_bound over text keys and an arbitrary raw-byte seek target.

LevelDBSnapshot keeps its existing representation/first shadow field. It creates
a fresh SnapshotImpl from the inner sequence capability rather than copying
another snapshot object's layout or list pointers. Inner unsupported/model
snapshots keep their actual optional capabilities; shadow zero is not promoted
to a fabricated sequence capability. Byte-map access forwards to the inner object.
These are Rust C-style handles; this stage does not add C ABI exports.

Qualification runs native/doc checks only for modeldb, snapshot, db and the
snapshot bridge test crate. Randomized and harness consumers compile with no-run;
no full randomized stress or whole-subsystem rerun is claimed. Required cases
cover distinct non-UTF-8 keys, all256 values, NUL/empty data, batch order,
bidirectional seek, comparator equivalence/reverse-order helpers, snapshot clones
and bridge lifetime, foreign snapshots, and an ordinary padded sequence snapshot.
The helper comparator tests exercise the exact lookup/handler/iterator functions;
they do not claim exhaustive custom-comparator validation.

The installed storage080 helper/policy, single warm033 target, incremental0,
existing test profile and finite resource budgets remain fixed. No cache cleanup,
new target, profile rebuild, debugger, interposition or security-sensitive exercise
is introduced. Source/receipt pins, real Git attribution, held/retired worktrees,
original master and configured signing policy retain their existing boundaries.

Remaining publication gates include other legacy text fixture/generator owners,
package contents and licenses/notices, consumer examples, supported toolchain and
feature scope, version/publication ordering and final release review. This stage
is not a whole-subsystem requalification or publication authorization.
