// ---------------- [ File: bitcoin-log/src/shrink_debug_file.rs ]
crate::ix!();

impl Logger {

    pub fn default_shrink_debug_file(&self) -> bool {
        let mask = self.categories().load(std::sync::atomic::Ordering::Relaxed);
        let none_val = LogFlags::NONE as u32;
        mask == none_val
    }

    pub fn shrink_debug_file(&mut self) {
        const RECENT_DEBUG_HISTORY_SIZE: usize = 10_000_000;
        if self.file_path().as_os_str().is_empty() {
            return;
        }
        let c_path = std::ffi::CString::new(
            self.file_path().as_os_str().to_string_lossy().as_bytes()
        ).expect("Invalid CString for file path");
        let mode_read = std::ffi::CString::new("r").unwrap();

        let file = unsafe { libc::fopen(c_path.as_ptr(), mode_read.as_ptr()) };
        if file.is_null() {
            return;
        }
        let size = match std::fs::metadata(&*self.file_path()) {
            Ok(md) => md.len() as usize,
            Err(_) => {
                unsafe { libc::fclose(file); }
                return;
            }
        };
        if size <= (RECENT_DEBUG_HISTORY_SIZE * 11 / 10) {
            unsafe { libc::fclose(file); }
            return;
        }

        let seek_pos = (RECENT_DEBUG_HISTORY_SIZE as i64) * -1;
        if unsafe { libc::fseek(file, seek_pos, libc::SEEK_END) } != 0 {
            unsafe { libc::fclose(file); }
            return;
        }
        let mut buffer = Vec::with_capacity(RECENT_DEBUG_HISTORY_SIZE);
        buffer.resize(RECENT_DEBUG_HISTORY_SIZE, 0);
        let n_read = unsafe {
            libc::fread(
                buffer.as_mut_ptr() as *mut libc::c_void,
                1,
                RECENT_DEBUG_HISTORY_SIZE,
                file
            )
        };
        unsafe { libc::fclose(file); }

        let mode_write = std::ffi::CString::new("w").unwrap();
        let file2 = unsafe { libc::fopen(c_path.as_ptr(), mode_write.as_ptr()) };
        if file2.is_null() {
            return;
        }
        unsafe {
            libc::fwrite(
                buffer.as_ptr() as *const libc::c_void,
                1,
                n_read,
                file2
            );
            libc::fclose(file2);
        }
    }
}
