crate::ix!();

pub fn wait_until_wake(
    condvar:      &Condvar,
    lock:         &mut AmoOuterReadGuard<ConnmanMsgProc>,
    timeout_time: Instant,
) -> WaitTimedOut {

    assert!(lock.is_some());

    let mtx: Mutex<()> = Default::default();

    while !WaitTimedOut(
        lock.as_ref().unwrap().msg_proc_wake.load(atomic::Ordering::Relaxed)
    ).timed_out() {

        let guard = mtx.lock();

        if RwLockReadGuard::unlocked(lock, || {
            // Move the guard in so it gets unlocked before we re-lock g
            let mut guard = guard;

            condvar.wait_until(&mut guard, timeout_time.into_inner())

        }).timed_out() == true {

            return WaitTimedOut(
                lock.as_ref().unwrap().msg_proc_wake.load(atomic::Ordering::Relaxed)
            );
        }
    }

    WaitTimedOut(true)
}

pub fn subroutine_thread_open_added_connections(connman: Arc<Connman>)  {

    set_syscall_sandbox_policy(SyscallSandboxPolicy::NET_ADD_CONNECTION);

    while true {

        let mut grant: SemaphoreGrant = SemaphoreGrant::new(
            connman.sem_addnode.clone(), 
            None
        );

        let info: Vec::<AddedNodeInfo> = connman.get_added_node_info();
        let mut tried: bool = false;

        for info in info.iter() {

            if !info.connected {

                if !grant.try_acquire() {

                    // If we've used up our
                    // semaphore and need
                    // a new one, let's not
                    // wait here since while
                    // we are waiting the
                    // addednodeinfo state
                    // might change.
                    break;
                }

                tried = true;

                let addr: Address = Address::new(
                    Service::default(), 
                    ServiceFlags::NODE_NONE
                );

                connman.open_network_connection(
                    &addr, 
                    false, 
                    Some(&mut grant), 
                    info.str_added_node.as_ptr(), 
                    ConnectionType::MANUAL
                );

                if !connman.interrupt_net.get_mut().sleep_for(
                    Duration::milliseconds(500)
                ) {
                    return;
                }
            }
        }

        // Retry every 60 seconds if
        // a connection was attempted,
        // otherwise two seconds
        if !connman.interrupt_net.get_mut().sleep_for(
            Duration::seconds(
                match tried {
                    true   => 60,
                    false  => 2
                }
            )
        ) {
            return;
        }
    }
}
