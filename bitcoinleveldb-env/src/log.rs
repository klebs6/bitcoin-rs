// ---------------- [ File: bitcoinleveldb-env/src/log.rs ]
crate::ix!();

/**
  | Log the specified data to *info_log
  | if info_log is non-null.
  |
  */
pub fn log(
        info_log: Rc<RefCell<dyn Logger>>,
        format:   *const u8,
        args:     &[&str])  {
    
    todo!();
        /*
            if (info_log != nullptr) {
        va_list ap;
        va_start(ap, format);
        info_log->Logv(format, ap);
        va_end(ap);
      }
        */
}
