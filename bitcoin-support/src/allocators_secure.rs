// ---------------- [ File: bitcoin-support/src/allocators_secure.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/allocators/secure.h]

/**
  | Allocator that locks its contents from
  | being paged out of memory and clears
  | its contents before deletion.
  |
  */
#[derive(Default,Clone)]
pub struct SecureAllocator {

}

impl SecureAllocator {

    pub fn new(_a: &SecureAllocator) -> Self {
    
        todo!();
        /*
        : base(a),
        */
    }
}

lazy_static!{
    pub static ref SECURE_ALLOCATOR: SecureAllocator = SecureAllocator {};
}

unsafe impl Allocator for SecureAllocator {

    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {

        todo!();

        /*
        fn allocate(&mut self, 
            n:    usize,
            hint: *const c_void) -> *mut T {
            let hint: *const c_void = hint.unwrap_or(0);

            todo!();
            /*
                T* allocation = static_cast<T*>(LockedPoolManager::Instance().alloc(sizeof(T) * n));
                if (!allocation) {
                    throw std::bad_alloc();
                }
                return allocation;
            */
        }
        */
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {

        todo!();

        /*
        pub fn deallocate(&mut self, 
            p: *mut T,
            n: usize)  {
            
            todo!();
            /*
                if (p != nullptr) {
                    memory_cleanse(p, sizeof(T) * n);
                }
                LockedPoolManager::Instance().free(p);
            */
        }
        */
    }
}

/**
  | This is exactly like std::string, but
  | with a custom allocator.
  |
  */
pub type SecureString = Box<String,SecureAllocator>;

impl Default for SecureString {
    fn default() -> Self {
        Box::new_in(String::new(), SecureAllocator::default())
    }
}
