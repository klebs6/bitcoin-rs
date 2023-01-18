crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/db.h]

// Update CMakeLists.txt if you change these
pub const MAJOR_VERSION: i32 = 1;
pub const MINOR_VERSION: i32 = 22;


/**
   A range of keys
  */
#[derive(Default)]
pub struct Range {

    /**
       Included in the range
      */
    start: Slice,

    /**
       Not included in the range
      */
    limit: Slice,
}

impl Range {

    pub fn new(
        s: &Slice,
        l: &Slice) -> Self {
    
        todo!();
        /*
        : start(s),
        : limit(l),

        
        */
    }
}

/**
  | A DB is a persistent ordered map from keys to
  | values.
  |
  | A DB is safe for concurrent access from
  | multiple threads without any external
  | synchronization.
  */
pub trait DB:
    Put
    + Delete
    + Write
    + Get
    + NewIterator
    + GetSnapshot
    + ReleaseSnapshot
    + GetProperty
    + GetApproximateSizes
    + CompactRange { 

    /**
      | Open the database with the specified "name".
      |
      | Stores a pointer to a heap-allocated database
      | in *dbptr and returns OK on success.
      |
      | Stores nullptr in *dbptr and returns a non-OK
      | status on error.
      |
      | Caller should delete *dbptr when it is no
      | longer needed.
      */
    fn open(&mut self, 
        options: &Options,
        dbname:  &String,
        dbptr:   *mut *mut dyn DB) -> crate::Status {
        
        todo!();
        /*
            *dbptr = nullptr;

      DBImpl* impl = new DBImpl(options, dbname);
      impl->mutex_.Lock();
      VersionEdit edit;
      // Recover handles create_if_missing, error_if_exists
      bool save_manifest = false;
      Status s = impl->Recover(&edit, &save_manifest);
      if (s.ok() && impl->mem_ == nullptr) {
        // Create new log and a corresponding memtable.
        uint64_t new_log_number = impl->versions_->NewFileNumber();
        WritableFile* lfile;
        s = options.env->NewWritableFile(LogFileName(dbname, new_log_number),
                                         &lfile);
        if (s.ok()) {
          edit.SetLogNumber(new_log_number);
          impl->logfile_ = lfile;
          impl->logfile_number_ = new_log_number;
          impl->log_ = new LogWriter(lfile);
          impl->mem_ = new MemTable(impl->internal_comparator_);
          impl->mem_->Ref();
        }
      }
      if (s.ok() && save_manifest) {
        edit.SetPrevLogNumber(0);  // No older logs needed after recovery.
        edit.SetLogNumber(impl->logfile_number_);
        s = impl->versions_->LogAndApply(&edit, &impl->mutex_);
      }
      if (s.ok()) {
        impl->DeleteObsoleteFiles();
        impl->MaybeScheduleCompaction();
      }
      impl->mutex_.Unlock();
      if (s.ok()) {
        assert(impl->mem_ != nullptr);
        *dbptr = impl;
      } else {
        delete impl;
      }
      return s;
        */
    }
}

pub trait Put {

    /**
      | Set the database entry for "key" to "value".
      | Returns OK on success, and a non-OK status on
      | error.
      |
      | Note: consider setting options.sync = true.
      |
      | Default implementations of convenience
      | methods that subclasses of DB can call
      | if they wish
      |
      */
    fn put(&mut self, 
        opt:   &WriteOptions,
        key_:   &Slice,
        value: &Slice) -> crate::Status {
        
        todo!();
        /*
            WriteBatch batch;
      batch.Put(key, value);
      return Write(opt, &batch);
        */
    }
}

pub trait Delete {

    /**
      | Remove the database entry (if any) for "key".
      | Returns OK on success, and a non-OK status on
      | error.  It is not an error if "key" did not
      | exist in the database.
      |
      | Note: consider setting options.sync = true.
      */
    fn delete(&mut self, 
        opt: &WriteOptions,
        key_: &Slice) -> crate::Status {
        
        todo!();
        /*
            WriteBatch batch;
      batch.Delete(key);
      return Write(opt, &batch);
        */
    }
}

pub trait Write {

    /**
      | Apply the specified updates to the database.
      |
      | Returns OK on success, non-OK on failure.
      |
      | Note: consider setting options.sync = true.
      */
    fn write(&mut self, 
            options: &WriteOptions,
            updates: *mut WriteBatch) -> crate::Status;
}

pub trait Get {

    /**
      | If the database contains an entry for "key"
      | store the corresponding value in *value and
      | return OK.
      |
      | If there is no entry for "key" leave *value
      | unchanged and return a status for which
      | Status::IsNotFound() returns true.
      |
      | May return some other Status on an error.
      */
    fn get(&mut self, 
            options: &ReadOptions,
            key_:     &Slice,
            value:   *mut String) -> crate::Status;
}

pub trait NewIterator {

    /**
      | Return a heap-allocated iterator over the
      | contents of the database.  The result of
      | NewIterator() is initially invalid (caller
      | must call one of the Seek methods on the
      | iterator before using it).
      |
      | Caller should delete the iterator when it is
      | no longer needed.  The returned iterator
      | should be deleted before this db is deleted.
      */
    fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator;
}

pub trait GetSnapshot {

    /**
      | Return a handle to the current DB state.
      | Iterators created with this handle will all
      | observe a stable snapshot of the current DB
      | state.
      |
      | The caller must call ReleaseSnapshot(result)
      | when the snapshot is no longer needed.
      */
    fn get_snapshot(&mut self) -> Box<dyn Snapshot>;
}

pub trait ReleaseSnapshot {

    /**
      | Release a previously acquired snapshot.
      | The caller must not use "snapshot" after
      | this call.
      |
      */
    fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>);
}

pub trait GetProperty {

    /**
      | DB implementations can export properties
      | about their state via this method.  If
      | "property" is a valid property understood by
      | this DB implementation, fills "*value" with
      | its current value and returns true.
      | Otherwise returns false.
      |
      |
      | Valid property names include:
      |
      |  "leveldb.num-files-at-level<N>" - return the
      |     number of files at level <N>, where <N>
      |     is an ASCII representation of a level
      |     number (e.g. "0").
      |
      |  "leveldb.stats" - returns a multi-line
      |     string that describes statistics about
      |     the internal operation of the DB.
      |
      |  "leveldb.sstables" - returns a multi-line
      |     string that describes all of the sstables
      |     that make up the db contents.
      |
      |  "leveldb.approximate-memory-usage" - returns
      |     the approximate number of bytes of memory
      |     in use by the DB.
      */
    fn get_property(&mut self, 
            property: &str,
            value:    *mut String) -> bool;
}

pub trait GetApproximateSizes {

    /**
      | For each i in [0,n-1], store in "sizes[i]",
      | the approximate file system space used by
      | keys in "[range[i].start .. range[i].limit)".
      |
      | Note that the returned sizes measure file
      | system space usage, so if the user data
      | compresses by a factor of ten, the returned
      | sizes will be one-tenth the size of the
      | corresponding user data size.
      |
      | The results may not include the sizes of
      | recently written data.
      */
    fn get_approximate_sizes(&mut self, 
            range: *const Range,
            n:     i32,
            sizes: *mut u64);
}

pub trait CompactRange {

    /**
      | Compact the underlying storage for the key
      | range [*begin,*end].  In particular, deleted
      | and overwritten versions are discarded, and
      | the data is rearranged to reduce the cost of
      | operations needed to access the data.  This
      | operation should typically only be invoked by
      | users who understand the underlying
      | implementation.
      |
      | begin==nullptr is treated as a key before all
      | keys in the database.  end==nullptr is
      | treated as a key after all keys in the
      | database.  Therefore the following call will
      | compact the entire database:
      | db->CompactRange(nullptr, nullptr);
      */
    fn compact_range(&mut self, 
            begin: *const Slice,
            end:   *const Slice);
}

/**
  | Destroy the contents of the specified database.
  | Be very careful using this method.
  |
  | Note: For backwards compatibility, if DestroyDB
  | is unable to list the database files,
  | Status::OK() will still be returned masking
  | this failure.
  */
pub fn destroydb(
        name:    &String,
        options: &Options) -> crate::Status {
    
    todo!();
        /*
        
        */
}

/**
  | If a DB cannot be opened, you may attempt to
  | call this method to resurrect as much of the
  | contents of the database as possible.
  |
  | Some data may be lost, so be careful when
  | calling this function on a database that
  | contains important information.
  */
pub fn repairdb(
        dbname:  &String,
        options: &Options) -> crate::Status {
    
    todo!();
        /*
        
        */
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/c.cc]

pub struct LevelDB {
    rep: Rc<RefCell<dyn DB>>,
}

pub struct LevelDBWriteBatch {
    rep: WriteBatch,
}

pub struct LevelDBSnapshot {
    rep: Rc<dyn Snapshot>,
}

pub struct LevelDBReadOptions {
    rep: ReadOptions,
}

pub struct LevelDBWriteOptions {
    rep: WriteOptions,
}

pub struct LevelDBOptions {
    rep: Options,
}

pub struct LevelDBCache {
    rep: Rc<RefCell<crate::Cache>>,
}

pub struct LevelDBSeqFile {
    rep: Rc<RefCell<dyn SequentialFile>>,
}

pub struct LevelDBRandomFile {
    rep: Rc<RefCell<dyn RandomAccessFile>>,
}

pub struct LevelDBWritableFile {
    rep: Rc<RefCell<dyn WritableFile>>,
}

pub struct LevelDBLogger {
    rep: Rc<RefCell<dyn Logger>>,
}

pub struct LevelDBFileLock {
    rep: Rc<RefCell<Box<dyn FileLock>>>,
}

///-----------------
pub struct LevelDBComparator {

    state:      *mut c_void,

    destructor: fn(_0: *mut c_void) -> c_void,

    compare:    fn(
            _0:   *mut c_void,
            a:    *const u8,
            alen: usize,
            b:    *const u8,
            blen: usize
    ) -> i32,

    name:       fn(_0: *mut c_void) -> *const u8,
}

impl Comparator<Slice> for LevelDBComparator {
    fn compare(&self, 
        a: &Slice,
        b: &Slice) -> Ordering {
        
        todo!();
        /*
            return (*compare_)(state_, a.data(), a.size(), b.data(), b.size());
        */
    }
}

impl FindShortestSeparator for LevelDBComparator {

    /**
      | No-ops since the C binding does not support
      | key shortening methods.
      |
      */
    fn find_shortest_separator(&self, 
        _0: *mut String,
        _1: &Slice)  {
        
        todo!();
        /*
        
        */
    }
}

impl FindShortSuccessor for LevelDBComparator {
    fn find_short_successor(&self, key_: *mut String)  {
        
        todo!();
        /*
        
        */
    }
}

impl Drop for LevelDBComparator {
    fn drop(&mut self) {
        todo!();
        /*
            (*destructor_)(state_);
        */
    }
}

impl Name for LevelDBComparator {

    fn name(&self) -> *const u8 {
        
        todo!();
        /*
            return (*name_)(state_);
        */
    }
}

///-----------------
pub struct LevelDBFilterPolicy {
    state:      *mut c_void,

    destructor: fn(_0: *mut c_void) -> c_void,

    name:       fn(_0: *mut c_void) -> *const u8,

    create:     fn(
            _0:               *mut c_void,
            key_array:        *const *const u8,
            key_length_array: *const usize,
            num_keys:         i32,
            filter_length:    *mut usize
    ) -> *mut u8,

    key_match:  fn(
            _0:            *mut c_void,
            key_:           *const u8,
            length:        usize,
            filter:        *const u8,
            filter_length: usize
    ) -> u8,
}

impl FilterPolicy for LevelDBFilterPolicy {

}

impl Drop for LevelDBFilterPolicy {
    fn drop(&mut self) {
        todo!();
        /*
            (*destructor_)(state_);
        */
    }
}

impl Name for LevelDBFilterPolicy {
    fn name(&self) -> *const u8 {
        
        todo!();
        /*
            return (*name_)(state_);
        */
    }
}

impl CreateFilter for LevelDBFilterPolicy {
    fn create_filter(&self, 
        keys: *const Slice,
        n:    i32,
        dst:  *mut String)  {
        
        todo!();
        /*
            std::vector<const char*> key_pointers(n);
            std::vector<size_t> key_sizes(n);
            for (int i = 0; i < n; i++) {
              key_pointers[i] = keys[i].data();
              key_sizes[i] = keys[i].size();
            }
            size_t len;
            char* filter = (*create_)(state_, &key_pointers[0], &key_sizes[0], n, &len);
            dst->append(filter, len);
            free(filter);
        */
    }
}

impl KeyMayMatch for LevelDBFilterPolicy {
    fn key_may_match(&self, 
        key_:    &Slice,
        filter: &Slice) -> bool {
        
        todo!();
        /*
            return (*key_match_)(state_, key.data(), key.size(), filter.data(),
                                 filter.size());
        */
    }
}

///-----------------
pub struct LevelDBEnv {
    rep:        Rc<RefCell<dyn Env>>,
    is_default: bool,
}

pub fn save_error(
        errptr: *mut *mut u8,
        s:      &Status) -> bool {
    
    todo!();
        /*
            assert(errptr != nullptr);
          if (s.ok()) {
            return false;
          } else if (*errptr == nullptr) {
            *errptr = strdup(s.ToString().c_str());
          } else {
            // TODO(sanjay): Merge with existing error?
            free(*errptr);
            *errptr = strdup(s.ToString().c_str());
          }
          return true;
        */
}

pub fn copy_string(str_: &String) -> *mut u8 {
    
    todo!();
        /*
            char* result = reinterpret_cast<char*>(malloc(sizeof(char) * str.size()));
          memcpy(result, str.data(), sizeof(char) * str.size());
          return result;
        */
}

pub fn leveldb_open(
        options: *const LevelDBOptions,
        name:    *const u8,
        errptr:  *mut *mut u8) -> *mut LevelDB {
    
    todo!();
        /*
            DB* db;
          if (SaveError(errptr, DB::Open(options->rep, std::string(name), &db))) {
            return nullptr;
          }
          leveldb_t* result = new leveldb_t;
          result->rep = db;
          return result;
        */
}

pub fn leveldb_close(db: *mut LevelDB)  {
    
    todo!();
        /*
            delete db->rep;
          delete db;
        */
}

pub fn leveldb_put(
        db:      *mut LevelDB,
        options: *const LevelDBWriteOptions,
        key_:     *const u8,
        keylen:  usize,
        val:     *const u8,
        vallen:  usize,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr,
                    db->rep->Put(options->rep, Slice(key, keylen), Slice(val, vallen)));
        */
}

pub fn leveldb_delete(
        db:      *mut LevelDB,
        options: *const LevelDBWriteOptions,
        key_:     *const u8,
        keylen:  usize,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, db->rep->Delete(options->rep, Slice(key, keylen)));
        */
}

pub fn leveldb_write(
        db:      *mut LevelDB,
        options: *const LevelDBWriteOptions,
        batch:   *mut LevelDBWriteBatch,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, db->rep->Write(options->rep, &batch->rep));
        */
}

pub fn leveldb_get(
        db:      *mut LevelDB,
        options: *const LevelDBReadOptions,
        key_:     *const u8,
        keylen:  usize,
        vallen:  *mut usize,
        errptr:  *mut *mut u8) -> *mut u8 {
    
    todo!();
        /*
            char* result = nullptr;
          std::string tmp;
          Status s = db->rep->Get(options->rep, Slice(key, keylen), &tmp);
          if (s.ok()) {
            *vallen = tmp.size();
            result = CopyString(tmp);
          } else {
            *vallen = 0;
            if (!s.IsNotFound()) {
              SaveError(errptr, s);
            }
          }
          return result;
        */
}

pub fn leveldb_create_iterator(
        db:      *mut LevelDB,
        options: *const LevelDBReadOptions) -> *mut LevelDBIterator {
    
    todo!();
        /*
            leveldb_iterator_t* result = new leveldb_iterator_t;
          result->rep = db->rep->NewIterator(options->rep);
          return result;
        */
}

pub fn leveldb_create_snapshot(db: *mut LevelDB) -> *const LevelDBSnapshot {
    
    todo!();
        /*
            leveldb_snapshot_t* result = new leveldb_snapshot_t;
          result->rep = db->rep->GetSnapshot();
          return result;
        */
}

pub fn leveldb_release_snapshot(
        db:       *mut LevelDB,
        snapshot: *const LevelDBSnapshot)  {
    
    todo!();
        /*
            db->rep->ReleaseSnapshot(snapshot->rep);
          delete snapshot;
        */
}

pub fn leveldb_property_value(
        db:       *mut LevelDB,
        propname: *const u8) -> *mut u8 {
    
    todo!();
        /*
            std::string tmp;
          if (db->rep->GetProperty(Slice(propname), &tmp)) {
            // We use strdup() since we expect human readable output.
            return strdup(tmp.c_str());
          } else {
            return nullptr;
          }
        */
}

pub fn leveldb_approximate_sizes(
        db:                  *mut LevelDB,
        num_ranges:          i32,
        range_start_key_:     *const *const u8,
        range_start_key_len: *const usize,
        range_limit_key_:     *const *const u8,
        range_limit_key_len: *const usize,
        sizes:               *mut u64)  {
    
    todo!();
        /*
            Range* ranges = new Range[num_ranges];
          for (int i = 0; i < num_ranges; i++) {
            ranges[i].start = Slice(range_start_key[i], range_start_key_len[i]);
            ranges[i].limit = Slice(range_limit_key[i], range_limit_key_len[i]);
          }
          db->rep->GetApproximateSizes(ranges, num_ranges, sizes);
          delete[] ranges;
        */
}

pub fn leveldb_compact_range(
        db:            *mut LevelDB,
        start_key_:     *const u8,
        start_key_len: usize,
        limit_key_:     *const u8,
        limit_key_len: usize)  {
    
    todo!();
        /*
            Slice a, b;
          db->rep->CompactRange(
              // Pass null Slice if corresponding "const char*" is null
              (start_key ? (a = Slice(start_key, start_key_len), &a) : nullptr),
              (limit_key ? (b = Slice(limit_key, limit_key_len), &b) : nullptr));
        */
}

pub fn leveldb_destroy_db(
        options: *const LevelDBOptions,
        name:    *const u8,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, DestroyDB(name, options->rep));
        */
}

pub fn leveldb_repair_db(
        options: *const LevelDBOptions,
        name:    *const u8,
        errptr:  *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, RepairDB(name, options->rep));
        */
}

pub fn leveldb_iter_destroy(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            delete iter->rep;
          delete iter;
        */
}

pub fn leveldb_iter_valid(iter: *const LevelDBIterator) -> u8 {
    
    todo!();
        /*
            return iter->rep->Valid();
        */
}

pub fn leveldb_iter_seek_to_first(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->SeekToFirst();
        */
}

pub fn leveldb_iter_seek_to_last(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->SeekToLast();
        */
}

pub fn leveldb_iter_seek(
        iter: *mut LevelDBIterator,
        k:    *const u8,
        klen: usize)  {
    
    todo!();
        /*
            iter->rep->Seek(Slice(k, klen));
        */
}

pub fn leveldb_iter_next(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->Next();
        */
}

pub fn leveldb_iter_prev(iter: *mut LevelDBIterator)  {
    
    todo!();
        /*
            iter->rep->Prev();
        */
}

pub fn leveldb_iter_key(
        iter: *const LevelDBIterator,
        klen: *mut usize) -> *const u8 {
    
    todo!();
        /*
            Slice s = iter->rep->key();
          *klen = s.size();
          return s.data();
        */
}

pub fn leveldb_iter_value(
        iter: *const LevelDBIterator,
        vlen: *mut usize) -> *const u8 {
    
    todo!();
        /*
            Slice s = iter->rep->value();
          *vlen = s.size();
          return s.data();
        */
}

pub fn leveldb_iter_get_error(
        iter:   *const LevelDBIterator,
        errptr: *mut *mut u8)  {
    
    todo!();
        /*
            SaveError(errptr, iter->rep->status());
        */
}

pub fn leveldb_writebatch_create() -> *mut LevelDBWriteBatch {
    
    todo!();
        /*
            return new leveldb_writebatch_t;
        */
}

pub fn leveldb_writebatch_destroy(b: *mut LevelDBWriteBatch)  {
    
    todo!();
        /*
            delete b;
        */
}

pub fn leveldb_writebatch_clear(b: *mut LevelDBWriteBatch)  {
    
    todo!();
        /*
            b->rep.Clear();
        */
}

pub fn leveldb_writebatch_put(
        b:    *mut LevelDBWriteBatch,
        key_:  *const u8,
        klen: usize,
        val:  *const u8,
        vlen: usize)  {
    
    todo!();
        /*
            b->rep.Put(Slice(key, klen), Slice(val, vlen));
        */
}

pub fn leveldb_writebatch_delete(
        b:    *mut LevelDBWriteBatch,
        key_:  *const u8,
        klen: usize)  {
    
    todo!();
        /*
            b->rep.Delete(Slice(key, klen));
        */
}

pub fn leveldb_writebatch_iterate(
        b:       *const LevelDBWriteBatch,
        state:   *mut c_void,
        put:     fn(
                _0:   *mut c_void,
                k:    *const u8,
                klen: usize,
                v:    *const u8,
                vlen: usize
        ) -> c_void,
        deleted: fn(
                _0:   *mut c_void,
                k:    *const u8,
                klen: usize
        ) -> c_void)  {
    
    todo!();
        /*
            class H : public WriteBatch::Handler {
           
            c_void* state_;
            c_void (*put_)(c_void*, const char* k, size_t klen, const char* v, size_t vlen);
            c_void (*deleted_)(c_void*, const char* k, size_t klen);
            c_void Put(const Slice& key, const Slice& value) override {
              (*put_)(state_, key.data(), key.size(), value.data(), value.size());
            }
            c_void Delete(const Slice& key) override {
              (*deleted_)(state_, key.data(), key.size());
            }
          };
          H handler;
          handler.state_ = state;
          handler.put_ = put;
          handler.deleted_ = deleted;
          b->rep.Iterate(&handler);
        */
}

pub fn leveldb_writebatch_append(
        destination: *mut LevelDBWriteBatch,
        source:      *const LevelDBWriteBatch)  {
    
    todo!();
        /*
            destination->rep.Append(source->rep);
        */
}

pub fn leveldb_options_create() -> *mut LevelDBOptions {
    
    todo!();
        /*
            return new leveldb_options_t;
        */
}

pub fn leveldb_options_destroy(options: *mut LevelDBOptions)  {
    
    todo!();
        /*
            delete options;
        */
}

pub fn leveldb_options_set_comparator(
        opt: *mut LevelDBOptions,
        cmp: *mut LevelDBComparator)  {
    
    todo!();
        /*
            opt->rep.comparator = cmp;
        */
}

pub fn leveldb_options_set_filter_policy(
        opt:    *mut LevelDBOptions,
        policy: *mut LevelDBFilterPolicy)  {
    
    todo!();
        /*
            opt->rep.filter_policy = policy;
        */
}

pub fn leveldb_options_set_create_if_missing(
        opt: *mut LevelDBOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.create_if_missing = v;
        */
}

pub fn leveldb_options_set_error_if_exists(
        opt: *mut LevelDBOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.error_if_exists = v;
        */
}

pub fn leveldb_options_set_paranoid_checks(
        opt: *mut LevelDBOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.paranoid_checks = v;
        */
}

pub fn leveldb_options_set_env(
        opt: *mut LevelDBOptions,
        env: *mut LevelDBEnv)  {
    
    todo!();
        /*
            opt->rep.env = (env ? env->rep : nullptr);
        */
}

pub fn leveldb_options_set_info_log(
        opt: *mut LevelDBOptions,
        l:   *mut LevelDBLogger)  {
    
    todo!();
        /*
            opt->rep.info_log = (l ? l->rep : nullptr);
        */
}

pub fn leveldb_options_set_write_buffer_size(
        opt: *mut LevelDBOptions,
        s:   usize)  {
    
    todo!();
        /*
            opt->rep.write_buffer_size = s;
        */
}

pub fn leveldb_options_set_max_open_files(
        opt: *mut LevelDBOptions,
        n:   i32)  {
    
    todo!();
        /*
            opt->rep.max_open_files = n;
        */
}

pub fn leveldb_options_set_cache(
        opt: *mut LevelDBOptions,
        c:   *mut LevelDBCache)  {
    
    todo!();
        /*
            opt->rep.block_cache = c->rep;
        */
}

pub fn leveldb_options_set_block_size(
        opt: *mut LevelDBOptions,
        s:   usize)  {
    
    todo!();
        /*
            opt->rep.block_size = s;
        */
}

pub fn leveldb_options_set_block_restart_interval(
        opt: *mut LevelDBOptions,
        n:   i32)  {
    
    todo!();
        /*
            opt->rep.block_restart_interval = n;
        */
}

pub fn leveldb_options_set_max_file_size(
        opt: *mut LevelDBOptions,
        s:   usize)  {
    
    todo!();
        /*
            opt->rep.max_file_size = s;
        */
}

pub fn leveldb_options_set_compression(
        opt: *mut LevelDBOptions,
        t:   i32)  {
    
    todo!();
        /*
            opt->rep.compression = static_cast<CompressionType>(t);
        */
}

pub fn leveldb_comparator_create(
        state:      *mut c_void,
        destructor: fn(_0: *mut c_void) -> c_void,
        compare:    fn(
                _0:   *mut c_void,
                a:    *const u8,
                alen: usize,
                b:    *const u8,
                blen: usize
        ) -> i32,
        name:       fn(_0: *mut c_void) -> *const u8) -> *mut LevelDBComparator {
    
    todo!();
        /*
            leveldb_comparator_t* result = new leveldb_comparator_t;
          result->state_ = state;
          result->destructor_ = destructor;
          result->compare_ = compare;
          result->name_ = name;
          return result;
        */
}

pub fn leveldb_comparator_destroy(cmp: *mut LevelDBComparator)  {
    
    todo!();
        /*
            delete cmp;
        */
}

pub fn leveldb_filterpolicy_create(
        state:         *mut c_void,
        destructor:    fn(_0: *mut c_void) -> c_void,
        create_filter: fn(
                _0:               *mut c_void,
                key_array:        *const *const u8,
                key_length_array: *const usize,
                num_keys:         i32,
                filter_length:    *mut usize
        ) -> *mut u8,
        key_may_match: fn(
                _0:            *mut c_void,
                key_:           *const u8,
                length:        usize,
                filter:        *const u8,
                filter_length: usize
        ) -> u8,
        name:          fn(_0: *mut c_void) -> *mut u8) -> *mut LevelDBFilterPolicy {
    
    todo!();
        /*
            leveldb_filterpolicy_t* result = new leveldb_filterpolicy_t;
          result->state_ = state;
          result->destructor_ = destructor;
          result->create_ = create_filter;
          result->key_match_ = key_may_match;
          result->name_ = name;
          return result;
        */
}

pub fn leveldb_filterpolicy_destroy(filter: *mut LevelDBFilterPolicy)  {
    
    todo!();
        /*
            delete filter;
        */
}

pub fn leveldb_filterpolicy_create_bloom(bits_per_key_: i32) -> *mut LevelDBFilterPolicy {
    
    todo!();
        /*
            // Make a leveldb_filterpolicy_t, but override all of its methods so
          // they delegate to a NewBloomFilterPolicy() instead of user
          // supplied C functions.
          struct Wrapper : public leveldb_filterpolicy_t {
            static c_void DoNothing(c_void*) {}

            ~Wrapper() { delete rep_; }
            const char* Name() const { return rep_->Name(); }
            c_void CreateFilter(const Slice* keys, int n, std::string* dst) const {
              return rep_->CreateFilter(keys, n, dst);
            }
            bool KeyMayMatch(const Slice& key, const Slice& filter) const {
              return rep_->KeyMayMatch(key, filter);
            }

            const FilterPolicy* rep_;
          };
          Wrapper* wrapper = new Wrapper;
          wrapper->rep_ = NewBloomFilterPolicy(bits_per_key);
          wrapper->state_ = nullptr;
          wrapper->destructor_ = &Wrapper::DoNothing;
          return wrapper;
        */
}

pub fn leveldb_readoptions_create() -> *mut LevelDBReadOptions {
    
    todo!();
        /*
            return new leveldb_readoptions_t;
        */
}

pub fn leveldb_readoptions_destroy(opt: *mut LevelDBReadOptions)  {
    
    todo!();
        /*
            delete opt;
        */
}

pub fn leveldb_readoptions_set_verify_checksums(
        opt: *mut LevelDBReadOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.verify_checksums = v;
        */
}

pub fn leveldb_readoptions_set_fill_cache(
        opt: *mut LevelDBReadOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.fill_cache = v;
        */
}

pub fn leveldb_readoptions_set_snapshot(
        opt:  *mut LevelDBReadOptions,
        snap: *const LevelDBSnapshot)  {
    
    todo!();
        /*
            opt->rep.snapshot = (snap ? snap->rep : nullptr);
        */
}

pub fn leveldb_writeoptions_create() -> *mut LevelDBWriteOptions {
    
    todo!();
        /*
            return new leveldb_writeoptions_t;
        */
}

pub fn leveldb_writeoptions_destroy(opt: *mut LevelDBWriteOptions)  {
    
    todo!();
        /*
            delete opt;
        */
}

pub fn leveldb_writeoptions_set_sync(
        opt: *mut LevelDBWriteOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.sync = v;
        */
}

pub fn leveldb_cache_create_lru(capacity: usize) -> *mut LevelDBCache {
    
    todo!();
        /*
            leveldb_cache_t* c = new leveldb_cache_t;
          c->rep = NewLRUCache(capacity);
          return c;
        */
}

pub fn leveldb_cache_destroy(cache: *mut LevelDBCache)  {
    
    todo!();
        /*
            delete cache->rep;
          delete cache;
        */
}

pub fn leveldb_create_default_env() -> *mut LevelDBEnv {
    
    todo!();
        /*
            leveldb_env_t* result = new leveldb_env_t;
          result->rep = Env::Default();
          result->is_default = true;
          return result;
        */
}

pub fn leveldb_env_destroy(env: *mut LevelDBEnv)  {
    
    todo!();
        /*
            if (!env->is_default) delete env->rep;
          delete env;
        */
}

pub fn leveldb_env_get_test_directory(env: *mut LevelDBEnv) -> *mut u8 {
    
    todo!();
        /*
            std::string result;
          if (!env->rep->GetTestDirectory(&result).ok()) {
            return nullptr;
          }

          char* buffer = static_cast<char*>(malloc(result.size() + 1));
          memcpy(buffer, result.data(), result.size());
          buffer[result.size()] = '\0';
          return buffer;
        */
}

pub fn leveldb_free(ptr: *mut c_void)  {
    
    todo!();
        /*
            free(ptr);
        */
}

pub fn leveldb_major_version() -> i32 {
    
    todo!();
        /*
            return kMajorVersion;
        */
}

pub fn leveldb_minor_version() -> i32 {
    
    todo!();
        /*
            return kMinorVersion;
        */
}
