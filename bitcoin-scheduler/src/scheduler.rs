// ---------------- [ File: bitcoin-scheduler/src/scheduler.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/scheduler.h]
//-------------------------------------------[.cpp/bitcoin/src/scheduler.cpp]

/// Simple class for background tasks that
/// should be run periodically or once "after
/// a while"
/// 
/// Usage:
/// 
/// -----------
/// @code
/// 
/// CScheduler* s = new CScheduler();
/// s->scheduleFromNow(doSomething, std::chrono::milliseconds{11}); // Assuming a: c_void doSomething() { }
/// s->scheduleFromNow([=] { this->func(argument); }, std::chrono::milliseconds{3});
/// std::thread* t = new std::thread([&] { s->serviceQueue(); });
///  
/// ... then at program shutdown, make sure to call stop() to clean up the thread(s) running serviceQueue:
/// s->stop();
/// t->join();
/// delete t;
/// delete s; // Must be done after thread is interrupted/joined.
///
pub struct Scheduler {
    service_thread:     Thread,
    new_task_mutex:     RefCell<Mutex<SchedulerInner>>,
    new_task_scheduled: Condvar,
}

pub trait SchedulerInterface
: ScheduleFromNow
+ Stop
+ StopWhenDrained
+ ShouldStop
+ ServiceQueue
+ Schedule
+ SchedulerMockForward
+ ScheduleEvery
+ SchedulerGetQueueInfo
+ AreThreadsServicingQueue
{}

impl SchedulerInterface for Scheduler {}

impl Drop for Scheduler {
    fn drop(&mut self) {
        todo!();
        /*
            assert(nThreadsServicingQueue == 0);
        if (stopWhenEmpty) assert(taskQueue.empty());
        */
    }
}
