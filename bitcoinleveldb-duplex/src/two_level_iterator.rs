// ---------------- [ File: bitcoinleveldb-duplex/src/two_level_iterator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/two_level_iterator.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/two_level_iterator.cc]

pub type BlockFunction = fn(*mut c_void,&ReadOptions,&Slice);

pub struct TwoLevelIterator {
    base:              LevelDBIterator,
    block_function:    BlockFunction,
    arg:               *mut c_void,
    options:           ReadOptions,
    status:            Status,
    index_iter:        LevelDBIteratorWrapper,

    /**
       May be nullptr
      */
    data_iter:         LevelDBIteratorWrapper,

    /**
      | If data_iter_ is non-null, then
      | "data_block_handle_" holds the
      | "index_value" passed to block_function_ to
      | create the data_iter_.
      |
      */
    data_block_handle: String,
}

impl TwoLevelIterator {
    
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return data_iter_.Valid();
        */
    }
    
    pub fn key(&self) -> Slice {
        
        todo!();
        /*
            assert(Valid());
        return data_iter_.key();
        */
    }
    
    pub fn value(&self) -> Slice {
        
        todo!();
        /*
            assert(Valid());
        return data_iter_.value();
        */
    }
    
    pub fn status(&self) -> crate::Status {
        
        todo!();
        /*
            // It'd be nice if status() returned a const Status& instead of a Status
        if (!index_iter_.status().ok()) {
          return index_iter_.status();
        } else if (data_iter_.iter() != nullptr && !data_iter_.status().ok()) {
          return data_iter_.status();
        } else {
          return status_;
        }
        */
    }
    
    pub fn save_error(&mut self, s: &Status)  {
        
        todo!();
        /*
            if (status_.ok() && !s.ok()) status_ = s;
        */
    }
    
    pub fn new(
        index_iter:     *mut LevelDBIterator,
        block_function: BlockFunction,
        arg:            *mut c_void,
        options:        &ReadOptions) -> Self {
    
        todo!();
        /*
        : block_function(block_function),
        : arg(arg),
        : options(options),
        : index_iter(index_iter),
        : data_iter(nullptr),

        
        */
    }
    
    pub fn seek(&mut self, target: &Slice)  {
        
        todo!();
        /*
            index_iter_.Seek(target);
      InitDataBlock();
      if (data_iter_.iter() != nullptr) data_iter_.Seek(target);
      SkipEmptyDataBlocksForward();
        */
    }
    
    pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
            index_iter_.SeekToFirst();
      InitDataBlock();
      if (data_iter_.iter() != nullptr) data_iter_.SeekToFirst();
      SkipEmptyDataBlocksForward();
        */
    }
    
    pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
            index_iter_.SeekToLast();
      InitDataBlock();
      if (data_iter_.iter() != nullptr) data_iter_.SeekToLast();
      SkipEmptyDataBlocksBackward();
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            assert(Valid());
      data_iter_.Next();
      SkipEmptyDataBlocksForward();
        */
    }
    
    pub fn prev(&mut self)  {
        
        todo!();
        /*
            assert(Valid());
      data_iter_.Prev();
      SkipEmptyDataBlocksBackward();
        */
    }
    
    pub fn skip_empty_data_blocks_forward(&mut self)  {
        
        todo!();
        /*
            while (data_iter_.iter() == nullptr || !data_iter_.Valid()) {
        // Move to next block
        if (!index_iter_.Valid()) {
          SetDataIterator(nullptr);
          return;
        }
        index_iter_.Next();
        InitDataBlock();
        if (data_iter_.iter() != nullptr) data_iter_.SeekToFirst();
      }
        */
    }
    
    pub fn skip_empty_data_blocks_backward(&mut self)  {
        
        todo!();
        /*
            while (data_iter_.iter() == nullptr || !data_iter_.Valid()) {
        // Move to next block
        if (!index_iter_.Valid()) {
          SetDataIterator(nullptr);
          return;
        }
        index_iter_.Prev();
        InitDataBlock();
        if (data_iter_.iter() != nullptr) data_iter_.SeekToLast();
      }
        */
    }
    
    pub fn set_data_iterator(&mut self, data_iter: *mut LevelDBIterator)  {
        
        todo!();
        /*
            if (data_iter_.iter() != nullptr) SaveError(data_iter_.status());
      data_iter_.Set(data_iter);
        */
    }
    
    pub fn init_data_block(&mut self)  {
        
        todo!();
        /*
            if (!index_iter_.Valid()) {
        SetDataIterator(nullptr);
      } else {
        Slice handle = index_iter_.value();
        if (data_iter_.iter() != nullptr &&
            handle.compare(data_block_handle_) == 0) {
          // data_iter_ is already constructed with this iterator, so
          // no need to change anything
        } else {
          Iterator* iter = (*block_function_)(arg_, options_, handle);
          data_block_handle_.assign(handle.data(), handle.size());
          SetDataIterator(iter);
        }
      }
        */
    }
}

/**
  | Return a new two level iterator.  A two-level
  | iterator contains an index iterator whose
  | values point to a sequence of blocks where each
  | block is itself a sequence of key,value pairs.
  | The returned two-level iterator yields the
  | concatenation of all key/value pairs in the
  | sequence of blocks.  Takes ownership of
  | "index_iter" and will delete it when no longer
  | needed.
  |
  | Uses a supplied function to convert an
  | index_iter value into an iterator over the
  | contents of the corresponding block.
  */
pub fn new_two_level_iterator(
        index_iter:     *mut LevelDBIterator,
        block_function: BlockFunction,
        arg:            *mut c_void,
        options:        &ReadOptions) -> *mut LevelDBIterator {
    
    todo!();
        /*
            return new TwoLevelIterator(index_iter, block_function, arg, options);
        */
}
