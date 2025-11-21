// ---------------- [ File: bitcoinleveldb-posixlogger/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{posix_logger}
x!{posix_logger_build_log_body_from_format_and_arguments}
x!{posix_logger_build_thread_identifier_label}
x!{posix_logger_capture_current_time_components}
x!{posix_logger_compute_required_log_bytes}
x!{posix_logger_construct_log_header_prefix}
x!{posix_logger_copy_full_log_line_into_buffer}
x!{posix_logger_copy_truncated_log_line_into_buffer}
x!{posix_logger_emit_log_line_with_two_phase_buffering}
x!{posix_logger_ensure_trailing_newline_for_buffer}
x!{posix_logger_flush_buffer_to_log_file}
x!{posix_logger_layout_log_line_in_buffer}
x!{posix_logger_logv}
