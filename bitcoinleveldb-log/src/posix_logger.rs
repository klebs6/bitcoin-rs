// ---------------- [ File: bitcoinleveldb-log/src/posix_logger.rs ]
/*!
  | Logger implementation that can be shared
  | by all environments where enough posix
  | functionality is available.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/posix_logger.h]

pub struct PosixLogger {
    fp:   *const libc::FILE,
}

impl Logger for PosixLogger {

}

impl Drop for PosixLogger {
    fn drop(&mut self) {
        todo!();
        /*
            std::fclose(fp_);
        */
    }
}

impl PosixLogger {

    /**
      | Creates a logger that writes to the given
      | file.
      |
      | The PosixLogger instance takes ownership of
      | the file handle.
      */
    pub fn new(fp: *mut libc::FILE) -> Self {
    
        todo!();
        /*
        : fp(fp),

            assert(fp != nullptr);
        */
    }
}
