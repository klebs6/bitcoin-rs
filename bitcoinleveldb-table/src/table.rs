// ---------------- [ File: bitcoinleveldb-table/src/table.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/table.h]

/**
  | A Table is a sorted map from strings to
  | strings.  Tables are immutable and persistent.
  | A Table may be safely accessed from multiple
  | threads without external synchronization.
  */
pub struct Table {
    rep: *const TableRep,
}

impl Table {

    pub fn new(rep: *mut TableRep) -> Self {
    
        todo!();
        /*
        : rep(rep),
        */
    }

    /**
      | Attempt to open the table that is stored in
      | bytes [0..file_size) of "file", and read the
      | metadata entries necessary to allow
      | retrieving data from the table.
      |
      | If successful, returns ok and sets "*table"
      | to the newly opened table.  The client should
      | delete "*table" when no longer needed.  If
      | there was an error while initializing the
      | table, sets "*table" to nullptr and returns
      | a non-ok status.  Does not take ownership of
      | "*source", but the client must ensure that
      | "source" remains live for the duration of the
      | returned table's lifetime.
      |
      | *file must remain live while this Table is in
      | use.
      */
    pub fn open(&mut self, 
        options: &Options,
        file:    Rc<RefCell<dyn RandomAccessFile>>,
        size:    u64,
        table:   *mut *mut Table) -> Status {
        
        todo!();
        /*
            *table = nullptr;
      if (size < Footer::kEncodedLength) {
        return Status::Corruption("file is too short to be an sstable");
      }

      char footer_space[Footer::kEncodedLength];
      Slice footer_input;
      Status s = file->Read(size - Footer::kEncodedLength, Footer::kEncodedLength,
                            &footer_input, footer_space);
      if (!s.ok()) return s;

      Footer footer;
      s = footer.DecodeFrom(&footer_input);
      if (!s.ok()) return s;

      // Read the index block
      BlockContents index_block_contents;
      if (s.ok()) {
        ReadOptions opt;
        if (options.paranoid_checks) {
          opt.verify_checksums = true;
        }
        s = ReadBlock(file, opt, footer.index_handle(), &index_block_contents);
      }

      if (s.ok()) {
        // We've successfully read the footer and the index block: we're
        // ready to serve requests.
        Block* index_block = new Block(index_block_contents);
        Rep* rep = new Table::Rep;
        rep->options = options;
        rep->file = file;
        rep->metaindex_handle = footer.metaindex_handle();
        rep->index_block = index_block;
        rep->cache_id = (options.block_cache ? options.block_cache->NewId() : 0);
        rep->filter_data = nullptr;
        rep->filter = nullptr;
        *table = new Table(rep);
        (*table)->ReadMeta(footer);
      }

      return s;
        */
    }
    
    pub fn read_meta(&mut self, footer: &Footer)  {
        
        todo!();
        /*
            if (rep_->options.filter_policy == nullptr) {
        return;  // Do not need any metadata
      }

      // TODO(sanjay): Skip this if footer.metaindex_handle() size indicates
      // it is an empty block.
      ReadOptions opt;
      if (rep_->options.paranoid_checks) {
        opt.verify_checksums = true;
      }
      BlockContents contents;
      if (!ReadBlock(rep_->file, opt, footer.metaindex_handle(), &contents).ok()) {
        // Do not propagate errors since meta info is not needed for operation
        return;
      }
      Block* meta = new Block(contents);

      Iterator* iter = meta->NewIterator(BytewiseComparator());
      std::string key = "filter.";
      key.append(rep_->options.filter_policy->Name());
      iter->Seek(key);
      if (iter->Valid() && iter->key() == Slice(key)) {
        ReadFilter(iter->value());
      }
      delete iter;
      delete meta;
        */
    }
    
    pub fn read_filter(&mut self, filter_handle_value: &Slice)  {
        
        todo!();
        /*
            Slice v = filter_handle_value;
      BlockHandle filter_handle;
      if (!filter_handle.DecodeFrom(&v).ok()) {
        return;
      }

      // We might want to unify with ReadBlock() if we start
      // requiring checksum verification in Table::Open.
      ReadOptions opt;
      if (rep_->options.paranoid_checks) {
        opt.verify_checksums = true;
      }
      BlockContents block;
      if (!ReadBlock(rep_->file, opt, filter_handle, &block).ok()) {
        return;
      }
      if (block.heap_allocated) {
        rep_->filter_data = block.data.data();  // Will need to delete later
      }
      rep_->filter = new FilterBlockReader(rep_->options.filter_policy, block.data);
        */
    }

    /**
      | Convert an index iterator value (i.e.,
      | an encoded BlockHandle) into an iterator
      | over the contents of the corresponding
      | block.
      |
      */
    pub fn block_reader(&mut self, 
        arg:         *mut c_void,
        options:     &ReadOptions,
        index_value: &Slice) -> *mut LevelDBIterator {
        
        todo!();
        /*
            Table* table = reinterpret_cast<Table*>(arg);
      Cache* block_cache = table->rep_->options.block_cache;
      Block* block = nullptr;
      Cache::Handle* cache_handle = nullptr;

      BlockHandle handle;
      Slice input = index_value;
      Status s = handle.DecodeFrom(&input);
      // We intentionally allow extra stuff in index_value so that we
      // can add more features in the future.

      if (s.ok()) {
        BlockContents contents;
        if (block_cache != nullptr) {
          char cache_key_buffer[16];
          EncodeFixed64(cache_key_buffer, table->rep_->cache_id);
          EncodeFixed64(cache_key_buffer + 8, handle.offset());
          Slice key(cache_key_buffer, sizeof(cache_key_buffer));
          cache_handle = block_cache->Lookup(key);
          if (cache_handle != nullptr) {
            block = reinterpret_cast<Block*>(block_cache->Value(cache_handle));
          } else {
            s = ReadBlock(table->rep_->file, options, handle, &contents);
            if (s.ok()) {
              block = new Block(contents);
              if (contents.cachable && options.fill_cache) {
                cache_handle = block_cache->Insert(key, block, block->size(),
                                                   &DeleteCachedBlock);
              }
            }
          }
        } else {
          s = ReadBlock(table->rep_->file, options, handle, &contents);
          if (s.ok()) {
            block = new Block(contents);
          }
        }
      }

      Iterator* iter;
      if (block != nullptr) {
        iter = block->NewIterator(table->rep_->options.comparator);
        if (cache_handle == nullptr) {
          iter->RegisterCleanup(&DeleteBlock, block, nullptr);
        } else {
          iter->RegisterCleanup(&ReleaseBlock, block_cache, cache_handle);
        }
      } else {
        iter = NewErrorIterator(s);
      }
      return iter;
        */
    }
    
    /**
      | Returns a new iterator over the table
      | contents.
      |
      | The result of NewIterator() is initially
      | invalid (caller must call one of the Seek
      | methods on the iterator before using it).
      */
    pub fn new_iterator(&self, options: &ReadOptions) -> *mut LevelDBIterator {
        
        todo!();
        /*
            return NewTwoLevelIterator(
          rep_->index_block->NewIterator(rep_->options.comparator),
          &Table::BlockReader, const_cast<Table*>(this), options);
        */
    }
    
    /**
      | Calls (*handle_result)(arg, ...) with the
      | entry found after a call to Seek(key).  May
      | not make such a call if filter policy says
      | that key is not present.
      */
    pub fn internal_get(&mut self, 
        options:       &ReadOptions,
        k:             &Slice,
        arg:           *mut c_void,
        handle_result: fn(
                _0: *mut c_void,
                _1: &Slice,
                _2: &Slice
        ) -> c_void) -> Status {
        
        todo!();
        /*
            Status s;
      Iterator* iiter = rep_->index_block->NewIterator(rep_->options.comparator);
      iiter->Seek(k);
      if (iiter->Valid()) {
        Slice handle_value = iiter->value();
        FilterBlockReader* filter = rep_->filter;
        BlockHandle handle;
        if (filter != nullptr && handle.DecodeFrom(&handle_value).ok() &&
            !filter->KeyMayMatch(handle.offset(), k)) {
          // Not found
        } else {
          Iterator* block_iter = BlockReader(this, options, iiter->value());
          block_iter->Seek(k);
          if (block_iter->Valid()) {
            (*handle_result)(arg, block_iter->key(), block_iter->value());
          }
          s = block_iter->status();
          delete block_iter;
        }
      }
      if (s.ok()) {
        s = iiter->status();
      }
      delete iiter;
      return s;
        */
    }
    
    /**
      | Given a key, return an approximate byte
      | offset in the file where the data for that
      | key begins (or would begin if the key were
      | present in the file).  The returned value is
      | in terms of file bytes, and so includes
      | effects like compression of the underlying
      | data.
      |
      | E.g., the approximate offset of the last key
      | in the table will be close to the file
      | length.
      */
    pub fn approximate_offset_of(&self, key_: &Slice) -> u64 {
        
        todo!();
        /*
            Iterator* index_iter =
          rep_->index_block->NewIterator(rep_->options.comparator);
      index_iter->Seek(key);
      uint64_t result;
      if (index_iter->Valid()) {
        BlockHandle handle;
        Slice input = index_iter->value();
        Status s = handle.DecodeFrom(&input);
        if (s.ok()) {
          result = handle.offset();
        } else {
          // Strange: we can't decode the block handle in the index block.
          // We'll just return the offset of the metaindex block, which is
          // close to the whole file size for this case.
          result = rep_->metaindex_handle.offset();
        }
      } else {
        // key is past the last key in the file.  Approximate the offset
        // by returning the offset of the metaindex block (which is
        // right near the end of the file).
        result = rep_->metaindex_handle.offset();
      }
      delete index_iter;
      return result;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/table.cc]

pub struct TableRep {

    options:          Options,
    status:           Status,
    file:             Rc<RefCell<dyn RandomAccessFile>>,
    cache_id:         u64,
    filter:           *mut FilterBlockReader,
    filter_data:      *const u8,

    /**
      | Handle to metaindex_block: saved from
      | footer
      |
      */
    metaindex_handle: BlockHandle,

    index_block:      *mut Block,
}

impl Drop for TableRep {
    fn drop(&mut self) {
        todo!();
        /*
            delete filter;
        delete[] filter_data;
        delete index_block;
        */
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        todo!();
        /*
            delete rep_;
        */
    }
}

pub fn delete_block(
        arg:     *mut c_void,
        ignored: *mut c_void)  {
    
    todo!();
        /*
            delete reinterpret_cast<Block*>(arg);
        */
}

pub fn delete_cached_block(
        key_:   &Slice,
        value: *mut c_void)  {
    
    todo!();
        /*
            Block* block = reinterpret_cast<Block*>(value);
      delete block;
        */
}

pub fn release_block(
        arg: *mut c_void,
        h:   *mut c_void)  {
    
    todo!();
        /*
            Cache* cache = reinterpret_cast<Cache*>(arg);
      Cache::Handle* handle = reinterpret_cast<Cache::Handle*>(h);
      cache->Release(handle);
        */
}
