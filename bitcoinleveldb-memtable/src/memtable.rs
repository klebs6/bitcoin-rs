// ---------------- [ File: bitcoinleveldb-memtable/src/memtable.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/memtable.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/memtable.cc]

pub struct MemTable {
    comparator: MemTableKeyComparator,
    refs:       i32,
    arena:      Arena,
    table:      MemTableTable,
}

pub struct MemTableKeyComparator {
    comparator: InternalKeyComparator,
}

impl MemTableKeyComparator {

    pub fn new(c: &InternalKeyComparator) -> Self {
    
        todo!();
        /*
        : comparator(c),
        */
    }
    
    pub fn invoke(&self, 
        aptr: *const u8,
        bptr: *const u8) -> i32 {
        
        todo!();
        /*
            // Internal keys are encoded as length-prefixed strings.
      Slice a = GetLengthPrefixedSlice(aptr);
      Slice b = GetLengthPrefixedSlice(bptr);
      return comparator.Compare(a, b);
        */
    }
}

pub type MemTableTable = SkipList<MemTableKeyComparator>;

impl Drop for MemTable {

    /**
      | Private since only Unref() should be
      | used to delete it
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            assert(refs_ == 0);
        */
    }
}

impl MemTable {

    /**
      | Increase reference count.
      |
      */
    pub fn ref_(&mut self)  {
        
        todo!();
        /*
            ++refs_;
        */
    }

    /**
      | Drop reference count. Delete if no more
      | references exist.
      |
      */
    pub fn unref(&mut self)  {
        
        todo!();
        /*
            --refs_;
        assert(refs_ >= 0);
        if (refs_ <= 0) {
          delete this;
        }
        */
    }

    /**
      | MemTables are reference counted. The
      | initial reference count is zero and
      | the caller must call Ref() at least once.
      |
      */
    pub fn new(comparator: &InternalKeyComparator) -> Self {
    
        todo!();
        /*
           : comparator_(comparator), refs_(0), table_(comparator_, &arena_)
           */
    }
    
    /**
      | Returns an estimate of the number of
      | bytes of data in use by this data structure.
      | It is safe to call when MemTable is being
      | modified.
      |
      */
    pub fn approximate_memory_usage(&mut self) -> usize {
        
        todo!();
        /*
            return arena_.MemoryUsage();
        */
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
        
        todo!();
        /*
            return new MemTableIterator(&table_);
        */
    }
    
    /**
      | Add an entry into memtable that maps key to
      | value at the specified sequence number and
      | with the specified type.
      |
      | Typically value will be empty if
      | type==kTypeDeletion.
      */
    pub fn add(&mut self, 
        s:     SequenceNumber,
        ty:    ValueType,
        key_:   &Slice,
        value: &Slice)  {
        
        todo!();
        /*
            // Format of an entry is concatenation of:
      //  key_size     : varint32 of internal_key.size()
      //  key bytes    : char[internal_key.size()]
      //  value_size   : varint32 of value.size()
      //  value bytes  : char[value.size()]
      size_t key_size = key.size();
      size_t val_size = value.size();
      size_t internal_key_size = key_size + 8;
      const size_t encoded_len = VarintLength(internal_key_size) +
                                 internal_key_size + VarintLength(val_size) +
                                 val_size;
      char* buf = arena_.Allocate(encoded_len);
      char* p = EncodeVarint32(buf, internal_key_size);
      memcpy(p, key.data(), key_size);
      p += key_size;
      EncodeFixed64(p, (s << 8) | type);
      p += 8;
      p = EncodeVarint32(p, val_size);
      memcpy(p, value.data(), val_size);
      assert(p + val_size == buf + encoded_len);
      table_.Insert(buf);
        */
    }
    
    /**
      | If memtable contains a value for key, store
      | it in *value and return true.
      |
      | If memtable contains a deletion for key,
      | store a NotFound() error in *status and
      | return true.
      |
      | Else, return false.
      */
    pub fn get(&mut self, 
        key_:   &LookupKey,
        value: *mut String,
        s:     *mut Status) -> bool {
        
        todo!();
        /*
            Slice memkey = key.memtable_key();
      Table::Iterator iter(&table_);
      iter.Seek(memkey.data());
      if (iter.Valid()) {
        // entry format is:
        //    klength  varint32
        //    userkey  char[klength]
        //    tag      uint64
        //    vlength  varint32
        //    value    char[vlength]
        // Check that it belongs to same user key.  We do not check the
        // sequence number since the Seek() call above should have skipped
        // all entries with overly large sequence numbers.
        const char* entry = iter.key();
        uint32_t key_length;
        const char* key_ptr = GetVarint32Ptr(entry, entry + 5, &key_length);
        if (comparator_.comparator.user_comparator()->Compare(
                Slice(key_ptr, key_length - 8), key.user_key()) == 0) {
          // Correct user key
          const uint64_t tag = DecodeFixed64(key_ptr + key_length - 8);
          switch (static_cast<ValueType>(tag & 0xff)) {
            case kTypeValue: {
              Slice v = GetLengthPrefixedSlice(key_ptr + key_length);
              value->assign(v.data(), v.size());
              return true;
            }
            case kTypeDeletion:
              *s = Status::NotFound(Slice());
              return true;
          }
        }
      }
      return false;
        */
    }
}

///--------------------------
pub struct MemTableIterator {
    base: LevelDBIterator,
    iter: LevelDBIterator,

    /**
       For passing to EncodeKey
      */
    tmp:  String,
}

impl MemTableIterator {

    pub fn new(table: *mut MemTableTable) -> Self {
    
        todo!();
        /*
        : iter(table),
        */
    }
    
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return iter_.Valid();
        */
    }
    
    pub fn seek(&mut self, k: &Slice)  {
        
        todo!();
        /*
            iter_.Seek(EncodeKey(&tmp_, k));
        */
    }
    
    pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
            iter_.SeekToFirst();
        */
    }
    
    pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
            iter_.SeekToLast();
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            iter_.Next();
        */
    }
    
    pub fn prev(&mut self)  {
        
        todo!();
        /*
            iter_.Prev();
        */
    }
    
    pub fn key(&self) -> Slice {
        
        todo!();
        /*
            return GetLengthPrefixedSlice(iter_.key());
        */
    }
    
    pub fn value(&self) -> Slice {
        
        todo!();
        /*
            Slice key_slice = GetLengthPrefixedSlice(iter_.key());
        return GetLengthPrefixedSlice(key_slice.data() + key_slice.size());
        */
    }
    
    pub fn status(&self) -> Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}

/**
  | Encode a suitable internal key target for
  | "target" and return it.
  |
  | Uses *scratch as scratch space, and the
  | returned pointer will point into this scratch
  | space.
  */
pub fn encode_key(
        scratch: *mut String,
        target:  &Slice) -> *const u8 {
    
    todo!();
        /*
            scratch->clear();
      PutVarint32(scratch, target.size());
      scratch->append(target.data(), target.size());
      return scratch->data();
        */
}

pub fn get_length_prefixed_slice(data: *const u8) -> Slice {
    
    todo!();
        /*
            uint32_t len;
      const char* p = data;
      p = GetVarint32Ptr(p, p + 5, &len);  // +5: we assume "p" is not corrupted
      return Slice(p, len);
        */
}

struct MemTableConstructor {
    base:                Constructor,
    internal_comparator: InternalKeyComparator,
    memtable:            *mut MemTable,
}

impl Drop for MemTableConstructor {
    fn drop(&mut self) {
        todo!();
        /*
            memtable_->Unref();
        */
    }
}

impl MemTableConstructor {

    pub fn new(cmp: Box<dyn SliceComparator>) -> Self {
    
        todo!();
        /*
        : constructor(cmp),
        : internal_comparator(cmp),

            memtable_ = new MemTable(internal_comparator_);
        memtable_->Ref();
        */
    }
    
    pub fn finish_impl(&mut self, 
        options: &Options,
        data:    &KVMap) -> Status {
        
        todo!();
        /*
            memtable_->Unref();
        memtable_ = new MemTable(internal_comparator_);
        memtable_->Ref();
        int seq = 1;
        for (const auto& kvp : data) {
          memtable_->Add(seq, kTypeValue, kvp.first, kvp.second);
          seq++;
        }
        return Status::OK();
        */
    }
    
    pub fn new_iterator(&self) -> *mut LevelDBIterator {
        
        todo!();
        /*
            return new KeyConvertingIterator(memtable_->NewIterator());
        */
    }
}

struct MemTableTest {}

#[test] fn mem_table_test_simple() {
    todo!();
    /*
    
      InternalKeyComparator cmp(BytewiseComparator());
      MemTable* memtable = new MemTable(cmp);
      memtable->Ref();
      WriteBatch batch;
      WriteBatchInternal::SetSequence(&batch, 100);
      batch.Put(std::string("k1"), std::string("v1"));
      batch.Put(std::string("k2"), std::string("v2"));
      batch.Put(std::string("k3"), std::string("v3"));
      batch.Put(std::string("largekey"), std::string("vlarge"));
      ASSERT_TRUE(WriteBatchInternal::InsertInto(&batch, memtable).ok());

      Iterator* iter = memtable->NewIterator();
      iter->SeekToFirst();
      while (iter->Valid()) {
        fprintf(stderr, "key_: '%s' -> '%s'\n", iter->key().ToString().c_str(),
                iter->value().ToString().c_str());
        iter->Next();
      }

      delete iter;
      memtable->Unref();

    */
}

fn between(
        val:  u64,
        low:  u64,
        high: u64) -> bool {
    
    todo!();
        /*
            bool result = (val >= low) && (val <= high);
      if (!result) {
        fprintf(stderr, "Value %llu is not in range [%llu, %llu]\n",
                (unsigned long long)(val), (unsigned long long)(low),
                (unsigned long long)(high));
      }
      return result;
        */
}
