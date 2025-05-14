// ---------------- [ File: bitcoin-peerman/src/set_best_height.rs ]
crate::ix!();

impl SetBestHeight for PeerManager {

    fn set_best_height(&mut self, height: i32)  {
        
        self.best_height.store(height, atomic::Ordering::Relaxed);
    }
}
