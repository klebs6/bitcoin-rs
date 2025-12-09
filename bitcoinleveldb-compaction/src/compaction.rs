// ---------------- [ File: bitcoinleveldb-compaction/src/compaction.rs ]
crate::ix!();

/**
  | A Compaction encapsulates information
  | about a compaction.
  |
  */
pub struct Compaction {

    level:                i32,
    max_output_file_size: u64,
    input_version:        *mut Version,
    edit:                 VersionEdit,

    /*
      | Each compaction reads inputs from "level_"
      | and "level_+1"
      |
      */

    /**
      | The two sets of inputs
      |
      */
    inputs:            [Vec<*mut FileMetaData>; 2],

    /**
      | State used to check for number of overlapping
      | grandparent files (parent == level_
      | + 1, grandparent == level_ + 2)
      |
      */
    grandparents:      Vec<*mut FileMetaData>,

    /**
      | Index in grandparent_starts_
      |
      */
    grandparent_index: usize,

    /**
      | Some output key has been seen
      |
      */
    seen_key_:          bool,

    /**
      | Bytes of overlap between current output
      | and grandparent files
      |
      */
    overlapped_bytes:  i64,

    /*
      State for implementing IsBaseLevelForKey
      */

    /**
      | level_ptrs_ holds indices into
      | input_version_->levels_: our state is that we
      | are positioned at one of the file ranges for
      | each higher level than the ones involved in
      | this compaction (i.e. for all L >= level_
      | + 2).
      */
    level_ptrs: [usize; NUM_LEVELS],
}

impl Drop for Compaction {

    fn drop(&mut self) {
        todo!();
        /*
        if (input_version_ != nullptr) {
            input_version_->Unref();
        }
        */
    }
}

impl Compaction {

    pub fn new(
        options: *const Options,
        level:   i32) -> Self {
    
        todo!();
        /*


            : level_(level),
          max_output_file_size_(MaxFileSizeForLevel(options, level)),
          input_version_(nullptr),
          grandparent_index_(0),
          seen_key_(false),
          overlapped_bytes_(0) 
      for (int i = 0; i < config::NUM_LEVELS; i++) {
        level_ptrs_[i] = 0;
      }
        */
    }
    
    /**
      | Is this a trivial compaction that can be
      | implemented by just moving a single input
      | file to the next level (no merging or
      | splitting)
      */
    pub fn is_trivial_move(&self) -> bool {
        
        todo!();
        /*
            const VersionSet* vset = input_version_->vset_;
      // Avoid a move if there is lots of overlapping grandparent data.
      // Otherwise, the move could create a parent file that will require
      // a very expensive merge later on.
      return (num_input_files(0) == 1 && num_input_files(1) == 0 &&
              TotalFileSize(grandparents_) <=
                  MaxGrandParentOverlapBytes(vset->options_));
        */
    }
    
    /**
      | Add all inputs to this compaction as
      | delete operations to *edit.
      |
      */
    pub fn add_input_deletions(&mut self, edit: *mut VersionEdit)  {
        
        todo!();
        /*
            for (int which = 0; which < 2; which++) {
        for (size_t i = 0; i < inputs_[which].size(); i++) {
          edit->DeleteFile(level_ + which, inputs_[which][i]->number);
        }
      }
        */
    }
    
    /**
      | Returns true if the information we have
      | available guarantees that the compaction is
      | producing data in "level+1" for which no data
      | exists in levels greater than "level+1".
      */
    pub fn is_base_level_for_key(&mut self, user_key_: &Slice) -> bool {
        
        todo!();
        /*
            // Maybe use binary search to find right entry instead of linear search?
      const Comparator* user_cmp = input_version_->vset_->icmp_.user_comparator();
      for (int lvl = level_ + 2; lvl < config::NUM_LEVELS; lvl++) {
        const std::vector<FileMetaData*>& files = input_version_->files_[lvl];
        while (level_ptrs_[lvl] < files.size()) {
          FileMetaData* f = files[level_ptrs_[lvl]];
          if (user_cmp->Compare(user_key, f->largest.user_key()) <= 0) {
            // We've advanced far enough
            if (user_cmp->Compare(user_key, f->smallest.user_key()) >= 0) {
              // Key falls in this file's range, so definitely not base level
              return false;
            }
            break;
          }
          level_ptrs_[lvl]++;
        }
      }
      return true;
        */
    }
    
    /**
      | Returns true iff we should stop building
      | the current output before processing
      | "internal_key".
      |
      */
    pub fn should_stop_before(&mut self, internal_key_: &Slice) -> bool {
        
        todo!();
        /*
            const VersionSet* vset = input_version_->vset_;
      // Scan to find earliest grandparent file that contains key.
      const InternalKeyComparator* icmp = &vset->icmp_;
      while (grandparent_index_ < grandparents_.size() &&
             icmp->Compare(internal_key,
                           grandparents_[grandparent_index_]->largest.Encode()) >
                 0) {
        if (seen_key_) {
          overlapped_bytes_ += grandparents_[grandparent_index_]->file_size;
        }
        grandparent_index_++;
      }
      seen_key_ = true;

      if (overlapped_bytes_ > MaxGrandParentOverlapBytes(vset->options_)) {
        // Too much overlap for current output; start new output
        overlapped_bytes_ = 0;
        return true;
      } else {
        return false;
      }
        */
    }
    
    /**
      | Release the input version for the compaction,
      | once the compaction is successful.
      |
      */
    pub fn release_inputs(&mut self)  {
        
        todo!();
        /*
            if (input_version_ != nullptr) {
        input_version_->Unref();
        input_version_ = nullptr;
      }
        */
    }

    /**
      | Return the level that is being compacted.
      | 
      | Inputs from "level" and "level+1" will
      | be merged to produce a set of "level+1"
      | files.
      |
      */
    pub fn level(&self) -> i32 {
        
        todo!();
        /*
            return level_;
        */
    }

    /**
      | Return the object that holds the edits
      | to the descriptor done by this compaction.
      |
      */
    pub fn edit(&mut self) -> *mut VersionEdit {
        
        todo!();
        /*
            return &edit_;
        */
    }

    /**
      | "which" must be either 0 or 1
      |
      */
    pub fn num_input_files(&self, which: i32) -> i32 {
        
        todo!();
        /*
            return inputs_[which].size();
        */
    }

    /**
      | Return the ith input file at "level()+which"
      | ("which" must be 0 or 1).
      |
      */
    pub fn input(&self, 
        which: i32,
        i:     i32) -> *mut FileMetaData {
        
        todo!();
        /*
            return inputs_[which][i];
        */
    }

    /**
      | Maximum size of files to build during
      | this compaction.
      |
      */
    pub fn max_output_file_size(&self) -> u64 {
        
        todo!();
        /*
            return max_output_file_size_;
        */
    }
}
