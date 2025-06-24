// ---------------- [ File: bitcoin-sync/src/types.rs ]
crate::ix!();

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

pub type DebugLock<'a, MutexArg> =
    UniqueLock<'a, RemoveReference<RemovePointer<MutexArg>>>;

