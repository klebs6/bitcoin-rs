// ---------------- [ File: bitcoinleveldb-cfg/src/cfg.rs ]
crate::ix!();

/// Grouping of constants. We may want to make some of these parameters set via
/// options.
///
pub const NUM_LEVELS: usize = 7;

/// Level-0 compaction is started when we hit this many files.
///
pub const L0_COMPACTION_TRIGGER: usize = 4;

/// Soft limit on number of level-0 files. We slow down writes at this point.
///
pub const L0_SLOWDOWN_WRITES_TRIGGER: usize = 8;

/// Maximum number of level-0 files. We stop writes at this point.
/// 
pub const L0_STOP_WRITES_TRIGGER: usize = 12;

/// Maximum level to which a new compacted memtable is pushed if it does not
/// create overlap.  
///
/// We try to push to level 2 to avoid the relatively expensive level 0=>1
/// compactions and to avoid some expensive manifest file operations.  
///
/// We do not push all the way to the largest level since that can generate
/// a lot of wasted disk space if the same key space is being repeatedly
/// overwritten.
///
pub const MAX_MEM_COMPACT_LEVEL: usize = 2;

/// Approximate gap in bytes between samples of data read during iteration.
/// 
pub const READ_BYTES_PERIOD: usize = 1048576;

#[cfg(test)]
mod cfg_invariants_spec {
    use super::*;

    #[instrument(level = "trace", skip_all)]
    fn log_current_cfg_snapshot() {
        info!(
            num_levels                = NUM_LEVELS,
            l0_compaction_trigger     = L0_COMPACTION_TRIGGER,
            l0_slowdown_writes        = L0_SLOWDOWN_WRITES_TRIGGER,
            l0_stop_writes            = L0_STOP_WRITES_TRIGGER,
            max_mem_compact_level     = MAX_MEM_COMPACT_LEVEL,
            read_bytes_period         = READ_BYTES_PERIOD,
            "cfg snapshot"
        );
    }

    #[traced_test]
    fn level0_trigger_monotonicity() {
        log_current_cfg_snapshot();

        assert!(
            L0_COMPACTION_TRIGGER < L0_SLOWDOWN_WRITES_TRIGGER,
            "compaction must begin before write‑slowdown"
        );
        assert!(
            L0_SLOWDOWN_WRITES_TRIGGER < L0_STOP_WRITES_TRIGGER,
            "write‑slowdown must precede write‑stop"
        );

        debug!(
            compaction = L0_COMPACTION_TRIGGER,
            slowdown   = L0_SLOWDOWN_WRITES_TRIGGER,
            stop       = L0_STOP_WRITES_TRIGGER,
            "level‑0 trigger ordering verified"
        );
    }

    #[traced_test]
    fn memtable_compaction_target_within_levels() {
        log_current_cfg_snapshot();

        assert!(
            MAX_MEM_COMPACT_LEVEL < NUM_LEVELS,
            "memtable compaction target must be a valid level index"
        );

        info!(
            max_mem_compact_level = MAX_MEM_COMPACT_LEVEL,
            num_levels            = NUM_LEVELS,
            "memtable compaction target is within configured levels"
        );
    }

    #[traced_test]
    fn read_sampling_period_is_power_of_two_and_aligned() {
        log_current_cfg_snapshot();

        const PAGE: usize = 4096;

        assert!(
            READ_BYTES_PERIOD.is_power_of_two(),
            "read sampling period should be a power‑of‑two for efficient masking"
        );
        assert!(
            READ_BYTES_PERIOD >= PAGE && READ_BYTES_PERIOD % PAGE == 0,
            "read sampling period should be page‑aligned and not smaller than a page"
        );

        debug!(
            read_bytes_period = READ_BYTES_PERIOD,
            page              = PAGE,
            "read sampling period alignment verified"
        );
    }

    #[traced_test]
    fn constants_nonzero_and_consistent() {
        log_current_cfg_snapshot();

        assert!(NUM_LEVELS > 0, "there must be at least one level");
        assert!(L0_COMPACTION_TRIGGER > 0, "level‑0 compaction trigger must be non‑zero");
        assert!(L0_SLOWDOWN_WRITES_TRIGGER > 0, "level‑0 slowdown trigger must be non‑zero");
        assert!(L0_STOP_WRITES_TRIGGER > 0, "level‑0 stop‑writes trigger must be non‑zero");

        // sanity: slowdown and stop thresholds should not be *smaller* than compaction
        assert!(
            L0_COMPACTION_TRIGGER <= L0_SLOWDOWN_WRITES_TRIGGER
            && L0_COMPACTION_TRIGGER <= L0_STOP_WRITES_TRIGGER,
            "compaction trigger must be the smallest of the three L0 thresholds"
        );

        info!("all cfg constants are non‑zero and consistent");
    }
}
