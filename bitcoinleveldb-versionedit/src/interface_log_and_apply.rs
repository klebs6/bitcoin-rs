// ---------------- [ File: bitcoinleveldb-versionedit/src/interface_log_and_apply.rs ]
crate::ix!();

pub trait VersionEditLogAndApply {
    
    /**
      | Apply *edit to the current version to form
      | a new descriptor that is both saved to
      | persistent state and installed as the new
      | current version.  Will release *mu while
      | actually writing to the file.
      |
      | REQUIRES: *mu is held on entry.
      |
      | REQUIRES: no other thread concurrently calls
      | LogAndApply()
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mu)]
    fn log_and_apply(&mut self, 
        edit: *mut VersionEdit,
        mu:   *mut RawMutex) -> Status;
}
