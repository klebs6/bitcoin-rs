# LevelDB publication preparation

Baseline: `leveldb-subsystem-qualified-20261111`, commit
`0214a455d9f49084dc093d281d4dcce42173de1e`.

**Status: publication is held pending the source and packaging checks below.**
The baseline records 107 compiled packages, 2,041 accumulated native passes and
107 successful doctest commands on the recorded Linux/default-feature setup.
Those results retain their original run/source provenance; they do not prove
all-feature coverage, arbitrary binary-data correctness, or memory safety.

## This preparation

- Review all 107 crate READMEs against captured source and manifests: 85
  `bitcoinleveldb-*` components, 21 `bitcoinleveldbt-*` test/support packages,
  and the `bitcoin-leveldb` facade. Replace placeholders and false APIs, keep
  useful crate-specific contracts and source links, and avoid unverified examples.
- Give local path dependencies explicit version requirements matching their
  existing package versions. No package version is bumped and no registry
  availability is inferred. The complete recorded local runtime/build closure
  contains 112 packages; the facade alone reaches 87, including two test-named
  support packages. Selecting all 85 component libraries plus the facade reaches
  109 local packages because some component crates aggregate test infrastructure.
  Package names alone do not define what can be omitted.
- Correct package descriptions/repository links, remove fabricated placeholder
  author entries, and remove the invalid `<none>` address from legacy author
  names. Preserve upstream author metadata and source notices. Do not assign
  authorship to a guessed identity.
- Repair Arena backing-allocation ownership: keep the exact allocation layout
  in a private owner, request the documented alignment, reject zero requests,
  and check aligned-size arithmetic. Regression coverage accompanies the repair;
  the protected targeted result is recorded separately.
- Journal retirement of clean merged repair reservations at their exact tips.
  Keep their worktree files, indexes, caches and evidence. Hold dirty worktrees.

## Remaining release gates

1. **Binary representation.** Coding, batches, keys and block paths currently
   use Rust `String` storage for binary representations, including unchecked
   UTF-8 conversion or mutable access to its byte vector. Rust `String` requires
   valid UTF-8. This is a source-level correctness blocker, including outside
   optional Snappy support; documentation cannot make the representation valid.
   Plan a byte-buffer migration with focused compatibility/round-trip tests.
2. **Known helper limits.** ModelDB's point `get` path is not implemented, and
   reverse-key comparator conversions are not a general arbitrary-byte ordering
   implementation. Keep these boundaries explicit and decide their public scope.
3. **Packaged consumer.** Inspect each intended `.crate` file, license/notices,
   examples and includes; establish registry dependency availability and publish
   order; verify a consumer outside the workspace under bounded resources.
   Workspace path resolution and version declarations are not package verification.
4. **Features/toolchain.** The current source uses nightly features. Record an
   explicit supported toolchain and test matrix; do not manufacture an MSRV or
   imply that default-feature results exercised optional features.
5. **Release identity/version.** Choose the release version and public package
   set, finish release notes, and make the final tag with the configured real
   Git identity. Do not silently move the qualification tag or publish artifacts.

The README/publication pass does not require repeating the entire native test
inventory. Run the affected Arena package for this production change; later
binary-buffer and package changes need their own targeted checks. See the
[bounded testing guide](../testing/leveldb-subsystem.md).

## Repair reservation lifecycle

The capture found 16 merged repair refs. Twelve attached worktrees were clean;
their refs can be retired only after verifying the exact tip, master ancestry,
clean status, and unchanged index. Detaching at the same commit preserves the
worktree and its data. A retirement receipt records actual changes and reflog
copies. Historical one-shot mutation-controller branch reservations are then
retired; their old branch-name preconditions should not be replayed as if active.

Four reservations have dirty/staged/untracked work and remain held:
`repair/leveldb-final-crates-runner033`,
`repair/leveldb-randomized-controlled023`,
`repair/leveldb-writablefile-shared`, and `repair/leveldb-sparsemerge`.
No worktree, cache, receipt, parked artifact, or directory is disposable merely
because its commits are merged.

## Sources and support

- [Repository and issue tracker](https://github.com/klebs6/bitcoin-rs)
- [Cargo packaging rules](https://doc.rust-lang.org/cargo/commands/cargo-package.html)
- [Rust String representation](https://doc.rust-lang.org/std/string/struct.String.html)
- [Attribution correction record](leveldb-attribution062.md)
