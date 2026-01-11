// ---------------- [ File: bitcoin-scheduler/src/mock.rs ]
crate::ix!();

#[cfg(test)]
pub(crate) fn scheduler_for_unit_testing() -> Scheduler {
    trace!("scheduler_for_unit_testing: constructing a fresh Scheduler instance");
    Scheduler::new()
}
