// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger.rs ]
/*!
  | Logger implementation that can be shared
  | by all environments where enough posix
  | functionality is available.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/posix_logger.h]

#[derive(Getters)]
#[getset(get="pub")]
pub struct PosixLogger {
    fp:   *const libc::FILE,
}

impl Logger for PosixLogger {

}

impl PosixLogger {

    pub const MAX_THREAD_ID_SIZE: usize = 32;
    pub const STACK_BUFFER_SIZE:  usize = 512;
    pub const MAX_HEADER_PREFIX:  usize = 28;

    /// Creates a logger that writes to the given
    /// file.
    /// 
    /// The PosixLogger instance takes ownership of
    /// the file handle.
    ///
    pub fn new(fp: *mut libc::FILE) -> Self {
        assert!(!fp.is_null(), "PosixLogger::new: fp must not be null");
        info!("PosixLogger::new: creating logger");
        PosixLogger { fp }
    }
}

impl Drop for PosixLogger {
    fn drop(&mut self) {
        trace!("PosixLogger::drop: closing log file");
        unsafe {
            if !self.fp.is_null() {
                libc::fclose(self.fp as *mut libc::FILE);
            }
        }
    }
}

#[cfg(test)]
mod posix_logger_core_behavior_tests {
    use super::*;

    fn create_tmp_file() -> *mut libc::FILE {
        info!("create_tmp_file: requesting temporary FILE* from libc::tmpfile");
        unsafe {
            let fp = libc::tmpfile();
            if fp.is_null() {
                error!("create_tmp_file: libc::tmpfile returned null");
            } else {
                debug!("create_tmp_file: successfully created temporary file");
            }
            assert!(!fp.is_null(), "tmpfile should not return null");
            fp
        }
    }

    #[traced_test]
    fn posix_logger_new_with_valid_file_pointer_succeeds() {
        info!("posix_logger_new_with_valid_file_pointer_succeeds: start");
        let fp = create_tmp_file();
        let logger = PosixLogger::new(fp);

        trace!(
            "posix_logger_new_with_valid_file_pointer_succeeds: logger.fp()={:?}",
            logger.fp()
        );
        assert!(!logger.fp().is_null(), "Logger should store non-null FILE*");
        assert_eq!(
            *logger.fp(),
            fp as *const libc::FILE,
            "Logger should retain original FILE* value"
        );

        debug!(
            "posix_logger_new_with_valid_file_pointer_succeeds: dropping logger to close FILE*"
        );
        drop(logger);
        info!("posix_logger_new_with_valid_file_pointer_succeeds: end");
    }

    #[traced_test]
    fn posix_logger_new_panics_on_null_file_pointer() {
        info!("posix_logger_new_panics_on_null_file_pointer: start");
        let result = std::panic::catch_unwind(|| {
            trace!("posix_logger_new_panics_on_null_file_pointer: constructing with null fp");
            let _logger = PosixLogger::new(std::ptr::null_mut());
            let _ = _logger.fp(); // avoid unused variable warning
        });

        if result.is_ok() {
            error!("posix_logger_new_panics_on_null_file_pointer: construction did not panic");
        }
        assert!(
            result.is_err(),
            "PosixLogger::new is expected to panic when given a null FILE*"
        );
        info!("posix_logger_new_panics_on_null_file_pointer: end");
    }

    #[traced_test]
    fn posix_logger_constants_have_expected_relationships() {
        info!("posix_logger_constants_have_expected_relationships: start");
        debug!(
            "constants: MAX_THREAD_ID_SIZE={} STACK_BUFFER_SIZE={} MAX_HEADER_PREFIX={}",
            PosixLogger::MAX_THREAD_ID_SIZE,
            PosixLogger::STACK_BUFFER_SIZE,
            PosixLogger::MAX_HEADER_PREFIX
        );

        assert!(
            PosixLogger::MAX_THREAD_ID_SIZE > 0,
            "MAX_THREAD_ID_SIZE should be positive"
        );
        assert!(
            PosixLogger::STACK_BUFFER_SIZE > PosixLogger::MAX_HEADER_PREFIX,
            "STACK_BUFFER_SIZE should be larger than MAX_HEADER_PREFIX"
        );
        assert!(
            PosixLogger::STACK_BUFFER_SIZE > PosixLogger::MAX_THREAD_ID_SIZE,
            "STACK_BUFFER_SIZE should be larger than MAX_THREAD_ID_SIZE"
        );

        trace!("posix_logger_constants_have_expected_relationships: completed assertions");
        info!("posix_logger_constants_have_expected_relationships: end");
    }

    #[traced_test]
    fn posix_logger_drop_closes_file_without_panic() {
        info!("posix_logger_drop_closes_file_without_panic: start");
        {
            let fp = create_tmp_file();
            let _logger = PosixLogger::new(fp);
            debug!(
                "posix_logger_drop_closes_file_without_panic: created logger with fp={:?}",
                _logger.fp()
            );
            // When `_logger` goes out of scope, Drop should close the FILE* safely.
            trace!("posix_logger_drop_closes_file_without_panic: leaving inner scope");
        }
        info!("posix_logger_drop_closes_file_without_panic: end");
    }
}
