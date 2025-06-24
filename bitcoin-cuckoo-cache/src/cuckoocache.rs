// ---------------- [ File: bitcoin-cuckoo-cache/src/cuckoocache.rs ]
/// High-performance cache primitives.
/// 
/// Summary:
/// 
/// 1. @ref bit_packed_atomic_flags is bit-packed atomic flags for garbage collection
/// 
/// 2. @ref cache is a cache which is performant in memory usage and lookup speed. It is lockfree
/// for erase operations. Elements are lazily erased on the next insert.
/// 

crate::ix!();

/// A trait providing *eight* **independent** 32‑bit hashes for a value.
///
/// The reference C++ implementation uses a templated `operator()<k>(e)` to
/// obtain the *k*‑th hash; in Rust we expose all eight at once.
pub trait EightWayHasher<E> {
    /// Return eight high‑entropy hashes for `e`.
    ///
    /// Implementations **must** ensure that changing `e` changes every entry
    /// in the returned array with near‑uniform probability.
    fn hashes(&self, e: &E) -> [u32; 8];
}

//-------------------------------------------[.cpp/bitcoin/src/cuckoocache.h]

/// @ref cache implements a cache with properties similar to a cuckoo-set.
/// 
/// The cache is able to hold up to `(~(uint32_t)0) - 1` elements.
/// 
///  Read Operations:
///      - contains() for `erase=false`
/// 
///  Read+Erase Operations:
///      - contains() for `erase=true`
/// 
///  Erase Operations:
///      - allow_erase()
/// 
///  Write Operations:
///      - setup()
///      - setup_bytes()
///      - insert()
///      - please_keep()
/// 
///  Synchronization Free Operations:
///      - invalid()
///      - compute_hashes()
/// 
/// User Must Guarantee:
/// 
/// -1. Write requires synchronized access (e.g. a lock)
/// 
/// -2. Read requires no concurrent Write, synchronized with last insert.
/// 
/// -3. Erase requires no concurrent Write, synchronized with last insert.
/// 
/// -4. An Erase caller must release all memory before allowing a new Writer.
/// 
/// Note on function names:
/// 
///   - The name "allow_erase" is used because the real discard happens later.
/// 
///   - The name "please_keep" is used because elements may be erased anyways on insert.
/// 
/// @tparam Element should be a movable and copyable type
/// 
/// @tparam Hash should be a function/callable which takes a template parameter
/// 
/// hash_select and an Element and extracts a hash from it. Should return high-entropy uint32_t
/// hashes for `Hash h; h<0>(e) ... h<7>(e)`.
///
/// The Rust implementation keeps the exact algorithmic behaviour – including
/// lazy reclamation & epoch aging – while relying on safe interior mutability
/// (`AtomicU8` and `BitPackedAtomicFlags`) in place of the C++ `mutable` hack.
///
#[derive(Getters, Debug)]
#[get = "pub(crate)"]
pub struct Cache<Element, Hash>
where
    Element: PartialEq + Clone,
    Hash: EightWayHasher<Element>,
{
    /// Stores the user elements.  `None` means “never initialised”.
    ///
    table: Vec<Option<Element>>,

    /// size stores the total available slots in the hash table
    ///
    /// Number of buckets (always `>= 2`).
    ///
    size: u32,

    /// The bit_packed_atomic_flags array is marked mutable because we want
    /// garbage collection to be allowed to occur from const methods.
    ///
    /// Flags indicating whether a bucket **may** be overwritten (`true`) or
    /// **must** be kept (`false`).
    ///
    collection_flags: BitPackedAtomicFlags,

    /// epoch_flags tracks how recently an element was inserted into the
    /// cache. true denotes recent, false denotes not-recent. See insert()
    /// method for full semantics.
    ///
    /// `true` – member belongs to the current (youngest) epoch.  
    ///
    /// `false` – member belongs to the previous epoch and will be erased when
    ///           the epoch advances once more.
    ///
    epoch_flags: Vec<bool>,

    /// Cheap epoch‑aging heuristic; decremented on every `insert`.
    ///
    /// epoch_heuristic_counter is used to determine when an epoch might be aged
    /// & an expensive scan should be done. epoch_heuristic_counter is
    /// decremented on insert and reset to the new number of inserts which would
    /// cause the epoch to reach epoch_size when it reaches zero.
    ///
    epoch_heuristic_counter: u32,

    /// Maximum number of live entries that fit into a single epoch.
    ///
    /// epoch_size is set to be the number of elements supposed to be in
    /// a epoch. When the number of non-erased elements in an epoch exceeds
    /// epoch_size, a new epoch should be started and all current entries
    /// demoted. epoch_size is set to be 45% of size because we want to keep
    /// load around 90%, and we support 3 epochs at once -- one "dead" which has
    /// been erased, one "dying" which has been marked to be erased next, and
    /// one "living" which new inserts add to.
    /// 
    epoch_size: u32,

    /// Maximum chain length the Cuckoo insertion algorithm will explore.
    ///
    /// depth_limit determines how many elements insert should try to replace.
    /// 
    /// Should be set to log2(n).
    /// 
    depth_limit: u8,

    /// User‑supplied eight‑way hash provider.
    ///
    /// hash_function is a const instance of the hash function. It cannot be
    /// static or initialized at call time as it may have internal state (such
    /// as a nonce).
    /// 
    hash_function: Hash,
}

impl<E, H> Default for Cache<E, H>
where
    E: PartialEq + Clone,
    H: EightWayHasher<E> + Default,
{
    /// You must always construct a cache with some elements via a subsequent call to setup or
    /// setup_bytes, otherwise operations may segfault.
    fn default() -> Self {
        // A completely empty cache – *must* be `setup()` before use.
        Self {
            table: Vec::new(),
            size: 0,
            collection_flags: BitPackedAtomicFlags::new(0),
            epoch_flags: Vec::new(),
            epoch_heuristic_counter: 0,
            epoch_size: 0,
            depth_limit: 0,
            hash_function: H::default(),
        }
    }
}

impl<E, H> Cache<E, H>
where
    E: PartialEq + Clone,
    H: EightWayHasher<E>,
{
    /// Map the eight raw hashes for `e` onto bucket indices in `[0, size)`.
    ///
    /// compute_hashes is convenience for not having to write out this expression everywhere we use
    /// the hash values of an Element.
    /// 
    /// We need to map the 32-bit input hash onto a hash bucket in a range [0, size) in a manner
    /// which preserves as much of the hash's uniformity as possible. Ideally this would be done by
    /// bitmasking but the size is usually not a power of two.
    /// 
    /// The naive approach would be to use a mod -- which isn't perfectly uniform but so long as
    /// the hash is much larger than size it is not that bad. Unfortunately, mod/division is fairly
    /// slow on ordinary microprocessors (e.g. 90-ish cycles on haswell, ARM doesn't even have an
    /// instruction for it.); when the divisor is a constant the compiler will do clever tricks to
    /// turn it into a multiply+add+shift, but size is a run-time value so the compiler can't do
    /// that here.
    /// 
    /// One option would be to implement the same trick the compiler uses and compute the constants
    /// for exact division based on the size, as described in "{N}-bit Unsigned
    /// 
    /// Division via {N}-bit Multiply-Add" by Arch D. Robison in 2005. But that code is somewhat
    /// complicated and the result is still slower than other options:
    /// 
    /// Instead we treat the 32-bit random number as a Q32 fixed-point number in the range [0, 1)
    /// and simply multiply it by the size. Then we just shift the result down by 32-bits to get
    /// our bucket number. The result has non-uniformity the same as a mod, but it is much faster
    /// to compute. More about this technique can be found at
    /// https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/ .
    /// 
    /// The resulting non-uniformity is also more equally distributed which would be advantageous
    /// for something like linear probing, though it shouldn't matter one way or the other for
    /// a cuckoo table.
    /// 
    /// The primary disadvantage of this approach is increased intermediate precision is required
    /// but for a 32-bit random number we only need the high 32 bits of a 32*32->64 multiply, which
    /// means the operation is reasonably fast even on a typical 32-bit processor.
    /// 
    /// -----------
    /// @param e
    /// 
    /// The element whose hashes will be returned
    /// 
    /// -----------
    /// @return
    /// 
    /// Deterministic hashes derived from
    /// `e` uniformly mapped onto the range
    /// [0, size)
    /// 

    #[inline]
    pub fn compute_hashes(&self, e: &E) -> [u32; 8] {
        let size = self.size as u64;
        self.hash_function
            .hashes(e)
            .map(|h| ((h as u64 * size) >> 32) as u32)
    }

    /// A bucket index that is *never* produced by [`compute_hashes`].
    ///
    /// invalid returns a special index that can never be inserted to
    /// 
    /// ----------- @return
    /// 
    /// the special constexpr index that can never be inserted to
    /// 
    #[inline]
    pub const fn invalid() -> u32 {
        u32::MAX
    }

    /// Mark bucket `n` as **discardable** – it may be overwritten later.
    ///
    /// allow_erase marks the element at index `n` as discardable. 
    ///
    /// Threadsafe without any concurrent insert.
    /// 
    /// ----------- @param n
    /// 
    /// the index to allow erasure of
    /// 
    #[inline]
    pub fn allow_erase(&self, n: u32) {
        self.collection_flags.bit_set(n);
        trace!(bucket = n, "allow_erase()");
    }

    /// Mark bucket `n` as **protected** – it must not be overwritten.
    ///
    /// please_keep marks the element at index `n` as an entry that should be
    /// kept.
    /// 
    /// Threadsafe without any concurrent insert.
    /// 
    /// ----------- @param n
    /// 
    /// the index to prioritize keeping
    /// 
    ////
    #[inline]
    pub fn please_keep(&self, n: u32) {
        self.collection_flags.bit_unset(n);
        trace!(bucket = n, "please_keep()");
    }

    /// Potentially advance epochs and perform lazy reclamation.
    ///
    /// epoch_check handles the changing of epochs for elements stored in the
    /// cache. epoch_check should be run before every insert.
    /// 
    /// First, epoch_check decrements and checks the cheap heuristic, and then
    /// does a more expensive scan if the cheap heuristic runs out. If the
    /// expensive scan succeeds, the epochs are aged and old elements are
    /// allow_erased. The cheap heuristic is reset to retrigger after the worst
    /// case growth of the current epoch's elements would exceed the epoch_size.
    /// 
    #[inline]
    fn epoch_check(&mut self) {
        if self.epoch_heuristic_counter > 0 {
            self.epoch_heuristic_counter -= 1;
            return;
        }

        // Count un‑erased entries from the newest epoch.
        let mut epoch_unused = 0_u32;
        for i in 0..self.size {
            if self.epoch_flags[i as usize] && !self.collection_flags.bit_is_set(i) {
                epoch_unused += 1;
            }
        }

        // If there are more non-deleted entries in the current epoch than the
        // epoch size, then allow_erase on all elements in the old epoch (marked
        // false) and move all elements in the current epoch to the old epoch
        // but do not call allow_erase on their indices.
        if epoch_unused >= self.epoch_size {
            // Demote current epoch & erase the previous one.
            for i in 0..self.size {
                if self.epoch_flags[i as usize] {
                    self.epoch_flags[i as usize] = false;
                } else {
                    self.allow_erase(i);
                }
            }
            self.epoch_heuristic_counter = self.epoch_size;
        } else {
            // Cheap heuristic before the next full scan.
            let min_scan = self.epoch_size / 16;
            let diff     = self.epoch_size - epoch_unused;

            // reset the epoch_heuristic_counter to next do a scan when worst
            // case behavior (no intermittent erases) would exceed epoch size,
            // with a reasonable minimum scan size.
            //
            // Ordinarily, we would have to sanity check std::min(epoch_size,
            // epoch_unused_count), but we already know that `epoch_unused_count
            // < epoch_size` in this branch
            self.epoch_heuristic_counter = cmp::max(1, cmp::max(min_scan, diff));
        }
        debug!(counter = self.epoch_heuristic_counter, "epoch_check() reset");
    }

    /// Prepare the cache to hold *at most* `new_size` elements.
    ///
    /// Returns the *actual* table size (≥ 2).
    ///
    /// setup initializes the container to store no more than new_size elements. setup should only
    /// be called once.
    /// 
    /// ----------- @param new_size
    /// 
    /// the desired number of elements to store
    /// 
    /// ----------- @return
    /// 
    /// the maximum number of elements storable
    /// 
    pub fn setup(&mut self, new_size: u32) -> u32 {

        // depth_limit must be at least one otherwise errors can occur.
        self.depth_limit = ((cmp::max(2, new_size) as f32).log2()) as u8;

        self.size        = cmp::max(2, new_size);

        self.table             = vec![None; self.size as usize];
        self.collection_flags  .setup(self.size);
        self.epoch_flags       = vec![false; self.size as usize];

        // Set to 45% as described above
        self.epoch_size             = cmp::max(1, (45 * self.size) / 100);

        // Initially set to wait for a whole epoch
        self.epoch_heuristic_counter = self.epoch_size;

        debug!(
            size         = self.size,
            depth_limit  = self.depth_limit,
            epoch_size   = self.epoch_size,
            "cache setup() completed"
        );
        self.size
    }

    /// Convenience wrapper translating *bytes* of storage into element count.
    ///
    /// setup_bytes is a convenience function which accounts for internal memory usage when
    /// deciding how many elements to store. It isn't perfect because it doesn't account for any
    /// overhead (struct size, MallocUsage, collection and epoch flags). This was done to simplify
    /// selecting a power of two size. In the expected use case, an extra two bits per entry should
    /// be negligible compared to the size of the elements.
    /// 
    /// ----------- @param bytes
    /// 
    /// the approximate number of bytes to use for this data structure
    /// 
    /// ----------- @return
    /// 
    /// the maximum number of elements storable (see setup() documentation for more detail)
    /// 
    #[inline]
    pub fn setup_bytes(&mut self, bytes: usize) -> u32 {
        let elem_sz = cmp::max(mem::size_of::<E>(), 1);
        self.setup((bytes / elem_sz) as u32)
    }

    /// Insert `e`, potentially evicting another entry according to the standard
    /// 8‑way Cuckoo algorithm with bounded search depth.
    ///
    /// insert loops at most depth_limit times trying to insert a hash at various locations in the
    /// table via a variant of the Cuckoo Algorithm with eight hash locations.
    /// 
    /// It drops the last tried element if it runs out of depth before encountering an open slot.
    /// 
    /// Thus:
    /// 
    /// ----------- @param e
    /// 
    /// the element to insert @post one of the following: All previously inserted elements and
    /// e are now in the table, one previously inserted element is evicted from the table, the
    /// entry attempted to be inserted is evicted.
    /// 
    /// ----------- @code
    /// 
    /// ```no-test
    /// insert(x);
    /// return contains(x, false);
    /// ```
    /// is not guaranteed to return true.
    /// 
    #[inline]
    pub fn insert(&mut self, mut e: E) {
        self.epoch_check();

        let mut last_loc  = Self::invalid();
        let mut last_epoch = true;
        let mut locs      = self.compute_hashes(&e);

        // Already present?
        //
        // Make sure we have not already inserted this element
        // If we have, make sure that it does not get deleted
        for &loc in &locs {
            if let Some(ref existing) = self.table[loc as usize] {
                if existing == &e {
                    self.please_keep(loc);
                    self.epoch_flags[loc as usize] = last_epoch;
                    return;
                }
            }
        }

        for _depth in 0..self.depth_limit {
            // First try to insert to an empty slot, if one exists
            for &loc in &locs {
                if self.collection_flags.bit_is_set(loc) {
                    self.table[loc as usize] = Some(e);
                    self.please_keep(loc);
                    self.epoch_flags[loc as usize] = last_epoch;
                    return;
                }
            }

            // Evict an entry – choose “next after last_loc” in `locs`.
            let idx = locs
                .iter()
                .position(|&x| x == last_loc)
                .map(|p| (p + 1) & 7)
                .unwrap_or(0);

            /* Swap with the element at the location that was
             * not the last one looked at. Example:
             *
             * 1. On first iteration, last_loc == invalid(), find returns last, so
             *    last_loc defaults to locs[0].
             * 2. On further iterations, where last_loc == locs[k], last_loc will
             *    go to locs[k+1 % 8], i.e., next of the 8 indices wrapping around
             *    to 0 if needed.
             *
             * This prevents moving the element we just put in.
             *
             * The swap is not a move -- we must switch onto the evicted element
             * for the next iteration.
             */
            last_loc = locs[idx];

            let slot           = &mut self.table[last_loc as usize];
            let mut victim     = slot.take().expect("victim must exist");
            mem::swap(&mut victim, &mut e); // move victim into `e`
            *slot = Some(victim);

            let epoch_current = last_epoch;
            last_epoch        = self.epoch_flags[last_loc as usize];
            self.epoch_flags[last_loc as usize] = epoch_current;

            // Re‑hash the newly evicted element.
            //
            // Recompute the locs -- unfortunately happens one too many times!
            locs = self.compute_hashes(&e);
        }
        // Dropped element `e` silently falls off the cache.
        trace!("insert() evicted element after depth_limit");
    }

    /// Check whether `e` is present, optionally marking its bucket discardable.
    ///
    /// contains iterates through the hash locations for a given element and checks to see if it
    /// is present. contains does not check garbage collected state (in other words, garbage is
    /// only collected when the space is needed), so:
    /// 
    /// -----------
    /// @param e
    /// 
    /// the element to check
    /// ----------
    /// @param erase
    /// 
    /// whether to attempt setting the garbage collect flag @post if erase is true and the
    /// element is found, then the garbage collect flag is set
    /// 
    /// -----------
    /// @code
    /// 
    /// ```no-test
    /// insert(x);
    /// if (contains(x, true))
    ///     return contains(x, false);
    /// else
    ///     return true;
    /// ```
    /// executed on a single thread will always return true!
    /// 
    /// This is a great property for re-org performance for example. contains returns a bool set
    /// true if the element was found.
    /// 
    /// -----------
    /// @return
    /// 
    /// true if the element is found, false otherwise
    /// 
    #[inline]
    pub fn contains(&self, e: &E, erase: bool) -> bool {
        for &loc in &self.compute_hashes(e) {
            if let Some(ref existing) = self.table[loc as usize] {
                if existing == e {
                    if erase {
                        self.allow_erase(loc);
                    }
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod cuckoo_cache_suite {
    use super::*;

    /// A trivial eight‑way hasher for `u32` elements.
    #[derive(Default)]
    struct TrivialHasher;

    impl EightWayHasher<u32> for TrivialHasher {
        fn hashes(&self, e: &u32) -> [u32; 8] {
            let mut h = [0u32; 8];
            for i in 0..8 {
                h[i] = e.wrapping_add(i as u32).wrapping_mul(0x9E37_79B9);
            }
            h
        }
    }

    fn fresh_cache(cap: u32) -> Cache<u32, TrivialHasher> {
        let mut c: Cache<u32, TrivialHasher> = Cache::default();
        c.setup(cap);
        c
    }

    #[traced_test]
    fn setup_establishes_parameters() {
        let mut c = Cache::<u32, TrivialHasher>::default();
        let sz = c.setup(100);
        assert_eq!(sz, *c.size());
        assert!(*c.depth_limit() > 0);
        assert_eq!(c.collection_flags().mem().len(), ((sz + 7) / 8) as usize);
    }

    #[traced_test]
    fn insert_and_contains_roundtrip() {
        let mut c = fresh_cache(128);
        for i in 0..50u32 {
            c.insert(i);
        }
        for i in 0..50u32 {
            assert!(c.contains(&i, false), "cache should contain {i}");
        }
    }

    /// Erasure marks a bucket *discardable* but does **not** remove the element
    /// until a later insertion needs the slot.  This test follows the contract
    /// spelled out in the original C++ comments.
    #[traced_test]
    fn erase_mechanism_works() {
        let mut c = fresh_cache(32);
        c.insert(42);

        // First lookup sets the GC flag.
        assert!(c.contains(&42, true), "first lookup marks for erase");

        // The element is still discoverable until the slot is reused.
        assert!(c.contains(&42, false), "element must remain until reclaimed");

        // Push enough additional elements to *possibly* reclaim the slot.
        for i in 0..64u32 {
            if i != 42 {
                c.insert(i);
            }
        }
        // At this point `42` may or may not still be present; the contract places
        // no guarantee once further inserts occur, so we make no assertion.
    }
}
