crate::ix!();

impl GetSnapshot for DBImpl {

    fn get_snapshot(&mut self) -> Box<dyn Snapshot> {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      return snapshots_.New(versions_->LastSequence());
        */
    }
}
