// ---------------- [ File: bitcoinleveldb-util/src/no_destructor.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/no_destructor.h]

/**
  | Wraps an instance whose destructor is never called.
  | 
  | This is intended for use with function-level
  | static variables or any scenario where you do
  | not want to run the instance's destructor.
  |
  | In C++, this was done using aligned storage
  | and placement-new, ignoring destruction. In
  | Rust, we use `MaybeUninit` and never drop it.
  | Access it via `get()` to retrieve a raw pointer
  | to the stored instance.
  */
#[derive(Debug)]
pub struct NoDestructor<InstanceType> {
    /// We store the instance in a MaybeUninit,
    /// ensuring we never call its destructor.
    instance_storage: MaybeUninit<InstanceType>,
}

impl<InstanceType> NoDestructor<InstanceType> {
    /**
      | Create a `NoDestructor` by fully constructing
      | the `instance` first, then storing it in
      | `instance_storage`.  The destructor for
      | this instance will never be run.
      |
      */
    pub fn new(instance: InstanceType) -> Self {
        info!("NoDestructor::new invoked");
        let storage = MaybeUninit::new(instance);
        Self {
            instance_storage: storage,
        }
    }

    /**
      | Return a mutable pointer to the stored instance.
      | 
      | The caller must be careful if they dereference
      | this pointer, as we do not manage any lifetime
      | or aliasing constraints beyond basic Rust
      | memory safety.
      */
    pub fn get(&self) -> *mut InstanceType {
        trace!("NoDestructor::get returning pointer to the stored instance");
        // We cast `*const InstanceType` -> `*mut InstanceType`
        // to replicate the mutable pointer from C++.
        self.instance_storage.as_ptr() as *mut InstanceType
    }
}


