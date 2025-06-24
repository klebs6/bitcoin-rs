// ---------------- [ File: bitcoin-locked-pool/src/config.rs ]
crate::ix!();

/// Size of one arena of locked memory. This is a compromise.
/// 
/// Do not set this too low, as managing many arenas will increase allocation and deallocation
/// overhead. Setting it too high allocates more locked memory from the OS than strictly necessary.
/// 
pub const LOCKED_POOL_ARENA_SIZE: usize = 256 * 1024;

/// Chunk alignment. 
///
/// Another compromise.
///
/// Setting this too high will waste memory, setting it too low will facilitate fragmentation.
/// 
pub const LOCKED_POOL_ARENA_ALIGN: usize = 16;

/// Callback when allocation succeeds but locking fails.
///
/// Return `true` to proceed (warnings only) or `false` to abort the allocation.
pub type LockingFailed_Callback = fn() -> bool;
