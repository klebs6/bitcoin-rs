// ---------------- [ File: bitcoinleveldbt-autocompaction/src/read_progress_observation.rs ]
crate::ix!();

#[derive(Debug,Clone,Getters,Builder)]
#[getset(get="pub")]
#[builder(pattern="owned")]
pub struct BitcoinLevelDbTAutoCompactionReadProgressObservation {
    /// Iteration number is 1-based and preserves the outer read-loop order.
    /// Reordering observations would invalidate temporal progress analysis.
    iteration_index: i32,
    /// This is the signed baseline size of the compacted-visible range.
    /// The sign is intentional: `ReadAll` has an empty "other" range, and the
    /// original C++ test computes signed lower bounds against zero.
    initial_visible_range_size_bytes: i64,
    /// This is the signed baseline size of the untouched range.
    /// For `ReadAll`, this is expected to remain zero.
    initial_untouched_range_size_bytes: i64,
    /// This is the signed measured size of the compacted-visible range after the
    /// iteration completed and the post-read sleep elapsed.
    visible_range_size_bytes: i64,
    /// This is the signed measured size of the untouched range after the
    /// iteration completed and the post-read sleep elapsed.
    untouched_range_size_bytes: i64,
    /// This is the parsed value of `leveldb.approximate-memory-usage`.
    /// Zero means either the DB reported zero or the property could not be parsed.
    approximate_memory_usage_bytes: u64,
    /// This is the raw `leveldb.stats` property snapshot for the same iteration.
    /// Consumers must treat it as opaque topology text.
    leveldb_stats: String,
    /// This is the raw `leveldb.sstables` property snapshot for the same iteration.
    /// Consumers must treat it as opaque topology text.
    leveldb_sstables: String,
}


#[derive(Debug,Clone,Getters,Builder)]
#[getset(get="pub")]
#[builder(pattern="owned")]
pub struct BitcoinLevelDbTAutoCompactionParameterizedFixturePreparationObservation {
    /// This is the total number of logical keys materialized into the fixture.
    /// The value must remain strictly positive so the fixture has observable SSTable topology.
    entry_count: i32,
    /// This is the payload size for each logical value in bytes.
    /// The value must remain strictly positive and large enough to cross memtable and read-sampling thresholds.
    value_size_bytes: i32,
    /// This is the number of keys belonging to the “visible” range [Key(0), Key(n)).
    /// It must satisfy 0 <= visible_key_count <= entry_count.
    visible_key_count: i32,
    /// This is the elapsed time for the initial population phase in microseconds.
    /// It is observational only and must not alter fixture semantics.
    fill_phase_elapsed_micros: u64,
    /// This is the elapsed time for the forced memtable compaction immediately after population.
    /// It is observational only and must not alter compaction semantics.
    compact_after_fill_elapsed_micros: u64,
    /// This is the elapsed time for the tombstone population phase in microseconds.
    /// It is observational only and must not alter deletion semantics.
    delete_phase_elapsed_micros: u64,
    /// This is the elapsed time for the forced memtable compaction immediately after tombstone insertion.
    /// It is observational only and must not alter compaction semantics.
    compact_after_delete_elapsed_micros: u64,
    /// This is the signed approximate size of the visible range after the tombstone fixture is prepared.
    /// Signed representation is invariant because the upstream test performs signed lower-bound arithmetic.
    initial_visible_range_size_bytes: i64,
    /// This is the signed approximate size of the untouched range after the tombstone fixture is prepared.
    /// Signed representation is invariant because the upstream test performs signed lower-bound arithmetic.
    initial_untouched_range_size_bytes: i64,
    /// This is a raw snapshot of `leveldb.approximate-memory-usage` after fixture preparation.
    /// Zero means either zero usage or an unparsable property value.
    approximate_memory_usage_bytes: u64,
    /// This is the raw `leveldb.stats` snapshot after fixture preparation.
    /// Consumers must treat it as opaque topology text.
    leveldb_stats: String,
    /// This is the raw `leveldb.sstables` snapshot after fixture preparation.
    /// Consumers must treat it as opaque topology text.
    leveldb_sstables: String,
}
