// ---------------- [ File: bitcoinleveldb-dbiterstate/src/iterstate.rs ]
crate::ix!();

pub struct IterState {
    mu:      *const Mutex<IterStateInner>,
}

pub struct IterStateInner {
    version: *const Version,
    mem:     *const MemTable,
    imm:     *const MemTable,
}

impl IterState {

    pub fn new(
        mutex:   *mut parking_lot::RawMutex,
        mem:     *mut MemTable,
        imm:     *mut MemTable,
        version: *mut Version) -> Self {
    
        todo!();
        /*
        : mu(mutex),
        : version(version),
        : mem(mem),
        : imm(imm),

        
        */
    }
}

pub fn cleanup_iterator_state(
        arg1: *mut c_void,
        arg2: *mut c_void)  {
    
    todo!();
        /*
            IterState* state = reinterpret_cast<IterState*>(arg1);
      state->mu->Lock();
      state->mem->Unref();
      if (state->imm != nullptr) state->imm->Unref();
      state->version->Unref();
      state->mu->Unlock();
      delete state;
        */
}
