# LevelDB generator boundary085

Parent: `14a87b550c9224bd82ae7105b52087e9c22da835`, existing
`release/leveldb-readiness062` branch. This source needs its own bounded client
qualification; previous receipts keep their original source/profile scopes.

The utility crate gains literal-byte `random_key_bytes` and
`dbtest_random_key_bytes`. The fixed alphabet is 00,01,61,62,63,64,65,fd,fe,ff.
Length selection keeps one_in(3), conditional one_in(100), skewed(10) or
uniform(10), followed by one uniform(10) draw per selected byte. The legacy String
helpers map those same draws to Unicode scalars, preserving existing valid-input
workload tapes. Existing randomized/harness text callers do not automatically
gain raw high-byte coverage; their migration remains a separate fixture gate.

random_string's printable-ASCII law already produces valid UTF-8. Its unreachable
lossy fallback becomes a checked invariant, not a claim that the old normal path
lost bytes. compressible_string now rejects a null PRNG or destination before
constructing its pattern, leaving output unchanged and returning an empty Slice.
Previously a positive-length null-PRNG call could repeatedly append an empty
pattern forever. Normal-input pattern lengths, prefixes and random draws stay
unchanged, including the minimum-one-byte pattern for a zero-length result.

Six regressions independently implement the PRNG with integer remainder and check
literal keys, fixed/conditional lengths, continuous draw streams, legacy text
mapping, printable ASCII, repeated-pattern prefixes, and finite null-input paths.
Only the utility crate runs native/doc verification. Randomized and harness
consumers compile with no-run; full stress and the whole subsystem are not rerun.

The exact installed storage080 helper/policy, shared warm033 target, incremental0,
existing profile and finite limits remain fixed. No additional target, cleanup,
budget increase, debugger or interposition is introduced. Original master, held
and retired worktrees, refs and real configured attribution remain protected.

The prior model stage083/084 qualified181 native identities,15 required regressions
and four doctest commands at their retained/fresh source boundaries. Constructor
and harness byte ownership/callers, legacy BlockIter test helpers, package contents,
licenses/notices, toolchain/features, publication order and final review remain
open. This stage does not authorize publication or claim full coverage.
