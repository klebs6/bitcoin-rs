// ---------------- [ File: bitcoinleveldb-logwriter/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{add_record_internal}
x!{add_record}
x!{append_header_and_payload}
x!{block_available_data_bytes}
x!{block_trailer_bytes_remaining}
x!{build_record_header}
x!{build_type_crc_table}
x!{choose_record_fragment_type}
x!{crc32c_for_record}
x!{create}
x!{emit_physical_record}
x!{initial_block_offset_from_length}
x!{log_writer}
x!{mock_writable_file_add_record}
x!{mock_writable_file_core}
x!{mock_writable_file_emit}
x!{should_start_new_block}
x!{validate_record_fits_in_block}
x!{validate_record_length}
x!{write_trailer_padding_if_necessary}
