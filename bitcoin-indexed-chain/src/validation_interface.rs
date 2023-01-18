/**
  | Alternate registration functions that release
  | a shared_ptr after the last notification is
  | sent. These are useful for race-free cleanup,
  | since unregistration is nonblocking and can
  | return before the last notification is
  | processed.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/validationinterface.cpp]

/**
  | Register subscriber
  |
  */
pub fn register_shared_validation_interface(callbacks: Arc<dyn ValidationInterface>)  {
    
    todo!();
        /*
            // Each connection captures the shared_ptr to ensure that each callback is
        // executed before the subscriber is destroyed. For more details see #18338.
        g_signals.m_internals->Register(std::move(callbacks));
        */
}

/**
  | Register subscriber
  |
  */
pub fn register_validation_interface(callbacks: Arc<Mutex<dyn ValidationInterface>>)  {
    
    todo!();
        /*
            // Create a shared_ptr with a no-op deleter - CValidationInterface lifecycle
        // is managed by the caller.
        RegisterSharedValidationInterface({callbacks, [](CValidationInterface*){}});
        */
}

/**
  | Unregister subscriber
  |
  */
pub fn unregister_shared_validation_interface(callbacks: Arc<dyn ValidationInterface>)  {
    
    todo!();
        /*
            UnregisterValidationInterface(callbacks.get());
        */
}

/**
  | Unregister subscriber. DEPRECATED.
  | This is not safe to use when the RPC server
  | or main message handler thread is running.
  |
  */
pub fn unregister_validation_interface(callbacks: Arc<Mutex<dyn ValidationInterface>>)  {
    
    todo!();
        /*
            if (g_signals.m_internals) {
            g_signals.m_internals->Unregister(callbacks);
        }
        */
}

/**
  | Unregister all subscribers
  |
  */
pub fn unregister_all_validation_interfaces()  {
    
    todo!();
        /*
            if (!g_signals.m_internals) {
            return;
        }
        g_signals.m_internals->Clear();
        */
}

/**
  | Pushes a function to callback onto the
  | notification queue, guaranteeing
  | any callbacks generated prior to now
  | are finished when the function is called.
  | 
  | Be very careful blocking on func to be
  | called if any locks are held - validation
  | interface clients may not be able to
  | make progress as they often wait for
  | things like cs_main, so blocking until
  | func is called with cs_main will result
  | in a deadlock (that DEBUG_LOCKORDER
  | will miss).
  |
  */
pub fn call_function_in_validation_interface_queue(func: fn() -> ())  {
    
    todo!();
        /*
            g_signals.m_internals->m_schedulerClient.AddToProcessQueue(std::move(func));
        */
}

/**
  | This is a synonym for the following,
  | which asserts certain locks are not
  | held:
  | 
  | std::promise<c_void> promise;
  | CallFunctionInValidationInterfaceQueue([&promise] {
  |     promise.set_value();
  | });
  | promise.get_future().wait();
  |
  */
#[LOCKS_EXCLUDED(cs_main)]
pub fn sync_with_validation_interface_queue()  {
    
    todo!();
        /*
            AssertLockNotHeld(cs_main);
        // Block until the validation queue drains
        std::promise<c_void> promise;
        CallFunctionInValidationInterfaceQueue([&promise] {
            promise.set_value();
        });
        promise.get_future().wait();
        */
}
