crate::ix!();

pub fn env_schedule_trampoline_return_value_zeroed_c_void() -> core::ffi::c_void {
    unsafe { core::mem::zeroed::<core::ffi::c_void>() }
}

pub fn bg_work_requires_non_null_db_pointer_or_panics(
    db: *mut core::ffi::c_void,
    tid: std::thread::ThreadId,
) -> *mut core::ffi::c_void {
    if db.is_null() {
        tracing::error!(?tid, "DBImpl::bg_work: received null db pointer");
        panic!();
    }

    db
}

pub fn log_dbimpl_bg_work_entry(tid: std::thread::ThreadId, db_ptr: usize) {
    tracing::info!(?tid, db_ptr, "DBImpl::bg_work: entry");
}

pub fn log_dbimpl_bg_work_dispatch_state_snapshot(
    tid: std::thread::ThreadId,
    db_ptr: usize,
    dbname: &str,
    scheduled: bool,
    bg_error: &Status,
    shutting_down: bool,
    mutex_ptr: usize,
    versions_ptr: usize,
    mem_ptr: usize,
    imm_ptr: usize,
) {
    tracing::info!(
        ?tid,
        dbname = %dbname,
        scheduled,
        bg_error = %bg_error.to_string(),
        shutting_down,
        mutex_ptr,
        versions_ptr,
        mem_ptr,
        imm_ptr,
        "DBImpl::bg_work: dispatching to background_call"
    );
}

pub fn log_dbimpl_bg_work_exit(
    tid: std::thread::ThreadId,
    db_ptr: usize,
    elapsed_ms: u64,
) {
    tracing::info!(?tid, db_ptr, elapsed_ms, "DBImpl::bg_work: exit");
}

pub fn background_compaction_scheduling_is_disallowed_due_to_existing_schedule(
    background_compaction_scheduled: bool,
) -> bool {
    background_compaction_scheduled
}

pub fn background_compaction_scheduling_is_disallowed_due_to_shutdown(shutting_down: bool) -> bool {
    shutting_down
}

pub fn background_compaction_scheduling_is_disallowed_due_to_background_error(bg_error: &Status) -> bool {
    !bg_error.is_ok()
}

pub unsafe fn background_compaction_required_by_memtable_or_manual_request(
    imm: *mut MemTable,
    manual_compaction: *mut ManualCompaction,
    versions: *mut VersionSet,
) -> bool {
    if imm.is_null() && manual_compaction.is_null() {
        (*versions).needs_compaction()
    } else {
        true
    }
}

pub fn background_call_should_execute_background_compaction(shutting_down: bool, bg_error: &Status) -> bool {
    !shutting_down && bg_error.is_ok()
}

pub fn clear_background_compaction_scheduled_flag(background_compaction_scheduled: &mut bool) {
    *background_compaction_scheduled = false;
}

pub fn signal_all_background_work_finished_waiters_using_coordinating_mutex(
    background_work_finished_mutex: &parking_lot::Mutex<()>,
    background_work_finished_signal: &parking_lot::Condvar,
    reason: &'static str,
) {
    let tid = std::thread::current().id();

    tracing::trace!(
        ?tid,
        reason,
        "signal_all_background_work_finished_waiters_using_coordinating_mutex: begin"
    );

    {
        let _guard = background_work_finished_mutex.lock();
        background_work_finished_signal.notify_all();
    }

    tracing::trace!(
        ?tid,
        reason,
        "signal_all_background_work_finished_waiters_using_coordinating_mutex: end"
    );
}

pub fn record_first_background_error_and_signal_waiters_if_needed(
    bg_error: &mut Status,
    new_error: &Status,
    background_work_finished_mutex: &parking_lot::Mutex<()>,
    background_work_finished_signal: &parking_lot::Condvar,
) {
    if bg_error.is_ok() {
        *bg_error = new_error.clone();

        tracing::trace!(
            status = %new_error.to_string(),
            "record_background_error: notifying background_work_finished_signal"
        );

        signal_all_background_work_finished_waiters_using_coordinating_mutex(
            background_work_finished_mutex,
            background_work_finished_signal,
            "record_background_error",
        );
    }
}

pub fn background_compaction_is_manual_requested(manual_compaction: *mut ManualCompaction) -> bool {
    !manual_compaction.is_null()
}

pub unsafe fn manual_compaction_resume_end_key_from_selected_compaction_inputs(
    compaction: *mut Compaction,
) -> InternalKey {
    let mut manual_end: InternalKey = Default::default();

    if !compaction.is_null() {
        let n0: i32 = (*compaction).num_input_files(0);
        if n0 > 0 {
            manual_end = (*(*compaction).input(0, n0 - 1)).largest().clone();
        }
    }

    manual_end
}

pub unsafe fn manual_compaction_begin_boundary_debug_string(m: *mut ManualCompaction) -> String {
    if (*m).begin().is_null() {
        "(begin)".to_string()
    } else {
        (*(*(*m).begin())).debug_string()
    }
}

pub unsafe fn manual_compaction_end_boundary_debug_string(m: *mut ManualCompaction) -> String {
    if (*m).end().is_null() {
        "(end)".to_string()
    } else {
        (*(*m).end()).debug_string()
    }
}

pub unsafe fn manual_compaction_stop_boundary_debug_string(
    m: *mut ManualCompaction,
    manual_end: &InternalKey,
) -> String {
    if *(*m).done() {
        "(end)".to_string()
    } else {
        manual_end.debug_string()
    }
}

pub unsafe fn select_manual_compaction_from_request_and_log_plan(
    versions: *mut VersionSet,
    manual_compaction: *mut ManualCompaction,
) -> (*mut Compaction, InternalKey) {
    let m: *mut ManualCompaction = manual_compaction;

    let c: *mut Compaction =
        (*versions).compact_range(*(*m).level(), *(*m).begin(), *(*m).end());

    (*m).set_done(c.is_null());

    let manual_end: InternalKey =
        manual_compaction_resume_end_key_from_selected_compaction_inputs(c);

    let begin_dbg: String = manual_compaction_begin_boundary_debug_string(m);
    let end_dbg: String = manual_compaction_end_boundary_debug_string(m);
    let stop_dbg: String = manual_compaction_stop_boundary_debug_string(m, &manual_end);

    tracing::info!(
        level = (*m).level(),
        begin = %begin_dbg,
        end = %end_dbg,
        stop = %stop_dbg,
        "Manual compaction"
    );

    (c, manual_end)
}

pub unsafe fn select_automatic_compaction_from_versionset(versions: *mut VersionSet) -> *mut Compaction {
    (*versions).pick_compaction()
}

pub unsafe fn background_compaction_is_trivial_move_candidate(
    is_manual: bool,
    compaction: *mut Compaction,
) -> bool {
    !is_manual && !compaction.is_null() && (*compaction).is_trivial_move()
}

pub unsafe fn execute_trivial_move_compaction_to_next_level_and_apply_version_edit(
    versions: *mut VersionSet,
    mutex: *mut parking_lot::RawMutex,
    compaction: *mut Compaction,
) -> (*mut FileMetaData, Status) {
    assert!(!compaction.is_null());
    assert_eq!((*compaction).num_input_files(0), 1);

    let f: *mut FileMetaData = (*compaction).input(0, 0);

    (*(*compaction).edit()).delete_file((*compaction).level(), *(*f).number());
    (*(*compaction).edit()).add_file(
        (*compaction).level() + 1,
        *(*f).number(),
        *(*f).file_size(),
        (*f).smallest(),
        (*f).largest(),
    );

    let status: Status = (*versions).log_and_apply((*compaction).edit(), mutex);

    (f, status)
}

pub unsafe fn versionset_level_summary_string_or_placeholder(versions: *mut VersionSet) -> String {
    let mut tmp: VersionSetLevelSummaryStorage = Default::default();
    let summary_ptr: *const u8 = (*versions).level_summary(&mut tmp);

    if summary_ptr.is_null() {
        "<null level summary>".to_string()
    } else {
        let buf: &[u8; 100] = tmp.buffer();
        let nul = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        String::from_utf8_lossy(&buf[..nul]).into_owned()
    }
}

pub unsafe fn log_trivial_move_compaction_to_next_level(
    file_meta: *mut FileMetaData,
    compaction: *mut Compaction,
    status: &Status,
    summary: &str,
) {
    tracing::info!(
        file_number = *(*file_meta).number() as u64,
        to_level    = (*compaction).level() + 1,
        file_size   = *(*file_meta).file_size() as u64,
        status      = %status.to_string(),
        summary     = %summary,
        "Moved file to next level"
    );
}

pub fn allocate_compaction_state_for_compaction(compaction: *mut Compaction) -> *mut CompactionState {
    Box::into_raw(Box::new(CompactionState::new(compaction)))
}

pub unsafe fn release_compaction_inputs(compaction: *mut Compaction) {
    (*compaction).release_inputs();
}

pub unsafe fn drop_boxed_compaction_if_non_null(compaction: *mut Compaction) {
    if !compaction.is_null() {
        drop(Box::from_raw(compaction));
    }
}

pub fn background_compaction_error_should_be_logged(status: &Status, shutting_down: bool) -> bool {
    !status.is_ok() && !shutting_down
}

pub unsafe fn finalize_manual_compaction_request_state_and_clear_pointer(
    manual_compaction_slot: &mut *mut ManualCompaction,
    status: &Status,
    manual_end: InternalKey,
) {
    let m: *mut ManualCompaction = *manual_compaction_slot;

    if m.is_null() {
        return;
    }

    if !status.is_ok() {
        (*m).set_done(true);
    }

    if !(*m).done() {
        (*m).set_tmp_storage(manual_end);
        (*m).set_begin((*m).tmp_storage() as *const _);
    }

    *manual_compaction_slot = core::ptr::null_mut();
}

pub unsafe fn cleanup_compaction_state_builder_and_pending_outputs_then_drop(
    compact: *mut CompactionState,
    pending_outputs: &mut std::collections::HashSet<u64>,
) {
    let builder_ptr: *mut TableBuilder = *(*compact).builder();

    if let Some(builder) = builder_ptr.as_mut() {
        builder.abandon();
        drop(Box::from_raw(builder_ptr));
        (*compact).set_builder(core::ptr::null_mut());
    }

    for out in (*compact).outputs().iter() {
        pending_outputs.remove(out.number());
    }

    drop(Box::from_raw(compact));
}

pub unsafe fn ref_current_version_from_versionset(versions: *mut VersionSet) -> *mut Version {
    let base: *mut Version = (*versions).current();
    (*base).ref_();
    base
}

pub unsafe fn unref_version(version: *mut Version) {
    (*version).unref();
}

pub fn override_ok_status_with_memtable_compaction_shutdown_error(status: &mut Status, shutting_down: bool) {
    if status.is_ok() && shutting_down {
        let msg = Slice::from_str("Deleting DB during memtable compaction");
        *status = Status::io_error(&msg, None);
    }
}

pub fn prepare_version_edit_for_memtable_compaction_commit(edit: &mut VersionEdit, logfile_number: u64) {
    edit.set_prev_log_number(0);
    edit.set_log_number(logfile_number);
}

pub unsafe fn log_and_apply_version_edit_to_versionset(
    versions: *mut VersionSet,
    edit: &mut VersionEdit,
    mutex: *mut parking_lot::RawMutex,
) -> Status {
    (*versions).log_and_apply(edit, mutex)
}

pub unsafe fn unref_and_clear_immutable_memtable_and_flag(
    imm: &mut *mut MemTable,
    has_imm: &core::sync::atomic::AtomicBool,
) {
    (*(*imm)).unref();
    *imm = core::ptr::null_mut();
    has_imm.store(false, core::sync::atomic::Ordering::Release);
}
