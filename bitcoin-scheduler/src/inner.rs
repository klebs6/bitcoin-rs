// ---------------- [ File: bitcoin-scheduler/src/inner.rs ]
crate::ix!();

pub type SchedulerFunction = Box<dyn FnMut() -> ()>;

pub struct SchedulerInner {
    task_queue:                MultiMap<TimePoint,SchedulerFunction>,
    n_threads_servicing_queue: i32, // default = { 0 }
    stop_requested:            bool, // default = { false }
    stop_when_empty:           bool, // default = { false }
}
