// ---------------- [ File: bitcoin-ipc/src/protocol.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/ipc/protocol.h]
//-------------------------------------------[.cpp/bitcoin/src/interfaces/ipc.h]

/**
  | IPC protocol interface for calling IPC methods
  | over sockets.
  |
  | There may be different implementations of this
  | interface for different IPC protocols
  | (e.g. Cap'n Proto, gRPC, JSON-RPC, or custom
  | protocols).
  */
pub trait Protocol: 
    Connect 
    + Serve 
    + AddCleanup 
    + Context { }

/**
  | IPC process interface for spawning bitcoin
  | processes and serving requests in processes
  | that have been spawned.
  |
  | There will be different implementations of
  | this interface depending on the platform
  | (e.g. unix, windows).
  */
pub trait ProcessInterface: 
    Spawn 
    + WaitSpawned 
    + CheckSpawned { }

/**
  | Interface providing access to
  | interprocess-communication (IPC)
  | functionality. The IPC implementation is
  | responsible for establishing connections
  | between a controlling process and a process
  | being controlled.  When a connection is
  | established, the process being controlled
  | returns an interfaces::Init pointer to the
  | controlling process, which the controlling
  | process can use to get access to other
  | interfaces and functionality.
  |
  | When spawning a new process, the steps are:
  |
  | 1. The controlling process calls
  |    interfaces::Ipc::spawnProcess(), which
  |    calls ipc::Process::spawn(), which spawns
  |    a new process and returns a socketpair file
  |    descriptor for communicating with it.
  |    interfaces::Ipc::spawnProcess() then calls
  |    ipc::Protocol::connect() passing the
  |    socketpair descriptor, which returns
  |    a local proxy interfaces::Init
  |    implementation calling remote
  |    interfaces::Init methods.
  |
  | 2. The spawned process calls
  |    interfaces::Ipc::startSpawnProcess(), which
  |    calls ipc::Process::checkSpawned() to read
  |    command line arguments and determine
  |    whether it is a spawned process and what
  |    socketpair file descriptor it should
  |    use. It then calls ipc::Protocol::serve()
  |    to handle incoming requests from the
  |    socketpair and invoke interfaces::Init
  |    interface methods, and exit when the socket
  |    is closed.
  |
  | 3. The controlling process calls local proxy
  |    interfaces::Init object methods to make
  |    other proxy objects calling other remote
  |    interfaces. It can also destroy the initial
  |    interfaces::Init object to close the
  |    connection and shut down the spawned
  |    process.
  */
pub trait IpcInterface: 
    SpawnProcess 
    + StartSpawnedProcess 
    + Context 
    + AddCleanup 
{
    /**
      | Add cleanup callback to remote interface
      | that will run when the interface is
      | deleted.
      */
    fn add_cleanup<Interface>(&mut self, 
        iface:   &mut Interface,
        cleanup: fn() -> ())  {
    
        todo!();
        /*
            addCleanup(typeid(Interface), &iface, std::move(cleanup));
        */
    }
}

///--------------------------
pub trait Spawn {

    /**
      | Spawn process and return socket file
      | descriptor for communicating with
      | it.
      |
      */
    fn spawn(&mut self, 
            new_exe_name: &String,
            argv0_path:   Box<&Path>,
            pid:          &mut i32) -> i32;
}

pub trait WaitSpawned {

    /**
      | Wait for spawned process to exit and
      | return its exit code.
      |
      */
    fn wait_spawned(&mut self, pid: i32) -> i32;
}

pub trait CheckSpawned {

    /**
      | Parse command line and determine if
      | current process is a spawned child
      | process. If so, return true and a file
      | descriptor for communicating with the
      | parent process.
      */
    fn check_spawned(&mut self, 
            argc: i32,
            argv: &[*mut u8],
            fd:   &mut i32) -> bool;
}

///--------------------------
pub trait Connect {

    /**
      | Return Init interface that forwards
      | requests over given socket descriptor.
      | Socket communication is handled on
      | a background thread.
      */
    fn connect(&mut self, 
            fd:       i32,
            exe_name: *const u8) -> Box<dyn Init>;
}

pub trait Serve {

    /**
      | Handle requests on provided socket
      | descriptor, forwarding them to the
      | provided Init interface. Socket
      | communication is handled on the current
      | thread, and this call blocks until the
      | socket is closed.
      */
    fn serve(&mut self, 
            fd:       i32,
            exe_name: *const u8,
            init:     Rc<RefCell<dyn Init>>);

}

pub trait AddCleanup {

    /**
      | Add cleanup callback to interface that
      | will run when the interface is deleted.
      |
      */
    fn add_cleanup(&mut self, 
            ty:      TypeIndex,
            iface:   *mut c_void,
            cleanup: fn() -> ());
}

///----------------------------------------
pub trait SpawnProcess {

    /**
      | Spawn a child process returning pointer
      | to its Init interface.
      |
      */
    fn spawn_process(&mut self, exe_name: *const u8) -> Box<dyn Init>;
}

pub trait StartSpawnedProcess {

    /**
      | If this is a spawned process, block and
      | handle requests from the parent process by
      | forwarding them to this process's Init
      | interface, then return true. If this is
      | not a spawned child process, return false.
      */
    fn start_spawned_process(&mut self, 
        argc:        i32,
        argv:        &[*mut u8],
        exit_status: &mut i32) -> bool;
}

pub trait Context {

    /**
      | IPC context struct accessor (see struct
      | definition for more description).
      |
      */
    fn context(&mut self) -> Rc<RefCell<IpcContext>>;
}
