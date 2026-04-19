# Proposed harness fix for version_get_probe comparator fidelity

## Summary

The `t_version_get_visibility_specifications` failures are consistent with the temporary harness creating SSTables with `BytewiseComparatorImpl` while `Version` / `VersionSet` metadata logic uses `InternalKeyComparator`. That makes `Version::for_each_overlapping` correct, but `Table::internal_get` seek inside each file incorrect for tombstones and snapshot cutoffs.

## Minimal patch

Mirror the comparator/filter-policy portion of `DBImpl::new` inside `VersionSetTemporaryDatabaseHarness::open_temporary_database_with_flags`.

### Structural change

Add `internal_filter_policy` to the harness so the internal-facing filter policy lives as long as the options / table cache / versionset.

```rust

```

### Constructor change

If you can import `sanitize_options`, use this version:

```rust
pub fn open_temporary_database_with_flags(
    test_prefix:          &str,
    create_if_missing:    bool,
    error_if_exists:      bool,
    table_cache_capacity: i32,
) -> Self {
    // ... directory setup unchanged ...
}
```

### Lower-risk fallback if `sanitize_options` is awkward to import

If `sanitize_options` is difficult to wire into the test harness right now, the immediate correctness patch is to rebind the harness options to the internal-facing comparator and filter policy before constructing `TableCache` and `VersionSet`.

```rust
let mut database_options = Box::new(Options::with_env(environment));
database_options.set_create_if_missing(create_if_missing);
database_options.set_error_if_exists(error_if_exists);

let internal_key_comparator = Box::new(
    InternalKeyComparator::new(database_options.comparator().as_ref())
);

let internal_filter_policy = Box::new(
    InternalFilterPolicy::new(database_options.filter_policy().as_ref())
);

// These setters may already exist in your Options impl; if they do, this is
// the smallest harness-only correction.
database_options.set_comparator(internal_key_comparator.as_ref());
database_options.set_filter_policy(internal_filter_policy.as_ref());

let mut table_cache = Box::new(TableCache::new(
    &database_name,
    database_options.as_ref(),
    table_cache_capacity,
));

let version_set = Some(VersionSet::new(
    &database_name,
    database_options.as_ref() as *const Options,
    table_cache.as_mut() as *mut TableCache,
    internal_key_comparator.as_ref() as *const InternalKeyComparator,
));
```

## Guard test to add immediately

This turns the current trace finding into a permanent tripwire.

```rust

```

## Expected re-run result

After the harness patch, the following four tests should flip green together:

- `version_get_probe_level_zero_newest_tombstone_masks_older_value_for_same_user_key`
- `version_get_probe_returns_not_found_when_newer_shallower_tombstone_layout_is_monotone`
- `version_get_probe_returns_not_found_when_stale_shallower_tombstone_masks_newer_deeper_value`
- `version_get_probe_snapshot_cutoff_returns_old_value_before_newer_deeper_sequence_is_visible`

## Next branch once green

Only after those pass should the next tranche focus on where compaction manufactures the inverted layout:

1. end-to-end `do_compaction_work` layout witness
2. `install_compaction_results` monotonicity witness
3. output-split witness around `should_stop_before`
4. snapshot-preserving stale-read witness over real compaction, not synthetic metadata injection
