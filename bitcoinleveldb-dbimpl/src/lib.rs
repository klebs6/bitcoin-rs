// ---------------- [ File: bitcoinleveldb-dbimpl/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{dbimpl}
x!{bg_compaction}
x!{bg_work}
x!{build_batch_group}
x!{cleanup_compaction}
x!{compact_mem_table}
x!{compact_range}
x!{create}
x!{delete}
x!{delete_obsolete_files}
x!{do_compaction_work}
x!{drop}
x!{finish_compaction_output_file}
x!{get}
x!{get_approximate_sizes}
x!{get_property}
x!{get_snapshot}
x!{install_compaction_results}
x!{make_room_for_write}
x!{maybe_ignore_error}
x!{maybe_schedule_compaction}
x!{new_db}
x!{new_internal_iterator}
x!{new_iterator}
x!{open_compaciton_output_file}
x!{put}
x!{record_background_error}
x!{record_read_sample}
x!{recover}
x!{recover_log_file}
x!{release_snapshot}
x!{test_compact_memtable}
x!{test_compact_range}
x!{test_max_next_level_overlapping_bytes}
x!{write}
x!{write_level0_table}
x!{log_reporter}
x!{open}
x!{dbimpl_null_file_lock}
x!{dbimpl_user_comparator_adapter}
x!{background_call}

xt!{test_hooks}
xt!{rt_probe}

xt!{t_dbimpl_compaction_output_boundary_specifications}
xt!{t_dbimpl_live_compaction_output_boundary_support}
xt!{t_live_compaction_mutex_probe_support}
