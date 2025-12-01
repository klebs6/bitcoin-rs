// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env.rs ]
crate::ix!();

pub struct InMemoryEnvInner {
    file_map: InMemoryEnvFileSystem,
}

/**
  | Map from filenames to FileState objects,
  | representing a simple file system.
  |
  */
pub type InMemoryEnvFileSystem = HashMap<String,*mut FileState>;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv.cc]

pub struct InMemoryEnv {
    base:     EnvWrapper,
    mutex:    Mutex<InMemoryEnvInMemoryEnvInner>,
}

impl Drop for InMemoryEnv {

    fn drop(&mut self) {
        trace!("InMemoryEnv::drop: cleaning up in‑memory file system");

        // We have &mut self here, so we can bypass locking with get_mut().
        let inner: &mut InMemoryEnvInner = self.mutex.get_mut();

        let file_count = inner.file_map.len();
        debug!(
            "InMemoryEnv::drop: {} file(s) remain in file_map; unref'ing all",
            file_count
        );

        for (fname, file_ptr) in inner.file_map.drain() {
            debug!(
                "InMemoryEnv::drop: Unref FileState for '{}' (ptr={:?})",
                fname, file_ptr
            );
            unsafe {
                FileState::unref_raw(file_ptr);
            }
        }
    }
}

impl InMemoryEnv {

    pub fn new(base_env: Rc<RefCell<dyn Env>>) -> Self {
        trace!("InMemoryEnv::new: constructing in‑memory env");
        InMemoryEnv {
            base: EnvWrapper::new(base_env),
            mutex: Mutex::new(InMemoryEnvInner {
                file_map: InMemoryEnvFileSystem::new(),
            }),
        }
    }
}

/// Returns a new environment that stores its data in memory and delegates all
/// non-file-storage tasks to base_env. 
///
/// The caller must delete the result when it is no longer needed.  *base_env
/// must remain live while the result is in use.
///
pub fn new_mem_env(base_env: Rc<RefCell<dyn Env>>) -> Rc<RefCell<dyn Env>> {
    trace!("new_mem_env: creating new InMemoryEnv");
    let env = InMemoryEnv::new(base_env);
    Rc::new(RefCell::new(env))
}

// Implement the Env marker trait for InMemoryEnv so it can be used as a dyn Env.
impl Env for InMemoryEnv {}
