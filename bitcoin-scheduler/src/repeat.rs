// ---------------- [ File: bitcoin-scheduler/src/repeat.rs ]
crate::ix!();

pub fn repeat(
        s:     &mut Scheduler,
        f:     SchedulerFunction,
        delta: Duration /* millis */)  {
    
    todo!();
        /*
            f();
        s.scheduleFromNow([=, &s] { Repeat(s, f, delta); }, delta);
        */
}
