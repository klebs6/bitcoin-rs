// ---------------- [ File: bitcoinleveldbt-autocompaction/src/bitcoinleveldbt_autocompaction.rs ]
crate::ix!();

use tracing_subscriber::EnvFilter;

/// This selector preserves caller intent for test-log topology.
/// If the process provides an explicit `RUST_LOG` filter and it parses
/// successfully, the test runtime must not widen it to global `trace`.
/// Widening an explicit filter destroys the wall-clock observability of
/// long-running storage tests and can turn bounded compaction loops into
/// apparent hangs through log amplification alone.
pub fn bitcoinleveldb_dbimpl_test_runtime_env_filter() -> EnvFilter {
    trace!(
        target: "bitcoinleveldb_dbimpl::test_hooks",
        event = "bitcoinleveldb_dbimpl_test_runtime_env_filter_entry",
        label = "bitcoinleveldb_dbimpl.test_runtime_env_filter.entry"
    );

    let runtime_env_filter = match EnvFilter::try_from_default_env() {
        Ok(explicit_env_filter) => {
            debug!(
                target: "bitcoinleveldb_dbimpl::test_hooks",
                event = "bitcoinleveldb_dbimpl_test_runtime_env_filter_decision",
                label = "bitcoinleveldb_dbimpl.test_runtime_env_filter.use_explicit_env",
                rust_log_explicit = true
            );

            explicit_env_filter
        }
        Err(explicit_env_filter_error) => {
            warn!(
                target: "bitcoinleveldb_dbimpl::test_hooks",
                event = "bitcoinleveldb_dbimpl_test_runtime_env_filter_decision",
                label = "bitcoinleveldb_dbimpl.test_runtime_env_filter.fallback_default_trace",
                rust_log_explicit = false,
                explicit_env_filter_error = ?explicit_env_filter_error
            );

            let trace_directive_result = "trace".parse();

            match trace_directive_result {
                Ok(trace_directive) => EnvFilter::default().add_directive(trace_directive),
                Err(trace_directive_parse_error) => {
                    error!(
                        target: "bitcoinleveldb_dbimpl::test_hooks",
                        event = "bitcoinleveldb_dbimpl_test_runtime_env_filter_trace_directive_parse_failed",
                        label = "bitcoinleveldb_dbimpl.test_runtime_env_filter.trace_directive_parse_failed",
                        trace_directive_parse_error = ?trace_directive_parse_error
                    );

                    EnvFilter::default()
                }
            }
        }
    };

    trace!(
        target: "bitcoinleveldb_dbimpl::test_hooks",
        event = "bitcoinleveldb_dbimpl_test_runtime_env_filter_exit",
        label = "bitcoinleveldb_dbimpl.test_runtime_env_filter.exit"
    );

    runtime_env_filter
}

pub fn init_tracing_for_tests() {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let (writer, guard) = tracing_appender::non_blocking(std::io::stderr());

        let runtime_env_filter = bitcoinleveldb_dbimpl_test_runtime_env_filter();

        tracing_subscriber::fmt()
            .with_env_filter(runtime_env_filter)
            .with_writer(writer)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .init();
    });
}

/// This wait primitive must never borrow the shared database `Env`.
/// The autocompaction harness opens the DB on a `PosixEnv` instance that
/// background compaction threads reach through `EnvWrapper`'s serialized
/// `RefCell` access gate. Direct harness borrows of that same `RefCell` during
/// the wait window can force those threads into borrow failure and convert
/// compaction progress into apparent hangs.
pub fn bitcoinleveldbt_autocompaction_sleep_for_microseconds_without_shared_env_borrow(
    micros: i32,
) {
    trace!(
        target: "bitcoinleveldbt_autocompaction::autocompact_test",
        event = "bitcoinleveldbt_autocompaction_sleep_for_microseconds_without_shared_env_borrow_entry",
        label = "bitcoinleveldbt_autocompaction.sleep_for_microseconds_without_shared_env_borrow.entry",
        micros = micros
    );

    match micros <= 0i32 {
        true => {
            debug!(
                target: "bitcoinleveldbt_autocompaction::autocompact_test",
                event = "bitcoinleveldbt_autocompaction_sleep_for_microseconds_without_shared_env_borrow_noop",
                label = "bitcoinleveldbt_autocompaction.sleep_for_microseconds_without_shared_env_borrow.noop",
                micros = micros
            );
        }
        false => {
            let sleep_duration = Duration::from_micros(micros as u64);

            debug!(
                target: "bitcoinleveldbt_autocompaction::autocompact_test",
                event = "bitcoinleveldbt_autocompaction_sleep_for_microseconds_without_shared_env_borrow_state_transition",
                label = "bitcoinleveldbt_autocompaction.sleep_for_microseconds_without_shared_env_borrow.state_transition",
                micros = micros
            );

            thread::sleep(sleep_duration);
        }
    }

    trace!(
        target: "bitcoinleveldbt_autocompaction::autocompact_test",
        event = "bitcoinleveldbt_autocompaction_sleep_for_microseconds_without_shared_env_borrow_exit",
        label = "bitcoinleveldbt_autocompaction.sleep_for_microseconds_without_shared_env_borrow.exit",
        micros = micros
    );
}

/// This timing primitive exists solely to observe wall-clock elapsed time in the
/// harness without borrowing the database `Env`. Observation must remain outside
/// the DB's internal concurrency topology.
pub fn bitcoinleveldbt_autocompaction_elapsed_micros_from_instant(
    started_at: Instant,
) -> u64 {
    trace!(
        target: "bitcoinleveldbt_autocompaction::autocompact_test",
        event = "bitcoinleveldbt_autocompaction_elapsed_micros_from_instant_entry",
        label = "bitcoinleveldbt_autocompaction.elapsed_micros_from_instant.entry"
    );

    let elapsed_micros_u128 = started_at.elapsed().as_micros();

    let elapsed_micros_u64 = match elapsed_micros_u128 > (u64::MAX as u128) {
        true => {
            warn!(
                target: "bitcoinleveldbt_autocompaction::autocompact_test",
                event = "bitcoinleveldbt_autocompaction_elapsed_micros_from_instant_saturated",
                label = "bitcoinleveldbt_autocompaction.elapsed_micros_from_instant.saturated",
                elapsed_micros_u128 = elapsed_micros_u128
            );
            u64::MAX
        }
        false => elapsed_micros_u128 as u64,
    };

    trace!(
        target: "bitcoinleveldbt_autocompaction::autocompact_test",
        event = "bitcoinleveldbt_autocompaction_elapsed_micros_from_instant_exit",
        label = "bitcoinleveldbt_autocompaction.elapsed_micros_from_instant.exit",
        elapsed_micros_u64 = elapsed_micros_u64
    );

    elapsed_micros_u64
}


//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/autocompact_test.cc]

pub struct AutoCompactTest {
    dbname:     String,
    tiny_cache: *mut Cache,
    options:    Options,
    db:         *mut dyn DB,
}

impl Default for AutoCompactTest {
    fn default() -> Self {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_default_entry"
        );

        let dbname = unique_db_path("/autocompact_test");
        let tiny_cache = new_lru_cache(100usize);

        let mut options = Options::default();
        options.set_env(Some(posix_default_env()));
        options.set_block_cache(tiny_cache);

        let destroy_status = destroydb(&dbname, &options);
        assert!(destroy_status.is_ok());

        options.set_create_if_missing(true);
        options.set_compression(CompressionType::None);

        let mut db: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        let mut opener = DBImpl::new(&options, &dbname);
        let open_status = opener.open(
            &options,
            &dbname,
            (&mut db) as *mut *mut dyn DB,
        );
        assert!(open_status.is_ok());

        let out = Self {
            dbname,
            tiny_cache,
            options,
            db,
        };

        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_default_exit",
            db_is_null = out.db.is_null()
        );

        out
    }
}

impl Drop for AutoCompactTest {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_drop_entry",
            db_is_null = self.db.is_null(),
            cache_is_null = self.tiny_cache.is_null()
        );

        if !self.db.is_null() {
            unsafe {
                drop(Box::from_raw(self.db));
            }
            self.db = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        }

        let _ = destroydb(&self.dbname, &Options::default());

        if !self.tiny_cache.is_null() {
            unsafe {
                drop(Box::from_raw(self.tiny_cache));
            }
            self.tiny_cache = core::ptr::null_mut();
        }

        debug!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_drop_exit"
        );
    }
}

impl AutoCompactTest {
    /// This preserves the original key encoding invariant.
    /// The zero-padded decimal suffix must remain lexicographically sortable.
    pub fn key(&mut self, i: i32) -> String {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_key_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.key.entry",
            i = i
        );

        let out = format!("key{:06}", i);

        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_key_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.key.exit",
            key_len = out.len()
        );

        out
    }

    /// This preserves the exact approximate-size query boundary semantics used by
    /// the original test harness.
    pub fn size(
        &mut self,
        start: &Slice,
        limit: &Slice,
    ) -> u64 {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_size_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.size.entry",
            start_len = *start.size(),
            limit_len = *limit.size()
        );

        let range = bitcoinleveldb_slice::Range::new(
            Slice::from_ptr_len(*start.data(), *start.size()),
            Slice::from_ptr_len(*limit.data(), *limit.size()),
        );

        let mut size: u64 = 0u64;
        unsafe {
            (&mut *self.db).get_approximate_sizes(
                (&range) as *const bitcoinleveldb_slice::Range,
                1i32,
                (&mut size) as *mut u64,
            );
        }

        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_size_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.size.exit",
            size = size
        );

        size
    }

    /// This preserves the original C++ signed-size arithmetic model.
    /// Empty ranges must remain representable without unsigned underflow.
    pub fn signed_size(
        &mut self,
        start: &Slice,
        limit: &Slice,
    ) -> i64 {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_signed_size_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.signed_size.entry",
            start_len = *start.size(),
            limit_len = *limit.size()
        );

        let raw_size: u64 = self.size(start, limit);

        let signed_size: i64 = match raw_size > i64::MAX as u64 {
            true => {
                warn!(
                    target: "bitcoinleveldbt_autocompaction::autocompact_test",
                    event = "auto_compact_test_signed_size_saturated",
                    label = "bitcoinleveldbt_autocompaction.auto_compact_test.signed_size.saturated",
                    raw_size = raw_size
                );
                i64::MAX
            }
            false => raw_size as i64,
        };

        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_signed_size_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.signed_size.exit",
            signed_size = signed_size
        );

        signed_size
    }

    /// This reads a DB property through the public interface and preserves the
    /// absence case as the empty string so diagnostics remain total and non-panicking.
    pub fn property_string_or_empty(
        &mut self,
        property_name: &str,
    ) -> String {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_property_string_or_empty_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.property_string_or_empty.entry",
            property_name = property_name
        );

        let mut value = String::new();

        let present = unsafe {
            (&mut *self.db).get_property(
                property_name,
                (&mut value) as *mut String,
            )
        };

        let out = match present {
            true => value,
            false => String::new(),
        };

        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_property_string_or_empty_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.property_string_or_empty.exit",
            property_name = property_name,
            present = present,
            value_len = out.len()
        );

        out
    }

    /// This preserves a total diagnostic surface for memory-usage collection.
    /// Parse failures degrade to zero and are always traced.
    pub fn approximate_memory_usage_property_bytes_or_zero(&mut self) -> u64 {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_approximate_memory_usage_property_bytes_or_zero_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.approximate_memory_usage_property_bytes_or_zero.entry"
        );

        let property_value =
            self.property_string_or_empty("leveldb.approximate-memory-usage");

        let parsed_value = match property_value.parse::<u64>() {
            Ok(parsed) => parsed,
            Err(parse_error) => {
                warn!(
                    target: "bitcoinleveldbt_autocompaction::autocompact_test",
                    event = "auto_compact_test_approximate_memory_usage_property_parse_failed",
                    label = "bitcoinleveldbt_autocompaction.auto_compact_test.approximate_memory_usage_property_bytes_or_zero.parse_failed",
                    property_len = property_value.len(),
                    parse_error = ?parse_error
                );
                0u64
            }
        };

        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_approximate_memory_usage_property_bytes_or_zero_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.approximate_memory_usage_property_bytes_or_zero.exit",
            parsed_value = parsed_value
        );

        parsed_value
    }

    /// This constructs one temporally ordered progress snapshot for a single
    /// read/measure iteration. The returned observation must describe one
    /// complete post-sleep state transition only.
    pub fn collect_read_progress_observation(
        &mut self,
        iteration_index: i32,
        initial_visible_range_size_bytes: i64,
        initial_untouched_range_size_bytes: i64,
        visible_range_size_bytes: i64,
        untouched_range_size_bytes: i64,
    ) -> BitcoinLevelDbTAutoCompactionReadProgressObservation {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_collect_read_progress_observation_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.collect_read_progress_observation.entry",
            iteration_index = iteration_index,
            initial_visible_range_size_bytes = initial_visible_range_size_bytes,
            initial_untouched_range_size_bytes = initial_untouched_range_size_bytes,
            visible_range_size_bytes = visible_range_size_bytes,
            untouched_range_size_bytes = untouched_range_size_bytes
        );

        let approximate_memory_usage_bytes =
            self.approximate_memory_usage_property_bytes_or_zero();
        let leveldb_stats = self.property_string_or_empty("leveldb.stats");
        let leveldb_sstables = self.property_string_or_empty("leveldb.sstables");

        let observation_builder =
            BitcoinLevelDbTAutoCompactionReadProgressObservationBuilder::default()
                .iteration_index(iteration_index)
                .initial_visible_range_size_bytes(initial_visible_range_size_bytes)
                .initial_untouched_range_size_bytes(initial_untouched_range_size_bytes)
                .visible_range_size_bytes(visible_range_size_bytes)
                .untouched_range_size_bytes(untouched_range_size_bytes)
                .approximate_memory_usage_bytes(approximate_memory_usage_bytes)
                .leveldb_stats(leveldb_stats)
                .leveldb_sstables(leveldb_sstables);

        let observation = match observation_builder.build() {
            Ok(value) => value,
            Err(builder_error) => {
                error!(
                    target: "bitcoinleveldbt_autocompaction::autocompact_test",
                    event = "auto_compact_test_collect_read_progress_observation_builder_failed",
                    label = "bitcoinleveldbt_autocompaction.auto_compact_test.collect_read_progress_observation.builder_failed",
                    iteration_index = iteration_index,
                    builder_error = ?builder_error
                );
                panic!();
            }
        };

        info!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_collect_read_progress_observation_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.collect_read_progress_observation.exit",
            iteration_index = *observation.iteration_index(),
            visible_range_size_bytes = *observation.visible_range_size_bytes(),
            untouched_range_size_bytes = *observation.untouched_range_size_bytes(),
            approximate_memory_usage_bytes = *observation.approximate_memory_usage_bytes(),
            leveldb_stats_len = observation.leveldb_stats().len(),
            leveldb_sstables_len = observation.leveldb_sstables().len()
        );

        observation
    }

    /// This prepares the exact “values then tombstones” fixture topology used by
    /// the autocompaction test, but all harness timing must remain outside the
    /// shared database `Env` so background compaction threads are not starved by
    /// direct `RefCell` borrows from the test thread.
    pub fn prepare_parameterized_deleted_fixture_for_auto_compaction_probe(
        &mut self,
        entry_count: i32,
        value_size_bytes: i32,
        visible_key_count: i32,
    ) -> BitcoinLevelDbTAutoCompactionParameterizedFixturePreparationObservation {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_prepare_parameterized_deleted_fixture_for_auto_compaction_probe_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.prepare_parameterized_deleted_fixture_for_auto_compaction_probe.entry",
            entry_count = entry_count,
            value_size_bytes = value_size_bytes,
            visible_key_count = visible_key_count
        );

        assert!(entry_count > 0i32);
        assert!(value_size_bytes > 0i32);
        assert!(visible_key_count >= 0i32);
        assert!(visible_key_count <= entry_count);

        let value = "x".repeat(value_size_bytes as usize);
        let dbi: *mut DBImpl = (self.db as *mut ()) as *mut DBImpl;

        let fill_progress_interval = match entry_count >= 64i32 {
            true => 16i32,
            false => 8i32,
        };

        let fill_started_at = Instant::now();

        let mut i: i32 = 0i32;
        while i < entry_count {
            let key = self.key(i);
            let status = unsafe {
                (&mut *self.db).put(
                    &WriteOptions::default(),
                    &Slice::from(&key),
                    &Slice::from(&value),
                )
            };
            assert!(status.is_ok());

            if ((i + 1i32) % fill_progress_interval) == 0i32 || (i + 1i32) == entry_count {
                info!(
                    target: "bitcoinleveldbt_autocompaction::autocompact_test",
                    event = "auto_compact_test_prepare_parameterized_deleted_fixture_for_auto_compaction_probe_fill_progress",
                    label = "bitcoinleveldbt_autocompaction.auto_compact_test.prepare_parameterized_deleted_fixture_for_auto_compaction_probe.fill_progress",
                    entry_count = entry_count,
                    value_size_bytes = value_size_bytes,
                    entries_written = i + 1i32
                );
            }

            i += 1i32;
        }

        let fill_phase_elapsed_micros =
            bitcoinleveldbt_autocompaction_elapsed_micros_from_instant(fill_started_at);

        let compact_after_fill_started_at = Instant::now();
        let compact_status_after_fill = unsafe { (&mut *dbi).test_compact_mem_table() };
        let compact_after_fill_elapsed_micros =
            bitcoinleveldbt_autocompaction_elapsed_micros_from_instant(
                compact_after_fill_started_at,
            );

        assert!(compact_status_after_fill.is_ok());

        let delete_started_at = Instant::now();

        i = 0i32;
        while i < entry_count {
            let key = self.key(i);
            let status = unsafe {
                (&mut *self.db).delete(
                    &WriteOptions::default(),
                    &Slice::from(&key),
                )
            };
            assert!(status.is_ok());

            if ((i + 1i32) % fill_progress_interval) == 0i32 || (i + 1i32) == entry_count {
                info!(
                    target: "bitcoinleveldbt_autocompaction::autocompact_test",
                    event = "auto_compact_test_prepare_parameterized_deleted_fixture_for_auto_compaction_probe_delete_progress",
                    label = "bitcoinleveldbt_autocompaction.auto_compact_test.prepare_parameterized_deleted_fixture_for_auto_compaction_probe.delete_progress",
                    entry_count = entry_count,
                    value_size_bytes = value_size_bytes,
                    entries_deleted = i + 1i32
                );
            }

            i += 1i32;
        }

        let delete_phase_elapsed_micros =
            bitcoinleveldbt_autocompaction_elapsed_micros_from_instant(delete_started_at);

        let compact_after_delete_started_at = Instant::now();
        let compact_status_after_delete = unsafe { (&mut *dbi).test_compact_mem_table() };
        let compact_after_delete_elapsed_micros =
            bitcoinleveldbt_autocompaction_elapsed_micros_from_instant(
                compact_after_delete_started_at,
            );

        assert!(compact_status_after_delete.is_ok());

        let key0 = self.key(0);
        let keyn = self.key(visible_key_count);
        let last_key = self.key(entry_count);

        let initial_visible_range_size_bytes = self.signed_size(
            &Slice::from(&key0),
            &Slice::from(&keyn),
        );

        let initial_untouched_range_size_bytes = self.signed_size(
            &Slice::from(&keyn),
            &Slice::from(&last_key),
        );

        let approximate_memory_usage_bytes =
            self.approximate_memory_usage_property_bytes_or_zero();
        let leveldb_stats = self.property_string_or_empty("leveldb.stats");
        let leveldb_sstables = self.property_string_or_empty("leveldb.sstables");

        let observation_builder =
            BitcoinLevelDbTAutoCompactionParameterizedFixturePreparationObservationBuilder::default()
                .entry_count(entry_count)
                .value_size_bytes(value_size_bytes)
                .visible_key_count(visible_key_count)
                .fill_phase_elapsed_micros(fill_phase_elapsed_micros)
                .compact_after_fill_elapsed_micros(compact_after_fill_elapsed_micros)
                .delete_phase_elapsed_micros(delete_phase_elapsed_micros)
                .compact_after_delete_elapsed_micros(compact_after_delete_elapsed_micros)
                .initial_visible_range_size_bytes(initial_visible_range_size_bytes)
                .initial_untouched_range_size_bytes(initial_untouched_range_size_bytes)
                .approximate_memory_usage_bytes(approximate_memory_usage_bytes)
                .leveldb_stats(leveldb_stats)
                .leveldb_sstables(leveldb_sstables);

        let observation = match observation_builder.build() {
            Ok(value) => value,
            Err(builder_error) => {
                error!(
                    target: "bitcoinleveldbt_autocompaction::autocompact_test",
                    event = "auto_compact_test_prepare_parameterized_deleted_fixture_for_auto_compaction_probe_builder_failed",
                    label = "bitcoinleveldbt_autocompaction.auto_compact_test.prepare_parameterized_deleted_fixture_for_auto_compaction_probe.builder_failed",
                    entry_count = entry_count,
                    value_size_bytes = value_size_bytes,
                    visible_key_count = visible_key_count,
                    builder_error = ?builder_error
                );
                panic!();
            }
        };

        info!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_prepare_parameterized_deleted_fixture_for_auto_compaction_probe_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.prepare_parameterized_deleted_fixture_for_auto_compaction_probe.exit",
            entry_count = *observation.entry_count(),
            value_size_bytes = *observation.value_size_bytes(),
            visible_key_count = *observation.visible_key_count(),
            fill_phase_elapsed_micros = *observation.fill_phase_elapsed_micros(),
            compact_after_fill_elapsed_micros = *observation.compact_after_fill_elapsed_micros(),
            delete_phase_elapsed_micros = *observation.delete_phase_elapsed_micros(),
            compact_after_delete_elapsed_micros = *observation.compact_after_delete_elapsed_micros(),
            initial_visible_range_size_bytes = *observation.initial_visible_range_size_bytes(),
            initial_untouched_range_size_bytes = *observation.initial_untouched_range_size_bytes(),
            leveldb_stats_len = observation.leveldb_stats().len(),
            leveldb_sstables_len = observation.leveldb_sstables().len()
        );

        observation
    }

    /// This mirrors the original `DoReads` read/measure loop while allowing the
    /// fixture scale and sleep window to be reduced for diagnosis. Waiting must
    /// never borrow the shared database `Env`.
    pub fn do_parameterized_reads_progress_probe(
        &mut self,
        entry_count: i32,
        value_size_bytes: i32,
        visible_key_count: i32,
        max_iterations: i32,
        sleep_after_each_read_micros: i32,
    ) -> Vec<BitcoinLevelDbTAutoCompactionReadProgressObservation> {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_parameterized_reads_progress_probe_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_parameterized_reads_progress_probe.entry",
            entry_count = entry_count,
            value_size_bytes = value_size_bytes,
            visible_key_count = visible_key_count,
            max_iterations = max_iterations,
            sleep_after_each_read_micros = sleep_after_each_read_micros
        );

        assert!(entry_count > 0i32);
        assert!(value_size_bytes > 0i32);
        assert!(visible_key_count >= 0i32);
        assert!(visible_key_count <= entry_count);
        assert!(max_iterations > 0i32);
        assert!(sleep_after_each_read_micros >= 0i32);

        let preparation =
            self.prepare_parameterized_deleted_fixture_for_auto_compaction_probe(
                entry_count,
                value_size_bytes,
                visible_key_count,
            );

        let initial_visible_range_size_bytes =
            *preparation.initial_visible_range_size_bytes();
        let initial_untouched_range_size_bytes =
            *preparation.initial_untouched_range_size_bytes();

        let limit_key = self.key(visible_key_count);

        let mut observations: Vec<BitcoinLevelDbTAutoCompactionReadProgressObservation> =
            Vec::new();

        let mut read: i32 = 0i32;
        while read < max_iterations {
            let iter_ptr = unsafe { (&mut *self.db).new_iterator(&ReadOptions::default()) };
            assert!(!iter_ptr.is_null());

            {
                let iter = unsafe { &mut *iter_ptr };
                iter.seek_to_first();
                while iter.valid() && iter.key().to_string() < limit_key {
                    // Drop data
                    iter.next();
                }
            }

            unsafe {
                drop(Box::from_raw(iter_ptr));
            }

            bitcoinleveldbt_autocompaction_sleep_for_microseconds_without_shared_env_borrow(
                sleep_after_each_read_micros,
            );

            let key0 = self.key(0);
            let keyn = self.key(visible_key_count);
            let last_key = self.key(entry_count);

            let visible_range_size_bytes = self.signed_size(
                &Slice::from(&key0),
                &Slice::from(&keyn),
            );

            let untouched_range_size_bytes = self.signed_size(
                &Slice::from(&keyn),
                &Slice::from(&last_key),
            );

            eprintln!(
                "parameterized probe iter {:3} => {:7.3} MB [other {:7.3} MB]",
                read + 1i32,
                visible_range_size_bytes as f64 / 1_048_576.0,
                untouched_range_size_bytes as f64 / 1_048_576.0,
            );

            let observation = self.collect_read_progress_observation(
                read + 1i32,
                initial_visible_range_size_bytes,
                initial_untouched_range_size_bytes,
                visible_range_size_bytes,
                untouched_range_size_bytes,
            );

            observations.push(observation);

            if visible_range_size_bytes <= initial_visible_range_size_bytes / 10i64 {
                break;
            }

            read += 1i32;
        }

        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_parameterized_reads_progress_probe_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_parameterized_reads_progress_probe.exit",
            entry_count = entry_count,
            value_size_bytes = value_size_bytes,
            visible_key_count = visible_key_count,
            observation_count = observations.len()
        );

        observations
    }

    /// This mirrors the original `DoReads` setup and outer read loop, but it is
    /// bounded and returns structured progress observations instead of waiting
    /// indefinitely for the success threshold. Waiting must never borrow the
    /// shared database `Env`.
    pub fn do_reads_progress_probe(
        &mut self,
        n: i32,
        max_iterations: i32,
        sleep_after_each_read_micros: i32,
    ) -> Vec<BitcoinLevelDbTAutoCompactionReadProgressObservation> {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_reads_progress_probe_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads_progress_probe.entry",
            n = n,
            max_iterations = max_iterations,
            sleep_after_each_read_micros = sleep_after_each_read_micros
        );

        let value = "x".repeat(VALUE_SIZE as usize);
        let dbi: *mut DBImpl = (self.db as *mut ()) as *mut DBImpl;

        // Fill database
        let mut i: i32 = 0i32;
        while i < COUNT {
            let key = self.key(i);
            let status = unsafe {
                (&mut *self.db).put(
                    &WriteOptions::default(),
                    &Slice::from(&key),
                    &Slice::from(&value),
                )
            };
            assert!(status.is_ok());
            i += 1i32;
        }

        let compact_status_after_fill = unsafe { (&mut *dbi).test_compact_mem_table() };
        assert!(compact_status_after_fill.is_ok());

        // Delete everything
        i = 0i32;
        while i < COUNT {
            let key = self.key(i);
            let status = unsafe {
                (&mut *self.db).delete(
                    &WriteOptions::default(),
                    &Slice::from(&key),
                )
            };
            assert!(status.is_ok());
            i += 1i32;
        }

        let compact_status_after_delete = unsafe { (&mut *dbi).test_compact_mem_table() };
        assert!(compact_status_after_delete.is_ok());

        let key0 = self.key(0);
        let keyn = self.key(n);
        let last_key = self.key(COUNT);

        // Get initial measurement of the space we will be reading.
        let initial_visible_range_size_bytes = self.signed_size(
            &Slice::from(&key0),
            &Slice::from(&keyn),
        );
        let initial_untouched_range_size_bytes = self.signed_size(
            &Slice::from(&keyn),
            &Slice::from(&last_key),
        );

        let limit_key = self.key(n);

        let mut observations: Vec<BitcoinLevelDbTAutoCompactionReadProgressObservation> =
            Vec::new();

        let mut read: i32 = 0i32;
        while read < max_iterations {
            let iter_ptr = unsafe { (&mut *self.db).new_iterator(&ReadOptions::default()) };
            assert!(!iter_ptr.is_null());

            {
                let iter = unsafe { &mut *iter_ptr };
                iter.seek_to_first();
                while iter.valid() && iter.key().to_string() < limit_key {
                    // Drop data
                    iter.next();
                }
            }

            unsafe {
                drop(Box::from_raw(iter_ptr));
            }

            bitcoinleveldbt_autocompaction_sleep_for_microseconds_without_shared_env_borrow(
                sleep_after_each_read_micros,
            );

            let key0_after = self.key(0);
            let keyn_after = self.key(n);
            let last_key_after = self.key(COUNT);

            let visible_range_size_bytes = self.signed_size(
                &Slice::from(&key0_after),
                &Slice::from(&keyn_after),
            );

            let untouched_range_size_bytes = self.signed_size(
                &Slice::from(&keyn_after),
                &Slice::from(&last_key_after),
            );

            eprintln!(
                "probe iter {:3} => {:7.3} MB [other {:7.3} MB]",
                read + 1i32,
                visible_range_size_bytes as f64 / 1_048_576.0,
                untouched_range_size_bytes as f64 / 1_048_576.0,
            );

            let observation = self.collect_read_progress_observation(
                read + 1i32,
                initial_visible_range_size_bytes,
                initial_untouched_range_size_bytes,
                visible_range_size_bytes,
                untouched_range_size_bytes,
            );

            observations.push(observation);

            if visible_range_size_bytes <= initial_visible_range_size_bytes / 10i64 {
                break;
            }

            read += 1i32;
        }

        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_reads_progress_probe_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads_progress_probe.exit",
            n = n,
            observation_count = observations.len()
        );

        observations
    }

    /**
      | Read through the first n keys repeatedly
      | and check that they get compacted (verified
      | by checking the size of the key space).
      |
      |
      | Waiting must never borrow the shared database `Env`.
      | The DB's background compaction threads reach that same `Env`
      | through `EnvWrapper`, and direct harness borrows can stall or
      | abort compaction progress.
      */
    pub fn do_reads(&mut self, n: i32) {
        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_reads_entry",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.entry",
            n = n
        );

        let value = "x".repeat(VALUE_SIZE as usize);
        let dbi: *mut DBImpl = (self.db as *mut ()) as *mut DBImpl;

        let fill_started_at = Instant::now();

        // Fill database
        let mut i: i32 = 0i32;
        while i < COUNT {
            let key = self.key(i);
            let status = unsafe {
                (&mut *self.db).put(
                    &WriteOptions::default(),
                    &Slice::from(&key),
                    &Slice::from(&value),
                )
            };
            assert!(status.is_ok());

            if ((i + 1i32) % 64i32) == 0i32 || (i + 1i32) == COUNT {
                info!(
                    target: "bitcoinleveldbt_autocompaction::autocompact_test",
                    event = "auto_compact_test_do_reads_fill_progress",
                    label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.fill_progress",
                    n = n,
                    entries_written = i + 1i32,
                    total_entries = COUNT
                );
            }

            i += 1i32;
        }

        let fill_elapsed_micros =
            bitcoinleveldbt_autocompaction_elapsed_micros_from_instant(fill_started_at);

        info!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_reads_fill_complete",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.fill_complete",
            n = n,
            entries_written = COUNT,
            fill_elapsed_micros = fill_elapsed_micros
        );

        let compact_after_fill_started_at = Instant::now();
        let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
        let compact_after_fill_elapsed_micros =
            bitcoinleveldbt_autocompaction_elapsed_micros_from_instant(
                compact_after_fill_started_at,
            );
        assert!(compact_status.is_ok());

        info!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_reads_compact_after_fill_complete",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.compact_after_fill_complete",
            n = n,
            compact_after_fill_elapsed_micros = compact_after_fill_elapsed_micros
        );

        let delete_started_at = Instant::now();

        // Delete everything
        i = 0i32;
        while i < COUNT {
            let key = self.key(i);
            let status = unsafe {
                (&mut *self.db).delete(
                    &WriteOptions::default(),
                    &Slice::from(&key),
                )
            };
            assert!(status.is_ok());

            if ((i + 1i32) % 64i32) == 0i32 || (i + 1i32) == COUNT {
                info!(
                    target: "bitcoinleveldbt_autocompaction::autocompact_test",
                    event = "auto_compact_test_do_reads_delete_progress",
                    label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.delete_progress",
                    n = n,
                    entries_deleted = i + 1i32,
                    total_entries = COUNT
                );
            }

            i += 1i32;
        }

        let delete_elapsed_micros =
            bitcoinleveldbt_autocompaction_elapsed_micros_from_instant(delete_started_at);

        info!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_reads_delete_complete",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.delete_complete",
            n = n,
            entries_deleted = COUNT,
            delete_elapsed_micros = delete_elapsed_micros
        );

        let compact_after_delete_started_at = Instant::now();
        let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
        let compact_after_delete_elapsed_micros =
            bitcoinleveldbt_autocompaction_elapsed_micros_from_instant(
                compact_after_delete_started_at,
            );
        assert!(compact_status.is_ok());

        info!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_reads_compact_after_delete_complete",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.compact_after_delete_complete",
            n = n,
            compact_after_delete_elapsed_micros = compact_after_delete_elapsed_micros
        );

        let key0 = self.key(0);
        let keyn = self.key(n);
        let last_key = self.key(COUNT);

        // Get initial measurement of the space we will be reading.
        let initial_size = self.signed_size(
            &Slice::from(&key0),
            &Slice::from(&keyn),
        );
        let initial_other_size = self.signed_size(
            &Slice::from(&keyn),
            &Slice::from(&last_key),
        );

        info!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_reads_initial_measurement",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.initial_measurement",
            n = n,
            initial_size = initial_size,
            initial_other_size = initial_other_size
        );

        // Read until size drops significantly.
        let limit_key = self.key(n);

        let mut read: i32 = 0i32;
        loop {
            assert!(read < 100, "Taking too long to compact");

            let iter_ptr = unsafe { (&mut *self.db).new_iterator(&ReadOptions::default()) };
            assert!(!iter_ptr.is_null());

            {
                let iter = unsafe { &mut *iter_ptr };
                iter.seek_to_first();
                while iter.valid() && iter.key().to_string() < limit_key {
                    // Drop data
                    iter.next();
                }
            }

            unsafe {
                drop(Box::from_raw(iter_ptr));
            }

            // Wait a little bit to allow any triggered compactions to complete.
            bitcoinleveldbt_autocompaction_sleep_for_microseconds_without_shared_env_borrow(
                1_000_000i32,
            );

            let key0 = self.key(0);
            let keyn = self.key(n);
            let last_key = self.key(COUNT);

            let size = self.signed_size(
                &Slice::from(&key0),
                &Slice::from(&keyn)
            );

            let other_size = self.signed_size(
                &Slice::from(&keyn),
                &Slice::from(&last_key)
            );

            let approximate_memory_usage_bytes =
                self.approximate_memory_usage_property_bytes_or_zero();
            let leveldb_stats = self.property_string_or_empty("leveldb.stats");
            let leveldb_sstables = self.property_string_or_empty("leveldb.sstables");

            info!(
                target: "bitcoinleveldbt_autocompaction::autocompact_test",
                event = "auto_compact_test_do_reads_progress",
                label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.progress",
                n = n,
                read_iteration = read + 1i32,
                size = size,
                other_size = other_size,
                initial_size = initial_size,
                initial_other_size = initial_other_size,
                approximate_memory_usage_bytes = approximate_memory_usage_bytes,
                leveldb_stats_len = leveldb_stats.len(),
                leveldb_sstables_len = leveldb_sstables.len()
            );

            eprintln!(
                "iter {:3} => {:7.3} MB [other {:7.3} MB]",
                read + 1i32,
                size as f64 / 1_048_576.0,
                other_size as f64 / 1_048_576.0,
            );

            if size <= initial_size / 10i64 {
                break;
            }

            read += 1i32;
        }

        let keyn = self.key(n);
        let last_key = self.key(COUNT);

        // Verify that the size of the key space not touched by the reads
        // is pretty much unchanged.
        let final_other_size = self.signed_size(
            &Slice::from(&keyn),
            &Slice::from(&last_key)
        );

        assert!(final_other_size <= initial_other_size + 1_048_576i64);
        assert!(final_other_size >= initial_other_size / 5i64 - 1_048_576i64);

        trace!(
            target: "bitcoinleveldbt_autocompaction::autocompact_test",
            event = "auto_compact_test_do_reads_exit",
            label = "bitcoinleveldbt_autocompaction.auto_compact_test.do_reads.exit",
            n = n
        );
    }
}

lazy_static! {
    /// This mutex defines the process-local execution boundary for autocompaction
    /// tests. The boundary is intentional: these tests exercise large compaction
    /// loops under a process-global tracing subscriber, and parallel execution
    /// makes wall-clock watchdog output non-diagnostic through shared log
    /// backpressure and concurrent compaction topology noise.
    pub static ref BITCOINLEVELDBT_AUTOCOMPACTION_SERIAL_HEAVY_TEST_MUTEX: Mutex<()> =
        Mutex::new(());
}

/// This wrapper preserves test semantics while serializing the heavy
/// autocompaction bodies inside the process. The wrapped closure must contain
/// the entire DB lifecycle so all background compaction and teardown events
/// stay inside the same serialized boundary.
pub fn bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary<TBitcoinLevelDbTAutoCompactionTestBody>(
    test_name: &'static str,
    test_body: TBitcoinLevelDbTAutoCompactionTestBody,
) where
    TBitcoinLevelDbTAutoCompactionTestBody: FnOnce(),
{
    trace!(
        target: "bitcoinleveldbt_autocompaction::autocompact_test",
        event = "bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary_entry",
        label = "bitcoinleveldbt_autocompaction.execute_with_serial_heavy_test_boundary.entry",
        test_name = test_name
    );

    let _serial_guard = BITCOINLEVELDBT_AUTOCOMPACTION_SERIAL_HEAVY_TEST_MUTEX.lock();

    debug!(
        target: "bitcoinleveldbt_autocompaction::autocompact_test",
        event = "bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary_state_transition",
        label = "bitcoinleveldbt_autocompaction.execute_with_serial_heavy_test_boundary.acquired",
        test_name = test_name
    );

    test_body();

    trace!(
        target: "bitcoinleveldbt_autocompaction::autocompact_test",
        event = "bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary_exit",
        label = "bitcoinleveldbt_autocompaction.execute_with_serial_heavy_test_boundary.exit",
        test_name = test_name
    );
}

#[traced_test]
fn auto_compact_test_read_all_empty_other_range_uses_signed_lower_bound_space() {
    bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary(
        "auto_compact_test_read_all_empty_other_range_uses_signed_lower_bound_space",
        || {
            let mut t = AutoCompactTest::default();

            let keyn = t.key(COUNT);
            let last_key = t.key(COUNT);

            let initial_other_size = t.signed_size(
                &Slice::from(&keyn),
                &Slice::from(&last_key),
            );

            let lower_bound = initial_other_size / 5i64 - 1_048_576i64;

            assert_eq!(initial_other_size, 0i64);
            assert!(lower_bound < 0i64);
        },
    );
}

#[traced_test]
fn auto_compact_test_parameterized_fixture_preparation_probe_reports_completed_compaction_barriers() {
    bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary(
        "auto_compact_test_parameterized_fixture_preparation_probe_reports_completed_compaction_barriers",
        || {
            let mut t = AutoCompactTest::default();

            let observation =
                t.prepare_parameterized_deleted_fixture_for_auto_compaction_probe(
                    96i32,
                    128i32 * 1024i32,
                    48i32,
                );

            assert_eq!(*observation.entry_count(), 96i32);
            assert_eq!(*observation.value_size_bytes(), 128i32 * 1024i32);
            assert_eq!(*observation.visible_key_count(), 48i32);
            assert!(*observation.initial_visible_range_size_bytes() > 0i64);
            assert!(*observation.initial_untouched_range_size_bytes() >= 0i64);
            assert!(
                observation.leveldb_stats().len() > 0usize
                    || observation.leveldb_sstables().len() > 0usize
            );
        },
    );
}

#[traced_test]
fn auto_compact_test_parameterized_small_fixture_read_half_progress_probe_exhibits_early_compaction_or_topology_motion() {
    bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary(
        "auto_compact_test_parameterized_small_fixture_read_half_progress_probe_exhibits_early_compaction_or_topology_motion",
        || {
            let mut t = AutoCompactTest::default();

            let observations =
                t.do_parameterized_reads_progress_probe(
                    96i32,
                    128i32 * 1024i32,
                    48i32,
                    16i32,
                    250_000i32,
                );

            assert!(!observations.is_empty());
            assert!(*observations[0].initial_visible_range_size_bytes() > 0i64);

            let first_visible_range_size_bytes =
                *observations[0].visible_range_size_bytes();

            let mut observed_visible_range_shrink = false;
            let mut observed_sstable_topology_motion = false;

            let mut previous_sstable_snapshot =
                observations[0].leveldb_sstables().clone();

            let mut index: usize = 1usize;
            while index < observations.len() {
                let current_observation =
                    &observations[index];

                if *current_observation.visible_range_size_bytes()
                    < first_visible_range_size_bytes
                {
                    observed_visible_range_shrink = true;
                }

                if current_observation.leveldb_sstables()
                    != &previous_sstable_snapshot
                {
                    observed_sstable_topology_motion = true;
                }

                previous_sstable_snapshot =
                    current_observation.leveldb_sstables().clone();

                index += 1usize;
            }

            assert!(
                observed_visible_range_shrink || observed_sstable_topology_motion,
                "Parameterized ReadHalf made no visible size progress and no sstable-topology progress within the bounded probe window"
            );
        },
    );
}

#[traced_test]
fn auto_compact_test_read_all_progress_probe_exhibits_early_compaction_or_topology_motion() {
    bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary(
        "auto_compact_test_read_all_progress_probe_exhibits_early_compaction_or_topology_motion",
        || {
            let mut t = AutoCompactTest::default();

            let observations =
                t.do_parameterized_reads_progress_probe(
                    96i32,
                    128i32 * 1024i32,
                    96i32,
                    16i32,
                    250_000i32,
                );

            assert!(!observations.is_empty());
            assert!(*observations[0].initial_visible_range_size_bytes() > 0i64);

            let first_visible_range_size_bytes =
                *observations[0].visible_range_size_bytes();

            let mut observed_visible_range_shrink = false;
            let mut observed_sstable_topology_motion = false;

            let mut previous_sstable_snapshot =
                observations[0].leveldb_sstables().clone();

            let mut index: usize = 1usize;
            while index < observations.len() {
                let current_observation =
                    &observations[index];

                if *current_observation.visible_range_size_bytes()
                    < first_visible_range_size_bytes
                {
                    observed_visible_range_shrink = true;
                }

                if current_observation.leveldb_sstables()
                    != &previous_sstable_snapshot
                {
                    observed_sstable_topology_motion = true;
                }

                previous_sstable_snapshot =
                    current_observation.leveldb_sstables().clone();

                index += 1usize;
            }

            assert!(
                observed_visible_range_shrink || observed_sstable_topology_motion,
                "Parameterized ReadAll made no visible size progress and no sstable-topology progress within the bounded probe window"
            );
        },
    );
}

#[traced_test]
fn auto_compact_test_read_all() {
    bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary(
        "auto_compact_test_read_all",
        || {
            let mut t = AutoCompactTest::default();
            t.do_reads(COUNT);
        },
    );
}

#[traced_test]
fn auto_compact_test_read_half() {
    bitcoinleveldbt_autocompaction_execute_with_serial_heavy_test_boundary(
        "auto_compact_test_read_half",
        || {
            let mut t = AutoCompactTest::default();
            t.do_reads(COUNT / 2);
        },
    );
}
