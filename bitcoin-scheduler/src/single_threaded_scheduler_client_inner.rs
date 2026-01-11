// ---------------- [ File: bitcoin-scheduler/src/single_threaded_scheduler_client_inner.rs ]
crate::ix!();

#[derive(Getters, MutGetters, Setters, CopyGetters)]
pub struct SingleThreadedSchedulerClientInner {
    #[getset(get = "pub", get_mut = "pub(crate)")]
    callbacks_pending: LinkedList<fn() -> ()>,

    #[getset(get_copy = "pub", get_mut = "pub(crate)", set = "pub(crate)")]
    are_callbacks_running: bool, // default = false
}

impl Default for SingleThreadedSchedulerClientInner {
    fn default() -> Self {
        Self {
            callbacks_pending: LinkedList::new(),
            are_callbacks_running: false,
        }
    }
}

#[cfg(test)]
mod single_threaded_scheduler_client_inner_contract_suite {
    use super::*;

    #[traced_test]
    fn client_inner_default_matches_commentary_defaults() {
        let inner = SingleThreadedSchedulerClientInner::default();

        assert!(inner.callbacks_pending().is_empty());
        assert!(!inner.are_callbacks_running());
    }
}
