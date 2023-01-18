crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/thread.h]
//-------------------------------------------[.cpp/bitcoin/src/util/thread.cpp]

/**
  | A wrapper for do-something-once thread
  | functions.
  |
  */
pub fn trace_thread<F>(
        thread_name: &str,
        thread_func: F)

    where F: FnOnce() -> ()
{
    
    todo!();
        /*
            ThreadRename(thread_name);
        try {
            LogPrintf("%s thread start\n", thread_name);
            thread_func();
            LogPrintf("%s thread exit\n", thread_name);
        } catch (const std::exception& e) {
            PrintExceptionContinue(&e, thread_name);
            throw;
        } catch (...) {
            PrintExceptionContinue(nullptr, thread_name);
            throw;
        }
        */
}

#[macro_export] macro_rules! launch_traced_thread {
    ($name:expr,$closure:expr) => {

        /*
        let builder = std::thread::Builder::new()
            .name($name.into());

        let join_handle: std::thread::JoinHandle<_> = builder.spawn(|| {
            trace_thread($name, $closure)
        });

        join_handle.ok()
        */

        std::thread::Builder::new()
            .name($name.into())
            .spawn(|| {
                trace_thread($name, $closure)
            })
            .ok()
    }
}

pub struct WaitTimedOut(pub bool);

impl WaitTimedOut {
    pub fn timed_out(&self) -> bool { self.0 }
}

pub fn wait_until<T: ?Sized, CondvarPredicate>(
    condvar:      &Condvar,
    lock:         &mut MutexGuard<'_, T>,
    timeout_time: Instant,
    mut stop_waiting: CondvarPredicate,
) -> WaitTimedOut 
where CondvarPredicate: FnMut() -> WaitTimedOut {

    while !stop_waiting().timed_out() {
        if condvar.wait_until(lock, timeout_time.into_inner()).timed_out() == true {
            return stop_waiting();
        }
    }

    WaitTimedOut(true)
}
