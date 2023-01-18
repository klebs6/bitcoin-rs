crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/sync_tests.cpp]

pub fn test_potential_dead_lock_detected<MutexType>(
        mutex1: &mut MutexType,
        mutex2: &mut MutexType)  {

    todo!();
        /*
            {
            LOCK2(mutex1, mutex2);
        }
        BOOST_CHECK(LockStackEmpty());
        bool error_thrown = false;
        try {
            LOCK2(mutex2, mutex1);
        } catch (const std::logic_error& e) {
            BOOST_CHECK_EQUAL(e.what(), "potential deadlock detected: mutex1 -> mutex2 -> mutex1");
            error_thrown = true;
        }
        BOOST_CHECK(LockStackEmpty());
        #ifdef DEBUG_LOCKORDER
        BOOST_CHECK(error_thrown);
        #else
        BOOST_CHECK(!error_thrown);
        #endif
        */
}

#[cfg(DEBUG_LOCKORDER)]
pub fn test_double_lock2<MutexType>(m: &mut MutexType)  {

    todo!();
        /*
            ENTER_CRITICAL_SECTION(m);
        LEAVE_CRITICAL_SECTION(m);
        */
}

#[cfg(DEBUG_LOCKORDER)]
pub fn test_double_lock<MutexType>(should_throw: bool)  {

    todo!();
        /*
            const bool prev = g_debug_lockorder_abort;
        g_debug_lockorder_abort = false;

        MutexType m;
        ENTER_CRITICAL_SECTION(m);
        if (should_throw) {
            BOOST_CHECK_EXCEPTION(TestDoubleLock2(m), std::logic_error,
                                  HasReason("double lock detected"));
        } else {
            BOOST_CHECK_NO_THROW(TestDoubleLock2(m));
        }
        LEAVE_CRITICAL_SECTION(m);

        BOOST_CHECK(LockStackEmpty());

        g_debug_lockorder_abort = prev;
        */
}

#[NO_THREAD_SAFETY_ANALYSIS]
pub fn test_inconsistent_lock_order_detected<MutexType>(
        mutex1: &mut MutexType,
        mutex2: &mut MutexType)  {

    todo!();
        /*
            ENTER_CRITICAL_SECTION(mutex1);
        ENTER_CRITICAL_SECTION(mutex2);
    #ifdef DEBUG_LOCKORDER
        BOOST_CHECK_EXCEPTION(LEAVE_CRITICAL_SECTION(mutex1), std::logic_error, HasReason("mutex1 was not most recent critical section locked"));
    #endif // DEBUG_LOCKORDER
        LEAVE_CRITICAL_SECTION(mutex2);
        LEAVE_CRITICAL_SECTION(mutex1);
        BOOST_CHECK(LockStackEmpty());
        */
}

#[cfg(test)]
pub mod sync_tests {

    #[test] fn potential_deadlock_detected() {
        todo!();
        /*
        
            #ifdef DEBUG_LOCKORDER
            bool prev = g_debug_lockorder_abort;
            g_debug_lockorder_abort = false;
            #endif

            RecursiveMutex rmutex1, rmutex2;
            TestPotentialDeadLockDetected(rmutex1, rmutex2);
            // The second test ensures that lock tracking data have not been broken by exception.
            TestPotentialDeadLockDetected(rmutex1, rmutex2);

            Mutex mutex1, mutex2;
            TestPotentialDeadLockDetected(mutex1, mutex2);
            // The second test ensures that lock tracking data have not been broken by exception.
            TestPotentialDeadLockDetected(mutex1, mutex2);

            #ifdef DEBUG_LOCKORDER
            g_debug_lockorder_abort = prev;
            #endif

        */
    }

    /**
      | Double lock would produce an undefined
      | behavior. Thus, we only do that if
      | 
      | DEBUG_LOCKORDER is activated to detect
      | it. We don't want non-DEBUG_LOCKORDER
      | build to produce tests that exhibit
      | known undefined behavior.
      |
      */
    #[cfg(DEBUG_LOCKORDER)]
    #[test] fn double_lock_mutex() {
        todo!();
        /*
        
            TestDoubleLock<Mutex>(true /* should throw */);

        */
    }

    #[cfg(DEBUG_LOCKORDER)]
    #[test] fn double_lock_recursive_mutex() {
        todo!();
        /*
        
            TestDoubleLock<RecursiveMutex>(false /* should not throw */);

        */
    }

    #[test] fn inconsistent_lock_order_detected() {
        todo!();
        /*
        
        #ifdef DEBUG_LOCKORDER
            bool prev = g_debug_lockorder_abort;
            g_debug_lockorder_abort = false;
        #endif // DEBUG_LOCKORDER

            RecursiveMutex rmutex1, rmutex2;
            TestInconsistentLockOrderDetected(rmutex1, rmutex2);
            // By checking lock order consistency (CheckLastCritical) before any unlocking (LeaveCritical)
            // the lock tracking data must not have been broken by exception.
            TestInconsistentLockOrderDetected(rmutex1, rmutex2);

            Mutex mutex1, mutex2;
            TestInconsistentLockOrderDetected(mutex1, mutex2);
            // By checking lock order consistency (CheckLastCritical) before any unlocking (LeaveCritical)
            // the lock tracking data must not have been broken by exception.
            TestInconsistentLockOrderDetected(mutex1, mutex2);

        #ifdef DEBUG_LOCKORDER
            g_debug_lockorder_abort = prev;
        #endif // DEBUG_LOCKORDER

        */
    }
}
