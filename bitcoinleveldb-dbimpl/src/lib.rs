// ---------------- [ File: bitcoinleveldb-dbimpl/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{dbimpl}

#[disable]
x!{bg_compaction}
#[disable]
x!{bg_work}
#[disable]
x!{build_batch_group}
#[disable]
x!{cleanup_compaction}
#[disable]
x!{compact_mem_table}
#[disable]
x!{compact_range}
#[disable]
x!{create}
#[disable]
x!{delete}
#[disable]
x!{delete_obsolete_files}
#[disable]
x!{do_compaction_work}

#[disable]
x!{drop}
#[disable]
x!{finish_compaction_output_file}
#[disable]
x!{get}
#[disable]
x!{get_approximate_sizes}
#[disable]
x!{get_property}
#[disable]
x!{get_snapshot}
#[disable]
x!{install_compaction_results}
#[disable]
x!{make_room_for_write}
#[disable]
x!{maybe_ignore_error}
#[disable]
x!{maybe_schedule_compaction}
#[disable]
x!{new_db}
#[disable]
x!{new_internal_iterator}
#[disable]
x!{new_iterator}

x!{open_compaciton_output_file}
x!{put}
x!{record_background_error}
x!{record_real_sample}
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

#[cfg(test)]
x!{test_support}
