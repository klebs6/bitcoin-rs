crate::ix!();

impl SendPings for PeerManager {

    fn send_pings(&mut self)  {
        
        let mut peer_map = self.peer_map.get_mut();

        for (ref k,ref mut v) in peer_map.iter_mut() {

            if v.is_some() {
                v.get().ping_queued.store(true, atomic::Ordering::Relaxed);
            }
        }
    }
}
