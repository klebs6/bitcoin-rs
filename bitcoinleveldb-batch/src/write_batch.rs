/*!
  | WriteBatch holds a collection of updates to
  | apply atomically to a DB.
  |
  | The updates are applied in the order in which
  | they are added to the WriteBatch.  For example,
  | the value of "key" will be "v3" after the
  | following batch is written:
  |
  |    batch.Put("key", "v1");
  |    batch.Delete("key");
  |    batch.Put("key", "v2");
  |    batch.Put("key", "v3");
  |
  | Multiple threads can invoke const methods on
  | a WriteBatch without external synchronization,
  | but if any of the threads may call a non-const
  | method, all threads accessing the same
  | WriteBatch must use external synchronization.
  */

crate::ix!();

/**
  | WriteBatch header has an 8-byte sequence
  | number followed by a 4-byte count.
  |
  */
pub const HEADER: usize = 12;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/write_batch.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/write_batch.cc]

pub trait WriteBatchPut {
    fn put(&mut self, 
            key_:   &Slice,
            value: &Slice);
}

pub trait WriteBatchDelete {
    fn delete(&mut self, key_: &Slice);
}

pub trait WriteBatchHandler: 
WriteBatchPut 
+ WriteBatchDelete { }

/**
  | WriteBatch::rep_ :=
  |    sequence: fixed64
  |    count: fixed32
  |    data: record[count]
  | record :=
  |    kTypeValue varstring varstring         |
  |    kTypeDeletion varstring
  | varstring :=
  |    len: varint32
  |    data: uint8[len]
  */
pub struct WriteBatch {

    /**
      | See comment in write_batch.cc for the
      | format of rep_
      |
      */
    rep: String,
}

impl WriteBatch {

    pub fn new() -> Self {
    
        todo!();
        /*
            Clear();
        */
    }
    
    /**
      | Clear all updates buffered in this batch.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            rep_.clear();
      rep_.resize(kHeader);
        */
    }
    
    /**
      | The size of the database changes caused by
      | this batch.
      |
      | This number is tied to implementation
      | details, and may change across releases. It
      | is intended for LevelDB usage metrics.
      */
    pub fn approximate_size(&self) -> usize {
        
        todo!();
        /*
            return rep_.size();
        */
    }
    
    /**
      | Support for iterating over the contents
      | of a batch.
      |
      */
    pub fn iterate(&self, handler: *mut dyn WriteBatchHandler) -> crate::Status {
        
        todo!();
        /*
            Slice input(rep_);
      if (input.size() < kHeader) {
        return Status::Corruption("malformed WriteBatch (too small)");
      }

      input.remove_prefix(kHeader);
      Slice key, value;
      int found = 0;
      while (!input.empty()) {
        found++;
        char tag = input[0];
        input.remove_prefix(1);
        switch (tag) {
          case kTypeValue:
            if (GetLengthPrefixedSlice(&input, &key) &&
                GetLengthPrefixedSlice(&input, &value)) {
              handler->Put(key, value);
            } else {
              return Status::Corruption("bad WriteBatch Put");
            }
            break;
          case kTypeDeletion:
            if (GetLengthPrefixedSlice(&input, &key)) {
              handler->Delete(key);
            } else {
              return Status::Corruption("bad WriteBatch Delete");
            }
            break;
          default:
            return Status::Corruption("unknown WriteBatch tag");
        }
      }
      if (found != WriteBatchInternal::Count(this)) {
        return Status::Corruption("WriteBatch has wrong count");
      } else {
        return Status::OK();
      }
        */
    }
    
    /**
      | Store the mapping "key->value" in the
      | database.
      |
      */
    pub fn put(&mut self, 
        key_:   &Slice,
        value: &Slice)  {
        
        todo!();
        /*
            WriteBatchInternal::SetCount(this, WriteBatchInternal::Count(this) + 1);
      rep_.push_back(static_cast<char>(kTypeValue));
      PutLengthPrefixedSlice(&rep_, key);
      PutLengthPrefixedSlice(&rep_, value);
        */
    }
    
    /**
      | If the database contains a mapping for
      | "key", erase it. Else do nothing.
      |
      */
    pub fn delete(&mut self, key_: &Slice)  {
        
        todo!();
        /*
            WriteBatchInternal::SetCount(this, WriteBatchInternal::Count(this) + 1);
      rep_.push_back(static_cast<char>(kTypeDeletion));
      PutLengthPrefixedSlice(&rep_, key);
        */
    }
    
    /**
      | Copies the operations in "source" to this
      | batch.
      |
      | This runs in O(source size) time. However,
      | the constant factor is better than calling
      | Iterate() over the source batch with
      | a Handler that replicates the operations into
      | this batch.
      */
    pub fn append(&mut self, source: &WriteBatch)  {
        
        todo!();
        /*
            WriteBatchInternal::Append(this, &source);
        */
    }
}
