# LevelDB registry boundary091

This preparation replaces four inherited bare `*` constraints with exact
versions already selected in the committed root Cargo.lock:

| Dependency | Requirement |
| --- | --- |
| sqlite | =0.25.3 |
| sqlite3 | =0.24.0 |
| sqlite3-sys | =0.12.0 |
| signal-hook | =0.4.1 |

The actual registry source/checksum records are retained in the controller
plan. The original lockfile, Rust code, package versions, feature definitions,
Git dependency declarations, author fields and notices remain unchanged.
Existing native evidence retains its original source/command references;
this step creates no new native passes.

A two-dependency workspace checks registry libevent =0.1.0 and libevent-sys
=0.2.6 independently of workspace Git patches. The optional explicit online
mode permits Cargo's public registry metadata/source downloads for that
probe and its transitive dependencies. It performs no crate compilation or native tests and executes no dependency
build script or downloaded dependency program. Offline mode never silently changes to online.

If the probe resolves, a separate staging workspace copies only the 112
selected packages from their recorded Cargo source-file lists. Its root is a
checked projection of the real workspace: 112 members, actually referenced
workspace dependency aliases, the same resolver defaults and unchanged
profiles. Every member manifest/source is copied byte-for-byte. The original
lock is copied as a seed; a distinct offline metadata step may prune/derive
that staged lock before locked packaging begins.

The staging directory is outside Git. Its archives therefore omit Cargo's
optional VCS metadata. No original commit identity is fabricated inside the
archives: an explicit derivation journal binds copied source fingerprints,
the original/release commit, the projected root and lock transitions instead.
Actual archive contents and normalized dependency semantics must match that
boundary, followed by isolated registry-source resolution. All subsequent
commands stay offline.

Current development versions remain provisional. Source-specific SPDX and
attribution mapping, supported matrix, consumer compilation/API behavior,
final versions and publication approval remain release gates.
