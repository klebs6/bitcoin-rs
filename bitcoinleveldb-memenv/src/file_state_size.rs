crate::ix!();

impl FileState {
   
    pub fn size(&self) -> u64 {
        trace!("FileState::size: querying current file size");
        let blocks_ref = self.blocks_mutex.borrow();
        let guard = match blocks_ref.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("FileState::size: blocks_mutex poisoned; recovering");
                poisoned.into_inner()
            }
        };

        let size = guard.size;
        debug!("FileState::size: returning size={}", size);
        size
    }
}
