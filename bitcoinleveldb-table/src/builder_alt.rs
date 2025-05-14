// ---------------- [ File: bitcoinleveldb-table/src/builder_alt.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/table_builder.h]

/**
  | TableBuilder provides the interface used to
  | build a Table (an immutable and sorted map from
  | keys to values).
  |
  | Multiple threads can invoke const methods on
  | a TableBuilder without external
  | synchronization, but if any of the threads may
  | call a non-const method, all threads accessing
  | the same TableBuilder must use external
  | synchronization.
  */
pub struct TableBuilder {
    rep: *mut TableBuilderRep,
}

impl Drop for TableBuilder {

    /**
      | REQUIRES: Either Finish() or Abandon()
      | has been called.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            assert(rep_->closed);  // Catch errors where caller forgot to call Finish()
      delete rep_->filter_block;
      delete rep_;
        */
    }
}

impl TableBuilder {
    
    /**
      | Create a builder that will store the contents
      | of the table it is building in *file.  Does
      | not close the file.  It is up to the caller
      | to close the file after calling Finish().
      */
    pub fn new(
        options: &Options,
        file:    *mut dyn WritableFile) -> Self {
    
        todo!();
        /*


            : rep_(new Rep(options, file)) 

      if (rep_->filter_block != nullptr) {
        rep_->filter_block->StartBlock(0);
      }
        */
    }
    
    /**
      | Change the options used by this builder.
      | Note: only some of the option fields can be
      | changed after construction.  If a field is
      | not allowed to change dynamically and its
      | value in the structure passed to the
      | constructor is different from its value in
      | the structure passed to this method, this
      | method will return an error without changing
      | any fields.
      */
    pub fn change_options(&mut self, options: &Options) -> crate::Status {
        
        todo!();
        /*
            // Note: if more fields are added to Options, update
      // this function to catch changes that should not be allowed to
      // change in the middle of building a Table.
      if (options.comparator != rep_->options.comparator) {
        return Status::InvalidArgument("changing comparator while building table");
      }

      // Note that any live BlockBuilders point to rep_->options and therefore
      // will automatically pick up the updated options.
      rep_->options = options;
      rep_->index_block_options = options;
      rep_->index_block_options.block_restart_interval = 1;
      return Status::OK();
        */
    }
    
    /**
      | Add key,value to the table being constructed.
      |
      | REQUIRES: key is after any previously added
      | key according to comparator.
      |
      | REQUIRES: Finish(), Abandon() have not been
      | called
      */
    pub fn add(&mut self, 
        key_:   &Slice,
        value: &Slice)  {
        
        todo!();
        /*
            Rep* r = rep_;
      assert(!r->closed);
      if (!ok()) return;
      if (r->num_entries > 0) {
        assert(r->options.comparator->Compare(key, Slice(r->last_key)) > 0);
      }

      if (r->pending_index_entry) {
        assert(r->data_block.empty());
        r->options.comparator->FindShortestSeparator(&r->last_key, key);
        std::string handle_encoding;
        r->pending_handle.EncodeTo(&handle_encoding);
        r->index_block.Add(r->last_key, Slice(handle_encoding));
        r->pending_index_entry = false;
      }

      if (r->filter_block != nullptr) {
        r->filter_block->AddKey(key);
      }

      r->last_key.assign(key.data(), key.size());
      r->num_entries++;
      r->data_block.Add(key, value);

      const size_t estimated_block_size = r->data_block.CurrentSizeEstimate();
      if (estimated_block_size >= r->options.block_size) {
        Flush();
      }
        */
    }
    
    /**
      | Advanced operation: flush any buffered
      | key/value pairs to file.
      |
      | Can be used to ensure that two adjacent
      | entries never live in the same data block.
      | Most clients should not need to use this
      | method.
      |
      | REQUIRES: Finish(), Abandon() have not been
      | called
      */
    pub fn flush(&mut self)  {
        
        todo!();
        /*
            Rep* r = rep_;
      assert(!r->closed);
      if (!ok()) return;
      if (r->data_block.empty()) return;
      assert(!r->pending_index_entry);
      WriteBlock(&r->data_block, &r->pending_handle);
      if (ok()) {
        r->pending_index_entry = true;
        r->status = r->file->Flush();
      }
      if (r->filter_block != nullptr) {
        r->filter_block->StartBlock(r->offset);
      }
        */
    }
    
    pub fn write_block(&mut self, 
        block:  *mut BlockBuilder,
        handle: *mut BlockHandle)  {
        
        todo!();
        /*
            // File format contains a sequence of blocks where each block has:
      //    block_data: uint8[n]
      //    type: uint8
      //    crc: uint32
      assert(ok());
      Rep* r = rep_;
      Slice raw = block->Finish();

      Slice block_contents;
      CompressionType type = r->options.compression;
      // TODO(postrelease): Support more compression options: zlib?
      switch (type) {
        case kNoCompression:
          block_contents = raw;
          break;

        case kSnappyCompression: {
          std::string* compressed = &r->compressed_output;
          if (Snappy_Compress(raw.data(), raw.size(), compressed) &&
              compressed->size() < raw.size() - (raw.size() / 8u)) {
            block_contents = *compressed;
          } else {
            // Snappy not supported, or compressed less than 12.5%, so just
            // store uncompressed form
            block_contents = raw;
            type = kNoCompression;
          }
          break;
        }
      }
      WriteRawBlock(block_contents, type, handle);
      r->compressed_output.clear();
      block->Reset();
        */
    }
    
    pub fn write_raw_block(&mut self, 
        block_contents: &Slice,
        ty:             CompressionType,
        handle:         *mut BlockHandle)  {
        
        todo!();
        /*
            Rep* r = rep_;
      handle->set_offset(r->offset);
      handle->set_size(block_contents.size());
      r->status = r->file->Append(block_contents);
      if (r->status.ok()) {
        char trailer[kBlockTrailerSize];
        trailer[0] = type;
        uint32_t crc = crc32c::Value(block_contents.data(), block_contents.size());
        crc = crc32c::Extend(crc, trailer, 1);  // Extend crc to cover block type
        EncodeFixed32(trailer + 1, crc32c::Mask(crc));
        r->status = r->file->Append(Slice(trailer, kBlockTrailerSize));
        if (r->status.ok()) {
          r->offset += block_contents.size() + kBlockTrailerSize;
        }
      }
        */
    }
    
    /**
      | Return non-ok iff some error has been
      | detected.
      |
      */
    pub fn status(&self) -> Status {
        
        todo!();
        /*
            return rep_->status;
        */
    }
    
    /**
      | Finish building the table.  Stops using the
      | file passed to the constructor after this
      | function returns.
      |
      | REQUIRES: Finish(), Abandon() have not been
      | called
      */
    pub fn finish(&mut self) -> Status {
        
        todo!();
        /*
            Rep* r = rep_;
      Flush();
      assert(!r->closed);
      r->closed = true;

      BlockHandle filter_block_handle, metaindex_block_handle, index_block_handle;

      // Write filter block
      if (ok() && r->filter_block != nullptr) {
        WriteRawBlock(r->filter_block->Finish(), kNoCompression,
                      &filter_block_handle);
      }

      // Write metaindex block
      if (ok()) {
        BlockBuilder meta_index_block(&r->options);
        if (r->filter_block != nullptr) {
          // Add mapping from "filter.Name" to location of filter data
          std::string key = "filter.";
          key.append(r->options.filter_policy->Name());
          std::string handle_encoding;
          filter_block_handle.EncodeTo(&handle_encoding);
          meta_index_block.Add(key, handle_encoding);
        }

        // TODO(postrelease): Add stats and other meta blocks
        WriteBlock(&meta_index_block, &metaindex_block_handle);
      }

      // Write index block
      if (ok()) {
        if (r->pending_index_entry) {
          r->options.comparator->FindShortSuccessor(&r->last_key);
          std::string handle_encoding;
          r->pending_handle.EncodeTo(&handle_encoding);
          r->index_block.Add(r->last_key, Slice(handle_encoding));
          r->pending_index_entry = false;
        }
        WriteBlock(&r->index_block, &index_block_handle);
      }

      // Write footer
      if (ok()) {
        Footer footer;
        footer.set_metaindex_handle(metaindex_block_handle);
        footer.set_index_handle(index_block_handle);
        std::string footer_encoding;
        footer.EncodeTo(&footer_encoding);
        r->status = r->file->Append(footer_encoding);
        if (r->status.ok()) {
          r->offset += footer_encoding.size();
        }
      }
      return r->status;
        */
    }
    
    /**
      | Indicate that the contents of this builder
      | should be abandoned.  Stops using the file
      | passed to the constructor after this function
      | returns.
      |
      | If the caller is not going to call Finish(),
      | it must call Abandon() before destroying this
      | builder.
      |
      | REQUIRES: Finish(), Abandon() have not been
      | called
      */
    pub fn abandon(&mut self)  {
        
        todo!();
        /*
            Rep* r = rep_;
      assert(!r->closed);
      r->closed = true;
        */
    }
    
    /**
      | Number of calls to Add() so far.
      |
      */
    pub fn num_entries(&self) -> u64 {
        
        todo!();
        /*
            return rep_->num_entries;
        */
    }
    
    /**
      | Size of the file generated so far. If
      | invoked after a successful Finish()
      | call, returns the size of the final generated
      | file.
      |
      */
    pub fn file_size(&self) -> u64 {
        
        todo!();
        /*
            return rep_->offset;
        */
    }

    pub fn ok(&self) -> bool {
        
        todo!();
        /*
            return status().ok();
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/table_builder.cc]

pub struct TableBuilderRep {
    options:             Options,
    index_block_options: Options,
    file:                *mut dyn WritableFile,
    offset:              u64,
    status:              Status,
    data_block:          BlockBuilder,
    index_block:         BlockBuilder,
    last_key_:            String,
    num_entries:         i64,

    /**
      | Either Finish() or Abandon() has been
      | called.
      |
      */
    closed:              bool,

    filter_block:        *mut FilterBlockBuilder,

    /**
      | We do not emit the index entry for a block
      | until we have seen the first key for the next
      | data block.  This allows us to use shorter
      | keys in the index block.  For example,
      | consider a block boundary between the keys
      | "the quick brown fox" and "the who".  We can
      | use "the r" as the key for the index block
      | entry since it is >= all entries in the first
      | block and < all entries in subsequent blocks.
      |
      | Invariant: r->pending_index_entry is true
      | only if data_block is empty.
      */
    pending_index_entry: bool,

    /**
      | Handle to add to index block
      |
      */
    pending_handle:      BlockHandle,

    compressed_output:   String,
}

impl TableBuilderRep {

    pub fn new(
        opt: &Options,
        f:   *mut dyn WritableFile) -> Self {
    
        todo!();
        /*


            : options(opt),
            index_block_options(opt),
            file(f),
            offset(0),
            data_block(&options),
            index_block(&index_block_options),
            num_entries(0),
            closed(false),
            filter_block(opt.filter_policy == nullptr
                             ? nullptr
                             : new FilterBlockBuilder(opt.filter_policy)),
            pending_index_entry(false) 
        index_block_options.block_restart_interval = 1;
        */
    }
}
