// ---------------- [ File: bitcoin-cuckoo-cache/src/bit_packed_atomic_flags.rs ]
crate::ix!();

/// @ref bit_packed_atomic_flags implements a container for garbage collection flags that is only
/// thread unsafe on calls to setup. This class bit-packs collection flags for memory efficiency.
/// 
/// All operations are `std::memory_order_relaxed` so external mechanisms must ensure that writes
/// and reads are properly synchronized.
/// 
/// On setup(n), all bits up to `n` are marked as collected.
/// 
/// Under the hood, because it is an 8-bit type, it makes sense to use a multiple of 8 for setup,
/// but it will be safe if that is not the case as well.
/// 
/// No default constructor, as there must be some size.
/// 
/// Thread‑unsafe **only** during [`BitPackedAtomicFlags::setup`] and `new`.
///
/// Internally stores one `AtomicU8` for every eight user‑visible bits, so its capacity is always
/// rounded _up_ to the next multiple of eight.
///
#[derive(Getters, Builder, Debug)]
pub struct BitPackedAtomicFlags {
    /// Packed bits; each atomic byte manages eight flag bits.
    ///
    /// *Never* expose this field publicly – use the generated `mem()` getter.
    #[get = "pub(crate)"]
    mem: Box<[AtomicU8]>,
}

impl BitPackedAtomicFlags {

    /// bit_packed_atomic_flags constructor creates memory to sufficiently keep
    /// track of garbage collection information for `size` entries.
    /// 
    /// ----------- @param size
    /// 
    /// the number of elements to allocate space for @post bit_set, bit_unset,
    /// and bit_is_set function properly forall x. x < size @post All calls to
    /// bit_is_set (without subsequent bit_unset) will return true.
    /// 
    /// Construct a new flag container able to track at least `size` bits.
    ///
    /// All bits start **set** (`true` / _collectable_).
    #[inline]
    pub fn new(size: u32) -> Self {
        trace!(size, "initialising BitPackedAtomicFlags");

        // Round up so that every element has a backing bit.
        let cells = ((size + 7) / 8) as usize;
        let boxed: Box<[AtomicU8]> = (0..cells)
            .map(|_| AtomicU8::new(0xFF))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        debug!(cells, "allocated atomic byte array");
        Self { mem: boxed }
    }

    /// setup marks all entries and ensures that bit_packed_atomic_flags can
    /// store at least `b` entries.
    /// 
    /// ----------- @param b
    /// 
    /// the number of elements to allocate space for @post bit_set, bit_unset,
    /// and bit_is_set function properly forall x. x < b @post All calls to
    /// bit_is_set (without subsequent bit_unset) will return true.
    /// 
    /// (Re)initialise the container to hold *at least* `b` bits, setting them
    /// all to **set** (`true`).
    ///
    /// Safe to call multiple times; any previous storage is dropped.
    #[inline]
    pub fn setup(&mut self, b: u32) {
        trace!(b, "re‑initialising BitPackedAtomicFlags via setup()");
        // Replace `self` in‑place to avoid double‑initialisation boilerplate.
        *self = Self::new(b);
    }

    /// bit_set sets an entry as discardable.
    /// 
    /// ----------- @param s
    /// 
    /// the index of the entry to bit_set @post immediately subsequent call
    /// (assuming proper external memory ordering) to bit_is_set(s) == true.
    /// 
    /// Mark bit `s` as **set** (collectable).
    ///
    #[inline]
    pub fn bit_set(&self, s: u32) {
        let idx = (s >> 3) as usize;
        let mask = 1u8 << (s & 7);
        self.mem[idx].fetch_or(mask, atomic::Ordering::Relaxed);
        trace!(index = s, mask, "bit_set()");
    }

    /// bit_unset marks an entry as something that should not be overwritten.
    /// 
    /// ----------- @param s
    /// 
    /// the index of the entry to bit_unset @post immediately subsequent call
    /// (assuming proper external memory ordering) to bit_is_set(s) == false.
    /// 
    /// Mark bit `s` as **unset** (keep / not‑collectable).
    #[inline]
    pub fn bit_unset(&self, s: u32) {
        let idx = (s >> 3) as usize;
        let mask = !(1u8 << (s & 7));
        self.mem[idx].fetch_and(mask, atomic::Ordering::Relaxed);
        trace!(index = s, mask, "bit_unset()");
    }

    /// bit_is_set queries the table for discardability at `s`.
    /// 
    /// ----------- @param s
    /// 
    /// the index of the entry to read
    /// 
    /// ----------- @return
    /// 
    /// true if the bit at index `s` was set, false otherwise
    /// 
    /// Returns `true` iff bit `s` is currently **set**.
    #[inline]
    pub fn bit_is_set(&self, s: u32) -> bool {
        let idx = (s >> 3) as usize;
        let mask = 1u8 << (s & 7);
        let value = (self.mem[idx].load(atomic::Ordering::Relaxed) & mask) != 0;
        trace!(index = s, mask, value, "bit_is_set()");
        value
    }
}

/// Exhaustive behavioural tests for [`BitPackedAtomicFlags`].
#[cfg(test)]
mod bit_packed_atomic_flags_suite {
    use super::*;

    /// Helper to build a flag container and assert all bits are initially set.
    fn assert_all_set(flags: &BitPackedAtomicFlags, bits: u32) {
        for i in 0..bits {
            assert!(
                flags.bit_is_set(i),
                "bit {i} expected to be set after construction"
            );
        }
    }

    #[traced_test]
    fn constructor_sets_all_bits() {
        let bits = 20;
        let flags = BitPackedAtomicFlags::new(bits);
        assert_all_set(&flags, bits);
    }

    #[traced_test]
    fn bit_set_and_unset_roundtrip() {
        let bits = 16;
        let flags = BitPackedAtomicFlags::new(bits);

        // Unset all, verify, then set again and verify.
        for i in 0..bits {
            flags.bit_unset(i);
            assert!(!flags.bit_is_set(i), "bit {i} should be unset");
            flags.bit_set(i);
            assert!(flags.bit_is_set(i), "bit {i} should be set again");
        }
    }

    #[traced_test]
    fn setup_reinitialises_capacity_and_state() {
        let mut flags = BitPackedAtomicFlags::new(8);
        // Tweak some bits to non‑default state.
        flags.bit_unset(3);
        flags.bit_unset(5);

        // Now grow and ensure brand‑new state.
        let new_bits = 40;
        flags.setup(new_bits);
        assert_all_set(&flags, new_bits);
        // Old capacity must have grown (5 bytes => needs at least 5)
        assert!(
            flags.mem().len() >= ((new_bits + 7) / 8) as usize,
            "internal capacity did not grow as expected"
        );
    }
}
