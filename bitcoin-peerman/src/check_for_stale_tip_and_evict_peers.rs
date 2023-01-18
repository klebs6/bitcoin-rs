crate::ix!();

impl CheckForStaleTipAndEvictPeers for PeerManager {

    fn check_for_stale_tip_and_evict_peers(self: Arc<Self>)  {
        
        let mut guard = CS_MAIN.lock();

        let now: OffsetDateTime = get_datetime(); /* time_in_seconds */

        self.clone().evict_extra_outbound_peers(now);

        if now > self.stale_tip_check_time.load(atomic::Ordering::Relaxed) {

            // Check whether our tip is stale, and
            // if so, allow using an extra
            // outbound peer
            if !IMPORTING.load(atomic::Ordering::Relaxed)
            && !REINDEX.load(atomic::Ordering::Relaxed) 
            && self.connman.get().get_network_active() 
            && self.connman.get().get_use_addrman_outgoing() 
            && self.clone().tip_may_be_stale() {

                log_printf!(
                    "Potential stale tip detected, will try using extra outbound peer (last tip update: {} seconds ago)\n", 
                    now - self.last_tip_update.get().unwrap_or(0)
                );

                self.connman.get_mut().set_try_new_outbound_peer(true);

            } else {

                if self.connman.get().get_try_new_outbound_peer() {
                    self.connman.get_mut().set_try_new_outbound_peer(false);
                }
            }

            self.stale_tip_check_time.store(now + STALE_CHECK_INTERVAL, atomic::Ordering::Relaxed);
        }

        if !self.initial_sync_finished.load(atomic::Ordering::Relaxed) 
        && self.clone().can_direct_fetch() 
        {
            self.connman.get_mut().start_extra_block_relay_peers();
            self.initial_sync_finished.store(true, atomic::Ordering::Relaxed);
        }
    }
}
