crate::ix!();

impl Logger {

    pub fn default_shrink_debug_file(&self) -> bool {
        let mask = self.categories.load(std::sync::atomic::Ordering::Relaxed);
        let none_val = LogFlags::NONE as u32;
        let result = mask == none_val;
        trace!("Logger::default_shrink_debug_file => mask=0x{:X}, none=0x{:X}, result={}", mask, none_val, result);
        result
    }

    /// This shrinks debug.log if it's too large. 
    /// In C++: it reads the file size, if > 10% bigger than 10MB, 
    /// it truncates and keeps the last 10MB.
    pub fn shrink_debug_file(&mut self) {
        trace!("Logger::shrink_debug_file => attempt to shrink debug.log if it's too large");
        // We'll do a naive partial translation. Full logic requires `fseek`, etc.
        // We'll replicate the general idea:
        const RECENT_DEBUG_HISTORY_SIZE: usize = 10_000_000;
        if self.file_path.as_os_str().is_empty() {
            debug!("Logger::shrink_debug_file => no path => cannot shrink");
            return;
        }

        // Open in read mode
        let c_path = std::ffi::CString::new(self.file_path.as_os_str().to_string_lossy().as_bytes())
            .expect("Invalid CString for file path");
        let mode_read = std::ffi::CString::new("r").unwrap();

        let file = unsafe { libc::fopen(c_path.as_ptr(), mode_read.as_ptr()) };
        if file.is_null() {
            debug!("Logger::shrink_debug_file => cannot open => skipping");
            return;
        }
        // We might attempt to get the size via `fs::metadata` in Rust:
        let size = match std::fs::metadata(&*self.file_path) {
            Ok(md) => md.len() as usize,
            Err(_) => {
                unsafe { libc::fclose(file); }
                return;
            }
        };

        if size <= (RECENT_DEBUG_HISTORY_SIZE * 11 / 10) {
            trace!("Logger::shrink_debug_file => size={} within limit => no shrink", size);
            unsafe { libc::fclose(file); }
            return;
        }

        // If we get here => we do want to keep only last 10MB
        trace!("Logger::shrink_debug_file => file is too large => size={} => keep last {} bytes", size, RECENT_DEBUG_HISTORY_SIZE);

        // Seek from end
        let seek_pos = (RECENT_DEBUG_HISTORY_SIZE as i64) * -1;
        if unsafe { libc::fseek(file, seek_pos, libc::SEEK_END) } != 0 {
            error!("Logger::shrink_debug_file => fseek(...) failed");
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

        // Now reopen in "w" to truncate
        let mode_write = std::ffi::CString::new("w").unwrap();
        let file2 = unsafe { libc::fopen(c_path.as_ptr(), mode_write.as_ptr()) };
        if file2.is_null() {
            error!("Logger::shrink_debug_file => reopen in 'w' failed => data lost?");
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
        trace!("Logger::shrink_debug_file => finished => wrote {} bytes to truncated file", n_read);
    }
}
