crate::ix!();

///----------------
pub struct MemTableInserter {
    sequence: SequenceNumber,
    mem:      *mut MemTable,
}

impl WriteBatchHandler for MemTableInserter {

}

impl WriteBatchPut for MemTableInserter {

    fn put(&mut self, 
        key_:   &Slice,
        value: &Slice)  {
        
        todo!();
        /*
            mem_->Add(sequence_, kTypeValue, key, value);
        sequence_++;
        */
    }
}

impl WriteBatchDelete for MemTableInserter {

    fn delete(&mut self, key_: &Slice)  {
        
        todo!();
        /*
            mem_->Add(sequence_, kTypeDeletion, key, Slice());
        sequence_++;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/write_batch_internal.h]

/**
  | WriteBatchInternal provides static
  | methods for manipulating a WriteBatch
  | that we don't want in the public WriteBatch
  | interface.
  |
  */
pub struct WriteBatchInternal {

}

impl WriteBatchInternal {

    pub fn insert_into(&mut self, 
        b:        *const WriteBatch,
        memtable: *mut MemTable) -> crate::Status {
        
        todo!();
        /*
            MemTableInserter inserter;
      inserter.sequence_ = WriteBatchInternal::Sequence(b);
      inserter.mem_ = memtable;
      return b->Iterate(&inserter);
        */
    }
    
    pub fn set_contents(&mut self, 
        b:        *mut WriteBatch,
        contents: &Slice)  {
        
        todo!();
        /*
            assert(contents.size() >= kHeader);
      b->rep_.assign(contents.data(), contents.size());
        */
    }
    
    pub fn append(&mut self, 
        dst: *mut WriteBatch,
        src: *const WriteBatch)  {
        
        todo!();
        /*
            SetCount(dst, Count(dst) + Count(src));
      assert(src->rep_.size() >= kHeader);
      dst->rep_.append(src->rep_.data() + kHeader, src->rep_.size() - kHeader);
        */
    }

    /**
      | Return the number of entries in the batch.
      |
      */
    pub fn count(&mut self, b: *const WriteBatch) -> i32 {
        
        todo!();
        /*
            return DecodeFixed32(b->rep_.data() + 8);
        */
    }
    
    /**
      | Set the count for the number of entries
      | in the batch.
      |
      */
    pub fn set_count(&mut self, 
        b: *mut WriteBatch,
        n: i32)  {
        
        todo!();
        /*
            EncodeFixed32(&b->rep_[8], n);
        */
    }
    
    /**
      | Return the sequence number for the start
      | of this batch.
      |
      */
    pub fn sequence(&mut self, b: *const WriteBatch) -> SequenceNumber {
        
        todo!();
        /*
            return SequenceNumber(DecodeFixed64(b->rep_.data()));
        */
    }
    
    /**
      | Store the specified number as the sequence
      | number for the start of this batch.
      |
      */
    pub fn set_sequence(&mut self, 
        b:   *mut WriteBatch,
        seq: SequenceNumber)  {
        
        todo!();
        /*
            EncodeFixed64(&b->rep_[0], seq);
        */
    }

    pub fn contents(batch: *const WriteBatch) -> Slice {
        
        todo!();
        /*
            return Slice(batch->rep_);
        */
    }
    
    pub fn byte_size(batch: *const WriteBatch) -> usize {
        
        todo!();
        /*
            return batch->rep_.size();
        */
    }
}
