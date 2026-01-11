// ---------------- [ File: bitcoin-scheduler/src/service_queue.rs ]
crate::ix!();

impl ServiceQueue for Scheduler {
    /// Services the queue 'forever'. Should be run in a thread.
    ///
    fn service_queue(&mut self) {
        trace!("Scheduler::service_queue: starting");

        // Mirrors the C++ try/catch decrement behavior by ensuring we decrement
        // n_threads_servicing_queue on unwind as well.
        struct ThreadServicingQueueCounter {
            scheduler: *const Scheduler,
            armed: bool,
        }

        impl ThreadServicingQueueCounter {
            fn disarm(&mut self) {
                self.armed = false;
            }
        }

        impl Drop for ThreadServicingQueueCounter {
            fn drop(&mut self) {
                if !self.armed {
                    return;
                }

                unsafe {
                    let scheduler = &*self.scheduler;
                    let mutex_ref = scheduler.new_task_mutex().borrow();
                    let mut inner = mutex_ref.lock();

                    {
                        let n_threads = inner.n_threads_servicing_queue_mut();
                        *n_threads -= 1;
                    }

                    trace!(
                        n_threads_servicing_queue = inner.n_threads_servicing_queue(),
                        "Scheduler::service_queue: decremented n_threads_servicing_queue during unwind"
                    );
                }
            }
        }

        #[cfg(target_os = "linux")]
        set_syscall_sandbox_policy(SyscallSandboxPolicy::SCHEDULER);

        let scheduler_ptr: *const Scheduler = self as *const Scheduler;
        let mut counter = ThreadServicingQueueCounter {
            scheduler: scheduler_ptr,
            armed: true,
        };

        let mutex_ref = self.new_task_mutex().borrow();
        let mut lock = mutex_ref.lock();

        {
            let n_threads = lock.n_threads_servicing_queue_mut();
            *n_threads += 1;
        }

        trace!(
            n_threads_servicing_queue = lock.n_threads_servicing_queue(),
            "Scheduler::service_queue: entered servicing loop"
        );

        // newTaskMutex is locked throughout this loop EXCEPT
        // when the thread is waiting or when the user's function
        // is called.
        while !(lock.stop_requested() || (lock.stop_when_empty() && lock.task_queue().is_empty())) {
            while !(lock.stop_requested()
                || (lock.stop_when_empty() && lock.task_queue().is_empty()))
                && lock.task_queue().is_empty()
            {
                // Wait until there is something to do.
                trace!("Scheduler::service_queue: waiting for new task");
                self.new_task_scheduled().wait(&mut lock);
            }

            // Wait until either there is a new task, or until
            // the time of the first item on the queue:
            while !(lock.stop_requested()
                || (lock.stop_when_empty() && lock.task_queue().is_empty()))
                && !lock.task_queue().is_empty()
            {
                let time_to_wait_for = match lock.task_queue().iter().next() {
                    Some((t, _)) => *t,
                    None => break,
                };

                if self
                    .new_task_scheduled()
                    .wait_until(&mut lock, time_to_wait_for.into())
                    .timed_out()
                {
                    break; // Exit loop after timeout, it means we reached the time of the event
                }
            }

            // If there are multiple threads, the queue can empty while we're waiting (another
            // thread may service the task we were waiting on).
            if (lock.stop_requested() || (lock.stop_when_empty() && lock.task_queue().is_empty()))
                || lock.task_queue().is_empty()
            {
                continue;
            }

            let (scheduled_time, f) = {
                let (t, mut tasks) = lock
                    .task_queue_mut()
                    .pop_first()
                    .expect("task_queue non-empty but pop_first() returned None");

                assert!(
                    !tasks.is_empty(),
                    "task_queue contained an empty task vector for a scheduled time"
                );

                let f = tasks.remove(0);

                if !tasks.is_empty() {
                    lock.task_queue_mut().insert(t, tasks);
                }

                (t, f)
            };

            {
                // Unlock before calling f, so it can reschedule itself or another task
                // without deadlocking:
                trace!(t = ?scheduled_time, "Scheduler::service_queue: invoking scheduled task");

                drop(lock);

                let mut f = f;
                f();

                lock = mutex_ref.lock();
            }
        }

        {
            let n_threads = lock.n_threads_servicing_queue_mut();
            *n_threads -= 1;
        }
        counter.disarm();

        trace!(
            n_threads_servicing_queue = lock.n_threads_servicing_queue(),
            "Scheduler::service_queue: exiting"
        );

        self.new_task_scheduled().notify_one();
    }
}

#[cfg(test)]
mod service_queue_contract_suite {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex as StdMutex};

    #[traced_test]
    fn service_queue_executes_tasks_in_chronological_order() {
        let mut scheduler = scheduler_for_unit_testing();

        let log: Arc<StdMutex<Vec<u32>>> = Arc::new(StdMutex::new(Vec::new()));

        let log_a = log.clone();
        let log_b = log.clone();
        let log_c = log.clone();

        let t_earliest = TimePoint::from_std_instant(std::time::Instant::now())
            - time_point::Duration::from_secs(3);
        let t_middle = TimePoint::from_std_instant(std::time::Instant::now())
            - time_point::Duration::from_secs(2);
        let t_latest = TimePoint::from_std_instant(std::time::Instant::now())
            - time_point::Duration::from_secs(1);

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("executing earliest task");
                log_a.lock().unwrap().push(1);
            }),
            t_earliest,
        );

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("executing latest task");
                log_c.lock().unwrap().push(3);
            }),
            t_latest,
        );

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("executing middle task");
                log_b.lock().unwrap().push(2);
            }),
            t_middle,
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        let observed = log.lock().unwrap().clone();
        assert_eq!(observed, vec![1, 2, 3]);
        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));
    }

    #[traced_test]
    fn service_queue_unlocks_before_invoking_user_task_so_task_can_schedule_another_task() {
        let mut scheduler = scheduler_for_unit_testing();

        let scheduler_ptr: *mut Scheduler = &mut scheduler;

        let saw_second_task = Arc::new(AtomicBool::new(false));
        let saw_second_task_cb = saw_second_task.clone();

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("task A running; scheduling task B");

                let saw_second_task_inner = saw_second_task_cb.clone();

                unsafe {
                    Schedule::schedule(
                        &mut *scheduler_ptr,
                        Box::new(move || {
                            trace!("task B running");
                            saw_second_task_inner.store(true, Ordering::SeqCst);
                        }),
                        TimePoint::from_std_instant(std::time::Instant::now())
                            - time_point::Duration::from_secs(1),
                    );
                }
            }),
            TimePoint::from_std_instant(std::time::Instant::now()) - time_point::Duration::from_secs(1),
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert!(saw_second_task.load(Ordering::SeqCst));
    }

    #[traced_test]
    fn service_queue_decrements_thread_count_on_panic_and_propagates_panic() {
        let mut scheduler = scheduler_for_unit_testing();

        Schedule::schedule(
            &mut scheduler,
            Box::new(|| {
                error!("intentional panic from scheduled task");
                panic!("intentional scheduled task panic");
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        let result = catch_unwind(AssertUnwindSafe(|| {
            ServiceQueue::service_queue(&mut scheduler);
        }));

        assert!(result.is_err());
        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));
    }

    #[traced_test]
    fn service_queue_exits_immediately_when_stop_requested_and_does_not_execute_pending_tasks() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran = Arc::new(AtomicUsize::new(0));
        let ran_cb = ran.clone();

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("this task should not run when stop_requested is set");
                ran_cb.fetch_add(1, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        Stop::stop(&mut scheduler);

        ServiceQueue::service_queue(&mut scheduler);

        assert_eq!(ran.load(Ordering::SeqCst), 0);

        let mut first = TimePoint::from_std_instant(std::time::Instant::now());
        let mut last = TimePoint::from_std_instant(std::time::Instant::now());
        let remaining = SchedulerGetQueueInfo::get_queue_info(&scheduler, &mut first, &mut last);

        assert_eq!(remaining, 1);
    }

    #[traced_test]
    fn service_queue_exits_after_draining_when_stop_when_drained_set_and_queue_becomes_empty() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran = Arc::new(AtomicUsize::new(0));
        let ran_a = ran.clone();
        let ran_b = ran.clone();

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("drain task A executed");
                ran_a.fetch_add(1, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(2),
        );

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("drain task B executed");
                ran_b.fetch_add(1, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert_eq!(ran.load(Ordering::SeqCst), 2);

        let mut first = TimePoint::from_std_instant(std::time::Instant::now());
        let mut last = TimePoint::from_std_instant(std::time::Instant::now());
        let remaining = SchedulerGetQueueInfo::get_queue_info(&scheduler, &mut first, &mut last);

        assert_eq!(remaining, 0);
    }
}
