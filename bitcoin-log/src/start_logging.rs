crate::ix!();

impl Logger {

    pub fn start_logging(&mut self) -> bool {

        let mut inner = self.cs().lock();

        assert!(
            inner.buffering(),
            "start_logging() called but buffering=false => logging already started!"
        );
        assert!(
            inner.fileout().is_null(),
            "start_logging => fileout should be null before opening"
        );

        if *self.print_to_file() {
            if self.file_path().as_os_str().is_empty() {
                return false;
            }
            let c_path = std::ffi::CString::new(
                self.file_path().as_os_str().to_string_lossy().as_bytes()
            ).expect("Invalid CString for file path");
            unsafe {
                let mode = std::ffi::CString::new("a").unwrap();
                let f = libc::fopen(c_path.as_ptr(), mode.as_ptr());
                if f.is_null() {
                    return false;
                }
                libc::setbuf(f, std::ptr::null_mut());
                inner.set_fileout(f);
                file_write_str("\n\n\n\n\n", f);
            }
        }

        inner.set_buffering(false);

        while let Some(msg) = inner.msgs_before_open_mut().pop_front() {
            if *self.print_to_file() {
                unsafe {
                    file_write_str(&msg, *inner.fileout());
                }
            }
            if *self.print_to_console() {
                unsafe {
                    libc::fwrite(
                        msg.as_ptr() as *const libc::c_void,
                        1,
                        msg.len(),
                        c_stdout()
                    );
                    libc::fflush(c_stdout());
                }
            }
            for cb in inner.print_callbacks().iter() {
                cb(&msg);
            }
        }

        if *self.print_to_console() {
            unsafe { libc::fflush(c_stdout()); }
        }
        true
    }
}
