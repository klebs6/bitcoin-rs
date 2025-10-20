// ---------------- [ File: bitcoin-ipc/src/ipc.rs ]
crate::ix!();

use crate::protocol::Protocol;

//-------------------------------------------[.cpp/bitcoin/src/ipc/interfaces.cpp]

pub struct Ipc {
    exe_name:      *const u8,
    process_argv0: *const u8,
    init:          Rc<RefCell<dyn Init>>,
    protocol:      Box<dyn Protocol>,
    process:       Box<dyn ProcessInterface>,
}

impl IpcInterface for Ipc { }

impl SpawnProcess for Ipc { 

    fn spawn_process(&mut self, new_exe_name: *const u8) -> Box<dyn Init> {
        
        todo!();
        /*
            int pid;
            int fd = m_process->spawn(new_exe_name, m_process_argv0, pid);
            LogPrint(::LogFlags::IPC, "Process %s pid %i launched\n", new_exe_name, pid);
            auto init = m_protocol->connect(fd, m_exe_name);
            Ipc::addCleanup(*init, [this, new_exe_name, pid] {
                int status = m_process->waitSpawned(pid);
                LogPrint(::LogFlags::IPC, "Process %s pid %i exited with status %i\n", new_exe_name, pid, status);
            });
            return init;
        */
    }
}

impl StartSpawnedProcess for Ipc { 

    fn start_spawned_process(&mut self, 
        argc:        i32,
        argv:        &[*mut u8],
        exit_status: &mut i32) -> bool {
        
        todo!();
        /*
            exit_status = EXIT_FAILURE;
            int32_t fd = -1;
            if (!m_process->checkSpawned(argc, argv, fd)) {
                return false;
            }
            m_protocol->serve(fd, m_exe_name, m_init);
            exit_status = EXIT_SUCCESS;
            return true;
        */
    }
}

impl AddCleanup for Ipc { 

    fn add_cleanup(&mut self, 
        ty:      TypeIndex,
        iface:   *mut c_void,
        cleanup: fn() -> ())  {
        
        todo!();
        /*
            m_protocol->addCleanup(type, iface, std::move(cleanup));
        */
    }
}

impl Context for Ipc { 

    fn context(&mut self) -> Rc<RefCell<IpcContext>> {
        
        todo!();
        /*
            return m_protocol->context();
        */
    }
}

impl Ipc {
    
    pub fn new(
        exe_name:      *const u8,
        process_argv0: *const u8,
        init:          Rc<RefCell<dyn Init>>) -> Self {
    
        todo!();
        /*
           : m_exe_name(exe_name), m_process_argv0(process_argv0), m_init(init),
           m_protocol(ipc::capnp::MakeCapnpProtocol()), m_process(ipc::MakeProcess())
           */
    }
}

/**
  | Return implementation of Ipc interface.
  |
  */
pub fn make_ipc(
        exe_name:      *const u8,
        process_argv0: *const u8,
        init:          Rc<RefCell<dyn Init>>) -> Box<Ipc> {
    
    todo!();
        /*
            return std::make_unique<ipc::Ipc>(exe_name, process_argv0, init);
        */
}
