// ---------------- [ File: bitcoin-connman/src/thread_message_handler.rs ]
crate::ix!();

pub fn subroutine_thread_message_handler(connman: Arc<Connman>)  {

    set_syscall_sandbox_policy(SyscallSandboxPolicy::MESSAGE_HANDLER);

    let mut rng = FastRandomContext::default();

    while !connman
        .flag_interrupt_msg_proc
        .load(atomic::Ordering::Relaxed) {

        let mut nodes_copy: Vec<Amo<Box<dyn NodeInterface>>> = vec![];

        {
            let mut guard = connman.cs_v_nodes.get();

            nodes_copy = guard.nodes.clone();

            for pnode in nodes_copy.iter() {

                let mut node = pnode.get_mut();

                node.add_ref();
            }
        }

        let mut more_work: bool = false;

        // Randomize the order in which we
        // process messages from/to our peers.
        //
        // This prevents attacks in which an
        // attacker exploits having multiple
        // consecutive connections in the
        // vNodes list.
        nodes_copy.shuffle(&mut rng);

        for pnode in nodes_copy.iter() {

            let mut node = pnode.get_mut();

            if node.marked_for_disconnect() {
                continue;
            }

            //  Receive messages
            let more_node_work: bool 
            = {
                todo!();

                //this is a problem with the
                //Arc<Self> stuff

                /*
                connman.msgproc
                    .get_mut()
                    .process_messages(
                        &mut node, 
                        &connman.flag_interrupt_msg_proc
                    )
                        */
            };

            more_work |= (more_node_work && !node.send_paused());

            if connman
                .flag_interrupt_msg_proc
                .load(atomic::Ordering::Relaxed) {
                return;
            }

            //  Send messages
            unsafe {

                node.lock_send_processing();

                todo!();
                //this is a problem with the
                //Arc<Self> stuff necessary for
                //cloning self and sending it to
                //the scheduler

                /*
                connman.msgproc
                    .get_mut()
                    .send_messages(pnode.clone());
                */

                node.unlock_send_processing();
            }

            if connman.flag_interrupt_msg_proc
                .load(atomic::Ordering::Relaxed) 
            {
                return;
            }
        }

        {
            let mut guard = connman.cs_v_nodes.get();

            for pnode in nodes_copy.iter() {

                pnode.get_mut().release();
            }
        }

        let mut guard = connman.mutex_msg_proc.getopt();

        if !more_work {

            let timeout_time = Instant::now() + Duration::milliseconds(100);

            wait_until_wake(
                &connman.cond_msg_proc,
                &mut guard, 
                timeout_time, 
            );
        }

        guard.as_ref().unwrap().msg_proc_wake.store(false, atomic::Ordering::Relaxed);
    }
}
