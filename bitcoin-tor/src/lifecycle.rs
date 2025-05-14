// ---------------- [ File: bitcoin-tor/src/lifecycle.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/torcontrol.h]

//-------------------------------------------[.cpp/bitcoin/src/torcontrol.cpp]

/****** Thread ********/
lazy_static!{
    /*
    static struct event_base *gBase;
    static std::thread torControlThread;
    */
}

pub fn tor_control_thread(onion_service_target: Service)  {
    
    todo!();
        /*
            SetSyscallSandboxPolicy(SyscallSandboxPolicy::TOR_CONTROL);
        TorController ctrl(gBase, gArgs.GetArg("-torcontrol", DEFAULT_TOR_CONTROL), onion_service_target);

        event_base_dispatch(gBase);
        */
}

pub fn start_tor_control(onion_service_target: Service)  {
    
    todo!();
        /*
            assert(!gBase);
    #ifdef WIN32
        evthread_use_windows_threads();
    #else
        evthread_use_pthreads();
    #endif
        gBase = event_base_new();
        if (!gBase) {
            LogPrintf("tor: Unable to create event_base\n");
            return;
        }

        torControlThread = std::thread(&util::TraceThread, "torcontrol", [onion_service_target] {
            TorControlThread(onion_service_target);
        });
        */
}

pub fn interrupt_tor_control()  {
    
    todo!();
        /*
            if (gBase) {
            LogPrintf("tor: Thread interrupt\n");
            event_base_once(gBase, -1, EV_TIMEOUT, [](evutil_socket_t, short, c_void*) {
                event_base_loopbreak(gBase);
            }, nullptr, nullptr);
        }
        */
}

pub fn stop_tor_control()  {
    
    todo!();
        /*
            if (gBase) {
            torControlThread.join();
            event_base_free(gBase);
            gBase = nullptr;
        }
        */
}

pub fn default_onion_service_target() -> Service {
    
    todo!();
        /*
            struct in_addr onion_service_target;
        onion_service_target.s_addr = htonl(INADDR_LOOPBACK);
        return {onion_service_target, BaseParams().OnionServiceTargetPort()};
        */
}
