/*!
  | A filter block is stored near the end of
  | a Table file.  It contains filters (e.g., bloom
  | filters) for all data blocks in the table
  | combined into a single filter block.
  |
  | See doc/table_format.md for an explanation of
  | the filter block format.
  */

crate::ix!();

/**
  | Generate new filter every 2KB of data
  |
  */
pub const FILTER_BASE_LG: usize = 11;
pub const FILTER_BASE:    usize = 1 << FILTER_BASE_LG;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/filter_block.h]

/**
  | A FilterBlockBuilder is used to construct all
  | of the filters for a particular Table.  It
  | generates a single string which is stored as
  | a special block in the Table.
  |
  | The sequence of calls to FilterBlockBuilder
  |      must match the regexp: (StartBlock
  |      AddKey*)* Finish
  */
pub struct FilterBlockBuilder {

    policy:         Box<dyn FilterPolicy>,

    /**
      | Flattened key contents
      |
      */
    keys:           String,

    /**
      | Starting index in keys_ of each key
      |
      */
    start:          Vec<usize>,

    /**
      | Filter data computed so far
      |
      */
    result:         String,

    /**
      | policy_->CreateFilter() argument
      |
      */
    tmp_keys:       Vec<Slice>,

    filter_offsets: Vec<u32>,
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/filter_block.cc]
impl FilterBlockBuilder {

    pub fn new(policy: Box<dyn FilterPolicy>) -> Self {
    
        todo!();
        /*
        : policy(policy),
        */
    }
    
    pub fn start_block(&mut self, block_offset: u64)  {
        
        todo!();
        /*
            uint64_t filter_index = (block_offset / kFilterBase);
      assert(filter_index >= filter_offsets_.size());
      while (filter_index > filter_offsets_.size()) {
        GenerateFilter();
      }
        */
    }
    
    pub fn add_key(&mut self, key_: &Slice)  {
        
        todo!();
        /*
            Slice k = key;
      start_.push_back(keys_.size());
      keys_.append(k.data(), k.size());
        */
    }
    
    pub fn finish(&mut self) -> Slice {
        
        todo!();
        /*
            if (!start_.empty()) {
        GenerateFilter();
      }

      // Append array of per-filter offsets
      const uint32_t array_offset = result_.size();
      for (size_t i = 0; i < filter_offsets_.size(); i++) {
        PutFixed32(&result_, filter_offsets_[i]);
      }

      PutFixed32(&result_, array_offset);
      result_.push_back(kFilterBaseLg);  // Save encoding parameter in result
      return Slice(result_);
        */
    }
    
    pub fn generate_filter(&mut self)  {
        
        todo!();
        /*
            const size_t num_keys = start_.size();
      if (num_keys == 0) {
        // Fast path if there are no keys for this filter
        filter_offsets_.push_back(result_.size());
        return;
      }

      // Make list of keys from flattened key structure
      start_.push_back(keys_.size());  // Simplify length computation
      tmp_keys_.resize(num_keys);
      for (size_t i = 0; i < num_keys; i++) {
        const char* base = keys_.data() + start_[i];
        size_t length = start_[i + 1] - start_[i];
        tmp_keys_[i] = Slice(base, length);
      }

      // Generate filter for current set of keys and append to result_.
      filter_offsets_.push_back(result_.size());
      policy_->CreateFilter(&tmp_keys_[0], static_cast<int>(num_keys), &result_);

      tmp_keys_.clear();
      keys_.clear();
      start_.clear();
        */
    }
}

pub struct FilterBlockReader {

    policy:  Box<dyn FilterPolicy>,

    /**
      | Pointer to filter data (at block-start)
      |
      */
    data:    *const u8,

    /**
      | Pointer to beginning of offset array
      | (at block-end)
      |
      */
    offset:  *const u8,

    /**
      | Number of entries in offset array
      |
      */
    num:     usize,

    /**
      | Encoding parameter (see kFilterBaseLg
      | in .cc file)
      |
      */
    base_lg: usize,
}

impl FilterBlockReader {

    /**
      | REQUIRES: "contents" and *policy must
      | stay live while *this is live.
      |
      */
    pub fn new(
        policy:   Box<dyn FilterPolicy>,
        contents: &Slice) -> Self {
    
        todo!();
        /*
        : policy(policy),
        : data(nullptr),
        : offset(nullptr),
        : num(0),
        : base_lg(0),

            size_t n = contents.size();
      if (n < 5) return;  // 1 byte for base_lg_ and 4 for start of offset array
      base_lg_ = contents[n - 1];
      uint32_t last_word = DecodeFixed32(contents.data() + n - 5);
      if (last_word > n - 5) return;
      data_ = contents.data();
      offset_ = data_ + last_word;
      num_ = (n - 5 - last_word) / 4;
        */
    }
    
    pub fn key_may_match(&mut self, 
        block_offset: u64,
        key_:          &Slice) -> bool {
        
        todo!();
        /*
            uint64_t index = block_offset >> base_lg_;
      if (index < num_) {
        uint32_t start = DecodeFixed32(offset_ + index * 4);
        uint32_t limit = DecodeFixed32(offset_ + index * 4 + 4);
        if (start <= limit && limit <= static_cast<size_t>(offset_ - data_)) {
          Slice filter = Slice(data_ + start, limit - start);
          return policy_->KeyMayMatch(key, filter);
        } else if (start == limit) {
          // Empty filters do not match any keys
          return false;
        }
      }
      return true;  // Errors are treated as potential matches
        */
    }
}
