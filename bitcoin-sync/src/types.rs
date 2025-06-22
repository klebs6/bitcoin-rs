crate::ix!();

// ---------------- [ File: bitcoin-sync/src/types.rs ]
/**
  | Wrapped mutex: supports recursive
  | locking, but no waiting
  | 
  | TODO: We should move away from using
  | the recursive lock by default.
  |
  */
pub type RecursiveMutex<T> = AnnotatedMixin<parking_lot::ReentrantMutex<T>>;

/**
  | Wrapped mutex: supports waiting but
  | not recursive locking
  |
  */
pub type Mutex = AnnotatedMixin<parking_lot::RawMutex>;

pub type DebugLock<MutexArg> = UniqueLock<RemoveReference<RemovePointer<MutexArg>>>;
