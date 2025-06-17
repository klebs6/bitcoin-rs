// ---------------- [ File: bitcoin-locked-pool/src/locked_pool_manager.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/lockedpool.h]

/**
  | Singleton class to keep track of locked
  | (ie, non-swappable) memory, for use
  | in std::allocator templates.
  | 
  | Some implementations of the STL allocate
  | memory in some constructors (i.e.,
  | see MSVC's vector<T> implementation where
  | it allocates 1 byte of memory in the allocator.)
  | 
  | Due to the unpredictable order of static
  | initializers, we have to make sure the
  | LockedPoolManager instance exists
  | before any other STL-based objects
  | that use secure_allocator are created.
  | So instead of having LockedPoolManager
  | also be static-initialized, it is created
  | on demand.
  |
  */
pub struct LockedPoolManager {
    base: LockedPool,
}

lazy_static!{
    /*
    static LockedPoolManager* _instance;
    LockedPoolManager* LockedPoolManager::_instance = nullptr;
    */
}

impl LockedPoolManager {

    /**
      | Return the current instance, or create
      | it once
      |
      */
    pub fn instance() -> &'static mut LockedPoolManager {
        
        todo!();
        /*
            static std::once_flag init_flag;
            std::call_once(init_flag, LockedPoolManager::CreateInstance);
            return *LockedPoolManager::_instance;
        */
    }
    
    pub fn new(_allocator_in: Box<dyn LockedPageAllocator>) -> Self {
    
        todo!();
        /*
            : LockedPool(std::move(allocator_in), &LockedPoolManager::LockingFailed)
        */
    }
    
    /**
      | Called when locking fails, warn the
      | user here
      |
      */
    pub fn locking_failed(&mut self) -> bool {
        
        todo!();
        /*
            // TODO: log something but how? without including util.h
        return true;
        */
    }
    
    /**
      | Create a new LockedPoolManager specialized
      | to the OS
      |
      */
    pub fn create_instance(&mut self)  {
        
        todo!();
        /*
            // Using a local static instance guarantees that the object is initialized
        // when it's first needed and also deinitialized after all objects that use
        // it are done with it.  I can think of one unlikely scenario where we may
        // have a static deinitialization order/problem, but the check in
        // LockedPoolManagerBase's destructor helps us detect if that ever happens.
    #ifdef WIN32
        std::unique_ptr<LockedPageAllocator> allocator(new Win32LockedPageAllocator());
    #else
        std::unique_ptr<LockedPageAllocator> allocator(new PosixLockedPageAllocator());
    #endif
        static LockedPoolManager instance(std::move(allocator));
        LockedPoolManager::_instance = &instance;
        */
    }
}
