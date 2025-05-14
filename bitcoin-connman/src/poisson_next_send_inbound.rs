// ---------------- [ File: bitcoin-connman/src/poisson_next_send_inbound.rs ]
crate::ix!();

impl Connman {

    /**
      | Attempts to obfuscate tx time through
      | exponentially distributed emitting.
      | 
      | Works assuming that a single interval
      | is used.
      | 
      | Variable intervals will result in privacy
      | decrease.
      |
      */
    pub fn poisson_next_send_inbound(&mut self, 
        now:              OffsetDateTime /* micros */,
        average_interval: Duration /* seconds */) -> OffsetDateTime /* micros */ {
        
        if self.next_send_inv_to_incoming.load(atomic::Ordering::Relaxed) < now {

            // If this function were called from
            // multiple threads simultaneously it
            // would possible that both update the
            // next send variable, and return
            // a different result to their caller.
            //
            // This is not possible in practice as
            // only the net processing thread
            // invokes this function.
            self.next_send_inv_to_incoming = Atomic::new(
                poisson_next_send(now,average_interval)
            );
        }

        self.next_send_inv_to_incoming.load(atomic::Ordering::Relaxed)
    }
}
