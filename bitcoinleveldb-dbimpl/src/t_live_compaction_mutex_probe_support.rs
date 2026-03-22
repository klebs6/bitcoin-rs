// ---------------- [ File: bitcoinleveldb-dbimpl/src/t_live_compaction_mutex_probe_support.rs ]
crate::ix!();

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BitcoinLeveldbDbimplLiveCompactionMutexProbeSite20260320 {
    /// Invariant: this site is probed only after the manual-compaction runner has
    /// returned control to the boundary harness, so any blocked reacquire reflects
    /// a leaked DB mutex rather than an in-flight compaction body.
    AfterManualCompactionRunner,

    /// Invariant: this site is probed only after a post-compaction inspection helper
    /// has returned, so any blocked reacquire identifies that inspection path as the
    /// first externally visible lock leak.
    AfterPostCompactionInspection,

    /// Invariant: this site is probed immediately before the harness allows owned DB
    /// state to fall out of scope, so a blocked reacquire means teardown would enter
    /// `Drop` with the DB mutex still held.
    BeforeHarnessDrop,
}

#[cfg(test)]
impl BitcoinLeveldbDbimplLiveCompactionMutexProbeSite20260320 {
    /// Postcondition: returns a stable machine-readable site label that must remain
    /// unchanged across refactors so trace consumers can compare runs deterministically.
    pub fn stable_label(self) -> &'static str {
        match self {
            Self::AfterManualCompactionRunner => "after_manual_compaction_runner",
            Self::AfterPostCompactionInspection => "after_post_compaction_inspection",
            Self::BeforeHarnessDrop => "before_harness_drop",
        }
    }
}

#[cfg(test)]
impl DBImpl {
    /// Preconditions: call this only from a live-compaction boundary harness after a
    /// logically complete phase has returned to the caller; do not call it while
    /// intentionally holding `self.mutex`.
    ///
    /// Postconditions:
    /// - returns `true` iff `self.mutex` was immediately reacquirable at the probe site;
    /// - if the probe acquires the mutex, it releases it before returning;
    /// - never waits, spins, unlocks a lock it did not acquire, or mutates DB state
    ///   beyond the temporary probe acquisition.
    ///
    /// Forbidden drift: this probe must stay non-blocking. Converting it into a wait
    /// would hide the first leak site and weaken the harness diagnostic boundary.
    pub fn test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness_20260320(
        &mut self,
        probe_site: BitcoinLeveldbDbimplLiveCompactionMutexProbeSite20260320,
        scenario_label: &'static str,
        phase_detail_label: &'static str,
    ) -> bool {
        let probe_site_label: &'static str = probe_site.stable_label();

        tracing::trace!(
            target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
            label = "bitcoinleveldb_dbimpl_live_compaction_mutex_probe.entry",
            scenario_label = scenario_label,
            phase_detail_label = phase_detail_label,
            probe_site = probe_site_label,
            dbname = %self.dbname,
            scheduled = self.background_compaction_scheduled,
            bg_error_ok = self.bg_error.is_ok(),
            shutting_down = self.shutting_down.load(atomic::Ordering::Acquire),
            manual_compaction_ptr = self.manual_compaction as usize,
            versions_ptr = self.versions as usize,
            mem_ptr = self.mem as usize,
            imm_ptr = self.imm as usize,
        );

        let reacquired: bool = self.mutex.try_lock();

        match reacquired {
            true => {
                tracing::debug!(
                    target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
                    label = "bitcoinleveldb_dbimpl_live_compaction_mutex_probe.observed",
                    scenario_label = scenario_label,
                    phase_detail_label = phase_detail_label,
                    probe_site = probe_site_label,
                    outcome = "reacquired",
                    dbname = %self.dbname,
                    scheduled = self.background_compaction_scheduled,
                    bg_error_ok = self.bg_error.is_ok(),
                    shutting_down = self.shutting_down.load(atomic::Ordering::Acquire),
                    manual_compaction_ptr = self.manual_compaction as usize,
                    versions_ptr = self.versions as usize,
                    mem_ptr = self.mem as usize,
                    imm_ptr = self.imm as usize,
                );

                unsafe {
                    self.mutex.unlock();
                }

                tracing::trace!(
                    target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
                    label = "bitcoinleveldb_dbimpl_live_compaction_mutex_probe.exit",
                    scenario_label = scenario_label,
                    phase_detail_label = phase_detail_label,
                    probe_site = probe_site_label,
                    outcome = "reacquired",
                    dbname = %self.dbname,
                );

                true
            }
            false => {
                tracing::error!(
                    target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
                    label = "bitcoinleveldb_dbimpl_live_compaction_mutex_probe.observed",
                    scenario_label = scenario_label,
                    phase_detail_label = phase_detail_label,
                    probe_site = probe_site_label,
                    outcome = "blocked",
                    dbname = %self.dbname,
                    scheduled = self.background_compaction_scheduled,
                    bg_error_ok = self.bg_error.is_ok(),
                    shutting_down = self.shutting_down.load(atomic::Ordering::Acquire),
                    manual_compaction_ptr = self.manual_compaction as usize,
                    versions_ptr = self.versions as usize,
                    mem_ptr = self.mem as usize,
                    imm_ptr = self.imm as usize,
                );

                tracing::trace!(
                    target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
                    label = "bitcoinleveldb_dbimpl_live_compaction_mutex_probe.exit",
                    scenario_label = scenario_label,
                    phase_detail_label = phase_detail_label,
                    probe_site = probe_site_label,
                    outcome = "blocked",
                    dbname = %self.dbname,
                );

                false
            }
        }
    }
}

/// Preconditions:
/// - `harness_storage` must contain a fully initialized live-compaction boundary harness.
/// - The caller must not have previously taken ownership out of this `ManuallyDrop`.
///
/// Postcondition:
/// - returns the exact harness object stored inside `harness_storage` without
///   changing ownership.
/// - no drop path is triggered by this borrow operation.
#[cfg(test)]
pub fn bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
    harness_storage: &mut ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness>,
) -> &mut BitcoinLevelDbDbImplLiveCompactionBoundaryHarness {
    tracing::trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop.entry",
        harness_storage_ptr =
            harness_storage as *mut ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness>
                as usize,
    );

    let harness_ref: &mut BitcoinLevelDbDbImplLiveCompactionBoundaryHarness = unsafe {
        &mut *(harness_storage as *mut ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness>
            as *mut BitcoinLevelDbDbImplLiveCompactionBoundaryHarness)
    };

    tracing::debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop.exit",
        temporary_database_directory = %harness_ref.temporary_database_directory(),
    );

    harness_ref
}

/// Preconditions:
/// - this must run immediately after the manual-compaction runner returns to the
///   boundary harness.
/// - `harness_storage` must still own the harness and its `DBImpl`.
///
/// Postcondition:
/// - asserts that the DB mutex is immediately reacquirable at the first
///   post-runner boundary.
/// - if the assertion fails, the harness remains undropped so the test fails
///   deterministically instead of hanging in `DBImpl::drop`.
#[cfg(test)]
pub fn bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_manual_compaction_runner_for_live_compaction_boundary_harness_20260320(
    harness_storage: &mut ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness>,
    scenario_label: &'static str,
) {
    tracing::trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_manual_compaction_runner_for_live_compaction_boundary_harness.entry",
        scenario_label = scenario_label,
    );

    let database_instance: &mut DBImpl =
        bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
            harness_storage,
        )
        .database_instance_mut()
        .as_mut();

    let reacquired: bool =
        bitcoinleveldb_dbimpl_probe_mutex_after_manual_compaction_runner_20260320(
            database_instance,
            scenario_label,
        );

    tracing::debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_manual_compaction_runner_for_live_compaction_boundary_harness.exit",
        scenario_label = scenario_label,
        dbname = %database_instance.dbname,
        reacquired,
    );

    assert!(
        reacquired,
        "live compaction boundary harness leaked DB mutex after manual compaction runner"
    );
}

/// Preconditions: call this immediately after the manual-compaction runner returns
/// to the boundary harness and before any post-compaction inspection helper runs.
///
/// Postcondition: returns whether the DB mutex was immediately reacquirable at the
/// first boundary after the runner returned, without asserting or triggering `Drop`.
#[cfg(test)]
pub fn bitcoinleveldb_dbimpl_probe_mutex_after_manual_compaction_runner_20260320(
    db: &mut DBImpl,
    scenario_label: &'static str,
) -> bool {
    tracing::trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_probe_mutex_after_manual_compaction_runner.entry",
        scenario_label = scenario_label,
        dbname = %db.dbname,
    );

    let reacquired: bool =
        db.test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness_20260320(
            BitcoinLeveldbDbimplLiveCompactionMutexProbeSite20260320::AfterManualCompactionRunner,
            scenario_label,
            "manual_compaction_runner",
        );

    tracing::debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_probe_mutex_after_manual_compaction_runner.exit",
        scenario_label = scenario_label,
        dbname = %db.dbname,
        reacquired,
    );

    reacquired
}

/// Preconditions:
/// - this must run immediately after one post-compaction inspection helper
///   returns.
/// - `inspection_label` must be a stable machine-readable identifier for the
///   inspection path that just completed.
///
/// Postcondition:
/// - asserts that the DB mutex is immediately reacquirable after that
///   inspection helper.
/// - if the assertion fails, the harness remains undropped so the test fails
///   deterministically instead of hanging in `DBImpl::drop`.
#[cfg(test)]
pub fn bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_post_compaction_inspection_for_live_compaction_boundary_harness_20260320(
    harness_storage: &mut ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness>,
    scenario_label: &'static str,
    inspection_label: &'static str,
) {
    tracing::trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_post_compaction_inspection_for_live_compaction_boundary_harness.entry",
        scenario_label = scenario_label,
        inspection_label = inspection_label,
    );

    let database_instance: &mut DBImpl =
        bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
            harness_storage,
        )
        .database_instance_mut()
        .as_mut();

    let reacquired: bool =
        bitcoinleveldb_dbimpl_probe_mutex_after_post_compaction_inspection_20260320(
            database_instance,
            scenario_label,
            inspection_label,
        );

    tracing::debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_post_compaction_inspection_for_live_compaction_boundary_harness.exit",
        scenario_label = scenario_label,
        inspection_label = inspection_label,
        dbname = %database_instance.dbname,
        reacquired,
    );

    assert!(
        reacquired,
        "live compaction boundary harness leaked DB mutex after post-compaction inspection"
    );
}

/// Preconditions:
/// - this must run as the last boundary check before the harness is explicitly
///   dropped.
/// - `harness_storage` must still own the harness and its `DBImpl`.
///
/// Postcondition:
/// - asserts that the DB mutex is immediately reacquirable at the final
///   pre-drop boundary.
/// - if the assertion fails, the harness remains undropped so the test fails
///   deterministically instead of hanging in `DBImpl::drop`.
#[cfg(test)]
pub fn bitcoinleveldb_dbimpl_assert_mutex_reacquirable_before_live_compaction_harness_drop_20260320(
    harness_storage: &mut ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness>,
    scenario_label: &'static str,
) {
    tracing::trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_assert_mutex_reacquirable_before_live_compaction_harness_drop.entry",
        scenario_label = scenario_label,
    );

    let database_instance: &mut DBImpl =
        bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
            harness_storage,
        )
        .database_instance_mut()
        .as_mut();

    let reacquired: bool =
        bitcoinleveldb_dbimpl_probe_mutex_before_live_compaction_harness_drop_20260320(
            database_instance,
            scenario_label,
        );

    tracing::debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_assert_mutex_reacquirable_before_live_compaction_harness_drop.exit",
        scenario_label = scenario_label,
        dbname = %database_instance.dbname,
        reacquired,
    );

    assert!(
        reacquired,
        "live compaction boundary harness leaked DB mutex before explicit drop"
    );
}

/// Preconditions: call this immediately after a post-compaction inspection helper
/// returns. `inspection_label` must be a stable machine-readable identifier for the
/// specific inspection path that just completed.
///
/// Postcondition: returns whether the DB mutex was immediately reacquirable after
/// that inspection helper, without asserting or triggering `Drop`.
#[cfg(test)]
pub fn bitcoinleveldb_dbimpl_probe_mutex_after_post_compaction_inspection_20260320(
    db: &mut DBImpl,
    scenario_label: &'static str,
    inspection_label: &'static str,
) -> bool {
    tracing::trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_probe_mutex_after_post_compaction_inspection.entry",
        scenario_label = scenario_label,
        inspection_label = inspection_label,
        dbname = %db.dbname,
    );

    let reacquired: bool =
        db.test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness_20260320(
            BitcoinLeveldbDbimplLiveCompactionMutexProbeSite20260320::AfterPostCompactionInspection,
            scenario_label,
            inspection_label,
        );

    tracing::debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_probe_mutex_after_post_compaction_inspection.exit",
        scenario_label = scenario_label,
        inspection_label = inspection_label,
        dbname = %db.dbname,
        reacquired,
    );

    reacquired
}

/// Preconditions:
/// - this must be called only after all intended assertions for the scenario have
///   passed.
/// - `harness_storage` must still own the harness and must not have been taken.
///
/// Postcondition:
/// - performs the final pre-drop mutex probe.
/// - takes ownership out of `harness_storage` exactly once and drops the harness
///   explicitly.
/// - leaves `harness_storage` inert so no automatic drop path can run afterward.
#[cfg(test)]
pub fn bitcoinleveldb_dbimpl_drop_live_compaction_boundary_harness_after_final_mutex_probe_20260320(
    harness_storage: &mut ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness>,
    scenario_label: &'static str,
) {
    tracing::trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_drop_live_compaction_boundary_harness_after_final_mutex_probe.entry",
        scenario_label = scenario_label,
        harness_storage_ptr =
            harness_storage as *mut ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness>
                as usize,
    );

    bitcoinleveldb_dbimpl_assert_mutex_reacquirable_before_live_compaction_harness_drop_20260320(
        harness_storage,
        scenario_label,
    );

    let owned_harness: BitcoinLevelDbDbImplLiveCompactionBoundaryHarness = unsafe {
        ManuallyDrop::take(harness_storage)
    };

    tracing::trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_drop_live_compaction_boundary_harness_after_final_mutex_probe.state_transition",
        scenario_label = scenario_label,
        temporary_database_directory = %owned_harness.temporary_database_directory(),
        action = "explicit_drop_begin",
    );

    drop(owned_harness);

    tracing::debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_drop_live_compaction_boundary_harness_after_final_mutex_probe.exit",
        scenario_label = scenario_label,
    );
}

/// Preconditions: call this immediately before the harness allows owned DB state to
/// leave scope or before it performs an explicit drop path.
///
/// Postcondition: returns whether the DB mutex was immediately reacquirable at the
/// final pre-drop boundary, without asserting or triggering `Drop`.
#[cfg(test)]
pub fn bitcoinleveldb_dbimpl_probe_mutex_before_live_compaction_harness_drop_20260320(
    db: &mut DBImpl,
    scenario_label: &'static str,
) -> bool {
    tracing::trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_probe_mutex_before_live_compaction_harness_drop.entry",
        scenario_label = scenario_label,
        dbname = %db.dbname,
    );

    let reacquired: bool =
        db.test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness_20260320(
            BitcoinLeveldbDbimplLiveCompactionMutexProbeSite20260320::BeforeHarnessDrop,
            scenario_label,
            "before_harness_drop",
        );

    tracing::debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_mutex_probe_support",
        label = "bitcoinleveldb_dbimpl_probe_mutex_before_live_compaction_harness_drop.exit",
        scenario_label = scenario_label,
        dbname = %db.dbname,
        reacquired,
    );

    reacquired
}
