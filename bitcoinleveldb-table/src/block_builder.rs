crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/block_builder.h]

/**
  | BlockBuilder generates blocks where keys are
  | prefix-compressed:
  |
  | When we store a key, we drop the prefix shared
  | with the previous string.  This helps reduce
  | the space requirement significantly.
  | Furthermore, once every K keys, we do not apply
  | the prefix compression and store the entire
  | key.  We call this a "restart point".  The tail
  | end of the block stores the offsets of all of
  | the restart points, and can be used to do
  | a binary search when looking for a particular
  | key.  Values are stored as-is (without
  | compression) immediately following the
  | corresponding key.
  |
  | An entry for a particular key-value pair has the form:
  |     shared_bytes: varint32
  |     unshared_bytes: varint32
  |     value_length: varint32
  |     key_delta: char[unshared_bytes]
  |     value: char[value_length]
  | shared_bytes == 0 for restart points.
  |
  | The trailer of the block has the form:
  |     restarts: uint32[num_restarts]
  |     num_restarts: uint32
  |
  | restarts[i] contains the offset within the
  | block of the ith restart point.
  */
pub struct BlockBuilder {

    options:  *const Options,

    /**
      | Destination buffer
      |
      */
    buffer:   String,

    /**
      | Restart points
      |
      */
    restarts: Vec<u32>,

    /**
      | Number of entries emitted since restart
      |
      */
    counter:  i32,

    /**
      | Has Finish() been called?
      |
      */
    finished: bool,

    last_key_: String,
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/block_builder.cc]
impl BlockBuilder {

    /**
      | Return true iff no entries have been
      | added since the last Reset()
      |
      */
    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return buffer_.empty();
        */
    }
    
    pub fn new(options: *const Options) -> Self {
    
        todo!();
        /*
        : options(options),
        : restarts(),
        : counter(0),
        : finished(false),

            assert(options->block_restart_interval >= 1);
      restarts_.push_back(0);  // First restart point is at offset 0
        */
    }
    
    /**
      | Reset the contents as if the BlockBuilder
      | was just constructed.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            buffer_.clear();
      restarts_.clear();
      restarts_.push_back(0);  // First restart point is at offset 0
      counter_ = 0;
      finished_ = false;
      last_key_.clear();
        */
    }
    
    /**
      | Returns an estimate of the current (uncompressed)
      | size of the block we are building.
      |
      */
    pub fn current_size_estimate(&self) -> usize {
        
        todo!();
        /*
            return (buffer_.size() +                       // Raw data buffer
              restarts_.size() * sizeof(uint32_t) +  // Restart array
              sizeof(uint32_t));                     // Restart array length
        */
    }
    
    /**
      | Finish building the block and return a slice
      | that refers to the block contents.  The
      | returned slice will remain valid for the
      | lifetime of this builder or until Reset() is
      | called.
      */
    pub fn finish(&mut self) -> Slice {
        
        todo!();
        /*
            // Append restart array
      for (size_t i = 0; i < restarts_.size(); i++) {
        PutFixed32(&buffer_, restarts_[i]);
      }
      PutFixed32(&buffer_, restarts_.size());
      finished_ = true;
      return Slice(buffer_);
        */
    }
    
    /**
      | REQUIRES: Finish() has not been called since
      | the last call to Reset().
      |
      | REQUIRES: key is larger than any previously
      | added key
      */
    pub fn add(&mut self, 
        key_:   &Slice,
        value: &Slice)  {
        
        todo!();
        /*
            Slice last_key_piece(last_key_);
      assert(!finished_);
      assert(counter_ <= options_->block_restart_interval);
      assert(buffer_.empty()  // No values yet?
             || options_->comparator->Compare(key, last_key_piece) > 0);
      size_t shared = 0;
      if (counter_ < options_->block_restart_interval) {
        // See how much sharing to do with previous string
        const size_t min_length = std::min(last_key_piece.size(), key.size());
        while ((shared < min_length) && (last_key_piece[shared] == key[shared])) {
          shared++;
        }
      } else {
        // Restart compression
        restarts_.push_back(buffer_.size());
        counter_ = 0;
      }
      const size_t non_shared = key.size() - shared;

      // Add "<shared><non_shared><value_size>" to buffer_
      PutVarint32(&buffer_, shared);
      PutVarint32(&buffer_, non_shared);
      PutVarint32(&buffer_, value.size());

      // Add string delta to buffer_ followed by value
      buffer_.append(key.data() + shared, non_shared);
      buffer_.append(value.data(), value.size());

      // Update state
      last_key_.resize(shared);
      last_key_.append(key.data() + shared, non_shared);
      assert(Slice(last_key_) == key);
      counter_++;
        */
    }
}
