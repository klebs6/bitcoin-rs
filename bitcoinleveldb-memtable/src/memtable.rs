// ---------------- [ File: bitcoinleveldb-memtable/src/memtable.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/memtable.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/memtable.cc]

pub type MemTableTable = SkipList<*const u8, MemTableKeyComparator>;

#[derive(Builder,Getters,Setters,MutGetters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[builder(pattern = "owned")]
pub struct MemTable {
    comparator:       MemTableKeyComparator,
    refs:             i32,
    arena:            Box<Arena>,
    table:            MemTableTable,
    base_arena_usage: usize,
}

impl Drop for MemTable {
    /**
      | Private since only Unref() should be
      | used to delete it
      |
      */
    fn drop(&mut self) {
        trace!(
            "MemTable::drop: self={:p}, refs_={}",
            self as *mut MemTable,
            self.refs
        );
        assert!(
            self.refs == 0,
            "MemTable::drop: refs_ must be 0 (found {})",
            self.refs
        );
    }
}

impl MemTable {

    /**
      | Increase reference count.
      |
      */
    pub fn ref_(&mut self) {
        self.refs += 1;
        trace!(
            "MemTable::ref_: self={:p}, refs_={}",
            self as *mut MemTable,
            self.refs
        );
    }

    /**
      | Drop reference count. Delete if no more
      | references exist.
      |
      */
    pub fn unref(&mut self) {
        trace!(
            "MemTable::unref: self={:p}, before refs_={}",
            self as *mut MemTable,
            self.refs
        );

        self.refs -= 1;

        assert!(
            self.refs >= 0,
            "MemTable::unref: refs_ went negative ({})",
            self.refs
        );

        if self.refs <= 0 {
            trace!(
                "MemTable::unref: refs_ reached {}; deleting self",
                self.refs
            );

            let self_ptr: *mut MemTable = self;

            // Safety: we rely on MemTable instances that participate in
            // reference counting to be allocated via `Box::into_raw`,
            // mirroring LevelDB's `delete this` pattern.
            unsafe {
                drop(Box::from_raw(self_ptr));
            }
        }
    }

    /**
      | MemTables are reference counted. The
      | initial reference count is zero and
      | the caller must call Ref() at least once.
      |
      */
    pub fn new(comparator: &InternalKeyComparator) -> Self {
        trace!(
            "MemTable::new: user_comparator_ptr={:p}",
            comparator.user_comparator()
        );

        // Comparator used for Get()
        let cmp_for_mem = MemTableKeyComparator::new(comparator);
        // Comparator instance used by the underlying skiplist
        let cmp_for_table = MemTableKeyComparator::new(comparator);

        let mut arena_box = Box::new(Arena::default());
        let arena_ptr: *mut Arena = &mut *arena_box;

        let table = MemTableTable::new(cmp_for_table, arena_ptr);

        let base_usage = arena_box.memory_usage();
        trace!(
            "MemTable::new: base_arena_usage={} bytes",
            base_usage
        );

        MemTable {
            comparator:       cmp_for_mem,
            refs:             0,
            arena:            arena_box,
            table,
            base_arena_usage: base_usage,
        }
    }

    /**
      | Returns an estimate of the number of
      | bytes of data in use by this data structure
      | beyond the baseline skiplist overhead.
      | It is safe to call when MemTable is being
      | modified.
      |
      */
    pub fn approximate_memory_usage(&mut self) -> usize {
        let total = self.arena.memory_usage();
        let base  = self.base_arena_usage;
        let usage = total.saturating_sub(base);

        trace!(
            "MemTable::approximate_memory_usage: total={} base={} usage={} bytes",
            total,
            base,
            usage
        );

        usage
    }

    /**
      | Return an iterator that yields the contents
      | of the memtable.
      |
      | The caller must ensure that the underlying
      | MemTable remains live while the returned
      | iterator is live.  The keys returned by this
      | iterator are internal keys encoded by
      | AppendInternalKey in the db/format.{h,cc}
      | module.
      */
    pub fn new_iterator(&mut self) -> *mut LevelDBIterator {
        trace!(
            "MemTable::new_iterator: self={:p}",
            self as *mut MemTable
        );

        let table_ptr: *mut MemTableTable = &mut self.table;

        // Concrete iterator that walks the memtable skiplist.
        let mem_iter = MemTableIterator::new(table_ptr);
        let iface: Box<dyn LevelDBIteratorInterface> = Box::new(mem_iter);

        // Wrap in the generic LevelDBIterator facade.
        let wrapper = LevelDBIterator::new(Some(iface));
        let boxed_wrapper = Box::new(wrapper);
        let raw_wrapper = Box::into_raw(boxed_wrapper);

        trace!(
            "MemTable::new_iterator: returning LevelDBIterator @ {:?}",
            raw_wrapper
        );

        raw_wrapper
    }
}
