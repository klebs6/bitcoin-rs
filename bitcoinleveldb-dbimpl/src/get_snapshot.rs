// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_snapshot.rs ]
crate::ix!();

impl DBGetSnapshot for DBImpl {

    fn get_snapshot(&mut self) -> Box<dyn Snapshot> { 
        todo!(); 
        /*
        self.mutex.lock();
        let snap = self
            .snapshots
            .new(unsafe { (*self.versions).last_sequence() });
        self.mutex.unlock();
        snap
                                                          */
    }
}
