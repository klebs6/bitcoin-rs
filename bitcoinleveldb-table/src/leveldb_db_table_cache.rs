/*!
  | Thread-safe (provides internal synchronization)
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/table_cache.h]

pub struct TableCache {
    env:     Box<dyn Env>,
    dbname:  String,
    options: Rc<Options>,
    cache:   *mut Cache,
}

impl Drop for TableCache {
    fn drop(&mut self) {
        todo!();
        /*
            delete cache_;
        */
    }
}

impl TableCache {

    pub fn new(
        dbname:  &String,
        options: &Options,
        entries: i32) -> Self {
    
        todo!();
        /*
        : env(options.env),
        : dbname(dbname),
        : options(options),
        : cache(NewLRUCache(entries)),
        */
    }
    
    pub fn find_table(&mut self, 
        file_number: u64,
        file_size:   u64,
        handle:      *mut *mut CacheHandle) -> crate::Status {
        
        todo!();
        /*
            Status s;
      char buf[sizeof(file_number)];
      EncodeFixed64(buf, file_number);
      Slice key(buf, sizeof(buf));
      *handle = cache_->Lookup(key);
      if (*handle == nullptr) {
        std::string fname = TableFileName(dbname_, file_number);
        RandomAccessFile* file = nullptr;
        Table* table = nullptr;
        s = env_->NewRandomAccessFile(fname, &file);
        if (!s.ok()) {
          std::string old_fname = SSTTableFileName(dbname_, file_number);
          if (env_->NewRandomAccessFile(old_fname, &file).ok()) {
            s = Status::OK();
          }
        }
        if (s.ok()) {
          s = Table::Open(options_, file, file_size, &table);
        }

        if (!s.ok()) {
          assert(table == nullptr);
          delete file;
          // We do not cache error results so that if the error is transient,
          // or somebody repairs the file, we recover automatically.
        } else {
          TableAndFile* tf = new TableAndFile;
          tf->file = file;
          tf->table = table;
          *handle = cache_->Insert(key, tf, 1, &DeleteEntry);
        }
      }
      return s;
        */
    }
    
    /**
      | Return an iterator for the specified file
      | number (the corresponding file length must be
      | exactly "file_size" bytes).  If "tableptr" is
      | non-null, also sets "*tableptr" to point to
      | the Table object underlying the returned
      | iterator, or to nullptr if no Table object
      | underlies the returned iterator.  The
      | returned "*tableptr" object is owned by the
      | cache and should not be deleted, and is valid
      | for as long as the returned iterator is live.
      */
    pub fn new_iterator(&mut self, 
        options:     &ReadOptions,
        file_number: u64,
        file_size:   u64,
        tableptr:    *mut *mut crate::table::Table) -> *mut LevelDBIterator {
        
        todo!();
        /*
            if (tableptr != nullptr) {
        *tableptr = nullptr;
      }

      CacheHandle* handle = nullptr;
      Status s = FindTable(file_number, file_size, &handle);
      if (!s.ok()) {
        return NewErrorIterator(s);
      }

      Table* table = reinterpret_cast<TableAndFile*>(cache_->Value(handle))->table;
      Iterator* result = table->NewIterator(options);
      result->RegisterCleanup(&UnrefEntry, cache_, handle);
      if (tableptr != nullptr) {
        *tableptr = table;
      }
      return result;
        */
    }
    
    /**
      | If a seek to internal key "k" in specified
      | file finds an entry, call
      | (*handle_result)(arg, found_key,
      | found_value).
      */
    pub fn get(&mut self, 
        options:       &ReadOptions,
        file_number:   u64,
        file_size:     u64,
        k:             &Slice,
        arg:           *mut c_void,
        handle_result: fn(
                _0: *mut c_void,
                _1: &Slice,
                _2: &Slice
        ) -> c_void) -> crate::Status {
        
        todo!();
        /*
            CacheHandle* handle = nullptr;
      Status s = FindTable(file_number, file_size, &handle);
      if (s.ok()) {
        Table* t = reinterpret_cast<TableAndFile*>(cache_->Value(handle))->table;
        s = t->InternalGet(options, k, arg, handle_result);
        cache_->Release(handle);
      }
      return s;
        */
    }
    
    /**
      | Evict any entry for the specified file
      | number
      |
      */
    pub fn evict(&mut self, file_number: u64)  {
        
        todo!();
        /*
            char buf[sizeof(file_number)];
      EncodeFixed64(buf, file_number);
      cache_->Erase(Slice(buf, sizeof(buf)));
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/table_cache.cc]

pub struct TableAndFile {
    file:  *mut dyn RandomAccessFile,
    table: *mut table::Table,
}

pub fn delete_entry(
        key_:  &Slice,
        value: *mut c_void)  {
    
    todo!();
        /*
            TableAndFile* tf = reinterpret_cast<TableAndFile*>(value);
      delete tf->table;
      delete tf->file;
      delete tf;
        */
}

pub fn unref_entry(
        arg1: *mut c_void,
        arg2: *mut c_void)  {
    
    todo!();
        /*
            Cache* cache = reinterpret_cast<Cache*>(arg1);
      CacheHandle* h = reinterpret_cast<CacheHandle*>(arg2);
      cache->Release(h);
        */
}
