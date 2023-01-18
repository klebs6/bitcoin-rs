crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/builder.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/builder.cc]

/**
  | Build a Table file from the contents
  | of *iter.
  | 
  | The generated file will be named according
  | to meta->number.
  | 
  | On success, the rest of *meta will be
  | filled with metadata about the generated
  | table.
  | 
  | If no data is present in *iter, meta->file_size
  | will be set to zero, and no Table file
  | will be produced.
  |
  */
pub fn build_table(
        dbname:      &String,
        env:         Rc<RefCell<dyn Env>>,
        options:     &Options,
        table_cache: *mut TableCache,
        iter:        *mut LevelDBIterator,
        meta:        *mut FileMetaData) -> crate::Status {
    
    todo!();
        /*
            Status s;
      meta->file_size = 0;
      iter->SeekToFirst();

      std::string fname = TableFileName(dbname, meta->number);
      if (iter->Valid()) {
        WritableFile* file;
        s = env->NewWritableFile(fname, &file);
        if (!s.ok()) {
          return s;
        }

        TableBuilder* builder = new TableBuilder(options, file);
        meta->smallest.DecodeFrom(iter->key());
        for (; iter->Valid(); iter->Next()) {
          Slice key = iter->key();
          meta->largest.DecodeFrom(key);
          builder->Add(key, iter->value());
        }

        // Finish and check for builder errors
        s = builder->Finish();
        if (s.ok()) {
          meta->file_size = builder->FileSize();
          assert(meta->file_size > 0);
        }
        delete builder;

        // Finish and check for file errors
        if (s.ok()) {
          s = file->Sync();
        }
        if (s.ok()) {
          s = file->Close();
        }
        delete file;
        file = nullptr;

        if (s.ok()) {
          // Verify that the table is usable
          Iterator* it = table_cache->NewIterator(ReadOptions(), meta->number,
                                                  meta->file_size);
          s = it->status();
          delete it;
        }
      }

      // Check for input iterator errors
      if (!iter->status().ok()) {
        s = iter->status();
      }

      if (s.ok() && meta->file_size > 0) {
        // Keep it
      } else {
        env->DeleteFile(fname);
      }
      return s;
        */
}
