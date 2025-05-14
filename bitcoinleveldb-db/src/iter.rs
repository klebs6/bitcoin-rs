// ---------------- [ File: bitcoinleveldb-db/src/iter.rs ]
crate::ix!();

/**
  | Memtables and sstables that make the DB
  | representation contain (userkey,seq,type) =>
  | uservalue entries.  DBIter combines multiple
  | entries for the same userkey found in the DB
  | representation into a single entry while
  | accounting for sequence numbers, deletion
  | markers, overwrites, etc.
  */
pub struct DBIter {
    base:                      LevelDBIterator,

    db:                        Rc<RefCell<DBImpl>>,
    user_comparator:           Box<dyn SliceComparator>,
    iter:                      LevelDBIterator,
    sequence:                  SequenceNumber,
    status:                    Status,

    /**
      | == current key when direction_==kReverse
      |
      */
    saved_key_:                 String,

    /**
      | == current raw value when direction_==kReverse
      |
      */
    saved_value:               String,

    direction:                 DBIterDirection,
    valid:                     bool,
    rnd:                       Random,
    bytes_until_read_sampling: usize,
}

/**
  | Which direction is the iterator currently
  | moving?
  |
  | (1) When moving forward, the internal
  |     iterator is positioned at the exact entry
  |     that yields this->key(), this->value()
  |
  | (2) When moving backwards, the internal
  |     iterator is positioned just before all
  |     entries whose user key == this->key().
  */
pub enum DBIterDirection { Forward, Reverse }

impl Drop for DBIter {
    fn drop(&mut self) {
        todo!();
        /*
            delete iter_;
        */
    }
}

impl DBIter {

    pub fn new(
        db:   *mut DBImpl,
        cmp:  Box<dyn SliceComparator>,
        iter: *mut LevelDBIterator,
        s:    SequenceNumber,
        seed: u32) -> Self {
    
        todo!();
        /*


            : db_(db),
            user_comparator_(cmp),
            iter_(iter),
            sequence_(s),
            direction_(kForward),
            valid_(false),
            rnd_(seed),
            bytes_until_read_sampling_(RandomCompactionPeriod())
        */
    }
    
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return valid_;
        */
    }
    
    pub fn key(&self) -> Slice {
        
        todo!();
        /*
            assert(valid_);
        return (direction_ == kForward) ? ExtractUserKey(iter_->key()) : saved_key_;
        */
    }
    
    pub fn value(&self) -> Slice {
        
        todo!();
        /*
            assert(valid_);
        return (direction_ == kForward) ? iter_->value() : saved_value_;
        */
    }
    
    pub fn status(&self) -> crate::Status {
        
        todo!();
        /*
            if (status_.ok()) {
          return iter_->status();
        } else {
          return status_;
        }
        */
    }
    
    #[inline] pub fn save_key(&mut self, 
        k:   &Slice,
        dst: *mut String)  {
        
        todo!();
        /*
            dst->assign(k.data(), k.size());
        */
    }
    
    #[inline] pub fn clear_saved_value(&mut self)  {
        
        todo!();
        /*
            if (saved_value_.capacity() > 1048576) {
          std::string empty;
          swap(empty, saved_value_);
        } else {
          saved_value_.clear();
        }
        */
    }

    /**
      | Picks the number of bytes that can be
      | read until a compaction is scheduled.
      |
      */
    pub fn random_compaction_period(&mut self) -> usize {
        
        todo!();
        /*
            return rnd_.Uniform(2 * config::kReadBytesPeriod);
        */
    }
    
    #[inline] pub fn parse_key(&mut self, ikey_: *mut ParsedInternalKey) -> bool {
        
        todo!();
        /*
            Slice k = iter_->key();

      size_t bytes_read = k.size() + iter_->value().size();
      while (bytes_until_read_sampling_ < bytes_read) {
        bytes_until_read_sampling_ += RandomCompactionPeriod();
        db_->RecordReadSample(k);
      }
      assert(bytes_until_read_sampling_ >= bytes_read);
      bytes_until_read_sampling_ -= bytes_read;

      if (!ParseInternalKey(k, ikey)) {
        status_ = Status::Corruption("corrupted internal key in DBIter");
        return false;
      } else {
        return true;
      }
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            assert(valid_);

      if (direction_ == kReverse) {  // Switch directions?
        direction_ = kForward;
        // iter_ is pointing just before the entries for this->key(),
        // so advance into the range of entries for this->key() and then
        // use the normal skipping code below.
        if (!iter_->Valid()) {
          iter_->SeekToFirst();
        } else {
          iter_->Next();
        }
        if (!iter_->Valid()) {
          valid_ = false;
          saved_key_.clear();
          return;
        }
        // saved_key_ already contains the key to skip past.
      } else {
        // Store in saved_key_ the current key so we skip it below.
        SaveKey(ExtractUserKey(iter_->key()), &saved_key_);

        // iter_ is pointing to current key. We can now safely move to the next to
        // avoid checking current key.
        iter_->Next();
        if (!iter_->Valid()) {
          valid_ = false;
          saved_key_.clear();
          return;
        }
      }

      FindNextUserEntry(true, &saved_key_);
        */
    }
    
    pub fn find_next_user_entry(&mut self, 
        skipping: bool,
        skip:     *mut String)  {
        
        todo!();
        /*
            // Loop until we hit an acceptable entry to yield
      assert(iter_->Valid());
      assert(direction_ == kForward);
      do {
        ParsedInternalKey ikey;
        if (ParseKey(&ikey) && ikey.sequence <= sequence_) {
          switch (ikey.type) {
            case kTypeDeletion:
              // Arrange to skip all upcoming entries for this key since
              // they are hidden by this deletion.
              SaveKey(ikey.user_key, skip);
              skipping = true;
              break;
            case kTypeValue:
              if (skipping &&
                  user_comparator_->Compare(ikey.user_key, *skip) <= 0) {
                // Entry hidden
              } else {
                valid_ = true;
                saved_key_.clear();
                return;
              }
              break;
          }
        }
        iter_->Next();
      } while (iter_->Valid());
      saved_key_.clear();
      valid_ = false;
        */
    }
    
    pub fn prev(&mut self)  {
        
        todo!();
        /*
            assert(valid_);

      if (direction_ == kForward) {  // Switch directions?
        // iter_ is pointing at the current entry.  Scan backwards until
        // the key changes so we can use the normal reverse scanning code.
        assert(iter_->Valid());  // Otherwise valid_ would have been false
        SaveKey(ExtractUserKey(iter_->key()), &saved_key_);
        while (true) {
          iter_->Prev();
          if (!iter_->Valid()) {
            valid_ = false;
            saved_key_.clear();
            ClearSavedValue();
            return;
          }
          if (user_comparator_->Compare(ExtractUserKey(iter_->key()), saved_key_) <
              0) {
            break;
          }
        }
        direction_ = kReverse;
      }

      FindPrevUserEntry();
        */
    }
    
    pub fn find_prev_user_entry(&mut self)  {
        
        todo!();
        /*
            assert(direction_ == kReverse);

      ValueType value_type = kTypeDeletion;
      if (iter_->Valid()) {
        do {
          ParsedInternalKey ikey;
          if (ParseKey(&ikey) && ikey.sequence <= sequence_) {
            if ((value_type != kTypeDeletion) &&
                user_comparator_->Compare(ikey.user_key, saved_key_) < 0) {
              // We encountered a non-deleted value in entries for previous keys,
              break;
            }
            value_type = ikey.type;
            if (value_type == kTypeDeletion) {
              saved_key_.clear();
              ClearSavedValue();
            } else {
              Slice raw_value = iter_->value();
              if (saved_value_.capacity() > raw_value.size() + 1048576) {
                std::string empty;
                swap(empty, saved_value_);
              }
              SaveKey(ExtractUserKey(iter_->key()), &saved_key_);
              saved_value_.assign(raw_value.data(), raw_value.size());
            }
          }
          iter_->Prev();
        } while (iter_->Valid());
      }

      if (value_type == kTypeDeletion) {
        // End
        valid_ = false;
        saved_key_.clear();
        ClearSavedValue();
        direction_ = kForward;
      } else {
        valid_ = true;
      }
        */
    }
    
    pub fn seek(&mut self, target: &Slice)  {
        
        todo!();
        /*
            direction_ = kForward;
      ClearSavedValue();
      saved_key_.clear();
      AppendInternalKey(&saved_key_,
                        ParsedInternalKey(target, sequence_, kValueTypeForSeek));
      iter_->Seek(saved_key_);
      if (iter_->Valid()) {
        FindNextUserEntry(false, &saved_key_ /* temporary storage */);
      } else {
        valid_ = false;
      }
        */
    }
    
    pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
            direction_ = kForward;
      ClearSavedValue();
      iter_->SeekToFirst();
      if (iter_->Valid()) {
        FindNextUserEntry(false, &saved_key_ /* temporary storage */);
      } else {
        valid_ = false;
      }
        */
    }
    
    pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
            direction_ = kReverse;
      ClearSavedValue();
      iter_->SeekToLast();
      FindPrevUserEntry();
        */
    }
}

/**
  | Return a new iterator that converts internal
  | keys (yielded by "*internal_iter") that were
  | live at the specified "sequence" number into
  | appropriate user keys.
  */
pub fn new_db_iterator(
        db:                  Rc<RefCell<DBImpl>>,
        user_key_comparator: Box<dyn SliceComparator>,
        internal_iter:       Rc<RefCell<LevelDBIterator>>,
        sequence:            SequenceNumber,
        seed:                u32) -> Rc<RefCell<LevelDBIterator>> {
    
    todo!();
        /*
            return new DBIter(db, user_key_comparator, internal_iter, sequence, seed);
        */
}
