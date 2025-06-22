crate::ix!();

/**
  | Template mixin that adds -Wthread-safety
  | locking annotations and lock order
  | checking to a subset of the mutex API.
  |
  */
#[LOCKABLE]
pub struct AnnotatedMixin<PARENT> {
    base: PARENT,
}

impl<PARENT> Drop for AnnotatedMixin<PARENT> {
    fn drop(&mut self) {
        todo!();
        /*
            DeleteLock((c_void*)this);
        */
    }
}

pub mod annotated_mixin {
    pub type UniqueLock<PARENT> = super::UniqueLock<PARENT>;
}

impl<PARENT> Not for AnnotatedMixin<PARENT> {
    type Output = AnnotatedMixin<PARENT>;

    /**
      | For negative capabilities in the Clang
      | Thread Safety Analysis.
      |
      | A negative requirement uses the
      | EXCLUSIVE_LOCKS_REQUIRED attribute, in
      | conjunction with the ! operator, to
      | indicate that a mutex should not be held.
      */
    #[inline] fn not(self) -> Self::Output {
        todo!();
        /*
            return *this;
        */
    }
}

impl<PARENT> AnnotatedMixin<PARENT> {

    #[EXCLUSIVE_LOCK_FUNCTION()]
    pub fn lock(&mut self)  {
        
        todo!();
        /*
            PARENT::lock();
        */
    }

    #[UNLOCK_FUNCTION()]
    pub fn unlock(&mut self)  {
        
        todo!();
        /*
            PARENT::unlock();
        */
    }

    #[EXCLUSIVE_TRYLOCK_FUNCTION(true)]
    pub fn try_lock(&mut self) -> bool {
        
        todo!();
        /*
            return PARENT::try_lock();
        */
    }
}
