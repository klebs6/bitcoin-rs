// ---------------- [ File: bitcoin-peerman/src/reattempt_initial_broadcast.rs ]
crate::ix!();

pub trait ReattemptInitialBroadcast {

    fn reattempt_initial_broadcast(
        self:      Arc<Self>, 
        scheduler: Arc<Mutex<Scheduler>>
    );
}

impl ReattemptInitialBroadcast for PeerManager {

    /**
      | Retrieve unbroadcast transactions
      | from the mempool and reattempt sending
      | to peers
      |
      */
    fn reattempt_initial_broadcast(
        self:      Arc<Self>, 
        scheduler: Arc<Mutex<Scheduler>>)
    {
        let mut mempool = self.mempool.get_mut();

        let unbroadcast_txids: HashSet::<u256> 
        = mempool.get_unbroadcast_txs();

        for txid in unbroadcast_txids.iter() {

            let tx: TransactionRef = mempool.get(txid);

            if tx.is_some() {

                let mut guard = CS_MAIN.lock();

                self.clone().relay_transaction(
                    txid, 
                    tx.get().get_witness_hash()
                );

            } else {

                mempool.remove_unbroadcast_tx(txid, Some(true));
            }
        }

        /**
          | Schedule next run for 10-15 minutes in
          | the future.
          |
          | We add randomness on every cycle to
          | avoid the possibility of P2P
          | fingerprinting.
          */
        let delta: Duration = Duration::minutes(10) + get_random_duration(Duration::minutes(5));

        let cself = self.clone();
        let cscheduler = scheduler.clone();

        let closure = Box::new(
            move || { 

                let cself      = cself.clone();
                let cscheduler = cscheduler.clone();

                cself.reattempt_initial_broadcast(cscheduler); 
            }
        );

        scheduler.lock().schedule_from_now(closure, delta);
    }
}
