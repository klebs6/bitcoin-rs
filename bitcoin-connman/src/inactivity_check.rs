// ---------------- [ File: bitcoin-connman/src/inactivity_check.rs ]
crate::ix!();

impl Connman {

    /**
      | Return true if we should disconnect
      | the peer for failing an inactivity check.
      |
      */
    pub fn should_run_inactivity_checks(&self, 
        node: &AmoWriteGuard<Box<dyn NodeInterface>>,
        now:  OffsetDateTime) -> bool {

        if let Some(time_connected) = node.n_time_connected() {

            time_connected + *self.peer_connect_timeout.get() < now

        } else {
            false
        }
    }

    /**
      | Return true if the peer is inactive and
      | should be disconnected.
      |
      */
    pub fn inactivity_check(&self, node: &AmoWriteGuard<Box<dyn NodeInterface>>) -> bool {

        // Use non-mockable system time (otherwise these
        // timers will pop when we use setmocktime
        // in the tests).
        let now = OffsetDateTime::now_utc();

        if !self.should_run_inactivity_checks(&node,now) {
            return false;
        }

        if node.n_last_recv() == None 
        || node.n_last_send() == None {

            log_print!(
                bc_log::NET, 
                "socket no message in first {} seconds, {} {} peer={}\n", 
                peer_connect_timeout, 
                node.n_last_recv != 0, 
                node.n_last_send != 0, 
                node.get_id()
            );

            return true;
        }

        if now > node.n_last_send().unwrap() + TIMEOUT_INTERVAL {

            log_print!(
                bc_log::NET, 
                "socket sending timeout: {}s peer={}\n", 
                now - node.n_last_send,
                node.get_id()
            );

            return true;
        }

        if now > node.n_last_recv().unwrap() + TIMEOUT_INTERVAL {

            log_print!(
                bc_log::NET, 
                "socket receive timeout: {}s peer={}\n", 
                now - node.n_last_recv, 
                node.get_id()
            );

            return true;
        }

        if !node.successfully_connected() {

            log_print!(
                bc_log::NET, 
                "version handshake timeout peer={}\n", 
                node.get_id()
            );

            return true;
        }

        false
    }
}
