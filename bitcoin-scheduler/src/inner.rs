// ---------------- [ File: bitcoin-scheduler/src/inner.rs ]
crate::ix!();

pub type SchedulerFunction = Box<dyn FnMut() -> ()>;

#[derive(Getters, MutGetters, Setters, CopyGetters)]
pub struct SchedulerInner {
    #[getset(get = "pub", get_mut = "pub(crate)")]
    task_queue: std::collections::BTreeMap<TimePoint, Vec<SchedulerFunction>>,

    #[getset(get_copy = "pub", get_mut = "pub(crate)")]
    n_threads_servicing_queue: i32, // default = { 0 }

    #[getset(get_copy = "pub", get_mut = "pub(crate)", set = "pub(crate)")]
    stop_requested: bool, // default = { false }

    #[getset(get_copy = "pub", get_mut = "pub(crate)", set = "pub(crate)")]
    stop_when_empty: bool, // default = { false }
}

impl Default for SchedulerInner {
    fn default() -> Self {
        Self {
            task_queue: Default::default(),
            n_threads_servicing_queue: 0,
            stop_requested: false,
            stop_when_empty: false,
        }
    }
}

#[cfg(test)]
mod scheduler_inner_type_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn scheduler_function_type_accepts_fnmut_and_can_be_invoked_multiple_times() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_cb = counter.clone();

        let mut f: SchedulerFunction = Box::new(move || {
            let n = counter_cb.fetch_add(1, Ordering::SeqCst) + 1;
            trace!(n, "SchedulerFunction invoked");
        });

        f();
        f();

        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[traced_test]
    fn scheduler_inner_default_constructs_expected_commentary_values() {
        let inner = SchedulerInner::default();

        assert_eq!(inner.n_threads_servicing_queue(), 0);
        assert!(!inner.stop_requested());
        assert!(!inner.stop_when_empty());
        assert!(inner.task_queue().is_empty());
    }
}
