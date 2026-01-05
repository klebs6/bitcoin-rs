// ---------------- [ File: bitcoin-scheduler/src/single_threaded_scheduler_client_inner.rs ]
crate::ix!();

pub struct SingleThreadedSchedulerClientInner {
    callbacks_pending:     LinkedList<fn() -> ()>,
    are_callbacks_running: bool, // default = false
}
