# LevelDB package preparation089

This stage changes README prose, five package descriptions and package-local
legal text selection. It changes no Rust source, feature, dependency, version,
build script, lockfile, author field or license expression. Existing native
qualification records retain their original source boundaries; no native tests
are rerun here.

Every one of the 112 packages in inventory088 receives:

- LICENSE: an exact copy of the captured repository MIT notice.
- LICENSE.leveldb: the complete notice and conditions from
  https://github.com/google/leveldb/blob/main/LICENSE .
- LICENSE.crc32c: the complete notice and conditions from
  https://github.com/google/crc32c/blob/main/LICENSE .
- NOTICE: provenance, scope and the remaining attribution/SPDX review boundary.

The upstream texts were checked on 2026-11-15; line wrapping/blank lines are
normalized. Content hashes for these local copies are recorded in the089
controller plan and resulting source inventory. The notices are bundled across
the dependency closure for retention, not as an assertion that every file
derives from both upstream projects or that an upstream holder authored the port.

The five shared READMEs now describe the actual setup helper, compatibility
attributes, broad prelude, CRC32C surface and allocation/event stand-ins.
The facade/readiness text refers to completed source repairs without presenting
the remaining distribution work as complete.

Verification consists of bounded local source/delta checks and one guarded,
offline112-package Cargo file listing. Each expected legal file and README must
be selected from its exact current source bytes. This is not archive or detached
consumer verification. Registry/fork compatibility, final versions, source-level
licensing/SPDX review and the supported release matrix remain separate gates.
