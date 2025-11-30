// ---------------- [ File: bitcoinleveldb-posixtools/src/flags.rs ]
crate::ix!();

/**
  | Set by
  |
  | EnvPosixTestHelper::set_read_only_mmap_limit()
  | and
  |
  | max_open_files().
  |
  | Stored as an AtomicI32 so it can be safely
  | read from multiple threads.
  */
lazy_static! {
    pub static ref OPEN_READ_ONLY_FILE_LIMIT: std::sync::atomic::AtomicI32 =
        std::sync::atomic::AtomicI32::new(-1);
}

/**
  | Common flags defined for all posix open
  | operations
  |
  */
#[cfg(HAVE_O_CLOEXEC)]
pub const OPEN_BASE_FLAGS: i32 = O_CLOEXEC;

#[cfg(not(HAVE_O_CLOEXEC))]
pub const OPEN_BASE_FLAGS: i32 = 0;

/**
  | Up to 4096 mmap regions for 64-bit binaries;
  | none for 32-bit.
  |
  */
pub const DEFAULT_MMAP_LIMIT: i32 =
    ternary! { size_of::<*mut c_void>() >= 8, 4096, 0 };

/**
  | Can be set using
  |
  | EnvPosixTestHelper::set_read_only_mmap_limit().
  |
  */
lazy_static! {
    pub static ref MMAP_LIMIT: std::sync::atomic::AtomicI32 =
        std::sync::atomic::AtomicI32::new(DEFAULT_MMAP_LIMIT);
}
