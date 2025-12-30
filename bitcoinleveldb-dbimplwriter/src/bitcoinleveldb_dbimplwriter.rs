// ---------------- [ File: bitcoinleveldb-dbimplwriter/src/bitcoinleveldb_dbimplwriter.rs ]
crate::ix!();

/**
  | Information kept for every waiting
  | writer
  |
  */
pub struct DBImplWriter {
    status: Status,
    batch:  *mut WriteBatch,
    sync:   bool,
    done:   bool,
    cv:     Condvar,
}

impl DBImplWriter {

    pub fn new(mu: *mut parking_lot::RawMutex) -> Self {
        todo!();
        /*
            : batch(nullptr), sync(false), done(false), cv(mu)
        */
    }
}
