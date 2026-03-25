// ---------------- [ File: bitcoinleveldb-tablecache/src/t_tablecache_refcell_borrow_topology.rs ]
crate::ix!();

const BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET: &str =
    "bitcoinleveldb_tablecache::t_tablecache_refcell_borrow_topology";

/**
  | Preserves the observable topology contract at
  | the `TableCache::find_table` boundary.
  |
  | The observer must distinguish a `RefCell`
  | borrow panic from an ordinary non-OK `Status`
  | without depending on panic payload identity.
  */
pub enum BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation {
    /// The harness cannot proceed when the `Options`
    /// surface does not expose an `Env` handle.
    MissingConfiguredEnv,

    /// `find_table` completed and reported success
    /// while no topology violation was observed.
    NoPanicWithOkStatus,

    /// `find_table` completed without a topology
    /// panic and returned its exact non-OK status.
    NoPanicWithNonOkStatus {
        /// The returned `Status` must survive
        /// unchanged so later hardening steps can
        /// separate IO failure from borrow failure.
        status: Status,
    },

    /// The same `Env` `RefCell` was re-borrowed
    /// mutably while an outer mutable borrow was
    /// still live.
    PanicObserved,
}

/**
  | Observes whether `TableCache::find_table`
  | re-borrows the configured `Env` mutably when
  | the caller already holds that mutable borrow.
  |
  | Preconditions:
  | - `Options::default()` must expose an `Env`
  |   handle.
  |
  | Postconditions:
  | - The returned observation cleanly separates
  |   topology panic from filesystem status.
  | - No panic escapes this function.
  | - No filesystem artifact is required for the
  |   observation to be meaningful because the
  |   re-borrow occurs before file open on cache
  |   miss.
  */
pub fn bitcoinleveldb_tablecache_observe_find_table_with_outer_env_mut_borrow_for_test_support(
) -> BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation {
    trace!(
        target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
        label = "tablecache_refcell_borrow_topology_observe_with_outer_borrow.entry",
        phase = "enter"
    );

    let options = Options::default();

    let env_handle = match options.env() {
        Some(handle) => {
            debug!(
                target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                label = "tablecache_refcell_borrow_topology_observe_with_outer_borrow.env_available",
                phase = "state_transition"
            );
            handle.clone()
        }
        None => {
            warn!(
                target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                label = "tablecache_refcell_borrow_topology_observe_with_outer_borrow.missing_env",
                phase = "decision"
            );
            return BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::MissingConfiguredEnv;
        }
    };

    let dbname =
        String::from("bitcoinleveldb_tablecache_refcell_borrow_topology_support_db");
    let mut table_cache = TableCache::new(&dbname, &options, 4);

    let outer_env_borrow_guard = env_handle.borrow_mut();

    debug!(
        target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
        label = "tablecache_refcell_borrow_topology_observe_with_outer_borrow.outer_borrow_acquired",
        phase = "state_transition"
    );

    let panic_result = catch_unwind(AssertUnwindSafe(|| {
        let mut handle: *mut CacheHandle = null_mut();
        table_cache.find_table(1, 0, &mut handle)
    }));

    drop(outer_env_borrow_guard);

    debug!(
        target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
        label = "tablecache_refcell_borrow_topology_observe_with_outer_borrow.outer_borrow_released",
        phase = "state_transition"
    );

    let observation = match panic_result {
        Ok(status) => match status.is_ok() {
            true => {
                debug!(
                    target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                    label = "tablecache_refcell_borrow_topology_observe_with_outer_borrow.no_panic_ok_status",
                    phase = "exit"
                );
                BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::NoPanicWithOkStatus
            }
            false => {
                debug!(
                    target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                    label = "tablecache_refcell_borrow_topology_observe_with_outer_borrow.no_panic_non_ok_status",
                    phase = "exit"
                );
                BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::NoPanicWithNonOkStatus {
                    status,
                }
            }
        },
        Err(_panic_payload) => {
            warn!(
                target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                label = "tablecache_refcell_borrow_topology_observe_with_outer_borrow.panic_observed",
                phase = "exit"
            );
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::PanicObserved
        }
    };

    trace!(
        target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
        label = "tablecache_refcell_borrow_topology_observe_with_outer_borrow.exit",
        phase = "return"
    );

    observation
}

/**
  | Observes the same `find_table` path without an
  | already-live outer mutable borrow of the shared
  | `Env` handle.
  |
  | Preconditions:
  | - `Options::default()` must expose an `Env`
  |   handle.
  |
  | Postconditions:
  | - The returned observation must not report a
  |   topology panic caused by the caller.
  | - Non-OK status remains visible so later steps
  |   can compare topology and IO surfaces.
  */
pub fn bitcoinleveldb_tablecache_observe_find_table_without_outer_env_mut_borrow_for_test_support(
) -> BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation {
    trace!(
        target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
        label = "tablecache_refcell_borrow_topology_observe_without_outer_borrow.entry",
        phase = "enter"
    );

    let options = Options::default();

    let env_handle = match options.env() {
        Some(handle) => {
            debug!(
                target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                label = "tablecache_refcell_borrow_topology_observe_without_outer_borrow.env_available",
                phase = "state_transition"
            );
            handle.clone()
        }
        None => {
            warn!(
                target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                label = "tablecache_refcell_borrow_topology_observe_without_outer_borrow.missing_env",
                phase = "decision"
            );
            return BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::MissingConfiguredEnv;
        }
    };

    drop(env_handle);

    let dbname =
        String::from("bitcoinleveldb_tablecache_refcell_borrow_topology_support_db");
    let mut table_cache = TableCache::new(&dbname, &options, 4);

    let panic_result = catch_unwind(AssertUnwindSafe(|| {
        let mut handle: *mut CacheHandle = null_mut();
        table_cache.find_table(1, 0, &mut handle)
    }));

    let observation = match panic_result {
        Ok(status) => match status.is_ok() {
            true => {
                debug!(
                    target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                    label = "tablecache_refcell_borrow_topology_observe_without_outer_borrow.no_panic_ok_status",
                    phase = "exit"
                );
                BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::NoPanicWithOkStatus
            }
            false => {
                debug!(
                    target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                    label = "tablecache_refcell_borrow_topology_observe_without_outer_borrow.no_panic_non_ok_status",
                    phase = "exit"
                );
                BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::NoPanicWithNonOkStatus {
                    status,
                }
            }
        },
        Err(_panic_payload) => {
            warn!(
                target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
                label = "tablecache_refcell_borrow_topology_observe_without_outer_borrow.panic_observed",
                phase = "exit"
            );
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::PanicObserved
        }
    };

    trace!(
        target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
        label = "tablecache_refcell_borrow_topology_observe_without_outer_borrow.exit",
        phase = "return"
    );

    observation
}

#[cfg(test)]
mod bitcoinleveldb_tablecache_refcell_borrow_topology_support_tests {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_tablecache_refcell_borrow_topology_support_observes_panic_with_outer_borrow() {
        trace!(
            target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
            label = "tablecache_refcell_borrow_topology_support_observes_panic_with_outer_borrow.entry",
            phase = "enter"
        );

        let observation =
            bitcoinleveldb_tablecache_observe_find_table_with_outer_env_mut_borrow_for_test_support();

        match observation {
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::PanicObserved => {}
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::MissingConfiguredEnv => {
                assert!(
                    false,
                    "Options::default() must expose an Env handle for borrow-topology support"
                );
            }
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::NoPanicWithOkStatus => {
                assert!(
                    false,
                    "find_table must not report OK when the shared Env RefCell is already mutably borrowed"
                );
            }
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::NoPanicWithNonOkStatus {
                status: _status,
            } => {
                assert!(
                    false,
                    "borrow-topology support must observe the RefCell panic before an ordinary non-OK status"
                );
            }
        }

        trace!(
            target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
            label = "tablecache_refcell_borrow_topology_support_observes_panic_with_outer_borrow.exit",
            phase = "return"
        );
    }

    #[traced_test]
    fn bitcoinleveldb_tablecache_refcell_borrow_topology_support_distinguishes_topology_from_missing_file_status() {
        trace!(
            target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
            label = "tablecache_refcell_borrow_topology_support_distinguishes_topology_from_missing_file_status.entry",
            phase = "enter"
        );

        let observation =
            bitcoinleveldb_tablecache_observe_find_table_without_outer_env_mut_borrow_for_test_support();

        match observation {
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::MissingConfiguredEnv => {
                assert!(
                    false,
                    "Options::default() must expose an Env handle for borrow-topology support"
                );
            }
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::NoPanicWithOkStatus => {}
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::NoPanicWithNonOkStatus {
                status,
            } => {
                assert!(
                    !status.is_ok(),
                    "control-path observation must preserve the non-OK status surface"
                );
            }
            BitcoinLeveldbTablecacheRefcellBorrowTopologyObservation::PanicObserved => {
                assert!(
                    false,
                    "control-path observation must not panic when no outer Env borrow is held"
                );
            }
        }

        trace!(
            target: BITCOINLEVELDB_TABLECACHE_REFCELL_BORROW_TOPOLOGY_TRACE_TARGET,
            label = "tablecache_refcell_borrow_topology_support_distinguishes_topology_from_missing_file_status.exit",
            phase = "return"
        );
    }
}
