# LevelDB packaging cycle093

The092 Cargo command completed76 archive constructions before lock generation
for `bitcoinleveldb-dbimpl` failed: its versioned development dependency names
`bitcoinleveldb-dbconstructor`, whose runtime dependency points back to DBImpl.
The other seven local development edges do not form cycles. The runtime/build
publication graph remains acyclic.

The repair removes only the registry version requirement from that one local
path development dependency. The local path and all other dependency semantics
remain unchanged. Cargo intentionally omits an unversioned path dev-dependency
from the normalized published manifest. Fixture-dependent tests remain supported
through the repository workflow; running them from a standalone archive is not
qualified. No Rust source, package version, feature, or original lock changes.

Cargo writes in-progress archives under `target/package/tmp-crate`, copies
completed archives to `tmp-registry`, and uplifts final archives only after the
whole batch succeeds. These directories are mutable tool staging. The092
wrapper incorrectly treated tmp-crate files as immutable prior outputs and
collected only final-root archives; that failure and the raw process logs are
retained. Seventy-six log messages are observations, not archive verification.

The093 controller inventories and atomically retains the whole exact admitted
package staging tree. It verifies the76 previously completed archives against
their recorded source-selection and normalization contracts before attempting
new packaging. It retains the prior092 relocation and all historical evidence.

One fresh112-package archive-only batch uses the same warm target and resource
limits. Cargo resets its temporary registry per batch, so a partial36-package
retry would require a separate registry overlay. Reassembly is explicitly
reported; no compilation, dependency build scripts, native tests, network,
publication, master/tag/ref promotion, or new build cache is authorized here.
Regenerated archives are fully inspected and compared with retained archives;
compressed-byte differences are recorded separately from source equivalence.

The derived source workspace is outside Git. Explicit source fingerprints bind
it to the release candidate; no Git attribution is manufactured inside archives.
The earlier derived lock is copied and must pass an offline locked metadata
check. Independent consumer resolution is an additional metadata gate and does
not establish compilation, Git-fork API equivalence, or full feature/platform
coverage. SPDX/source-specific attribution and final registry versions remain
separate release gates.
