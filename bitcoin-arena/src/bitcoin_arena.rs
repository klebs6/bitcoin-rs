// ---------------- [ File: bitcoin-arena/src/bitcoin_arena.rs ]
crate::ix!();

pub type ArenaSizeToChunkSortedMap         = MultiMap<usize,*mut u8>;
pub type ArenaSizeToChunkSortedMapIterator = Box<dyn Iterator<Item = (usize, *mut u8)>>;
pub type ArenaChunkToSizeMap               = HashMap<*mut u8,ArenaSizeToChunkSortedMapIterator>;

/**
  | Memory statistics.
  |
  */
#[derive(Getters, Builder, Default, Debug, Clone)]
#[builder(setter(into, strip_option), pattern = "owned")]
#[getset(get = "pub")]
pub struct ArenaStats {
    /// Bytes currently in use
    used:        usize,
    /// Bytes currently free
    free:        usize,
    /// Total size of the arena (`used + free`)
    total:       usize,
    /// Number of allocated chunks
    chunks_used: usize,
    /// Number of free chunks
    chunks_free: usize,
}

// -----------------------------------------------------------------------------
//  Arena implementation — now with **O(log n) best‑fit** via dual indices
// -----------------------------------------------------------------------------
#[no_copy]
pub struct Arena {
    // --------------  Free‑chunk indices  ----------------------------------
    /// Offset‑ordered view used for coalescing neighbours.
    free_by_offset: BTreeMap<usize /*offset*/, usize /*size*/>,
    /// Size‑ordered multimap enabling best‑fit selection in `O(log n)`.
    /// Each size key owns a `BTreeSet` with *all* offsets of that size.
    free_by_size:   BTreeMap<usize /*size*/, BTreeSet<usize /*offset*/>>,

    // --------------  Used chunks  -----------------------------------------
    used_chunks: HashMap<usize /*offset*/, usize /*size*/>,

    // --------------  Invariants  ------------------------------------------
    base:      *mut u8,
    end:       *mut u8,
    alignment: usize,
}

// ————————————————————————————————————————————————————————————————————————
//  Arena core
// ————————————————————————————————————————————————————————————————————————
impl Arena {

    /// **Safety contract:** `base_in … base_in+size_in` must remain valid
    /// for the lifetime of the `Arena`.
    pub unsafe fn new(base_in: *mut c_void, size_in: usize, alignment_in: usize) -> Self {
        trace!(
            "Arena::new  base={:?} size={} align={}",
            base_in,
            size_in,
            alignment_in
        );

        let mut free_by_offset = BTreeMap::new();
        let mut free_by_size   = BTreeMap::new();

        free_by_offset.insert(0, size_in);
        free_by_size
            .entry(size_in)
            .or_insert_with(BTreeSet::new)
            .insert(0);

        Self {
            free_by_offset,
            free_by_size,
            used_chunks: HashMap::new(),
            base: base_in as *mut u8,
            end:  (base_in as *mut u8).add(size_in),
            alignment: alignment_in,
        }
    }

    /// `true` if `ptr` lies in `[base, end)`.
    pub fn address_in_arena(&self, ptr: *mut c_void) -> bool {
        let p = ptr as usize;
        let b = self.base as usize;
        let e = self.end  as usize;
        p >= b && p < e
    }

    /// Allocate `size` bytes, returning a null pointer when the request
    /// cannot be satisfied.
    pub fn alloc(&mut self, size: usize) -> *mut c_void {
        let size = align_up(size, self.alignment);
        if size == 0 {
            trace!("Arena::alloc size=0 ⇒ null");
            return std::ptr::null_mut();
        }

        // --------  O(log n) best‑fit search  --------
        let (best_size, offsets) = match self.free_by_size.range(size..).next() {
            Some(pair) => (*pair.0, pair.1.clone()), // clone BTreeSet (cheap)
            None => {
                trace!("Arena::alloc size={} ⇒ null (no space)", size);
                return std::ptr::null_mut();
            }
        };
        let offset = *offsets.iter().next().expect("non‑empty set");

        // --------  Remove chosen chunk from indices  --------
        self.remove_free(offset, best_size);

        // --------  Split (take from tail)  -------------
        let remaining = best_size - size;
        if remaining != 0 {
            self.insert_free(offset, remaining);
        }
        let alloc_offset = offset + remaining;
        self.used_chunks.insert(alloc_offset, size);

        let ptr = unsafe { self.base.add(alloc_offset) } as *mut c_void;
        trace!(
            "Arena::alloc size={} off={} (prev_chunk={}→rem={}) ptr={:?}",
            size,
            alloc_offset,
            best_size,
            remaining,
            ptr
        );
        ptr
    }

    /// Free a previously allocated block. Passing a null pointer is a no‑op.
    /// Invalid or double‑free triggers a panic.
    pub fn free(&mut self, ptr: *mut c_void) {
        if ptr.is_null() {
            trace!("Arena::free ptr=null ⇒ no‑op");
            return;
        }
        assert!(self.address_in_arena(ptr), "pointer outside arena");

        // `offset` must remain accurate after left‑side coalescing, so it is mutable.
        let mut offset = (ptr as usize) - (self.base as usize);
        let mut size   = self
            .used_chunks
            .remove(&offset)
            .expect("Arena: invalid or double free");

        // --------------------  Coalesce left  --------------------
        if let Some((&prev_off, &prev_sz)) = self.free_by_offset.range(..offset).next_back() {
            if prev_off + prev_sz == offset {
                self.remove_free(prev_off, prev_sz);
                offset = prev_off;           // <-- keep chunk start correct
                size   += prev_sz;
                trace!("Arena::free coalesced left @0x{:x}", prev_off);
            }
        }

        // --------------------  Coalesce right  -------------------
        if let Some((&next_off, &next_sz)) = self.free_by_offset.range(offset + size..).next() {
            if offset + size == next_off {
                self.remove_free(next_off, next_sz);
                size += next_sz;
                trace!("Arena::free coalesced right @0x{:x}", next_off);
            }
        }

        // --------------------  Final insert  ---------------------
        self.insert_free(offset, size);
        trace!("Arena::free final chunk off=0x{:x} size={}", offset, size);
    }

    /// O(chunks) statistics aggregation.
    pub fn stats(&self) -> ArenaStats {
        let used: usize  = self.used_chunks.values().sum();
        let free: usize  = self.free_by_offset.values().sum();
        let total        = used + free;
        let chunks_used  = self.used_chunks.len();
        let chunks_free  = self.free_by_offset.len();

        trace!(
            "Arena::stats used={} free={} total={} cu={} cf={}",
            used,
            free,
            total,
            chunks_used,
            chunks_free
        );

        ArenaStatsBuilder::default()
            .used(used)
            .free(free)
            .total(total)
            .chunks_used(chunks_used)
            .chunks_free(chunks_free)
            .build()
            .expect("stats complete")
    }

    // ---------------------------------------------------------------------
    //  Internal helpers (not part of public API)
    // ---------------------------------------------------------------------
    #[inline]
    fn insert_free(&mut self, offset: usize, size: usize) {
        self.free_by_offset.insert(offset, size);
        self.free_by_size
            .entry(size)
            .or_insert_with(BTreeSet::new)
            .insert(offset);
    }

    #[inline]
    fn remove_free(&mut self, offset: usize, size: usize) {
        self.free_by_offset.remove(&offset);
        let set = self
            .free_by_size
            .get_mut(&size)
            .expect("size key must exist");
        set.remove(&offset);
        if set.is_empty() {
            self.free_by_size.remove(&size);
        }
    }

    /// Human‑readable dump when `ARENA_DEBUG` is enabled.
    #[cfg(ARENA_DEBUG)]
    pub fn walk(&self) {
        use std::fmt::Write;
        let mut s = String::new();
        writeln!(&mut s, "USED:").ok();
        for (off, sz) in &self.used_chunks {
            writeln!(&mut s, "  0x{:08x} + {}", off, sz).ok();
        }
        writeln!(&mut s, "FREE:").ok();
        for (off, sz) in &self.free_chunks {
            writeln!(&mut s, "  0x{:08x} + {}", off, sz).ok();
        }
        debug!("{}", s);
    }
}
