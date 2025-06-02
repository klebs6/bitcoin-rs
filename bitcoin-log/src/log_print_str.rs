crate::ix!();

impl Logger {

    pub fn log_print_str(
        &mut self,
        msg: &str,
        logging_function: &str,
        source_file: &str,
        source_line: i32
    ) {
        let mut escaped = log_escape_message(msg);

        let started_line = self
            .started_new_line()
            .load(std::sync::atomic::Ordering::Relaxed);

        if *self.log_sourcelocations() && started_line {
            let shortened = remove_prefix(source_file, "./");
            let prefix = format!("[{}:{}] [{}] ", shortened, source_line, logging_function);
            escaped.insert_str(0, &prefix);
        }

        if *self.log_threadnames() && started_line {
            let thread_name = "main"; // placeholder
            let prefix = format!("[{}] ", thread_name);
            escaped.insert_str(0, &prefix);
        }

        let final_msg = self.log_timestamp_str(&escaped);
        let ends_newline = final_msg.ends_with('\n');
        self.started_new_line()
            .store(ends_newline, std::sync::atomic::Ordering::Relaxed);

        let mut inner = self.cs().lock();

        if *inner.buffering() {
            inner.msgs_before_open_mut().push_back(final_msg);
            return;
        }

        if *self.print_to_console() {
            unsafe {
                libc::fwrite(
                    final_msg.as_ptr() as *const libc::c_void,
                    1,
                    final_msg.len(),
                    c_stdout()
                );
                libc::fflush(c_stdout());
            }
        }

        for cb in inner.print_callbacks().iter() {
            cb(&final_msg);
        }

        if *self.print_to_file() {
            if inner.fileout().is_null() {
                return;
            }
            let was_reopen = self
                .reopen_file()
                .swap(false, std::sync::atomic::Ordering::Relaxed);
            if was_reopen {
                unsafe {
                    let c_path = std::ffi::CString::new(
                        self.file_path().as_os_str().to_string_lossy().as_bytes()
                    ).expect("Invalid CString for file path");
                    let mode = std::ffi::CString::new("a").unwrap();
                    let new_file = libc::fopen(c_path.as_ptr(), mode.as_ptr());
                    if !new_file.is_null() {
                        libc::setbuf(new_file, std::ptr::null_mut());
                        libc::fclose(*inner.fileout());
                        inner.set_fileout(new_file);
                    }
                }
            }
            unsafe {
                file_write_str(&final_msg, *inner.fileout());
            }
        }
    }
}
