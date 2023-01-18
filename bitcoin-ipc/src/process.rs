crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/ipc/process.h]
//-------------------------------------------[.cpp/bitcoin/src/ipc/process.cpp]

pub struct Process {

}

impl ProcessInterface for Process { }

impl Spawn for Process { 

    fn spawn(&mut self, 
        new_exe_name: &String,
        argv0_path:   Box<&Path>,
        pid:          &mut i32) -> i32 {
        
        todo!();
        /*
            return mp::SpawnProcess(pid, [&](int fd) {
                fs::path path = argv0_path;
                path.remove_filename();
                path /= fs::PathFromString(new_exe_name);
                return std::vector<std::string>{fs::PathToString(path), "-ipcfd", strprintf("%i", fd)};
            });
        */
    }
}

impl WaitSpawned for Process { 

    fn wait_spawned(&mut self, pid: i32) -> i32 {
        
        todo!();
        /*
            return mp::WaitProcess(pid);
        */
    }
}

impl CheckSpawned for Process {

    fn check_spawned(&mut self, 
        argc: i32,
        argv: &[*mut u8],
        fd:   &mut i32) -> bool {
        
        todo!();
        /*
            // If this process was not started with a single -ipcfd argument, it is
            // not a process spawned by the spawn() call above, so return false and
            // do not try to serve requests.
            if (argc != 3 || strcmp(argv[1], "-ipcfd") != 0) {
                return false;
            }
            // If a single -ipcfd argument was provided, return true and get the
            // file descriptor so Protocol::serve() can be called to handle
            // requests from the parent process. The -ipcfd argument is not valid
            // in combination with other arguments because the parent process
            // should be able to control the child process through the IPC protocol
            // without passing information out of band.
            if (!ParseInt32(argv[2], &fd)) {
                throw std::runtime_error(strprintf("Invalid -ipcfd number '%s'", argv[2]));
            }
            return true;
        */
    }
}

/**
  | Constructor for Process interface.
  | Implementation will vary depending
  | on the platform (unix or windows).
  |
  */
pub fn make_process() -> Box<Process> {
    
    todo!();
        /*
            return std::make_unique<Process>();
        */
}
