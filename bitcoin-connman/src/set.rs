crate::ix!();

impl Connman {

    /**
      | This allows temporarily exceeding
      | m_max_outbound_full_relay, with the goal of
      | finding a peer that is better than all our
      | current peers.
      */
    pub fn set_try_new_outbound_peer(&mut self, flag: bool)  {
        
        self.try_another_outbound_peer.store(
            flag, 
            atomic::Ordering::Relaxed
        );

        log_print!(
            LogFlags::NET, 
            "net: setting try another outbound peer={}\n", 
            match flag {
                true   => "true",
                false  => "false"
            }
        );
    }

    pub fn set_network_active(&mut self, active: bool)  {
        
        log_printf!("{}: {}\n", func, active);

        if self.network_active.load(atomic::Ordering::Relaxed) == active {
            return;
        }

        self.network_active.store(active, atomic::Ordering::Relaxed);

        let mut ci = self.client_interface.get_mut();

        let network_active 
        = self.network_active.load(atomic::Ordering::Relaxed);

        ci.notify_network_active_changed(network_active);
    }
}
