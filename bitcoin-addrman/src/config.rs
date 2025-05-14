// ---------------- [ File: bitcoin-addrman/src/config.rs ]
crate::ix!();

pub type AddrManNewBucketList   = [[i32; ADDRMAN_NEW_BUCKET_COUNT];   ADDRMAN_BUCKET_SIZE];
pub type AddrManTriedBucketList = [[i32; ADDRMAN_TRIED_BUCKET_COUNT]; ADDRMAN_BUCKET_SIZE];

/**
  | Default for -checkaddrman
  |
  */
pub const DEFAULT_ADDRMAN_CONSISTENCY_CHECKS: usize = 0;

/**
  | Over how many buckets entries with tried
  | addresses from a single group (/16 for
  | IPv4) are spread
  |
  */
pub const ADDRMAN_TRIED_BUCKETS_PER_GROUP: usize = 8;

/**
  | Over how many buckets entries with new
  | addresses originating from a single
  | group are spread
  |
  */
pub const ADDRMAN_NEW_BUCKETS_PER_SOURCE_GROUP: usize = 64;

/**
  | Maximum number of times an address can
  | occur in the new table
  |
  */
pub const ADDRMAN_NEW_BUCKETS_PER_ADDRESS: usize = 8;

/**
  | How old addresses can maximally be
  |
  */
pub const ADDRMAN_HORIZON_DAYS: usize = 30;

/**
  | After how many failed attempts we give
  | up on a new node
  |
  */
pub const ADDRMAN_RETRIES: usize = 3;

/**
  | How many successive failures are allowed
  | ...
  |
  */
pub const ADDRMAN_MAX_FAILURES: usize = 10;

/**
  | ... in at least this many days
  |
  */
pub const ADDRMAN_MIN_FAIL_DAYS: usize = 7;

/**
  | How recent a successful connection
  | should be before we allow an address
  | to be evicted from tried
  |
  */
pub const ADDRMAN_REPLACEMENT_HOURS: usize = 4;

/**
  | The maximum number of tried addr collisions
  | to store
  |
  */
pub const ADDRMAN_SET_TRIED_COLLISION_SIZE: usize = 10;

/**
  | The maximum time we'll spend trying
  | to resolve a tried table collision,
  | in seconds
  |
  */
pub const ADDRMAN_TEST_WINDOW: usize = 40*60; // 40 minutes

/**
  | Total number of buckets for tried addresses
  |
  */
pub const ADDRMAN_TRIED_BUCKET_COUNT_LOG2: usize = 8;
pub const ADDRMAN_TRIED_BUCKET_COUNT:      usize = 1 << ADDRMAN_TRIED_BUCKET_COUNT_LOG2;

/**
  | Total number of buckets for new addresses
  |
  */
pub const ADDRMAN_NEW_BUCKET_COUNT_LOG2: usize = 10;
pub const ADDRMAN_NEW_BUCKET_COUNT:      usize = 1 << ADDRMAN_NEW_BUCKET_COUNT_LOG2;

/**
  | Maximum allowed number of entries in
  | buckets for new and tried addresses
  |
  */
pub const ADDRMAN_BUCKET_SIZE_LOG2: usize = 6;
pub const ADDRMAN_BUCKET_SIZE:      usize = 1 << ADDRMAN_BUCKET_SIZE_LOG2;

/**
  | The initial value of a field that is
  | incremented every time an incompatible
  | format change is made (such that old
  | software versions would not be able to
  | parse and understand the new file
  | format). This is 32 because we overtook the
  | "key size" field which was 32 historically.
  | @note Don't increment this. Increment
  | `lowest_compatible` in `Serialize()`
  | instead.
  */
pub const ADDR_MAN_INCOMPATIBILITY_BASE: u8 = 32;
