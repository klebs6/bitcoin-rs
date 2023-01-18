crate::ix!();

impl StartScheduledTasks for PeerManager {

    fn start_scheduled_tasks(
        self:      Arc<Self>, 
        scheduler: Arc<Mutex<Scheduler>>)  
    {
        // Stale tip checking and peer eviction
        // are on two different timers, but we
        // don't want them to get out of sync due
        // to drift in the scheduler, so we
        // combine them in one function and
        // schedule at the quicker (peer-eviction)
        // timer.
        const_assert!(
            EXTRA_PEER_CHECK_INTERVAL.whole_seconds() < STALE_CHECK_INTERVAL.whole_seconds(),
        ); //peer eviction timer should be less than stale tip check timer


        let cself      = self.clone();

        scheduler.lock().schedule_every(

            Box::new(move || {

                let cself = cself.clone();

                cself.check_for_stale_tip_and_evict_peers();
            }), 

            EXTRA_PEER_CHECK_INTERVAL
        );

        // schedule next run for 10-15 minutes in
        // the future
        let delta: Duration 
        = Duration::minutes(10) + get_random_duration(Duration::minutes(5));

        let cself      = self.clone();
        let cscheduler = scheduler.clone();

        scheduler.lock().schedule_from_now(
            Box::new(move || {

                let cself      = cself.clone();
                let cscheduler = cscheduler.clone();

                cself.reattempt_initial_broadcast(cscheduler);
            }), 
            delta
        );
    }
}
