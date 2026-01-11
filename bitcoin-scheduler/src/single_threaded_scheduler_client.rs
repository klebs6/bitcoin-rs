// ---------------- [ File: bitcoin-scheduler/src/single_threaded_scheduler_client.rs ]
crate::ix!();

/// Class used by CScheduler clients which may schedule multiple jobs which are required to be run
/// serially.
/// 
/// Jobs may not be run on the same thread, but no two jobs will be executed at the same time and
/// memory will be release-acquire consistent (the scheduler will internally do an acquire before
/// invoking a callback as well as a release at the end).
/// 
/// In practice this means that a callback
/// 
/// B() will be able to observe all of the effects of callback A() which executed before it.
///
#[derive(Getters)]
#[getset(get = "pub")]
pub struct SingleThreadedSchedulerClient {
    pscheduler: *mut Scheduler,
    cs_callbacks_pending:
        parking_lot::ReentrantMutex<std::cell::RefCell<SingleThreadedSchedulerClientInner>>,
}

impl SingleThreadedSchedulerClient {
    pub fn new(pscheduler_in: *mut Scheduler) -> Self {
        trace!(
            pscheduler_is_null = pscheduler_in.is_null(),
            "SingleThreadedSchedulerClient::new"
        );

        Self {
            pscheduler: pscheduler_in,
            cs_callbacks_pending: parking_lot::ReentrantMutex::new(std::cell::RefCell::new(
                SingleThreadedSchedulerClientInner::default(),
            )),
        }
    }

    pub fn maybe_schedule_process_queue(&mut self) {
        trace!("SingleThreadedSchedulerClient::maybe_schedule_process_queue: begin");

        {
            let guard = self.cs_callbacks_pending.lock();
            let inner = guard.borrow();

            // Try to avoid scheduling too many copies here, but if we
            // accidentally have two ProcessQueue's scheduled at once its
            // not a big deal.
            if inner.are_callbacks_running() {
                trace!(
                    pending = inner.callbacks_pending().len(),
                    "SingleThreadedSchedulerClient::maybe_schedule_process_queue: callbacks already running"
                );
                return;
            }
            if inner.callbacks_pending().is_empty() {
                trace!("SingleThreadedSchedulerClient::maybe_schedule_process_queue: no callbacks pending");
                return;
            }
        }

        assert!(!self.pscheduler.is_null());

        let self_ptr: *mut SingleThreadedSchedulerClient =
            self as *mut SingleThreadedSchedulerClient;

        unsafe {
            let scheduler = &mut *self.pscheduler;
            scheduler.schedule(
                Box::new(move || unsafe {
                    (*self_ptr).process_queue();
                }),
                TimePoint::from_std_instant(std::time::Instant::now()),
            );
        }

        trace!("SingleThreadedSchedulerClient::maybe_schedule_process_queue: scheduled ProcessQueue");

        /*
            {
            LOCK(m_cs_callbacks_pending);
            // Try to avoid scheduling too many copies here, but if we
            // accidentally have two ProcessQueue's scheduled at once its
            // not a big deal.
            if (m_are_callbacks_running) return;
            if (m_callbacks_pending.empty()) return;
        }
        m_pscheduler->schedule(std::bind(&SingleThreadedSchedulerClient::ProcessQueue, this), std::chrono::system_clock::now());
        */
    }

    pub fn process_queue(&mut self) {
        trace!("SingleThreadedSchedulerClient::process_queue: begin");

        let callback: fn() -> () = {
            let guard = self.cs_callbacks_pending.lock();
            let mut inner = guard.borrow_mut();

            if inner.are_callbacks_running() {
                trace!("SingleThreadedSchedulerClient::process_queue: callbacks already running");
                return;
            }
            if inner.callbacks_pending().is_empty() {
                trace!("SingleThreadedSchedulerClient::process_queue: no callbacks pending");
                return;
            }

            inner.set_are_callbacks_running(true);

            inner
                .callbacks_pending_mut()
                .pop_front()
                .expect("callbacks_pending non-empty but pop_front() returned None")
        };

        // RAII the setting of are_callbacks_running and calling maybe_schedule_process_queue
        // to ensure both happen safely even if callback() panics.
        struct CallbacksRunningGuard {
            instance: *mut SingleThreadedSchedulerClient,
        }

        impl Drop for CallbacksRunningGuard {
            fn drop(&mut self) {
                unsafe {
                    let instance = &mut *self.instance;

                    {
                        let guard = instance.cs_callbacks_pending.lock();
                        let mut inner = guard.borrow_mut();
                        inner.set_are_callbacks_running(false);
                    }

                    instance.maybe_schedule_process_queue();
                }
            }
        }

        let _callbacks_running_guard = CallbacksRunningGuard {
            instance: self as *mut SingleThreadedSchedulerClient,
        };

        trace!("SingleThreadedSchedulerClient::process_queue: invoking callback");
        callback();

        trace!("SingleThreadedSchedulerClient::process_queue: callback completed");

        /*
            std::function<c_void()> callback;
        {
            LOCK(m_cs_callbacks_pending);
            if (m_are_callbacks_running) return;
            if (m_callbacks_pending.empty()) return;
            m_are_callbacks_running = true;

            callback = std::move(m_callbacks_pending.front());
            m_callbacks_pending.pop_front();
        }

        // RAII the setting of fCallbacksRunning and calling MaybeScheduleProcessQueue
        // to ensure both happen safely even if callback() throws.
        struct RAIICallbacksRunning {
            SingleThreadedSchedulerClient* instance;
            explicit RAIICallbacksRunning(SingleThreadedSchedulerClient* _instance) : instance(_instance) {}
            ~RAIICallbacksRunning()
            {
                {
                    LOCK(instance->m_cs_callbacks_pending);
                    instance->m_are_callbacks_running = false;
                }
                instance->MaybeScheduleProcessQueue();
            }
        } raiicallbacksrunning(this);

        callback();
        */
    }

    /**
      | Add a callback to be executed. Callbacks
      | are executed serially and memory is
      | release-acquire consistent between
      | callback executions.
      |
      | Practically, this means that callbacks
      | can behave as if they are executed in
      | order by a single thread.
      |
      */
    pub fn add_to_process_queue(&mut self, func: fn() -> ()) {
        trace!("SingleThreadedSchedulerClient::add_to_process_queue: begin");

        assert!(!self.pscheduler.is_null());

        {
            let guard = self.cs_callbacks_pending.lock();
            let mut inner = guard.borrow_mut();

            let pending = {
                let list = inner.callbacks_pending_mut();
                list.push_back(func);
                list.len()
            };

            trace!(
                pending,
                "SingleThreadedSchedulerClient::add_to_process_queue: queued callback"
            );
        }

        self.maybe_schedule_process_queue();
    }

    /**
      | Processes all remaining queue members
      | on the calling thread, blocking until
      | queue is empty
      |
      | Must be called after the CScheduler
      | has no remaining processing threads!
      |
      */
    pub fn empty_queue(&mut self) {
        trace!("SingleThreadedSchedulerClient::empty_queue: begin");

        assert!(!self.pscheduler.is_null());

        unsafe {
            assert!(!(*self.pscheduler).are_threads_servicing_queue());
        }

        let mut should_continue = true;

        while should_continue {
            self.process_queue();

            let guard = self.cs_callbacks_pending.lock();
            let inner = guard.borrow();
            should_continue = !inner.callbacks_pending().is_empty();
        }

        trace!("SingleThreadedSchedulerClient::empty_queue: done");

        /*
            assert(!m_pscheduler->AreThreadsServicingQueue());
        bool should_continue = true;
        while (should_continue) {
            ProcessQueue();
            LOCK(m_cs_callbacks_pending);
            should_continue = !m_callbacks_pending.empty();
        }
        */
    }

    pub fn callbacks_pending(&mut self) -> usize {
        let guard = self.cs_callbacks_pending.lock();
        let inner = guard.borrow();
        let result = inner.callbacks_pending().len();
        trace!(result, "SingleThreadedSchedulerClient::callbacks_pending");
        result
    }
}


#[cfg(test)]
mod single_threaded_scheduler_client_contract_suite {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::ptr;

    thread_local! {
        static CALLBACK_ORDER: RefCell<Vec<u32>> = RefCell::new(Vec::new());
        static CLIENT_PTR: Cell<*mut SingleThreadedSchedulerClient> = Cell::new(ptr::null_mut());
    }

    fn clear_callback_order_for_current_test_thread() {
        CALLBACK_ORDER.with(|v| {
            v.borrow_mut().clear();
        });
    }

    fn take_callback_order_for_current_test_thread() -> Vec<u32> {
        CALLBACK_ORDER.with(|v| std::mem::take(&mut *v.borrow_mut()))
    }

    fn record_callback_step(id: u32) {
        CALLBACK_ORDER.with(|v| {
            v.borrow_mut().push(id);
        });
        trace!(id, "recorded callback step");
    }

    fn callback_one_records() {
        record_callback_step(1);
    }

    fn callback_two_records() {
        record_callback_step(2);
    }

    fn callback_three_records() {
        record_callback_step(3);
    }

    fn callback_reentrant_process_queue_attempt() {
        record_callback_step(9);

        CLIENT_PTR.with(|cell| {
            let ptr = cell.get();
            if !ptr.is_null() {
                trace!("attempting reentrant process_queue()");
                unsafe {
                    (*ptr).process_queue();
                }
                trace!("returned from reentrant process_queue() attempt");
            }
        });

        record_callback_step(10);
    }

    fn callback_panics_intentionally() {
        record_callback_step(99);
        error!("intentional panic inside SingleThreadedSchedulerClient callback");
        panic!("intentional callback panic");
    }

    #[traced_test]
    fn client_new_starts_with_zero_callbacks_pending() {
        clear_callback_order_for_current_test_thread();

        let mut scheduler = scheduler_for_unit_testing();
        let mut client = SingleThreadedSchedulerClient::new(&mut scheduler as *mut Scheduler);

        assert_eq!(client.callbacks_pending(), 0);
    }

    #[traced_test]
    fn maybe_schedule_process_queue_does_not_schedule_when_no_callbacks_pending() {
        clear_callback_order_for_current_test_thread();

        let mut scheduler = scheduler_for_unit_testing();
        let mut client = SingleThreadedSchedulerClient::new(&mut scheduler as *mut Scheduler);

        client.maybe_schedule_process_queue();

        let mut first = TimePoint::from_std_instant(std::time::Instant::now());
        let mut last = TimePoint::from_std_instant(std::time::Instant::now());
        let size = SchedulerGetQueueInfo::get_queue_info(&scheduler, &mut first, &mut last);

        assert_eq!(size, 0);
    }

    #[traced_test]
    fn add_to_process_queue_executes_callbacks_serially_and_in_fifo_order_via_scheduler() {
        clear_callback_order_for_current_test_thread();

        let mut scheduler = scheduler_for_unit_testing();
        let mut client = SingleThreadedSchedulerClient::new(&mut scheduler as *mut Scheduler);

        client.add_to_process_queue(callback_one_records);
        client.add_to_process_queue(callback_two_records);
        client.add_to_process_queue(callback_three_records);

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        let observed = take_callback_order_for_current_test_thread();
        assert_eq!(observed, vec![1, 2, 3]);

        assert_eq!(client.callbacks_pending(), 0);
    }

    #[traced_test]
    fn process_queue_is_reentrancy_safe_via_are_callbacks_running_guard() {
        clear_callback_order_for_current_test_thread();

        let mut scheduler = scheduler_for_unit_testing();
        let mut client = SingleThreadedSchedulerClient::new(&mut scheduler as *mut Scheduler);

        CLIENT_PTR.with(|cell| {
            cell.set(&mut client as *mut SingleThreadedSchedulerClient);
        });

        client.add_to_process_queue(callback_reentrant_process_queue_attempt);

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        let observed = take_callback_order_for_current_test_thread();
        assert_eq!(observed, vec![9, 10]);

        CLIENT_PTR.with(|cell| cell.set(ptr::null_mut()));
    }

    #[traced_test]
    fn callbacks_running_guard_clears_state_even_if_callback_panics_allows_subsequent_processing() {
        clear_callback_order_for_current_test_thread();

        let mut scheduler = scheduler_for_unit_testing();
        let mut client = SingleThreadedSchedulerClient::new(&mut scheduler as *mut Scheduler);

        client.add_to_process_queue(callback_panics_intentionally);
        client.add_to_process_queue(callback_one_records);

        let result = catch_unwind(AssertUnwindSafe(|| {
            ServiceQueue::service_queue(&mut scheduler);
        }));
        assert!(result.is_err());

        client.process_queue();

        let observed = take_callback_order_for_current_test_thread();
        assert_eq!(observed, vec![99, 1]);

        assert_eq!(client.callbacks_pending(), 0);
    }

    #[traced_test]
    fn add_to_process_queue_panics_when_scheduler_pointer_is_null() {
        clear_callback_order_for_current_test_thread();

        let mut client = SingleThreadedSchedulerClient::new(ptr::null_mut());

        let result = catch_unwind(AssertUnwindSafe(|| {
            client.add_to_process_queue(callback_one_records);
        }));

        assert!(result.is_err());
    }

    #[traced_test]
    fn empty_queue_processes_all_callbacks_on_calling_thread_without_scheduler_threads() {
        clear_callback_order_for_current_test_thread();

        let mut scheduler = scheduler_for_unit_testing();
        let mut client = SingleThreadedSchedulerClient::new(&mut scheduler as *mut Scheduler);

        client.add_to_process_queue(callback_one_records);
        client.add_to_process_queue(callback_two_records);
        client.add_to_process_queue(callback_three_records);

        client.empty_queue();

        let observed = take_callback_order_for_current_test_thread();
        assert_eq!(observed, vec![1, 2, 3]);

        assert_eq!(client.callbacks_pending(), 0);
    }
}
