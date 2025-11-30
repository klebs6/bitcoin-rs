// ---------------- [ File: bitcoinleveldb-posixrafile/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{drop}
x!{filename_cstring_or_status}
x!{maybe_close_temporary_fd}
x!{open_fd_for_read}
x!{posix_random_access_file}
x!{pread_into_slice}
x!{read}
