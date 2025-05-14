// ---------------- [ File: bitcoin-support/src/allocators_zeroafterfree.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/support/allocators/zeroafterfree.h]

#[derive(Default)]
pub struct ZeroAfterFreeAllocator {

}

unsafe impl Allocator for ZeroAfterFreeAllocator {

    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        todo!();
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        todo!();

        /*
        pub fn deallocate(&mut self, 
            p: *mut T,
            n: usize)  {
            
            todo!();
            /*
                if (p != nullptr)
                    memory_cleanse(p, sizeof(T) * n);
                std::allocator<T>::deallocate(p, n);
            */
        }
        */
    }
}

impl ZeroAfterFreeAllocator {

    pub fn new(_a: &ZeroAfterFreeAllocator) -> Self {
    
        todo!();
        /*
        : base(a),
        */
    }
}

/**
  | Byte-vector that clears its contents
  | before deletion.
  |
  */
pub type SerializeData = Vec<u8,ZeroAfterFreeAllocator>;
