// ---------------- [ File: bitcoin-scheduler/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{are_threads_servicing_queue}
x!{inner}
x!{repeat}
x!{schedule}
x!{schedule_every}
x!{schedule_from_now}
x!{scheduler}
x!{scheduler_get_queue_info}
x!{scheduler_interface}
x!{scheduler_mock_forward}
x!{service_queue}
x!{should_stop}
x!{single_threaded_scheduler_client}
x!{single_threaded_scheduler_client_inner}
x!{stop}
x!{stop_when_drained}
x!{drop}
x!{mock}
x!{time}

#[cfg(test)]
mod imports_reexport_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[traced_test]
    fn imports_module_exposes_tracing_and_common_time_types_expected_by_the_crate() {
        tracing::trace!("trace macro available");
        tracing::debug!("debug macro available");
        tracing::info!("info macro available");
        tracing::warn!("warn macro available");
        tracing::error!("error macro available");

        let _d = Duration::milliseconds(1);
        let _t = TimePoint::from_std_instant(std::time::Instant::now());

        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        COUNTER.fetch_add(1, Ordering::SeqCst);

        assert!(COUNTER.load(Ordering::SeqCst) >= 1);
    }
}

#[cfg(test)]
mod bitcoin_scheduler_public_surface_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn crate_level_surface_supports_scheduling_mock_forward_and_draining_end_to_end() {
        let mut scheduler = scheduler_for_unit_testing();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_cb = counter.clone();

        let delta = Duration::seconds(2);

        ScheduleFromNow::schedule_from_now(
            &mut scheduler,
            Box::new(move || {
                trace!("crate-level end-to-end scheduled task ran");
                counter_cb.fetch_add(1, Ordering::SeqCst);
            }),
            delta,
        );

        SchedulerMockForward::mock_forward(&mut scheduler, delta);

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[traced_test]
    fn crate_level_surface_supports_repeat_free_function_basic_contract() {
        let mut scheduler = scheduler_for_unit_testing();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_cb = counter.clone();

        repeat(
            &mut scheduler,
            Box::new(move || {
                let n = counter_cb.fetch_add(1, Ordering::SeqCst) + 1;
                trace!(n, "repeat callback executed");
            }),
            Duration::milliseconds(0),
        );

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
